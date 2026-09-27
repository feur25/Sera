use super::common::{prepare, Prepared};
use super::config::SunburstConfig;
use super::variant::SunburstVariant;
use crate::plot::statistical::_3d::hierarchy::{depth_cap, descendant_groups, faded_height, MIN_SPAN};
pub use crate::plot::statistical::_3d::wedge::Wedges;
use crate::plot::statistical::_3d::wedge::HEIGHT as WEDGE_HEIGHT;

const NODE_CAP: usize = 600;
const GAP_RADIANS: f64 = 0.02;

fn narrowed(spans: &[(f64, f64)], gap: f64) -> Vec<(f64, f64)> {
    spans
        .iter()
        .map(|&(a0, a1)| if (a1 - a0).abs() > gap * 2.5 { (a0 + gap, a1 - gap) } else { (a0, a1) })
        .collect()
}

fn finite(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { *v } else { 0.0 }).collect()
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

pub fn wedges(cfg: &SunburstConfig) -> Option<(Wedges, Vec<Vec<u32>>)> {
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
    let groups = if matches!(cfg.variant, SunburstVariant::Zoomable) { descendant_groups(&p.depth, &spans, &kept) } else { Vec::new() };
    Some((w, groups))
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

    fn draw_wedges(variant: SunburstVariant) -> Wedges {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { variant, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        wedges(&cfg).unwrap().0
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
    fn deeper_nodes_sit_at_a_larger_radius_than_the_root() {
        let w = draw_wedges(SunburstVariant::Basic);
        let root_radius = w.inner_r[0];
        let deepest_radius = w.outer_r.iter().cloned().fold(0.0, f64::max);
        assert!(deepest_radius > root_radius);
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
        let (w, _groups) = wedges(&cfg).unwrap();
        assert!(!w.a0.is_empty() && w.a0.len() <= labels.len());
        assert_eq!(w.names.len(), w.a0.len());
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(wedges(&SunburstConfig::default()).is_none());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["Root".to_string(), "A".to_string()];
        let parents = vec![String::new(), "Root".to_string()];
        let values = vec![f64::NAN, f64::INFINITY];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        if let Some((w, _groups)) = wedges(&cfg) {
            assert!(w.a0.iter().chain(&w.a1).chain(&w.inner_r).chain(&w.outer_r).chain(&w.height).all(|v| v.is_finite()));
        }
    }

    #[test]
    fn a_single_node_with_no_parent_still_draws_a_full_ring() {
        let labels = vec!["Root".to_string()];
        let parents = vec![String::new()];
        let values = vec![10.0];
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let (w, _groups) = wedges(&cfg).unwrap();
        assert_eq!(w.names, vec!["Root".to_string()]);
    }

    #[test]
    fn zoomable_groups_a_parent_with_its_descendants_while_basic_emits_no_groups() {
        let labels = vec!["Root".to_string(), "A".to_string(), "B".to_string()];
        let parents = vec![String::new(), "Root".to_string(), "Root".to_string()];
        let values = vec![0.0, 40.0, 60.0];
        let draw_rooted = |variant| {
            let cfg = SunburstConfig { variant, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
            wedges(&cfg).unwrap()
        };

        let (w, groups) = draw_rooted(SunburstVariant::Zoomable);
        assert_eq!(groups.len(), w.names.len());
        let root_pos = w.names.iter().position(|n| n == "Root").unwrap();
        assert_eq!(groups[root_pos].len(), w.names.len());
        let a_pos = w.names.iter().position(|n| n == "A").unwrap();
        assert!(groups[a_pos].len() < w.names.len());

        let (_, basic_groups) = draw_rooted(SunburstVariant::Basic);
        assert!(basic_groups.is_empty());
    }

    #[test]
    fn wedge_angles_follow_bfs_order_and_match_the_prepared_hierarchy_span() {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let p = prepare(&cfg).unwrap();
        let w = wedges(&cfg).unwrap().0;
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
        let w = wedges(&cfg).unwrap().0;
        let distinct: std::collections::HashSet<_> = w.color_idx.iter().map(|c| c.to_bits()).collect();
        assert!(distinct.len() >= 2, "a single-root tree must still color A and B differently");
    }

    #[test]
    fn a_grandchild_inherits_its_top_level_branch_color_not_its_own() {
        let (labels, parents, values) = tree();
        let cfg = SunburstConfig { labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
        let p = prepare(&cfg).unwrap();
        let w = wedges(&cfg).unwrap().0;
        let a_pos = p.bfs_order.iter().position(|&i| labels[i] == "A").unwrap();
        let a1_pos = p.bfs_order.iter().position(|&i| labels[i] == "A1").unwrap();
        assert_eq!(w.color_idx[a_pos], w.color_idx[a1_pos], "A1 must share A's branch color");
    }

    #[test]
    fn wedges_is_none_for_empty_input() {
        assert!(wedges(&SunburstConfig::default()).is_none());
    }
}
