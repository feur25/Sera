use super::config::CorrelogramConfig;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.5;
pub const COLORMAP: &str = "updown";
const HW: f64 = 0.42;
const HD: f64 = 0.42;

fn correlation_columns(n: usize, matrix: &[f64]) -> Vec<Bar3DBlock> {
    (0..n)
        .flat_map(|r| (0..n).map(move |c| (r, c)))
        .enumerate()
        .filter_map(|(idx, (r, c))| {
            matrix.get(r * n + c).map(|&raw| {
                let v = if raw.is_finite() { raw.clamp(-1.0, 1.0) } else { 0.0 };
                let (z0, z1) = if v >= 0.0 { (0.0, v) } else { (v, 0.0) };
                Bar3DBlock::new(c as f64, r as f64, z0, z1, HW, HD, idx)
            })
        })
        .collect()
}

pub fn layout_3d(cfg: &CorrelogramConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &CorrelogramConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len();
    if n == 0 || cfg.matrix.len() < n * n {
        return (Vec::new(), Vec::new());
    }
    let blocks = correlation_columns(n, cfg.matrix);
    let names: Vec<String> = (0..n).flat_map(|r| (0..n).map(move |c| (r, c))).map(|(r, c)| format!("{} \u{d7} {}", cfg.labels[r], cfg.labels[c])).collect();
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::statistical::CorrelogramVariant;

    fn labels() -> Vec<String> {
        ["A", "B", "C"].iter().map(|s| s.to_string()).collect()
    }

    fn matrix() -> Vec<f64> {
        vec![1.0, 0.6, -0.4, 0.6, 1.0, -0.2, -0.4, -0.2, 1.0]
    }

    fn draw(variant: CorrelogramVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let l = labels();
        let m = matrix();
        let cfg = CorrelogramConfig { variant, labels: &l, matrix: &m, ..CorrelogramConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_correlogram_variant_draws_the_full_matrix_and_names_every_cell() {
        for &variant in CorrelogramVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 9, "{variant:?}");
            assert_eq!(names.len(), 9, "{variant:?}");
            assert!(names.contains(&"A \u{d7} B".to_string()));
        }
    }

    #[test]
    fn positive_and_negative_correlations_grow_in_opposite_directions() {
        let (blocks, _) = draw(CorrelogramVariant::Circle);
        assert!(blocks[1].z1 > 0.0 && blocks[1].z0 == 0.0);
        assert!(blocks[2].z0 < 0.0 && blocks[2].z1 == 0.0);
    }

    #[test]
    fn the_diagonal_is_a_perfect_positive_correlation() {
        let (blocks, _) = draw(CorrelogramVariant::Circle);
        assert_eq!(blocks[0].z1, 1.0);
        assert_eq!(blocks[4].z1, 1.0);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&CorrelogramConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let l = labels();
        let m = vec![1.0, f64::NAN, f64::INFINITY, f64::NAN, 1.0, -f64::INFINITY, f64::INFINITY, -f64::INFINITY, 1.0];
        let cfg = CorrelogramConfig { labels: &l, matrix: &m, ..CorrelogramConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite()));
    }
}
