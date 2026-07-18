use crate::plot::{apply_h, parse_all};
pub mod arrowed;
pub mod basic;
pub mod common;
pub mod config;
pub mod delta;
pub mod horizontal;
pub mod lollipop;
pub mod stepped;
pub mod variant;

pub use config::WaterfallConfig;
pub use variant::WaterfallVariant;

pub fn render_waterfall_html(cfg: &WaterfallConfig) -> String {
    use variant::WaterfallVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Stepped => stepped::render(cfg),
        Lollipop => lollipop::render(cfg),
        Arrowed => arrowed::render(cfg),
        Delta => delta::render(cfg),
        Horizontal => horizontal::render(cfg),
    }
}

pub use build as build_waterfall;

#[crate::sera_alias("waterfall", "waterfalls", "waterfall_chart", "waterfall_family")]
#[crate::sera_builder("build_waterfall")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_waterfall_html, WaterfallConfig, WaterfallVariant};
    let hover = o.hj();
    let variant = WaterfallVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_waterfall_html(&WaterfallConfig {
        title,
        variant,
        labels: &labels,
        values: &values,
        x_label: &o.xl(),
        y_label: &o.yl(),
        show_text: o.show_text.unwrap_or(true),
        gridlines: o.grid(),
        width: o.w(900),
        height: o.h(480),
        sort_order: &o.srt(),
        hover: &hover,
        legend_position: &o.lp(),
        orientation: o.orient_byte(),
    });
    apply_h(html, &o)
}
