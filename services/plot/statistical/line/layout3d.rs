use super::config::LineConfig;
use super::variant::LineVariant;
use crate::plot::decimate::{combined_magnitude, lttb_indices};
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::curves::{
    area_blocks, band_blocks, catmull_rom, dashed_blocks, joined_by_jump, marker_blocks, regime_plates, regimes,
    ribbon_blocks, slope_tones, step_blocks, toned_ribbon, Point, Riser,
};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::local_maxima_indices;

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "updown";
const ROW_SHARE: f64 = 0.05;
const ROW_PITCH: f64 = 1.2;
const RIBBON_DEPTH: f64 = 0.12;
const AREA_DEPTH: f64 = 0.4;
const MARKER_SHARE: f64 = 0.28;
const MARKER_HEIGHT_SHARE: f64 = 0.035;
const PLATE_SHARE: f64 = 0.02;
const REGIME_TOLERANCE: f64 = 0.03;
const BAND_SPREAD: f64 = 0.08;
const DASH_PATTERNS: [(usize, usize); 4] = [(3, 2), (1, 2), (5, 2), (2, 1)];

#[derive(Clone, Copy)]
enum Glyph {
    Ribbon,
    Steps,
    Spline,
    Area,
    Dashed,
    Band,
}

#[derive(Clone, Copy)]
enum Highlight {
    Off,
    Peaks,
    Regimes,
    Pace,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    rows: bool,
    thin: bool,
    markers: bool,
    highlight: Highlight,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, rows: false, thin: false, markers: false, highlight: Highlight::Off }
    }

    const fn rows(mut self) -> Self {
        self.rows = true;
        self
    }

    const fn thin(mut self) -> Self {
        self.thin = true;
        self
    }

    const fn dotted(mut self) -> Self {
        self.markers = true;
        self
    }

    const fn highlighting(mut self, highlight: Highlight) -> Self {
        self.highlight = highlight;
        self
    }
}

fn recipe(variant: LineVariant) -> Recipe {
    use LineVariant::*;
    let ribbon = Recipe::of(Glyph::Ribbon);
    match variant {
        Basic => ribbon,
        Multi => ribbon.rows(),
        Stepped => Recipe::of(Glyph::Steps).rows(),
        Spline => Recipe::of(Glyph::Spline).rows(),
        Filled => Recipe::of(Glyph::Area).rows(),
        Sparkline => ribbon.rows().thin().dotted(),
        Dashed => Recipe::of(Glyph::Dashed).rows(),
        ConnectedScatter => ribbon.rows().dotted(),
        Gapped => ribbon.rows().dotted(),
        Band => Recipe::of(Glyph::Band),
        Momentum => ribbon.highlighting(Highlight::Peaks),
        Epoch => ribbon.highlighting(Highlight::Regimes),
        Pace => ribbon.highlighting(Highlight::Pace),
    }
}

fn tracks(cfg: &LineConfig, budget: &Budget) -> Vec<Vec<Point>> {
    let sources: Vec<&[f64]> = if !cfg.series.is_empty() {
        cfg.series.iter().map(|(_, v)| v.as_slice()).collect()
    } else if !cfg.values.is_empty() {
        vec![cfg.values]
    } else {
        Vec::new()
    };
    let n = sources.iter().map(|v| v.len()).max().unwrap_or(0);
    let kept = lttb_indices(&combined_magnitude(&sources, n), budget.points);
    sources
        .iter()
        .map(|values| kept.iter().filter_map(|&i| values.get(i).map(|&y| (i as f64, y))).collect())
        .collect()
}

fn span_of(tracks: &[Vec<Point>]) -> (f64, f64, f64) {
    let (lo, hi) = tracks
        .iter()
        .flatten()
        .map(|p| p.1)
        .filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    let width = tracks.iter().flatten().map(|p| p.0).fold(1.0_f64, f64::max);
    if lo.is_finite() { (lo, hi, width) } else { (0.0, 1.0, width) }
}

fn dash_of(cfg: &LineConfig, series: usize) -> (usize, usize) {
    let parsed: Vec<usize> = cfg
        .dash_pattern
        .split(',')
        .filter_map(|t| t.trim().parse::<f64>().ok())
        .map(|v| (v / 2.0).round().max(1.0) as usize)
        .collect();
    match parsed.as_slice() {
        [on, off, ..] => (*on, *off),
        _ => DASH_PATTERNS[series % DASH_PATTERNS.len()],
    }
}

fn smoothing(len: usize) -> usize {
    match len {
        0..=120 => 8,
        121..=400 => 3,
        _ => 1,
    }
}

