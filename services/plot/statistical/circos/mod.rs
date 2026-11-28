use crate::plot::{apply, parse_all};

pub mod basic;
pub mod config;
pub mod variant;

pub use config::CircosConfig;
pub use variant::CircosVariant;

pub fn render_circos_html(cfg: &CircosConfig) -> String {
    use CircosVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
    }
}

pub use build as build_circos;

#[crate::sera_alias("circos", "circos_plot", "multitrack_circle", "genome_browser", "circular_tracks")]
#[crate::sera_builder("build_circos")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let item_labels = a.labels.unwrap_or_default();
    let item_groups = a.categories.unwrap_or_default();
    let heat_categories = a.axes.unwrap_or_default();
    let heat_matrix = a.matrix.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let names: Vec<String> = o
        .series_names
        .clone()
        .unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let bar_series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let link_sources = o.edges_i.clone().unwrap_or_default();
    let link_targets = o.edges_j.clone().unwrap_or_default();
    let hover = o.hj();
    let variant = CircosVariant::from_str(o.variant.as_deref().unwrap_or("basic"));

    let html = render_circos_html(&CircosConfig {
        variant,
        title,
        item_labels: &item_labels,
        item_groups: &item_groups,
        bar_series: &bar_series,
        heat_categories: &heat_categories,
        heat_matrix: &heat_matrix,
        link_sources: &link_sources,
        link_targets: &link_targets,
        palette: &o.pal(),
        hover: &hover,
        width: o.w(960),
        height: o.h(960),
        ..CircosConfig::default()
    });
    apply(html, &o)
}
