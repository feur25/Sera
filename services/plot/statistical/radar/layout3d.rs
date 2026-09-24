use super::variant::RadarVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::generic::{radial_band_columns, radial_columns, radial_stacked_columns};
use crate::plot::statistical::_3d::lineage::{paths, Point, EDGE_STEPS};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::{FRAC_PI_2, TAU};

const LOOP_RADIUS: f64 = 2.6;
const LOOP_HW: f64 = 0.05;
const ROW_STEP: f64 = 1.2;
const RING_HW: f64 = 0.18;
const RING_BASE: f64 = 2.4;
const RING_STEP: f64 = 0.9;

fn closed_loop_3d(axes: &[String], series: &[(String, Vec<f64>)]) -> Vec<Bar3DBlock> {
    let n_axes = axes.len();
    if n_axes == 0 {
        return Vec::new();
    }
    let n_series = series.len().max(1);
    let mut blocks = Vec::new();
    for (si, (_, vals)) in series.iter().enumerate() {
        let max_val = vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);
        let row = si as f64 * ROW_STEP;
        let mut loop_pts: Vec<Point> = (0..n_axes.min(vals.len()))
            .map(|ai| {
                let a = TAU * ai as f64 / n_axes as f64 - FRAC_PI_2;
                let r = LOOP_RADIUS * (vals[ai] / max_val).max(0.05);
                (r * a.cos(), r * a.sin(), row)
            })
            .collect();
        if let Some(&first) = loop_pts.first() {
            loop_pts.push(first);
        }
        if loop_pts.len() < 2 {
            continue;
        }
        blocks.extend(paths(&[loop_pts], LOOP_HW, (EDGE_STEPS * n_axes).max(EDGE_STEPS), |_| si, |_| si as f64 / n_series as f64));
    }
    blocks
}

fn ring_glyph_3d(series: &[(String, Vec<f64>)], n_axes: usize, hw: f64) -> Vec<Bar3DBlock> {
    let mut blocks = Vec::new();
    for (si, (_, vals)) in series.iter().enumerate() {
        let radius = RING_BASE + si as f64 * RING_STEP;
        blocks.extend(radial_columns(&vals[..n_axes.min(vals.len())], radius, hw, hw));
    }
    blocks
}

fn radar_3d(axes: &[String], series: &[(String, Vec<f64>)], variant: RadarVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n_axes = axes.len();
    if n_axes == 0 || series.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let names: Vec<String> = series.iter().map(|(name, _)| name.clone()).collect();
    use RadarVariant::*;
    let blocks = match variant {
        PolarBar => ring_glyph_3d(series, n_axes, 0.18),
        Petal => ring_glyph_3d(series, n_axes, 0.32),
        Band if series.len() >= 2 => radial_band_columns(&series[0].1, &series[1].1, RING_BASE + RING_STEP, 0.3, 0.3),
        Band => ring_glyph_3d(series, n_axes, 0.3),
        Stacked => radial_stacked_columns(series, n_axes, RING_BASE + RING_STEP, 0.26, 0.26),
        Basic | Lines | Filled | Markers | Dashed => closed_loop_3d(axes, series),
    };
    (blocks, names)
}

pub fn layout_3d(axes: &[String], series: &[(String, Vec<f64>)], variant: RadarVariant, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(axes, series, variant, _budget).0
}

pub fn layout_named(axes: &[String], series: &[(String, Vec<f64>)], variant: RadarVariant, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    radar_3d(axes, series, variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        (
            ["Python", "Rust", "SQL", "ML", "DevOps"].iter().map(|s| s.to_string()).collect(),
            vec![("Alice".to_string(), vec![9.0, 7.0, 8.0, 8.0, 6.0]), ("Bob".to_string(), vec![5.0, 10.0, 6.0, 4.0, 9.0])],
        )
    }

    fn draw(variant: RadarVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (axes, series) = data();
        layout_named(&axes, &series, variant, &Budget::default())
    }

    #[test]
    fn every_variant_draws_something_per_series_and_names_every_series() {
        for &variant in RadarVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names, vec!["Alice".to_string(), "Bob".to_string()], "{variant:?}");
            assert!(!blocks.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn each_closed_loop_series_sits_at_its_own_row() {
        let (blocks, _) = draw(RadarVariant::Basic);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (((b.z0 + b.z1) / 2.0) * 1000.0) as i64).collect();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn petal_requests_a_wider_footprint_than_polar_bar() {
        let axes: Vec<String> = (0..40).map(|i| format!("A{i}")).collect();
        let series = vec![("S".to_string(), vec![1.0; 40])];
        let (petal, _) = layout_named(&axes, &series, RadarVariant::Petal, &Budget::default());
        let (polar_bar, _) = layout_named(&axes, &series, RadarVariant::PolarBar, &Budget::default());
        assert!(petal[0].hw > polar_bar[0].hw);
    }

    #[test]
    fn band_with_a_single_series_still_draws_a_ring_instead_of_nothing() {
        let (axes, series) = data();
        let single = &series[..1];
        let (blocks, names) = layout_named(&axes, single, RadarVariant::Band, &Budget::default());
        assert_eq!(names.len(), 1);
        assert!(!blocks.is_empty());
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&[], &[], RadarVariant::Basic, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
