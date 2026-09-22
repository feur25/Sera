use super::common::{build_circles, Circle};
use super::config::CirclePackConfig;
use super::variant::CirclePackVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::branch_tone;
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.8;
pub const COLORMAP: &str = "jet";
const R_MAX: f64 = 6.0;
const NODE_CAP: usize = 600;
const BASE_HEIGHT: f64 = 1.4;
const THIN_HEIGHT: f64 = BASE_HEIGHT * 0.16;
const GRID_STEP: f64 = 1.3;
const GRID_HW: f64 = 0.5;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Packed,
    Grid,
}

#[derive(Clone, Copy)]
struct Recipe {
    shape: Shape,
    thin: bool,
    leaves_only: bool,
}

impl Recipe {
    const fn of(shape: Shape) -> Self {
        Self { shape, thin: false, leaves_only: false }
    }

    const fn thinned(mut self) -> Self {
        self.thin = true;
        self
    }

    const fn leafy(mut self) -> Self {
        self.leaves_only = true;
        self
    }
}

fn recipe(variant: CirclePackVariant) -> Recipe {
    use CirclePackVariant::*;
    match variant {
        Basic | Bubble | Swarm => Recipe::of(Shape::Packed),
        Flat => Recipe::of(Shape::Packed).thinned(),
        Outlined => Recipe::of(Shape::Packed).thinned(),
        LeafFocus => Recipe::of(Shape::Packed).leafy(),
        Matrix => Recipe::of(Shape::Grid),
    }
}

fn is_leaf(circle: &Circle) -> bool {
    circle.children.is_empty()
}

fn packed_blocks(circles: &[Circle], plan: Recipe, branch_count: usize) -> Vec<Bar3DBlock> {
    circles
        .iter()
        .enumerate()
        .filter(|(_, c)| c.r > 1e-6)
        .map(|(i, c)| {
            let leaf = is_leaf(c);
            let height = if plan.leaves_only && !leaf {
                THIN_HEIGHT
            } else if plan.thin {
                THIN_HEIGHT
            } else {
                BASE_HEIGHT
            };
            Bar3DBlock::new(c.x, c.y, 0.0, height, c.r, c.r, i).with_tone(branch_tone(c.color_idx, branch_count))
        })
        .collect()
}

fn grid_blocks(circles: &[Circle], branch_count: usize) -> Vec<Bar3DBlock> {
    let leaves: Vec<(usize, &Circle)> = circles.iter().enumerate().filter(|(_, c)| is_leaf(c) && c.r > 1e-6).collect();
    let cols = (leaves.len() as f64).sqrt().ceil().max(1.0) as usize;
    leaves
        .iter()
        .enumerate()
        .map(|(slot, &(i, c))| {
            let (row, col) = (slot / cols, slot % cols);
            let height = BASE_HEIGHT * (0.4 + 0.6 * (c.r / R_MAX).clamp(0.0, 1.0));
            Bar3DBlock::new(col as f64 * GRID_STEP, row as f64 * GRID_STEP, 0.0, height, GRID_HW, GRID_HW, i)
                .with_tone(branch_tone(c.color_idx, branch_count))
        })
        .collect()
}

fn circle_pack_3d(cfg: &CirclePackConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let padded_parents: Vec<String> = (0..n).map(|i| cfg.parents.get(i).cloned().unwrap_or_default()).collect();
    let clean_values: Vec<f64> = cfg.values[..n].iter().map(|v| if v.is_finite() { v.max(0.0) } else { 0.0 }).collect();
    let circles = build_circles(&cfg.labels[..n], &padded_parents, &clean_values, 0.0, 0.0, R_MAX, cfg.padding.max(0.4));
    let cap = circles.len().min(NODE_CAP);
    let kept: Vec<Circle> = if circles.len() > cap {
        let mut order: Vec<usize> = (0..circles.len()).collect();
        order.sort_by(|&a, &b| circles[b].r.total_cmp(&circles[a].r));
        order.truncate(cap);
        order.sort_unstable();
        order.into_iter().map(|i| clone_circle(&circles[i])).collect()
    } else {
        circles.iter().map(clone_circle).collect()
    };
    let branch_count = kept.iter().map(|c| c.color_idx).max().unwrap_or(0) + 1;
    let plan = recipe(cfg.variant);
    let blocks = match plan.shape {
        Shape::Packed => packed_blocks(&kept, plan, branch_count),
        Shape::Grid => grid_blocks(&kept, branch_count),
    };
    let names = blocks.iter().map(|b| kept[b.ci].label.clone()).collect();
    (blocks, names)
}

