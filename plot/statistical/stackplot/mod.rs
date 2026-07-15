use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod streamgraph;
pub mod variant;

pub use config::StackplotConfig;
pub use variant::StackplotVariant;

pub fn render_stackplot_html(cfg: &StackplotConfig) -> String {
    use variant::StackplotVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Streamgraph => streamgraph::render(cfg),
    }
}

pub use build as build_stackplot;

#[crate::sera_alias("stackplot", "stack_plot", "stacked_area")]
#[crate::sera_builder("build_stackplot")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let names_raw = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if names_raw.is_empty() {
        (0..series_flat.len()).map(|i| format!("Series {}", i + 1)).collect()
    } else {
        names_raw
    };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat).collect();
    let dec = crate::plot::decimate::Decimator::for_series(o.max_points, &series);
    let x_labels = dec.apply(x_labels);
    let series = dec.apply_each(series);
    use crate::plot::statistical::{render_stackplot_html, StackplotConfig, StackplotVariant};
    let hover = o.hj();
    let variant = StackplotVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();
    let xl = o.xl();
    let yl = o.yl();
    let html = render_stackplot_html(&StackplotConfig {
        variant,
        title,
        x_label: &xl,
        y_label: &yl,
        gridlines: o.grid(),
        x_labels: &x_labels,
        series: &series,
        palette: &palette,
        hover: &hover,
        width: o.w(1100),
        height: o.h(480),
        ..StackplotConfig::default()
    });
    apply(html, &o)
}
