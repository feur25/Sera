use super::common::{prepare, Prepared};
use super::config::SunburstConfig;
use super::variant::SunburstVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::{depth_cap, descendant_groups, faded_height, rings, BASE_HEIGHT, HOLE, MIN_SPAN};
pub use crate::plot::statistical::_3d::wedge::Wedges;
use crate::plot::statistical::_3d::wedge::HEIGHT as WEDGE_HEIGHT;
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

fn branch_tone(branch: &[usize], i: usize) -> f64 {
    let n = branch.iter().copied().max().unwrap_or(0) + 1;
    branch[i] as f64 / n.max(2) as f64
}

fn finite(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { *v } else { 0.0 }).collect()
}

fn sunburst_3d(cfg: &SunburstConfig) -> (Vec<Bar3DBlock>, Vec<String>, Vec<Vec<u32>>) {
    let values = finite(cfg.values);
    let sound = SunburstConfig { labels: cfg.labels, parents: cfg.parents, values: &values, palette: cfg.palette, ..SunburstConfig::default() };
    let Some(p) = prepare(&sound) else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let cap_depth = depth_cap(&p.depth, NODE_CAP);
    let spans = narrowed(&p.ang, plan.gap);
    let branch = branch_keys(&p);
    let blocks = rings(
        &p.bfs_order,
        &p.depth,
        &spans,
        cap_depth,
        plan.hole,
        |i| if plan.fade { faded_height(p.depth[i], plan.height) } else { plan.height },
        |i| if plan.mono { MONO_TONE } else { branch_tone(&branch, i) },
    );
    let names = p.bfs_order.iter().filter(|&&i| p.depth[i] <= cap_depth).map(|&i| p.labels[i].clone()).collect();
    let groups = if matches!(cfg.variant, SunburstVariant::Zoomable) {
        let block_ci: Vec<usize> = blocks.iter().map(|b| b.ci).collect();
        descendant_groups(&p.depth, &p.ang, &block_ci)
    } else {
        Vec::new()
    };
    (blocks, names, groups)
}

fn wedge_plan(variant: SunburstVariant) -> (f64, f64, bool) {
    use SunburstVariant::*;
    match variant {
        Donut => (0.32, 0.0, false),
        Gapped => (0.09, GAP_RADIANS, false),
        Mono => (0.09, 0.0, true),
        Basic | Outlined | DepthFade | Zoomable => (0.09, 0.0, false),
    }
}

fn ring_height_scale(variant: SunburstVariant, depth: usize) -> f64 {
    use SunburstVariant::*;
    match variant {
        DepthFade => faded_height(depth, 1.0),
        Outlined => 0.4,
        _ => 1.0,
    }
}

fn branch_keys(p: &Prepared) -> Vec<usize> {
    let target_depth = if p.roots.len() <= 1 { 1 } else { 0 };
    let mut key = vec![0usize; p.n];
    for &i in &p.bfs_order {
        key[i] = if p.depth[i] <= target_depth {
            i
        } else {
            let (a0, a1) = p.ang[i];
            p.bfs_order
                .iter()
                .copied()
                .find(|&j| p.depth[j] == target_depth && p.ang[j].0 <= a0 + 1e-9 && p.ang[j].1 >= a1 - 1e-9)
                .unwrap_or(i)
        };
    }
    key
}

