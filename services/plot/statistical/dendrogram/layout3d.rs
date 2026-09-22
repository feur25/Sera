use super::common::{tree_for, TreeNode};
use super::config::DendrogramConfig;
use super::variant::DendrogramVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::branch_tone;
use crate::plot::statistical::_3d::lineage::{markers, midpoints, paths, Point, EDGE_HW, EDGE_STEPS, NODE_HW};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::TAU;

pub const HEIGHT_RATIO: f64 = 0.75;
const ROW_STEP: f64 = 1.3;
const COMPACT_STEP: f64 = 0.75;
const SPREAD: f64 = 1.5;
const RADIAL_HOLE: f64 = 1.0;
const RADIAL_STEP: f64 = 1.1;
const RADIAL_TIGHT: f64 = 0.7;
const TRUNK_TONE: f64 = 0.05;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Vertical,
    Horizontal,
    Radial,
}

#[derive(Clone, Copy)]
struct Recipe {
    shape: Shape,
    elbow: bool,
    row_step: f64,
}

impl Recipe {
    const fn of(shape: Shape) -> Self {
        Self { shape, elbow: true, row_step: ROW_STEP }
    }

    const fn diagonal(mut self) -> Self {
        self.elbow = false;
        self
    }

    const fn stepped(mut self, step: f64) -> Self {
        self.row_step = step;
        self
    }
}

fn recipe(variant: DendrogramVariant) -> Recipe {
    use DendrogramVariant::*;
    match variant {
        Vertical => Recipe::of(Shape::Vertical),
        Horizontal => Recipe::of(Shape::Horizontal),
        Compact => Recipe::of(Shape::Vertical).stepped(COMPACT_STEP),
        Elegant => Recipe::of(Shape::Vertical).diagonal(),
        Triangular => Recipe::of(Shape::Vertical).diagonal(),
        Radial => Recipe::of(Shape::Radial).stepped(RADIAL_STEP),
        Genealogy => Recipe::of(Shape::Radial).stepped(RADIAL_STEP),
        Bloom => Recipe::of(Shape::Radial).stepped(RADIAL_TIGHT),
    }
}

fn positions(nodes: &[TreeNode], plan: Recipe, leaf_count: f64) -> Vec<Point> {
    match plan.shape {
        Shape::Vertical => nodes.iter().map(|n| (n.x * SPREAD, 0.0, n.depth as f64 * plan.row_step)).collect(),
        Shape::Horizontal => nodes.iter().map(|n| (n.depth as f64 * plan.row_step, 0.0, n.x * SPREAD)).collect(),
        Shape::Radial => nodes
            .iter()
            .map(|n| {
                let a = TAU * n.x / leaf_count.max(1.0);
                let r = RADIAL_HOLE + n.depth as f64 * plan.row_step;
                (r * a.cos(), r * a.sin(), 0.0)
            })
            .collect(),
    }
}

fn link_path(parent: Point, child: Point, elbow: bool) -> Vec<Point> {
    if elbow {
        vec![parent, (parent.0, parent.1, child.2), child]
    } else {
        vec![parent, child]
    }
}

fn dendrogram_3d(cfg: &DendrogramConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some((mut nodes, roots)) = tree_for(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let n = nodes.len();
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, node) in nodes.iter().enumerate() {
        if let Some(p) = node.parent {
            children[p].push(i);
        }
    }
    let mut order = vec![0.0; n];
    let mut next_leaf = 0.0;
    for &r in &roots {
        midpoints(&children, &mut order, r, &mut next_leaf);
    }
    for (i, node) in nodes.iter_mut().enumerate() {
        node.x = order[i];
    }
    let plan = recipe(cfg.variant);
    let pos = positions(&nodes, plan, next_leaf);
    let branch_count = nodes.iter().map(|n| n.color_idx).max().unwrap_or(0) + 1;
    let node_tones: Vec<f64> = nodes.iter().map(|n| if n.above_cut { TRUNK_TONE } else { branch_tone(n.color_idx, branch_count) }).collect();
    let mut blocks = markers(&pos, NODE_HW, |i| i, |i| node_tones[i]);
    let links: Vec<Vec<Point>> = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| node.parent.map(|p| link_path(pos[p], pos[i], plan.elbow)))
        .collect();
    let edge_tones: Vec<f64> = nodes.iter().enumerate().filter(|(_, n)| n.parent.is_some()).map(|(i, _)| node_tones[i]).collect();
    blocks.extend(paths(&links, EDGE_HW, EDGE_STEPS, |li| li, |li| edge_tones[li]));
    let names = nodes.iter().map(|n| n.label.clone()).collect();
    (blocks, names)
}

