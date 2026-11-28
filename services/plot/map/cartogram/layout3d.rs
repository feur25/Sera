use super::config::CartogramConfig;
use crate::plot::map::_3d::geo::geo_xy;
use crate::plot::map::regions;
use crate::plot::map::world_data::latlon_to_normalized;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;
use std::collections::HashSet;

const HEIGHT: f64 = 2.6;
const FOOT: f64 = 0.2;

fn point_path(cfg: &CartogramConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let peak = cfg.values[..n].iter().map(|v| v.abs()).fold(1e-12, f64::max);
    let blocks: Vec<Bar3DBlock> = (0..n)
        .map(|i| {
            let (nx, ny) = latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
            let (cx, cy) = geo_xy(nx, ny);
            let h = (HEIGHT * (cfg.values[i].abs() / peak)).max(0.05);
            Bar3DBlock::new(cx, cy, 0.0, h, FOOT, FOOT, i).with_tone((cfg.values[i].abs() / peak).clamp(0.0, 1.0))
        })
        .collect();
    let names: Vec<String> = (0..n).map(|i| cfg.labels.get(i).cloned().unwrap_or_else(|| format!("Point {}", i + 1))).collect();
    (blocks, names)
}

fn region_path(cfg: &CartogramConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
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
        let h = (HEIGHT * (value.abs() / peak)).max(0.05);
        blocks.push(Bar3DBlock::new(cx, cy, 0.0, h, FOOT, FOOT, ci).with_tone((value.abs() / peak).clamp(0.0, 1.0)));
    }
    (blocks, names)
}

fn cartogram_3d(cfg: &CartogramConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.lats.len() >= 1 && cfg.lats.len() == cfg.lons.len() {
        point_path(cfg)
    } else {
        region_path(cfg)
    }
}

pub fn layout_3d(cfg: &CartogramConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &CartogramConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    cartogram_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::variant::CartogramVariant;

    fn draw_points(variant: CartogramVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["USA".to_string(), "CHN".to_string(), "IND".to_string()];
        let values = vec![331.0, 1412.0, 1408.0];
        let lats = vec![39.0, 35.0, 21.0];
        let lons = vec![-98.0, 105.0, 78.0];
        let mut cfg = CartogramConfig::new(region);
        cfg.variant = variant;
        cfg.labels = &labels;
        cfg.values = &values;
        cfg.lats = &lats;
        cfg.lons = &lons;
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_column_per_point_and_names_it() {
        for &variant in CartogramVariant::all() {
            let (blocks, names) = draw_points(variant);
            assert_eq!(names, vec!["USA".to_string(), "CHN".to_string(), "IND".to_string()], "{variant:?}");
            assert_eq!(blocks.len(), 3, "{variant:?}");
        }
    }

    #[test]
    fn a_bigger_value_stands_taller() {
        let (blocks, names) = draw_points(CartogramVariant::Dorling);
        let usa = blocks[names.iter().position(|n| n == "USA").unwrap()];
        let chn = blocks[names.iter().position(|n| n == "CHN").unwrap()];
        assert!(chn.z1 > usa.z1);
    }

    #[test]
    fn falls_back_to_region_lookup_when_no_lat_lon_is_given() {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["FR".to_string(), "DE".to_string()];
        let values = vec![10.0, 20.0];
        let mut cfg = CartogramConfig::new(region);
        cfg.labels = &labels;
        cfg.values = &values;
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, vec!["FR".to_string(), "DE".to_string()]);
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let region = regions::default_region_set().unwrap();
        let cfg = CartogramConfig::new(region);
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
