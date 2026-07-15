use crate::plot::{apply, parse_all};

pub mod basic;
pub mod config;
pub mod variant;

pub use config::HiveConfig;
pub use variant::HiveVariant;

pub fn render_hive_html(cfg: &HiveConfig) -> String {
    use HiveVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Curved   => basic::render_curved(cfg),
        Gradient => basic::render_gradient(cfg),
        Weighted => basic::render_weighted(cfg),
        Minimal  => basic::render_minimal(cfg),
    }
}

pub use build as build_hive;

#[crate::sera_alias("hive", "hive_plot", "hive_chart", "hive_graph", "radial_network")]
#[crate::sera_builder("build_hive")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let axes       = a.axes.unwrap_or_default();
    let labels     = a.labels.unwrap_or_default();
    let categories = a.categories.unwrap_or_default();
    let values     = a.values.unwrap_or_default();
    let sources    = o.edges_i.clone().unwrap_or_default();
    let targets    = o.edges_j.clone().unwrap_or_default();
    let weights    = o.edges_w.clone().unwrap_or_else(|| vec![1.0; sources.len()]);
    let palette    = o.pal();
    let variant    = HiveVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let hover      = o.hj();

    let w = o.w(620);
    let h = o.h(580);
    let outer_r = ((w.min(h) as f64) / 2.0 - 48.0).max(60.0);

    let html = render_hive_html(&HiveConfig {
        variant,
        title,
        axes:       &axes,
        labels:     &labels,
        categories: &categories,
        values:     &values,
        sources:    &sources,
        targets:    &targets,
        weights:    &weights,
        palette:    &palette,
        hover:      &hover,
        width:      w,
        height:     h,
        outer_r,
        ..HiveConfig::default()
    });
    apply(html, &o)
}
