use super::config::AreaConfig;
use super::variant::AreaVariant;
use crate::plot::statistical::_3d::budget::{Budget, Buckets};
use crate::plot::statistical::_3d::curves::{area_blocks, band_blocks, catmull_rom, joined_by_jump, points_of, thickness_of, Point};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "jet";
const ROW_PITCH: f64 = 1.2;
const AREA_DEPTH: f64 = 0.4;
const THIN_DEPTH: f64 = 0.14;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Filled,
    Stacked,
    Percent,
    Signed,
    Leader,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    spline: bool,
    step: bool,
    graded: bool,
    thin: bool,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, spline: false, step: false, graded: false, thin: false }
    }

    const fn splined(mut self) -> Self {
        self.spline = true;
        self
    }

    const fn stepped(mut self) -> Self {
        self.step = true;
        self
    }

    const fn graded(mut self) -> Self {
        self.graded = true;
        self
    }

    const fn thinned(mut self) -> Self {
        self.thin = true;
        self
    }
}

fn recipe(variant: AreaVariant) -> Recipe {
    use AreaVariant::*;
    match variant {
        Basic => Recipe::of(Glyph::Filled),
        Stacked => Recipe::of(Glyph::Stacked),
        Percent => Recipe::of(Glyph::Percent),
        Spline => Recipe::of(Glyph::Filled).splined(),
        Step => Recipe::of(Glyph::Filled).stepped(),
        Gradient => Recipe::of(Glyph::Filled).splined().graded(),
        Ribbon => Recipe::of(Glyph::Percent).thinned(),
        Wave => Recipe::of(Glyph::Signed).splined(),
        Leader => Recipe::of(Glyph::Leader),
    }
}

