pub fn lttb_indices(y: &[f64], threshold: usize) -> Vec<usize> {
    let n = y.len();
    if threshold >= n || threshold < 3 || n < 3 {
        return (0..n).collect();
    }

    let mut sampled: Vec<usize> = Vec::with_capacity(threshold);
    sampled.push(0);

    let bucket_size = (n - 2) as f64 / (threshold - 2) as f64;
    let mut a = 0usize;

    for i in 0..threshold - 2 {
        let avg_range_start = ((i + 1) as f64 * bucket_size) as usize + 1;
        let avg_range_end = (((i + 2) as f64 * bucket_size) as usize + 1).min(n);
        let avg_range_len = avg_range_end.saturating_sub(avg_range_start).max(1) as f64;

        let mut avg_x = 0.0f64;
        let mut avg_y = 0.0f64;
        for j in avg_range_start..avg_range_end {
            avg_x += j as f64;
            avg_y += y[j];
        }
        avg_x /= avg_range_len;
        avg_y /= avg_range_len;

        let range_offs = (i as f64 * bucket_size) as usize + 1;
        let range_to = (((i + 1) as f64 * bucket_size) as usize + 1).min(n);
        let point_ax = a as f64;
        let point_ay = y[a];

        let mut max_area = -1.0f64;
        let mut max_area_idx = range_offs.min(n - 1);
        for j in range_offs..range_to {
            let area = ((point_ax - avg_x) * (y[j] - point_ay) - (point_ax - j as f64) * (avg_y - point_ay)).abs() * 0.5;
            if area > max_area {
                max_area = area;
                max_area_idx = j;
            }
        }
        sampled.push(max_area_idx);
        a = max_area_idx;
    }

    sampled.push(n - 1);
    sampled
}

pub fn apply_indices<T: Clone>(data: &[T], idx: &[usize]) -> Vec<T> {
    idx.iter().map(|&i| data[i].clone()).collect()
}

pub fn combined_magnitude(series: &[&[f64]], n: usize) -> Vec<f64> {
    let mut out = vec![0.0f64; n];
    for s in series {
        for (i, v) in out.iter_mut().enumerate() {
            *v += s.get(i).copied().unwrap_or(0.0).abs();
        }
    }
    out
}

pub struct Decimator {
    idx: Option<Vec<usize>>,
    n: usize,
}

impl Decimator {
    pub fn new(max_points: Option<usize>, primary: &[f64]) -> Self {
        let n = primary.len();
        let idx = match max_points {
            Some(mp) if mp > 0 && mp < n => Some(lttb_indices(primary, mp)),
            _ => None,
        };
        Decimator { idx, n }
    }

    pub fn for_series(max_points: Option<usize>, series: &[(String, Vec<f64>)]) -> Self {
        let n = series.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
        let refs: Vec<&[f64]> = series.iter().map(|(_, v)| v.as_slice()).collect();
        let magnitude = combined_magnitude(&refs, n);
        Self::new(max_points, &magnitude)
    }

    pub fn active(&self) -> bool {
        self.idx.is_some()
    }

    pub fn apply<T: Clone>(&self, data: Vec<T>) -> Vec<T> {
        match &self.idx {
            Some(idx) if data.len() >= self.n => apply_indices(&data, idx),
            _ => data,
        }
    }

    pub fn apply_each(&self, series: Vec<(String, Vec<f64>)>) -> Vec<(String, Vec<f64>)> {
        match &self.idx {
            Some(idx) => series
                .into_iter()
                .map(|(name, vals)| {
                    let v = if vals.len() >= self.n {
                        apply_indices(&vals, idx)
                    } else {
                        vals
                    };
                    (name, v)
                })
                .collect(),
            None => series,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_endpoints_and_reduces_count() {
        let y: Vec<f64> = (0..10_000).map(|i| (i as f64 * 0.01).sin()).collect();
        let idx = lttb_indices(&y, 500);
        assert_eq!(idx.len(), 500);
        assert_eq!(idx[0], 0);
        assert_eq!(*idx.last().unwrap(), y.len() - 1);
        for w in idx.windows(2) {
            assert!(w[0] < w[1]);
        }
    }

    #[test]
    fn no_op_below_threshold() {
        let y = vec![1.0, 2.0, 3.0];
        let idx = lttb_indices(&y, 10);
        assert_eq!(idx, vec![0, 1, 2]);
    }

    #[test]
    fn preserves_a_sharp_spike() {
        let mut y = vec![0.0f64; 5000];
        y[2500] = 1000.0;
        let idx = lttb_indices(&y, 200);
        assert!(idx.iter().any(|&i| (i as i64 - 2500).abs() <= 2));
    }

    #[test]
    fn decimator_inactive_without_max_points() {
        let y: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let dec = Decimator::new(None, &y);
        assert!(!dec.active());
        let x: Vec<f64> = (0..1000).map(|i| i as f64 * 2.0).collect();
        assert_eq!(dec.apply(x.clone()), x);
    }

    #[test]
    fn decimator_syncs_parallel_arrays() {
        let y: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let labels: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
        let dec = Decimator::new(Some(100), &y);
        assert!(dec.active());
        let y2 = dec.apply(y);
        let labels2 = dec.apply(labels);
        assert_eq!(y2.len(), 100);
        assert_eq!(labels2.len(), 100);
        for (v, l) in y2.iter().zip(labels2.iter()) {
            assert_eq!(*v, l.parse::<f64>().unwrap());
        }
    }

    #[test]
    fn decimator_for_series_syncs_multi_series() {
        let series = vec![
            ("A".to_string(), (0..2000).map(|i| (i as f64 * 0.01).sin()).collect()),
            ("B".to_string(), (0..2000).map(|i| (i as f64 * 0.02).cos()).collect()),
        ];
        let dec = Decimator::for_series(Some(150), &series);
        let out = dec.apply_each(series);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].1.len(), 150);
        assert_eq!(out[1].1.len(), 150);
    }
}
