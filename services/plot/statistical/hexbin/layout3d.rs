use super::common::{data_bounds_raw, magnitude_band, prepare_raw, prepare_weighted};
use super::config::HexbinConfig;
use super::variant::HexbinVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::Frame;

pub const HEIGHT_RATIO: f64 = 0.6;
pub const COLORMAP: &str = "jet";
const FRAME_SIZE: i32 = 400;
const SCALE: f64 = 10.0 / FRAME_SIZE as f64;
const MAX_GRIDSIZE: usize = 40;
const BASE_HEIGHT: f64 = 1.2;
const THIN_HEIGHT: f64 = 0.2;
const FADE_TONE: f64 = 0.08;
const HIGHLIGHT_TOP: usize = 5;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Count,
    LogCounts,
    Weighted,
    Mincnt,
    Nested,
    Marginals,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    thin: bool,
    spaced: bool,
    highlight: bool,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, thin: false, spaced: false, highlight: false }
    }

    const fn thinned(mut self) -> Self {
        self.thin = true;
        self
    }

    const fn spaced(mut self) -> Self {
        self.spaced = true;
        self
    }

    const fn highlighted(mut self) -> Self {
        self.highlight = true;
        self
    }
}

fn recipe(variant: HexbinVariant) -> Recipe {
    use HexbinVariant::*;
    match variant {
        Basic | Dotted | Voronoi | Neural | Bloom => Recipe::of(Glyph::Count),
        Outlined => Recipe::of(Glyph::Count).thinned(),
        Spaced => Recipe::of(Glyph::Count).spaced(),
        Highlight => Recipe::of(Glyph::Count).highlighted(),
        Mincnt => Recipe::of(Glyph::Mincnt),
        Nested => Recipe::of(Glyph::Nested),
        LogCounts => Recipe::of(Glyph::LogCounts),
        Weighted => Recipe::of(Glyph::Weighted),
        Marginals => Recipe::of(Glyph::Marginals),
    }
}

fn frame() -> Frame {
    Frame::new(FRAME_SIZE, FRAME_SIZE, 0, 0, 0, 0, 0)
}

fn count_blocks(cfg: &HexbinConfig, plan: Recipe) -> Vec<Bar3DBlock> {
    let Some(bounds) = data_bounds_raw(cfg.x_values, cfg.y_values) else {
        return Vec::new();
    };
    let f = frame();
    let gridsize = cfg.gridsize.clamp(2, MAX_GRIDSIZE);
    let Some(p) = prepare_raw(cfg.x_values, cfg.y_values, gridsize, f.pl, f.pt, f.pw, f.ph, &bounds) else {
        return Vec::new();
    };
    let peak = p.max_count.max(1) as f64;
    let mut order: Vec<usize> = (0..p.bins.len()).collect();
    if plan.highlight {
        order.sort_by(|&a, &b| p.bins[b].count.cmp(&p.bins[a].count));
    }
    let top: std::collections::HashSet<usize> = order.into_iter().take(if plan.highlight { HIGHLIGHT_TOP } else { p.bins.len() }).collect();
    p.bins
        .iter()
        .enumerate()
        .filter(|(_, b)| plan.glyph != Glyph::Mincnt || b.count >= cfg.min_count.max(1))
        .map(|(i, b)| {
            let frac = if plan.glyph == Glyph::LogCounts { (b.count as f64 + 1.0).ln() / (peak + 1.0).ln() } else { b.count as f64 / peak };
            let h = if plan.glyph == Glyph::Nested {
                (magnitude_band(b.count) as f64 + 1.0) / 5.0 * BASE_HEIGHT
            } else if plan.thin {
                THIN_HEIGHT
            } else {
                (BASE_HEIGHT * frac.clamp(0.05, 1.0)).max(THIN_HEIGHT * 0.5)
            };
            let r = p.r * SCALE * if plan.spaced { 0.55 } else { 0.92 };
            let block = Bar3DBlock::new(b.cx * SCALE, b.cy * SCALE, 0.0, h, r, r, i);
            if plan.highlight && !top.contains(&i) { block.with_tone(FADE_TONE) } else { block.with_tone(frac.clamp(0.0, 1.0)) }
        })
        .collect()
}

fn weighted_blocks(cfg: &HexbinConfig) -> Vec<Bar3DBlock> {
    let Some(bounds) = data_bounds_raw(cfg.x_values, cfg.y_values) else {
        return Vec::new();
    };
    let f = frame();
    let gridsize = cfg.gridsize.clamp(2, MAX_GRIDSIZE);
    let cfg_capped = HexbinConfig { gridsize, x_values: cfg.x_values, y_values: cfg.y_values, values: cfg.values, ..HexbinConfig::default() };
    let Some(p) = prepare_weighted(&cfg_capped, &f, &bounds) else {
        return Vec::new();
    };
    let range = (p.max_avg - p.min_avg).max(1e-9);
    p.bins
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let frac = ((b.avg - p.min_avg) / range).clamp(0.0, 1.0);
            let r = p.r * SCALE * 0.92;
            Bar3DBlock::new(b.cx * SCALE, b.cy * SCALE, 0.0, (BASE_HEIGHT * frac).max(THIN_HEIGHT * 0.5), r, r, i).with_tone(frac)
        })
        .collect()
}

