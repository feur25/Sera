use super::config::ArcDiagramConfig;
use super::variant::ArcDiagramVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{markers, weighted_paths, Point, EDGE_STEPS, NODE_HW};
use crate::plot::statistical::bar::Bar3DBlock;

const SPAN: f64 = 10.0;
const ARC_HEIGHT: f64 = 2.4;
const MIN_EDGE: f64 = 0.03;

#[derive(Clone, Copy)]
struct Recipe {
    bilateral: bool,
    max_edge: f64,
}

impl Recipe {
    const fn of() -> Self {
        Self { bilateral: false, max_edge: 0.14 }
    }

    const fn split_above_below(mut self) -> Self {
        self.bilateral = true;
        self
    }

    const fn edge_cap(mut self, v: f64) -> Self {
        self.max_edge = v;
        self
    }
}

fn recipe(variant: ArcDiagramVariant) -> Recipe {
    use ArcDiagramVariant::*;
    match variant {
        Basic | Directed => Recipe::of(),
        Bilateral => Recipe::of().split_above_below(),
        Weighted => Recipe::of().edge_cap(0.24),
        Minimal => Recipe::of().edge_cap(0.06),
    }
}

fn arc_diagram_3d(cfg: &ArcDiagramConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len();
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let step = if n > 1 { SPAN / (n as f64 - 1.0) } else { 0.0 };
    let x: Vec<f64> = (0..n).map(|i| i as f64 * step - SPAN / 2.0).collect();
    let positions: Vec<Point> = x.iter().map(|&xi| (xi, 0.0, 0.0)).collect();
    let mut blocks = markers(&positions, NODE_HW, |i| i, |i| i as f64 / n.max(1) as f64);

    let e = cfg.sources.len().min(cfg.targets.len());
    let edges: Vec<(Vec<Point>, f64, usize)> = (0..e)
        .filter_map(|k| {
            let s = cfg.sources[k] as usize;
            let t = cfg.targets[k] as usize;
            if s >= n || t >= n || s == t {
                return None;
            }
            let w = cfg.weights.get(k).copied().unwrap_or(1.0).max(0.0);
            let above = !plan.bilateral || k % 2 == 0;
            let apex_z = if above { ARC_HEIGHT } else { -ARC_HEIGHT };
            let mid_x = (x[s] + x[t]) / 2.0;
            let path = vec![(x[s], 0.0, 0.0), (mid_x, 0.0, apex_z), (x[t], 0.0, 0.0)];
            Some((path, w, s))
        })
        .collect();
    let links: Vec<Vec<Point>> = edges.iter().map(|(p, _, _)| p.clone()).collect();
    let weights: Vec<f64> = edges.iter().map(|(_, w, _)| *w).collect();
    let sources_idx: Vec<usize> = edges.iter().map(|(_, _, s)| *s).collect();
    let peak = weights.iter().copied().fold(1e-12, f64::max);
    blocks.extend(weighted_paths(
        &links,
        &weights,
        MIN_EDGE,
        plan.max_edge,
        EDGE_STEPS,
        |li| sources_idx[li],
        |li| weights[li] / peak,
    ));
    (blocks, cfg.labels.to_vec())
}

pub fn layout_3d(cfg: &ArcDiagramConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &ArcDiagramConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    arc_diagram_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn net() -> (Vec<String>, Vec<i32>, Vec<i32>, Vec<f64>) {
        (
            ["A", "B", "C", "D", "E", "F"].iter().map(|s| s.to_string()).collect(),
            vec![0, 0, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5, 0],
            vec![3.0, 5.0, 2.0, 4.0, 6.0, 1.0],
        )
    }

    fn draw(variant: ArcDiagramVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, sources, targets, weights) = net();
        let cfg = ArcDiagramConfig { variant, labels: &labels, sources: &sources, targets: &targets, weights: &weights, ..ArcDiagramConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_marker_per_node_plus_arcs_and_names_every_node() {
        for &variant in ArcDiagramVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 6, "{variant:?}");
            assert!(blocks.len() > 6, "{variant:?}");
        }
    }

    #[test]
    fn nodes_are_spread_left_to_right_in_label_order() {
        let (blocks, names) = draw(ArcDiagramVariant::Basic);
        let a = blocks[names.iter().position(|n| n == "A").unwrap()];
        let f = blocks[names.iter().position(|n| n == "F").unwrap()];
        assert!(f.cx > a.cx);
    }

    #[test]
    fn bilateral_sends_alternating_arcs_above_and_below_the_line() {
        let (blocks, names) = draw(ArcDiagramVariant::Bilateral);
        let above = blocks.iter().any(|b| b.z1 > 0.1 || b.z0 > 0.1);
        let below = blocks.iter().any(|b| b.z1 < -0.1 || b.z0 < -0.1);
        let _ = names;
        assert!(above && below);
    }

    #[test]
    fn weighted_edges_are_thicker_than_minimal_edges() {
        let (weighted, _) = draw(ArcDiagramVariant::Weighted);
        let (minimal, _) = draw(ArcDiagramVariant::Minimal);
        let max_hw = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        assert!(max_hw(&weighted) > max_hw(&minimal));
    }

    #[test]
    fn a_self_loop_is_dropped_instead_of_drawing_a_zero_length_arc() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let sources = vec![0, 0];
        let targets = vec![0, 1];
        let weights = vec![2.0, 3.0];
        let cfg = ArcDiagramConfig { labels: &labels, sources: &sources, targets: &targets, weights: &weights, ..ArcDiagramConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names.len(), 2);
        assert!(blocks.len() < 20);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&ArcDiagramConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
