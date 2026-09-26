use super::config::BubbleConfig;
use super::variant::BubbleVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::ohlc::{DOWN, UP};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::{angle_at, hash01};
use std::f64::consts::PI;

pub const HEIGHT_RATIO: f64 = 0.5;
pub const COLORMAP: &str = "jet";
const SIZE_MIN: f64 = 0.08;
const SIZE_MAX: f64 = 0.5;
const HEIGHT: f64 = 0.14;
const GRID_STEP: f64 = 1.4;
const BURST_MIN_R: f64 = 0.6;
const BURST_MAX_R: f64 = 3.2;
const BURST_SPREAD: f64 = PI * 0.42;
const ROW_INNER: f64 = 1.0;
const ROW_STEP: f64 = 0.85;
const ROW_RISE: f64 = 0.55;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Points,
    Negative,
    Split,
    Burst,
    RadialRows,
}

fn recipe(variant: BubbleVariant) -> Glyph {
    use BubbleVariant::*;
    match variant {
        Basic | Categorical | Labeled | Outlined => Glyph::Points,
        Negative => Glyph::Negative,
        Split => Glyph::Split,
        Burst => Glyph::Burst,
        RadialRows => Glyph::RadialRows,
    }
}

fn order_of(values: &[String]) -> Vec<String> {
    let mut order = Vec::new();
    for v in values {
        if !order.contains(v) {
            order.push(v.clone());
        }
    }
    order
}

fn x_extent(x: &[f64], n: usize) -> (f64, f64) {
    if x.is_empty() {
        (0.0, (n.max(2) - 1) as f64)
    } else {
        x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)))
    }
}

fn size_frac(sizes: &[f64], i: usize, lo: f64, range: f64) -> f64 {
    sizes.get(i).map(|&s| ((s.abs() - lo) / range).clamp(0.0, 1.0)).unwrap_or(0.5)
}

