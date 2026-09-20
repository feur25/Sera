use super::common::{bin_to_edges, compute_bins, group_indices};
use super::config::HistogramConfig;
use super::variant::HistogramVariant;
use crate::plot::statistical::_3d::generic::{grouped_columns, plate_columns, transposed};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.8;
const BIN_HW: f64 = 0.46;
const BIN_DEPTH: f64 = 0.42;
const PLATE_SHARE: f64 = 0.02;

#[derive(Clone, Copy)]
enum Measure {
    Count,
    Density,
    Cumulative,
}

#[derive(Clone, Copy)]
enum Arrangement {
    Single,
    Stacked,
    Overlay,
}

#[derive(Clone, Copy)]
struct Recipe {
    measure: Measure,
    arrangement: Arrangement,
    plates: bool,
    swap: bool,
}

impl Recipe {
    const fn of(measure: Measure) -> Self {
        Self { measure, arrangement: Arrangement::Single, plates: false, swap: false }
    }

    const fn arranged(mut self, arrangement: Arrangement) -> Self {
        self.arrangement = arrangement;
        self
    }

    const fn plated(mut self) -> Self {
        self.plates = true;
        self
    }

    const fn swapped(mut self) -> Self {
        self.swap = true;
        self
    }
}

fn recipe(variant: HistogramVariant) -> Recipe {
    use HistogramVariant::*;
    let count = Recipe::of(Measure::Count);
    match variant {
        Basic => count,
        Horizontal => count.swapped(),
        Normalized => Recipe::of(Measure::Density),
        Cumulative => Recipe::of(Measure::Cumulative),
        Stacked => count.arranged(Arrangement::Stacked),
        Overlay => count.arranged(Arrangement::Overlay),
        Step => count.plated(),
    }
}

fn counted(cfg: &HistogramConfig, edges: &[f64]) -> Vec<(String, Vec<u64>)> {
    if !cfg.categories.is_empty() {
        let n = cfg.values.len();
        let (groups, idx) = group_indices(cfg.categories, n);
        let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); groups.len().max(1)];
        for (i, &v) in cfg.values.iter().enumerate() {
            let slot = idx.get(i).copied().unwrap_or(0).min(buckets.len() - 1);
            buckets[slot].push(v);
        }
        return groups
            .into_iter()
            .zip(buckets)
            .map(|(name, bucket)| (name, bin_to_edges(&bucket, edges).0))
            .collect();
    }
    let (first, second) = match cfg.series_names {
        Some((a, b)) => (a.to_string(), b.to_string()),
        None => ("A".to_string(), "B".to_string()),
    };
    match cfg.overlay_values {
        Some(overlay) => vec![
            (first, bin_to_edges(cfg.values, edges).0),
            (second, bin_to_edges(overlay, edges).0),
        ],
        None => vec![(first, bin_to_edges(cfg.values, edges).0)],
    }
}

fn measured(counts: &[u64], measure: Measure, width: f64) -> Vec<f64> {
    let total: f64 = counts.iter().sum::<u64>() as f64;
    match measure {
        Measure::Count => counts.iter().map(|&c| c as f64).collect(),
        Measure::Density => counts.iter().map(|&c| c as f64 / (total.max(1.0) * width.max(1e-12))).collect(),
        Measure::Cumulative => counts
            .iter()
            .scan(0.0, |acc, &c| {
                *acc += c as f64;
                Some(*acc)
            })
            .collect(),
    }
}

pub fn layout_3d(cfg: &HistogramConfig) -> Vec<Bar3DBlock> {
    if cfg.values.is_empty() {
        return Vec::new();
    }
    let (_, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = edges.len().saturating_sub(1);
    if n_bins == 0 {
        return Vec::new();
    }
    let plan = recipe(cfg.variant);
    let width = (edges[n_bins] - edges[0]) / n_bins as f64;
    let series: Vec<(String, Vec<f64>)> = counted(cfg, &edges)
        .into_iter()
        .map(|(name, counts)| (name, measured(&counts, plan.measure, width)))
        .collect();
    let mut blocks = if plan.plates {
        let peak = series.iter().flat_map(|(_, v)| v.iter().cloned()).fold(1e-12, f64::max);
        series
            .iter()
            .enumerate()
            .flat_map(|(s, (_, values))| {
                plate_columns(values, peak * PLATE_SHARE, 0.5, BIN_DEPTH)
                    .into_iter()
                    .map(move |b| Bar3DBlock { ci: s, ..b })
            })
            .collect()
    } else {
        let stacked = matches!(plan.arrangement, Arrangement::Stacked);
        grouped_columns(&series, n_bins, stacked, BIN_HW, BIN_DEPTH)
    };
    if plan.swap {
        blocks = transposed(blocks);
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: [f64; 12] = [1.0, 1.5, 2.0, 2.2, 2.4, 3.0, 3.1, 3.3, 4.0, 4.2, 5.0, 5.5];

    fn blocks_for(variant: HistogramVariant) -> Vec<Bar3DBlock> {
        layout_3d(&HistogramConfig {
            variant,
            values: &VALUES,
            bins: 4,
            ..HistogramConfig::default()
        })
    }

    #[test]
    fn every_histogram_variant_draws_something() {
        for variant in HistogramVariant::all() {
            assert!(!blocks_for(*variant).is_empty(), "{} must draw blocks", variant.name());
        }
    }

    #[test]
    fn basic_draws_one_column_per_bin_and_counts_every_sample() {
        let blocks = blocks_for(HistogramVariant::Basic);
        assert_eq!(blocks.len(), 4);
        assert_eq!(blocks.iter().map(|b| b.z1).sum::<f64>(), 12.0);
    }

    #[test]
    fn cumulative_grows_to_the_sample_count_and_normalized_integrates_to_one() {
        let cumulative = blocks_for(HistogramVariant::Cumulative);
        assert_eq!(cumulative.last().unwrap().z1, 12.0);
        assert!(cumulative.windows(2).all(|w| w[1].z1 >= w[0].z1));
        let density = blocks_for(HistogramVariant::Normalized);
        let width = (VALUES[11] - VALUES[0]) / 4.0;
        assert!((density.iter().map(|b| b.z1 * width).sum::<f64>() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn stacked_and_overlay_split_the_samples_by_group_or_overlay_series() {
        let groups: Vec<String> = (0..12).map(|i| if i % 2 == 0 { "A" } else { "B" }.to_string()).collect();
        let stacked = layout_3d(&HistogramConfig {
            variant: HistogramVariant::Stacked,
            values: &VALUES,
            bins: 4,
            categories: &groups,
            ..HistogramConfig::default()
        });
        assert_eq!(stacked.len(), 8);
        let overlay = layout_3d(&HistogramConfig {
            variant: HistogramVariant::Overlay,
            values: &VALUES,
            overlay_values: Some(&VALUES),
            bins: 4,
            ..HistogramConfig::default()
        });
        assert_eq!(overlay.len(), 8);
        assert!(overlay.iter().any(|b| b.cy < 0.0) && overlay.iter().any(|b| b.cy > 0.0));
    }

    #[test]
    fn step_is_a_row_of_thin_plates_and_horizontal_a_quarter_turn() {
        let plates = blocks_for(HistogramVariant::Step);
        assert!(plates.iter().all(|b| b.z1 - b.z0 < 0.5));
        assert_eq!(blocks_for(HistogramVariant::Basic)[2].cx, blocks_for(HistogramVariant::Horizontal)[2].cy);
    }

    #[test]
    fn empty_values_draw_nothing() {
        assert!(layout_3d(&HistogramConfig::default()).is_empty());
    }
}
