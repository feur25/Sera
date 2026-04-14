use wasm_bindgen::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct JsOpts {
    width: Option<i32>,
    height: Option<i32>,
    x_label: Option<String>,
    y_label: Option<String>,
    z_label: Option<String>,
    gridlines: Option<bool>,
    sort_order: Option<String>,
    hover_json: Option<String>,
    legend_position: Option<String>,
    palette: Option<Vec<u32>>,
    background: Option<String>,
    bg_color: Option<String>,
    no_x_axis: Option<bool>,
    no_y_axis: Option<bool>,
    color_hex: Option<u32>,
    orientation: Option<String>,
    show_text: Option<bool>,
    color_groups: Option<Vec<String>>,
    show_points: Option<bool>,
    show_regression: Option<bool>,
    regression_type: Option<String>,
    labels: Option<Vec<String>>,
    sizes: Option<Vec<f64>>,
    categories: Option<Vec<String>>,
    bins: Option<i32>,
    show_counts: Option<bool>,
    overlay_color_hex: Option<u32>,
    show_values: Option<bool>,
    color_low: Option<u32>,
    color_mid: Option<u32>,
    color_high: Option<u32>,
    col_labels: Option<Vec<String>>,
    show_pct: Option<bool>,
    inner_radius_ratio: Option<f64>,
    left_label: Option<String>,
    right_label: Option<String>,
    parents: Option<Vec<String>>,
    stacked: Option<bool>,
    series_names: Option<Vec<String>>,
    targets: Option<Vec<f64>>,
    max_vals: Option<Vec<f64>>,
    ranges: Option<Vec<f64>>,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    filled: Option<bool>,
    fill_opacity: Option<i32>,
    bandwidth: Option<f64>,
    overlap: Option<f64>,
    min_font: Option<f64>,
    max_font: Option<f64>,
    min_val: Option<f64>,
    max_val: Option<f64>,
    label: Option<String>,
    series_name_start: Option<String>,
    series_name_end: Option<String>,
    point_labels: Option<Vec<String>>,
    interval_ms: Option<u32>,
    cols: Option<usize>,
    gap: Option<i32>,
    cell_height: Option<i32>,
    images: Option<Vec<Option<String>>>,
    descriptions: Option<Vec<Vec<Vec<String>>>>,
    eps: Option<f64>,
    min_samples: Option<usize>,
    normalize: Option<bool>,
}

impl JsOpts {
    fn w(&self, default: i32) -> i32 { self.width.unwrap_or(default) }
    fn h(&self, default: i32) -> i32 { self.height.unwrap_or(default) }
    fn xl(&self) -> String { self.x_label.clone().unwrap_or_default() }
    fn yl(&self) -> String { self.y_label.clone().unwrap_or_default() }
    fn zl(&self) -> String { self.z_label.clone().unwrap_or_else(|| "Z".to_string()) }
    fn grid(&self) -> bool { self.gridlines.unwrap_or(false) }
    fn srt(&self) -> String { self.sort_order.clone().unwrap_or_else(|| "none".to_string()) }
    fn lp(&self) -> String { self.legend_position.clone().unwrap_or_else(|| "right".to_string()) }
    fn pal(&self) -> Vec<u32> { self.palette.clone().unwrap_or_default() }
    fn hj(&self) -> Vec<crate::html::hover::HoverSlot> {
        self.hover_json.as_ref()
            .filter(|s| !s.is_empty())
            .map(|s| crate::plot::statistical::parse_hover_json(s))
            .unwrap_or_default()
    }
    fn bg_str(&self) -> Option<String> {
        self.background.clone().or_else(|| self.bg_color.clone()).filter(|s| !s.is_empty())
    }
    fn bg(&self) -> Option<&str> { None }
    fn no_x(&self) -> bool { self.no_x_axis.unwrap_or(false) }
    fn no_y(&self) -> bool { self.no_y_axis.unwrap_or(false) }
}

fn parse_opts(opts_json: &str) -> JsOpts {
    serde_json::from_str(opts_json).unwrap_or_default()
}

fn apply(html: String, o: &JsOpts) -> String {
    let bg_str = o.bg_str();
    let bg = bg_str.as_deref();
    crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y())
}

fn apply_bg3d(html: String, o: &JsOpts) -> String {
    let bg_str = o.bg_str();
    if let Some(bg) = bg_str.as_deref() {
        crate::html::hover::apply_bg(html, Some(bg))
    } else {
        html
    }
}

