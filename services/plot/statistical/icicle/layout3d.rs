use super::common::{prepare, Prepared};
use super::config::IcicleConfig;
use super::variant::IcicleVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::generic::transposed;
use crate::plot::statistical::_3d::hierarchy::{depth_cap, radians_from_unit, rings, tiers, BASE_HEIGHT};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.7;
const NODE_CAP: usize = 600;
const GAP_UNITS: f64 = 0.006;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Tiers,
    Sideways,
    Rings,
}

#[derive(Clone, Copy)]
struct Recipe {
    shape: Shape,
    gap: f64,
    ranked: bool,
}

impl Recipe {
    const fn of(shape: Shape) -> Self {
        Self { shape, gap: 0.0, ranked: false }
    }

    const fn gapped(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    const fn ranked(mut self) -> Self {
        self.ranked = true;
        self
    }
}

fn recipe(variant: IcicleVariant) -> Recipe {
    use IcicleVariant::*;
    match variant {
        Basic => Recipe::of(Shape::Tiers),
        Gapped => Recipe::of(Shape::Tiers).gapped(GAP_UNITS),
        Horizontal => Recipe::of(Shape::Sideways),
        Radial => Recipe::of(Shape::Rings),
        Rank => Recipe::of(Shape::Tiers).ranked(),
    }
}

pub fn colormap(variant: IcicleVariant) -> &'static str {
    if recipe(variant).ranked { "viridis" } else { "jet" }
}

fn narrowed(spans: &[(f64, f64)], gap: f64) -> Vec<(f64, f64)> {
    spans
        .iter()
        .map(|&(x0, x1)| if (x1 - x0).abs() > gap * 2.5 { (x0 + gap, x1 - gap) } else { (x0, x1) })
        .collect()
}

fn branch_tone(p: &Prepared, i: usize) -> f64 {
    let n = p.roots.len().max(1);
    p.cidx[i] as f64 / n.max(2) as f64
}

fn sibling_rank(p: &Prepared) -> Vec<f64> {
    let max_depth = p.depth.iter().copied().max().unwrap_or(0);
    let mut rank_frac = vec![0.0f64; p.n];
    for d in 0..=max_depth {
        let mut idxs: Vec<usize> = (0..p.n).filter(|&i| p.depth[i] == d).collect();
        idxs.sort_by(|&a, &b| p.values_eff[a].total_cmp(&p.values_eff[b]));
        let m = idxs.len();
        for (rank, &i) in idxs.iter().enumerate() {
            rank_frac[i] = if m > 1 { rank as f64 / (m as f64 - 1.0) } else { 1.0 };
        }
    }
    rank_frac
}

fn finite(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { *v } else { 0.0 }).collect()
}

