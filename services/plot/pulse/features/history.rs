use crate::plot::push_registry::PushMeta;
use crate::plot::record_registry::{self, Record};
use crate::Chart;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use seraplot_macros::sera_doc_impl;
use std::collections::HashMap;

fn descale(meta: &PushMeta, raw: &[i16]) -> Vec<f64> {
    match meta {
        PushMeta::Point(p) => {
            let x = p.min_x + raw[0] as f64 * p.range_x / p.plot_w as f64;
            let y = p.min_y + (p.plot_h as f64 - raw[1] as f64) * p.range_y / p.plot_h as f64;
            vec![x, y]
        }
        PushMeta::Scalar(s) => vec![s.min_val + raw[0] as f64 * s.range_val / s.axis_px as f64],
        PushMeta::Vector(v) => raw.iter().map(|&r| v.min_val + r as f64 * v.range_val / v.axis_px as f64).collect(),
    }
}

fn final_state(meta: &PushMeta, records: &[Record]) -> HashMap<u32, Vec<f64>> {
    let vpp = meta.values_per_point() as usize;
    let mut out: HashMap<u32, Vec<f64>> = HashMap::new();
    for rec in records {
        let n = rec.indices.len().min(rec.n as usize);
        for i in 0..n {
            let start = i * vpp;
            let end = start + vpp;
            if end > rec.values.len() {
                continue;
            }
            out.insert(rec.indices[i], descale(meta, &rec.values[start..end]));
        }
    }
    out
}

pub struct IndexDelta {
    pub index: u32,
    pub baseline: Vec<f64>,
    pub current: Vec<f64>,
    pub abs_delta: f64,
    pub pct_change: f64,
}

pub struct Comparison {
    pub matched: usize,
    pub added: usize,
    pub removed: usize,
    pub mean_abs_delta: f64,
    pub max_abs_delta: f64,
    pub mean_pct_change: f64,
    pub top_changes: Vec<IndexDelta>,
}

const TOP_CHANGES_CAP: usize = 20;

pub(crate) fn compare(baseline: &HashMap<u32, Vec<f64>>, current: &HashMap<u32, Vec<f64>>) -> Comparison {
    let mut deltas: Vec<IndexDelta> = Vec::with_capacity(baseline.len().min(current.len()));
    let mut removed = 0usize;
    for (idx, base_vals) in baseline {
        let Some(cur_vals) = current.get(idx) else {
            removed += 1;
            continue;
        };
        let n = base_vals.len().min(cur_vals.len()).max(1);
        let abs_delta = base_vals.iter().zip(cur_vals.iter()).map(|(b, c)| (c - b).abs()).sum::<f64>() / n as f64;
        let base_mag = base_vals.iter().map(|v| v.abs()).sum::<f64>() / n as f64;
        let pct_change = if base_mag > 1e-9 { abs_delta / base_mag * 100.0 } else { 0.0 };
        deltas.push(IndexDelta { index: *idx, baseline: base_vals.clone(), current: cur_vals.clone(), abs_delta, pct_change });
    }
    let added = current.keys().filter(|k| !baseline.contains_key(k)).count();
    let matched = deltas.len();
    let mean_abs_delta = if matched > 0 { deltas.iter().map(|d| d.abs_delta).sum::<f64>() / matched as f64 } else { 0.0 };
    let max_abs_delta = deltas.iter().map(|d| d.abs_delta).fold(0.0_f64, f64::max);
    let mean_pct_change = if matched > 0 { deltas.iter().map(|d| d.pct_change).sum::<f64>() / matched as f64 } else { 0.0 };
    deltas.sort_by(|a, b| b.abs_delta.partial_cmp(&a.abs_delta).unwrap_or(std::cmp::Ordering::Equal));
    deltas.truncate(TOP_CHANGES_CAP);
    Comparison { matched, added, removed, mean_abs_delta, max_abs_delta, mean_pct_change, top_changes: deltas }
}

