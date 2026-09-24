use super::config::FlowMapConfig;
use super::variant::FlowMapVariant;
use crate::plot::map::_3d::geo::geo_xy;
use crate::plot::map::regions;
use crate::plot::map::world_data::latlon_to_normalized;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{markers, weighted_paths, Point, EDGE_STEPS, NODE_HW};
use crate::plot::statistical::bar::Bar3DBlock;

const ARC_HEIGHT: f64 = 1.6;
const MIN_EDGE: f64 = 0.03;

#[derive(Clone, Copy)]
struct Recipe {
    max_edge: f64,
}

fn recipe(variant: FlowMapVariant) -> Recipe {
    use FlowMapVariant::*;
    match variant {
        Ribbon | RangeRings => Recipe { max_edge: 0.24 },
        Straight | Track => Recipe { max_edge: 0.1 },
        Arc | Animated => Recipe { max_edge: 0.14 },
    }
}

fn node_positions(cfg: &FlowMapConfig, n: usize) -> Vec<Point> {
    if cfg.lats.len() >= n && cfg.lons.len() >= n && n > 0 {
        return (0..n)
            .map(|i| {
                let (nx, ny) = latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
                let (x, y) = geo_xy(nx, ny);
                (x, y, 0.0)
            })
            .collect();
    }
    (0..n)
        .map(|i| {
            let pos = cfg
                .labels
                .get(i)
                .and_then(|l| (cfg.region.lookup)(l))
                .map(|shape| regions::centroid_of(cfg.region, shape))
                .unwrap_or([0.5, 0.5]);
            let (x, y) = geo_xy(pos[0], pos[1]);
            (x, y, 0.0)
        })
        .collect()
}

fn track_path_3d(cfg: &FlowMapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.lats.len().min(cfg.lons.len());
    if n < 2 {
        return (Vec::new(), Vec::new());
    }
    let points: Vec<Point> = (0..n)
        .map(|i| {
            let (nx, ny) = latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
            let (x, y) = geo_xy(nx, ny);
            (x, y, 0.0)
        })
        .collect();
    let peak = cfg.track_values[..cfg.track_values.len().min(n)].iter().copied().fold(1e-12, f64::max);
    let mut blocks = markers(&points, NODE_HW, |i| i, |i| {
        (cfg.track_values.get(i).copied().unwrap_or(0.0) / peak).clamp(0.0, 1.0)
    });
    let links: Vec<Vec<Point>> = points.windows(2).map(|w| w.to_vec()).collect();
    let seg_weights: Vec<f64> = (0..links.len()).map(|i| cfg.track_values.get(i).copied().unwrap_or(1.0).max(0.1)).collect();
    let peak_seg = seg_weights.iter().copied().fold(1e-12, f64::max);
    blocks.extend(weighted_paths(&links, &seg_weights, MIN_EDGE, 0.14, EDGE_STEPS, |li| li, |li| seg_weights[li] / peak_seg));
    let names: Vec<String> = (0..n).map(|i| format!("Point {}", i + 1)).collect();
    (blocks, names)
}

fn flow_map_3d(cfg: &FlowMapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len();
    if n == 0 && cfg.sources.is_empty() {
        return track_path_3d(cfg);
    }
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let positions = node_positions(cfg, n);
    let mut blocks = markers(&positions, NODE_HW, |i| i, |i| i as f64 / n.max(1) as f64);

    let e = cfg.sources.len().min(cfg.targets.len());
    let edges: Vec<(Vec<Point>, f64, usize)> = (0..e)
        .filter_map(|k| {
            let s = cfg.sources[k] as usize;
            let t = cfg.targets[k] as usize;
            if s >= n || t >= n || s == t {
                return None;
            }
            let w = cfg.weights.get(k).copied().unwrap_or(1.0).max(0.0);
            let (sx, sy, _) = positions[s];
            let (tx, ty, _) = positions[t];
            let mid = ((sx + tx) / 2.0, (sy + ty) / 2.0, ARC_HEIGHT);
            let path = vec![(sx, sy, 0.0), mid, (tx, ty, 0.0)];
            Some((path, w, s))
        })
        .collect();
    let links: Vec<Vec<Point>> = edges.iter().map(|(p, _, _)| p.clone()).collect();
    let weights: Vec<f64> = edges.iter().map(|(_, w, _)| *w).collect();
    let sources_idx: Vec<usize> = edges.iter().map(|(_, _, s)| *s).collect();
    let peak = weights.iter().copied().fold(1e-12, f64::max);
    blocks.extend(weighted_paths(
        &links,
        &weights,
        MIN_EDGE,
        plan.max_edge,
        EDGE_STEPS,
        |li| sources_idx[li],
        |li| weights[li] / peak,
    ));
    (blocks, cfg.labels.to_vec())
}

