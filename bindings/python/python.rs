#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use pyo3::types::PyAny;

#[cfg(feature = "python")]
pub(crate) struct ChartOpts {
    pub w: i32,
    pub h: i32,
    pub pal: Vec<u32>,
    pub xl: String,
    pub yl: String,
    pub zl: String,
    pub grid: bool,
    pub srt: String,
    pub lp: String,
    pub hj: Option<String>,
}

#[cfg(feature = "python")]
impl ChartOpts {
    fn hover(&self) -> Vec<crate::html::hover::HoverSlot> {
        self.hj.as_ref().map(|s| crate::plot::statistical::parse_hover_json(s)).unwrap_or_default()
    }
    fn from_params(w: i32, h: i32, palette: Option<Vec<u32>>, xl: &str, yl: &str, grid: bool, srt: &str, hj: &str, lp: &str) -> Self {
        Self { w, h, pal: palette.unwrap_or_default(), xl: xl.to_string(), yl: yl.to_string(), zl: "Z".to_string(), grid, srt: srt.to_string(), lp: lp.to_string(), hj: if hj.is_empty() { None } else { Some(hj.to_string()) } }
    }
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(name = "set_bg", signature = (html, color=None))]
pub fn set_bg_fn(html: &PyAny, color: Option<&str>) -> PyResult<Chart> {
    let raw: String = if let Ok(chart) = html.extract::<PyRef<Chart>>() {
        chart.html.clone()
    } else {
        html.extract::<String>()?
    };
    Ok(Chart::new(crate::html::hover::apply_bg(raw, color)))
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn show_chart_value(chart_json: &str) -> bool {
    let chart_json_c = std::ffi::CString::new(chart_json).unwrap_or_default();
    unsafe {
        crate::viewer::chart::sera_show_chart_value(chart_json_c.as_ptr())
    }
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn bench_chart_value(chart_json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(chart_json).is_ok()
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn set_chart_kind(kind: u8) -> bool {
    crate::viewer::chart::sera_set_current_chart_kind(kind);
    true
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn set_chart_orientation(vertical: bool) -> bool {
    crate::viewer::chart::sera_set_chart_orientation(vertical);
    true
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (n=2000))]
pub fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) {
    use std::time::Instant;
    let ages: Vec<f64> = (0..891).map(|i| 10.0 + (i % 70) as f64).collect();
    let fare: Vec<f64> = (0..891).map(|i| (i % 50) as f64 * 2.5).collect();
    let ages100: Vec<f64> = ages[..100].to_vec();
    let fare100: Vec<f64> = fare[..100].to_vec();
    let labs: Vec<String> = (0..30).map(|i| format!("Cat {i}")).collect();
    let vals: Vec<f64> = (0..30).map(|i| i as f64 * 3.7).collect();
    let n_lbl = 11usize;
    let corr_labels: Vec<String> = (0..n_lbl).map(|i| format!("F{i}")).collect();
    let flat: Vec<f64> = (0..n_lbl * n_lbl).map(|i| ((i % 11) as f64 - 5.0) * 0.15).collect();
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, HeatmapConfig, render_heatmap_html};
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_histogram_html(&HistogramConfig {
            title: "B", values: &ages, bins: 20, width: 900, height: 400,
            ..HistogramConfig::default()
        });
    }
    let hist_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_bars_html("B", &labs, &vals, 900, 480, &[], b'v', &[], false, "", "", &[], 0, true, "");
    }
    let bar_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_scatter_html("B", &ages100, &fare100, &[], 900, 540, &[], &[], &[], &[], "", "", 0, true, false);
    }
    let scatter_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_heatmap_html(&HeatmapConfig {
            title: "B", row_labels: &corr_labels, col_labels: &[], flat_matrix: &flat,
            width: 800, ..HeatmapConfig::default()
        });
    }
    let heatmap_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    (hist_ms, bar_ms, scatter_ms, heatmap_ms)
}

