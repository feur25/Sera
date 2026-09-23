use super::config::PlotWebConfig;
use super::variant::PlotWebVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::{PI, TAU};

pub const HEIGHT_RATIO: f64 = 0.6;
pub const COLORMAP: &str = "jet";
const BASE_SIZE: f64 = 0.12;
const MAX_SIZE: f64 = 0.4;
const BASE_HEIGHT: f64 = 1.2;
const INNER_R: f64 = 1.0;
const OUTER_R: f64 = 4.0;

fn group_order(groups: &[String]) -> Vec<String> {
    let mut order = Vec::new();
    for g in groups {
        if !order.contains(g) {
            order.push(g.clone());
        }
    }
    order
}

fn normalized(values: &[f64], log: bool) -> Vec<f64> {
    let mapped: Vec<f64> = if log { values.iter().map(|&v| v.max(1e-9).log10()).collect() } else { values.to_vec() };
    let (lo, hi) = mapped.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if hi > lo { mapped.iter().map(|&v| (v - lo) / (hi - lo)).collect() } else { mapped.iter().map(|_| 0.5).collect() }
}

pub fn layout_3d(cfg: &PlotWebConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &PlotWebConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.x_values.len().min(cfg.y_values.len()).min(cfg.labels.len()).min(cfg.groups.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let keep = even_indices(n, budget.cloud());
    let x = pick(cfg.x_values, &keep);
    let y = pick(cfg.y_values, &keep);
    let labels = pick(cfg.labels, &keep);
    let groups = pick(cfg.groups, &keep);
    let sizes = pick(cfg.sizes, &keep);
    let order = group_order(&groups);
    let class_of = |g: &str| order.iter().position(|o| o == g).unwrap_or(0);

    let nx = normalized(&x, cfg.x_log);
    let ny = normalized(&y, false);
    let has_sizes = sizes.len() == keep.len();
    let ns: Vec<f64> = if has_sizes { normalized(&sizes, false) } else { vec![0.5; keep.len()] };

    let blocks: Vec<Bar3DBlock> = if matches!(cfg.variant, PlotWebVariant::Radial) {
        let ng = order.len().max(1);
        (0..keep.len())
            .map(|i| {
                let gi = class_of(&groups[i]);
                let angle = TAU * gi as f64 / ng as f64 - PI / 2.0;
                let r = INNER_R + nx[i] * (OUTER_R - INNER_R);
                let (cx, cy) = (r * angle.cos(), r * angle.sin());
                let size = BASE_SIZE + ns[i] * (MAX_SIZE - BASE_SIZE);
                Bar3DBlock::new(cx, cy, 0.0, (BASE_HEIGHT * ny[i]).max(0.05), size, size, gi)
            })
            .collect()
    } else {
        (0..keep.len())
            .map(|i| {
                let size = if has_sizes { BASE_SIZE + ns[i] * (MAX_SIZE - BASE_SIZE) } else { BASE_SIZE };
                Bar3DBlock::new(nx[i] * OUTER_R * 2.0, ny[i] * OUTER_R * 2.0, 0.0, size * 2.0, size, size, class_of(&groups[i]))
            })
            .collect()
    };
    (blocks, labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg_of() -> (Vec<f64>, Vec<f64>, Vec<String>, Vec<String>) {
        (
            vec![12.0, 25.0, 40.0, 55.0, 70.0, 85.0],
            vec![18.0, 42.0, 28.0, 55.0, 38.0, 72.0],
            ["A", "B", "C", "D", "E", "F"].iter().map(|s| s.to_string()).collect(),
            ["G1", "G1", "G2", "G2", "G3", "G3"].iter().map(|s| s.to_string()).collect(),
        )
    }

    fn draw(variant: PlotWebVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (x, y, labels, groups) = cfg_of();
        let cfg = PlotWebConfig { variant, x_values: &x, y_values: &y, labels: &labels, groups: &groups, ..PlotWebConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_plot_web_variant_draws_every_point_and_names_it() {
        for &variant in PlotWebVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 6, "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn radial_places_the_same_group_on_the_same_spoke() {
        let (blocks, _) = draw(PlotWebVariant::Radial);
        let angle_of = |b: &Bar3DBlock| b.cy.atan2(b.cx);
        assert!((angle_of(&blocks[0]) - angle_of(&blocks[1])).abs() < 1e-9);
        assert!((angle_of(&blocks[2]) - angle_of(&blocks[3])).abs() < 1e-9);
        assert!((angle_of(&blocks[0]) - angle_of(&blocks[2])).abs() > 1e-6);
    }

    #[test]
    fn scatter_spreads_points_by_their_own_x_and_y_position() {
        let (blocks, _) = draw(PlotWebVariant::Scatter);
        let xs: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cx * 1000.0) as i64).collect();
        assert_eq!(xs.len(), 6);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&PlotWebConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_point_cloud_is_capped_by_the_budget() {
        let n = 300_000;
        let x: Vec<f64> = (0..n).map(|i| (i % 100) as f64).collect();
        let y: Vec<f64> = (0..n).map(|i| ((i * 3) % 100) as f64).collect();
        let labels: Vec<String> = (0..n).map(|i| format!("L{i}")).collect();
        let groups: Vec<String> = (0..n).map(|i| format!("G{}", i % 4)).collect();
        let cfg = PlotWebConfig { x_values: &x, y_values: &y, labels: &labels, groups: &groups, ..PlotWebConfig::default() };
        let budget = Budget::new(Some(600));
        let (blocks, _) = layout_named(&cfg, &budget);
        assert_eq!(blocks.len(), budget.cloud());
    }
}