fn icicle_3d(cfg: &IcicleConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let values = finite(cfg.values);
    let sound = IcicleConfig { labels: cfg.labels, parents: cfg.parents, values: &values, palette: cfg.palette, ..IcicleConfig::default() };
    let Some(p) = prepare(&sound) else {
        return (Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let cap_depth = depth_cap(&p.depth, NODE_CAP);
    let ranks = sibling_rank(&p);
    let tone_of = |i: usize| if plan.ranked { ranks[i] } else { branch_tone(&p, i) };
    let blocks = match plan.shape {
        Shape::Tiers => tiers(&p.bfs_order, &p.depth, &narrowed(&p.xspan, plan.gap), cap_depth, |_| BASE_HEIGHT, tone_of),
        Shape::Sideways => transposed(tiers(&p.bfs_order, &p.depth, &p.xspan, cap_depth, |_| BASE_HEIGHT, tone_of)),
        Shape::Rings => {
            let radians: Vec<(f64, f64)> = p.xspan.iter().map(|&s| radians_from_unit(s)).collect();
            rings(&p.bfs_order, &p.depth, &radians, cap_depth, crate::plot::statistical::_3d::hierarchy::HOLE, |_| BASE_HEIGHT, tone_of)
        }
    };
    let names = p.bfs_order.iter().filter(|&&i| p.depth[i] <= cap_depth).map(|&i| p.labels[i].clone()).collect();
    (blocks, names)
}

pub fn layout_3d(cfg: &IcicleConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &IcicleConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.labels.is_empty() {
        return (Vec::new(), Vec::new());
    }
    icicle_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> (Vec<String>, Vec<String>, Vec<f64>) {
        (
            ["Company", "Engineering", "Sales", "Backend", "Frontend"].iter().map(|s| s.to_string()).collect(),
            ["", "Company", "Company", "Engineering", "Engineering"].iter().map(|s| s.to_string()).collect(),
            vec![0.0, 40.0, 60.0, 25.0, 15.0],
        )
    }

    fn draw(variant: IcicleVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, parents, values) = tree();
        let cfg = IcicleConfig { variant, labels: &labels, parents: &parents, values: &values, ..IcicleConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_every_node_and_names_it() {
        for &variant in IcicleVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 5, "{variant:?}");
            if variant == IcicleVariant::Radial {
                assert!(blocks.len() >= 5, "{variant:?}");
            } else {
                assert_eq!(blocks.len(), 5, "{variant:?}");
            }
        }
    }

    #[test]
    fn tiers_stack_rows_by_depth_and_horizontal_swaps_the_row_axis() {
        let vertical = draw(IcicleVariant::Basic).0;
        assert!(vertical.iter().any(|b| b.cy > 0.0) && vertical.iter().any(|b| b.cy == 0.0));
        let sideways = draw(IcicleVariant::Horizontal).0;
        assert!(sideways.iter().any(|b| b.cx > 0.0) && sideways.iter().any(|b| b.cx == 0.0));
    }

    #[test]
    fn gapped_tiers_are_narrower_than_the_basic_ones() {
        let basic = draw(IcicleVariant::Basic).0;
        let gapped = draw(IcicleVariant::Gapped).0;
        let width_of = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).sum::<f64>();
        assert!(width_of(&gapped) < width_of(&basic));
    }

    #[test]
    fn radial_wraps_the_tree_around_a_ring_instead_of_a_straight_axis() {
        let (radial, _) = draw(IcicleVariant::Radial);
        assert!(radial.iter().all(|b| b.cx.hypot(b.cy) > 0.0));
        assert_eq!(colormap(IcicleVariant::Radial), "jet");
    }

    #[test]
    fn rank_tones_siblings_by_their_percentile_within_the_same_depth() {
        let (blocks, names) = draw(IcicleVariant::Rank);
        let backend = names.iter().position(|n| n == "Backend").unwrap();
        let frontend = names.iter().position(|n| n == "Frontend").unwrap();
        assert!(blocks[backend].tone.unwrap() > blocks[frontend].tone.unwrap());
        assert_eq!(colormap(IcicleVariant::Rank), "viridis");
    }

    #[test]
    fn a_deep_wide_tree_is_capped_by_depth() {
        let mut labels = vec!["Root".to_string()];
        let mut parents = vec![String::new()];
        let mut values = vec![0.0];
        for d in 0..10 {
            for k in 0..70 {
                labels.push(format!("d{d}n{k}"));
                parents.push(if d == 0 { "Root".to_string() } else { format!("d{}n{}", d - 1, k % 70) });
                values.push(1.0);
            }
        }
        let cfg = IcicleConfig { labels: &labels, parents: &parents, values: &values, ..IcicleConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty() && blocks.len() < labels.len());
        assert_eq!(blocks.len(), names.len());
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&IcicleConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_values_are_handled_by_the_shared_hierarchy_preparation() {
        let labels = vec!["Root".to_string(), "A".to_string()];
        let parents = vec![String::new(), "Root".to_string()];
        let values = vec![f64::NAN, f64::INFINITY];
        let cfg = IcicleConfig { labels: &labels, parents: &parents, values: &values, ..IcicleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.hw.is_finite()));
    }
}
