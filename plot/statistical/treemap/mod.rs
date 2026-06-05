use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod flat;
pub mod gapped;
pub mod gradient;
pub mod heat;
pub mod mono;
pub mod nested;
pub mod outlined;
pub mod variant;

pub use config::TreemapConfig;
pub use variant::TreemapVariant;

pub fn render_treemap_html(cfg: &TreemapConfig) -> String {
    use variant::TreemapVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Flat => flat::render(cfg),
        Gradient => gradient::render(cfg),
        Outlined => outlined::render(cfg),
        Gapped => gapped::render(cfg),
        Nested => nested::render(cfg),
        Heat => heat::render(cfg),
        Mono => mono::render(cfg),
    }
}

pub use build as build_treemap;

#[crate::sera_alias(
    "treemap",
    "treemaps",
    "treemap_chart",
    "treemap_family",
    "treemap_unified"
)]
#[crate::sera_builder("build_treemap")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let pars = a.parents.unwrap_or_default();
    use crate::plot::statistical::{render_treemap_html, TreemapConfig, TreemapVariant};
    let hover = o.hj();
    let variant = TreemapVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_treemap_html(&TreemapConfig {
        title,
        labels: &labels,
        values: &values,
        parents: &pars,
        palette: &o.pal(),
        sort_order: &o.srt(),
        width: o.w(1100),
        height: o.h(520),
        hover: &hover,
        variant,
        ..TreemapConfig::default()
    });
    apply(html, &o)
}
