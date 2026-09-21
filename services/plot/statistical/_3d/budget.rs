use super::super::bar::Bar3DBlock;

pub const DEFAULT_POINTS: usize = 1500;
pub const MAX_POINTS: usize = 4000;
pub const MIN_POINTS: usize = 8;
pub const MIN_CELLS: usize = 64;
pub const MAX_CELLS: usize = 12000;
pub const SAMPLE_CAP: usize = 300;
pub const GROUP_CAP: usize = 400;
pub const HARD_BLOCKS: usize = 12000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Budget {
    pub points: usize,
}

impl Budget {
    pub fn new(max_points: Option<usize>) -> Self {
        Self { points: max_points.filter(|p| *p >= MIN_POINTS).unwrap_or(DEFAULT_POINTS).min(MAX_POINTS) }
    }

    pub fn cells(&self) -> usize {
        (self.points * 3).clamp(MIN_CELLS, MAX_CELLS)
    }
}

impl Default for Budget {
    fn default() -> Self {
        Self::new(None)
    }
}

#[derive(Clone, Debug)]
pub struct Buckets {
    spans: Vec<(usize, usize)>,
    identity: bool,
}

impl Buckets {
    pub fn new(n: usize, target: usize) -> Self {
        if target == 0 || n <= target {
            return Self { spans: (0..n).map(|i| (i, i + 1)).collect(), identity: true };
        }
        Self { spans: (0..target).map(|k| (k * n / target, (k + 1) * n / target)).collect(), identity: false }
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_identity(&self) -> bool {
        self.identity
    }

    pub fn mean(&self, values: &[f64]) -> Vec<f64> {
        self.spans
            .iter()
            .map(|&(a, b)| {
                let (sum, count) = values[a.min(values.len())..b.min(values.len())]
                    .iter()
                    .filter(|v| v.is_finite())
                    .fold((0.0, 0usize), |(s, c), v| (s + v, c + 1));
                if count == 0 { 0.0 } else { sum / count as f64 }
            })
            .collect()
    }

    pub fn sum(&self, values: &[f64]) -> Vec<f64> {
        self.spans
            .iter()
            .map(|&(a, b)| values[a.min(values.len())..b.min(values.len())].iter().filter(|v| v.is_finite()).sum())
            .collect()
    }

    pub fn high(&self, values: &[f64]) -> Vec<f64> {
        self.spans
            .iter()
            .map(|&(a, b)| {
                values[a.min(values.len())..b.min(values.len())]
                    .iter()
                    .cloned()
                    .filter(|v| v.is_finite())
                    .fold(f64::NEG_INFINITY, f64::max)
            })
            .map(|v| if v.is_finite() { v } else { 0.0 })
            .collect()
    }

    pub fn low(&self, values: &[f64]) -> Vec<f64> {
        self.spans
            .iter()
            .map(|&(a, b)| {
                values[a.min(values.len())..b.min(values.len())]
                    .iter()
                    .cloned()
                    .filter(|v| v.is_finite())
                    .fold(f64::INFINITY, f64::min)
            })
            .map(|v| if v.is_finite() { v } else { 0.0 })
            .collect()
    }

    pub fn first<T: Clone>(&self, items: &[T]) -> Vec<T> {
        self.spans.iter().filter_map(|&(a, _)| items.get(a).cloned()).collect()
    }

    pub fn last<T: Clone>(&self, items: &[T]) -> Vec<T> {
        self.spans.iter().filter_map(|&(_, b)| items.get(b.saturating_sub(1).min(items.len().saturating_sub(1))).cloned()).collect()
    }
}

pub fn pooled_grid(n_rows: usize, n_cols: usize, cells: &[f64], max_cells: usize) -> (usize, usize, Vec<f64>) {
    let total = n_rows * n_cols;
    if total <= max_cells || max_cells == 0 || cells.len() < total {
        return (n_rows, n_cols, cells.to_vec());
    }
    let stride = ((total as f64 / max_cells as f64).sqrt().ceil() as usize).max(1);
    let (out_rows, out_cols) = (n_rows.div_ceil(stride), n_cols.div_ceil(stride));
    let out = (0..out_rows)
        .flat_map(|r| (0..out_cols).map(move |c| (r, c)))
        .map(|(r, c)| {
            let (r1, c1) = ((r * stride + stride).min(n_rows), (c * stride + stride).min(n_cols));
            let (sum, count) = (r * stride..r1)
                .flat_map(|row| (c * stride..c1).map(move |col| cells[row * n_cols + col]))
                .filter(|v| v.is_finite())
                .fold((0.0, 0usize), |(s, n), v| (s + v, n + 1));
            if count == 0 { 0.0 } else { sum / count as f64 }
        })
        .collect();
    (out_rows, out_cols, out)
}

pub fn quantile_sample(samples: &[f64], cap: usize) -> Vec<f64> {
    let mut sorted: Vec<f64> = samples.iter().copied().filter(|v| v.is_finite()).collect();
    sorted.sort_by(|a, b| a.total_cmp(b));
    if cap == 0 || sorted.len() <= cap {
        return sorted;
    }
    (0..cap).map(|i| sorted[((i as f64 + 0.5) * sorted.len() as f64 / cap as f64) as usize]).collect()
}

pub fn even_indices(n: usize, cap: usize) -> Vec<usize> {
    if cap == 0 || n <= cap {
        return (0..n).collect();
    }
    (0..cap).map(|i| (i * n / cap).min(n - 1)).collect()
}

fn is_sound(b: &Bar3DBlock) -> bool {
    [b.cx, b.cy, b.z0, b.z1, b.hw, b.hd].iter().all(|v| v.is_finite())
        && b.end.map(|(a, c)| a.is_finite() && c.is_finite()).unwrap_or(true)
        && b.tone.map(|t| t.is_finite()).unwrap_or(true)
}

pub fn sound(blocks: &[Bar3DBlock]) -> Vec<Bar3DBlock> {
    blocks.iter().copied().filter(is_sound).collect()
}

pub fn thin(blocks: Vec<Bar3DBlock>, cap: usize) -> Vec<Bar3DBlock> {
    if cap == 0 || blocks.len() <= cap {
        return blocks;
    }
    let window = blocks.len().div_ceil(cap);
    blocks
        .chunks(window)
        .filter_map(|chunk| chunk.iter().copied().max_by(|a, b| (a.z1 - a.z0).abs().total_cmp(&(b.z1 - b.z0).abs())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budgets_clamp_the_requested_point_count() {
        assert_eq!(Budget::new(None).points, DEFAULT_POINTS);
        assert_eq!(Budget::new(Some(2)).points, DEFAULT_POINTS);
        assert_eq!(Budget::new(Some(100)).points, 100);
        assert_eq!(Budget::new(Some(usize::MAX)).points, MAX_POINTS);
        assert_eq!(Budget::default().cells(), DEFAULT_POINTS * 3);
        assert_eq!(Budget::new(Some(8)).cells(), MIN_CELLS);
        assert_eq!(Budget::new(Some(MAX_POINTS)).cells(), MAX_CELLS);
    }

    #[test]
    fn small_inputs_keep_their_identity_and_large_ones_are_pooled_evenly() {
        let same = Buckets::new(10, 100);
        assert!(same.is_identity() && same.len() == 10);
        let pooled = Buckets::new(1000, 10);
        assert!(!pooled.is_identity() && pooled.len() == 10);
        let values: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        assert_eq!(pooled.sum(&values).iter().sum::<f64>(), values.iter().sum::<f64>());
        assert_eq!(pooled.mean(&values)[0], 49.5);
        assert_eq!(pooled.high(&values)[9], 999.0);
        assert_eq!(pooled.low(&values)[9], 900.0);
    }

    #[test]
    fn pooling_keeps_the_first_and_last_item_of_each_bucket() {
        let pooled = Buckets::new(6, 3);
        let names = ["a", "b", "c", "d", "e", "f"];
        assert_eq!(pooled.first(&names), vec!["a", "c", "e"]);
        assert_eq!(pooled.last(&names), vec!["b", "d", "f"]);
    }

    #[test]
    fn non_finite_values_never_poison_a_bucket() {
        let pooled = Buckets::new(4, 2);
        assert_eq!(pooled.mean(&[1.0, f64::NAN, f64::INFINITY, 3.0]), vec![1.0, 3.0]);
        assert_eq!(pooled.high(&[f64::NAN, f64::NAN, 2.0, 1.0]), vec![0.0, 2.0]);
    }

    #[test]
    fn grids_are_mean_pooled_to_the_cell_budget() {
        let cells: Vec<f64> = (0..10_000).map(|i| (i % 100) as f64).collect();
        let (rows, cols, out) = pooled_grid(100, 100, &cells, 2500);
        assert_eq!((rows, cols, out.len()), (50, 50, 2500));
        assert_eq!(out[0], 0.5);
        assert_eq!(pooled_grid(10, 10, &cells[..100], 2500).2.len(), 100);
    }

    #[test]
    fn samples_keep_their_distribution_when_capped() {
        let samples: Vec<f64> = (0..10_000).map(|i| i as f64).collect();
        let kept = quantile_sample(&samples, 100);
        assert_eq!(kept.len(), 100);
        assert!(kept.windows(2).all(|w| w[0] < w[1]));
        assert!(kept[0] < 100.0 && kept[99] > 9900.0);
        assert_eq!(quantile_sample(&[3.0, f64::NAN, 1.0], 100), vec![1.0, 3.0]);
        assert_eq!(even_indices(1000, 4), vec![0, 250, 500, 750]);
    }

    #[test]
    fn unsound_blocks_are_dropped_and_thinning_keeps_the_tallest_of_each_window() {
        let good = Bar3DBlock::new(0.0, 0.0, 0.0, 1.0, 0.4, 0.4, 0);
        let bad = Bar3DBlock::new(f64::NAN, 0.0, 0.0, 1.0, 0.4, 0.4, 0);
        let infinite = Bar3DBlock::new(0.0, 0.0, 0.0, f64::INFINITY, 0.4, 0.4, 0);
        assert_eq!(sound(&[good, bad, infinite]).len(), 1);
        let blocks: Vec<Bar3DBlock> = (0..100).map(|i| Bar3DBlock::new(i as f64, 0.0, 0.0, (i % 10) as f64, 0.4, 0.4, i)).collect();
        let kept = thin(blocks, 10);
        assert_eq!(kept.len(), 10);
        assert!(kept.iter().all(|b| b.z1 == 9.0));
    }
}