#[cfg(feature = "python")]
#[pyfunction]
/// Simple HTML chart (no kwargs). Args: title, labels, values, width, height.
pub fn build_html_chart(title: &str, labels: Vec<String>, values: Vec<f64>, width: i32, height: i32) -> Chart {
    Chart::new({
        let exporter = crate::html::FastHtmlExporter::new(width, height, title.to_string());
        exporter.build_optimized(labels, values)
    })
}
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, color_hex=0_u32, orientation="v", show_text=false, color_groups=None, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Bar chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), orientation ("v"/"h"), show_text (bool), color_groups (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_bar_chart(title: &str, labels: Vec<String>, values: Vec<f64>, color_hex: u32, orientation: &str, show_text: bool, color_groups: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    let orient = if orientation == "h" { b'h' } else { b'v' };
    let groups = color_groups.unwrap_or_default();
    let hover = o.hover();
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, o.w, o.h, &hover, orient, &groups, show_text, &o.xl, &o.yl, &o.pal, color_hex, o.grid, &o.srt,
    );
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, color_hex=0_u32, show_text=true, color_groups=None, width=900_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Horizontal bar chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), show_text (bool, default true), color_groups (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_hbar(title: &str, labels: Vec<String>, values: Vec<f64>, color_hex: u32, show_text: bool, color_groups: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    let groups = color_groups.unwrap_or_default();
    let hover = o.hover();
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, o.w, o.h, &hover, b'h', &groups, show_text, &o.xl, "", &o.pal, color_hex, o.grid, &o.srt,
    );
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, color_hex=0x6366F1_u32, show_points=true, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Line chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), show_points (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_line_chart(title: &str, labels: Vec<String>, values: Vec<f64>, color_hex: u32, show_points: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    let cap = 100usize;
    let (vals, step) = if values.len() > cap { let s = (values.len() + cap - 1) / cap; (values.into_iter().step_by(s).collect::<Vec<_>>(), s) } else { (values, 1) };
    let lbls: Vec<String> = if step <= 1 || labels.is_empty() { labels } else { labels.into_iter().step_by(step).collect() };
    let hover = o.hover();
    let html = crate::plot::default::render_lines_html(
        title, &lbls, &vals, o.w, o.h, &hover, color_hex, &o.xl, &o.yl, o.grid, show_points, &o.srt,
    );
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, *, color_hex=0_u32, show_text=false, labels=None, sizes=None, color_groups=None, width=900_i32, height=540_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Scatter plot.
///
/// Args: title, x_values, y_values.
///
/// Kwargs: color_hex (int), show_text (bool), labels (list[str]), sizes (list[float]),
/// color_groups (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_scatter_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, color_hex: u32, show_text: bool, labels: Option<Vec<String>>, sizes: Option<Vec<f64>>, color_groups: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    let cap = 200usize;
    let step = { let n = x_values.len().max(y_values.len()); if n > cap { (n + cap - 1) / cap } else { 1 } };
    let x: Vec<f64> = if step > 1 { x_values.into_iter().step_by(step).collect() } else { x_values };
    let y: Vec<f64> = if step > 1 { y_values.into_iter().step_by(step).collect() } else { y_values };
    let lbls: Vec<String> = { let l = labels.unwrap_or_default(); if step <= 1 || l.is_empty() { l } else { l.into_iter().step_by(step).collect() } };
    let sz: Vec<f64> = { let s = sizes.unwrap_or_default(); if step > 1 { s.into_iter().step_by(step).collect() } else { s } };
    let cgs: Vec<String> = { let c = color_groups.unwrap_or_default(); if step > 1 { c.into_iter().step_by(step).collect() } else { c } };
    let hover = o.hover();
    let html = crate::plot::default::render_scatter_html(
        title, &x, &y, &lbls, o.w, o.h, &hover, &sz, &cgs, &o.pal, &o.xl, &o.yl, color_hex, o.grid, show_text,
    );
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, *, color_hex=0x6366F1_u32, bins=0_i32, show_counts=false, width=860_i32, height=380_i32, x_label="", y_label="Count", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Histogram.
///
/// Args: title, values.
///
/// Kwargs: color_hex (int), bins (int, 0=auto), show_counts (bool),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_histogram(title: &str, values: Vec<f64>, color_hex: u32, bins: i32, show_counts: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{HistogramConfig, render_histogram_html};
    let cap = 1000usize;
    let (vals, scale) = if values.len() > cap { let s = (values.len() + cap - 1) / cap; (values.into_iter().step_by(s).collect::<Vec<_>>(), s) } else { (values, 1) };
    let hover = o.hover();
    let html = render_histogram_html(&HistogramConfig {
        title, values: &vals, bins: bins as usize, color: color_hex, x_label: &o.xl, y_label: &o.yl,
        show_counts, gridlines: o.grid, width: o.w, height: o.h, hover: &hover, count_scale: scale,
        sort_order: &o.srt, ..HistogramConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, overlay_values, *, color_hex=0x6366F1_u32, overlay_color_hex=0xF43F5E_u32, bins=0_i32, series_names=None, width=860_i32, height=380_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Overlaid histogram (two distributions).
///
/// Args: title, values, overlay_values.
///
/// Kwargs: color_hex (int), overlay_color_hex (int), bins (int, 0=auto),
/// series_names (list[str] of 2 names),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_histogram_overlay(title: &str, values: Vec<f64>, overlay_values: Vec<f64>, color_hex: u32, overlay_color_hex: u32, bins: i32, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{HistogramConfig, render_histogram_html};
    let hover = o.hover();
    let names: Option<(String, String)> = series_names.and_then(|v| {
        let a = v.get(0).cloned().unwrap_or_default();
        let b = v.get(1).cloned().unwrap_or_default();
        if a.is_empty() && b.is_empty() { None } else { Some((a, b)) }
    });
    let html = render_histogram_html(&HistogramConfig {
        title, values: &values, bins: bins as usize, color: color_hex, overlay_color: overlay_color_hex,
        overlay_values: Some(&overlay_values), width: o.w, height: o.h, hover: &hover,
        series_names: names.as_ref().map(|(a, b)| (a.as_str(), b.as_str())),
        x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
        sort_order: &o.srt, ..HistogramConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_values, *, show_values=false, series_names=None, width=1100_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Grouped bar chart.
///
/// Args: title, category_labels, series_values (flat, num_series inferred from series_names).
///
/// Kwargs: show_values (bool), series_names (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_grouped_bar(title: &str, category_labels: Vec<String>, series_values: Vec<f64>, show_values: bool, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
    let hover = o.hover();
    let sn = series_names.unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_values.len() + n_cats - 1) / n_cats } else { 0 };
    let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
    let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)).collect();
        (name.clone(), vals)
    }).collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title, category_labels: &category_labels, series: &series,
        palette: &o.pal, x_label: &o.xl, y_label: &o.yl, show_values, gridlines: o.grid, sort_order: &o.srt,
        hover: &hover, ..GroupedBarConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_values, *, show_values=false, series_names=None, width=1100_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Stacked bar chart.
///
/// Args: title, category_labels, series_values (flat, num_series inferred from series_names).
///
/// Kwargs: show_values (bool), series_names (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_stacked_bar(title: &str, category_labels: Vec<String>, series_values: Vec<f64>, show_values: bool, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
    let hover = o.hover();
    let sn = series_names.unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_values.len() + n_cats - 1) / n_cats } else { 0 };
    let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
    let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)).collect();
        (name.clone(), vals)
    }).collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title, category_labels: &category_labels, series: &series,
        palette: &o.pal, x_label: &o.xl, y_label: &o.yl, show_values, gridlines: o.grid, sort_order: &o.srt,
        hover: &hover, stacked: true, ..GroupedBarConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, flat_matrix, *, show_values=true, color_low=0x6366F1_u32, color_mid=0xfafbfc_u32, color_high=0xF43F5E_u32, col_labels=None, width=720_i32, height=440_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Heatmap.
