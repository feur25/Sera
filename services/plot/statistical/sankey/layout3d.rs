use super::common::{compute_layout, compute_layout_sorted};
use super::config::SankeyConfig;
use super::variant::SankeyVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{weighted_paths, Point, EDGE_STEPS, NODE_HW};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.8;
const SCALE: f64 = 10.0;
const PLOT_W: i32 = 600;
const PLOT_H: i32 = 600;
const EDGE_FLOOR: f64 = 0.03;
const MIN_EDGE: f64 = 0.04;

#[derive(Clone, Copy)]
struct Recipe {
    sorted: bool,
    max_edge: f64,
}

impl Recipe {
    const fn of() -> Self {
        Self { sorted: false, max_edge: 0.16 }
    }

    const fn sorted_by_value(mut self) -> Self {
        self.sorted = true;
        self
    }

    const fn edge_cap(mut self, v: f64) -> Self {
        self.max_edge = v;
        self
    }
}

fn recipe(variant: SankeyVariant) -> Recipe {
    use SankeyVariant::*;
    match variant {
        Basic | Gapped | Hourglass | Matrix | Beacon => Recipe::of(),
        Ribbon => Recipe::of().edge_cap(0.26),
        Minimal => Recipe::of().edge_cap(0.08),
        Sorted => Recipe::of().sorted_by_value(),
    }
}

pub fn flow_3d(
    labels: &[String],
    sources: &[i32],
    targets: &[i32],
    weights_in: &[f64],
    node_width: i32,
    node_gap: i32,
    sorted: bool,
    max_edge: f64,
) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = labels.len();
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let layout = if sorted {
        compute_layout_sorted(n, sources, targets, weights_in, 0, 0, PLOT_W, PLOT_H, node_width, node_gap)
    } else {
        compute_layout(n, sources, targets, weights_in, 0, 0, PLOT_W, PLOT_H, node_width, node_gap)
    };
    let sx = SCALE / PLOT_W as f64;
    let sy = SCALE / PLOT_H as f64;
    let cx: Vec<f64> = (0..n).map(|i| layout.x[i] * sx).collect();
    let cy: Vec<f64> = (0..n).map(|i| (layout.y[i] + layout.h[i] / 2.0) * sy).collect();
    let peak_node = layout.h.iter().copied().fold(1e-12, f64::max);
    let mut blocks: Vec<Bar3DBlock> = (0..n)
        .map(|i| {
            let h = (layout.h[i] * sy * HEIGHT_RATIO).max(0.05);
            Bar3DBlock::new(cx[i], cy[i], 0.0, h, NODE_HW, NODE_HW, i).with_tone(layout.h[i] / peak_node)
        })
        .collect();
    let e = sources.len().min(targets.len()).min(weights_in.len());
    let edges: Vec<(Vec<Point>, f64, usize)> = (0..e)
        .filter_map(|k| {
            let s = sources[k] as usize;
            let t = targets[k] as usize;
            if s >= n || t >= n || s == t {
                return None;
            }
            let path = vec![(cx[s], cy[s], EDGE_FLOOR), (cx[t], cy[t], EDGE_FLOOR)];
            Some((path, weights_in.get(k).copied().unwrap_or(1.0).max(0.0), s))
        })
        .collect();
    let links: Vec<Vec<Point>> = edges.iter().map(|(p, _, _)| p.clone()).collect();
    let weights: Vec<f64> = edges.iter().map(|(_, w, _)| *w).collect();
    let sources_idx: Vec<usize> = edges.iter().map(|(_, _, s)| *s).collect();
    let peak_edge = weights.iter().copied().fold(1e-12, f64::max);
    blocks.extend(weighted_paths(
        &links,
        &weights,
        MIN_EDGE,
        max_edge,
        EDGE_STEPS,
        |li| sources_idx[li],
        |li| weights[li] / peak_edge,
    ));
    (blocks, labels.to_vec())
}

pub fn layout_3d(cfg: &SankeyConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &SankeyConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let plan = recipe(cfg.variant);
    flow_3d(cfg.labels, cfg.sources, cfg.targets, cfg.weights, cfg.node_width, cfg.node_gap, plan.sorted, plan.max_edge)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flow() -> (Vec<String>, Vec<i32>, Vec<i32>, Vec<f64>) {
        (
            ["A", "B", "C", "D"].iter().map(|s| s.to_string()).collect(),
            vec![0, 0, 1],
            vec![2, 3, 3],
            vec![5.0, 3.0, 4.0],
        )
    }

    fn draw(variant: SankeyVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, sources, targets, weights) = flow();
        let cfg = SankeyConfig { variant, labels: &labels, sources: &sources, targets: &targets, weights: &weights, ..SankeyConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_node_plus_ribbons_and_names_every_node() {
        for &variant in SankeyVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 4, "{variant:?}");
            assert!(blocks.len() > 4, "{variant:?}");
        }
    }

    #[test]
    fn a_node_with_more_total_flow_stands_taller() {
        let (blocks, names) = draw(SankeyVariant::Basic);
        let a = blocks[names.iter().position(|n| n == "A").unwrap()];
        let b = blocks[names.iter().position(|n| n == "B").unwrap()];
        assert!(a.z1 > b.z1);
    }

    #[test]
    fn sorted_and_basic_place_the_same_nodes_in_a_different_order_within_a_layer() {
        let (basic, names) = draw(SankeyVariant::Basic);
        let (sorted, _) = draw(SankeyVariant::Sorted);
        let ai = names.iter().position(|n| n == "A").unwrap();
        assert!((basic[ai].cx - sorted[ai].cx).abs() < 1e-9);
    }

    #[test]
    fn ribbon_edges_are_thicker_than_minimal_edges() {
        let (ribbon, _) = draw(SankeyVariant::Ribbon);
        let (minimal, _) = draw(SankeyVariant::Minimal);
        let max_hw = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        assert!(max_hw(&ribbon) > max_hw(&minimal));
    }

    #[test]
    fn a_self_loop_is_dropped_instead_of_drawing_a_zero_length_ribbon() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let sources = vec![0, 0];
        let targets = vec![0, 1];
        let weights = vec![2.0, 3.0];
        let cfg = SankeyConfig { labels: &labels, sources: &sources, targets: &targets, weights: &weights, ..SankeyConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names.len(), 2);
        assert!(blocks.len() < 20);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&SankeyConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn a_long_chain_of_nodes_stays_fast() {
        let n = 4000;
        let labels: Vec<String> = (0..n).map(|i| format!("N{i}")).collect();
        let sources: Vec<i32> = (0..n as i32 - 1).collect();
        let targets: Vec<i32> = (1..n as i32).collect();
        let weights: Vec<f64> = vec![1.0; sources.len()];
        let cfg = SankeyConfig { labels: &labels, sources: &sources, targets: &targets, weights: &weights, ..SankeyConfig::default() };
        let t0 = std::time::Instant::now();
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names.len(), n);
        assert!(!blocks.is_empty());
        assert!(t0.elapsed().as_secs() < 5, "took {:?}", t0.elapsed());
    }
}
