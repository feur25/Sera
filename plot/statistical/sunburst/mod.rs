use crate::plot::{parse_all, apply};
pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod donut;
pub mod flame;
pub mod rainbow;
pub mod outlined;
pub mod gapped;
pub mod depth_fade;
pub mod mono;

pub use variant::SunburstVariant;
pub use config::SunburstConfig;

pub fn render_sunburst_html(cfg: &SunburstConfig) -> String {
    use variant::SunburstVariant::*;
    match cfg.variant {
        Basic     => basic::render(cfg),
        Donut     => donut::render(cfg),
        Flame     => flame::render(cfg),
        Rainbow   => rainbow::render(cfg),
        Outlined  => outlined::render(cfg),
        Gapped    => gapped::render(cfg),
        DepthFade => depth_fade::render(cfg),
        Mono      => mono::render(cfg),
    }
}


pub use build as build_sunburst;

#[crate::sera_alias("sunburst", "sunbursts", "sunburst_chart", "sunburst_family", "sunburst_unified")]
#[crate::sera_builder("build_sunburst")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{SunburstConfig, SunburstVariant, render_sunburst_html};
    let hover = o.hj();
    let variant = SunburstVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_sunburst_html(&SunburstConfig {
        title, variant, labels: &labels, parents: &parents, values: &values, palette: &o.pal(),
        width: o.w(700), height: o.h(700), hover: &hover, ..SunburstConfig::default()
    });
    apply(html, &o)
}