pub fn layout_3d(cfg: &FlowMapConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &FlowMapConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    flow_map_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(variant: FlowMapVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["US".to_string(), "CN".to_string(), "DE".to_string(), "GB".to_string()];
        let sources = vec![0, 0, 1];
        let targets = vec![1, 2, 3];
        let weights = vec![420.0, 380.0, 210.0];
        let mut cfg = FlowMapConfig::new(region);
        cfg.variant = variant;
        cfg.labels = &labels;
        cfg.sources = &sources;
        cfg.targets = &targets;
        cfg.weights = &weights;
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_marker_per_node_plus_arcs_and_names_every_node() {
        for &variant in FlowMapVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 4, "{variant:?}");
            assert!(blocks.len() > 4, "{variant:?}");
        }
    }

    #[test]
    fn nodes_sit_at_different_geographic_positions() {
        let (blocks, _) = draw(FlowMapVariant::Arc);
        assert!((blocks[0].cx - blocks[1].cx).abs() > 1e-6 || (blocks[0].cy - blocks[1].cy).abs() > 1e-6);
    }

    #[test]
    fn raw_lat_lon_points_override_region_lookup_positions() {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["US".to_string(), "CN".to_string()];
        let sources = vec![0];
        let targets = vec![1];
        let weights = vec![1.0];
        let lats = vec![10.0, -10.0];
        let lons = vec![20.0, -20.0];
        let mut with_latlon = FlowMapConfig::new(region);
        with_latlon.labels = &labels;
        with_latlon.sources = &sources;
        with_latlon.targets = &targets;
        with_latlon.weights = &weights;
        with_latlon.lats = &lats;
        with_latlon.lons = &lons;
        let mut without_latlon = FlowMapConfig::new(region);
        without_latlon.labels = &labels;
        without_latlon.sources = &sources;
        without_latlon.targets = &targets;
        without_latlon.weights = &weights;
        let (with_blocks, _) = layout_named(&with_latlon, &Budget::default());
        let (without_blocks, _) = layout_named(&without_latlon, &Budget::default());
        assert!((with_blocks[0].cx - without_blocks[0].cx).abs() > 1e-6);
    }

    #[test]
    fn a_track_with_no_labels_or_edges_connects_its_points_in_sequence() {
        let region = regions::default_region_set().unwrap();
        let lats = vec![10.5, 12.1, 13.8, 15.9];
        let lons = vec![-38.5, -45.2, -50.1, -54.6];
        let track_values = vec![25.0, 35.0, 45.0, 60.0];
        let mut cfg = FlowMapConfig::new(region);
        cfg.lats = &lats;
        cfg.lons = &lons;
        cfg.track_values = &track_values;
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names.len(), 4);
        assert!(blocks.len() > 4);
    }

    #[test]
    fn ribbon_edges_are_thicker_than_straight_edges() {
        let (ribbon, _) = draw(FlowMapVariant::Ribbon);
        let (straight, _) = draw(FlowMapVariant::Straight);
        let max_hw = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw).fold(0.0f64, f64::max);
        assert!(max_hw(&ribbon) > max_hw(&straight));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let region = regions::default_region_set().unwrap();
        let cfg = FlowMapConfig::new(region);
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