pub fn wedges(cfg: &SunburstConfig) -> Option<Wedges> {
    let values = finite(cfg.values);
    let sound = SunburstConfig { labels: cfg.labels, parents: cfg.parents, values: &values, palette: cfg.palette, ..SunburstConfig::default() };
    let p = prepare(&sound)?;
    let (hole, gap, mono) = wedge_plan(cfg.variant);
    let cap_depth = depth_cap(&p.depth, NODE_CAP);
    let spans = narrowed(&p.ang, gap);
    let branch = branch_keys(&p);
    let kept: Vec<usize> = p
        .bfs_order
        .iter()
        .copied()
        .filter(|&i| p.depth[i] <= cap_depth && spans[i].1 - spans[i].0 > MIN_SPAN)
        .collect();
    if kept.is_empty() {
        return None;
    }
    let n_rings = kept.iter().map(|&i| p.depth[i]).max().unwrap_or(0) + 1;
    let ring_span = (1.0 - hole) / n_rings as f64;
    let mut w = Wedges::default();
    for &i in &kept {
        let (a0, a1) = spans[i];
        let inner_r = hole + ring_span * p.depth[i] as f64;
        let outer_r = inner_r + ring_span * 0.96;
        let height = ring_height_scale(cfg.variant, p.depth[i]) * WEDGE_HEIGHT;
        let color_idx = if mono { 0.0 } else { branch[i] as f64 };
        w.push(a0, a1, inner_r, outer_r, 0.0, 0.0, height, p.values_eff[i], color_idx, p.labels[i].clone());
    }
    Some(w)
}

