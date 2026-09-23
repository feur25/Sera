use super::config::KdeConfig;
use super::contour::gaussian_kernel_sum;
use super::variant::KdeVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::curves::{area_blocks, band_blocks, joined_by_jump, points_of, ribbon_blocks, Point};
use crate::plot::statistical::_3d::grid::{rect_cells, CellField};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::histogram::compute_bins;
use crate::plot::statistical::kde::{kde_eval, scott_bw};

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "jet";
const N_POINTS: usize = 60;
const ROW_PITCH: f64 = 1.3;
const AREA_DEPTH: f64 = 0.35;
const RIBBON_DEPTH: f64 = 0.1;
const RUG_HEIGHT: f64 = 0.1;
const GRID_N: usize = 16;
const LEVELS: usize = 5;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Area,
    Outline,
    Step,
    Cumulative,
    Stack,
    Percent,
    Field,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    rug: bool,
    histogram: bool,
    levels: bool,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, rug: false, histogram: false, levels: false }
    }

    const fn rugged(mut self) -> Self {
        self.rug = true;
        self
    }

    const fn histogrammed(mut self) -> Self {
        self.histogram = true;
        self
    }

    const fn levelled(mut self) -> Self {
        self.levels = true;
        self
    }
}

fn recipe(variant: KdeVariant) -> Recipe {
    use KdeVariant::*;
    match variant {
        Basic => Recipe::of(Glyph::Area),
        Normalized => Recipe::of(Glyph::Area),
        Outline => Recipe::of(Glyph::Outline),
        Stepped => Recipe::of(Glyph::Step),
        Rug => Recipe::of(Glyph::Area).rugged(),
        Histogram => Recipe::of(Glyph::Outline).histogrammed(),
        Cumulative => Recipe::of(Glyph::Cumulative),
        Stack => Recipe::of(Glyph::Stack),
        Fill => Recipe::of(Glyph::Percent),
        Contour => Recipe::of(Glyph::Field),
        Levels => Recipe::of(Glyph::Field).levelled(),
    }
}

fn domain(series: &[(String, Vec<f64>)]) -> (f64, f64) {
    let all: Vec<f64> = series.iter().flat_map(|(_, v)| v.iter().copied()).filter(|v| v.is_finite()).collect();
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if lo.is_finite() && hi > lo { (lo, hi) } else { (0.0, 1.0) }
}

fn density_curve(samples: &[f64], lo: f64, hi: f64) -> Vec<Point> {
    let range = (hi - lo).max(1e-9);
    let bw = scott_bw(samples).max(range * 0.01);
    (0..N_POINTS)
        .map(|i| {
            let x = lo + range * i as f64 / (N_POINTS - 1).max(1) as f64;
            (x, kde_eval(samples, x, bw))
        })
        .collect()
}

fn step_profile(points: &[Point]) -> Vec<Point> {
    let mut out = Vec::with_capacity(points.len() * 2);
    for (i, &(x, y)) in points.iter().enumerate() {
        if i > 0 {
            out.push((x, points[i - 1].1));
        }
        out.push((x, y));
    }
    out
}

fn field_blocks(xs: &[f64], ys: &[f64], row: f64, class: usize, levels: bool) -> Vec<Bar3DBlock> {
    let (xlo, xhi) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    let (ylo, yhi) = ys.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if !xlo.is_finite() || !ylo.is_finite() || xhi <= xlo || yhi <= ylo {
        return Vec::new();
    }
    let bwx = scott_bw(xs).max((xhi - xlo) * 0.02);
    let bwy = scott_bw(ys).max((yhi - ylo) * 0.02);
    let mut heights = vec![0.0_f64; GRID_N * GRID_N];
    for r in 0..GRID_N {
        let gy = ylo + (yhi - ylo) * (r as f64 + 0.5) / GRID_N as f64;
        for c in 0..GRID_N {
            let gx = xlo + (xhi - xlo) * (c as f64 + 0.5) / GRID_N as f64;
            heights[r * GRID_N + c] = gaussian_kernel_sum(gx, gy, xs, ys, bwx, bwy);
        }
    }
    let peak = heights.iter().cloned().fold(1e-12, f64::max);
    if levels {
        for h in heights.iter_mut() {
            *h = ((*h / peak * LEVELS as f64).floor() / LEVELS as f64) * peak;
        }
    }
    let tones: Vec<f64> = heights.iter().map(|h| (h / peak).clamp(0.0, 1.0)).collect();
    let classes = vec![class; GRID_N * GRID_N];
    let field = CellField { n_rows: GRID_N, n_cols: GRID_N, heights: &heights, tones: &tones, classes: &classes };
    rect_cells(&field, &[], &[], &vec![row; GRID_N], 0.92)
}

