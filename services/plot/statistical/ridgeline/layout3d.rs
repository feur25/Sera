use super::config::RidgelineConfig;
use super::variant::RidgelineVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::grid::{ridge_cells, CellField};
use crate::plot::statistical::_3d::spread::grouped_by_label;
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::boxplot::common::compute_box;
use crate::plot::statistical::kde::{kde_eval, scott_bw};

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "jet";
const N_POINTS: usize = 48;
const FILL: f64 = 1.05;
const THIN_FILL: f64 = 0.18;
const THICKNESS: f64 = 0.4;
const ROW_GAP: f64 = 0.62;
const SPACED_GAP: f64 = 1.3;
const PLATE_TONE: f64 = 0.5;
const RUG_HEIGHT: f64 = 0.12;
const HEAT_CMAP: &str = "viridis";

#[derive(Clone, Copy, PartialEq)]
enum Overlay {
    None,
    Quartiles,
    Mean,
    Rug,
}

#[derive(Clone, Copy)]
struct Recipe {
    overlay: Overlay,
    thin: bool,
    heat: bool,
    row_gap: f64,
}

impl Recipe {
    const fn of(overlay: Overlay) -> Self {
        Self { overlay, thin: false, heat: false, row_gap: ROW_GAP }
    }

    const fn thinned(mut self) -> Self {
        self.thin = true;
        self
    }

    const fn heated(mut self) -> Self {
        self.heat = true;
        self
    }

    const fn spaced(mut self, gap: f64) -> Self {
        self.row_gap = gap;
        self
    }
}

fn recipe(variant: RidgelineVariant) -> Recipe {
    use RidgelineVariant::*;
    match variant {
        Basic => Recipe::of(Overlay::None),
        Lines => Recipe::of(Overlay::None).thinned(),
        Quartiles => Recipe::of(Overlay::Quartiles),
        Mean => Recipe::of(Overlay::Mean),
        Rug => Recipe::of(Overlay::Rug).thinned(),
        Heatmap => Recipe::of(Overlay::None).heated(),
        Spaced => Recipe::of(Overlay::None).spaced(SPACED_GAP),
    }
}

pub fn colormap(variant: RidgelineVariant) -> &'static str {
    if recipe(variant).heat { HEAT_CMAP } else { COLORMAP }
}

fn densities(groups: &[(String, Vec<f64>)], lo: f64, hi: f64) -> Vec<Vec<f64>> {
    let range = (hi - lo).max(1e-9);
    groups
        .iter()
        .map(|(_, samples)| {
            let bw = scott_bw(samples).max(range * 0.01);
            (0..N_POINTS).map(|c| kde_eval(samples, lo + range * c as f64 / (N_POINTS - 1).max(1) as f64, bw)).collect()
        })
        .collect()
}

fn plate(row: usize, col: f64, row_gap: f64) -> Bar3DBlock {
    Bar3DBlock::new(col, row as f64 * row_gap, 0.0, THICKNESS * 1.6, THICKNESS * 0.7, THICKNESS * 0.7, row).with_tone(PLATE_TONE)
}