#[wasm_bindgen(js_name = "buildBarChart")]
pub fn build_bar_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let orient = if o.orientation.as_deref() == Some("h") { b'h' } else { b'v' };
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, o.w(900), o.h(480), &hover, orient,
        &groups, o.show_text.unwrap_or(false), &o.xl(), &o.yl(),
        &o.pal(), o.color_hex.unwrap_or(0), o.grid(), &o.srt(),
    );
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildHbar")]
pub fn build_hbar(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, o.w(900), o.h(500), &hover, b'h',
        &groups, o.show_text.unwrap_or(true), &o.xl(), &o.yl(),
        &o.pal(), o.color_hex.unwrap_or(0), o.grid(), &o.srt(),
    );
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildLineChart")]
pub fn build_line_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let hover = o.hj();
    let html = crate::plot::default::render_lines_html(
        title, &labels, &values, o.w(900), o.h(480), &hover,
        o.color_hex.unwrap_or(0x6366F1), &o.xl(), &o.yl(),
        o.grid(), o.show_points.unwrap_or(true), &o.srt(),
    );
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildScatterChart")]
pub fn build_scatter_chart(title: &str, x_json: &str, y_json: &str, opts_json: &str) -> String {
    let x: Vec<f64> = serde_json::from_str(x_json).unwrap_or_default();
    let y: Vec<f64> = serde_json::from_str(y_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let lbls = o.labels.clone().unwrap_or_default();
    let sz = o.sizes.clone().unwrap_or_default();
    let cgs = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_scatter_html(
        title, &x, &y, &lbls, o.w(900), o.h(540), &hover, &sz, &cgs,
        &o.pal(), &o.xl(), &o.yl(), o.color_hex.unwrap_or(0), o.grid(),
        o.show_text.unwrap_or(false), o.show_regression.unwrap_or(false),
        o.regression_type.as_deref().unwrap_or("linear"),
    );
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildHistogram")]
pub fn build_histogram(title: &str, values_json: &str, opts_json: &str) -> String {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{HistogramConfig, render_histogram_html};
    let hover = o.hj();
    let html = render_histogram_html(&HistogramConfig {
        title, values: &values, bins: o.bins.unwrap_or(0) as usize,
        color: o.color_hex.unwrap_or(0x6366F1),
        x_label: &o.xl(), y_label: &o.yl(), show_counts: o.show_counts.unwrap_or(false),
        gridlines: o.grid(), width: o.w(860), height: o.h(380), hover: &hover,
        sort_order: &o.srt(), ..HistogramConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildHistogramOverlay")]
pub fn build_histogram_overlay(title: &str, values_json: &str, overlay_json: &str, opts_json: &str) -> String {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let overlay: Vec<f64> = serde_json::from_str(overlay_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{HistogramConfig, render_histogram_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names = if sn.len() >= 2 { Some((sn[0].as_str(), sn[1].as_str())) } else { None };
    let ref_names: Option<(&str, &str)> = names.as_ref().map(|(a, b)| (*a, *b));
    let html = render_histogram_html(&HistogramConfig {
        title, values: &values, bins: o.bins.unwrap_or(0) as usize,
        color: o.color_hex.unwrap_or(0x6366F1),
        overlay_color: o.overlay_color_hex.unwrap_or(0xF43F5E),
        overlay_values: Some(&overlay),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        width: o.w(860), height: o.h(380), hover: &hover,
        series_names: ref_names,
        sort_order: &o.srt(), ..HistogramConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildGroupedBar")]
pub fn build_grouped_bar(title: &str, cat_json: &str, series_json: &str, opts_json: &str) -> String {
    let category_labels: Vec<String> = serde_json::from_str(cat_json).unwrap_or_default();
    let series_values: Vec<f64> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_values.len() + n_cats - 1) / n_cats } else { 0 };
    let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
    let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)).collect();
        (name.clone(), vals)
    }).collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title, category_labels: &category_labels, series: &series,
        palette: &o.pal(), x_label: &o.xl(), y_label: &o.yl(),
        show_values: o.show_values.unwrap_or(false), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..GroupedBarConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildStackedBar")]
pub fn build_stacked_bar(title: &str, cat_json: &str, series_json: &str, opts_json: &str) -> String {
    let category_labels: Vec<String> = serde_json::from_str(cat_json).unwrap_or_default();
    let series_values: Vec<f64> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_values.len() + n_cats - 1) / n_cats } else { 0 };
    let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
    let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)).collect();
        (name.clone(), vals)
    }).collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title, category_labels: &category_labels, series: &series,
        palette: &o.pal(), x_label: &o.xl(), y_label: &o.yl(),
        show_values: o.show_values.unwrap_or(false), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, stacked: true, ..GroupedBarConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildHeatmap")]
