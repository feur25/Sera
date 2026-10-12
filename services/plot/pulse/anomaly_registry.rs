use crate::Chart;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use seraplot_macros::sera_doc_impl;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 256;
const MAX_TRACKED_INDICES_PER_CHART: usize = 4096;
const SEASONAL_HOURS: usize = 24;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Method {
    Zscore,
    Iqr,
    Seasonal,
}

impl Method {
    pub fn parse(s: &str) -> PyResult<Self> {
        match s {
            "zscore" => Ok(Method::Zscore),
            "iqr" => Ok(Method::Iqr),
            "seasonal" => Ok(Method::Seasonal),
            other => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "seraplot: unknown anomaly detection method '{other}', expected 'zscore', 'iqr' or 'seasonal'"
            ))),
        }
    }
}

fn zscore_score(buf: &VecDeque<f64>, value: f64) -> f64 {
    let mean: f64 = buf.iter().sum::<f64>() / buf.len() as f64;
    let var: f64 = buf.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / buf.len() as f64;
    let std = var.sqrt().max(1e-9);
    (value - mean) / std
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.len() == 1 {
        return sorted[0];
    }
    let idx = p * (sorted.len() - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    if lo == hi {
        sorted[lo]
    } else {
        sorted[lo] + (sorted[hi] - sorted[lo]) * (idx - lo as f64)
    }
}

fn iqr_score(buf: &VecDeque<f64>, value: f64) -> f64 {
    let mut sorted: Vec<f64> = buf.iter().copied().collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q1 = percentile(&sorted, 0.25);
    let q3 = percentile(&sorted, 0.75);
    let iqr = (q3 - q1).max(1e-9);
    if value < q1 {
        (value - q1) / iqr
    } else if value > q3 {
        (value - q3) / iqr
    } else {
        0.0
    }
}

fn hour_of_day(now_ms: i64) -> usize {
    (now_ms.div_euclid(3_600_000).rem_euclid(SEASONAL_HOURS as i64)) as usize
}

struct PerIndexState {
    buffers: HashMap<u32, VecDeque<f64>>,
    order: VecDeque<u32>,
}

fn new_seasonal_buckets() -> [VecDeque<f64>; SEASONAL_HOURS] {
    std::array::from_fn(|_| VecDeque::new())
}

struct SeasonalIndexState {
    buckets: HashMap<u32, [VecDeque<f64>; SEASONAL_HOURS]>,
    order: VecDeque<u32>,
}

fn check_seasonal_with_hour(
    state: &Mutex<SeasonalIndexState>,
    window: usize,
    threshold: f64,
    index: u32,
    value: f64,
    hour: usize,
) -> Option<f64> {
    let mut st = state.lock().unwrap_or_else(|e| e.into_inner());
    if !st.buckets.contains_key(&index) {
        st.order.push_back(index);
        while st.order.len() > MAX_TRACKED_INDICES_PER_CHART {
            if let Some(old) = st.order.pop_front() {
                st.buckets.remove(&old);
            }
        }
    }
    let entry = st.buckets.entry(index).or_insert_with(new_seasonal_buckets);
    let buf = &mut entry[hour];
    let score = if buf.len() >= 2 { Some(zscore_score(buf, value)) } else { None };
    buf.push_back(value);
    while buf.len() > window {
        buf.pop_front();
    }
    score.filter(|s| s.abs() > threshold)
}

enum DetectorState {
    Window(Mutex<PerIndexState>),
    Seasonal(Mutex<SeasonalIndexState>),
}

impl DetectorState {
    fn new(method: Method) -> Self {
        match method {
            Method::Zscore | Method::Iqr => DetectorState::Window(Mutex::new(PerIndexState { buffers: HashMap::new(), order: VecDeque::new() })),
            Method::Seasonal => DetectorState::Seasonal(Mutex::new(SeasonalIndexState { buckets: HashMap::new(), order: VecDeque::new() })),
        }
    }
}

pub struct Detector {
    window: usize,
    threshold: f64,
    method: Method,
    state: DetectorState,
    callback: PyObject,
}

impl Detector {
    fn check(&self, index: u32, value: f64) -> Option<f64> {
        match &self.state {
            DetectorState::Window(state) => {
                let mut st = state.lock().unwrap_or_else(|e| e.into_inner());
                if !st.buffers.contains_key(&index) {
                    st.order.push_back(index);
                    while st.order.len() > MAX_TRACKED_INDICES_PER_CHART {
                        if let Some(old) = st.order.pop_front() {
                            st.buffers.remove(&old);
                        }
                    }
                }
                let window = self.window;
                let buf = st.buffers.entry(index).or_insert_with(|| VecDeque::with_capacity(window));
                let score = if buf.len() >= 2 {
                    Some(match self.method {
                        Method::Zscore => zscore_score(buf, value),
                        Method::Iqr => iqr_score(buf, value),
                        Method::Seasonal => unreachable!("seasonal detectors always use DetectorState::Seasonal"),
                    })
                } else {
                    None
                };
                buf.push_back(value);
                while buf.len() > window {
                    buf.pop_front();
                }
                score.filter(|s| s.abs() > self.threshold)
            }
            DetectorState::Seasonal(state) => {
                let hour = hour_of_day(crate::plot::now_ms());
                check_seasonal_with_hour(state, self.window, self.threshold, index, value, hour)
            }
        }
    }
}

struct Registry {
    map: HashMap<u64, std::sync::Arc<Detector>>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn set(hid: u64, window: usize, threshold: f64, method: Method, callback: PyObject) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
    reg.map.insert(
        hid,
        std::sync::Arc::new(Detector {
            window: window.max(2),
            threshold,
            method,
            state: DetectorState::new(method),
            callback,
        }),
    );
}

pub fn clear(hid: u64) {
    if let Ok(mut guard) = REGISTRY.lock() {
        if let Some(reg) = guard.as_mut() {
            reg.map.remove(&hid);
        }
    }
}

fn get(hid: u64) -> Option<std::sync::Arc<Detector>> {
    REGISTRY.lock().ok()?.as_ref()?.map.get(&hid).cloned()
}

#[derive(Clone)]
pub struct AnomalyEvent {
    pub index: u32,
    pub value: f64,
    pub z: f64,
    pub at_ms: i64,
}

const MAX_LOGGED_EVENTS_PER_CHART: usize = 500;

struct EventLog {
    map: HashMap<u64, VecDeque<AnomalyEvent>>,
    order: VecDeque<u64>,
}

static EVENT_LOG: Mutex<Option<EventLog>> = Mutex::new(None);

fn log_event(hid: u64, index: u32, value: f64, z: f64) {
    let Ok(mut guard) = EVENT_LOG.lock() else { return };
    let log = guard.get_or_insert_with(|| EventLog { map: HashMap::new(), order: VecDeque::new() });
    if !log.map.contains_key(&hid) {
        log.order.push_back(hid);
        while log.order.len() > CAPACITY {
            if let Some(old) = log.order.pop_front() {
                log.map.remove(&old);
            }
        }
    }
    let events = log.map.entry(hid).or_insert_with(VecDeque::new);
    events.push_back(AnomalyEvent { index, value, z, at_ms: crate::plot::now_ms() });
    while events.len() > MAX_LOGGED_EVENTS_PER_CHART {
        events.pop_front();
    }
}

pub fn logged_events(hid: u64) -> Vec<AnomalyEvent> {
    let Ok(guard) = EVENT_LOG.lock() else { return Vec::new() };
    guard.as_ref().and_then(|l| l.map.get(&hid)).map(|events| events.iter().cloned().collect()).unwrap_or_default()
}

fn take_last<T>(mut v: Vec<T>, limit: Option<usize>) -> Vec<T> {
    if let Some(n) = limit {
        if v.len() > n {
            v.drain(0..v.len() - n);
        }
    }
    v
}

pub fn check_and_notify(py: Python<'_>, hid: u64, indices: &[u32], values: &[f64]) {
    let Some(det) = get(hid) else { return };
    for (idx, &v) in indices.iter().zip(values.iter()) {
        if let Some(z) = det.check(*idx, v) {
            let _ = det.callback.call1(py, (*idx, v, z));
            super::features::metrics::incr("seraplot_pulse_anomalies_total", hid);
            super::features::alerts::dispatch(hid, *idx, v, z);
            log_event(hid, *idx, v, z);
        }
    }
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (limit = None))]
    #[sera_doc(
        category = "chart_method",
        module = "anomaly",
        file = "seraplot/pulse-pricing.md",
        en = "Returns this chart's own anomaly event log - every anomaly on_anomaly() has fired on, oldest first, each as {index, value, z, at_ms} (up to the last 500 per chart; older ones are dropped as new ones arrive). Useful for a post-incident summary without keeping your own list inside the callback.",
        fr = "Retourne le journal des anomalies propre à ce chart — chaque anomalie déclenchée par on_anomaly(), du plus ancien au plus récent, sous forme de {index, value, z, at_ms} (jusqu'aux 500 dernières par chart ; les plus anciennes sont retirées au fil des nouvelles). Utile pour un résumé post-incident sans tenir sa propre liste dans le callback.",
        param(name = "limit", ty = "int | None", en = "If given, only the most recent `limit` events are returned instead of the full log.", fr = "Si fourni, seuls les `limit` événements les plus récents sont retournés au lieu du journal complet.")
    )]
    fn anomaly_log(&self, py: Python<'_>, limit: Option<usize>) -> PyResult<Py<PyList>> {
        let hid = super::features::require_hid(&self.html)?;
        let events = take_last(logged_events(hid), limit);
        let out = PyList::empty_bound(py);
        for e in events {
            let d = PyDict::new_bound(py);
            d.set_item("index", e.index)?;
            d.set_item("value", e.value)?;
            d.set_item("z", e.z)?;
            d.set_item("at_ms", e.at_ms)?;
            out.append(d)?;
        }
        Ok(out.unbind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_detector(window: usize, threshold: f64, method: Method) -> Detector {
        Python::with_gil(|py| Detector {
            window: window.max(2),
            threshold,
            method,
            state: DetectorState::new(method),
            callback: py.None(),
        })
    }

    #[test]
    fn different_indices_do_not_pollute_each_others_baseline() {
        pyo3::prepare_freethreaded_python();
        let det = make_detector(20, 3.0, Method::Zscore);
        let noisy_baseline = [9.0, 11.0, 10.0, 9.5, 10.5, 9.8, 10.2, 9.6, 10.4, 10.0];
        for &v in &noisy_baseline {
            det.check(0, v);
        }
        for _ in 0..10 {
            det.check(1, 10_000.0);
        }
        let z = det.check(0, 10.1);
        assert!(z.is_none(), "index 0's own noisy-but-stable baseline should not flag another similarly-sized move as anomalous, got {z:?}");
    }

    #[test]
    fn a_real_jump_within_one_indexs_own_history_is_still_detected() {
        pyo3::prepare_freethreaded_python();
        let det = make_detector(20, 3.0, Method::Zscore);
        for _ in 0..10 {
            det.check(5, 10.0);
        }
        let z = det.check(5, 10_000.0);
        assert!(z.is_some(), "a genuine jump in index 5's own history should still be flagged");
    }

    #[test]
    fn per_index_tracking_is_bounded() {
        pyo3::prepare_freethreaded_python();
        let det = make_detector(5, 3.0, Method::Zscore);
        for i in 0..(MAX_TRACKED_INDICES_PER_CHART as u32 + 500) {
            det.check(i, 1.0);
        }
        let DetectorState::Window(state) = &det.state else { panic!("zscore detector should use window state") };
        let state = state.lock().unwrap();
        assert!(state.buffers.len() <= MAX_TRACKED_INDICES_PER_CHART);
    }

    #[test]
    fn overhead_of_check_on_a_warm_index_is_small() {
        pyo3::prepare_freethreaded_python();
        let det = make_detector(20, 3.0, Method::Zscore);
        for i in 0..20 {
            det.check(0, i as f64);
        }
        let n = 100_000u32;
        let started = std::time::Instant::now();
        for i in 0..n {
            det.check(0, (i % 20) as f64);
        }
        let elapsed = started.elapsed();
        println!("Detector::check (warm, single index) x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 500, "check() got too slow: {elapsed:?} for {n} calls");
    }

    #[test]
    fn logged_events_capture_index_value_and_z() {
        let hid = 700_001u64;
        log_event(hid, 3, 42.0, 5.5);
        log_event(hid, 7, 99.0, 6.1);
        let events = logged_events(hid);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].index, 3);
        assert!((events[0].value - 42.0).abs() < 1e-9);
        assert!((events[0].z - 5.5).abs() < 1e-9);
        assert!(events[0].at_ms > 0);
    }

    #[test]
    fn event_log_is_bounded_per_chart() {
        let hid = 700_002u64;
        for i in 0..(MAX_LOGGED_EVENTS_PER_CHART + 100) {
            log_event(hid, i as u32, 1.0, 4.0);
        }
        let events = logged_events(hid);
        assert_eq!(events.len(), MAX_LOGGED_EVENTS_PER_CHART, "event log should drop the oldest entries once the per-chart cap is hit");
        assert_eq!(events[0].index, 100, "the oldest 100 events should have been evicted, first remaining should be index 100");
    }

    #[test]
    fn unknown_chart_has_no_logged_events() {
        assert_eq!(logged_events(799_999_999u64).len(), 0);
    }

    #[test]
    fn overhead_of_anomaly_log_at_full_capacity_is_negligible() {
        let hid = 700_005u64;
        for i in 0..MAX_LOGGED_EVENTS_PER_CHART {
            log_event(hid, i as u32, 1.0, 4.0);
        }
        let n = 50_000u32;
        let started = std::time::Instant::now();
        for _ in 0..n {
            let _ = take_last(logged_events(hid), Some(10));
        }
        let elapsed = started.elapsed();
        println!("anomaly_log(limit=10) at {MAX_LOGGED_EVENTS_PER_CHART}-event cap x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 2000, "anomaly_log() got too slow: {elapsed:?} for {n} calls");
    }

    #[test]
    fn take_last_keeps_only_the_most_recent_n_and_is_a_noop_when_limit_is_none_or_large() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(take_last(v.clone(), None), vec![1, 2, 3, 4, 5]);
        assert_eq!(take_last(v.clone(), Some(2)), vec![4, 5]);
        assert_eq!(take_last(v.clone(), Some(100)), vec![1, 2, 3, 4, 5]);
        assert_eq!(take_last(v.clone(), Some(0)), Vec::<i32>::new());
    }

    #[test]
    fn a_real_anomaly_through_check_and_notify_is_logged() {
        pyo3::prepare_freethreaded_python();
        let hid = 700_003u64;
        Python::with_gil(|py| {
            set(hid, 5, 3.0, Method::Zscore, py.None());
            for _ in 0..10 {
                check_and_notify(py, hid, &[1], &[10.0]);
            }
            check_and_notify(py, hid, &[1], &[10_000.0]);
        });
        let events = logged_events(hid);
        assert!(!events.is_empty(), "a real jump routed through check_and_notify should land in the event log");
        assert_eq!(events.last().unwrap().index, 1);
        clear(hid);
    }

    #[test]
    fn method_parse_accepts_known_names_and_rejects_the_rest() {
        pyo3::prepare_freethreaded_python();
        assert_eq!(Method::parse("zscore").unwrap(), Method::Zscore);
        assert_eq!(Method::parse("iqr").unwrap(), Method::Iqr);
        assert_eq!(Method::parse("seasonal").unwrap(), Method::Seasonal);
        let err = Method::parse("weekly").unwrap_err();
        assert!(Python::with_gil(|py| err.value_bound(py).to_string().contains("unknown anomaly detection method")));
    }

    #[test]
    fn iqr_does_not_flag_values_within_the_recent_spread_but_still_flags_a_real_jump() {
        pyo3::prepare_freethreaded_python();
        let det = make_detector(10, 3.0, Method::Iqr);
        let stable = [10.0, 11.0, 9.0, 10.5, 9.5, 10.2, 9.8, 10.1, 9.9, 10.3];
        for &v in &stable {
            det.check(0, v);
        }
        let inside_fence = det.check(0, 10.4);
        assert!(inside_fence.is_none(), "a value close to the recent spread should not be flagged by IQR, got {inside_fence:?}");

        let det2 = make_detector(10, 3.0, Method::Iqr);
        for &v in &stable {
            det2.check(0, v);
        }
        let real_jump = det2.check(0, 500.0);
        assert!(real_jump.is_some(), "a genuine large jump should still be flagged by IQR");
    }

    #[test]
    fn iqr_still_flags_a_moderate_anomaly_that_zscore_masks_after_one_wild_outlier() {
        pyo3::prepare_freethreaded_python();
        let contaminated = [10.0, 10.2, 9.8, 10.1, 9.9, 50_000.0];
        let moderately_anomalous = 15.0;

        let zscore_det = make_detector(6, 3.0, Method::Zscore);
        for &v in &contaminated {
            zscore_det.check(0, v);
        }
        let zscore_result = zscore_det.check(0, moderately_anomalous);

        let iqr_det = make_detector(6, 3.0, Method::Iqr);
        for &v in &contaminated {
            iqr_det.check(0, v);
        }
        let iqr_result = iqr_det.check(0, moderately_anomalous);

        assert!(zscore_result.is_none(), "z-score should be masked by the single huge outlier and miss the moderate anomaly, got {zscore_result:?}");
        assert!(iqr_result.is_some(), "IQR should still catch the moderate anomaly despite the same outlier being in the window, got {iqr_result:?}");
    }

    #[test]
    fn overhead_of_iqr_check_on_a_warm_index_is_small() {
        pyo3::prepare_freethreaded_python();
        let det = make_detector(20, 3.0, Method::Iqr);
        for i in 0..20 {
            det.check(0, i as f64);
        }
        let n = 100_000u32;
        let started = std::time::Instant::now();
        for i in 0..n {
            det.check(0, (i % 20) as f64);
        }
        let elapsed = started.elapsed();
        println!("Detector::check IQR (warm, single index) x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 1000, "IQR check() got too slow: {elapsed:?} for {n} calls");
    }

    #[test]
    fn hour_of_day_wraps_correctly_across_epoch_and_dst_agnostic_boundaries() {
        assert_eq!(hour_of_day(0), 0);
        assert_eq!(hour_of_day(3_600_000), 1);
        assert_eq!(hour_of_day(23 * 3_600_000), 23);
        assert_eq!(hour_of_day(24 * 3_600_000), 0);
        assert_eq!(hour_of_day(25 * 3_600_000), 1);
        assert_eq!(hour_of_day(3_600_000 - 1), 0);
    }

    #[test]
    fn seasonal_tracks_each_hour_of_day_independently() {
        let state = Mutex::new(SeasonalIndexState { buckets: HashMap::new(), order: VecDeque::new() });
        for v in [9.0, 11.0, 10.0, 9.5, 10.5, 9.8, 10.2] {
            check_seasonal_with_hour(&state, 20, 3.0, 0, v, 5);
        }
        for v in [990.0, 1010.0, 1000.0, 995.0, 1005.0, 998.0, 1002.0] {
            check_seasonal_with_hour(&state, 20, 3.0, 0, v, 10);
        }

        let normal_for_hour_10 = check_seasonal_with_hour(&state, 20, 3.0, 0, 1001.0, 10);
        assert!(normal_for_hour_10.is_none(), "1001 should look normal against hour 10's own ~1000 baseline, got {normal_for_hour_10:?}");

        let anomalous_for_hour_5 = check_seasonal_with_hour(&state, 20, 3.0, 0, 1001.0, 5);
        assert!(anomalous_for_hour_5.is_some(), "the same value 1001 should be flagged against hour 5's own ~10 baseline, even though it's normal for hour 10");
    }

    #[test]
    fn seasonal_still_detects_a_real_jump_within_a_single_hours_own_history() {
        let state = Mutex::new(SeasonalIndexState { buckets: HashMap::new(), order: VecDeque::new() });
        for _ in 0..10 {
            check_seasonal_with_hour(&state, 20, 3.0, 3, 10.0, 8);
        }
        let jump = check_seasonal_with_hour(&state, 20, 3.0, 3, 10_000.0, 8);
        assert!(jump.is_some(), "a genuine jump within hour 8's own history should still be flagged");
    }

    #[test]
    fn seasonal_per_index_tracking_is_bounded() {
        let state = Mutex::new(SeasonalIndexState { buckets: HashMap::new(), order: VecDeque::new() });
        for i in 0..(MAX_TRACKED_INDICES_PER_CHART as u32 + 500) {
            check_seasonal_with_hour(&state, 5, 3.0, i, 1.0, 0);
        }
        let locked = state.lock().unwrap();
        assert!(locked.buckets.len() <= MAX_TRACKED_INDICES_PER_CHART);
    }

    #[test]
    fn seasonal_detector_via_check_routes_through_the_current_real_hour() {
        pyo3::prepare_freethreaded_python();
        let hid = 700_004u64;
        Python::with_gil(|py| {
            set(hid, 5, 3.0, Method::Seasonal, py.None());
            for _ in 0..10 {
                check_and_notify(py, hid, &[9], &[10.0]);
            }
            check_and_notify(py, hid, &[9], &[10_000.0]);
        });
        let events = logged_events(hid);
        assert!(!events.is_empty(), "a real jump within the current hour's own seasonal history should still be logged");
        clear(hid);
    }

    #[test]
    fn overhead_of_seasonal_check_on_a_warm_index_is_small() {
        let state = Mutex::new(SeasonalIndexState { buckets: HashMap::new(), order: VecDeque::new() });
        for i in 0..20 {
            check_seasonal_with_hour(&state, 20, 3.0, 0, i as f64, 12);
        }
        let n = 100_000u32;
        let started = std::time::Instant::now();
        for i in 0..n {
            check_seasonal_with_hour(&state, 20, 3.0, 0, (i % 20) as f64, 12);
        }
        let elapsed = started.elapsed();
        println!("check_seasonal_with_hour (warm, single index/hour) x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 500, "seasonal check got too slow: {elapsed:?} for {n} calls");
    }
}