pub fn layout_3d(cfg: &SunburstConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &SunburstConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>, Vec<Vec<u32>>) {
    if cfg.labels.is_empty() {
        return (Vec::new(), Vec::new(), Vec::new());
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

    fn draw(variant: SunburstVariant) -> (Vec<Bar3DBlock>, Vec<String>, Vec<Vec<u32>>) {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { variant, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_every_node_and_names_it() {
        for &variant in SunburstVariant::all() {
            let (blocks, names, _groups) = draw(variant);
            assert_eq!(names.len(), 5, "{variant:?}");
            assert!(!blocks.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn deeper_nodes_sit_at_a_larger_radius_than_the_root() {
        let (blocks, _, _) = draw(SunburstVariant::Basic);
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
        let (blocks, names, _groups) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty() && blocks.len() < labels.len() * 6);
        assert_eq!(names.len(), blocks.iter().map(|b| b.ci).collect::<std::collections::HashSet<_>>().len());
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names, groups) = layout_named(&SunburstConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty() && groups.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["Root".to_string(), "A".to_string()];
        let parents = vec![String::new(), "Root".to_string()];
        let values = vec![f64::NAN, f64::INFINITY];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let (blocks, _, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.hw.is_finite()));
    }

    #[test]
    fn a_single_node_with_no_parent_still_draws_a_full_ring() {
        let labels = vec!["Root".to_string()];
        let parents = vec![String::new()];
        let values = vec![10.0];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let (blocks, names, _groups) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty());
        assert_eq!(names, vec!["Root".to_string()]);
    }

    #[test]
    fn zoomable_groups_a_parent_with_its_descendants_while_basic_emits_no_groups() {
        let labels = vec!["Root".to_string(), "A".to_string(), "B".to_string()];
        let parents = vec![String::new(), "Root".to_string(), "Root".to_string()];
        let values = vec![0.0, 40.0, 60.0];
        let draw_rooted = |variant| {
            let cfg = SunburstConfig { variant, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
            layout_named(&cfg, &Budget::default())
        };

        let (blocks, _names, groups) = draw_rooted(SunburstVariant::Zoomable);
        assert_eq!(groups.len(), blocks.len());
        let root_group_size = groups.iter().zip(blocks.iter()).find(|(_, b)| b.ci == 0).unwrap().0.len();
        assert_eq!(root_group_size, blocks.len());
        let a_ci = blocks.iter().find(|b| b.ci != 0).unwrap().ci;
        let a_group_size = groups.iter().zip(blocks.iter()).find(|(_, b)| b.ci == a_ci).unwrap().0.len();
        assert!(a_group_size < blocks.len());

        let (_, _, basic_groups) = draw_rooted(SunburstVariant::Basic);
        assert!(basic_groups.is_empty());
    }

    fn draw_wedges(variant: SunburstVariant) -> Wedges {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { variant, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        wedges(&cfg).unwrap()
    }

    #[test]
    fn every_variant_draws_every_kept_node_with_matching_arrays_and_names() {
        for &variant in SunburstVariant::all() {
            let w = draw_wedges(variant);
            assert!(!w.a0.is_empty(), "{variant:?}");
            let n = w.a0.len();
            for field in [w.a1.len(), w.inner_r.len(), w.outer_r.len(), w.cx.len(), w.cy.len(), w.height.len(), w.value.len(), w.color_idx.len(), w.names.len()] {
                assert_eq!(field, n, "{variant:?}");
            }
        }
    }

    #[test]
    fn wedge_angles_follow_bfs_order_and_match_the_prepared_hierarchy_span() {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let p = prepare(&cfg).unwrap();
        let w = wedges(&cfg).unwrap();
        assert_eq!(w.a0.len(), p.bfs_order.len());
        for (k, &i) in p.bfs_order.iter().enumerate() {
            assert_eq!(w.a0[k], p.ang[i].0, "node {i} at position {k}");
            assert_eq!(w.a1[k], p.ang[i].1, "node {i} at position {k}");
            assert_eq!(w.names[k], p.labels[i]);
        }
    }

    #[test]
    fn donut_opens_a_wider_hole_than_basic() {
        assert!(draw_wedges(SunburstVariant::Donut).inner_r[0] > draw_wedges(SunburstVariant::Basic).inner_r[0]);
    }

    #[test]
    fn gapped_narrows_every_span_while_basic_keeps_the_full_prepared_span() {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let p = prepare(&cfg).unwrap();
        let basic = draw_wedges(SunburstVariant::Basic);
        let gapped = draw_wedges(SunburstVariant::Gapped);
        let root_span = p.ang[0].1 - p.ang[0].0;
        assert_eq!(basic.a1[0] - basic.a0[0], root_span);
        assert!(gapped.a1[0] - gapped.a0[0] < root_span);
    }

    #[test]
    fn depth_fade_shrinks_ring_height_deeper_while_basic_stays_flat() {
        let basic = draw_wedges(SunburstVariant::Basic);
        assert!(basic.height.iter().all(|&h| (h - WEDGE_HEIGHT).abs() < 1e-9));
        let faded = draw_wedges(SunburstVariant::DepthFade);
        assert!(faded.height.iter().any(|&h| (h - WEDGE_HEIGHT).abs() < 1e-9), "the root ring must keep full height");
        assert!(faded.height.iter().any(|&h| h < WEDGE_HEIGHT - 1e-9), "a deeper ring must shrink");
    }

    #[test]
    fn outlined_is_flatter_than_basic() {
        let outlined = draw_wedges(SunburstVariant::Outlined);
        assert!(outlined.height.iter().all(|&h| h < WEDGE_HEIGHT));
    }

    #[test]
    fn mono_collapses_every_color_index_while_basic_varies_by_branch() {
        let mono = draw_wedges(SunburstVariant::Mono);
        assert!(mono.color_idx.iter().all(|&c| c == 0.0));
        let basic = draw_wedges(SunburstVariant::Basic);
        let distinct: std::collections::HashSet<_> = basic.color_idx.iter().map(|c| c.to_bits()).collect();
        assert!(distinct.len() >= 2);
    }

    #[test]
    fn a_single_root_still_colors_its_top_level_children_differently() {
        let labels = vec!["Root".to_string(), "A".to_string(), "B".to_string()];
        let parents = vec![String::new(), "Root".to_string(), "Root".to_string()];
        let values = vec![0.0, 40.0, 60.0];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let w = wedges(&cfg).unwrap();
        let distinct: std::collections::HashSet<_> = w.color_idx.iter().map(|c| c.to_bits()).collect();
        assert!(distinct.len() >= 2, "a single-root tree must still color A and B differently");
    }

    #[test]
    fn a_grandchild_inherits_its_top_level_branch_color_not_its_own() {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let p = prepare(&cfg).unwrap();
        let w = wedges(&cfg).unwrap();
        let a_pos = p.bfs_order.iter().position(|&i| labels[i] == "A").unwrap();
        let a1_pos = p.bfs_order.iter().position(|&i| labels[i] == "A1").unwrap();
        assert_eq!(w.color_idx[a_pos], w.color_idx[a1_pos], "A1 must share A's branch color");
    }

    #[test]
    fn wedges_is_none_for_empty_input() {
        assert!(wedges(&SunburstConfig::default()).is_none());
    }
}