pub fn build_heatmap(title: &str, labels_json: &str, matrix_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let flat_matrix: Vec<f64> = serde_json::from_str(matrix_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{HeatmapConfig, render_heatmap_html};
    let col_lbl = o.col_labels.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_heatmap_html(&HeatmapConfig {
        title, row_labels: &labels, col_labels: &col_lbl, flat_matrix: &flat_matrix,
        show_values: o.show_values.unwrap_or(true),
        color_low: o.color_low.unwrap_or(0x6366F1),
        color_mid: o.color_mid.unwrap_or(0xfafbfc),
        color_high: o.color_high.unwrap_or(0xF43F5E),
        width: o.w(720), height: o.h(440), hover: &hover,
        sort_order: &o.srt(), ..HeatmapConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildPieChart")]
pub fn build_pie_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{PieConfig, render_pie_html};
    let hover = o.hj();
    let html = render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true), sort_order: &o.srt(),
        width: o.w(720), height: o.h(440), hover: &hover, ..PieConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildDonutChart")]
pub fn build_donut_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{PieConfig, render_pie_html};
    let hover = o.hj();
    let html = render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true), sort_order: &o.srt(),
        width: o.w(720), height: o.h(440), hover: &hover,
        donut: o.inner_radius_ratio.unwrap_or(0.55).clamp(0.0, 0.9),
        ..PieConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildBoxplot")]
pub fn build_boxplot(title: &str, cat_json: &str, values_json: &str, opts_json: &str) -> String {
    let category_labels: Vec<String> = serde_json::from_str(cat_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let hover = o.hj();
    let html = crate::plot::statistical::render_boxplot_html(&crate::plot::statistical::BoxplotConfig {
        title, category_labels: &category_labels, values: &values, palette: &o.pal(),
        hover: &hover, x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        width: o.w(900), height: o.h(500), sort_order: &o.srt(), legend_position: &o.lp(),
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildViolin")]
pub fn build_violin(title: &str, cat_json: &str, values_json: &str, opts_json: &str) -> String {
    let categories: Vec<String> = serde_json::from_str(cat_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{ViolinConfig, render_violin_html};
    let hover = o.hj();
    let html = render_violin_html(&ViolinConfig {
        title, categories: &categories, values: &values,
        x_label: &o.xl(), y_label: &o.yl(), palette: &o.pal(), gridlines: o.grid(),
        width: o.w(900), height: o.h(500), sort_order: &o.srt(), hover: &hover,
        legend_position: &o.lp(),
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildSlope")]
pub fn build_slope(title: &str, labels_json: &str, left_json: &str, right_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values_left: Vec<f64> = serde_json::from_str(left_json).unwrap_or_default();
    let values_right: Vec<f64> = serde_json::from_str(right_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{SlopeConfig, render_slope_html};
    let hover = o.hj();
    let ll = o.left_label.as_deref().unwrap_or("Before").to_string();
    let rl = o.right_label.as_deref().unwrap_or("After").to_string();
    let html = render_slope_html(&SlopeConfig {
        title, labels: &labels, values_left: &values_left, values_right: &values_right,
        left_label: &ll, right_label: &rl, palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true), width: o.w(700), height: o.h(500),
        sort_order: &o.srt(), hover: &hover, ..SlopeConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildSunburst")]
pub fn build_sunburst(title: &str, labels_json: &str, parents_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let parents: Vec<String> = serde_json::from_str(parents_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{SunburstConfig, render_sunburst_html};
    let hover = o.hj();
    let html = render_sunburst_html(&SunburstConfig {
        title, labels: &labels, parents: &parents, values: &values,
        width: o.w(700), height: o.h(700), hover: &hover, ..SunburstConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildFunnel")]
pub fn build_funnel(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{FunnelConfig, render_funnel_html};
    let hover = o.hj();
    let html = render_funnel_html(&FunnelConfig {
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true), width: o.w(800), height: o.h(480),
        sort_order: &o.srt(), hover: &hover, ..FunnelConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildTreemap")]
pub fn build_treemap(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{TreemapConfig, render_treemap_html};
    let pars = o.parents.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_treemap_html(&TreemapConfig {
        title, labels: &labels, values: &values, parents: &pars,
        palette: &o.pal(), sort_order: &o.srt(), width: o.w(1100), height: o.h(520),
        hover: &hover, ..TreemapConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildMultilineChart")]
pub fn build_multiline_chart(title: &str, xlabels_json: &str, series_json: &str, opts_json: &str) -> String {
    let x_labels: Vec<String> = serde_json::from_str(xlabels_json).unwrap_or_default();
    let series_flat: Vec<Vec<f64>> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{MultiLineConfig, render_multiline_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() { (0..series_flat.len()).map(|_| String::new()).collect() } else { sn };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let html = render_multiline_html(&MultiLineConfig {
        title, x_labels: &x_labels, series: &series, palette: &o.pal(),
        x_label: &o.xl(), y_label: &o.yl(), show_points: o.show_points.unwrap_or(true),
        gridlines: o.grid(), width: o.w(1100), height: o.h(480), hover: &hover,
        sort_order: &o.srt(), ..MultiLineConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildAreaChart")]
pub fn build_area_chart(title: &str, xlabels_json: &str, series_json: &str, opts_json: &str) -> String {
    let x_labels: Vec<String> = serde_json::from_str(xlabels_json).unwrap_or_default();
    let series_flat: Vec<Vec<f64>> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{AreaConfig, render_area_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() { (0..series_flat.len()).map(|_| String::new()).collect() } else { sn };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let html = render_area_html(&AreaConfig {
        title, x_labels: &x_labels, series: &series, stacked: o.stacked.unwrap_or(false),
        palette: &o.pal(), x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        width: o.w(1100), height: o.h(480), hover: &hover, sort_order: &o.srt(),
        ..AreaConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildWaterfall")]
pub fn build_waterfall(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{WaterfallConfig, render_waterfall_html};
    let hover = o.hj();
    let html = render_waterfall_html(&WaterfallConfig {
        title, labels: &labels, values: &values, x_label: &o.xl(), y_label: &o.yl(),
        show_text: o.show_text.unwrap_or(true), gridlines: o.grid(),
        width: o.w(900), height: o.h(480), sort_order: &o.srt(), hover: &hover,
        legend_position: &o.lp(),
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildBullet")]
pub fn build_bullet(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{BulletConfig, render_bullet_html};
    let targets = o.targets.clone().unwrap_or_default();
    let max_vals = o.max_vals.clone().unwrap_or_default();
    let ranges = o.ranges.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_bullet_html(&BulletConfig {
        title, labels: &labels, values: &values, targets: &targets,
        max_vals: &max_vals, ranges: &ranges, width: o.w(800), height: o.h(300),
        hover: &hover, ..BulletConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildBubbleMap")]
pub fn build_bubble_map(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let hover = o.hj();
    let html = crate::plot::map::render_bubble_map_html(title, &labels, &values, o.w(1200), o.h(600), &hover);
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildChoropleth")]
pub fn build_choropleth(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let hover = o.hj();
    let html = crate::plot::map::render_choropleth_html(title, &labels, &values, o.w(1200), o.h(600), &hover);
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildScatter3dChart")]
pub fn build_scatter3d_chart(title: &str, x_json: &str, y_json: &str, z_json: &str, opts_json: &str) -> String {
    let x: Vec<f64> = serde_json::from_str(x_json).unwrap_or_default();
    let y: Vec<f64> = serde_json::from_str(y_json).unwrap_or_default();
    let z: Vec<f64> = serde_json::from_str(z_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_scatter3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildBar3dChart")]
pub fn build_bar3d_chart(title: &str, x_json: &str, y_json: &str, z_json: &str, opts_json: &str) -> String {
    let x: Vec<f64> = serde_json::from_str(x_json).unwrap_or_default();
    let y: Vec<f64> = serde_json::from_str(y_json).unwrap_or_default();
    let z: Vec<f64> = serde_json::from_str(z_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_bar3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildLine3dChart")]
pub fn build_line3d_chart(title: &str, x_json: &str, y_json: &str, z_json: &str, opts_json: &str) -> String {
    let x: Vec<f64> = serde_json::from_str(x_json).unwrap_or_default();
    let y: Vec<f64> = serde_json::from_str(y_json).unwrap_or_default();
    let z: Vec<f64> = serde_json::from_str(z_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_line3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildRadarChart")]
pub fn build_radar_chart(title: &str, axes_json: &str, series_json: &str, opts_json: &str) -> String {
    let axes: Vec<String> = serde_json::from_str(axes_json).unwrap_or_default();
    let series_flat: Vec<Vec<f64>> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{RadarConfig, render_radar_html};
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let hover = o.hj();
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let html = render_radar_html(&RadarConfig {
        title, axes: &axes, series: &series, palette: &o.pal(),
        filled: o.filled.unwrap_or(true), fill_opacity: o.fill_opacity.unwrap_or(50) as u8,
        width: o.w(700), height: o.h(560), hover: &hover, ..RadarConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildLollipopChart")]
pub fn build_lollipop_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{LollipopConfig, render_lollipop_html};
    let orient = if o.orientation.as_deref() == Some("h") { b'h' } else { b'v' };
    let hover = o.hj();
    let html = render_lollipop_html(&LollipopConfig {
        title, labels: &labels, values: &values, x_label: &o.xl(), y_label: &o.yl(),
        palette: &o.pal(), color_hex: o.color_hex.unwrap_or(0), gridlines: o.grid(),
        show_values: o.show_values.unwrap_or(false), orientation: orient,
        sort_order: &o.srt(), width: o.w(900), height: o.h(480), hover: &hover,
        legend_position: &o.lp(),
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildKdeChart")]
pub fn build_kde_chart(title: &str, values_json: &str, opts_json: &str) -> String {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{KdeConfig, render_kde_html};
    let series: Vec<(String, Vec<f64>)> = if let Some(cats) = o.categories.clone() {
        let mut group_order: Vec<String> = Vec::new();
        let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        for (v, c) in values.iter().zip(cats.iter()) {
            group_vals.entry(c.clone()).or_default().push(*v);
            if !group_order.contains(c) { group_order.push(c.clone()); }
        }
        group_order.into_iter().map(|k| { let v = group_vals.remove(&k).unwrap_or_default(); (k, v) }).collect()
    } else {
        vec![("Series".to_string(), values)]
    };
    let hover = o.hj();
    let xl = o.xl();
    let yl = if o.y_label.is_none() { "Density".to_string() } else { o.yl() };
    let html = render_kde_html(&KdeConfig {
        title, series: &series, palette: &o.pal(), x_label: &xl, y_label: &yl,
        bandwidth: o.bandwidth.unwrap_or(0.0), filled: o.filled.unwrap_or(true),
        fill_opacity: o.fill_opacity.unwrap_or(50) as u8, gridlines: o.grid(),
        width: o.w(900), height: o.h(420), sort_order: &o.srt(), hover: &hover,
        ..KdeConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildRidgelineChart")]
pub fn build_ridgeline_chart(title: &str, values_json: &str, categories_json: &str, opts_json: &str) -> String {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let categories: Vec<String> = serde_json::from_str(categories_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
    let hover = o.hj();
    let html = render_ridgeline_html(&RidgelineConfig {
        title, values: &values, categories: &categories, palette: &o.pal(),
        x_label: &o.xl(), y_label: &o.yl(), overlap: o.overlap.unwrap_or(0.5),
        bandwidth: o.bandwidth.unwrap_or(0.0), width: o.w(900), height: o.h(520),
        gridlines: o.grid(), sort_order: &o.srt(), hover: &hover, ..RidgelineConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildBubble3dChart")]
pub fn build_bubble3d_chart(title: &str, x_json: &str, y_json: &str, z_json: &str, size_json: &str, opts_json: &str) -> String {
    let x: Vec<f64> = serde_json::from_str(x_json).unwrap_or_default();
    let y: Vec<f64> = serde_json::from_str(y_json).unwrap_or_default();
    let z: Vec<f64> = serde_json::from_str(z_json).unwrap_or_default();
    let size_values: Vec<f64> = serde_json::from_str(size_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let n = x.len().min(y.len()).min(z.len()).min(size_values.len());
    let smn = size_values[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let smx = size_values[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sr = (smx - smn).max(1e-9);
    let size_js = format!("var S=[{}];",
        size_values[..n].iter().map(|&s| format!("{:.3}", (s - smn) / sr)).collect::<Vec<_>>().join(","));
    let bg_str = o.bg_str();
    let html = crate::html::js_3d::render_3d_html_impl(
        16, title, &x[..n], &y[..n], &z[..n],
        (&o.xl(), &o.yl(), &o.zl()), &cv, &cl, o.w(900), o.h(560),
        bg_str.as_deref(), size_js.as_bytes(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildPie3dChart")]
pub fn build_pie3d_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::common::apply_sort;
    let srt = o.srt();
    let (labels, values) = apply_sort(&labels, &values, &srt);
    let n = labels.len().min(values.len());
    let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_pie3d_html(
        title, &xv, &yv, &values[..n], ("", "", ""), &cv, &labels[..n].to_vec(),
        o.w(700), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildHeatmap3dChart")]
pub fn build_heatmap3d_chart(title: &str, xlabels_json: &str, ylabels_json: &str, values_json: &str, opts_json: &str) -> String {
    let x_labels: Vec<String> = serde_json::from_str(xlabels_json).unwrap_or_default();
    let y_labels: Vec<String> = serde_json::from_str(ylabels_json).unwrap_or_default();
    let values: Vec<Vec<f64>> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let nr = y_labels.len().min(values.len());
    let nc = x_labels.len();
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new();
    let mut cv = Vec::new(); let mut cl = Vec::new();
    for r in 0..nr {
        let row = &values[r];
        for c2 in 0..nc.min(row.len()) {
            xv.push(c2 as f64); yv.push(r as f64); zv.push(row[c2]);
            cv.push(0.0); cl.push(format!("{}/{}", y_labels[r], x_labels[c2]));
        }
    }
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_heatmap3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildCandlestick3dChart")]
pub fn build_candlestick3d_chart(title: &str, labels_json: &str, open_json: &str, high_json: &str, low_json: &str, close_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let open: Vec<f64> = serde_json::from_str(open_json).unwrap_or_default();
    let high: Vec<f64> = serde_json::from_str(high_json).unwrap_or_default();
    let low: Vec<f64> = serde_json::from_str(low_json).unwrap_or_default();
    let close: Vec<f64> = serde_json::from_str(close_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let n = labels.len().min(open.len()).min(high.len()).min(low.len()).min(close.len());
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new();
    for i in 0..n {
        xv.extend([open[i], high[i], low[i], close[i]]);
        yv.extend([i as f64; 4]);
        zv.extend([0.0f64; 4]);
    }
    let xl = o.xl(); let yl = o.yl(); let zl = o.zl();
    let x_lbl = if xl.is_empty() { "Price" } else { &xl };
    let y_lbl = if yl.is_empty() { "Bar" } else { &yl };
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_candlestick3d_html(
        title, &xv, &yv, &zv, (x_lbl, y_lbl, &zl), &[], &labels,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildDumbbell3dChart")]
pub fn build_dumbbell3d_chart(title: &str, labels_json: &str, start_json: &str, end_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values_start: Vec<f64> = serde_json::from_str(start_json).unwrap_or_default();
    let values_end: Vec<f64> = serde_json::from_str(end_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let s_name = o.series_name_start.as_deref().unwrap_or("Start").to_string();
    let e_name = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let yl = o.yl();
    let y_lbl = if yl.is_empty() { "Item" } else { &yl };
    let n = labels.len().min(values_start.len()).min(values_end.len());
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new(); let mut cv = Vec::new();
    for i in 0..n {
        xv.push(values_start[i]); yv.push(i as f64); zv.push(values_end[i]); cv.push(i as f64);
    }
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_dumbbell3d_html(
        title, &xv, &yv, &zv, (&s_name, y_lbl, &e_name), &cv, &labels,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildFunnel3dChart")]
pub fn build_funnel3d_chart(title: &str, labels_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::common::apply_sort;
    let srt = o.srt();
    let (labels, values) = apply_sort(&labels, &values, &srt);
    let n = labels.len().min(values.len());
    let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_funnel3d_html(
        title, &xv, &yv, &values[..n], ("", "Stage", "Value"),
        &cv, &labels[..n].to_vec(), o.w(700), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildSunburst3dChart")]
pub fn build_sunburst3d_chart(title: &str, labels_json: &str, parents_json: &str, values_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let parents: Vec<String> = serde_json::from_str(parents_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let n = labels.len().min(parents.len()).min(values.len());
    let mut ring_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    ring_map.insert(String::new(), 0);
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new();
    let mut cv = Vec::new(); let mut cl = Vec::new();
    for i in 0..n {
        let parent_ring = ring_map.get(&parents[i]).copied().unwrap_or(0);
        let my_ring = parent_ring + 1;
        ring_map.insert(labels[i].clone(), my_ring);
        xv.push(i as f64); yv.push(my_ring as f64); zv.push(values[i]);
        cv.push(i as f64); cl.push(labels[i].clone());
    }
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_sunburst3d_html(
        title, &xv, &yv, &zv, ("", "Ring", "Value"), &cv, &cl,
        o.w(700), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildStackedBar3dChart")]
pub fn build_stacked_bar3d_chart(title: &str, cat_json: &str, series_json: &str, opts_json: &str) -> String {
    let category_labels: Vec<String> = serde_json::from_str(cat_json).unwrap_or_default();
    let series_values: Vec<Vec<f64>> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let n_cat = category_labels.len();
    let n_ser = series_values.len();
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..n_ser).map(|_| String::new()).collect());
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new();
    let mut cv = Vec::new(); let mut cl = Vec::new();
    for ci in 0..n_cat {
        for si in 0..n_ser {
            let v = series_values[si].get(ci).copied().unwrap_or(0.0);
            xv.push(ci as f64); yv.push(si as f64); zv.push(v);
            cv.push(si as f64); cl.push(format!("{}/{}", category_labels[ci], names[si]));
        }
    }
    let xl = o.xl(); let yl = o.yl(); let zl = o.zl();
    let x_lbl = if xl.is_empty() { "Category" } else { &xl };
    let y_lbl = if yl.is_empty() { "Series" } else { &yl };
    let z_lbl = if zl == "Z" { "Value".to_string() } else { zl };
    let bg_str = o.bg_str();
    let html = crate::plot::statistical::_3d::render_stacked_bar3d_html(
        title, &xv, &yv, &zv, (x_lbl, y_lbl, &z_lbl), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildGlobe3dChart")]
pub fn build_globe3d_chart(title: &str, lats_json: &str, lons_json: &str, values_json: &str, opts_json: &str) -> String {
    let latitudes: Vec<f64> = serde_json::from_str(lats_json).unwrap_or_default();
    let longitudes: Vec<f64> = serde_json::from_str(lons_json).unwrap_or_default();
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let n = latitudes.len().min(longitudes.len()).min(values.len());
    let lbl = o.point_labels.clone().unwrap_or_default();
    let cl = if lbl.is_empty() { (0..n).map(|i| format!("Point {}", i + 1)).collect() } else { lbl };
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    let html = crate::plot::map::_3d::render_globe3d_html(
        title, &longitudes[..n], &latitudes[..n], &values[..n],
        ("Longitude", "Latitude", "Value"), &cv, &cl, o.w(800), o.h(600), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildWordcloud")]
pub fn build_wordcloud(title: &str, words_json: &str, freq_json: &str, opts_json: &str) -> String {
    let words: Vec<String> = serde_json::from_str(words_json).unwrap_or_default();
    let frequencies: Vec<f64> = serde_json::from_str(freq_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let pal = o.pal();
    let hover = o.hj();
    let bg_str = o.bg_str();
    use crate::plot::statistical::wordcloud::{WordCloudConfig, render_wordcloud_html};
    let html = render_wordcloud_html(&WordCloudConfig {
        title, words: &words, frequencies: &frequencies,
        palette: &pal, width: o.w(900), height: o.h(500),
        min_font: o.min_font.unwrap_or(12.0), max_font: o.max_font.unwrap_or(72.0),
        bg_color: bg_str.as_deref(), sort_order: &o.srt(), hover: &hover,
        ..WordCloudConfig::default()
    });
    apply_bg3d(html, &o)
}

#[wasm_bindgen(js_name = "buildCandlestick")]
pub fn build_candlestick(title: &str, labels_json: &str, open_json: &str, high_json: &str, low_json: &str, close_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let open: Vec<f64> = serde_json::from_str(open_json).unwrap_or_default();
    let high: Vec<f64> = serde_json::from_str(high_json).unwrap_or_default();
    let low: Vec<f64> = serde_json::from_str(low_json).unwrap_or_default();
    let close: Vec<f64> = serde_json::from_str(close_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::candlestick::{CandlestickConfig, render_candlestick_html};
    let hover = o.hj();
    let html = render_candlestick_html(&CandlestickConfig {
        title, labels: &labels, open: &open, high: &high, low: &low, close: &close,
        palette: &o.pal(), width: o.w(1100), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..CandlestickConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildDumbbell")]
pub fn build_dumbbell(title: &str, labels_json: &str, start_json: &str, end_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let values_start: Vec<f64> = serde_json::from_str(start_json).unwrap_or_default();
    let values_end: Vec<f64> = serde_json::from_str(end_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::dumbbell::{DumbbellConfig, render_dumbbell_html};
    let s = o.series_name_start.as_deref().unwrap_or("Start").to_string();
    let e = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let hover = o.hj();
    let html = render_dumbbell_html(&DumbbellConfig {
        title, labels: &labels, values_start: &values_start, values_end: &values_end,
        series_names: (&s, &e),
        palette: &o.pal(), width: o.w(1000), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..DumbbellConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildBubble")]
pub fn build_bubble(title: &str, x_json: &str, y_json: &str, sizes_json: &str, opts_json: &str) -> String {
    let x_values: Vec<f64> = serde_json::from_str(x_json).unwrap_or_default();
    let y_values: Vec<f64> = serde_json::from_str(y_json).unwrap_or_default();
    let sizes: Vec<f64> = serde_json::from_str(sizes_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::plot::statistical::bubble::{BubbleConfig, render_bubble_html};
    let cs = o.categories.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_bubble_html(&BubbleConfig {
        title, x_values: &x_values, y_values: &y_values, sizes: &sizes,
        categories: &cs, palette: &o.pal(), width: o.w(900), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..BubbleConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildGauge")]
pub fn build_gauge(title: &str, value: f64, opts_json: &str) -> String {
    let o = parse_opts(opts_json);
    let hover = o.hj();
    use crate::plot::statistical::gauge::{GaugeConfig, render_gauge_html};
    let lbl = o.label.as_deref().unwrap_or("").to_string();
    let html = render_gauge_html(&GaugeConfig {
        title, value, min_val: o.min_val.unwrap_or(0.0), max_val: o.max_val.unwrap_or(100.0),
        label: &lbl, width: o.w(400), height: o.h(300),
        hover: &hover, ..GaugeConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildParallel")]
pub fn build_parallel(title: &str, axes_json: &str, series_json: &str, opts_json: &str) -> String {
    let axes: Vec<String> = serde_json::from_str(axes_json).unwrap_or_default();
    let series_values: Vec<Vec<f64>> = serde_json::from_str(series_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let pal = o.pal();
    let hover = o.hj();
    use crate::plot::statistical::parallel::{ParallelConfig, render_parallel_html};
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_values.len()).map(|_| String::new()).collect());
    let html = render_parallel_html(&ParallelConfig {
        title, axes: &axes, series_names: &names, series_values: &series_values,
        palette: &pal, width: o.w(1000), height: o.h(500),
        hover: &hover, ..ParallelConfig::default()
    });
    apply(html, &o)
}

#[wasm_bindgen(js_name = "buildGrid")]
pub fn build_grid(charts_json: &str, opts_json: &str) -> String {
    let charts: Vec<String> = serde_json::from_str(charts_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    let cols = o.cols.unwrap_or(2).max(1);
    let gap = o.gap.unwrap_or(16);
    let bg_color = o.background.as_deref().unwrap_or("#f0f2f5");
    let cell_height = o.cell_height.unwrap_or(520);
    let mut buf = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        *{{box-sizing:border-box}}body{{margin:0;padding:{gap}px;background:{bg_color}}}\
        .spg{{display:grid;grid-template-columns:repeat({cols},1fr);gap:{gap}px}}\
        .spg-c iframe{{width:100%;height:{cell_height}px;border:none;display:block;\
        border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07)}}\
        </style></head><body><div class=\"spg\">"
    );
    for html in &charts {
        let esc = html.replace('&', "&amp;").replace('"', "&quot;");
        buf.push_str("<div class=\"spg-c\"><iframe srcdoc=\"");
        buf.push_str(&esc);
        buf.push_str("\"></iframe></div>");
    }
    buf.push_str("</div></body></html>");
    buf
}

#[wasm_bindgen(js_name = "buildSlideshow")]
pub fn build_slideshow(charts_json: &str, opts_json: &str) -> String {
    let charts: Vec<String> = serde_json::from_str(charts_json).unwrap_or_default();
    if charts.is_empty() { return String::new(); }
    let o = parse_opts(opts_json);
    let ivms = o.interval_ms.unwrap_or(2500);
    let width = o.w(900);
    let height = o.h(520);
    let title = o.label.as_deref().unwrap_or("").to_string();
    let title_html = if title.is_empty() { String::new() } else {
        format!("<div style=\"color:#1e293b;font-family:system-ui;font-size:22px;font-weight:700;text-align:center;margin-bottom:16px\">{}</div>", title)
    };
    let n = charts.len();
    let json_array = format!("[{}]", charts.iter()
        .map(|s| serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string()))
        .collect::<Vec<_>>().join(","));
    format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        body{{margin:0;padding:24px;background:#f0f2f5;display:flex;flex-direction:column;align-items:center;font-family:system-ui}}\
        .sp-frm{{border-radius:12px;overflow:hidden;box-shadow:0 2px 12px rgba(0,0,0,.1);background:#fff}}\
        .sp-ctrl{{display:flex;gap:10px;margin-top:14px;align-items:center}}\
        .sp-btn{{cursor:pointer;background:#6366f1;color:#fff;border:none;border-radius:8px;padding:7px 20px;font-size:14px;font-weight:600}}\
        .sp-btn:hover{{background:#4f46e5}}\
        .sp-ctr{{color:#64748b;font-size:13px;min-width:64px;text-align:center}}\
        .sp-prog{{width:{width}px;height:4px;background:#e2e8f0;border-radius:2px;margin-top:10px;overflow:hidden}}\
        .sp-bar{{height:100%;background:#6366f1;border-radius:2px;width:0%}}\
        </style></head><body>\
        {title_html}\
        <div class=\"sp-frm\" id=\"sp-frm\" style=\"width:{width}px;height:{height}px\"></div>\
        <div class=\"sp-ctrl\">\
        <button class=\"sp-btn\" id=\"sp-p\">&#9664;</button>\
        <div class=\"sp-ctr\" id=\"sp-c\">1 / {n}</div>\
        <button class=\"sp-btn\" id=\"sp-n\">&#9654;</button>\
        </div>\
        <div class=\"sp-prog\"><div class=\"sp-bar\" id=\"sp-b\"></div></div>\
        <script type=\"application/json\" id=\"sp-d\">{json_array}</script>\
        <script>\
        const frames=JSON.parse(document.getElementById('sp-d').textContent);\
        let idx=0,timer;\
        function show(i){{idx=((i%frames.length)+frames.length)%frames.length;\
          document.getElementById('sp-frm').innerHTML=frames[idx];\
          document.getElementById('sp-c').textContent=(idx+1)+' / '+frames.length;\
          const b=document.getElementById('sp-b');\
          b.style.transition='none';b.style.width='0%';\
          setTimeout(()=>{{b.style.transition='width {ivms}ms linear';b.style.width='100%';}},20);}}\
        function play(){{clearInterval(timer);timer=setInterval(()=>{{show(idx+1);}},{ivms});}}\
        show(0);play();\
        document.getElementById('sp-p').onclick=()=>{{clearInterval(timer);show(idx-1);play();}};\
        document.getElementById('sp-n').onclick=()=>{{clearInterval(timer);show(idx+1);play();}};\
        </script></body></html>"
    )
}

#[wasm_bindgen(js_name = "buildHoverJson")]
pub fn build_hover_json(labels_json: &str, opts_json: &str) -> String {
    let labels: Vec<String> = serde_json::from_str(labels_json).unwrap_or_default();
    let o = parse_opts(opts_json);
    use crate::html::hover::{HoverSlot, slots_to_json};
    let n = labels.len();
    let mut slots = Vec::with_capacity(n);
    for i in 0..n {
        let mut slot = HoverSlot::new(&labels[i]);
        if let Some(ref imgs) = o.images {
            if let Some(Some(ref url)) = imgs.get(i) {
                slot = slot.image(url.clone());
            }
        }
        if let Some(ref descs) = o.descriptions {
            if let Some(row) = descs.get(i) {
                for kv in row {
                    if kv.len() >= 2 {
                        slot = slot.kv(kv[0].clone(), kv[1].clone());
                    }
                }
            }
        }
        slots.push(slot);
    }
    slots_to_json(&slots)
}