///
/// Args: title, labels (row labels), flat_matrix (row-major flat values).
///
/// Kwargs: show_values (bool, default true), color_low/color_mid/color_high (int),
/// col_labels (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_heatmap(title: &str, labels: Vec<String>, flat_matrix: Vec<f64>, show_values: bool, color_low: u32, color_mid: u32, color_high: u32, col_labels: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{HeatmapConfig, render_heatmap_html};
    let col_lbl = col_labels.unwrap_or_default();
    let hover = o.hover();
    let html = render_heatmap_html(&HeatmapConfig {
        title, row_labels: &labels, col_labels: &col_lbl, flat_matrix: &flat_matrix,
        show_values, color_low, color_mid, color_high, width: o.w, height: o.h, hover: &hover,
        sort_order: &o.srt, ..HeatmapConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, show_pct=true, width=720_i32, height=440_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Pie chart.
///
/// Args: title, labels, values.
///
/// Kwargs: show_pct (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_pie_chart(title: &str, labels: Vec<String>, values: Vec<f64>, show_pct: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{PieConfig, render_pie_html};
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &o.pal,
        show_pct, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover,
        ..PieConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, show_pct=true, inner_radius_ratio=0.55_f64, width=720_i32, height=440_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Donut chart.
///
/// Args: title, labels, values.
///
/// Kwargs: show_pct (bool, default true), inner_radius_ratio (float, default 0.55),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_donut_chart(title: &str, labels: Vec<String>, values: Vec<f64>, show_pct: bool, inner_radius_ratio: f64, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{PieConfig, render_pie_html};
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &o.pal,
        show_pct, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover,
        donut: inner_radius_ratio.clamp(0.0, 0.9),
        ..PieConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, values, *, width=900_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Box plot.
///
/// Args: title, category_labels, values (flat, auto-split by categories).
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_boxplot(title: &str, category_labels: Vec<String>, values: Vec<f64>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(crate::plot::statistical::render_boxplot_html(&crate::plot::statistical::BoxplotConfig {
        title, category_labels: &category_labels, values: &values, palette: &o.pal, hover: &hover,
        x_label: &o.xl, y_label: &o.yl, gridlines: o.grid, width: o.w, height: o.h,
        sort_order: &o.srt, legend_position: &o.lp,
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, categories, values, *, width=900_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Violin plot.
///
/// Args: title, categories, values (flat, auto-split by categories).
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_violin(title: &str, categories: Vec<String>, values: Vec<f64>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{ViolinConfig, render_violin_html};
    let hover = o.hover();
    let html = render_violin_html(&ViolinConfig {
        title, categories: &categories, values: &values,
        x_label: &o.xl, y_label: &o.yl, palette: &o.pal, gridlines: o.grid, width: o.w, height: o.h,
        sort_order: &o.srt, hover: &hover, legend_position: &o.lp,
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_left, values_right, *, left_label="Before", right_label="After", show_text=true, width=700_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Slope chart (before/after comparison).
///
/// Args: title, labels, values_left, values_right.
///
/// Kwargs: left_label (str, default "Before"), right_label (str, default "After"),
/// show_text (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_slope(title: &str, labels: Vec<String>, values_left: Vec<f64>, values_right: Vec<f64>, left_label: &str, right_label: &str, show_text: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{SlopeConfig, render_slope_html};
    let hover = o.hover();
    let html = render_slope_html(&SlopeConfig {
        title, labels: &labels, values_left: &values_left, values_right: &values_right,
        left_label, right_label, palette: &o.pal, show_text, width: o.w, height: o.h,
        sort_order: &o.srt, hover: &hover, ..SlopeConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, parents, values, *, width=700_i32, height=700_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Sunburst chart (hierarchical).
///
/// Args: title, labels, parents (parent label per node), values.
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_sunburst(title: &str, labels: Vec<String>, parents: Vec<String>, values: Vec<f64>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{SunburstConfig, render_sunburst_html};
    let hover = o.hover();
    let html = render_sunburst_html(&SunburstConfig {
        title, labels: &labels, parents: &parents, values: &values, width: o.w, height: o.h,
        hover: &hover, ..SunburstConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, show_text=true, width=800_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Funnel chart.
///
/// Args: title, labels, values.
///
/// Kwargs: show_text (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_funnel(title: &str, labels: Vec<String>, values: Vec<f64>, show_text: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{FunnelConfig, render_funnel_html};
    let hover = o.hover();
    let html = render_funnel_html(&FunnelConfig {
        title, labels: &labels, values: &values, palette: &o.pal, show_text, width: o.w, height: o.h,
        sort_order: &o.srt, hover: &hover, ..FunnelConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, parents=None, width=1100_i32, height=520_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Treemap.
///
/// Args: title, labels, values.
///
/// Kwargs: parents (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_treemap(title: &str, labels: Vec<String>, values: Vec<f64>, parents: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{TreemapConfig, render_treemap_html};
    let pars = parents.unwrap_or_default();
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_treemap_html(&TreemapConfig {
        title, labels: &labels, values: &values, parents: &pars,
        palette: &o.pal, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover,
        ..TreemapConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_values, *, show_points=true, series_names=None, width=1100_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Multi-line chart (multiple series).
///
/// Args: title, x_labels, series_values (list of list[float]).
///
/// Kwargs: show_points (bool, default true), series_names (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_multiline_chart(title: &str, x_labels: Vec<String>, series_values: Vec<Vec<f64>>, show_points: bool, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{MultiLineConfig, render_multiline_html};
    let hover = o.hover();
    let sn = series_names.unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() { (0..series_values.len()).map(|_| String::new()).collect() } else { sn };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_values.into_iter()).collect();
    let html = render_multiline_html(&MultiLineConfig {
        title, x_labels: &x_labels, series: &series, palette: &o.pal,
        x_label: &o.xl, y_label: &o.yl, show_points, gridlines: o.grid, width: o.w, height: o.h, hover: &hover,
        sort_order: &o.srt, ..MultiLineConfig::default()
    });
    Ok(Chart::new(crate::html::hover::apply_opts(html, background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_values, *, stacked=false, series_names=None, width=1100_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Area chart.
///
/// Args: title, x_labels, series_values (list of list[float]).
///
/// Kwargs: stacked (bool), series_names (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_area_chart(title: &str, x_labels: Vec<String>, series_values: Vec<Vec<f64>>, stacked: bool, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{AreaConfig, render_area_html};
    let hover = o.hover();
    let sn = series_names.unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() { (0..series_values.len()).map(|_| String::new()).collect() } else { sn };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_values.into_iter()).collect();
    Ok(Chart::new(crate::html::hover::apply_opts(render_area_html(&AreaConfig {
        title, x_labels: &x_labels, series: &series, stacked, palette: &o.pal,
        x_label: &o.xl, y_label: &o.yl, gridlines: o.grid, width: o.w, height: o.h, hover: &hover,
        sort_order: &o.srt, ..AreaConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, show_text=true, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Waterfall chart.
///
/// Args: title, labels, values (positive=up, negative=down).
///
/// Kwargs: show_text (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_waterfall(title: &str, labels: Vec<String>, values: Vec<f64>, show_text: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{WaterfallConfig, render_waterfall_html};
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_waterfall_html(&WaterfallConfig {
        title, labels: &labels, values: &values, x_label: &o.xl, y_label: &o.yl, show_text, gridlines: o.grid, width: o.w, height: o.h,
        sort_order: &o.srt, hover: &hover, legend_position: &o.lp,
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, targets=None, max_vals=None, ranges=None, width=800_i32, height=300_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Bullet chart.
///
/// Args: title, labels, values.
///
/// Kwargs: targets (list[float]), max_vals (list[float]), ranges (list[float]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_bullet(title: &str, labels: Vec<String>, values: Vec<f64>, targets: Option<Vec<f64>>, max_vals: Option<Vec<f64>>, ranges: Option<Vec<f64>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{BulletConfig, render_bullet_html};
    let targets = targets.unwrap_or_default();
    let max_vals = max_vals.unwrap_or_default();
    let ranges = ranges.unwrap_or_default();
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_bullet_html(&BulletConfig {
        title, labels: &labels, values: &values, targets: &targets, max_vals: &max_vals, ranges: &ranges, width: o.w, height: o.h,
        hover: &hover, ..BulletConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, width=1200_i32, height=600_i32, hover_json="", series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Bubble map (world map with sized bubbles).
///
/// Args: title, labels (country names/codes), values.
///
/// Kwargs: width, height, hover_json, series_names (list[str], reserved),
/// background (str|None, default None=transparent).
pub fn build_bubble_map(title: &str, labels: Vec<String>, values: Vec<f64>, width: i32, height: i32, hover_json: &str, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { Vec::new() };
    Ok(Chart::new(crate::html::hover::apply_opts(crate::plot::map::render_bubble_map_html(title, &labels, &values, width, height, &hover), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, width=1200_i32, height=600_i32, hover_json="", series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Choropleth map (colored regions).
///
/// Args: title, labels (country names/codes), values.
///
/// Kwargs: width, height, hover_json, series_names (list[str], reserved),
/// background (str|None, default None=transparent).
pub fn build_choropleth(title: &str, labels: Vec<String>, values: Vec<f64>, width: i32, height: i32, hover_json: &str, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { Vec::new() };
    Ok(Chart::new(crate::html::hover::apply_opts(crate::plot::map::render_choropleth_html(title, &labels, &values, width, height, &hover), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, *, color_values=None, color_labels=None, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D scatter plot (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), series_names (list[str]), bg_color (str),
/// width, height, x_label, y_label, z_label, hover_json.
pub fn build_scatter3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, color_values: Option<Vec<f64>>, color_labels: Option<Vec<String>>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    Ok(Chart::new(crate::plot::default::render_scatter3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &cv, &cl, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, *, color_values=None, color_labels=None, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D bar chart (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), series_names (list[str]), bg_color (str),
/// width, height, x_label, y_label, z_label, hover_json.
pub fn build_bar3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, color_values: Option<Vec<f64>>, color_labels: Option<Vec<String>>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    Ok(Chart::new(crate::plot::default::render_bar3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &cv, &cl, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, *, color_values=None, color_labels=None, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D line chart (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), series_names (list[str]), bg_color (str),
/// width, height, x_label, y_label, z_label, hover_json.
pub fn build_line3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, color_values: Option<Vec<f64>>, color_labels: Option<Vec<String>>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    Ok(Chart::new(crate::plot::default::render_line3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &cv, &cl, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_values, *, series_names=None, filled=true, fill_opacity=50_i32, width=700_i32, height=560_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Radar/spider chart.
///
/// Args: title, axes, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]), filled (bool, default true), fill_opacity (int 0-255, default 50),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_radar_chart(title: &str, axes: Vec<String>, series_values: Vec<Vec<f64>>, series_names: Option<Vec<String>>, filled: bool, fill_opacity: i32, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{RadarConfig, render_radar_html};
    let names = series_names.unwrap_or_else(|| (0..series_values.len()).map(|_| String::new()).collect());
    let hover = o.hover();
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_values.into_iter()).collect();
    Ok(Chart::new(crate::html::hover::apply_opts(render_radar_html(&RadarConfig {
        title, axes: &axes, series: &series, palette: &o.pal, filled, fill_opacity: fill_opacity as u8, width: o.w, height: o.h,
        hover: &hover, ..RadarConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_values, *, series_names=None, width=700_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D radar chart (WebGL).
///
/// Args: title, axes, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]), width, height, x_label, y_label, z_label, hover_json.
pub fn build_radar3d_chart(title: &str, axes: Vec<String>, series_values: Vec<Vec<f64>>, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let n_axes = axes.len();
    if n_axes == 0 { return Ok(Chart::new(String::new())); }
    let n_series = series_values.len();
    let names: Vec<String> = series_names.unwrap_or_else(|| (0..n_series).map(|_| String::new()).collect());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let n_series = names.len().min(series_values.len());
    for si in 0..n_series {
        let vals = &series_values[si];
        let max_val = vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);
        for ai in 0..n_axes.min(vals.len()) {
            let angle = std::f64::consts::TAU * ai as f64 / n_axes as f64;
            let r = vals[ai] / max_val;
            xv.push(angle.cos() * r);
            yv.push(si as f64);
            zv.push(angle.sin() * r);
            cv.push(si as f64);
        }
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_radar3d_html(
        title, &xv, &yv, &zv,
        (x_label, y_label, z_label),
        &cv, &names, width, height, None,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, color_hex=0_u32, orientation="v", show_values=false, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Lollipop chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), orientation ("v"/"h"), show_values (bool),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_lollipop_chart(title: &str, labels: Vec<String>, values: Vec<f64>, color_hex: u32, orientation: &str, show_values: bool, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{LollipopConfig, render_lollipop_html};
    let orient = if orientation == "h" { b'h' } else { b'v' };
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_lollipop_html(&LollipopConfig {
        title, labels: &labels, values: &values, x_label: &o.xl, y_label: &o.yl,
        palette: &o.pal, color_hex, gridlines: o.grid, show_values, orientation: orient, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover, legend_position: &o.lp,
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, *, color_labels=None, series_names=None, width=900_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D lollipop chart (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_labels (list[str]), series_names (list[str]), width, height, x_label, y_label, z_label, hover_json.
pub fn build_lollipop3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, color_labels: Option<Vec<String>>, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let cl = color_labels.unwrap_or_default();
    Ok(Chart::new(crate::plot::statistical::_3d::render_lollipop3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &[], &cl, width, height, None,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, *, categories=None, filled=true, fill_opacity=50_i32, bandwidth=0.0_f64, width=900_i32, height=420_i32, x_label="", y_label="Density", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// KDE (kernel density estimation) plot.
///
/// Args: title, values.
///
/// Kwargs: categories (list[str] for multi-kde), filled (bool, default true), fill_opacity (int 0-255, default 50),
/// bandwidth (float, 0=auto),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_kde_chart(title: &str, values: Vec<f64>, categories: Option<Vec<String>>, filled: bool, fill_opacity: i32, bandwidth: f64, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{KdeConfig, render_kde_html};
    let series: Vec<(String, Vec<f64>)> = if let Some(cats) = categories {
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
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_kde_html(&KdeConfig {
        title, series: &series, palette: &o.pal, x_label: &o.xl, y_label: &o.yl,
        bandwidth, filled, fill_opacity: fill_opacity as u8, gridlines: o.grid, width: o.w, height: o.h,
        sort_order: &o.srt, hover: &hover, ..KdeConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, *, categories=None, series_names=None, width=900_i32, height=560_i32, x_label="", y_label="Category", z_label="Density", hover_json=""))]
/// 3D KDE chart (WebGL).
///
/// Args: title, values.
///
/// Kwargs: categories (list[str]), series_names (list[str]), width, height, x_label, y_label, z_label, hover_json.
pub fn build_kde3d_chart(title: &str, values: Vec<f64>, categories: Option<Vec<String>>, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let cats: Vec<String> = categories.unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = if cats.is_empty() {
            vec![("Series".to_string(), values)]
        } else {
            let mut group_order: Vec<String> = Vec::new();
            let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
            for (v, c) in values.iter().zip(cats.iter()) {
                group_vals.entry(c.clone()).or_default().push(*v);
                if !group_order.contains(c) { group_order.push(c.clone()); }
            }
            group_order.into_iter().map(|k| { let v = group_vals.remove(&k).unwrap_or_default(); (k, v) }).collect()
        };
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if all_vals.is_empty() { return Ok(Chart::new(String::new())); }
    let xmin = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad;
    let hi = xmax + pad;
    let n_pts: usize = 120;
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let names: Vec<String> = series.iter().map(|(n,_)| n.clone()).collect();
    for (si, (_name, vals)) in series.iter().enumerate() {
        let bw = scott_bw(vals);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(vals, t, bw);
            xv.push(t); yv.push(si as f64); zv.push(d); cv.push(si as f64);
        }
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_kde3d_html(
        title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &names, width, height, None,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, *, overlap=0.5_f64, bandwidth=0.0_f64, width=900_i32, height=520_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Ridgeline (joy) plot.
///
/// Args: title, values (flat), categories (one per value).
///
/// Kwargs: overlap (float, default 0.5), bandwidth (float, 0=auto),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_ridgeline_chart(title: &str, values: Vec<f64>, categories: Vec<String>, overlap: f64, bandwidth: f64, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_ridgeline_html(&RidgelineConfig {
        title, values: &values, categories: &categories, palette: &o.pal,
        x_label: &o.xl, y_label: &o.yl, overlap, bandwidth, width: o.w, height: o.h, gridlines: o.grid,
        sort_order: &o.srt, hover: &hover, ..RidgelineConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, *, series_names=None, width=900_i32, height=560_i32, x_label="", y_label="Category", z_label="Density", hover_json=""))]
/// 3D ridgeline chart (WebGL).
///
/// Args: title, values (flat), categories (one per value).
///
/// Kwargs: series_names (list[str]), width, height, x_label, y_label, z_label, hover_json.
pub fn build_ridgeline3d_chart(title: &str, values: Vec<f64>, categories: Vec<String>, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let mut group_order: Vec<String> = Vec::new();
    let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    for (v, c) in values.iter().zip(categories.iter()) {
        group_vals.entry(c.clone()).or_default().push(*v);
        if !group_order.contains(c) { group_order.push(c.clone()); }
    }
    if group_order.is_empty() { return Ok(Chart::new(String::new())); }
    let xmin = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad;
    let hi = xmax + pad;
    let n_pts: usize = 60;
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for (gi, name) in group_order.iter().enumerate() {
        let gvals = group_vals.get(name).map(|v| v.as_slice()).unwrap_or(&[]);
        let cap = 40usize;
        let step = if gvals.len() > cap { (gvals.len() + cap - 1) / cap } else { 1 };
        let sampled: Vec<f64> = if step > 1 { gvals.iter().step_by(step).copied().collect() } else { gvals.to_vec() };
        let bw = scott_bw(&sampled);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(&sampled, t, bw) * step as f64;
            xv.push(t); yv.push(gi as f64); zv.push(d); cv.push(gi as f64);
        }
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_ridgeline3d_html(
        title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &group_order, width, height, None,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, size_values, *, color_values=None, color_labels=None, series_names=None, width=900_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D bubble chart (WebGL) — variable-size spheres with realistic 3D shading.
///
/// Args: title, x_values, y_values, z_values, size_values (controls sphere radius).
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), series_names (list[str]),
/// width, height, x_label, y_label, z_label, hover_json.
pub fn build_bubble3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, size_values: Vec<f64>, color_values: Option<Vec<f64>>, color_labels: Option<Vec<String>>, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    let n = x_values.len().min(y_values.len()).min(z_values.len()).min(size_values.len());
    let smn = size_values[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let smx = size_values[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let sr = (smx - smn).max(1e-9);
    let size_js = format!("var S=[{}];",
        size_values[..n].iter().map(|&s| format!("{:.3}", (s - smn) / sr)).collect::<Vec<_>>().join(","));
    Ok(Chart::new(crate::html::js_3d::render_3d_html_impl(
        16, title, &x_values[..n], &y_values[..n], &z_values[..n],
        (x_label, y_label, z_label), &cv, &cl, width, height, None, size_js.as_bytes(),
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, series_names=None, bg_color="", sort_order="none", width=700_i32, height=560_i32, palette=None, hover_json=""))]
/// 3D pie chart (WebGL).
///
/// Args: title, labels, values.
///
/// Kwargs: series_names (list[str]), bg_color (str), sort_order (str), width, height, palette, hover_json.
pub fn build_pie3d_chart(title: &str, labels: Vec<String>, values: Vec<f64>, series_names: Option<Vec<String>>, bg_color: &str, sort_order: &str, width: i32, height: i32, palette: Option<Vec<u32>>, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let pal = palette.unwrap_or_default();
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    use crate::plot::statistical::common::apply_sort;
    let (labels, values) = apply_sort(&labels, &values, sort_order);
    let n = labels.len().min(values.len());
    let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let _ = pal;
    Ok(Chart::new(crate::plot::statistical::_3d::render_pie3d_html(
        title, &xv, &yv, &values[..n], ("", "", ""), &cv, &labels[..n].to_vec(), width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, *, categories=None, series_names=None, width=900_i32, height=560_i32, x_label="Value", y_label="Category", z_label="Density", hover_json=""))]
/// 3D violin chart (WebGL).
///
/// Args: title, values (flat).
///
/// Kwargs: categories (list[str]), series_names (list[str]), width, height, x_label, y_label, z_label, hover_json.
pub fn build_violin3d_chart(title: &str, values: Vec<f64>, categories: Option<Vec<String>>, series_names: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let xl = x_label;
    let yl = y_label;
    let zl = z_label;
    let cats: Vec<String> = categories.unwrap_or_default();
    let mut group_order: Vec<String> = Vec::new();
    let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    if cats.is_empty() {
        group_order.push("Series".to_string());
        group_vals.insert("Series".to_string(), values);
    } else {
        for (v, c) in values.iter().zip(cats.iter()) {
            group_vals.entry(c.clone()).or_default().push(*v);
            if !group_order.contains(c) { group_order.push(c.clone()); }
            }
        }
    let all: Vec<f64> = group_vals.values().flat_map(|v| v.iter().copied()).collect();
    if all.is_empty() { return Ok(Chart::new(String::new())); }
    let xmin = all.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad;
    let hi = xmax + pad;
    let n_pts: usize = 80;
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for (gi, name) in group_order.iter().enumerate() {
        let gv = group_vals.get(name).map(|v| v.as_slice()).unwrap_or(&[]);
        let bw = scott_bw(gv);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(gv, t, bw);
            xv.push(t); yv.push(gi as f64); zv.push(d); cv.push(gi as f64);
        }
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_violin3d_html(
        title, &xv, &yv, &zv, (xl, yl, zl), &cv, &group_order, width, height, None,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, y_labels, values, *, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="", y_label="", z_label="Z", hover_json=""))]
/// 3D heatmap (WebGL).
///
/// Args: title, x_labels, y_labels, values (list of list[float]).
///
/// Kwargs: series_names (list[str]), bg_color (str), width, height, x_label, y_label, z_label, hover_json.
pub fn build_heatmap3d_chart(title: &str, x_labels: Vec<String>, y_labels: Vec<String>, values: Vec<Vec<f64>>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    let nr = y_labels.len().min(values.len());
    let nc = x_labels.len();
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let mut cl = Vec::new();
    for r in 0..nr {
        let row = &values[r];
        for c2 in 0..nc.min(row.len()) {
            xv.push(c2 as f64); yv.push(r as f64); zv.push(row[c2]);
            cv.push(0.0); cl.push(format!("{}/{}", y_labels[r], x_labels[c2]));
        }
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_heatmap3d_html(
        title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &cl, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, open, high, low, close, *, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="Price", y_label="Bar", z_label="", hover_json=""))]
/// 3D candlestick chart (WebGL).
///
/// Args: title, labels, open, high, low, close.
///
/// Kwargs: series_names (list[str]), bg_color (str), width, height, x_label, y_label, z_label, hover_json.
pub fn build_candlestick3d_chart(title: &str, labels: Vec<String>, open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    let n = labels.len().min(open.len()).min(high.len()).min(low.len()).min(close.len());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    for i in 0..n {
        xv.push(open[i]); xv.push(high[i]); xv.push(low[i]); xv.push(close[i]);
        yv.push(i as f64); yv.push(i as f64); yv.push(i as f64); yv.push(i as f64);
        zv.push(0.0); zv.push(0.0); zv.push(0.0); zv.push(0.0);
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_candlestick3d_html(
        title, &xv, &yv, &zv, (x_label, y_label, z_label), &[], &labels, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_start, values_end, *, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="Start", y_label="Item", z_label="End", hover_json=""))]
/// 3D dumbbell chart (WebGL).
///
/// Args: title, labels, values_start, values_end.
///
/// Kwargs: series_names (tuple[str,str], default ("Start","End")), bg_color (str),
/// width, height, x_label, y_label, z_label, hover_json.
pub fn build_dumbbell3d_chart(title: &str, labels: Vec<String>, values_start: Vec<f64>, values_end: Vec<f64>, series_names: Option<(String, String)>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    let sl = series_names.unwrap_or(("Start".to_string(), "End".to_string()));
    let axis_names = (sl.0.as_str(), y_label, sl.1.as_str());
    let n = labels.len().min(values_start.len()).min(values_end.len());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for i in 0..n {
        xv.push(values_start[i]); yv.push(i as f64); zv.push(values_end[i]); cv.push(i as f64);
    }
    let _ = x_label;
    let _ = z_label;
    Ok(Chart::new(crate::plot::statistical::_3d::render_dumbbell3d_html(
        title, &xv, &yv, &zv, axis_names, &cv, &labels, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, *, series_names=None, bg_color="", sort_order="none", width=700_i32, height=560_i32, hover_json=""))]
/// 3D funnel chart (WebGL).
///
/// Args: title, labels, values.
///
/// Kwargs: series_names (list[str]), bg_color (str), sort_order (str), width, height, hover_json.
pub fn build_funnel3d_chart(title: &str, labels: Vec<String>, values: Vec<f64>, series_names: Option<Vec<String>>, bg_color: &str, sort_order: &str, width: i32, height: i32, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    use crate::plot::statistical::common::apply_sort;
    let (labels, values) = apply_sort(&labels, &values, sort_order);
    let n = labels.len().min(values.len());
    let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    Ok(Chart::new(crate::plot::statistical::_3d::render_funnel3d_html(
        title, &xv, &yv, &values[..n], ("", "Stage", "Value"), &cv, &labels[..n].to_vec(), width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, parents, values, *, series_names=None, bg_color="", width=700_i32, height=560_i32, hover_json=""))]
/// 3D sunburst chart (WebGL).
///
/// Args: title, labels, parents, values.
///
/// Kwargs: series_names (list[str]), bg_color (str), width, height, hover_json.
pub fn build_sunburst3d_chart(title: &str, labels: Vec<String>, parents: Vec<String>, values: Vec<f64>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    let n = labels.len().min(parents.len()).min(values.len());
    let mut ring_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    ring_map.insert(String::new(), 0);
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let mut cl = Vec::new();
    for i in 0..n {
        let parent = &parents[i];
        let parent_ring = ring_map.get(parent).copied().unwrap_or(0);
        let my_ring = parent_ring + 1;
        ring_map.insert(labels[i].clone(), my_ring);
        xv.push(i as f64); yv.push(my_ring as f64); zv.push(values[i]);
        cv.push(i as f64); cl.push(labels[i].clone());
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_sunburst3d_html(
        title, &xv, &yv, &zv, ("", "Ring", "Value"), &cv, &cl, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_values, *, series_names=None, bg_color="", width=900_i32, height=560_i32, x_label="Category", y_label="Series", z_label="Value", hover_json=""))]
/// 3D stacked bar chart (WebGL).
///
/// Args: title, category_labels, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]), bg_color (str), width, height, x_label, y_label, z_label, hover_json.
pub fn build_stacked_bar3d_chart(title: &str, category_labels: Vec<String>, series_values: Vec<Vec<f64>>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, x_label: &str, y_label: &str, z_label: &str, hover_json: &str) -> PyResult<Chart> {
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    let n_cat = category_labels.len();
    let n_ser = series_values.len();
    let names: Vec<String> = series_names.unwrap_or_else(|| (0..n_ser).map(|_| String::new()).collect());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let mut cl = Vec::new();
    for ci in 0..n_cat {
        for si in 0..n_ser {
            let v = series_values[si].get(ci).copied().unwrap_or(0.0);
            xv.push(ci as f64); yv.push(si as f64); zv.push(v);
            cv.push(si as f64); cl.push(format!("{}/{}", category_labels[ci], names[si]));
        }
    }
    Ok(Chart::new(crate::plot::statistical::_3d::render_stacked_bar3d_html(
        title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &names, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, latitudes, longitudes, values, *, labels=None, series_names=None, bg_color="", width=800_i32, height=600_i32, hover_json=""))]
/// 3D globe chart (WebGL).
///
/// Args: title, latitudes, longitudes, values.
///
/// Kwargs: labels (list[str]), series_names (list[str]), bg_color (str), width, height, hover_json.
pub fn build_globe3d_chart(title: &str, latitudes: Vec<f64>, longitudes: Vec<f64>, values: Vec<f64>, labels: Option<Vec<String>>, series_names: Option<Vec<String>>, bg_color: &str, width: i32, height: i32, hover_json: &str) -> PyResult<Chart> {
    let _ = series_names;
    let _hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { vec![] };
    let bg = if !bg_color.is_empty() { Some(bg_color) } else { None };
    let n = latitudes.len().min(longitudes.len()).min(values.len());
    let lbl = labels.unwrap_or_default();
    let cl = if lbl.is_empty() { (0..n).map(|i| format!("Point {}", i + 1)).collect() } else { lbl };
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    Ok(Chart::new(crate::plot::map::_3d::render_globe3d_html(
        title, &longitudes[..n], &latitudes[..n], &values[..n],
        ("Longitude", "Latitude", "Value"), &cv, &cl, width, height, bg,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, words, frequencies, *, min_font=12.0_f64, max_font=72.0_f64, background=None, width=900_i32, height=500_i32, sort_order="none", hover_json="", palette=None, series_names=None))]
/// Word cloud.
///
/// Args: title, words, frequencies.
///
/// Kwargs: min_font (float, default 12), max_font (float, default 72), background (str|None, default None=transparent),
/// width, height, palette, sort_order, hover_json, series_names (list[str], reserved).
pub fn build_wordcloud(title: &str, words: Vec<String>, frequencies: Vec<f64>, min_font: f64, max_font: f64, background: Option<&str>, width: i32, height: i32, sort_order: &str, hover_json: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>) -> PyResult<Chart> {
    let _ = series_names;
    let pal = palette.unwrap_or_default();
    let hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { Vec::new() };
    use crate::plot::statistical::wordcloud::{WordCloudConfig, render_wordcloud_html};
    Ok(Chart::new(render_wordcloud_html(&WordCloudConfig {
        title, words: &words, frequencies: &frequencies,
        palette: &pal, width, height, min_font, max_font,
        bg_color: background, sort_order, hover: &hover, ..WordCloudConfig::default()
    })))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, open, high, low, close, *, width=1100_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Candlestick (OHLC) chart.
///
/// Args: title, labels, open, high, low, close.
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_candlestick(title: &str, labels: Vec<String>, open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::candlestick::{CandlestickConfig, render_candlestick_html};
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_candlestick_html(&CandlestickConfig {
        title, labels: &labels, open: &open, high: &high, low: &low, close: &close,
        palette: &o.pal, width: o.w, height: o.h, x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
        sort_order: &o.srt, hover: &hover, ..CandlestickConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_start, values_end, *, series_names=None, width=1000_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Dumbbell chart (start-end comparison).
///
/// Args: title, labels, values_start, values_end.
///
/// Kwargs: series_names (tuple[str,str], default ("Start","End")),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_dumbbell(title: &str, labels: Vec<String>, values_start: Vec<f64>, values_end: Vec<f64>, series_names: Option<(String, String)>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::dumbbell::{DumbbellConfig, render_dumbbell_html};
    let sl = series_names.unwrap_or(("Start".to_string(), "End".to_string()));
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_dumbbell_html(&DumbbellConfig {
        title, labels: &labels, values_start: &values_start, values_end: &values_end,
        series_names: (&sl.0, &sl.1),
        palette: &o.pal, width: o.w, height: o.h, x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
        sort_order: &o.srt, hover: &hover, ..DumbbellConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, sizes, *, categories=None, width=900_i32, height=500_i32, x_label="", y_label="", gridlines=false, sort_order="none", hover_json="", legend_position="right", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Bubble chart.
///
/// Args: title, x_values, y_values, sizes.
///
/// Kwargs: categories (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position,
/// series_names (list[str], reserved), background (str|None, default None=transparent).
pub fn build_bubble(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, sizes: Vec<f64>, categories: Option<Vec<String>>, width: i32, height: i32, x_label: &str, y_label: &str, gridlines: bool, sort_order: &str, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let o = ChartOpts::from_params(width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position);
    use crate::plot::statistical::bubble::{BubbleConfig, render_bubble_html};
    let cs = categories.unwrap_or_default();
    let hover = o.hover();
    Ok(Chart::new(crate::html::hover::apply_opts(render_bubble_html(&BubbleConfig {
        title, x_values: &x_values, y_values: &y_values, sizes: &sizes,
        categories: &cs, palette: &o.pal, width: o.w, height: o.h, x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
        sort_order: &o.srt, hover: &hover, ..BubbleConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, value, *, min_val=0.0_f64, max_val=100.0_f64, label="", width=400_i32, height=300_i32, hover_json="", palette=None, series_names=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Gauge chart.
///
/// Args: title, value (float).
///
/// Kwargs: min_val (float, default 0), max_val (float, default 100), label (str),
/// width, height, palette, hover_json, series_names (list[str], reserved),
/// background (str|None, default None=transparent).
pub fn build_gauge(title: &str, value: f64, min_val: f64, max_val: f64, label: &str, width: i32, height: i32, hover_json: &str, palette: Option<Vec<u32>>, series_names: Option<Vec<String>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let _ = series_names;
    let _pal = palette.unwrap_or_default();
    let hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { Vec::new() };
    use crate::plot::statistical::gauge::{GaugeConfig, render_gauge_html};
    Ok(Chart::new(crate::html::hover::apply_opts(render_gauge_html(&GaugeConfig {
        title, value, min_val, max_val, label, width, height,
        hover: &hover, ..GaugeConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_values, *, series_names=None, width=1000_i32, height=500_i32, hover_json="", legend_position="right", palette=None, background=None, no_x_axis=false, no_y_axis=false))]
/// Parallel coordinates chart.
///
/// Args: title, axes, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]),
/// width, height, palette, hover_json, legend_position,
/// background (str|None, default None=transparent).
pub fn build_parallel(title: &str, axes: Vec<String>, series_values: Vec<Vec<f64>>, series_names: Option<Vec<String>>, width: i32, height: i32, hover_json: &str, legend_position: &str, palette: Option<Vec<u32>>, background: Option<&str>, no_x_axis: bool, no_y_axis: bool) -> PyResult<Chart> {
    let pal = palette.unwrap_or_default();
    let hover = if !hover_json.is_empty() { crate::plot::statistical::parse_hover_json(hover_json) } else { Vec::new() };
    use crate::plot::statistical::parallel::{ParallelConfig, render_parallel_html};
    let names: Vec<String> = series_names.unwrap_or_else(|| (0..series_values.len()).map(|_| String::new()).collect());
    Ok(Chart::new(crate::html::hover::apply_opts(render_parallel_html(&ParallelConfig {
        title, axes: &axes, series_names: &names, series_values: &series_values,
        palette: &pal, width, height,
        hover: &hover, ..ParallelConfig::default()
    }), background, !no_x_axis, !no_y_axis)))
}


#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (charts, cols=2, gap=16, bg=None, cell_height=520))]
/// Grid layout combining multiple charts.
///
/// Args: charts (list[Chart]), cols (int, default 2), gap (int, default 16),
/// bg (str, default "#f0f2f5"), cell_height (int, default 520).
pub fn build_grid(
    charts: Vec<PyRef<Chart>>,
    cols: usize,
    gap: i32,
    bg: Option<&str>,
    cell_height: i32,
) -> Chart {
    let bg_color = bg.unwrap_or("#f0f2f5");
    let cols = cols.max(1);
    let mut buf = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        *{{box-sizing:border-box}}body{{margin:0;padding:{gap}px;background:{bg_color}}}\
        .spg{{display:grid;grid-template-columns:repeat({cols},1fr);gap:{gap}px}}\
        .spg-c iframe{{width:100%;height:{cell_height}px;border:none;display:block;\
        border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07)}}\
        </style></head><body><div class=\"spg\">"
    );
    for c in charts.iter() {
        let esc = c.html.replace('&', "&amp;").replace('"', "&quot;");
        buf.push_str("<div class=\"spg-c\"><iframe srcdoc=\"");
        buf.push_str(&esc);
        buf.push_str("\"></iframe></div>");
    }
    buf.push_str("</div></body></html>");
    Chart::new(buf)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (charts, interval_ms=2500, title="", width=900, height=520))]
/// Slideshow cycling through charts.
///
/// Args: charts (list[Chart]), interval_ms (int, default 2500), title (str),
/// width (int, default 900), height (int, default 520).
pub fn build_slideshow(
    charts: Vec<PyRef<Chart>>,
    interval_ms: u32,
    title: &str,
    width: i32,
    height: i32,
) -> Chart {
    if charts.is_empty() { return Chart::new(String::new()); }
    let svgs: Vec<String> = charts.iter().map(|c| {
        let h = &c.html;
        let start = h.find("<svg").unwrap_or(0);
        let end = h.rfind("</svg>").map(|i| i + 6).unwrap_or(h.len());
        if start < end { h[start..end].to_string() }
        else { "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100\" height=\"100\"></svg>".to_string() }
    }).collect();
    let json_array = format!("[{}]", svgs.iter()
        .map(|s| serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string()))
        .collect::<Vec<_>>().join(","));
    let n = charts.len();
    let ivms = interval_ms;
    let title_html = if title.is_empty() { String::new() } else {
        format!("<div style=\"color:#1e293b;font-family:system-ui;font-size:22px;font-weight:700;text-align:center;margin-bottom:16px\">{}</div>", title)
    };
    Chart::new(format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        body{{margin:0;padding:24px;background:#f0f2f5;display:flex;flex-direction:column;align-items:center;font-family:system-ui}}\
        .sp-frm{{border-radius:12px;overflow:hidden;box-shadow:0 2px 12px rgba(0,0,0,.1);background:#fff}}\
        .sp-frm svg{{width:{width}px;height:{height}px;display:block}}\
        .sp-ctrl{{display:flex;gap:10px;margin-top:14px;align-items:center}}\
        .sp-btn{{cursor:pointer;background:#6366f1;color:#fff;border:none;border-radius:8px;padding:7px 20px;font-size:14px;font-weight:600}}\
        .sp-btn:hover{{background:#4f46e5}}\
        .sp-ctr{{color:#64748b;font-size:13px;min-width:64px;text-align:center}}\
        .sp-prog{{width:{width}px;height:4px;background:#e2e8f0;border-radius:2px;margin-top:10px;overflow:hidden}}\
        .sp-bar{{height:100%;background:#6366f1;border-radius:2px;width:0%}}\
        </style></head><body>\
        {title_html}\
        <div class=\"sp-frm\" id=\"sp-frm\"></div>\
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
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
/// Build hover tooltip JSON. Pass result as hover_json kwarg to any chart.
///
/// Args: labels (list[str]), images (list[str|None], optional),
/// descriptions (list[str|list[str]|None], optional).
pub fn build_hover_json(
    labels: Vec<String>,
    images: Option<Vec<Option<String>>>,
    descriptions: Option<&PyAny>,
) -> PyResult<String> {
    use crate::html::hover::{HoverSlot, slots_to_json};
    let mut slots = Vec::new();
    let n = labels.len();
    
    for i in 0..n {
        let mut slot = HoverSlot::new(&labels.get(i).cloned().unwrap_or_default());
        
        if let Some(ref imgs) = images {
            if i < imgs.len() {
                if let Some(ref img_url) = imgs[i] {
                    slot = slot.image(img_url.clone());
                }
            }
        }
        
        if let Some(descs_py) = descriptions {
            if let Ok(descs_list) = descs_py.extract::<Vec<Vec<(String, String)>>>() {
                if i < descs_list.len() {
                    for (k, v) in &descs_list[i] {
                        slot = slot.kv(k.clone(), v.clone());
                    }
                }
            }
        }
        
        slots.push(slot);
    }
    
    Ok(slots_to_json(&slots))
}