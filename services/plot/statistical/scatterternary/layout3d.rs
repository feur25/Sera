use super::config::ScatterTernaryConfig;
use super::variant::ScatterTernaryVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.5;
pub const COLORMAP: &str = "jet";
const BASE_SIZE: f64 = 0.09;
const BASE_HEIGHT: f64 = 0.2;
const BUBBLE_MAX: f64 = 0.32;
const SCALE: f64 = 8.0;

fn ternary_xy(a: f64, b: f64, c: f64) -> (f64, f64) {
    let total = (a + b + c).max(1e-9);
    let (a, b, c) = (a / total, b / total, c / total);
    (0.5 * (2.0 * b + c) * SCALE, (3f64.sqrt() / 2.0) * c * SCALE)
}

pub fn layout_3d(cfg: &ScatterTernaryConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &ScatterTernaryConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.a_values.len().min(cfg.b_values.len()).min(cfg.c_values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let keep = even_indices(n, budget.cloud());
    let a = pick(cfg.a_values, &keep);
    let b = pick(cfg.b_values, &keep);
    let c = pick(cfg.c_values, &keep);
    let cv = pick(cfg.color_values, &keep);
    let labels = pick(cfg.labels, &keep);
    let bubble = matches!(cfg.variant, ScatterTernaryVariant::Bubble);
    let (lo, hi) = cv.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    let range = (hi - lo).max(1e-9);
    let blocks: Vec<Bar3DBlock> = (0..keep.len())
        .map(|i| {
            let (x, y) = ternary_xy(a[i], b[i], c[i]);
            let frac = if lo.is_finite() && cv.len() == keep.len() { ((cv[i] - lo) / range).clamp(0.0, 1.0) } else { 0.5 };
            let size = if bubble { BASE_SIZE + frac * (BUBBLE_MAX - BASE_SIZE) } else { BASE_SIZE };
            let block = Bar3DBlock::new(x, y, 0.0, BASE_HEIGHT, size, size, i);
            if cv.len() == keep.len() { block.with_tone(frac) } else { block }
        })
        .collect();
    let names: Vec<String> = if labels.len() == keep.len() { labels } else { (0..keep.len()).map(|i| format!("Point {}", i + 1)).collect() };
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(variant: ScatterTernaryVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let a = vec![0.7, 0.2, 0.1, 0.4, 0.33];
        let b = vec![0.2, 0.6, 0.1, 0.3, 0.33];
        let c = vec![0.1, 0.2, 0.8, 0.3, 0.34];
        let cfg = ScatterTernaryConfig { variant, a_values: &a, b_values: &b, c_values: &c, ..ScatterTernaryConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_scatterternary_variant_draws_every_point() {
        for &variant in ScatterTernaryVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 5, "{variant:?}");
            assert_eq!(names.len(), 5, "{variant:?}");
        }
    }

    #[test]
    fn a_pure_corner_point_lands_at_the_matching_triangle_vertex() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let c = vec![0.0, 0.0, 1.0];
        let cfg = ScatterTernaryConfig { a_values: &a, b_values: &b, c_values: &c, ..ScatterTernaryConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!((blocks[0].cx).abs() < 1e-6 && (blocks[0].cy).abs() < 1e-6);
        assert!((blocks[2].cy - SCALE * 3f64.sqrt() / 2.0).abs() < 1e-6);
    }

    #[test]
    fn bubble_scales_marker_size_with_the_color_value() {
        let a = vec![0.7, 0.2];
        let b = vec![0.2, 0.6];
        let c = vec![0.1, 0.2];
        let cv = vec![0.0, 1.0];
        let cfg = ScatterTernaryConfig { variant: ScatterTernaryVariant::Bubble, a_values: &a, b_values: &b, c_values: &c, color_values: &cv, ..ScatterTernaryConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks[1].hw > blocks[0].hw);
    }

    #[test]
    fn labels_reach_the_returned_names_when_present() {
        let a = vec![0.5, 0.5];
        let b = vec![0.3, 0.3];
        let c = vec![0.2, 0.2];
        let labels = vec!["Alpha".to_string(), "Beta".to_string()];
        let cfg = ScatterTernaryConfig { a_values: &a, b_values: &b, c_values: &c, labels: &labels, ..ScatterTernaryConfig::default() };
        let (_, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, labels);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&ScatterTernaryConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_point_cloud_is_capped_by_the_budget() {
        let a: Vec<f64> = (0..300_000).map(|i| (i % 100) as f64).collect();
        let b: Vec<f64> = (0..300_000).map(|i| ((i * 3) % 100) as f64).collect();
        let c: Vec<f64> = (0..300_000).map(|i| ((i * 7) % 100) as f64).collect();
        let cfg = ScatterTernaryConfig { a_values: &a, b_values: &b, c_values: &c, ..ScatterTernaryConfig::default() };
        let budget = Budget::new(Some(600));
        let (blocks, _) = layout_named(&cfg, &budget);
        assert_eq!(blocks.len(), budget.cloud());
    }
}
