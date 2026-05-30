use crate::plot::{parse_all, apply};
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


pub use build as build_candlestick;

#[crate::sera_alias("candlestick", "candlesticks", "candlestick_chart", "candlestick_family", "ohlc", "ohlc_chart")]
#[crate::sera_builder("build_candlestick")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
    use crate::plot::statistical::{CandlestickConfig, CandlestickVariant, render_candlestick_html};
    let hover = o.hj();
    let variant = CandlestickVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_candlestick_html(&CandlestickConfig {
        title, labels: &labels, open: &open, high: &high, low: &low, close: &close,
        palette: &o.pal(), width: o.w(1100), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, variant, ..CandlestickConfig::default()
    });
    apply(html, &o)
}