pub fn layout_3d(cfg: &RidgelineConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &RidgelineConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let groups = grouped_by_label(cfg.categories, cfg.values);
    if groups.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let names: Vec<String> = groups.iter().map(|(name, _)| name.clone()).collect();
    let all: Vec<f64> = groups.iter().flat_map(|(_, v)| v.iter().copied()).filter(|v| v.is_finite()).collect();
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if !lo.is_finite() || !hi.is_finite() {
        return (Vec::new(), Vec::new());
    }
    let rows = densities(&groups, lo, hi);
    let n_rows = rows.len();
    let heights: Vec<f64> = rows.iter().flatten().copied().collect();
    let peak = heights.iter().cloned().fold(1e-12, f64::max);
    let tones: Vec<f64> = if plan.heat { heights.iter().map(|h| (h / peak).clamp(0.0, 1.0)).collect() } else { Vec::new() };
    let classes: Vec<usize> = (0..n_rows).flat_map(|r| std::iter::repeat(r).take(N_POINTS)).collect();
    let field = CellField { n_rows, n_cols: N_POINTS, heights: &heights, tones: &tones, classes: &classes };
    let fill = if plan.thin { THIN_FILL } else { FILL };
    let mut blocks = ridge_cells(&field, plan.row_gap, fill, THICKNESS);
    let step = (hi - lo).max(1e-9) / (N_POINTS - 1).max(1) as f64;
    match plan.overlay {
        Overlay::None => {}
        Overlay::Quartiles => {
            for (r, (_, samples)) in groups.iter().enumerate() {
                let s = compute_box(samples);
                for v in [s.q1, s.median, s.q3] {
                    blocks.push(plate(r, (v - lo) / step, plan.row_gap));
                }
            }
        }
        Overlay::Mean => {
            for (r, (_, samples)) in groups.iter().enumerate() {
                let mean = compute_box(samples).mean;
                blocks.push(plate(r, (mean - lo) / step, plan.row_gap));
            }
        }
        Overlay::Rug => {
            for (r, (_, samples)) in groups.iter().enumerate() {
                for &v in samples.iter().filter(|v| v.is_finite()) {
                    let col = (v - lo) / step;
                    blocks.push(Bar3DBlock::new(col, r as f64 * plan.row_gap, 0.0, RUG_HEIGHT, THIN_FILL * 0.3, THICKNESS * 0.5, r));
                }
            }
        }
    }
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn categories() -> Vec<String> {
        (0..30).map(|i| if i < 10 { "A" } else if i < 20 { "B" } else { "C" }.to_string()).collect()
    }

    fn values() -> Vec<f64> {
        (0..30).map(|i| ((i * 37) % 23) as f64 + (i / 10) as f64 * 5.0).collect()
    }

    fn draw(variant: RidgelineVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let c = categories();
        let v = values();
        let cfg = RidgelineConfig { variant, categories: &c, values: &v, ..RidgelineConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_ridgeline_variant_draws_something_and_names_every_category() {
        for &variant in RidgelineVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string(), "C".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn each_category_sits_on_its_own_depth_row() {
        let (blocks, _) = draw(RidgelineVariant::Basic);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0).round() as i64).collect();
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn spaced_pushes_the_rows_further_apart_than_basic() {
        let (basic, _) = draw(RidgelineVariant::Basic);
        let (spaced, _) = draw(RidgelineVariant::Spaced);
        let max_row = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.cy).fold(0.0, f64::max);
        assert!(max_row(&spaced) > max_row(&basic));
    }

    #[test]
    fn lines_uses_a_much_thinner_footprint_than_basic() {
        let (basic, _) = draw(RidgelineVariant::Basic);
        let (lines, _) = draw(RidgelineVariant::Lines);
        assert!(lines[0].hw < basic[0].hw);
    }

    #[test]
    fn heatmap_tones_every_cell_by_its_own_density() {
        let (blocks, _) = draw(RidgelineVariant::Heatmap);
        assert!(blocks.iter().any(|b| b.tone.is_some()));
        let tones: std::collections::BTreeSet<i64> = blocks.iter().filter_map(|b| b.tone).map(|t| (t * 1000.0) as i64).collect();
        assert!(tones.len() > 3);
    }

    #[test]
    fn quartiles_and_mean_add_marker_plates_per_category() {
        let (basic, _) = draw(RidgelineVariant::Basic);
        let (quartiles, _) = draw(RidgelineVariant::Quartiles);
        let (mean, _) = draw(RidgelineVariant::Mean);
        assert_eq!(quartiles.len(), basic.len() + 3 * 3);
        assert_eq!(mean.len(), basic.len() + 3);
    }

    #[test]
    fn rug_adds_one_tick_per_raw_sample() {
        let (basic, _) = draw(RidgelineVariant::Rug);
        let ticks = basic.iter().filter(|b| b.z1 == RUG_HEIGHT).count();
        assert_eq!(ticks, 30);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&RidgelineConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_sample_set_still_produces_a_bounded_number_of_blocks() {
        let categories: Vec<String> = (0..200_000).map(|i| format!("C{}", i % 12)).collect();
        let values: Vec<f64> = (0..200_000).map(|i| ((i * 7919) % 10_007) as f64).collect();
        let cfg = RidgelineConfig { categories: &categories, values: &values, ..RidgelineConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 12 * N_POINTS + 100);
    }
}
