use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod heatmap;
pub mod lines;
pub mod mean;
pub mod quartiles;
pub mod rug;
pub mod spaced;
pub mod variant;

pub use config::RidgelineConfig;
pub use variant::RidgelineVariant;

pub fn render_ridgeline_html(cfg: &RidgelineConfig) -> String {
    use variant::RidgelineVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Lines => lines::render(cfg),
        Quartiles => quartiles::render(cfg),
        Mean => mean::render(cfg),
        Rug => rug::render(cfg),
        Heatmap => heatmap::render(cfg),
        Spaced => spaced::render(cfg),
    }
}

pub use build as build_ridgeline_chart;

#[crate::sera_alias(
    "ridgeline",
    "ridgelines",
    "ridgeline_chart",
    "ridgeline_family",
    "joy_plot",
    "joyplot"
)]
#[crate::sera_builder("build_ridgeline_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let categories = a.categories.or(a.labels).unwrap_or_default();
    use crate::plot::statistical::{render_ridgeline_html, RidgelineConfig, RidgelineVariant};
    let hover = o.hj();
    let variant = RidgelineVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_ridgeline_html(&RidgelineConfig {
        title,
        variant,
        values: &values,
        categories: &categories,
        palette: &o.pal(),
        x_label: &o.xl(),
        y_label: &o.yl(),
        overlap: o.overlap.unwrap_or(0.5),
        bandwidth: o.bandwidth.unwrap_or(0.0),
        width: o.w(900),
        height: o.h(520),
        gridlines: o.grid(),
        sort_order: &o.srt(),
        hover: &hover,
        ..RidgelineConfig::default()
    });
    apply(html, &o)
}