fn smoothing(len: usize) -> usize {
    match len {
        0..=120 => 8,
        121..=400 => 3,
        _ => 1,
    }
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

fn stacked_bands(series: &[(String, Vec<f64>)], n: usize, percent: bool, signed: bool) -> Vec<(Vec<Point>, Vec<Point>)> {
    let mut totals = vec![0.0_f64; n];
    if percent {
        for (_, vals) in series {
            for i in 0..n {
                totals[i] += vals.get(i).copied().unwrap_or(0.0).max(0.0);
            }
        }
    }
    let scale = |v: f64, i: usize| if percent && totals[i] > 0.0 { v / totals[i] * 100.0 } else if percent { 0.0 } else { v };
    let mut running = vec![0.0_f64; n];
    series
        .iter()
        .map(|(_, vals)| {
            let low: Vec<Point> = (0..n).map(|i| (i as f64, scale(running[i], i))).collect();
            for i in 0..n {
                let raw = vals.get(i).copied().unwrap_or(0.0);
                running[i] += if signed { raw } else { raw.max(0.0) };
            }
            let high: Vec<Point> = (0..n).map(|i| (i as f64, scale(running[i], i))).collect();
            (low, high)
        })
        .collect()
}

fn leader_ribbon(tracks: &[Vec<Point>]) -> Vec<Bar3DBlock> {
    let n = tracks.iter().map(|t| t.len()).max().unwrap_or(0);
    if n < 2 || tracks.is_empty() {
        return Vec::new();
    }
    let all: Vec<Point> = tracks.iter().flatten().copied().collect();
    let half = thickness_of(&all);
    let leader_at = |i: usize| -> usize {
        let mut best = 0usize;
        let mut best_v = f64::NEG_INFINITY;
        for (s, t) in tracks.iter().enumerate() {
            if let Some(&(_, y)) = t.get(i) {
                if y.is_finite() && y > best_v {
                    best_v = y;
                    best = s;
                }
            }
        }
        best
    };
    (0..n - 1)
        .filter_map(|i| {
            let s = leader_at(i);
            let &(x0, y0) = tracks[s].get(i)?;
            let &(x1, y1) = tracks[s].get(i + 1)?;
            if !y0.is_finite() || !y1.is_finite() {
                return None;
            }
            Some(Bar3DBlock::sloped((x0 + x1) / 2.0, 0.0, (y0 - half, y0 + half), (y1 - half, y1 + half), (x1 - x0) / 2.0, THIN_DEPTH, s))
        })
        .collect()
}

pub fn layout_3d(cfg: &AreaConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &AreaConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.series.is_empty() || cfg.x_labels.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let raw_n = cfg.x_labels.len();
    let buckets = Buckets::new(raw_n, budget.elements());
    let series: Vec<(String, Vec<f64>)> = cfg.series.iter().map(|(name, v)| (name.clone(), buckets.mean(v))).collect();
    let n = buckets.len();
    let names: Vec<String> = series.iter().map(|(name, _)| name.clone()).collect();
    let blocks = match plan.glyph {
        Glyph::Stacked | Glyph::Percent | Glyph::Signed => {
            let bands = stacked_bands(&series, n, plan.glyph == Glyph::Percent, plan.glyph == Glyph::Signed);
            let depth = if plan.thin { THIN_DEPTH } else { AREA_DEPTH };
            bands.iter().enumerate().flat_map(|(s, (low, high))| band_blocks(low, high, 0.0, depth, s)).collect()
        }
        Glyph::Filled => series
            .iter()
            .enumerate()
            .flat_map(|(s, (_, values))| {
                let mut points = points_of(values);
                if plan.step {
                    points = step_profile(&points);
                }
                if plan.spline {
                    points = catmull_rom(&points, smoothing(points.len()), 0.5);
                }
                let joined = joined_by_jump(&points, f64::NAN);
                area_blocks(&points, &joined, 0.0, s as f64 * ROW_PITCH, AREA_DEPTH, s, plan.graded)
            })
            .collect(),
        Glyph::Leader => {
            let tracks: Vec<Vec<Point>> = series.iter().map(|(_, v)| points_of(v)).collect();
            let mut out: Vec<Bar3DBlock> = tracks
                .iter()
                .enumerate()
                .flat_map(|(s, points)| {
                    let joined = joined_by_jump(points, f64::NAN);
                    area_blocks(points, &joined, 0.0, s as f64 * ROW_PITCH, THIN_DEPTH, s, false)
                })
                .collect();
            out.extend(leader_ribbon(&tracks));
            out
        }
    };
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series() -> Vec<(String, Vec<f64>)> {
        vec![
            ("North".to_string(), vec![10.0, 12.0, 14.0, 16.0, 18.0]),
            ("South".to_string(), vec![8.0, 9.0, 11.0, 10.0, 12.0]),
            ("East".to_string(), vec![6.0, 14.0, 8.0, 15.0, 9.0]),
        ]
    }

    fn labels() -> Vec<String> {
        (0..5).map(|i| i.to_string()).collect()
    }

    fn draw(variant: AreaVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let xl = labels();
        let s = series();
        let cfg = AreaConfig { variant, x_labels: &xl, series: &s, ..AreaConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_area_variant_draws_something_and_names_every_series() {
        for &variant in AreaVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["North".to_string(), "South".to_string(), "East".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn basic_puts_every_series_on_its_own_depth_row() {
        let (blocks, _) = draw(AreaVariant::Basic);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0).round() as i64).collect();
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn stacked_bands_sit_on_top_of_each_other_not_from_the_floor() {
        let (blocks, names) = draw(AreaVariant::Stacked);
        let south = names.iter().position(|n| n == "South").unwrap();
        let south_block = blocks.iter().find(|b| b.ci == south).unwrap();
        assert!(south_block.end.map(|(z0, _)| z0 > 0.0).unwrap_or(south_block.z0 > 0.0));
    }

    #[test]
    fn percent_normalises_the_top_band_to_one_hundred() {
        let (blocks, names) = draw(AreaVariant::Percent);
        let east = names.iter().position(|n| n == "East").unwrap();
        for b in blocks.iter().filter(|b| b.ci == east) {
            let top = b.end.map(|(_, z1)| z1).unwrap_or(b.z1);
            assert!(top > 99.0 && top < 101.0, "top band must read ~100%: {top}");
        }
    }

    #[test]
    fn wave_lets_the_signed_stack_go_below_zero() {
        let xl: Vec<String> = (0..4).map(|i| i.to_string()).collect();
        let s = vec![("A".to_string(), vec![-3.0, -2.0, -4.0, -1.0]), ("B".to_string(), vec![1.0, 2.0, 1.0, 3.0])];
        let cfg = AreaConfig { variant: AreaVariant::Wave, x_labels: &xl, series: &s, ..AreaConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().any(|b| b.z0 < 0.0 || b.end.map(|(z0, _)| z0 < 0.0).unwrap_or(false)));
    }

    #[test]
    fn leader_ribbon_switches_class_when_the_top_series_changes() {
        let xl: Vec<String> = (0..4).map(|i| i.to_string()).collect();
        let s = vec![("A".to_string(), vec![10.0, 8.0, 2.0, 1.0]), ("B".to_string(), vec![1.0, 2.0, 9.0, 12.0])];
        let cfg = AreaConfig { variant: AreaVariant::Leader, x_labels: &xl, series: &s, ..AreaConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let classes: std::collections::BTreeSet<usize> = blocks.iter().filter(|b| b.hd == THIN_DEPTH && b.tone.is_none()).map(|b| b.ci).collect();
        assert!(classes.contains(&0) && classes.contains(&1));
    }

    #[test]
    fn spline_and_gradient_produce_more_and_toned_blocks_than_basic() {
        let (basic, _) = draw(AreaVariant::Basic);
        let (spline, _) = draw(AreaVariant::Spline);
        assert!(spline.len() > basic.len());
        let (gradient, _) = draw(AreaVariant::Gradient);
        assert!(gradient.iter().all(|b| b.tone.is_some()));
    }

    #[test]
    fn a_huge_series_stays_within_the_element_budget() {
        let xl: Vec<String> = (0..200_000).map(|i| i.to_string()).collect();
        let values: Vec<f64> = (0..200_000).map(|i| (i as f64 * 0.001).sin() * 10.0).collect();
        let s = vec![("only".to_string(), values)];
        let cfg = AreaConfig { x_labels: &xl, series: &s, ..AreaConfig::default() };
        let budget = Budget::new(Some(600));
        let (blocks, _) = layout_named(&cfg, &budget);
        assert!(blocks.len() <= budget.elements());
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&AreaConfig::default(), &Budget::default()).0.is_empty());
    }
}
