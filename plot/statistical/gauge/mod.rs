use crate::plot::{parse_all, apply};
pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod radial;
pub mod arc270;
pub mod sleek;
pub mod tick;
pub mod segmented;
pub mod glow;
pub mod concentric;

pub use variant::GaugeVariant;
pub use config::GaugeConfig;

pub fn render_gauge_html(cfg: &GaugeConfig) -> String {
    use variant::GaugeVariant::*;
    match cfg.variant {
        Basic      => basic::render(cfg),
        Radial     => radial::render(cfg),
        Arc270     => arc270::render(cfg),
        Sleek      => sleek::render(cfg),
        Tick       => tick::render(cfg),
        Segmented  => segmented::render(cfg),
        Glow       => glow::render(cfg),
        Concentric => concentric::render(cfg),
    }
}


pub use build as build_gauge;

#[crate::sera_alias("gauge", "gauges", "gauge_chart", "gauge_family", "speedometer")]
#[crate::sera_builder("build_gauge")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let value = a.value.unwrap_or(0.0);
    let hover = o.hj();
    use crate::plot::statistical::gauge::{GaugeConfig, GaugeVariant, render_gauge_html};
    let variant = GaugeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let comparison = o.comparison.unwrap_or(0.0);
    let lbl = o.label.as_deref().unwrap_or("").to_string();
    let html = render_gauge_html(&GaugeConfig {
        variant, title, value, min_val: o.min_val.unwrap_or(0.0), max_val: o.max_val.unwrap_or(100.0),
        label: &lbl, comparison, width: o.w(400), height: o.h(300),
        hover: &hover, ..GaugeConfig::default()
    });
    apply(html, &o)
}
