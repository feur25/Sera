use crate::plot::{apply, parse_all};
pub mod annotated;
pub mod polar;
pub mod basic;
pub mod bubble;
pub mod categorical;
pub mod cluster;
pub mod common;
pub mod config;
pub mod confusion;
pub mod contour;
pub mod correlation;
pub mod density;
pub mod discrete;
pub mod log;
pub mod marginal;
pub mod pivot;
pub mod temporal;
pub mod unequal;
pub mod variant;

pub use config::HeatmapConfig;
pub use variant::HeatmapVariant;

pub struct Heatmap;

pub fn render_heatmap_html(cfg: &HeatmapConfig) -> String {
    match cfg.variant {
        HeatmapVariant::Basic => basic::render(cfg),
        HeatmapVariant::Annotated => annotated::render(cfg),
        HeatmapVariant::Categorical => categorical::render(cfg),
        HeatmapVariant::Unequal => unequal::render(cfg),
        HeatmapVariant::Log => log::render(cfg),
        HeatmapVariant::Discrete => discrete::render(cfg),
        HeatmapVariant::Correlation => correlation::render(cfg),
        HeatmapVariant::Density => density::render(cfg),
        HeatmapVariant::Contour => contour::render(cfg),
        HeatmapVariant::Temporal => temporal::render(cfg),
        HeatmapVariant::Cluster => cluster::render(cfg),
        HeatmapVariant::Bubble => bubble::render(cfg),
        HeatmapVariant::Marginal => marginal::render(cfg),
        HeatmapVariant::Confusion => confusion::render(cfg),
        HeatmapVariant::Pivot => pivot::render(cfg),
        HeatmapVariant::Polar => polar::render(cfg),
    }
}

pub use build as build_heatmap;

#[crate::sera_alias("heatmap", "heatmaps", "heatmap_family", "heatmap_unified")]
#[crate::sera_builder("build_heatmap")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let flat_matrix = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_heatmap_html, HeatmapConfig, HeatmapVariant};
    let col_lbl = o.col_labels.clone().unwrap_or_default();
    let hover = o.hj();
    let palette = o.pal();
    let variant = HeatmapVariant::from_str(&o.variant.clone().unwrap_or_default());
    let x_widths: Vec<f64> = o.widths.clone().unwrap_or_default();
    let y_heights: Vec<f64> = o.ranges.clone().unwrap_or_default();
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let colorbar_position = o
        .colorbar_position
        .clone()
        .unwrap_or_else(|| "right".to_string());
    let html = render_heatmap_html(&HeatmapConfig {
        title,
        variant,
        row_labels: &labels,
        col_labels: &col_lbl,
        flat_matrix: &flat_matrix,
        show_values: o.show_values.unwrap_or(true),
        color_low: o.color_low.unwrap_or(0x636EFA),
        color_mid: o.color_mid.unwrap_or(0xfafbfc),
        color_high: o.color_high.unwrap_or(0xF43F5E),
        palette: &palette,
        discrete_steps: o.bins.unwrap_or(0).max(0) as usize,
        x_widths: &x_widths,
        y_heights: &y_heights,
        colorscale: &colorscale,
        colorbar_position: &colorbar_position,
        origin_lower: o.origin_lower.unwrap_or(false),
        width: o.w(720),
        height: o.h(440),
        hover: &hover,
        sort_order: &o.srt(),
        ..HeatmapConfig::default()
    });
    apply(html, &o)
}
