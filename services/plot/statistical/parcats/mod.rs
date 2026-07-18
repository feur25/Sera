use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod variant;

pub use config::ParcatsConfig;
pub use variant::ParcatsVariant;

pub fn render_parcats_html(cfg: &ParcatsConfig) -> String {
    use variant::ParcatsVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Highlight => basic::render_highlight(cfg),
    }
}

pub use build as build_parcats;

#[crate::sera_alias("parcats", "parallel_categories", "parcats_chart", "parallel_categories_chart")]
#[crate::sera_builder("build_parcats")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let category_series = a.category_series.unwrap_or_default();
    use crate::plot::statistical::{render_parcats_html, ParcatsConfig, ParcatsVariant};
    let hover = o.hj();
    let variant = ParcatsVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();
    let html = render_parcats_html(&ParcatsConfig {
        variant,
        title,
        axes: &axes,
        category_series: &category_series,
        palette: &palette,
        hover: &hover,
        width: o.w(900),
        height: o.h(520),
        ..ParcatsConfig::default()
    });
    apply(html, &o)
}
