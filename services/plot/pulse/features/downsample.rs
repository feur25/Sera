use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 256;
const MAX_TRACKED_INDICES_PER_CHART: usize = 4096;

#[derive(Clone, Copy)]
struct Aggregate {
    sum: f64,
    min: f64,
    max: f64,
    count: u64,
}

impl Aggregate {
    fn one(value: f64) -> Self {
        Aggregate { sum: value, min: value, max: value, count: 1 }
    }

    fn absorb(&mut self, value: f64) {
        self.sum += value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.count += 1;
    }

    fn mean(&self) -> f64 {
        self.sum / self.count as f64
    }
}

struct ChartState {
    enabled: bool,
    pending: HashMap<u32, Aggregate>,
    order: VecDeque<u32>,
}

struct Registry {
    map: HashMap<u64, ChartState>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn set_enabled(hid: u64, enabled: bool) {
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
    let state = reg.map.entry(hid).or_insert_with(|| ChartState { enabled: false, pending: HashMap::new(), order: VecDeque::new() });
    state.enabled = enabled;
    if !enabled {
        state.pending.clear();
        state.order.clear();
    }
}

pub fn is_enabled(hid: u64) -> bool {
    let Ok(guard) = REGISTRY.lock() else { return false };
    guard.as_ref().and_then(|r| r.map.get(&hid)).map(|s| s.enabled).unwrap_or(false)
}

pub fn absorb(hid: u64, indices: &[u32], values: &[f64]) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let Some(reg) = guard.as_mut() else { return };
    let Some(state) = reg.map.get_mut(&hid) else { return };
    for (&idx, &value) in indices.iter().zip(values.iter()) {
        if !state.pending.contains_key(&idx) {
            state.order.push_back(idx);
            while state.order.len() > MAX_TRACKED_INDICES_PER_CHART {
                if let Some(old) = state.order.pop_front() {
                    state.pending.remove(&old);
                }
            }
        }
        state.pending.entry(idx).and_modify(|a| a.absorb(value)).or_insert_with(|| Aggregate::one(value));
    }
}

pub fn take_pending(hid: u64, indices: &[u32]) -> Vec<(u32, f64)> {
    let Ok(mut guard) = REGISTRY.lock() else { return Vec::new() };
    let Some(reg) = guard.as_mut() else { return Vec::new() };
    let Some(state) = reg.map.get_mut(&hid) else { return Vec::new() };
    let mut out = Vec::new();
    for &idx in indices {
        if let Some(agg) = state.pending.remove(&idx) {
            out.push((idx, agg.mean()));
        }
    }
    out
}

pub fn clear(hid: u64) {
    if let Ok(mut guard) = REGISTRY.lock() {
        if let Some(reg) = guard.as_mut() {
            reg.map.remove(&hid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_by_default_and_absorb_is_a_no_op() {
        let hid = 5_000_001u64;
        assert!(!is_enabled(hid));
        absorb(hid, &[1], &[42.0]);
        assert_eq!(take_pending(hid, &[1]), Vec::new(), "absorb() before set_enabled(true) should not create any state");
    }

    #[test]
    fn enabling_and_disabling_round_trips() {
        let hid = 5_000_002u64;
        set_enabled(hid, true);
        assert!(is_enabled(hid));
        set_enabled(hid, false);
        assert!(!is_enabled(hid));
        clear(hid);
    }

    #[test]
    fn absorbed_values_are_averaged_into_a_single_mean() {
        let hid = 5_000_003u64;
        set_enabled(hid, true);
        absorb(hid, &[7], &[10.0]);
        absorb(hid, &[7], &[20.0]);
        absorb(hid, &[7], &[30.0]);
        let pending = take_pending(hid, &[7]);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0, 7);
        assert!((pending[0].1 - 20.0).abs() < 1e-9, "mean of 10/20/30 should be 20, got {}", pending[0].1);
        clear(hid);
    }

    #[test]
    fn take_pending_clears_what_it_returns_but_leaves_the_rest() {
        let hid = 5_000_004u64;
        set_enabled(hid, true);
        absorb(hid, &[1, 2], &[100.0, 200.0]);
        let first = take_pending(hid, &[1]);
        assert_eq!(first, vec![(1, 100.0)]);
        assert_eq!(take_pending(hid, &[1]), Vec::new(), "index 1 should be empty now that it was taken");
        assert_eq!(take_pending(hid, &[2]), vec![(2, 200.0)], "index 2 should be untouched by taking index 1");
        clear(hid);
    }

    #[test]
    fn disabling_clears_pending_aggregates() {
        let hid = 5_000_005u64;
        set_enabled(hid, true);
        absorb(hid, &[9], &[1.0]);
        set_enabled(hid, false);
        set_enabled(hid, true);
        assert_eq!(take_pending(hid, &[9]), Vec::new(), "toggling off should discard whatever was pending, not carry it silently into a later re-enable");
        clear(hid);
    }

    #[test]
    fn per_index_tracking_is_bounded() {
        let hid = 5_000_006u64;
        set_enabled(hid, true);
        for i in 0..(MAX_TRACKED_INDICES_PER_CHART as u32 + 500) {
            absorb(hid, &[i], &[1.0]);
        }
        let Ok(guard) = REGISTRY.lock() else { panic!("lock") };
        let count = guard.as_ref().unwrap().map[&hid].pending.len();
        assert!(count <= MAX_TRACKED_INDICES_PER_CHART);
        drop(guard);
        clear(hid);
    }

    #[test]
    fn overhead_of_absorb_is_negligible() {
        let hid = 5_000_007u64;
        set_enabled(hid, true);
        let n = 100_000u32;
        let idx = [0u32];
        let started = std::time::Instant::now();
        for i in 0..n {
            absorb(hid, &idx, &[i as f64]);
        }
        let elapsed = started.elapsed();
        println!("downsample::absorb x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 500, "absorb() got too slow: {elapsed:?} for {n} calls");
        clear(hid);
    }
}
