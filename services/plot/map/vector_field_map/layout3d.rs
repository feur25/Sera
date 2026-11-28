use super::config::VectorFieldMapConfig;
use crate::plot::map::_3d::geo::geo_xy;
use crate::plot::map::world_data::latlon_to_normalized;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{weighted_paths, Point, EDGE_STEPS};
use crate::plot::statistical::bar::Bar3DBlock;

const ARROW_DEG: f64 = 0.6;
const MIN_HW: f64 = 0.015;
const MAX_HW: f64 = 0.05;
const HEIGHT_SCALE: f64 = 1.2;

fn vector_field_map_3d(cfg: &VectorFieldMapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.u.len()).min(cfg.v.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let mags: Vec<f64> = (0..n).map(|i| (cfg.u[i] * cfg.u[i] + cfg.v[i] * cfg.v[i]).sqrt()).collect();
    let peak = mags.iter().copied().fold(1e-12, f64::max);

    let links: Vec<Vec<Point>> = (0..n)
        .map(|i| {
            let (bx0, by0) = latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
            let (bx, by) = geo_xy(bx0, by0);
            let tip_lat = cfg.lats[i] + cfg.v[i] * ARROW_DEG;
            let tip_lon = cfg.lons[i] + cfg.u[i] * ARROW_DEG;
            let (tx0, ty0) = latlon_to_normalized(tip_lat, tip_lon);
            let (tx, ty) = geo_xy(tx0, ty0);
            let h = (mags[i] / peak) * HEIGHT_SCALE;
            vec![(bx, by, 0.0), (tx, ty, h)]
        })
        .collect();
    let blocks = weighted_paths(&links, &mags, MIN_HW, MAX_HW, EDGE_STEPS, |i| i, |i| (mags[i] / peak).clamp(0.0, 1.0));
    let names: Vec<String> = (0..n).map(|i| format!("Point {}", i + 1)).collect();
    (blocks, names)
}

pub fn layout_3d(cfg: &VectorFieldMapConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &VectorFieldMapConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    vector_field_map_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::variant::VectorFieldMapVariant;

    fn draw(variant: VectorFieldMapVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let lats = vec![-55.0, -35.0, -15.0, 15.0];
        let lons = vec![-150.0, -90.0, -30.0, 30.0];
        let u = vec![-12.0, 30.0, -18.0, -18.0];
        let v = vec![3.0, -3.0, 6.0, -6.0];
        let cfg = VectorFieldMapConfig { variant, title: "t", lats: &lats, lons: &lons, u: &u, v: &v, width: 1200, height: 650, color_low: 0, color_high: 0 };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_vector_per_grid_point_and_names_it() {
        for &variant in VectorFieldMapVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 4, "{variant:?}");
            assert!(!blocks.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn a_stronger_vector_gets_a_thicker_segment() {
        let (blocks, _) = draw(VectorFieldMapVariant::Arrows);
        let widest = blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        let narrowest = blocks.iter().map(|b| b.hw).fold(f64::INFINITY, f64::min);
        assert!(widest > narrowest);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let cfg = VectorFieldMapConfig { variant: VectorFieldMapVariant::Arrows, title: "t", lats: &[], lons: &[], u: &[], v: &[], width: 1200, height: 650, color_low: 0, color_high: 0 };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
