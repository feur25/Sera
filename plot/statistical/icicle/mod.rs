use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod gapped;
pub mod horizontal;
pub mod radial;
pub mod rank;
pub mod variant;

pub use config::IcicleConfig;
pub use variant::IcicleVariant;

pub fn render_icicle_html(cfg: &IcicleConfig) -> String {
    use variant::IcicleVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Gapped => gapped::render(cfg),
        Horizontal => horizontal::render(cfg),
        Radial => radial::render(cfg),
        Rank => rank::render(cfg),
    }
}

pub use build as build_icicle;

#[crate::sera_alias("icicle", "icicles", "icicle_chart", "icicle_family")]
#[crate::sera_builder("build_icicle")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_icicle_html, IcicleConfig, IcicleVariant};
    let hover = o.hj();
    let variant = IcicleVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_icicle_html(&IcicleConfig {
        title,
        variant,
        labels: &labels,
        parents: &parents,
        values: &values,
        palette: &o.pal(),
        width: o.w(760),
        height: o.h(520),
        hover: &hover,
        ..IcicleConfig::default()
    });
    apply(html, &o)
}