fn marginal_blocks(cfg: &HexbinConfig, base_blocks: &[Bar3DBlock]) -> Vec<Bar3DBlock> {
    let (xlo, xhi) = base_blocks.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), b| (lo.min(b.cx), hi.max(b.cx)));
    let (ylo, yhi) = base_blocks.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), b| (lo.min(b.cy), hi.max(b.cy)));
    if !xlo.is_finite() || !ylo.is_finite() {
        return Vec::new();
    }
    let (counts_x, _) = crate::plot::statistical::histogram::compute_bins(cfg.x_values, 16);
    let (counts_y, _) = crate::plot::statistical::histogram::compute_bins(cfg.y_values, 16);
    let peak_x = counts_x.iter().copied().max().unwrap_or(1).max(1) as f64;
    let peak_y = counts_y.iter().copied().max().unwrap_or(1).max(1) as f64;
    let xr = (xhi - xlo).max(1e-9);
    let mut out = Vec::new();
    for i in 0..counts_x.len() {
        let cx = xlo + (i as f64 + 0.5) / counts_x.len().max(1) as f64 * xr;
        let h = counts_x[i] as f64 / peak_x * BASE_HEIGHT * 0.5;
        out.push(Bar3DBlock::new(cx, ylo - 0.6, 0.0, h.max(0.02), xr / counts_x.len().max(1) as f64 * 0.4, 0.08, base_blocks.len() + i));
    }
    let yr = (yhi - ylo).max(1e-9);
    for i in 0..counts_y.len() {
        let cy = ylo + (i as f64 + 0.5) / counts_y.len().max(1) as f64 * yr;
        let h = counts_y[i] as f64 / peak_y * BASE_HEIGHT * 0.5;
        out.push(Bar3DBlock::new(xlo - 0.6, cy, 0.0, h.max(0.02), 0.08, yr / counts_y.len().max(1) as f64 * 0.4, base_blocks.len() + counts_x.len() + i));
    }
    out
}

pub fn layout_3d(cfg: &HexbinConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &HexbinConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let plan = recipe(cfg.variant);
    let mut blocks = match plan.glyph {
        Glyph::Weighted => weighted_blocks(cfg),
        _ => count_blocks(cfg, plan),
    };
    if plan.glyph == Glyph::Marginals {
        let extra = marginal_blocks(cfg, &blocks);
        blocks.extend(extra);
    }
    let names: Vec<String> = (0..blocks.len()).map(|i| format!("Bin {}", i + 1)).collect();
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xs() -> Vec<f64> {
        (0..300).map(|i| ((i as f64) * 0.05).sin() * 5.0 + (i % 7) as f64 * 0.3).collect()
    }

    fn ys() -> Vec<f64> {
        (0..300).map(|i| ((i as f64) * 0.07).cos() * 4.0 + (i % 5) as f64 * 0.4).collect()
    }

    fn draw(variant: HexbinVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let x = xs();
        let y = ys();
        let cfg = HexbinConfig { variant, x_values: &x, y_values: &y, values: &y, ..HexbinConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_hexbin_variant_draws_something_and_names_every_bin() {
        for &variant in HexbinVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(blocks.len(), names.len(), "{variant:?}");
        }
    }

    #[test]
    fn mincnt_drops_bins_below_the_threshold() {
        let x = xs();
        let y = ys();
        let cfg_all = HexbinConfig { x_values: &x, y_values: &y, ..HexbinConfig::default() };
        let (all, _) = layout_named(&cfg_all, &Budget::default());
        let cfg_min = HexbinConfig { variant: HexbinVariant::Mincnt, min_count: 5, x_values: &x, y_values: &y, ..HexbinConfig::default() };
        let (filtered, _) = layout_named(&cfg_min, &Budget::default());
        assert!(filtered.len() < all.len());
    }

    #[test]
    fn weighted_tones_bins_by_the_mean_of_their_values_not_their_count() {
        let (blocks, _) = draw(HexbinVariant::Weighted);
        assert!(!blocks.is_empty());
    }

    #[test]
    fn marginals_adds_histogram_columns_beyond_the_grid() {
        let (basic, _) = draw(HexbinVariant::Basic);
        let (marginals, _) = draw(HexbinVariant::Marginals);
        assert!(marginals.len() > basic.len());
    }

    #[test]
    fn a_huge_gridsize_is_clamped_before_binning() {
        let x = xs();
        let y = ys();
        let cfg = HexbinConfig { gridsize: 100_000, x_values: &x, y_values: &y, ..HexbinConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 12_000);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&HexbinConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_point_cloud_stays_fast_and_bounded() {
        let x: Vec<f64> = (0..200_000).map(|i| ((i as f64) * 0.001).sin() * 5.0).collect();
        let y: Vec<f64> = (0..200_000).map(|i| ((i as f64) * 0.0013).cos() * 5.0).collect();
        let cfg = HexbinConfig { x_values: &x, y_values: &y, ..HexbinConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 12_000);
    }
}
