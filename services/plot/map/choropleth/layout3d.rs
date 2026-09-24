use super::config::ChoroplethConfig;
use super::daynight::terminator_lat_deg;
use super::variant::ChoroplethVariant;
use crate::plot::map::_3d::geo::{geo_xy, outline_trace, MAX_TRACED_SHAPES, TRACE_PTS};
use crate::plot::map::regions;
use crate::plot::map::world_data::latlon_to_normalized;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::lineage::{paths, Point, EDGE_STEPS};
use crate::plot::statistical::_3d::ohlc::{DOWN, UP};
use crate::plot::statistical::bar::Bar3DBlock;
use std::collections::{HashMap, HashSet};

const HEIGHT: f64 = 2.6;
const FOOT: f64 = 0.16;
const OUTLINE_HW: f64 = 0.03;
const LON_MIN: f64 = -169.110266;
const LON_MAX: f64 = 190.486279;
const TERM_STEPS: usize = 48;
const SUN_HEIGHT: f64 = 1.4;

#[derive(Clone, Copy, PartialEq)]
enum Tone {
    ByValue,
    Diverging,
    Binned,
    Bivariate,
}

fn recipe(variant: ChoroplethVariant) -> Tone {
    use ChoroplethVariant::*;
    match variant {
        Sequential | Orthographic | Polar | DotDensity | DayNight => Tone::ByValue,
        Binned => Tone::Binned,
        Diverging => Tone::Diverging,
        Bivariate => Tone::Bivariate,
    }
}

fn daynight_3d(cfg: &ChoroplethConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let sub_lat = cfg.center_lat.unwrap_or(0.0);
    let sub_lon = cfg.center_lon.unwrap_or(0.0);
    let term: Vec<Point> = (0..=TERM_STEPS)
        .map(|i| {
            let lon = LON_MIN + (LON_MAX - LON_MIN) * i as f64 / TERM_STEPS as f64;
            let lat = terminator_lat_deg(lon, sub_lat, sub_lon);
            let (nx, ny) = latlon_to_normalized(lat, lon);
            let (gx, gy) = geo_xy(nx, ny);
            (gx, gy, 0.0)
        })
        .collect();
    let mut blocks = paths(&[term], OUTLINE_HW, TERM_STEPS, |_| 0, |_| 0.5);
    let (snx, sny) = latlon_to_normalized(sub_lat, sub_lon);
    let (sx, sy) = geo_xy(snx, sny);
    blocks.push(Bar3DBlock::new(sx, sy, 0.0, SUN_HEIGHT, FOOT * 1.5, FOOT * 1.5, 1).with_tone(1.0));
    (blocks, vec!["Terminator".to_string(), "Sun".to_string()])
}

fn choropleth_3d(cfg: &ChoroplethConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.variant == ChoroplethVariant::DayNight {
        return daynight_3d(cfg);
    }
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
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
    let peak_sv = matched.iter().map(|(j, _, _)| cfg.secondary_values.get(*j).copied().unwrap_or(0.0).abs()).fold(1e-12, f64::max);
    let trace_all = visible.len() <= MAX_TRACED_SHAPES;
    let mut trace_cache: HashMap<&str, Vec<Point>> = HashMap::new();
    let mut blocks = Vec::new();
    let mut names = Vec::new();

    for &(i, shape, value) in &matched {
        let centroid = regions::centroid_of(cfg.region, shape);
        let (cx, cy) = geo_xy(centroid[0], centroid[1]);
        let ci = names.len();
        names.push(cfg.labels[i].clone());

        let (z0, z1, tone) = match plan {
            Tone::Diverging => {
                let h = (HEIGHT * (value.abs() / peak)).max(0.05);
                let t = if value >= 0.0 { UP } else { DOWN };
                if value >= 0.0 {
                    (0.0, h, t)
                } else {
                    (-h, 0.0, t)
                }
            }
            Tone::Binned => {
                let bins = cfg.bins.max(1);
                let step = ((value.abs() / peak) * bins as f64).floor().clamp(0.0, bins as f64 - 1.0);
                (0.0, HEIGHT * (step + 1.0) / bins as f64, step / bins as f64)
            }
            Tone::Bivariate => {
                let sv = cfg.secondary_values.get(i).copied().unwrap_or(0.0);
                (0.0, (HEIGHT * (value.abs() / peak)).max(0.05), (sv.abs() / peak_sv).clamp(0.0, 1.0))
            }
            Tone::ByValue => (0.0, (HEIGHT * (value.abs() / peak)).max(0.05), (value.abs() / peak).clamp(0.0, 1.0)),
        };
        blocks.push(Bar3DBlock::new(cx, cy, z0, z1, FOOT, FOOT, ci).with_tone(tone));

        if trace_all {
            let trace = trace_cache.entry(shape.id.as_str()).or_insert_with(|| outline_trace(cfg.region, shape, TRACE_PTS));
            if trace.len() >= 2 {
                blocks.extend(paths(std::slice::from_ref(trace), OUTLINE_HW, EDGE_STEPS, |_| ci, |_| tone));
            }
        }
    }
    (blocks, names)
}

