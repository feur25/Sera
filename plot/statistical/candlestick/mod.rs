pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod hollow;
pub mod ohlc;
pub mod heikin;
pub mod outlined;
pub mod line;
pub mod mountain;
pub mod range;

pub use variant::CandlestickVariant;
pub use config::CandlestickConfig;

pub fn render_candlestick_html(cfg: &CandlestickConfig) -> String {
    use variant::CandlestickVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Hollow   => hollow::render(cfg),
        Ohlc     => ohlc::render(cfg),
        Heikin   => heikin::render(cfg),
        Outlined => outlined::render(cfg),
        Line     => line::render(cfg),
        Mountain => mountain::render(cfg),
        Range    => range::render(cfg),
    }
}

pub fn demo_kwargs(v: CandlestickVariant) -> &'static str {
    use CandlestickVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Hollow             => hollow::DEMO_KWARGS,
        Ohlc               => ohlc::DEMO_KWARGS,
        Heikin             => heikin::DEMO_KWARGS,
        Outlined           => outlined::DEMO_KWARGS,
        Line               => line::DEMO_KWARGS,
        Mountain           => mountain::DEMO_KWARGS,
        Range              => range::DEMO_KWARGS,
    }
}
