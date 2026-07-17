use crate::plot::{apply, parse_all};

pub mod basic;
pub mod config;
pub mod variant;

pub use config::VennConfig;
pub use variant::VennVariant;

pub fn render_venn_html(cfg: &VennConfig) -> String {
    use VennVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Euler    => basic::render_euler(cfg),
        Filled   => basic::render_filled(cfg),
        Gradient => basic::render_gradient(cfg),
        Minimal  => basic::render_minimal(cfg),
        Heat     => basic::render_heat(cfg),
    }
}

pub use build as build_venn;

#[crate::sera_alias("venn", "venn_diagram", "venn_chart", "euler", "set_diagram", "overlap")]
#[crate::sera_builder("build_venn")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let values  = a.values.unwrap_or_default();
    let palette = o.pal();
    let variant = VennVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let hover   = o.hj();

    let html = render_venn_html(&VennConfig {
        variant,
        title,
        labels:  &labels,
        values:  &values,
        palette: &palette,
        hover:   &hover,
        width:   o.w(560),
        height:  o.h(420),
        ..VennConfig::default()
    });
    apply(html, &o)
}
