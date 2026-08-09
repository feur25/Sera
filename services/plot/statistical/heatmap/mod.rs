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
pub mod radial_cluster;
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
        HeatmapVariant::RadialCluster => radial_cluster::render(cfg),
    }
}

pub use build as build_heatmap;

const HEATMAP_COLOR_LOW: u32 = 0x313695;
const HEATMAP_COLOR_HIGH: u32 = 0xA50026;
const HEATMAP_COLOR_SCALE: i32 = 1000;

pub fn render_heatmap_family_native(
    title: &str,
    row_labels: &[String],
    values: &[f64],
    opts: &crate::plot::canvas_points::NativeChartOpts,
) -> (String, u64) {
    use crate::html::hover::{html_id, html_prefix, html_suffix};
    use crate::plot::canvas_points::{pack_scalar_i16, push_color_patch_js};
    use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i};

    let n_cols = opts.cols.max(1) as usize;
    let n = values.len();
    let n_rows = n.div_ceil(n_cols).max(1);

    let pad_l = 70i32;
    let pad_t = 36i32;
    let pad_b = 20i32;
    let pad_r = 20i32;
    let plot_w = (opts.width - pad_l - pad_r).max(10);
    let plot_h = (opts.height - pad_t - pad_b).max(10);
    let cell_w = plot_w as f64 / n_cols as f64;
    let cell_h = plot_h as f64 / n_rows as f64;

    let (min_v, max_v) = crate::bindings::utils::simd_ops::find_minmax(values);
    let range_v = (max_v - min_v).max(1e-12);
    let values_px = pack_scalar_i16(values, min_v, range_v, HEATMAP_COLOR_SCALE);

    let hid = html_id();
    let svg_id = format!("sphmsvg{hid}");
    let mut buf = Vec::<u8>::with_capacity(n * 30 + 8192);
    html_prefix(&mut buf, title, hid);
    push_b(&mut buf, b"<svg id=\"");
    buf.extend_from_slice(svg_id.as_bytes());
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, opts.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, opts.height);
    push_b(&mut buf, b"\" style=\"display:block\">");

    if !title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, opts.width / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"700\" font-size=\"15\" fill=\"#1a202c\">");
        escape_xml(&mut buf, title);
        push_b(&mut buf, b"</text>");
    }

    let row_tick_count = row_labels.len().min(n_rows).min(10);
    if row_tick_count > 0 {
        push_b(&mut buf, b"<g fill=\"#6b7280\" font-family=\"Arial,sans-serif\" font-size=\"9\" text-anchor=\"end\">");
        for t in 0..row_tick_count {
            let ridx = if row_tick_count == 1 { 0 } else { t * (n_rows - 1) / (row_tick_count - 1) };
            if ridx >= row_labels.len() {
                continue;
            }
            let y = pad_t as f64 + (ridx as f64 + 0.5) * cell_h;
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, pad_l - 6);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, y + 3.0);
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &row_labels[ridx]);
            push_b(&mut buf, b"</text>");
        }
        push_b(&mut buf, b"</g>");
    }

    push_b(&mut buf, b"<g>");
    for i in 0..n {
        let row = i / n_cols;
        let col = i % n_cols;
        let x = pad_l as f64 + col as f64 * cell_w;
        let y = pad_t as f64 + row as f64 * cell_h;
        push_b(&mut buf, b"<rect class=\"cell\" x=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, y);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, (cell_w - 0.5).max(0.5));
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, (cell_h - 0.5).max(0.5));
        push_b(&mut buf, b"\" fill=\"#eeeeee\"/>");
    }
    push_b(&mut buf, b"</g></svg>");

    push_b(&mut buf, b"<script>(function(){");
    push_b(&mut buf, b"var svg=document.getElementById('");
    buf.extend_from_slice(svg_id.as_bytes());
    push_b(&mut buf, b"');");
    push_b(&mut buf, b"var CELLS=svg.querySelectorAll('rect.cell');");
    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var V=b64('");
    buf.extend_from_slice(values_px.as_bytes());
    push_b(&mut buf, b"');");
    push_color_patch_js(&mut buf, hid, b"CELLS", HEATMAP_COLOR_SCALE, HEATMAP_COLOR_LOW, HEATMAP_COLOR_HIGH);
    push_b(&mut buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(&mut buf, b"'](Array.from({length:V.length},function(_,i){return i;}),V);");
    push_b(&mut buf, b"})();</script>");

    html_suffix(&mut buf, hid, "[]");
    (unsafe { String::from_utf8_unchecked(buf) }, hid)
}

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