pub fn layout_3d(cfg: &ChoroplethConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &ChoroplethConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    choropleth_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(variant: ChoroplethVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["FR".to_string(), "DE".to_string(), "US".to_string()];
        let values = vec![10.0, -5.0, 20.0];
        let secondary_values = vec![0.2, 0.8, 0.4];
        let mut cfg = ChoroplethConfig::new(region);
        cfg.variant = variant;
        cfg.labels = &labels;
        cfg.values = &values;
        cfg.secondary_values = &secondary_values;
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_something_named_and_positioned() {
        for &variant in ChoroplethVariant::all() {
            let (blocks, names) = draw(variant);
            let expected = if variant == ChoroplethVariant::DayNight { 2 } else { 3 };
            assert_eq!(names.len(), expected, "{variant:?}");
            assert!(!blocks.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn daynight_draws_a_terminator_trace_and_a_sun_marker_with_no_country_data_at_all() {
        let region = regions::default_region_set().unwrap();
        let mut cfg = ChoroplethConfig::new(region);
        cfg.variant = ChoroplethVariant::DayNight;
        cfg.center_lat = Some(15.0);
        cfg.center_lon = Some(-40.0);
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, vec!["Terminator".to_string(), "Sun".to_string()]);
        assert!(blocks.len() > 2);
        let sun = blocks.last().unwrap();
        assert!(sun.z1 > 0.0);
    }

    #[test]
    fn columns_sit_at_different_geographic_positions() {
        let (blocks, _) = draw(ChoroplethVariant::Sequential);
        assert!((blocks[0].cx - blocks[2].cx).abs() > 1e-6 || (blocks[0].cy - blocks[2].cy).abs() > 1e-6);
    }

    #[test]
    fn diverging_sends_a_negative_value_below_the_floor() {
        let (blocks, names) = draw(ChoroplethVariant::Diverging);
        let de = blocks[names.iter().position(|n| n == "DE").unwrap()];
        assert!(de.z0 < 0.0 && de.z1 <= 0.0);
        let fr = blocks[names.iter().position(|n| n == "FR").unwrap()];
        assert!(fr.z1 > 0.0 && fr.z0 >= 0.0);
    }

    #[test]
    fn an_unmatched_label_is_dropped_instead_of_drawing_a_blank_column() {
        let region = regions::default_region_set().unwrap();
        let labels = vec!["FR".to_string(), "ATLANTIS".to_string()];
        let values = vec![10.0, 99.0];
        let mut cfg = ChoroplethConfig::new(region);
        cfg.labels = &labels;
        cfg.values = &values;
        let (_, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, vec!["FR".to_string()]);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let region = regions::default_region_set().unwrap();
        let cfg = ChoroplethConfig::new(region);
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn massive_duplicate_matches_reuse_a_cached_outline_trace_instead_of_recomputing_it() {
        let region = regions::resolve("usa_states").unwrap();
        let n = 500_000;
        let pool = ["CA", "TX", "NY", "FL", "IL"];
        let labels: Vec<String> = (0..n).map(|i| pool[i % pool.len()].to_string()).collect();
        let values: Vec<f64> = (0..n).map(|i| (i % 100) as f64).collect();
        let mut cfg = ChoroplethConfig::new(region);
        cfg.labels = &labels;
        cfg.values = &values;
        let t0 = std::time::Instant::now();
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names.len(), n);
        assert!(!blocks.is_empty());
        assert!(t0.elapsed().as_secs() < 3, "took {:?}", t0.elapsed());
    }
}
