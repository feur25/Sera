use super::common::{prepare, Prepared};
use super::config::SunburstConfig;
use super::variant::SunburstVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::{depth_cap, faded_height, rings, BASE_HEIGHT, HOLE};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "jet";
const NODE_CAP: usize = 600;
const DONUT_HOLE: f64 = HOLE * 2.0;
const THIN_HEIGHT: f64 = BASE_HEIGHT * 0.4;
const MONO_TONE: f64 = 0.55;
const GAP_RADIANS: f64 = 0.02;

#[derive(Clone, Copy)]
struct Recipe {
    hole: f64,
    height: f64,
    fade: bool,
    mono: bool,
    gap: f64,
}

impl Recipe {
    const fn of() -> Self {
        Self { hole: HOLE, height: BASE_HEIGHT, fade: false, mono: false, gap: 0.0 }
    }

    const fn holed(mut self, hole: f64) -> Self {
        self.hole = hole;
        self
    }

    const fn thinned(mut self) -> Self {
        self.height = THIN_HEIGHT;
        self
    }

    const fn fading(mut self) -> Self {
        self.fade = true;
        self
    }

    const fn monotone(mut self) -> Self {
        self.mono = true;
        self
    }

    const fn gapped(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }
}

fn recipe(variant: SunburstVariant) -> Recipe {
    use SunburstVariant::*;
    match variant {
        Basic | Zoomable => Recipe::of(),
        Donut => Recipe::of().holed(DONUT_HOLE),
        Outlined => Recipe::of().thinned(),
        Gapped => Recipe::of().gapped(GAP_RADIANS),
        DepthFade => Recipe::of().fading(),
        Mono => Recipe::of().monotone(),
    }
}

fn narrowed(spans: &[(f64, f64)], gap: f64) -> Vec<(f64, f64)> {
    spans
        .iter()
        .map(|&(a0, a1)| if (a1 - a0).abs() > gap * 2.5 { (a0 + gap, a1 - gap) } else { (a0, a1) })
        .collect()
}

fn branch_tone(p: &Prepared, i: usize) -> f64 {
    let n = p.cidx.iter().copied().max().unwrap_or(0) + 1;
    p.cidx[i] as f64 / n.max(2) as f64
}

fn finite(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { *v } else { 0.0 }).collect()
}

fn sunburst_3d(cfg: &SunburstConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let values = finite(cfg.values);
    let sound = SunburstConfig { labels: cfg.labels, parents: cfg.parents, values: &values, palette: cfg.palette, ..SunburstConfig::default() };
    let Some(p) = prepare(&sound) else {
        return (Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let cap_depth = depth_cap(&p.depth, NODE_CAP);
    let spans = narrowed(&p.ang, plan.gap);
    let blocks = rings(
        &p.bfs_order,
        &p.depth,
        &spans,
        cap_depth,
        plan.hole,
        |i| if plan.fade { faded_height(p.depth[i], plan.height) } else { plan.height },
        |i| if plan.mono { MONO_TONE } else { branch_tone(&p, i) },
    );
    let names = p.bfs_order.iter().filter(|&&i| p.depth[i] <= cap_depth).map(|&i| p.labels[i].clone()).collect();
    (blocks, names)
}

pub fn layout_3d(cfg: &SunburstConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &SunburstConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.labels.is_empty() {
        return (Vec::new(), Vec::new());
    }
    sunburst_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> (Vec<String>, Vec<String>, Vec<f64>) {
        (
            ["A", "B", "A1", "A2", "B1"].iter().map(|s| s.to_string()).collect(),
            ["", "", "A", "A", "B"].iter().map(|s| s.to_string()).collect(),
            vec![40.0, 60.0, 25.0, 15.0, 20.0],
        )
    }

    fn draw(variant: SunburstVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { variant, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_every_node_and_names_it() {
        for &variant in SunburstVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 5, "{variant:?}");
            assert!(!blocks.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn deeper_nodes_sit_at_a_larger_radius_than_the_root() {
        let (blocks, _) = draw(SunburstVariant::Basic);
        let root_radius = blocks.iter().filter(|b| b.ci == 0).map(|b| b.cx.hypot(b.cy)).fold(0.0, f64::max);
        let deepest_radius = blocks.iter().map(|b| b.cx.hypot(b.cy)).fold(0.0, f64::max);
        assert!(deepest_radius > root_radius);
    }

    #[test]
    fn donut_starts_its_first_ring_farther_out_than_basic() {
        let basic = draw(SunburstVariant::Basic).0;
        let donut = draw(SunburstVariant::Donut).0;
        let min_r = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.cx.hypot(b.cy)).fold(f64::INFINITY, f64::min);
        assert!(min_r(&donut) > min_r(&basic));
    }

    #[test]
    fn outlined_wedges_are_flatter_than_the_basic_ones() {
        let basic = draw(SunburstVariant::Basic).0;
        let outlined = draw(SunburstVariant::Outlined).0;
        assert!(outlined[0].z1 < basic[0].z1);
    }

    #[test]
    fn depth_fade_shrinks_deeper_rings_while_basic_keeps_a_constant_height() {
        let faded = draw(SunburstVariant::DepthFade).0;
        assert!(faded.iter().any(|b| b.z1 < BASE_HEIGHT));
        let basic = draw(SunburstVariant::Basic).0;
        assert!(basic.iter().all(|b| b.z1 == BASE_HEIGHT));
    }

    #[test]
    fn mono_paints_every_wedge_the_same_tone_while_basic_varies_by_branch() {
        let mono = draw(SunburstVariant::Mono).0;
        assert!(mono.iter().all(|b| b.tone == Some(MONO_TONE)));
        let basic = draw(SunburstVariant::Basic).0;
        let tones: std::collections::HashSet<_> = basic.iter().map(|b| b.tone.unwrap().to_bits()).collect();
        assert!(tones.len() >= 2);
    }

    #[test]
    fn a_deep_wide_tree_is_capped_by_depth_and_names_follow_the_kept_nodes() {
        let mut labels = vec!["Root".to_string()];
        let mut parents = vec![String::new()];
        let mut values = vec![0.0];
        for d in 0..8 {
            for k in 0..50 {
                labels.push(format!("d{d}n{k}"));
                parents.push(if d == 0 { "Root".to_string() } else { format!("d{}n{}", d - 1, k % 50) });
                values.push(1.0);
            }
        }
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty() && blocks.len() < labels.len() * 6);
        assert_eq!(names.len(), blocks.iter().map(|b| b.ci).collect::<std::collections::HashSet<_>>().len());
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&SunburstConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["Root".to_string(), "A".to_string()];
        let parents = vec![String::new(), "Root".to_string()];
        let values = vec![f64::NAN, f64::INFINITY];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.hw.is_finite()));
    }

    #[test]
    fn a_single_node_with_no_parent_still_draws_a_full_ring() {
        let labels = vec!["Root".to_string()];
        let parents = vec![String::new()];
        let values = vec![10.0];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty());
        assert_eq!(names, vec!["Root".to_string()]);
    }
}
