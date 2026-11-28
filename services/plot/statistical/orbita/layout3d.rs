use super::config::OrbitaConfig;
use super::variant::OrbitaVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::curves::marker_blocks;
use crate::plot::statistical::_3d::ohlc::{DOWN, UP};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::{PI, TAU};

pub const HEIGHT_RATIO: f64 = 0.6;
pub const COLORMAP: &str = "jet";
const MARKER_SIZE: f64 = 0.16;
const BUBBLE_MAX: f64 = 0.42;
const BASE_HEIGHT: f64 = 0.12;
const TRAIL_HEIGHT: f64 = 0.06;
const MAX_SERIES: usize = 12;
const MAX_LABELS: usize = 48;

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Dots,
    Bubble,
    Trail,
    Delta,
}

fn recipe(variant: OrbitaVariant) -> Style {
    use OrbitaVariant::*;
    match variant {
        Classic | Glow | Minimal => Style::Dots,
        Bubble => Style::Bubble,
        Trail => Style::Trail,
        Delta => Style::Delta,
    }
}

pub fn layout_3d(cfg: &OrbitaConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &OrbitaConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let capped_names: Vec<String>;
    let capped_labels: Vec<String>;
    let capped_matrix: Vec<f64>;
    let scoped_cfg: OrbitaConfig;
    let cfg = if cfg.series_names.len() > MAX_SERIES || cfg.labels.len() > MAX_LABELS {
        let ns0 = cfg.series_names.len();
        let nc0 = cfg.labels.len();
        let ns = ns0.min(MAX_SERIES);
        let nc = nc0.min(MAX_LABELS);
        capped_names = cfg.series_names[..ns].to_vec();
        capped_labels = cfg.labels[..nc].to_vec();
        capped_matrix = (0..ns).flat_map(|si| (0..nc).map(move |ci| (si, ci))).filter_map(|(si, ci)| cfg.matrix.get(si * nc0 + ci).copied()).collect();
        scoped_cfg = OrbitaConfig { variant: cfg.variant, series_names: &capped_names, labels: &capped_labels, matrix: &capped_matrix, ..OrbitaConfig::default() };
        &scoped_cfg
    } else {
        cfg
    };
    let ns = cfg.series_names.len();
    let nc = cfg.labels.len();
    if ns == 0 || nc == 0 || cfg.matrix.len() < ns * nc {
        return (Vec::new(), Vec::new());
    }
    let style = recipe(cfg.variant);
    let max_v = cfg.matrix.iter().copied().filter(|v| v.is_finite()).fold(0.0_f64, f64::max).max(1e-9);
    let angle_step = TAU / nc as f64;
    let angle_of = |ci: usize| -PI / 2.0 + ci as f64 * angle_step;

    let mut names = Vec::with_capacity(ns * nc);
    let mut points = Vec::with_capacity(ns * nc);
    let mut classes = Vec::with_capacity(ns * nc);
    let mut tones = Vec::with_capacity(ns * nc);
    let mut sizes = Vec::with_capacity(ns * nc);
    for si in 0..ns {
        let orbit_r = (si as f64 + 1.0) / ns as f64 * 4.0;
        for ci in 0..nc {
            let v = cfg.matrix[si * nc + ci];
            let vv = if v.is_finite() { v.max(0.0) } else { 0.0 };
            let frac = (vv / max_v).clamp(0.0, 1.0);
            let angle = angle_of(ci);
            points.push((orbit_r * angle.cos(), orbit_r * angle.sin()));
            classes.push(si);
            sizes.push(if style == Style::Bubble { MARKER_SIZE + frac * (BUBBLE_MAX - MARKER_SIZE) } else { MARKER_SIZE });
            let tone = if style == Style::Delta && si > 0 {
                let prev = cfg.matrix[(si - 1) * nc + ci];
                if vv > prev { Some(UP) } else if vv < prev { Some(DOWN) } else { None }
            } else {
                None
            };
            tones.push(tone);
            names.push(format!("{} \u{b7} {}", cfg.series_names[si], cfg.labels[ci]));
        }
    }

    let mut blocks: Vec<Bar3DBlock> = (0..points.len())
        .map(|i| {
            let (x, y) = points[i];
            let block = Bar3DBlock::new(x, y, 0.0, BASE_HEIGHT, sizes[i], sizes[i], classes[i]);
            match tones[i] {
                Some(t) => block.with_tone(t),
                None => block,
            }
        })
        .collect();

    if style == Style::Trail {
        for si in 0..ns {
            let ring: Vec<(f64, f64)> = (0..=nc).map(|ci| points[si * nc + ci % nc]).collect();
            blocks.extend(marker_blocks(&ring, MARKER_SIZE * 0.4, TRAIL_HEIGHT, 0.0, si, None));
        }
    }
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(variant: OrbitaVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let series_names = ["2021", "2022", "2023"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let labels = ["Q1", "Q2", "Q3", "Q4"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let matrix = vec![0.4, 0.7, 0.5, 0.8, 0.6, 0.3, 0.9, 0.5, 0.8, 0.6, 0.4, 0.7];
        let cfg = OrbitaConfig { variant, series_names: &series_names, labels: &labels, matrix: &matrix, ..OrbitaConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_orbita_variant_draws_every_point_and_names_it() {
        for &variant in OrbitaVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(blocks.len() >= 12, "{variant:?}");
            assert_eq!(names.len(), 12, "{variant:?}");
            assert!(names.contains(&"2021 \u{b7} Q1".to_string()));
        }
    }

    #[test]
    fn each_series_sits_on_its_own_orbit_radius() {
        let (blocks, _) = draw(OrbitaVariant::Classic);
        let radii: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cx.hypot(b.cy) * 1000.0) as i64).collect();
        assert_eq!(radii.len(), 3);
    }

    #[test]
    fn bubble_scales_marker_size_with_the_value() {
        let (blocks, _) = draw(OrbitaVariant::Bubble);
        let widest = blocks.iter().map(|b| b.hw).fold(0.0, f64::max);
        let narrowest = blocks.iter().map(|b| b.hw).fold(f64::INFINITY, f64::min);
        assert!(widest > narrowest);
    }

    #[test]
    fn delta_tones_rising_and_falling_points_against_the_previous_orbit() {
        let (blocks, names) = draw(OrbitaVariant::Delta);
        let idx = names.iter().position(|n| n == "2022 \u{b7} Q1").unwrap();
        assert_eq!(blocks[idx].tone, Some(crate::plot::statistical::_3d::ohlc::UP));
    }

    #[test]
    fn trail_adds_ring_markers_beyond_the_plain_dots() {
        let (classic, _) = draw(OrbitaVariant::Classic);
        let (trail, _) = draw(OrbitaVariant::Trail);
        assert!(trail.len() > classic.len());
    }

    #[test]
    fn a_huge_series_and_label_count_are_capped_before_the_grid_is_built() {
        let series_names: Vec<String> = (0..500).map(|i| format!("S{i}")).collect();
        let labels: Vec<String> = (0..500).map(|i| format!("L{i}")).collect();
        let matrix: Vec<f64> = (0..500 * 500).map(|i| (i % 13) as f64).collect();
        let cfg = OrbitaConfig { series_names: &series_names, labels: &labels, matrix: &matrix, ..OrbitaConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(names.len() <= MAX_SERIES * MAX_LABELS);
        assert!(blocks.len() < 12_000);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&OrbitaConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let series_names = vec!["A".to_string()];
        let labels = vec!["X".to_string(), "Y".to_string()];
        let matrix = vec![f64::NAN, f64::INFINITY];
        let cfg = OrbitaConfig { series_names: &series_names, labels: &labels, matrix: &matrix, ..OrbitaConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.cx.is_finite() && b.hw.is_finite()));
    }
}