fn track_blocks(cfg: &LineConfig, plan: Recipe, points: &[Point], series: usize, row: f64, unit: f64, floor: f64) -> Vec<Bar3DBlock> {
    let joined = joined_by_jump(points, if matches!(cfg.variant, LineVariant::Gapped) { cfg.gap_threshold } else { f64::NAN });
    let depth = unit * RIBBON_DEPTH * if plan.thin { 0.5 } else { 1.0 };
    match plan.glyph {
        Glyph::Ribbon | Glyph::Band => ribbon_blocks(points, &joined, row, depth, series, None),
        Glyph::Steps => step_blocks(points, Riser::parse(cfg.step_shape), row, depth, series, None),
        Glyph::Spline => {
            let smooth = catmull_rom(points, smoothing(points.len()), cfg.spline_tension);
            ribbon_blocks(&smooth, &vec![true; smooth.len().saturating_sub(1)], row, depth, series, None)
        }
        Glyph::Area => area_blocks(points, &joined, floor, row, unit * AREA_DEPTH, series, false),
        Glyph::Dashed => dashed_blocks(points, dash_of(cfg, series), 6, row, depth, series, None),
    }
}

fn baseline(lo: f64, hi: f64) -> f64 {
    if lo <= 0.0 && hi >= 0.0 { 0.0 } else { lo }
}

fn banded(tracks: &[Vec<Point>], unit: f64) -> Vec<Bar3DBlock> {
    let (low, high): (Vec<Point>, Vec<Point>) = match tracks {
        [a, b, ..] => (a.clone(), b.clone()),
        [only] => {
            let spread = only.iter().map(|p| p.1.abs()).fold(0.0, f64::max) * BAND_SPREAD;
            (only.iter().map(|p| (p.0, p.1 - spread)).collect(), only.iter().map(|p| (p.0, p.1 + spread)).collect())
        }
        [] => return Vec::new(),
    };
    let middle: Vec<Point> = low.iter().zip(&high).map(|(l, h)| (l.0, (l.1 + h.1) / 2.0)).collect();
    let mut blocks = band_blocks(&low, &high, 0.0, unit * AREA_DEPTH, 0);
    blocks.extend(ribbon_blocks(&middle, &vec![true; middle.len().saturating_sub(1)], 0.0, unit * RIBBON_DEPTH, 1, None));
    blocks
}

fn highlights(cfg: &LineConfig, plan: Recipe, points: &[Point], unit: f64, floor: f64, height: f64) -> Vec<Bar3DBlock> {
    match plan.highlight {
        Highlight::Off => Vec::new(),
        Highlight::Peaks => {
            let values: Vec<f64> = points.iter().map(|p| p.1).collect();
            let picked = local_maxima_indices(&values, 3, (points.len() / 8).max(2));
            let chosen: Vec<Point> = picked.iter().map(|&i| points[i]).collect();
            marker_blocks(&chosen, unit * MARKER_SHARE * 1.6, height * MARKER_HEIGHT_SHARE * 1.6, 0.0, 1, Some(1.0))
        }
        Highlight::Regimes => {
            let tones = regimes(points, REGIME_TOLERANCE);
            regime_plates(points, &tones, floor, height * PLATE_SHARE, 0.0, unit * AREA_DEPTH, 0)
        }
        Highlight::Pace => match (cfg.pace_target, points.first(), points.last()) {
            (Some(target), Some(first), Some(last)) => {
                let pace = [(first.0, first.1), (last.0, target)];
                dashed_blocks(&pace, (3, 2), 60, 0.0, unit * RIBBON_DEPTH * 0.8, 1, Some(0.5))
            }
            _ => Vec::new(),
        },
    }
}

fn toned_track(cfg: &LineConfig, plan: Recipe, points: &[Point], unit: f64) -> Option<Vec<Bar3DBlock>> {
    let depth = unit * RIBBON_DEPTH;
    match plan.highlight {
        Highlight::Peaks => Some(toned_ribbon(points, &slope_tones(points), 0.0, depth, 0)),
        Highlight::Regimes => Some(toned_ribbon(points, &regimes(points, REGIME_TOLERANCE), 0.0, depth, 0)),
        Highlight::Pace => cfg.pace_target.zip(points.first().zip(points.last())).map(|(target, (first, last))| {
            let tones: Vec<f64> = points
                .windows(2)
                .map(|w| {
                    let t = (w[1].0 - first.0) / (last.0 - first.0).max(1e-12);
                    if w[1].1 >= first.1 + (target - first.1) * t { 1.0 } else { 0.0 }
                })
                .collect();
            toned_ribbon(points, &tones, 0.0, depth, 0)
        }),
        _ => None,
    }
}

