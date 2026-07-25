use crate::plot::{apply, parse_all};

pub mod circle;
pub mod common;
pub mod config;
pub mod heatmap;
pub mod mixed;
pub mod sorted;
pub mod text;
pub mod variant;

pub use config::CorrelogramConfig;
pub use variant::CorrelogramVariant;

pub fn render_correlogram_html(cfg: &CorrelogramConfig) -> String {
    use CorrelogramVariant::*;
    match cfg.variant {
        Circle   => circle::render(cfg),
        Heatmap  => heatmap::render(cfg),
        Text     => text::render(cfg),
        Mixed    => mixed::render(cfg),
        Sorted   => sorted::render(cfg),
    }
}

pub use build as build_correlogram;

#[crate::sera_alias("correlogram", "corrplot", "correlation_matrix", "corr", "correlation_map")]
#[crate::sera_builder("build_correlogram")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let matrix_2d = a.matrix.unwrap_or_default();
    let matrix: Vec<f64> = matrix_2d.into_iter().flatten().collect();
    let palette = o.pal();
    let variant = CorrelogramVariant::from_str(o.variant.as_deref().unwrap_or("circle"));
    let hover   = o.hj();

    let html = render_correlogram_html(&CorrelogramConfig {
        variant,
        title,
        labels:  &labels,
        matrix:  &matrix,
        palette: &palette,
        hover:   &hover,
        width:   o.w(560),
        height:  o.h(520),
        ..CorrelogramConfig::default()
    });
    apply(html, &o)
}
