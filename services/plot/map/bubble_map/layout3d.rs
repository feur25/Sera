use super::config::BubbleMapConfig;
use super::variant::BubbleMapVariant;
use crate::plot::map::_3d::geo::geo_xy;
use crate::plot::map::regions;
use crate::plot::map::world_data::latlon_to_normalized;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;
use std::collections::HashSet;

const HEIGHT: f64 = 2.6;
const MIN_FOOT: f64 = 0.08;
const MAX_FOOT: f64 = 0.34;

#[derive(Clone, Copy)]
struct Recipe {
    min_foot: f64,
    max_foot: f64,
}

fn recipe(variant: BubbleMapVariant) -> Recipe {
    use BubbleMapVariant::*;
    match variant {
        Ring => Recipe { min_foot: MIN_FOOT * 1.4, max_foot: MAX_FOOT * 0.7 },
        _ => Recipe { min_foot: MIN_FOOT, max_foot: MAX_FOOT },
    }
}

fn point_value(cfg: &BubbleMapConfig, i: usize) -> f64 {
    if let Some(row) = cfg.series.get(i) {
        if !row.is_empty() {
            return row.iter().sum();
        }
    }
    cfg.values.get(i).copied().unwrap_or(1.0)
}

fn foot_of(value: f64, peak: f64, plan: Recipe) -> f64 {
    let t = (value.abs() / peak).sqrt();
    plan.min_foot + t * (plan.max_foot - plan.min_foot)
}

fn point_path(cfg: &BubbleMapConfig, plan: Recipe) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.lats.len().min(cfg.lons.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let values: Vec<f64> = (0..n).map(|i| point_value(cfg, i)).collect();
    let peak = values.iter().map(|v| v.abs()).fold(1e-12, f64::max);
    let blocks: Vec<Bar3DBlock> = (0..n)
        .map(|i| {
            let (nx, ny) = latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
            let (cx, cy) = geo_xy(nx, ny);
            let foot = foot_of(values[i], peak, plan);
            let h = (HEIGHT * (values[i].abs() / peak)).max(0.05);
            Bar3DBlock::new(cx, cy, 0.0, h, foot, foot, i).with_tone((values[i].abs() / peak).clamp(0.0, 1.0))
        })
        .collect();
    let names: Vec<String> = (0..n).map(|i| cfg.labels.get(i).cloned().unwrap_or_else(|| format!("Point {}", i + 1))).collect();
    (blocks, names)
}

fn region_path(cfg: &BubbleMapConfig, plan: Recipe) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let visible_ids: HashSet<&str> = visible.iter().map(|s| s.id.as_str()).collect();
    let matched: Vec<(usize, &_, f64)> = (0..n)
        .filter_map(|i| {
            let shape = (cfg.region.lookup)(&cfg.labels[i])?;
            visible_ids.contains(shape.id.as_str()).then_some((i, shape, cfg.values[i]))
        })
        .collect();
    if matched.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let peak = matched.iter().map(|(_, _, v)| v.abs()).fold(1e-12, f64::max);
    let mut blocks = Vec::new();
    let mut names = Vec::new();
    for &(i, shape, value) in &matched {
        let centroid = regions::centroid_of(cfg.region, shape);
        let (cx, cy) = geo_xy(centroid[0], centroid[1]);
        let ci = names.len();
        names.push(cfg.labels[i].clone());
        let foot = foot_of(value, peak, plan);
        let h = (HEIGHT * (value.abs() / peak)).max(0.05);
        blocks.push(Bar3DBlock::new(cx, cy, 0.0, h, foot, foot, ci).with_tone((value.abs() / peak).clamp(0.0, 1.0)));
    }
    (blocks, names)
}

fn bubble_map_3d(cfg: &BubbleMapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let plan = recipe(cfg.variant);
    if cfg.lats.len() >= 1 && cfg.lats.len() == cfg.lons.len() {
        point_path(cfg, plan)
    } else {
        region_path(cfg, plan)
    }
}

pub fn layout_3d(cfg: &BubbleMapConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &BubbleMapConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    bubble_map_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw_regions(variant: BubbleMapVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["FR".to_string(), "DE".to_string(), "US".to_string()];
        let values = vec![10.0, 5.0, 20.0];
        let mut cfg = BubbleMapConfig::new(region);
        cfg.variant = variant;
        cfg.labels = &labels;
        cfg.values = &values;
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_marker_per_matched_country_and_names_it() {
        for &variant in BubbleMapVariant::all() {
            let (blocks, names) = draw_regions(variant);
            assert_eq!(names.len(), 3, "{variant:?}");
            assert_eq!(blocks.len(), 3, "{variant:?}");
        }
    }

    #[test]
    fn ring_markers_are_thinner_than_proportional_markers() {
        let (ring, _) = draw_regions(BubbleMapVariant::Ring);
        let (prop, _) = draw_regions(BubbleMapVariant::Proportional);
        assert!(ring[2].hw < prop[2].hw);
    }

    #[test]
    fn raw_lat_lon_points_switch_to_the_point_path_instead_of_region_lookup() {
        let region = regions::default_region_set().unwrap();
        let lats = vec![40.7, 51.5, 35.7];
        let lons = vec![-74.0, -0.12, 139.7];
        let mut cfg = BubbleMapConfig::new(region);
        cfg.lats = &lats;
        cfg.lons = &lons;
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(blocks.len(), 3);
        assert_eq!(names, vec!["Point 1".to_string(), "Point 2".to_string(), "Point 3".to_string()]);
    }

    #[test]
    fn pie_markers_sizes_a_point_by_the_sum_of_its_series_row() {
        let region = regions::default_region_set().unwrap();
        let lats = vec![40.7, 51.5];
        let lons = vec![-74.0, -0.12];
        let series = vec![vec![40.0, 25.0, 35.0], vec![5.0, 5.0, 5.0]];
        let mut cfg = BubbleMapConfig::new(region);
        cfg.variant = BubbleMapVariant::PieMarkers;
        cfg.lats = &lats;
        cfg.lons = &lons;
        cfg.series = &series;
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks[0].z1 > blocks[1].z1);
    }

    #[test]
    fn an_unmatched_label_is_dropped_instead_of_drawing_a_blank_marker() {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["FR".to_string(), "ATLANTIS".to_string()];
        let values = vec![10.0, 99.0];
        let mut cfg = BubbleMapConfig::new(region);
        cfg.labels = &labels;
        cfg.values = &values;
        let (_, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, vec!["FR".to_string()]);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let region = regions::default_region_set().unwrap();
        let cfg = BubbleMapConfig::new(region);
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