pub fn layout_3d(cfg: &BubbleConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &BubbleConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let glyph = recipe(cfg.variant);

    if glyph == Glyph::Split {
        let n = cfg.x_categories.len().min(cfg.y_categories.len()).min(cfg.categories.len()).min(cfg.sizes.len());
        if n == 0 {
            return (Vec::new(), Vec::new());
        }
        let keep = even_indices(n, budget.cloud());
        let xc = pick(cfg.x_categories, &keep);
        let yc = pick(cfg.y_categories, &keep);
        let sc = pick(cfg.categories, &keep);
        let sizes = pick(cfg.sizes, &keep);
        let xo = order_of(&xc);
        let yo = order_of(&yc);
        let so = order_of(&sc);
        let (lo, hi) = sizes.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v.abs()), hi.max(v.abs())));
        let range = (hi - lo).max(1e-9);
        let blocks: Vec<Bar3DBlock> = (0..keep.len())
            .map(|i| {
                let xi = xo.iter().position(|c| c == &xc[i]).unwrap_or(0);
                let yi = yo.iter().position(|c| c == &yc[i]).unwrap_or(0);
                let si = so.iter().position(|c| c == &sc[i]).unwrap_or(0);
                let frac = size_frac(&sizes, i, lo, range);
                let size = SIZE_MIN + frac * (SIZE_MAX - SIZE_MIN);
                Bar3DBlock::new(xi as f64 * GRID_STEP, yi as f64 * GRID_STEP, 0.0, HEIGHT, size, size, si)
            })
            .collect();
        let names: Vec<String> = (0..keep.len()).map(|i| format!("{} \u{d7} {}", xc[i], yc[i])).collect();
        return (blocks, names);
    }

    let n = match glyph {
        Glyph::Burst | Glyph::RadialRows => cfg.sizes.len(),
        _ => cfg.x_values.len().min(cfg.y_values.len()).min(cfg.sizes.len()),
    };
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let keep = even_indices(n, budget.cloud());
    let x = pick(cfg.x_values, &keep);
    let y = pick(cfg.y_values, &keep);
    let sizes = pick(cfg.sizes, &keep);
    let categories = pick(cfg.categories, &keep);
    let x_categories = pick(cfg.x_categories, &keep);
    let labels = pick(cfg.labels, &keep);
    let names: Vec<String> = if labels.len() == keep.len() { labels } else { (0..keep.len()).map(|i| format!("Point {}", i + 1)).collect() };
    let order = order_of(&categories);
    let class_of = |i: usize| order.iter().position(|c| Some(c) == categories.get(i)).unwrap_or(0);
    let (slo, shi) = sizes.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v.abs()), hi.max(v.abs())));
    let srange = (shi - slo).max(1e-9);

    let blocks: Vec<Bar3DBlock> = match glyph {
        Glyph::Negative => (0..keep.len())
            .map(|i| {
                let frac = size_frac(&sizes, i, slo, srange);
                let size = SIZE_MIN + frac * (SIZE_MAX - SIZE_MIN);
                let (z0, z1) = if sizes[i] >= 0.0 { (0.0, HEIGHT) } else { (-HEIGHT, 0.0) };
                let block = Bar3DBlock::new(x[i], y[i], z0, z1, size, size, class_of(i));
                block.with_tone(if sizes[i] >= 0.0 { UP } else { DOWN })
            })
            .collect(),
        Glyph::Burst => {
            let ng = order.len().max(1);
            let (xlo, xhi) = x_extent(&x, keep.len());
            let xrange = (xhi - xlo).max(1e-9);
            (0..keep.len())
                .map(|i| {
                    let gi = class_of(i);
                    let cluster_az = angle_at(gi as f64, ng as f64, -PI / 2.0);
                    let frac = size_frac(&sizes, i, slo, srange);
                    let pos = x.get(i).copied().unwrap_or(i as f64);
                    let t = ((pos - xlo) / xrange).clamp(0.0, 1.0);
                    let r = BURST_MIN_R + t * (BURST_MAX_R - BURST_MIN_R);
                    let spread = (1.0 - frac).powf(1.6) * BURST_SPREAD;
                    let az = cluster_az + (hash01(i * 2 + 1) * 2.0 - 1.0) * spread;
                    let el = (hash01(i * 2 + 2) * 2.0 - 1.0) * spread;
                    let size = SIZE_MIN + frac * (SIZE_MAX - SIZE_MIN);
                    let cx = r * el.cos() * az.cos();
                    let cy = r * el.cos() * az.sin();
                    let cz = r * el.sin();
                    Bar3DBlock::new(cx, cy, cz, cz + HEIGHT, size, size, gi)
                })
                .collect()
        }
        Glyph::RadialRows => {
            let cols = order_of(&x_categories);
            let has_cols = !cols.is_empty();
            let (xlo, xhi) = x_extent(&x, keep.len());
            let xrange = (xhi - xlo).max(1e-9);
            (0..keep.len())
                .map(|i| {
                    let ri = class_of(i);
                    let radius = ROW_INNER + ri as f64 * ROW_STEP;
                    let elev = ri as f64 * ROW_RISE;
                    let t = if has_cols {
                        let ci = cols.iter().position(|c| Some(c) == x_categories.get(i)).unwrap_or(0);
                        ci as f64 / (cols.len().max(2) - 1) as f64
                    } else {
                        let pos = x.get(i).copied().unwrap_or(i as f64);
                        ((pos - xlo) / xrange).clamp(0.0, 1.0)
                    };
                    let angle = -PI / 2.0 + t * PI;
                    let frac = size_frac(&sizes, i, slo, srange);
                    let size = SIZE_MIN + frac * (SIZE_MAX - SIZE_MIN);
                    Bar3DBlock::new(radius * angle.cos(), radius * angle.sin(), elev, elev + HEIGHT, size, size, ri)
                })
                .collect()
        }
        _ => (0..keep.len())
            .map(|i| {
                let frac = size_frac(&sizes, i, slo, srange);
                let size = SIZE_MIN + frac * (SIZE_MAX - SIZE_MIN);
                Bar3DBlock::new(x[i], y[i], 0.0, HEIGHT, size, size, class_of(i))
            })
            .collect(),
    };
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xy_sizes() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        (vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![2.0, 3.0, 1.0, 4.0, 2.5], vec![5.0, 10.0, 15.0, 20.0, 8.0])
    }

    fn draw(variant: BubbleVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (x, y, s) = xy_sizes();
        let cfg = BubbleConfig { variant, x_values: &x, y_values: &y, sizes: &s, ..BubbleConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_bubble_variant_draws_every_point() {
        for &variant in BubbleVariant::all() {
            if matches!(variant, BubbleVariant::Split) {
                continue;
            }
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 5, "{variant:?}");
            assert_eq!(names.len(), 5, "{variant:?}");
        }
    }

    #[test]
    fn bigger_sizes_produce_wider_markers() {
        let (blocks, _) = draw(BubbleVariant::Basic);
        let smallest = blocks.iter().map(|b| b.hw).fold(f64::INFINITY, f64::min);
        let largest = blocks.iter().map(|b| b.hw).fold(0.0, f64::max);
        assert!(largest > smallest);
    }

    #[test]
    fn negative_sizes_sink_below_the_floor_and_tone_by_sign() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0, 2.0];
        let s = vec![10.0, -10.0];
        let cfg = BubbleConfig { variant: BubbleVariant::Negative, x_values: &x, y_values: &y, sizes: &s, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks[0].z1 > 0.0 && blocks[0].z0 == 0.0);
        assert!(blocks[1].z0 < 0.0 && blocks[1].z1 == 0.0);
    }

    #[test]
    fn split_places_every_cell_on_its_own_category_grid_position() {
        let xc = vec!["A".to_string(), "A".to_string(), "B".to_string()];
        let yc = vec!["X".to_string(), "Y".to_string(), "X".to_string()];
        let cats = vec!["S1".to_string(), "S2".to_string(), "S1".to_string()];
        let sizes = vec![5.0, 8.0, 3.0];
        let cfg = BubbleConfig { variant: BubbleVariant::Split, x_categories: &xc, y_categories: &yc, categories: &cats, sizes: &sizes, ..BubbleConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(blocks.len(), 3);
        assert_eq!(names.len(), 3);
        let positions: std::collections::BTreeSet<(i64, i64)> = blocks.iter().map(|b| ((b.cx * 1000.0) as i64, (b.cy * 1000.0) as i64)).collect();
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn split_is_capped_by_the_budget_for_big_data() {
        let n = 300_000;
        let xc: Vec<String> = (0..n).map(|i| format!("X{}", i % 40)).collect();
        let yc: Vec<String> = (0..n).map(|i| format!("Y{}", i % 40)).collect();
        let cats: Vec<String> = (0..n).map(|i| format!("S{}", i % 5)).collect();
        let sizes: Vec<f64> = (0..n).map(|i| (i % 50) as f64).collect();
        let cfg = BubbleConfig { variant: BubbleVariant::Split, x_categories: &xc, y_categories: &yc, categories: &cats, sizes: &sizes, ..BubbleConfig::default() };
        let budget = Budget::new(Some(600));
        let (blocks, names) = layout_named(&cfg, &budget);
        assert_eq!(blocks.len(), budget.cloud());
        assert_eq!(names.len(), budget.cloud());
    }

    #[test]
    fn burst_places_the_largest_bubble_exactly_on_its_clusters_axis() {
        let s = vec![5.0, 20.0];
        let categories = vec!["A".to_string(), "A".to_string()];
        let cfg = BubbleConfig { variant: BubbleVariant::Burst, sizes: &s, categories: &categories, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let biggest = &blocks[1];
        assert_eq!(biggest.z0, 0.0);
        assert!((biggest.cy.atan2(biggest.cx) - (-PI / 2.0)).abs() < 1e-6);
    }

    #[test]
    fn burst_clusters_stay_angularly_separated_for_many_categories() {
        let s = vec![10.0; 6];
        let categories = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string()];
        let cfg = BubbleConfig { variant: BubbleVariant::Burst, sizes: &s, categories: &categories, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let azimuths: Vec<f64> = blocks.iter().map(|b| b.cy.atan2(b.cx)).collect();
        let distinct: std::collections::BTreeSet<i64> = azimuths.iter().map(|&a| (a * 100.0) as i64).collect();
        assert_eq!(distinct.len(), 6);
    }

    #[test]
    fn burst_uses_the_position_field_for_radius_within_a_cluster() {
        let x = vec![0.0, 100.0];
        let s = vec![20.0, 20.0];
        let categories = vec!["A".to_string(), "A".to_string()];
        let cfg = BubbleConfig { variant: BubbleVariant::Burst, x_values: &x, sizes: &s, categories: &categories, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let r0 = blocks[0].cx.hypot(blocks[0].cy);
        let r1 = blocks[1].cx.hypot(blocks[1].cy);
        assert!((r1 - r0).abs() > 1.0);
    }

    #[test]
    fn radial_rows_gives_each_category_its_own_ring_radius_and_height() {
        let s = vec![5.0, 5.0, 5.0];
        let categories = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let cfg = BubbleConfig { variant: BubbleVariant::RadialRows, sizes: &s, categories: &categories, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let radii: Vec<f64> = blocks.iter().map(|b| b.cx.hypot(b.cy)).collect();
        let heights: Vec<f64> = blocks.iter().map(|b| b.z0).collect();
        assert!(radii[0] < radii[1] && radii[1] < radii[2]);
        assert!(heights[0] < heights[1] && heights[1] < heights[2]);
    }

    #[test]
    fn radial_rows_spreads_points_within_a_row_by_position() {
        let x = vec![0.0, 10.0];
        let s = vec![5.0, 5.0];
        let categories = vec!["A".to_string(), "A".to_string()];
        let cfg = BubbleConfig { variant: BubbleVariant::RadialRows, x_values: &x, sizes: &s, categories: &categories, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!((blocks[0].cy.atan2(blocks[0].cx) - blocks[1].cy.atan2(blocks[1].cx)).abs() > 0.5);
    }

    #[test]
    fn radial_rows_respects_x_categories_as_the_angular_position() {
        let xc = vec!["Verre".to_string(), "Fer".to_string(), "Verre".to_string()];
        let s = vec![5.0, 5.0, 5.0];
        let categories = vec!["A".to_string(), "A".to_string(), "B".to_string()];
        let cfg = BubbleConfig { variant: BubbleVariant::RadialRows, x_categories: &xc, sizes: &s, categories: &categories, ..BubbleConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let a0 = blocks[0].cy.atan2(blocks[0].cx);
        let a1 = blocks[1].cy.atan2(blocks[1].cx);
        assert!((a0 - a1).abs() > 0.1);
    }

    #[test]
    fn burst_and_radial_rows_spread_categories_around_a_centre() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![1.0, 2.0, 3.0, 4.0];
        let s = vec![5.0, 5.0, 5.0, 5.0];
        let categories = vec!["A".to_string(), "A".to_string(), "B".to_string(), "B".to_string()];
        for variant in [BubbleVariant::Burst, BubbleVariant::RadialRows] {
            let cfg = BubbleConfig { variant, x_values: &x, y_values: &y, sizes: &s, categories: &categories, ..BubbleConfig::default() };
            let (blocks, _) = layout_named(&cfg, &Budget::default());
            let angle_a = blocks[0].cy.atan2(blocks[0].cx);
            let angle_b = blocks[2].cy.atan2(blocks[2].cx);
            assert!((angle_a - angle_b).abs() > 1e-6, "{variant:?}");
        }
    }

    #[test]
    fn burst_and_radial_rows_never_needed_x_or_y_in_the_first_place() {
        let s = vec![5.0, 8.0, 3.0, 6.0];
        let categories = vec!["A".to_string(), "A".to_string(), "B".to_string(), "B".to_string()];
        for variant in [BubbleVariant::Burst, BubbleVariant::RadialRows] {
            let cfg = BubbleConfig { variant, sizes: &s, categories: &categories, ..BubbleConfig::default() };
            let (blocks, names) = layout_named(&cfg, &Budget::default());
            assert_eq!(blocks.len(), 4, "{variant:?} must draw every point from sizes alone, x/y were never provided");
            assert_eq!(names.len(), 4, "{variant:?}");
        }
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&BubbleConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_point_cloud_is_capped_by_the_budget() {
        let x: Vec<f64> = (0..300_000).map(|i| (i % 100) as f64).collect();
        let y: Vec<f64> = (0..300_000).map(|i| ((i * 3) % 100) as f64).collect();
        let s: Vec<f64> = (0..300_000).map(|i| ((i * 7) % 50) as f64).collect();
        let cfg = BubbleConfig { x_values: &x, y_values: &y, sizes: &s, ..BubbleConfig::default() };
        let budget = Budget::new(Some(600));
        let (blocks, _) = layout_named(&cfg, &budget);
        assert_eq!(blocks.len(), budget.cloud());
    }
}
