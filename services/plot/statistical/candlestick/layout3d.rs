use super::common::heikin_ashi;
use super::config::CandlestickConfig;
use super::variant::CandlestickVariant;
use crate::plot::statistical::_3d::ohlc::{
    area_blocks, candle_blocks, extreme_markers, moving_average, stem_blocks, tick_blocks, track_blocks, volume_blocks,
    Quotes, UP,
};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 1.0;
pub const COLORMAP: &str = "updown";
const HOLLOW_UP_TONE: f64 = 0.72;
const VOLUME_SHARE: f64 = 0.5;
const VOLUME_ROW: f64 = 1.6;
const AVERAGE_TONES: [f64; 2] = [0.35, 0.65];

#[derive(Clone, Copy)]
enum Glyph {
    Candle,
    Hollow,
    Outlined,
    Ohlc,
    Range,
    Line,
    Mountain,
}

#[derive(Clone, Copy)]
enum Overlay {
    Off,
    Volume,
    Milestones,
    Averages,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    smoothed: bool,
    overlay: Overlay,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, smoothed: false, overlay: Overlay::Off }
    }

    const fn smoothed(mut self) -> Self {
        self.smoothed = true;
        self
    }

    const fn with(mut self, overlay: Overlay) -> Self {
        self.overlay = overlay;
        self
    }
}

fn recipe(variant: CandlestickVariant) -> Recipe {
    use CandlestickVariant::*;
    let candle = Recipe::of(Glyph::Candle);
    match variant {
        Basic => candle,
        Hollow => Recipe::of(Glyph::Hollow),
        Ohlc => Recipe::of(Glyph::Ohlc),
        Heikin => candle.smoothed(),
        Outlined => Recipe::of(Glyph::Outlined),
        Line => Recipe::of(Glyph::Line),
        Mountain => Recipe::of(Glyph::Mountain),
        Range => Recipe::of(Glyph::Range),
        Volume => candle.with(Overlay::Volume),
        Milestone => candle.with(Overlay::Milestones),
        Indicators => candle.with(Overlay::Averages),
    }
}

fn glyph_blocks(glyph: Glyph, quotes: &Quotes) -> Vec<Bar3DBlock> {
    let n = quotes.len();
    match glyph {
        Glyph::Candle => candle_blocks(quotes, 0.32, 0.05, 0.32, UP),
        Glyph::Hollow => candle_blocks(quotes, 0.32, 0.05, 0.32, HOLLOW_UP_TONE),
        Glyph::Outlined => candle_blocks(quotes, 0.24, 0.12, 0.24, UP),
        Glyph::Ohlc => tick_blocks(quotes, 0.05, 0.3, 0.1),
        Glyph::Range => stem_blocks(quotes, 0.2, 0.2),
        Glyph::Line => track_blocks(&quotes.close[..n], quotes.span(), 0.1, 0.0, 0.5),
        Glyph::Mountain => area_blocks(&quotes.close[..n], quotes.floor(), 0.45),
    }
}

fn overlay_blocks(overlay: Overlay, quotes: &Quotes, volume: &[f64]) -> Vec<Bar3DBlock> {
    let n = quotes.len();
    match overlay {
        Overlay::Off => Vec::new(),
        Overlay::Volume if volume.is_empty() => Vec::new(),
        Overlay::Volume => volume_blocks(quotes, volume, VOLUME_SHARE, VOLUME_ROW, 0.32),
        Overlay::Milestones => extreme_markers(quotes, quotes.span() * 0.05),
        Overlay::Averages => {
            let window = (n / 6).max(2);
            AVERAGE_TONES
                .iter()
                .enumerate()
                .flat_map(|(k, &tone)| {
                    let average = moving_average(&quotes.close[..n], window * (k + 1));
                    track_blocks(&average, quotes.span(), 0.1, (k + 1) as f64, tone)
                })
                .collect()
        }
    }
}

pub fn layout_3d(cfg: &CandlestickConfig) -> Vec<Bar3DBlock> {
    let plan = recipe(cfg.variant);
    let raw = Quotes { open: cfg.open, high: cfg.high, low: cfg.low, close: cfg.close };
    let n = raw.len();
    if n == 0 {
        return Vec::new();
    }
    let smoothed = if plan.smoothed {
        heikin_ashi(&cfg.open[..n], &cfg.high[..n], &cfg.low[..n], &cfg.close[..n])
    } else {
        Default::default()
    };
    let quotes = if plan.smoothed {
        Quotes { open: &smoothed.0, high: &smoothed.1, low: &smoothed.2, close: &smoothed.3 }
    } else {
        raw
    };
    let mut blocks = glyph_blocks(plan.glyph, &quotes);
    blocks.extend(overlay_blocks(plan.overlay, &quotes, cfg.volume));
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPEN: [f64; 6] = [100.0, 102.0, 105.0, 103.0, 108.0, 111.0];
    const HIGH: [f64; 6] = [105.0, 107.0, 109.0, 110.0, 114.0, 116.0];
    const LOW: [f64; 6] = [99.0, 101.0, 103.0, 102.0, 107.0, 109.0];
    const CLOSE: [f64; 6] = [102.0, 105.0, 103.0, 108.0, 112.0, 110.0];
    const VOLUME: [f64; 6] = [1200.0, 900.0, 1500.0, 800.0, 2000.0, 1000.0];

    fn blocks_for(variant: CandlestickVariant) -> Vec<Bar3DBlock> {
        layout_3d(&CandlestickConfig {
            variant,
            open: &OPEN,
            high: &HIGH,
            low: &LOW,
            close: &CLOSE,
            volume: &VOLUME,
            ..CandlestickConfig::default()
        })
    }

    #[test]
    fn every_candlestick_variant_draws_something() {
        for variant in CandlestickVariant::all() {
            assert!(!blocks_for(*variant).is_empty(), "{} must draw blocks", variant.name());
        }
    }

    #[test]
    fn overlays_add_blocks_on_top_of_the_candles() {
        let base = blocks_for(CandlestickVariant::Basic).len();
        assert_eq!(base, 12);
        assert_eq!(blocks_for(CandlestickVariant::Volume).len(), base + 6);
        assert_eq!(blocks_for(CandlestickVariant::Milestone).len(), base + 2);
        assert_eq!(blocks_for(CandlestickVariant::Indicators).len(), base + 12);
    }

    #[test]
    fn heikin_smooths_the_first_open_and_ohlc_uses_three_blocks_per_bar() {
        let heikin = blocks_for(CandlestickVariant::Heikin);
        let basic = blocks_for(CandlestickVariant::Basic);
        assert_ne!(heikin[1].z0, basic[1].z0);
        assert_eq!(blocks_for(CandlestickVariant::Ohlc).len(), 18);
    }

    #[test]
    fn hollow_up_candles_are_softer_than_solid_ones() {
        let hollow = blocks_for(CandlestickVariant::Hollow);
        let solid = blocks_for(CandlestickVariant::Basic);
        assert!(hollow[0].tone.unwrap() < solid[0].tone.unwrap());
    }

    #[test]
    fn empty_quotes_draw_nothing() {
        assert!(layout_3d(&CandlestickConfig::default()).is_empty());
    }
}
