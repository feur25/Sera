use super::config::SlopeConfig;
use super::variant::SlopeVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::curves::{catmull_rom, ribbon_blocks, Point};
use crate::plot::statistical::_3d::generic::diverging_columns;
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "updown";
const DEPTH: f64 = 0.08;
const MONO_TONE: f64 = 0.5;
const FADE_TONE: f64 = 0.08;
const HIGHLIGHT_TOP: usize = 3;
const THICK_MIN: f64 = 0.03;
const THICK_MAX: f64 = 0.24;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Line,
    Diverging,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    ranked: bool,
    curved: bool,
    stepped: bool,
    thick: bool,
    mono: bool,
    highlight: bool,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, ranked: false, curved: false, stepped: false, thick: false, mono: false, highlight: false }
    }

    const fn ranked(mut self) -> Self {
        self.ranked = true;
        self
    }

    const fn curved(mut self) -> Self {
        self.curved = true;
        self
    }

    const fn stepped(mut self) -> Self {
        self.stepped = true;
        self
    }

    const fn thick(mut self) -> Self {
        self.thick = true;
        self
    }

    const fn mono(mut self) -> Self {
        self.mono = true;
        self
    }

    const fn highlighted(mut self) -> Self {
        self.highlight = true;
        self
    }
}

fn recipe(variant: SlopeVariant) -> Recipe {
    use SlopeVariant::*;
    match variant {
        Basic => Recipe::of(Glyph::Line),
        Monochrome => Recipe::of(Glyph::Line).mono(),
        Highlighted => Recipe::of(Glyph::Line).highlighted(),
        Bumps => Recipe::of(Glyph::Line).ranked().curved(),
        Curved => Recipe::of(Glyph::Line).curved(),
        Thick => Recipe::of(Glyph::Line).thick(),
        Diverging => Recipe::of(Glyph::Diverging),
        Stepped => Recipe::of(Glyph::Line).stepped(),
    }
}