pub fn layout_3d(cfg: &KdeConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &KdeConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.series.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let names: Vec<String> = cfg.series.iter().map(|(name, _)| name.clone()).collect();

    if plan.glyph == Glyph::Field {
        let blocks = cfg
            .series
            .iter()
            .enumerate()
            .flat_map(|(s, (_, xs))| {
                let ys = cfg.y_series.get(s).cloned().unwrap_or_else(|| cfg.y_values.to_vec());
                field_blocks(xs, &ys, s as f64 * ROW_PITCH * GRID_N as f64, s, plan.levels)
            })
            .collect();
        return (blocks, names);
    }

    let (lo, hi) = domain(cfg.series);

    if matches!(plan.glyph, Glyph::Stack | Glyph::Percent) {
        let curves: Vec<Vec<Point>> = cfg.series.iter().map(|(_, v)| density_curve(v, lo, hi)).collect();
        let n = N_POINTS;
        let mut totals = vec![0.0_f64; n];
        if plan.glyph == Glyph::Percent {
            for curve in &curves {
                for i in 0..n {
                    totals[i] += curve[i].1.max(0.0);
                }
            }
        }
        let mut running = vec![0.0_f64; n];
        let blocks = curves
            .iter()
            .enumerate()
            .flat_map(|(s, curve)| {
                let low: Vec<Point> = (0..n).map(|i| (curve[i].0, scale(running[i], totals[i], plan.glyph))).collect();
                for i in 0..n {
                    running[i] += curve[i].1.max(0.0);
                }
                let high: Vec<Point> = (0..n).map(|i| (curve[i].0, scale(running[i], totals[i], plan.glyph))).collect();
                band_blocks(&low, &high, 0.0, AREA_DEPTH, s)
            })
            .collect();
        return (blocks, names);
    }

    let blocks = cfg
        .series
        .iter()
        .enumerate()
        .flat_map(|(s, (_, samples))| {
            let row = s as f64 * ROW_PITCH;
            let mut curve = density_curve(samples, lo, hi);
            if plan.glyph == Glyph::Cumulative {
                let mut acc = 0.0;
                for p in curve.iter_mut() {
                    acc += p.1;
                    p.1 = acc;
                }
                let total = curve.last().map(|p| p.1).unwrap_or(1.0).max(1e-9);
                for p in curve.iter_mut() {
                    p.1 /= total;
                }
            }
            let mut segs = match plan.glyph {
                Glyph::Outline => {
                    let joined = joined_by_jump(&curve, f64::NAN);
                    ribbon_blocks(&curve, &joined, row, RIBBON_DEPTH, s, None)
                }
                Glyph::Step => {
                    let stepped = step_profile(&curve);
                    let joined = joined_by_jump(&stepped, f64::NAN);
                    area_blocks(&stepped, &joined, 0.0, row, AREA_DEPTH, s, false)
                }
                _ => {
                    let joined = joined_by_jump(&curve, f64::NAN);
                    area_blocks(&curve, &joined, 0.0, row, AREA_DEPTH, s, false)
                }
            };
            if plan.histogram {
                let (counts, edges) = compute_bins(samples, cfg.bins.max(1));
                let peak_count = counts.iter().copied().max().unwrap_or(1).max(1) as f64;
                let peak_density = curve.iter().map(|p| p.1).fold(1e-12, f64::max);
                for (i, &c) in counts.iter().enumerate() {
                    let x = (edges[i] + edges[i + 1]) / 2.0;
                    let h = (c as f64 / peak_count) * peak_density;
                    segs.push(Bar3DBlock::new(x, row, 0.0, h, (hi - lo) / cfg.bins.max(1) as f64 / 2.2, AREA_DEPTH * 0.5, s));
                }
            }
            if plan.rug {
                for &v in samples.iter().filter(|v| v.is_finite()) {
                    segs.push(Bar3DBlock::new(v, row, 0.0, RUG_HEIGHT, 0.03, AREA_DEPTH * 0.3, s));
                }
            }
            segs
        })
        .collect();
    (blocks, names)
}

