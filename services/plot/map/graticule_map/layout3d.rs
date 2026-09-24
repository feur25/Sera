use super::common::{lat_steps, lon_steps};
use super::config::GraticuleMapConfig;
use crate::plot::map::_3d::geo::geo_xy;
use crate::plot::map::world_data::latlon_to_normalized;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{paths, Point};
use crate::plot::statistical::bar::Bar3DBlock;

const LINE_HW: f64 = 0.02;
const LINE_STEPS: usize = 8;

fn graticule_map_3d(cfg: &GraticuleMapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let step = cfg.step.max(1.0);
    let lats = lat_steps(step);
    let lons = lon_steps(step);
    if lats.is_empty() && lons.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut links: Vec<Vec<Point>> = Vec::with_capacity(lats.len() + lons.len());
    let mut names: Vec<String> = Vec::with_capacity(lats.len() + lons.len());

    for &lat in &lats {
        let a = latlon_to_normalized(lat, -180.0);
        let b = latlon_to_normalized(lat, 180.0);
        let (ax, ay) = geo_xy(a.0, a.1);
        let (bx, by) = geo_xy(b.0, b.1);
        links.push(vec![(ax, ay, 0.0), (bx, by, 0.0)]);
        names.push(format!("{lat:.0} deg lat"));
    }
    let n_parallels = links.len();
    for &lon in &lons {
        let a = latlon_to_normalized(-90.0, lon);
        let b = latlon_to_normalized(90.0, lon);
        let (ax, ay) = geo_xy(a.0, a.1);
        let (bx, by) = geo_xy(b.0, b.1);
        links.push(vec![(ax, ay, 0.0), (bx, by, 0.0)]);
        names.push(format!("{lon:.0} deg lon"));
    }

    let blocks = paths(&links, LINE_HW, LINE_STEPS, |li| li, |li| if li < n_parallels { 0.3 } else { 0.7 });
    (blocks, names)
}

pub fn layout_3d(cfg: &GraticuleMapConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &GraticuleMapConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    graticule_map_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::variant::GraticuleMapVariant;

    fn draw(variant: GraticuleMapVariant, step: f64) -> (Vec<Bar3DBlock>, Vec<String>) {
        let mut cfg = GraticuleMapConfig::new();
        cfg.variant = variant;
        cfg.step = step;
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_line_per_meridian_and_parallel() {
        for &variant in GraticuleMapVariant::all() {
            let (blocks, names) = draw(variant, 30.0);
            assert_eq!(names.len(), lat_steps(30.0).len() + lon_steps(30.0).len(), "{variant:?}");
            assert_eq!(blocks.len(), names.len() * (LINE_STEPS - 1), "{variant:?}");
        }
    }

    #[test]
    fn a_finer_step_draws_more_lines_than_a_coarser_one() {
        let (coarse, _) = draw(GraticuleMapVariant::Lines, 45.0);
        let (fine, _) = draw(GraticuleMapVariant::Lines, 10.0);
        assert!(fine.len() > coarse.len());
    }

    #[test]
    fn empty_input_still_has_a_sane_default_step() {
        let cfg = GraticuleMapConfig::new();
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty());
        assert!(!names.is_empty());
    }
}
