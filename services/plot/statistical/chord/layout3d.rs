use super::common::compute_layout;
use super::config::ChordConfig;
use super::variant::ChordVariant;
use crate::plot::statistical::_3d::lineage::{weighted_paths, Point, EDGE_STEPS, NODE_HW};
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;

const MAX_CHORD_NODES: usize = 24;
const RING: f64 = 4.0;
const ARC_STEP: f64 = 0.22;
const NODE_HEIGHT: f64 = 2.6;
const MIN_EDGE: f64 = 0.02;

#[derive(Clone, Copy)]
struct Recipe {
    mono: bool,
    max_edge: f64,
}

impl Recipe {
    const fn of() -> Self {
        Self { mono: false, max_edge: 0.16 }
    }

    const fn one_tone(mut self) -> Self {
        self.mono = true;
        self
    }

    const fn edge_cap(mut self, v: f64) -> Self {
        self.max_edge = v;
        self
    }
}

fn recipe(variant: ChordVariant) -> Recipe {
    use ChordVariant::*;
    match variant {
        Basic | Directed | Bipartite => Recipe::of(),
        Ribbon => Recipe::of().edge_cap(0.3),
        Arc => Recipe::of().edge_cap(0.08),
        Mono => Recipe::of().one_tone(),
    }
}

fn chord_3d(cfg: &ChordConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let capped_labels: Vec<String>;
    let capped_matrix: Vec<Vec<f64>>;
    let scoped_cfg: ChordConfig;
    let cfg = if cfg.labels.len() > MAX_CHORD_NODES {
        capped_labels = cfg.labels[..MAX_CHORD_NODES].to_vec();
        capped_matrix = cfg.matrix[..cfg.matrix.len().min(MAX_CHORD_NODES)]
            .iter()
            .map(|row| row[..row.len().min(MAX_CHORD_NODES)].to_vec())
            .collect();
        scoped_cfg = ChordConfig { variant: cfg.variant, labels: &capped_labels, matrix: &capped_matrix, ..ChordConfig::default() };
        &scoped_cfg
    } else {
        cfg
    };

    let Some(layout) = compute_layout(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let n = layout.arcs.len();
    let plan = recipe(cfg.variant);

    let mid_angle = |(a0, a1): (f64, f64)| (a0 + a1) / 2.0;
    let peak_total = layout.totals.iter().copied().fold(1e-12, f64::max);
    let mut blocks: Vec<Bar3DBlock> = (0..n)
        .flat_map(|i| {
            let (a0, a1) = layout.arcs[i];
            let steps = (((a1 - a0).abs() / ARC_STEP).ceil() as usize).max(1);
            let tone = if plan.mono { 0.5 } else { i as f64 / n.max(1) as f64 };
            let height = NODE_HEIGHT * (layout.totals[i] / peak_total).max(0.05);
            (0..steps).map(move |k| {
                let t = a0 + (a1 - a0) * (k as f64 + 0.5) / steps as f64;
                Bar3DBlock::new(RING * t.cos(), RING * t.sin(), 0.0, height, NODE_HW, NODE_HW, i).with_tone(tone)
            })
        })
        .collect();

    let mut edges: Vec<(Vec<Point>, f64, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let w = cfg.matrix.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
            if w <= 0.0 {
                continue;
            }
            let from = mid_angle(layout.sub_arcs[i][j]);
            let to = mid_angle(layout.sub_arcs[j][i]);
            let path = vec![(RING * from.cos(), RING * from.sin(), 0.0), (0.0, 0.0, 0.0), (RING * to.cos(), RING * to.sin(), 0.0)];
            edges.push((path, w, i));
        }
    }
    let links: Vec<Vec<Point>> = edges.iter().map(|(p, _, _)| p.clone()).collect();
    let weights: Vec<f64> = edges.iter().map(|(_, w, _)| *w).collect();
    let classes: Vec<usize> = edges.iter().map(|(_, _, c)| *c).collect();
    let peak_edge = weights.iter().copied().fold(1e-12, f64::max);
    blocks.extend(weighted_paths(
        &links,
        &weights,
        MIN_EDGE,
        plan.max_edge,
        EDGE_STEPS,
        |li| if plan.mono { n } else { classes[li] },
        |li| if plan.mono { 0.5 } else { weights[li] / peak_edge },
    ));
    (blocks, cfg.labels[..n.min(cfg.labels.len())].to_vec())
}

pub fn layout_3d(cfg: &ChordConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &ChordConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    chord_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mat() -> (Vec<String>, Vec<Vec<f64>>) {
        (
            ["A", "B", "C"].iter().map(|s| s.to_string()).collect(),
            vec![vec![0.0, 5.0, 2.0], vec![3.0, 0.0, 4.0], vec![1.0, 6.0, 0.0]],
        )
    }

    fn draw(variant: ChordVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, matrix) = mat();
        let cfg = ChordConfig { variant, labels: &labels, matrix: &matrix, ..ChordConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_ring_segments_plus_ribbons_and_names_every_node() {
        for &variant in ChordVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 3, "{variant:?}");
            assert!(blocks.len() > 3, "{variant:?}");
        }
    }

    #[test]
    fn mono_paints_every_block_the_same_tone_while_basic_can_vary() {
        let (mono, _) = draw(ChordVariant::Mono);
        assert!(mono.iter().all(|b| b.tone == Some(0.5)));
        let (basic, _) = draw(ChordVariant::Basic);
        assert!(basic.iter().any(|b| b.tone != Some(0.5)));
    }

    #[test]
    fn ribbon_edges_are_thicker_than_arc_edges() {
        let (ribbon, _) = draw(ChordVariant::Ribbon);
        let (arc, _) = draw(ChordVariant::Arc);
        let max_hw = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        assert!(max_hw(&ribbon) > max_hw(&arc));
    }

    #[test]
    fn a_huge_matrix_is_capped_before_the_quadratic_layout_runs() {
        let n = 400;
        let labels: Vec<String> = (0..n).map(|i| format!("L{i}")).collect();
        let matrix: Vec<Vec<f64>> = (0..n).map(|i| (0..n).map(|j| if i == j { 0.0 } else { 1.0 }).collect()).collect();
        let cfg = ChordConfig { labels: &labels, matrix: &matrix, ..ChordConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(names.len() <= MAX_CHORD_NODES);
        assert!(blocks.len() < 20_000);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&ChordConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
