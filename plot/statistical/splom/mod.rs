use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod correlation;
pub mod variant;

pub use config::SplomConfig;
pub use variant::SplomVariant;

pub fn render_splom_html(cfg: &SplomConfig) -> String {
    use variant::SplomVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Correlation => correlation::render(cfg),
    }
}

pub use build as build_splom;

#[crate::sera_alias("splom", "scatter_matrix", "splom_chart", "pairplot", "scatterplot_matrix")]
#[crate::sera_builder("build_splom")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
    use crate::plot::statistical::{render_splom_html, SplomConfig, SplomVariant};
    let hover = o.hj();
    let variant = SplomVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let html = render_splom_html(&SplomConfig {
        variant,
        title,
        axes: &axes,
        series_values: &series_values,
        palette: &palette,
        colorscale: &colorscale,
        point_size: o.point_size.unwrap_or(2.2),
        hover: &hover,
        width: o.w(900),
        height: o.h(900),
        ..SplomConfig::default()
    });
    apply(html, &o)
}