#[derive(Debug)]
pub(crate) struct LoadedRecording {
    pub state: HashMap<u32, Vec<f64>>,
    pub started_at_ms: Option<i64>,
    pub ended_at_ms: Option<i64>,
}

pub(crate) fn load_final_state(path: &str) -> PyResult<LoadedRecording> {
    let recording = record_registry::read_all(path)
        .map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to read {path}: {e}")))?;
    let scale = recording.scale.ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "{path} was recorded before scale metadata was embedded (pre-v2 .spls format) and can't be compared quantitatively; re-record it to compare"
        ))
    })?;
    let ended_at_ms = recording
        .started_at_ms
        .map(|started| started + recording.records.last().map(|r| r.dt_ms as i64).unwrap_or(0));
    let state = final_state(&scale, &recording.records);
    Ok(LoadedRecording { state, started_at_ms: recording.started_at_ms, ended_at_ms })
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        module = "history",
        file = "seraplot/pulse-pricing.md",
        aliases("history_diff"),
        en = "Compares the final state of two .spls recordings (e.g. this week vs last week) index by index, descaling back to real data values using each recording's embedded scale. Returns matched/added/removed counts, aggregate delta stats, and the biggest individual changes.",
        fr = "Compare l'état final de deux enregistrements .spls (ex : cette semaine vs la semaine dernière) indice par indice, en redonnant les vraies valeurs de données via l'échelle embarquée dans chaque enregistrement. Retourne les comptes appariés/ajoutés/supprimés, des statistiques de delta agrégées, et les plus gros changements individuels.",
        param(name = "baseline_path", ty = "str", en = "Path to the baseline .spls recording.", fr = "Chemin de l'enregistrement .spls de référence."),
        param(name = "current_path", ty = "str", en = "Path to the .spls recording to compare against the baseline.", fr = "Chemin de l'enregistrement .spls à comparer à la référence.")
    )]
    fn compare_history(&self, py: Python<'_>, baseline_path: &str, current_path: &str) -> PyResult<Py<PyDict>> {
        let baseline = load_final_state(baseline_path)?;
        let current = load_final_state(current_path)?;
        let result = compare(&baseline.state, &current.state);

        let dict = PyDict::new_bound(py);
        dict.set_item("matched", result.matched)?;
        dict.set_item("added", result.added)?;
        dict.set_item("removed", result.removed)?;
        dict.set_item("mean_abs_delta", result.mean_abs_delta)?;
        dict.set_item("max_abs_delta", result.max_abs_delta)?;
        dict.set_item("mean_pct_change", result.mean_pct_change)?;
        let top = PyList::empty_bound(py);
        for d in &result.top_changes {
            let item = PyDict::new_bound(py);
            item.set_item("index", d.index)?;
            item.set_item("baseline", d.baseline.clone())?;
            item.set_item("current", d.current.clone())?;
            item.set_item("abs_delta", d.abs_delta)?;
            item.set_item("pct_change", d.pct_change)?;
            top.append(item)?;
        }
        dict.set_item("top_changes", top)?;
        Ok(dict.unbind())
    }

    fn history_diff(&self, py: Python<'_>, baseline_path: &str, current_path: &str) -> PyResult<Py<PyDict>> {
        self.compare_history(py, baseline_path, current_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::push_registry::{PointMeta, ScalarMeta, VectorMeta};

    fn rec(dt_ms: u32, indices: Vec<u32>, values: Vec<i16>, vpp: u32) -> Record {
        Record { dt_ms, n: indices.len() as u32, vpp, indices, values }
    }

    #[test]
    fn scalar_descale_matches_pack_scalar_i16_raw() {
        let meta = PushMeta::Scalar(ScalarMeta { min_val: 10.0, range_val: 90.0, axis_px: 300 });
        let original = vec![10.0, 55.0, 100.0];
        let packed = crate::plot::canvas_points::pack_scalar_i16_raw(&original, 10.0, 90.0, 300);
        let raw: Vec<i16> = packed.chunks_exact(2).map(|c| i16::from_le_bytes(c.try_into().unwrap())).collect();
        for (i, &r) in raw.iter().enumerate() {
            let back = descale(&meta, &[r])[0];
            assert!((back - original[i]).abs() < 1.0, "descale({r}) = {back}, expected close to {}", original[i]);
        }
    }

    #[test]
    fn final_state_keeps_last_value_per_index() {
        let meta = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let records = vec![rec(0, vec![0, 1], vec![10, 20], 1), rec(100, vec![0], vec![50], 1)];
        let state = final_state(&meta, &records);
        assert_eq!(state.len(), 2);
        assert!((state[&0][0] - 50.0).abs() < 1.0);
        assert!((state[&1][0] - 20.0).abs() < 1.0);
    }

    #[test]
    fn compare_detects_added_removed_and_changed() {
        let mut baseline = HashMap::new();
        baseline.insert(0u32, vec![10.0]);
        baseline.insert(1u32, vec![20.0]);
        let mut current = HashMap::new();
        current.insert(0u32, vec![15.0]);
        current.insert(2u32, vec![99.0]);

        let result = compare(&baseline, &current);
        assert_eq!(result.matched, 1);
        assert_eq!(result.added, 1);
        assert_eq!(result.removed, 1);
        assert!((result.mean_abs_delta - 5.0).abs() < 1e-9);
        assert_eq!(result.top_changes[0].index, 0);
    }

    #[test]
    fn vector_descale_uses_shared_scale_across_components() {
        let meta = PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 200.0, axis_px: 200, vpp: 3 });
        let out = descale(&meta, &[0, 100, 200]);
        assert_eq!(out.len(), 3);
        assert!((out[0] - 0.0).abs() < 1.0);
        assert!((out[1] - 100.0).abs() < 1.0);
        assert!((out[2] - 200.0).abs() < 1.0);
    }

    #[test]
    fn point_descale_inverts_y_flip() {
        let meta = PushMeta::Point(PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 100.0, plot_w: 100, plot_h: 100 });
        let out = descale(&meta, &[0, 0]);
        assert!((out[1] - 100.0).abs() < 1.0, "top pixel row should descale near max y, got {}", out[1]);
    }

    #[test]
    fn end_to_end_file_round_trip_and_compare_perf() {
        let base_path = std::env::temp_dir().join(format!("sp_history_test_base_{}.spls", std::process::id()));
        let cur_path = std::env::temp_dir().join(format!("sp_history_test_cur_{}.spls", std::process::id()));
        let n = 50_000u32;

        let meta = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 1000.0, axis_px: 1000 });
        for (path, hid, offset) in [(&base_path, 10u64, 0i16), (&cur_path, 11u64, 25i16)] {
            record_registry::start(hid, path.to_str().unwrap(), &meta).expect("start");
            let state = record_registry::get(hid).expect("registered");
            let indices: Vec<u32> = (0..n).collect();
            let values: Vec<i16> = (0..n).map(|i| ((i % 900) as i16 + offset).clamp(0, 999)).collect();
            let mut frame = Vec::with_capacity(12 + indices.len() * 4 + values.len() * 2);
            frame.extend_from_slice(&(hid as u32).to_le_bytes());
            frame.extend_from_slice(&n.to_le_bytes());
            frame.extend_from_slice(&1u32.to_le_bytes());
            for i in &indices {
                frame.extend_from_slice(&i.to_le_bytes());
            }
            for v in &values {
                frame.extend_from_slice(&v.to_le_bytes());
            }
            state.append(&frame);
            record_registry::stop(hid);
        }

        let started = std::time::Instant::now();
        let baseline = load_final_state(base_path.to_str().unwrap()).expect("load baseline");
        let current = load_final_state(cur_path.to_str().unwrap()).expect("load current");
        let result = compare(&baseline.state, &current.state);
        let elapsed = started.elapsed();

        let _ = std::fs::remove_file(&base_path);
        let _ = std::fs::remove_file(&cur_path);

        assert_eq!(result.matched, n as usize);
        assert_eq!(result.added, 0);
        assert_eq!(result.removed, 0);
        assert!(result.mean_abs_delta > 0.0);
        println!("compare_history: {n} indices, read+descale+compare in {elapsed:?}");
        assert!(elapsed.as_millis() < 2000, "compare_history took too long for {n} indices: {elapsed:?}");
    }

    #[test]
    fn missing_scale_reports_clear_error_not_garbage_numbers() {
        pyo3::prepare_freethreaded_python();
        let path = std::env::temp_dir().join(format!("sp_history_test_v1_{}.spls", std::process::id()));
        {
            use std::io::Write;
            let mut f = std::fs::File::create(&path).unwrap();
            f.write_all(b"SPLS").unwrap();
            f.write_all(&[1u8, 0, 0, 0]).unwrap();
        }
        let err = load_final_state(path.to_str().unwrap()).unwrap_err();
        let _ = std::fs::remove_file(&path);
        assert!(Python::with_gil(|py| err.value_bound(py).to_string().contains("pre-v2")));
    }

    fn scalar_wire_frame(hid: u64, indices: &[u32], values: &[i16]) -> Vec<u8> {
        let n = indices.len() as u32;
        let mut frame = Vec::with_capacity(12 + indices.len() * 4 + values.len() * 2);
        frame.extend_from_slice(&(hid as u32).to_le_bytes());
        frame.extend_from_slice(&n.to_le_bytes());
        frame.extend_from_slice(&1u32.to_le_bytes());
        for i in indices {
            frame.extend_from_slice(&i.to_le_bytes());
        }
        for v in values {
            frame.extend_from_slice(&v.to_le_bytes());
        }
        frame
    }

    #[test]
    fn load_final_state_exposes_a_real_wall_clock_window() {
        let path = std::env::temp_dir().join(format!("sp_history_test_window_{}.spls", std::process::id()));
        let meta = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let before = crate::plot::now_ms();
        record_registry::start(20, path.to_str().unwrap(), &meta).expect("start");
        let state = record_registry::get(20).expect("registered");
        state.append(&scalar_wire_frame(20, &[0], &[10]));
        std::thread::sleep(std::time::Duration::from_millis(5));
        state.append(&scalar_wire_frame(20, &[1], &[20]));
        record_registry::stop(20);
        let after = crate::plot::now_ms();

        let loaded = load_final_state(path.to_str().unwrap()).expect("load");
        let _ = std::fs::remove_file(&path);

        let started = loaded.started_at_ms.expect("fresh recordings should have a wall-clock anchor");
        let ended = loaded.ended_at_ms.expect("ended_at_ms should be derivable when an anchor is present");
        assert!(started >= before && started <= after, "started_at_ms {started} should fall within [{before}, {after}]");
        assert!(ended >= started, "ended_at_ms {ended} should be at or after started_at_ms {started}");
    }

    #[test]
    fn missing_wall_clock_anchor_is_reported_as_none_not_a_guess() {
        let path = std::env::temp_dir().join(format!("sp_history_test_v2_noanchor_{}.spls", std::process::id()));
        {
            use std::io::Write;
            let mut f = std::fs::File::create(&path).unwrap();
            f.write_all(b"SPLS").unwrap();
            f.write_all(&[2u8, 0, 0, 0]).unwrap();
            f.write_all(&[1u8]).unwrap();
            f.write_all(&0.0f64.to_le_bytes()).unwrap();
            f.write_all(&100.0f64.to_le_bytes()).unwrap();
            f.write_all(&100i32.to_le_bytes()).unwrap();
        }
        let loaded = load_final_state(path.to_str().unwrap()).expect("v2 files without an anchor still load");
        let _ = std::fs::remove_file(&path);
        assert!(loaded.started_at_ms.is_none());
        assert!(loaded.ended_at_ms.is_none(), "no anchor means no derivable end time either, not a fabricated one");
    }
}
