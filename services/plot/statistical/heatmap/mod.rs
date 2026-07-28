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

const HEATMAP_MAX_CELLS: usize = 40_000;

fn decimate_grid(
    row_labels: Vec<String>,
    col_labels: Vec<String>,
    flat_matrix: Vec<f64>,
    x_widths: Vec<f64>,
    y_heights: Vec<f64>,
) -> (Vec<String>, Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let n_rows = row_labels.len();
    let n_cols = if col_labels.is_empty() {
        n_rows
    } else {
        col_labels.len()
    };
    let total = n_rows.saturating_mul(n_cols);
    if n_rows == 0 || n_cols == 0 || flat_matrix.len() < total || total <= HEATMAP_MAX_CELLS {
        return (row_labels, col_labels, flat_matrix, x_widths, y_heights);
    }
    let scale = (total as f64 / HEATMAP_MAX_CELLS as f64).sqrt();
    let step = (scale.ceil() as usize).max(1);
    let new_row_idx: Vec<usize> = (0..n_rows).step_by(step).collect();
    let new_col_idx: Vec<usize> = (0..n_cols).step_by(step).collect();
    let new_row_labels: Vec<String> = new_row_idx.iter().map(|&i| row_labels[i].clone()).collect();
    let new_col_labels: Vec<String> = if col_labels.is_empty() {
        Vec::new()
    } else {
        new_col_idx.iter().map(|&i| col_labels[i].clone()).collect()
    };
    let mut new_matrix = Vec::with_capacity(new_row_idx.len() * new_col_idx.len());
    for &r in &new_row_idx {
        for &c in &new_col_idx {
            new_matrix.push(flat_matrix[r * n_cols + c]);
        }
    }
    let new_x_widths = if x_widths.len() == n_cols {
        new_col_idx.iter().map(|&i| x_widths[i]).collect()
    } else {
        Vec::new()
    };
    let new_y_heights = if y_heights.len() == n_rows {
        new_row_idx.iter().map(|&i| y_heights[i]).collect()
    } else {
        Vec::new()
    };
    (new_row_labels, new_col_labels, new_matrix, new_x_widths, new_y_heights)
}

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
    let (labels, col_lbl, flat_matrix, x_widths, y_heights) =
        decimate_grid(labels, col_lbl, flat_matrix, x_widths, y_heights);
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
        show_values: o.show_values.unwrap_or(false),
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