fn ranks_desc(vals: &[f64]) -> Vec<f64> {
    let n = vals.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| vals[b].partial_cmp(&vals[a]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank = vec![0.0_f64; n];
    for (r, &i) in idx.iter().enumerate() {
        rank[i] = (n - r) as f64;
    }
    rank
}

fn track_for(left: f64, right: f64, plan: Recipe) -> Vec<Point> {
    if plan.stepped {
        return vec![(0.0, left), (0.5, left), (0.5, right), (1.0, right)];
    }
    if plan.curved {
        let mid = (0.0 + 1.0) / 2.0;
        return catmull_rom(&[(0.0, left), (mid, (left + right) / 2.0), (1.0, right)], 8, 0.5);
    }
    vec![(0.0, left), (1.0, right)]
}

pub fn layout_3d(cfg: &SlopeConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &SlopeConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values_left.len()).min(cfg.values_right.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let names: Vec<String> = cfg.labels[..n].to_vec();
    let clean = |v: f64| if v.is_finite() { v } else { 0.0 };
    let left: Vec<f64> = cfg.values_left[..n].iter().copied().map(clean).collect();
    let right: Vec<f64> = cfg.values_right[..n].iter().copied().map(clean).collect();

    if plan.glyph == Glyph::Diverging {
        let deltas: Vec<f64> = (0..n).map(|i| right[i] - left[i]).collect();
        return (diverging_columns(&deltas, 0.3, 0.3), names);
    }

    let (ly, ry) = if plan.ranked { (ranks_desc(&left), ranks_desc(&right)) } else { (left.clone(), right.clone()) };
    let deltas: Vec<f64> = (0..n).map(|i| right[i] - left[i]).collect();
    let max_delta = deltas.iter().map(|d| d.abs()).fold(0.0_f64, f64::max).max(1e-9);

    let mut top: Vec<bool> = vec![false; n];
    if plan.highlight {
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| deltas[b].abs().partial_cmp(&deltas[a].abs()).unwrap_or(std::cmp::Ordering::Equal));
        for &i in order.iter().take(HIGHLIGHT_TOP.min(n)) {
            top[i] = true;
        }
    }

    let blocks = (0..n)
        .flat_map(|i| {
            let points = track_for(ly[i], ry[i], plan);
            let depth = if plan.thick { THICK_MIN + (deltas[i].abs() / max_delta) * (THICK_MAX - THICK_MIN) } else { DEPTH };
            let joined = vec![true; points.len().saturating_sub(1)];
            let mut segs = ribbon_blocks(&points, &joined, 0.0, depth, i, None);
            if plan.mono {
                for b in segs.iter_mut() {
                    b.tone = Some(MONO_TONE);
                }
            } else if plan.highlight && !top[i] {
                for b in segs.iter_mut() {
                    b.tone = Some(FADE_TONE);
                }
            } else if !plan.mono {
                let tone = if deltas[i] >= 0.0 { crate::plot::statistical::_3d::ohlc::UP } else { crate::plot::statistical::_3d::ohlc::DOWN };
                for b in segs.iter_mut() {
                    b.tone = Some(tone);
                }
            }
            segs
        })
        .collect();
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(variant: SlopeVariant, labels: &'static [&'static str], left: &'static [f64], right: &'static [f64]) -> (Vec<String>, Vec<f64>, Vec<f64>) {
        (labels.iter().map(|s| s.to_string()).collect(), left.to_vec(), right.to_vec())
    }

    fn draw(variant: SlopeVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, left, right) = cfg(variant, &["A", "B", "C", "D", "E"], &[20.0, 35.0, 15.0, 42.0, 28.0], &[35.0, 28.0, 40.0, 55.0, 22.0]);
        let sc = SlopeConfig { variant, labels: &labels, values_left: &left, values_right: &right, ..SlopeConfig::default() };
        layout_named(&sc, &Budget::default())
    }

    #[test]
    fn every_slope_variant_draws_something_and_names_every_label() {
        for &variant in SlopeVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn basic_tones_rising_and_falling_lines_differently() {
        let (blocks, names) = draw(SlopeVariant::Basic);
        let a = names.iter().position(|n| n == "A").unwrap();
        let b = names.iter().position(|n| n == "B").unwrap();
        let ta = blocks.iter().find(|blk| blk.ci == a).unwrap().tone;
        let tb = blocks.iter().find(|blk| blk.ci == b).unwrap().tone;
        assert_ne!(ta, tb);
    }

    #[test]
    fn monochrome_gives_every_line_the_same_flat_tone() {
        let (blocks, _) = draw(SlopeVariant::Monochrome);
        assert!(blocks.iter().all(|b| b.tone == Some(MONO_TONE)));
    }

    #[test]
    fn highlighted_fades_everything_but_the_three_biggest_movers() {
        let (blocks, _) = draw(SlopeVariant::Highlighted);
        let faded = blocks.iter().filter(|b| b.tone == Some(FADE_TONE)).count();
        let vivid = blocks.iter().filter(|b| b.tone != Some(FADE_TONE)).count();
        assert!(faded > 0 && vivid > 0);
    }

    #[test]
    fn bumps_positions_by_rank_not_raw_value() {
        let (basic, names) = draw(SlopeVariant::Basic);
        let (bumps, _) = draw(SlopeVariant::Bumps);
        let d = names.iter().position(|n| n == "D").unwrap();
        let basic_range = basic.iter().find(|blk| blk.ci == d).unwrap().z_range();
        let bumps_range = bumps.iter().find(|blk| blk.ci == d).unwrap().z_range();
        assert_ne!(basic_range, bumps_range);
    }

    #[test]
    fn curved_and_stepped_produce_more_segments_than_the_plain_diagonal() {
        let (basic, _) = draw(SlopeVariant::Basic);
        let (curved, _) = draw(SlopeVariant::Curved);
        let (stepped, _) = draw(SlopeVariant::Stepped);
        assert!(curved.len() > basic.len());
        assert!(stepped.len() > basic.len());
    }

    #[test]
    fn thick_scales_depth_with_the_size_of_the_change() {
        let (labels, left, right) = cfg(SlopeVariant::Thick, &["Small", "Big"], &[10.0, 10.0], &[11.0, 90.0]);
        let sc = SlopeConfig { variant: SlopeVariant::Thick, labels: &labels, values_left: &left, values_right: &right, ..SlopeConfig::default() };
        let (blocks, names) = layout_named(&sc, &Budget::default());
        let small = names.iter().position(|n| n == "Small").unwrap();
        let big = names.iter().position(|n| n == "Big").unwrap();
        let small_depth = blocks.iter().find(|blk| blk.ci == small).unwrap().hd;
        let big_depth = blocks.iter().find(|blk| blk.ci == big).unwrap().hd;
        assert!(big_depth > small_depth);
    }

    #[test]
    fn diverging_draws_centered_bars_that_grow_up_or_down_with_the_change() {
        let (blocks, names) = draw(SlopeVariant::Diverging);
        assert_eq!(blocks.len(), names.len());
        assert!(blocks.iter().any(|b| b.z1 > 0.0));
        assert!(blocks.iter().any(|b| b.z0 < 0.0));
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&SlopeConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let left = vec![f64::NAN, f64::INFINITY];
        let right = vec![10.0, -5.0];
        for &variant in SlopeVariant::all() {
            let sc = SlopeConfig { variant, labels: &labels, values_left: &left, values_right: &right, ..SlopeConfig::default() };
            let (blocks, _) = layout_named(&sc, &Budget::default());
            assert!(blocks.iter().all(|b| b.cx.is_finite() && b.hw.is_finite()), "{variant:?}");
        }
    }
}
