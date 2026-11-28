use super::config::HiveConfig;
use super::variant::HiveVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{weighted_paths, Point, EDGE_STEPS, NODE_HW};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::TAU;
use std::collections::HashMap;

const HOLE: f64 = 0.8;
const OUTER: f64 = 4.6;
const NODE_HEIGHT: f64 = 2.2;
const MIN_EDGE: f64 = 0.03;

#[derive(Clone, Copy)]
struct Recipe {
    curved: bool,
    max_edge: f64,
}

impl Recipe {
    const fn of() -> Self {
        Self { curved: false, max_edge: 0.14 }
    }

    const fn bent_through_center(mut self) -> Self {
        self.curved = true;
        self
    }

    const fn edge_cap(mut self, v: f64) -> Self {
        self.max_edge = v;
        self
    }
}

fn recipe(variant: HiveVariant) -> Recipe {
    use HiveVariant::*;
    match variant {
        Basic | Directed => Recipe::of(),
        Curved => Recipe::of().bent_through_center(),
        Weighted => Recipe::of().edge_cap(0.24),
        Minimal => Recipe::of().edge_cap(0.06),
    }
}

fn node_pos(angle: f64, value: f64) -> (f64, f64) {
    let r = HOLE + value.clamp(0.0, 1.0) * (OUTER - HOLE);
    (r * angle.cos(), r * angle.sin())
}

fn hive_3d(cfg: &HiveConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let na = cfg.axes.len();
    let n = cfg.labels.len();
    if na == 0 || n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let axis_angles: Vec<f64> = (0..na).map(|i| TAU * i as f64 / na as f64 - std::f64::consts::FRAC_PI_2).collect();

    let mut axis_of = HashMap::new();
    for (i, cat) in cfg.categories.iter().enumerate() {
        if let Some(ai) = cfg.axes.iter().position(|a| a == cat) {
            axis_of.insert(i, ai);
        }
    }
    let mut by_axis: Vec<Vec<usize>> = vec![Vec::new(); na];
    for i in 0..n {
        if let Some(&ai) = axis_of.get(&i) {
            by_axis[ai].push(i);
        }
    }

    let mut positions = vec![(0.0f64, 0.0f64); n];
    let mut values = vec![0.5f64; n];
    for ai in 0..na {
        let members = &by_axis[ai];
        for (k, &ni) in members.iter().enumerate() {
            let v = cfg.values.get(ni).copied().unwrap_or_else(|| (k as f64 + 1.0) / members.len().max(1) as f64);
            values[ni] = v.clamp(0.0, 1.0);
            positions[ni] = node_pos(axis_angles[ai], values[ni]);
        }
    }

    let mut blocks: Vec<Bar3DBlock> = (0..n)
        .map(|i| {
            let ai = axis_of.get(&i).copied().unwrap_or(0);
            Bar3DBlock::new(positions[i].0, positions[i].1, 0.0, NODE_HEIGHT * values[i].max(0.08), NODE_HW, NODE_HW, ai)
                .with_tone(ai as f64 / na.max(1) as f64)
        })
        .collect();

    let e = cfg.sources.len().min(cfg.targets.len());
    let edges: Vec<(Vec<Point>, f64, usize)> = (0..e)
        .filter_map(|k| {
            let s = cfg.sources[k] as usize;
            let t = cfg.targets[k] as usize;
            if s >= n || t >= n || s == t {
                return None;
            }
            let (sx, sy) = positions[s];
            let (tx, ty) = positions[t];
            let w = cfg.weights.get(k).copied().unwrap_or(1.0).max(0.0);
            let path = if plan.curved {
                vec![(sx, sy, 0.0), (0.0, 0.0, 0.0), (tx, ty, 0.0)]
            } else {
                vec![(sx, sy, 0.0), (tx, ty, 0.0)]
            };
            let class = axis_of.get(&s).copied().unwrap_or(0);
            Some((path, w, class))
        })
        .collect();
    let links: Vec<Vec<Point>> = edges.iter().map(|(p, _, _)| p.clone()).collect();
    let weights: Vec<f64> = edges.iter().map(|(_, w, _)| *w).collect();
    let classes: Vec<usize> = edges.iter().map(|(_, _, c)| *c).collect();
    let peak = weights.iter().copied().fold(1e-12, f64::max);
    blocks.extend(weighted_paths(
        &links,
        &weights,
        MIN_EDGE,
        plan.max_edge,
        EDGE_STEPS,
        |li| classes[li],
        |li| weights[li] / peak,
    ));
    (blocks, cfg.labels.to_vec())
}

pub fn layout_3d(cfg: &HiveConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &HiveConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    hive_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plot() -> (Vec<String>, Vec<String>, Vec<String>, Vec<f64>, Vec<i32>, Vec<i32>, Vec<f64>) {
        (
            ["Biology", "Chemistry", "Physics"].iter().map(|s| s.to_string()).collect(),
            ["n1", "n2", "n3", "n4", "n5", "n6"].iter().map(|s| s.to_string()).collect(),
            ["Biology", "Biology", "Chemistry", "Chemistry", "Physics", "Physics"].iter().map(|s| s.to_string()).collect(),
            vec![0.3, 0.7, 0.2, 0.9, 0.5, 0.8],
            vec![0, 1, 2, 4],
            vec![2, 3, 4, 5],
            vec![1.0, 2.0, 1.5, 0.8],
        )
    }

    fn draw(variant: HiveVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (axes, labels, categories, values, sources, targets, weights) = plot();
        let cfg = HiveConfig {
            variant,
            axes: &axes,
            labels: &labels,
            categories: &categories,
            values: &values,
            sources: &sources,
            targets: &targets,
            weights: &weights,
            ..HiveConfig::default()
        };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_node_plus_edges_and_names_every_node() {
        for &variant in HiveVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 6, "{variant:?}");
            assert!(blocks.len() > 6, "{variant:?}");
        }
    }

    #[test]
    fn nodes_on_different_axes_sit_at_different_angles_around_the_hub() {
        let (blocks, names) = draw(HiveVariant::Basic);
        let n1 = blocks[names.iter().position(|n| n == "n1").unwrap()];
        let n3 = blocks[names.iter().position(|n| n == "n3").unwrap()];
        assert!((n1.cx - n3.cx).abs() > 1e-6 || (n1.cy - n3.cy).abs() > 1e-6);
    }

    #[test]
    fn curved_edges_route_through_the_hub_while_basic_goes_direct() {
        let (basic, names) = draw(HiveVariant::Basic);
        let (curved, _) = draw(HiveVariant::Curved);
        let n = names.len();
        let dist = |b: &Bar3DBlock| (b.cx * b.cx + b.cy * b.cy).sqrt();
        let min_basic_edge = basic[n..].iter().map(dist).fold(f64::INFINITY, f64::min);
        let min_curved_edge = curved[n..].iter().map(dist).fold(f64::INFINITY, f64::min);
        assert!(min_curved_edge < min_basic_edge * 0.5);
    }

    #[test]
    fn weighted_edges_are_thicker_than_minimal_edges() {
        let (weighted, _) = draw(HiveVariant::Weighted);
        let (minimal, _) = draw(HiveVariant::Minimal);
        let max_hw = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        assert!(max_hw(&weighted) > max_hw(&minimal));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&HiveConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