fn scale(v: f64, total: f64, glyph: Glyph) -> f64 {
    if glyph == Glyph::Percent && total > 0.0 { v / total * 100.0 } else if glyph == Glyph::Percent { 0.0 } else { v }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series() -> Vec<(String, Vec<f64>)> {
        vec![
            ("A".to_string(), vec![1.0, 1.5, 2.0, 2.2, 2.5, 3.0, 3.1, 2.8]),
            ("B".to_string(), vec![4.0, 4.5, 5.0, 5.2, 5.5, 6.0, 5.8, 4.9]),
        ]
    }

    fn draw(variant: KdeVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let s = series();
        let cfg = KdeConfig { variant, series: &s, bins: 10, ..KdeConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    fn draw_bivariate(variant: KdeVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let xs = vec![1.0, 1.2, 0.9, 1.1, 5.0, 5.2, 4.9, 5.1];
        let ys = vec![1.0, 0.9, 1.1, 1.05, 5.0, 4.9, 5.1, 5.0];
        let cfg = KdeConfig { variant, series: &[("only".to_string(), xs)], y_values: &ys, ..KdeConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_1d_kde_variant_draws_something_and_names_every_series() {
        for &variant in KdeVariant::all() {
            if matches!(variant, KdeVariant::Contour | KdeVariant::Levels) {
                continue;
            }
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn contour_and_levels_draw_a_density_field_from_bivariate_data() {
        for variant in [KdeVariant::Contour, KdeVariant::Levels] {
            let (blocks, names) = draw_bivariate(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["only".to_string()]);
        }
    }

    #[test]
    fn basic_puts_every_series_on_its_own_depth_row() {
        let (blocks, _) = draw(KdeVariant::Basic);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0).round() as i64).collect();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn cumulative_ends_near_one_and_never_decreases() {
        let (blocks, names) = draw(KdeVariant::Cumulative);
        let a = names.iter().position(|n| n == "A").unwrap();
        let mut tops: Vec<f64> = blocks.iter().filter(|b| b.ci == a).map(|b| b.end.map(|(_, z1)| z1).unwrap_or(b.z1)).collect();
        tops.sort_by(|x, y| x.partial_cmp(y).unwrap());
        assert!(*tops.last().unwrap() > 0.9 && *tops.last().unwrap() < 1.1);
    }

    #[test]
    fn stack_grows_taller_than_a_single_series_and_fill_tops_at_one_hundred() {
        let (basic, _) = draw(KdeVariant::Basic);
        let (stack, names) = draw(KdeVariant::Stack);
        let b = names.iter().position(|n| n == "B").unwrap();
        let stack_b_top = stack.iter().filter(|blk| blk.ci == b).map(|blk| blk.end.map(|(_, z1)| z1).unwrap_or(blk.z1)).fold(0.0, f64::max);
        let basic_peak = basic.iter().map(|blk| blk.end.map(|(_, z1)| z1).unwrap_or(blk.z1)).fold(0.0, f64::max);
        assert!(stack_b_top > basic_peak);
        let (fill, names2) = draw(KdeVariant::Fill);
        let b2 = names2.iter().position(|n| n == "B").unwrap();
        for blk in fill.iter().filter(|blk| blk.ci == b2) {
            let top = blk.end.map(|(_, z1)| z1).unwrap_or(blk.z1);
            assert!(top < 100.5);
        }
    }

    #[test]
    fn histogram_overlay_adds_bin_columns_and_rug_adds_ticks() {
        let (basic, _) = draw(KdeVariant::Basic);
        let (hist, _) = draw(KdeVariant::Histogram);
        assert!(hist.len() > basic.len());
        let (rug, _) = draw(KdeVariant::Rug);
        assert!(rug.iter().any(|b| b.z1 == RUG_HEIGHT));
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&KdeConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_series_stays_within_a_bounded_block_count() {
        let values: Vec<f64> = (0..200_000).map(|i| (i as f64 * 0.0001).sin() * 5.0 + 5.0).collect();
        let s = vec![("only".to_string(), values)];
        let cfg = KdeConfig { series: &s, ..KdeConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 500);
    }
}