pub fn layout_3d(cfg: &DendrogramConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &DendrogramConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    dendrogram_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> (Vec<String>, Vec<String>) {
        (
            ["Root", "A", "B", "A1", "A2"].iter().map(|s| s.to_string()).collect(),
            ["", "Root", "Root", "A", "A"].iter().map(|s| s.to_string()).collect(),
        )
    }

    fn draw(variant: DendrogramVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, parents) = tree();
        let cfg = DendrogramConfig { variant, labels: &labels, parents: &parents, ..DendrogramConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_marker_per_node_plus_edges_and_names_every_node() {
        for &variant in DendrogramVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 5, "{variant:?}");
            assert!(blocks.len() > 5, "{variant:?}");
        }
    }

    #[test]
    fn leaves_are_spread_in_order_and_a_parent_sits_at_their_midpoint() {
        let (blocks, names) = draw(DendrogramVariant::Vertical);
        let a1 = blocks[names.iter().position(|n| n == "A1").unwrap()];
        let a2 = blocks[names.iter().position(|n| n == "A2").unwrap()];
        let a = blocks[names.iter().position(|n| n == "A").unwrap()];
        assert!((a.cx - (a1.cx + a2.cx) / 2.0).abs() < 1e-9);
    }

    #[test]
    fn vertical_grows_in_z_by_depth_and_horizontal_swaps_the_depth_axis() {
        let (vertical, names) = draw(DendrogramVariant::Vertical);
        let root = vertical[names.iter().position(|n| n == "Root").unwrap()];
        let a1v = vertical[names.iter().position(|n| n == "A1").unwrap()];
        assert!(a1v.z0 > root.z0);
        let (horizontal, names2) = draw(DendrogramVariant::Horizontal);
        let a1h = horizontal[names2.iter().position(|n| n == "A1").unwrap()];
        assert!(a1h.cx > 0.0);
    }

    #[test]
    fn radial_places_every_node_at_a_radius_that_grows_with_depth() {
        let (blocks, names) = draw(DendrogramVariant::Radial);
        let root = blocks[names.iter().position(|n| n == "Root").unwrap()];
        let a1 = blocks[names.iter().position(|n| n == "A1").unwrap()];
        assert!(a1.cx.hypot(a1.cy) > root.cx.hypot(root.cy));
    }

    #[test]
    fn an_elbow_link_keeps_the_parents_x_until_it_reaches_the_childs_depth() {
        let elbow = link_path((0.0, 0.0, 0.0), (3.0, 0.0, 5.0), true);
        assert_eq!(elbow, vec![(0.0, 0.0, 0.0), (0.0, 0.0, 5.0), (3.0, 0.0, 5.0)]);
        let straight = link_path((0.0, 0.0, 0.0), (3.0, 0.0, 5.0), false);
        assert_eq!(straight, vec![(0.0, 0.0, 0.0), (3.0, 0.0, 5.0)]);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&DendrogramConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn a_single_node_still_draws_its_own_marker() {
        let labels = vec!["Root".to_string()];
        let parents = vec![String::new()];
        let cfg = DendrogramConfig { labels: &labels, parents: &parents, ..DendrogramConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!((blocks.len(), names.len()), (1, 1));
    }
}