pub fn layout_3d(cfg: &LineConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    let plan = recipe(cfg.variant);
    let all = tracks(cfg, budget);
    if all.is_empty() || all.iter().all(|t| t.is_empty()) {
        return Vec::new();
    }
    let (lo, hi, width) = span_of(&all);
    let height = (hi - lo).max(1e-9);
    let floor = baseline(lo, hi);
    let unit = (width * ROW_SHARE).max(1.0);
    if matches!(plan.glyph, Glyph::Band) {
        return banded(&all, unit);
    }
    let mut blocks = Vec::new();
    for (s, points) in all.iter().enumerate() {
        let row = if plan.rows { s as f64 * unit * ROW_PITCH } else { 0.0 };
        let toned = if s == 0 { toned_track(cfg, plan, points, unit) } else { None };
        match toned {
            Some(t) => blocks.extend(t),
            None => blocks.extend(track_blocks(cfg, plan, points, s, row, unit, floor)),
        }
        if (plan.markers && cfg.show_points) || matches!(cfg.variant, LineVariant::ConnectedScatter) {
            blocks.extend(marker_blocks(points, unit * MARKER_SHARE, height * MARKER_HEIGHT_SHARE, row, s, None));
        }
    }
    if let Some(first) = all.first() {
        blocks.extend(highlights(cfg, plan, first, unit, floor, height));
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: [f64; 8] = [12.0, 19.0, 15.0, 22.0, 86.0, 79.0, 18.0, 24.0];

    fn series() -> Vec<(String, Vec<f64>)> {
        vec![
            ("A".to_string(), vec![12.0, 19.0, 15.0, 22.0, 28.0, 24.0]),
            ("B".to_string(), vec![8.0, 14.0, 18.0, 16.0, 22.0, 20.0]),
        ]
    }

    fn blocks_for(variant: LineVariant) -> Vec<Bar3DBlock> {
        let data = series();
        layout_3d(
            &LineConfig {
                variant,
                values: &VALUES,
                series: if matches!(variant, LineVariant::Band | LineVariant::Multi | LineVariant::ConnectedScatter) { &data } else { &[] },
                pace_target: Some(120.0),
                gap_threshold: 30.0,
                show_points: true,
                ..LineConfig::default()
            },
            &Budget::default(),
        )
    }

    #[test]
    fn every_line_variant_draws_something() {
        for variant in LineVariant::all() {
            assert!(!blocks_for(*variant).is_empty(), "{} must draw blocks", variant.name());
        }
    }

    #[test]
    fn multi_series_sit_on_their_own_depth_rows() {
        let rows: std::collections::BTreeSet<i64> = blocks_for(LineVariant::Multi).iter().map(|b| (b.cy * 100.0) as i64).collect();
        assert_eq!(rows.len(), 2);
        let single: std::collections::BTreeSet<i64> = blocks_for(LineVariant::Basic).iter().map(|b| (b.cy * 100.0) as i64).collect();
        assert_eq!(single.len(), 1);
    }

    #[test]
    fn ribbons_slope_and_steps_stay_flat() {
        assert!(blocks_for(LineVariant::Basic).iter().all(|b| b.end.is_some()));
        assert!(blocks_for(LineVariant::Stepped).iter().all(|b| b.end.is_none()));
    }

    #[test]
    fn splines_densify_and_filled_adds_areas_under_the_ribbon() {
        assert!(blocks_for(LineVariant::Spline).len() > blocks_for(LineVariant::Basic).len() * 4);
        assert!(blocks_for(LineVariant::Filled).iter().all(|b| b.z0 == 12.0 && b.end.map(|e| e.0 == 12.0).unwrap_or(false)));
    }

    #[test]
    fn gapped_breaks_the_line_at_the_jump_and_dashed_leaves_holes() {
        assert!(blocks_for(LineVariant::Gapped).len() < blocks_for(LineVariant::ConnectedScatter).len() + 20);
        let gapped: usize = blocks_for(LineVariant::Gapped).iter().filter(|b| b.end.is_some()).count();
        assert!(gapped < VALUES.len() - 1);
    }

    #[test]
    fn momentum_epoch_and_pace_carry_tones() {
        for variant in [LineVariant::Momentum, LineVariant::Epoch, LineVariant::Pace] {
            assert!(blocks_for(variant).iter().any(|b| b.tone.is_some()), "{} must carry tones", variant.name());
        }
    }

    #[test]
    fn long_series_are_decimated_to_the_budget_keeping_the_spike() {
        let n = 200_000;
        let mut values: Vec<f64> = (0..n).map(|i| (i as f64 * 0.001).sin() * 10.0).collect();
        values[100_000] = 500.0;
        let cfg = LineConfig { values: &values, ..LineConfig::default() };
        let blocks = layout_3d(&cfg, &Budget::new(Some(600)));
        assert!(blocks.len() <= 600);
        assert!(blocks.iter().any(|b| b.z1 > 400.0 || b.end.map(|e| e.1 > 400.0).unwrap_or(false)));
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_3d(&LineConfig::default(), &Budget::default()).is_empty());
    }
}
