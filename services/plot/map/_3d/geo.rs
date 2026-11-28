use crate::plot::map::regions::RegionSetEntry;
use crate::plot::map::svg_parser::CountryShape;
use crate::plot::statistical::_3d::budget::{even_indices, pick};
use crate::plot::statistical::_3d::lineage::Point;

pub const SCALE: f64 = 10.0;
pub const TRACE_PTS: usize = 10;
pub const MAX_TRACED_SHAPES: usize = 60;

pub fn geo_xy(nx: f32, ny: f32) -> (f64, f64) {
    (nx as f64 * SCALE, (1.0 - ny as f64) * SCALE)
}

pub fn outline_trace(region: &RegionSetEntry, shape: &CountryShape, max_pts: usize) -> Vec<Point> {
    let polys = (region.normalize)(shape);
    let Some(ring) = polys.iter().max_by_key(|p| p.len()) else {
        return Vec::new();
    };
    if ring.len() < 3 {
        return Vec::new();
    }
    let keep = even_indices(ring.len(), max_pts.min(ring.len()).max(3));
    let picked = pick(ring, &keep);
    let mut pts: Vec<Point> = picked
        .iter()
        .map(|&[x, y]| {
            let (gx, gy) = geo_xy(x, y);
            (gx, gy, 0.0)
        })
        .collect();
    if let Some(&first) = pts.first() {
        pts.push(first);
    }
    pts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::map::regions;

    #[test]
    fn geo_xy_flips_y_so_north_reads_as_a_larger_3d_y() {
        let (_, north) = geo_xy(0.5, 0.0);
        let (_, south) = geo_xy(0.5, 1.0);
        assert!(north > south);
    }

    #[test]
    fn geo_xy_scales_into_the_shared_unit_space() {
        let (x, y) = geo_xy(0.0, 0.0);
        assert!((x - 0.0).abs() < 1e-9);
        assert!((y - SCALE).abs() < 1e-9);
    }

    #[test]
    fn outline_trace_returns_a_closed_loop_capped_to_the_requested_size() {
        let region = regions::default_region_set().expect("world region set must be registered");
        let shape = &(region.all)()[0];
        let trace = outline_trace(region, shape, 8);
        assert!(trace.len() <= 9);
        assert_eq!(trace.first(), trace.last());
    }

    #[test]
    fn outline_trace_of_an_empty_shape_is_empty() {
        let region = regions::default_region_set().expect("world region set must be registered");
        let empty = CountryShape { id: "ZZ".to_string(), name: "Nowhere".to_string(), polygons: vec![] };
        assert!(outline_trace(region, &empty, 8).is_empty());
    }
}