fn clone_circle(c: &Circle) -> Circle {
    Circle { x: c.x, y: c.y, r: c.r, depth: c.depth, color_idx: c.color_idx, label: c.label.clone(), children: c.children.clone() }
}

pub fn layout_3d(cfg: &CirclePackConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &CirclePackConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    circle_pack_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> (Vec<String>, Vec<String>, Vec<f64>) {
        (
            ["Root", "A", "B", "A1", "A2"].iter().map(|s| s.to_string()).collect(),
            ["", "Root", "Root", "A", "A"].iter().map(|s| s.to_string()).collect(),
            vec![0.0, 40.0, 30.0, 20.0, 20.0],
        )
    }

    fn draw(variant: CirclePackVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, parents, values) = tree();
        let cfg = CirclePackConfig { variant, labels: &labels, parents: &parents, values: &values, ..CirclePackConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_every_node_and_names_it() {
        for &variant in CirclePackVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert!(!names.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn packed_circles_keep_the_2d_positions_as_a_square_footprint() {
        let (blocks, _) = draw(CirclePackVariant::Basic);
        assert_eq!(blocks.len(), 5);
        assert!(blocks.iter().all(|b| b.hw == b.hd));
    }

    #[test]
    fn leaf_focus_gives_leaves_the_full_height_and_flattens_branches() {
        let (blocks, names) = draw(CirclePackVariant::LeafFocus);
        let root = &blocks[names.iter().position(|n| n == "Root").unwrap()];
        let a1 = &blocks[names.iter().position(|n| n == "A1").unwrap()];
        assert!(a1.z1 > root.z1);
    }

    #[test]
    fn matrix_lays_only_the_leaves_out_on_a_grid() {
        let (blocks, names) = draw(CirclePackVariant::Matrix);
        assert_eq!(blocks.len(), 3);
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["A1".to_string(), "A2".to_string(), "B".to_string()]);
        assert!(names.iter().all(|n| n != "Root" && n != "A"));
    }

    #[test]
    fn branches_share_a_tone_and_differ_from_other_branches() {
        let (blocks, names) = draw(CirclePackVariant::Basic);
        let a1 = blocks[names.iter().position(|n| n == "A1").unwrap()];
        let a2 = blocks[names.iter().position(|n| n == "A2").unwrap()];
        let b = blocks[names.iter().position(|n| n == "B").unwrap()];
        assert_eq!(a1.tone, a2.tone);
        assert_ne!(a1.tone, b.tone);
    }

    #[test]
    fn a_wide_flat_input_packs_every_circle_as_its_own_root() {
        let labels: Vec<String> = (0..6).map(|i| format!("L{i}")).collect();
        let values = vec![10.0, 20.0, 15.0, 5.0, 8.0, 12.0];
        let cfg = CirclePackConfig { labels: &labels, values: &values, ..CirclePackConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!((blocks.len(), names.len()), (6, 6));
    }

    #[test]
    fn a_large_tree_is_capped_by_the_node_budget_keeping_the_biggest_circles() {
        let mut labels = vec!["Root".to_string()];
        let mut parents = vec![String::new()];
        let mut values = vec![0.0];
        for k in 0..900 {
            labels.push(format!("n{k}"));
            parents.push("Root".to_string());
            values.push(1.0);
        }
        let cfg = CirclePackConfig { labels: &labels, parents: &parents, values: &values, ..CirclePackConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(blocks.len(), NODE_CAP);
        assert_eq!(names.len(), NODE_CAP);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&CirclePackConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_and_negative_values_are_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let values = vec![f64::NAN, f64::INFINITY, -5.0];
        let cfg = CirclePackConfig { labels: &labels, values: &values, ..CirclePackConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.cx.is_finite() && b.hw.is_finite() && b.z1.is_finite()));
    }
}
