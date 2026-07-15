use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod depth_fade;
pub mod donut;
pub mod gapped;
pub mod mono;
pub mod outlined;
pub mod variant;
pub mod zoomable;

pub use config::SunburstConfig;
pub use variant::SunburstVariant;

pub fn render_sunburst_html(cfg: &SunburstConfig) -> String {
    use variant::SunburstVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Donut => donut::render(cfg),
        Outlined => outlined::render(cfg),
        Gapped => gapped::render(cfg),
        DepthFade => depth_fade::render(cfg),
        Mono => mono::render(cfg),
        Zoomable => zoomable::render(cfg),
    }
}

pub use build as build_sunburst;

#[crate::sera_alias(
    "sunburst",
    "sunbursts",
    "sunburst_chart",
    "sunburst_family",
    "sunburst_unified"
)]
#[crate::sera_builder("build_sunburst")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_sunburst_html, SunburstConfig, SunburstVariant};
    let hover = o.hj();
    let variant = SunburstVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_sunburst_html(&SunburstConfig {
        title,
        variant,
        labels: &labels,
        parents: &parents,
        values: &values,
        palette: &o.pal(),
        width: o.w(700),
        height: o.h(700),
        hover: &hover,
        ..SunburstConfig::default()
    });
    apply(html, &o)
}
