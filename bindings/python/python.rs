#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use pyo3::types::{PyDict, PyList};

#[cfg(feature = "python")]
fn parse_palette(palette: Option<Vec<u32>>) -> Vec<u32> {
    palette.unwrap_or_default()
}

#[cfg(feature = "python")]
fn kw_i32(kw: Option<&PyDict>, key: &str, d: i32) -> i32 {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or(d)
}
#[cfg(feature = "python")]
fn kw_u32(kw: Option<&PyDict>, key: &str, d: u32) -> u32 {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or(d)
}
#[cfg(feature = "python")]
fn kw_u8(kw: Option<&PyDict>, key: &str, d: u8) -> u8 {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or(d)
}
#[cfg(feature = "python")]
fn kw_f64(kw: Option<&PyDict>, key: &str, d: f64) -> f64 {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or(d)
}
#[cfg(feature = "python")]
fn kw_bool(kw: Option<&PyDict>, key: &str, d: bool) -> bool {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or(d)
}
#[cfg(feature = "python")]
fn kw_str(kw: Option<&PyDict>, key: &str, d: &str) -> String {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract::<String>().ok()).unwrap_or_else(|| d.to_string())
}
#[cfg(feature = "python")]
fn kw_opt_str(kw: Option<&PyDict>, key: &str) -> Option<String> {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract().ok())
}
#[cfg(feature = "python")]
fn kw_pal(kw: Option<&PyDict>) -> Vec<u32> {
    kw.and_then(|k| k.get_item("palette").ok().flatten()).and_then(|v| v.extract::<Vec<u32>>().ok()).unwrap_or_default()
}
#[cfg(feature = "python")]
fn kw_vec_str(kw: Option<&PyDict>, key: &str) -> Vec<String> {
    kw.and_then(|k| k.get_item(key).ok().flatten()).and_then(|v| v.extract::<Vec<String>>().ok()).unwrap_or_default()
}
#[cfg(feature = "python")]
fn kw_bg(kw: Option<&PyDict>, html: String) -> String {
    match kw_opt_str(kw, "bg_color") {
        Some(c) => crate::html::hover::apply_bg(html, Some(&c)),
        None => html,
    }
}

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
    fn new(kw: Option<&PyDict>, dw: i32, dh: i32) -> Self {
        Self {
            w: kw_i32(kw, "width", dw),
            h: kw_i32(kw, "height", dh),
            pal: kw_pal(kw),
            xl: kw_str(kw, "x_label", ""),
            yl: kw_str(kw, "y_label", ""),
            zl: kw_str(kw, "z_label", "Z"),
            grid: kw_bool(kw, "gridlines", false),
            srt: kw_str(kw, "sort_order", "none"),
            lp: kw_str(kw, "legend_position", "right"),
            hj: kw_opt_str(kw, "hover_json"),
        }
    }
    fn hover(&self) -> Vec<crate::html::hover::HoverSlot> {
        self.hj.as_ref().map(|s| crate::plot::statistical::parse_hover_json(s)).unwrap_or_default()
    }
}

#[cfg(feature = "python")]
fn coerce_pyany<'a>(data: &'a PyAny) -> &'a PyAny {
    use pyo3::types::PyList;
    if data.downcast::<PyList>().is_ok() { return data; }
    if let Ok(lst) = data.call_method0("tolist") { return lst; }
    data
}

#[cfg(feature = "python")]
fn fast_f64(data: &PyAny, cap: usize) -> PyResult<(Vec<f64>, usize)> {
    let data = coerce_pyany(data);
    use pyo3::types::PyList;
    if let Ok(lst) = data.downcast::<PyList>() {
        let n = lst.len();
        if n <= cap {
            Ok((data.extract::<Vec<f64>>()?, 1))
        } else {
            let s = (n + cap - 1) / cap;
            let mut v = Vec::with_capacity(cap + 1);
            let mut i = 0;
            while i < n { v.push(lst.get_item(i)?.extract::<f64>()?); i += s; }
            Ok((v, s))
        }
    } else {
        let v: Vec<f64> = data.extract()?;
        let n = v.len();
        if n <= cap {
            Ok((v, 1))
        } else {
            let s = (n + cap - 1) / cap;
            Ok((v.into_iter().step_by(s).collect(), s))
        }
    }
}

#[cfg(feature = "python")]
fn fast_labels(labels: Vec<String>, step: usize) -> Vec<String> {
    if step <= 1 || labels.is_empty() { labels }
    else { labels.into_iter().step_by(step).collect() }
}

#[cfg(feature = "python")]
fn fast_labels_py(data: &PyAny, cap: usize) -> PyResult<(Vec<String>, usize)> {
    let data = coerce_pyany(data);
    use pyo3::types::PyList;
    if let Ok(lst) = data.downcast::<PyList>() {
        let n = lst.len();
        if n <= cap {
            Ok((data.extract::<Vec<String>>()?, 1))
        } else {
            let s = (n + cap - 1) / cap;
            let mut v = Vec::with_capacity(cap + 1);
            let mut i = 0;
            while i < n { v.push(lst.get_item(i)?.extract::<String>()?); i += s; }
            Ok((v, s))
        }
    } else {
        let v: Vec<String> = data.extract()?;
        let n = v.len();
        if n <= cap {
            Ok((v, 1))
        } else {
            let s = (n + cap - 1) / cap;
            Ok((v.into_iter().step_by(s).collect(), s))
        }
    }
}

#[cfg(feature = "python")]
fn fast_vecs(names: Vec<String>, vecs: Vec<Vec<f64>>, cap: usize) -> (Vec<String>, Vec<Vec<f64>>, usize) {
    if vecs.is_empty() { return (names, vecs, 1); }
    let max_len = vecs.iter().map(|v| v.len()).max().unwrap_or(0);
    if max_len <= cap { return (names, vecs, 1); }
    let s = (max_len + cap - 1) / cap;
    let vs: Vec<Vec<f64>> = vecs.into_iter().map(|v| v.into_iter().step_by(s).collect()).collect();
    (names, vs, s)
}

#[cfg(feature = "python")]
fn fast_vecs_py(names: Vec<String>, data: &PyAny, cap: usize) -> PyResult<(Vec<String>, Vec<Vec<f64>>, usize)> {
    use pyo3::types::PyList;
    if let Ok(outer) = data.downcast::<PyList>() {
        let n_ser = outer.len();
        if n_ser == 0 { return Ok((names, Vec::new(), 1)); }
        let first_len = if let Ok(inner) = outer.get_item(0)?.downcast::<PyList>() { inner.len() } else { 0 };
        if first_len <= cap {
            let mut vecs = Vec::with_capacity(n_ser);
            for si in 0..n_ser { vecs.push(outer.get_item(si)?.extract::<Vec<f64>>()?); }
            return Ok((names, vecs, 1));
        }
        let s = (first_len + cap - 1) / cap;
        let mut vecs = Vec::with_capacity(n_ser);
        for si in 0..n_ser {
            let inner = outer.get_item(si)?;
            if let Ok(lst) = inner.downcast::<PyList>() {
                let n = lst.len();
                let mut v = Vec::with_capacity(cap + 1);
                let mut i = 0;
                while i < n { v.push(lst.get_item(i)?.extract::<f64>()?); i += s; }
                vecs.push(v);
            } else {
                let full: Vec<f64> = inner.extract()?;
                vecs.push(full.into_iter().step_by(s).collect());
            }
        }
        Ok((names, vecs, s))
    } else {
        let vecs: Vec<Vec<f64>> = data.extract()?;
        let (names, vecs, s) = fast_vecs(names, vecs, cap);
        Ok((names, vecs, s))
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
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Bar chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), orientation ("v"/"h"), show_text (bool), color_groups (list[str]),
/// bg_color (str), width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_bar_chart(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 480);

        let clr = kw_u32(kwargs, "color_hex", 0);
        let orient = if kw_str(kwargs, "orientation", "v") == "h" { b'h' } else { b'v' };
        let show_text = kw_bool(kwargs, "show_text", false);
        let groups = kw_vec_str(kwargs, "color_groups");
        let hover = o.hover();
        let html = crate::plot::default::render_bars_html(
            title, &labels, &values, o.w, o.h, &hover, orient, &groups, show_text, &o.xl, &o.yl, &o.pal, clr, o.grid, &o.srt,
        );
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Horizontal bar chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), show_text (bool, default true), color_groups (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_hbar(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 500);

        let clr = kw_u32(kwargs, "color_hex", 0);
        let show_text = kw_bool(kwargs, "show_text", true);
        let groups = kw_vec_str(kwargs, "color_groups");
        let hover = o.hover();
        let html = crate::plot::default::render_bars_html(
            title, &labels, &values, o.w, o.h, &hover, b'h', &groups, show_text, &o.xl, "", &o.pal, clr, o.grid, &o.srt,
        );
        Ok(Chart::new(html))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Line chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), show_points (bool, default true), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_line_chart(title: &str, labels: &PyAny, values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 480);

        let clr = kw_u32(kwargs, "color_hex", 0x6366F1);
        let show_points = kw_bool(kwargs, "show_points", true);
        let (vals, _) = fast_f64(values, 100)?;
        let (lbls, _) = fast_labels_py(labels, 100)?;
        let hover = o.hover();
        let html = crate::plot::default::render_lines_html(
            title, &lbls, &vals, o.w, o.h, &hover, clr, &o.xl, &o.yl, o.grid, show_points, &o.srt,
        );
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Scatter plot.
///
/// Args: title, x_values, y_values.
///
/// Kwargs: color_hex (int), show_text (bool), labels (list[str]), sizes (list[float]),
/// color_groups (list[str]), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_scatter_chart(title: &str, x_values: &PyAny, y_values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 540);

        let clr = kw_u32(kwargs, "color_hex", 0);
        let show_text = kw_bool(kwargs, "show_text", false);
        let hover = o.hover();
        let lbl: Vec<String> = kwargs.and_then(|k| k.get_item("labels").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let sizes: Vec<f64> = kwargs.and_then(|k| k.get_item("sizes").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let cg: Vec<String> = kw_vec_str(kwargs, "color_groups");
        let (x, sx) = fast_f64(x_values, 200)?;
        let (y, sy) = fast_f64(y_values, 200)?;
        let step = sx.max(sy);
        let lbls = fast_labels(lbl, step);
        let sz: Vec<f64> = if step > 1 { sizes.into_iter().step_by(step).collect() } else { sizes };
        let cgs: Vec<String> = if step > 1 { cg.into_iter().step_by(step).collect() } else { cg };
        let html = crate::plot::default::render_scatter_html(
            title, &x, &y, &lbls, o.w, o.h, &hover, &sz, &cgs, &o.pal, &o.xl, &o.yl, clr, o.grid, show_text,
        );
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Histogram.
///
/// Args: title, values.
///
/// Kwargs: color_hex (int), bins (int, 0=auto), show_counts (bool), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_histogram(title: &str, values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 860, 380);

        use crate::plot::statistical::{HistogramConfig, render_histogram_html};
        let clr = kw_u32(kwargs, "color_hex", 0x6366F1);
        let yl = kw_str(kwargs, "y_label", "Count");
        let show_counts = kw_bool(kwargs, "show_counts", false);
        let bins = kw_i32(kwargs, "bins", 0) as usize;
        let hover = o.hover();
        let (vals, scale) = fast_f64(values, 1000)?;
        let html = render_histogram_html(&HistogramConfig {
            title, values: &vals, bins, color: clr, x_label: &o.xl, y_label: &o.yl,
            show_counts, gridlines: o.grid, width: o.w, height: o.h, hover: &hover, count_scale: scale,
            sort_order: &o.srt, ..HistogramConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, overlay_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Overlaid histogram (two distributions).
///
/// Args: title, values, overlay_values.
///
/// Kwargs: color_hex (int), overlay_color_hex (int), bins (int, 0=auto),
/// series_names (list[str] of 2 names), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_histogram_overlay(title: &str, values: Vec<f64>, overlay_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 860, 380);

        use crate::plot::statistical::{HistogramConfig, render_histogram_html};
        let clr = kw_u32(kwargs, "color_hex", 0x6366F1);
        let overlay_clr = kw_u32(kwargs, "overlay_color_hex", 0xF43F5E);
        let bins = kw_i32(kwargs, "bins", 0) as usize;
        let labels: Option<Vec<String>> = kwargs.and_then(|k| k.get_item("series_names").ok().flatten()).and_then(|v| v.extract().ok());
        let hover = o.hover();
        let names: Option<(String, String)> = labels.and_then(|v| {
            let a = v.get(0).cloned().unwrap_or_default();
            let b = v.get(1).cloned().unwrap_or_default();
            if a.is_empty() && b.is_empty() { None } else { Some((a, b)) }
        });
        let html = render_histogram_html(&HistogramConfig {
            title, values: &values, bins, color: clr, overlay_color: overlay_clr,
            overlay_values: Some(&overlay_values), width: o.w, height: o.h, hover: &hover,
            series_names: names.as_ref().map(|(a, b)| (a.as_str(), b.as_str())),
            x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
            sort_order: &o.srt, ..HistogramConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Grouped bar chart.
///
/// Args: title, category_labels, series_values (flat, num_series inferred from series_names).
///
/// Kwargs: show_values (bool), series_names (list[str]), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_grouped_bar(title: &str, category_labels: Vec<String>, series_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1100, 480);

        use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
        let show_values = kw_bool(kwargs, "show_values", false);
        let hover = o.hover();
        let sn = kw_vec_str(kwargs, "series_names");
        let n_cats = category_labels.len();
        let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_values.len() + n_cats - 1) / n_cats } else { 0 };
        let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
        let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
            let vals: Vec<f64> = (0..n_cats).map(|ci| {
                series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)
            }).collect();
            (name.clone(), vals)
        }).collect();
        let html = render_grouped_bar_html(&GroupedBarConfig {
            title, category_labels: &category_labels, series: &series,
            palette: &o.pal, x_label: &o.xl, y_label: &o.yl, show_values, gridlines: o.grid, sort_order: &o.srt,
            hover: &hover, ..GroupedBarConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Stacked bar chart.
///
/// Args: title, category_labels, series_values (flat, num_series inferred from series_names).
///
/// Kwargs: show_values (bool), series_names (list[str]), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_stacked_bar(title: &str, category_labels: Vec<String>, series_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1100, 480);

        use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
        let show_values = kw_bool(kwargs, "show_values", false);
        let hover = o.hover();
        let sn = kw_vec_str(kwargs, "series_names");
        let n_cats = category_labels.len();
        let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_values.len() + n_cats - 1) / n_cats } else { 0 };
        let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
        let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
            let vals: Vec<f64> = (0..n_cats).map(|ci| {
                series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)
            }).collect();
            (name.clone(), vals)
        }).collect();
        let html = render_grouped_bar_html(&GroupedBarConfig {
            title, category_labels: &category_labels, series: &series,
            palette: &o.pal, x_label: &o.xl, y_label: &o.yl, show_values, gridlines: o.grid, sort_order: &o.srt,
            hover: &hover, stacked: true, ..GroupedBarConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, flat_matrix, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Heatmap.
///
/// Args: title, labels (row labels), flat_matrix (row-major flat values).
///
/// Kwargs: show_values (bool, default true), color_low/color_mid/color_high (int),
/// col_labels (list[str]), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_heatmap(title: &str, labels: Vec<String>, flat_matrix: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 720, 440);

        use crate::plot::statistical::{HeatmapConfig, render_heatmap_html};
        let show_values = kw_bool(kwargs, "show_values", true);
        let color_low = kw_u32(kwargs, "color_low", 0x6366F1);
        let color_mid = kw_u32(kwargs, "color_mid", 0xfafbfc);
        let color_high = kw_u32(kwargs, "color_high", 0xF43F5E);
        let col_labels = kw_vec_str(kwargs, "col_labels");
        let hover = o.hover();
        let html = render_heatmap_html(&HeatmapConfig {
            title, row_labels: &labels, col_labels: &col_labels, flat_matrix: &flat_matrix,
            show_values, color_low, color_mid, color_high, width: o.w, height: o.h, hover: &hover,
            sort_order: &o.srt, ..HeatmapConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Pie chart.
///
/// Args: title, labels, values.
///
/// Kwargs: show_pct (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_pie_chart(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 720, 440);

        use crate::plot::statistical::{PieConfig, render_pie_html};
        let show_pct = kw_bool(kwargs, "show_pct", true);
        let hover = o.hover();
        Ok(Chart::new(render_pie_html(&PieConfig {
            title, labels: &labels, values: &values, palette: &o.pal,
            show_pct, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover,
            ..PieConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Donut chart.
///
/// Args: title, labels, values.
///
/// Kwargs: show_pct (bool, default true), inner_radius_ratio (float, default 0.55),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_donut_chart(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 720, 440);

        use crate::plot::statistical::{PieConfig, render_pie_html};
        let show_pct = kw_bool(kwargs, "show_pct", true);
        let inner = kw_f64(kwargs, "inner_radius_ratio", 0.55);
        let hover = o.hover();
        Ok(Chart::new(render_pie_html(&PieConfig {
            title, labels: &labels, values: &values, palette: &o.pal,
            show_pct, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover,
            donut: inner.clamp(0.0, 0.9),
            ..PieConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Box plot.
///
/// Args: title, category_labels, values (flat, auto-split by categories).
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_boxplot(title: &str, category_labels: &PyAny, values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 500);

        let hover = o.hover();
        let (v, _) = fast_f64(values, 100)?;
        let (cl, _) = fast_labels_py(category_labels, 100)?;
        Ok(Chart::new(crate::plot::statistical::render_boxplot_html(&crate::plot::statistical::BoxplotConfig {
            title, category_labels: &cl, values: &v, palette: &o.pal, hover: &hover,
            x_label: &o.xl, y_label: &o.yl, gridlines: o.grid, width: o.w, height: o.h,
            sort_order: &o.srt, legend_position: &o.lp,
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, categories, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Violin plot.
///
/// Args: title, categories, values (flat, auto-split by categories).
///
/// Kwargs: bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_violin(title: &str, categories: &PyAny, values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 500);

        use crate::plot::statistical::{ViolinConfig, render_violin_html};
        let hover = o.hover();
        let (v, _) = fast_f64(values, 50)?;
        let (cats, _) = fast_labels_py(categories, 50)?;
        let html = render_violin_html(&ViolinConfig {
            title, categories: &cats, values: &v,
            x_label: &o.xl, y_label: &o.yl, palette: &o.pal, gridlines: o.grid, width: o.w, height: o.h,
            sort_order: &o.srt, hover: &hover, legend_position: &o.lp,
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_left, values_right, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Slope chart (before/after comparison).
///
/// Args: title, labels, values_left, values_right.
///
/// Kwargs: left_label (str, default "Before"), right_label (str, default "After"),
/// show_text (bool, default true), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_slope(title: &str, labels: Vec<String>, values_left: Vec<f64>, values_right: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 500);

        use crate::plot::statistical::{SlopeConfig, render_slope_html};
        let left_label = kw_str(kwargs, "left_label", "Before");
        let right_label = kw_str(kwargs, "right_label", "After");
        let show_text = kw_bool(kwargs, "show_text", true);
        let hover = o.hover();
        let html = render_slope_html(&SlopeConfig {
            title, labels: &labels, values_left: &values_left, values_right: &values_right,
            left_label: &left_label, right_label: &right_label, palette: &o.pal, show_text, width: o.w, height: o.h,
            sort_order: &o.srt, hover: &hover, ..SlopeConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, parents, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Sunburst chart (hierarchical).
///
/// Args: title, labels, parents (parent label per node), values.
///
/// Kwargs: bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_sunburst(title: &str, labels: Vec<String>, parents: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 700);

        use crate::plot::statistical::{SunburstConfig, render_sunburst_html};
        let hover = o.hover();
        let html = render_sunburst_html(&SunburstConfig {
            title, labels: &labels, parents: &parents, values: &values, width: o.w, height: o.h,
            hover: &hover, ..SunburstConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Funnel chart.
///
/// Args: title, labels, values.
///
/// Kwargs: show_text (bool, default true), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_funnel(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 800, 480);

        use crate::plot::statistical::{FunnelConfig, render_funnel_html};
        let show_text = kw_bool(kwargs, "show_text", true);
        let hover = o.hover();
        let html = render_funnel_html(&FunnelConfig {
            title, labels: &labels, values: &values, palette: &o.pal, show_text, width: o.w, height: o.h,
            sort_order: &o.srt, hover: &hover, ..FunnelConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Treemap.
///
/// Args: title, labels, values.
///
/// Kwargs: parents (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_treemap(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1100, 520);

        use crate::plot::statistical::{TreemapConfig, render_treemap_html};
        let pars = kw_vec_str(kwargs, "parents");
        let hover = o.hover();
        Ok(Chart::new(render_treemap_html(&TreemapConfig {
            title, labels: &labels, values: &values, parents: &pars,
            palette: &o.pal, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover,
            ..TreemapConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Multi-line chart (multiple series).
///
/// Args: title, x_labels, series_values (list of list[float]).
///
/// Kwargs: show_points (bool, default true), series_names (list[str]), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_multiline_chart(title: &str, x_labels: &PyAny, series_values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1100, 480);

        use crate::plot::statistical::{MultiLineConfig, render_multiline_html};
        let show_points = kw_bool(kwargs, "show_points", true);
        let hover = o.hover();
        let sn = kw_vec_str(kwargs, "series_names");
        let (names, vecs, _) = fast_vecs_py(sn, series_values, 60)?;
        let (xlabels, _) = fast_labels_py(x_labels, 60)?;
        let names: Vec<String> = if names.is_empty() { (0..vecs.len()).map(|_| String::new()).collect() } else { names };
        let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(vecs.into_iter()).collect();
        let html = render_multiline_html(&MultiLineConfig {
            title, x_labels: &xlabels, series: &series, palette: &o.pal,
            x_label: &o.xl, y_label: &o.yl, show_points, gridlines: o.grid, width: o.w, height: o.h, hover: &hover,
            sort_order: &o.srt, ..MultiLineConfig::default()
        });
        Ok(Chart::new(kw_bg(kwargs, html)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Area chart.
///
/// Args: title, x_labels, series_values (list of list[float]).
///
/// Kwargs: stacked (bool), series_names (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_area_chart(title: &str, x_labels: &PyAny, series_values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1100, 480);

        use crate::plot::statistical::{AreaConfig, render_area_html};
        let stacked = kw_bool(kwargs, "stacked", false);
        let hover = o.hover();
        let sn = kw_vec_str(kwargs, "series_names");
        let (names, vecs, _) = fast_vecs_py(sn, series_values, 60)?;
        let (xlabels, _) = fast_labels_py(x_labels, 60)?;
        let names: Vec<String> = if names.is_empty() { (0..vecs.len()).map(|_| String::new()).collect() } else { names };
        let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(vecs.into_iter()).collect();
        Ok(Chart::new(render_area_html(&AreaConfig {
            title, x_labels: &xlabels, series: &series, stacked, palette: &o.pal,
            x_label: &o.xl, y_label: &o.yl, gridlines: o.grid, width: o.w, height: o.h, hover: &hover,
            sort_order: &o.srt, ..AreaConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Waterfall chart.
///
/// Args: title, labels, values (positive=up, negative=down).
///
/// Kwargs: show_text (bool, default true),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_waterfall(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 480);

        use crate::plot::statistical::{WaterfallConfig, render_waterfall_html};
        let show_text = kw_bool(kwargs, "show_text", true);
        let hover = o.hover();
        Ok(Chart::new(render_waterfall_html(&WaterfallConfig {
            title, labels: &labels, values: &values, x_label: &o.xl, y_label: &o.yl, show_text, gridlines: o.grid, width: o.w, height: o.h,
            sort_order: &o.srt, hover: &hover, legend_position: &o.lp,
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Bullet chart.
///
/// Args: title, labels, values.
///
/// Kwargs: targets (list[float]), max_vals (list[float]), ranges (list[float]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_bullet(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 800, 300);

        use crate::plot::statistical::{BulletConfig, render_bullet_html};
        let targets: Vec<f64> = kwargs.and_then(|k| k.get_item("targets").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let max_vals: Vec<f64> = kwargs.and_then(|k| k.get_item("max_vals").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let ranges: Vec<f64> = kwargs.and_then(|k| k.get_item("ranges").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let hover = o.hover();
        Ok(Chart::new(render_bullet_html(&BulletConfig {
            title, labels: &labels, values: &values, targets: &targets, max_vals: &max_vals, ranges: &ranges, width: o.w, height: o.h,
            hover: &hover, ..BulletConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Bubble map (world map with sized bubbles).
///
/// Args: title, labels (country names/codes), values.
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_bubble_map(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1200, 600);

        let hover = o.hover();
        Ok(Chart::new(crate::plot::map::render_bubble_map_html(title, &labels, &values, o.w, o.h, &hover)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Choropleth map (colored regions).
///
/// Args: title, labels (country names/codes), values.
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_choropleth(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1200, 600);

        let hover = o.hover();
        Ok(Chart::new(crate::plot::map::render_choropleth_html(title, &labels, &values, o.w, o.h, &hover)))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D scatter plot (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), bg_color (str),
/// width, height, x_label, y_label, z_label.
pub fn build_scatter3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let cv: Vec<f64> = kwargs.and_then(|k| k.get_item("color_values").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let cl = kw_vec_str(kwargs, "color_labels");
        let bg = kw_opt_str(kwargs, "bg_color");
        Ok(Chart::new(crate::plot::default::render_scatter3d_html(
            title, &x_values, &y_values, &z_values,
            (&o.xl, &o.yl, &o.zl), &cv, &cl, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D bar chart (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), bg_color (str),
/// width, height, x_label, y_label, z_label.
pub fn build_bar3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let cv: Vec<f64> = kwargs.and_then(|k| k.get_item("color_values").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let cl = kw_vec_str(kwargs, "color_labels");
        let bg = kw_opt_str(kwargs, "bg_color");
        Ok(Chart::new(crate::plot::default::render_bar3d_html(
            title, &x_values, &y_values, &z_values,
            (&o.xl, &o.yl, &o.zl), &cv, &cl, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D line chart (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_values (list[float]), color_labels (list[str]), bg_color (str),
/// width, height, x_label, y_label, z_label.
pub fn build_line3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let cv: Vec<f64> = kwargs.and_then(|k| k.get_item("color_values").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let cl = kw_vec_str(kwargs, "color_labels");
        let bg = kw_opt_str(kwargs, "bg_color");
        Ok(Chart::new(crate::plot::default::render_line3d_html(
            title, &x_values, &y_values, &z_values,
            (&o.xl, &o.yl, &o.zl), &cv, &cl, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Radar/spider chart.
///
/// Args: title, axes, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]), filled (bool, default true), fill_opacity (int 0-255, default 50),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_radar_chart(title: &str, axes: Vec<String>, series_values: Vec<Vec<f64>>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 560);

        use crate::plot::statistical::{RadarConfig, render_radar_html};
        let series_names: Option<Vec<String>> = kwargs.and_then(|k| k.get_item("series_names").ok().flatten()).and_then(|v| v.extract().ok());
        let filled = kw_bool(kwargs, "filled", true);
        let fill_opacity = kw_u8(kwargs, "fill_opacity", 50);
        let names = series_names.unwrap_or_else(|| (0..series_values.len()).map(|_| String::new()).collect());
        let hover = o.hover();
        let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_values.into_iter()).collect();
        Ok(Chart::new(render_radar_html(&RadarConfig {
            title, axes: &axes, series: &series, palette: &o.pal, filled, fill_opacity, width: o.w, height: o.h,
            hover: &hover, ..RadarConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D radar chart (WebGL).
///
/// Args: title, axes, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]), width, height, x_label, y_label, z_label.
pub fn build_radar3d_chart(title: &str, axes: Vec<String>, series_values: Vec<Vec<f64>>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 560);

        let sn = kw_vec_str(kwargs, "series_names");
        let n_axes = axes.len();
        if n_axes == 0 { return Ok(Chart::new(String::new())); }
        let n_series = series_values.len();
        let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
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
            ("Axis", "Series", "Axis"),
            &cv, &names, o.w, o.h, None,
        )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Lollipop chart.
///
/// Args: title, labels, values.
///
/// Kwargs: color_hex (int), orientation ("v"/"h"), show_values (bool),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_lollipop_chart(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 480);

        use crate::plot::statistical::{LollipopConfig, render_lollipop_html};
        let clr = kw_u32(kwargs, "color_hex", 0);
        let orient = if kw_str(kwargs, "orientation", "v") == "h" { b'h' } else { b'v' };
        let show_values = kw_bool(kwargs, "show_values", false);
        let hover = o.hover();
        Ok(Chart::new(render_lollipop_html(&LollipopConfig {
            title, labels: &labels, values: &values, x_label: &o.xl, y_label: &o.yl,
            palette: &o.pal, color_hex: clr, gridlines: o.grid, show_values, orientation: orient, sort_order: &o.srt, width: o.w, height: o.h, hover: &hover, legend_position: &o.lp,
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D lollipop chart (WebGL).
///
/// Args: title, x_values, y_values, z_values.
///
/// Kwargs: color_labels (list[str]), width, height, x_label, y_label, z_label.
pub fn build_lollipop3d_chart(title: &str, x_values: Vec<f64>, y_values: Vec<f64>, z_values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let xl = kw_str(kwargs, "x_label", "Feature Index");
        let yl = kw_str(kwargs, "y_label", "|Correlation|");
        let zl = kw_str(kwargs, "z_label", "Correlation");
        let cl = kw_vec_str(kwargs, "color_labels");
        Ok(Chart::new(crate::plot::statistical::_3d::render_lollipop3d_html(
            title, &x_values, &y_values, &z_values,
            (&o.xl, &o.yl, &o.zl), &[], &cl, o.w, o.h, None,
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// KDE (kernel density estimation) plot.
///
/// Args: title, values.
///
/// Kwargs: filled (bool, default true), fill_opacity (int 0-255, default 50),
/// bandwidth (float, 0=auto), categories (list[str] for multi-kde),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_kde_chart(title: &str, values: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 420);

        use crate::plot::statistical::{KdeConfig, render_kde_html};
        let yl = kw_str(kwargs, "y_label", "Density");
        let filled = kw_bool(kwargs, "filled", true);
        let fill_opacity = kw_u8(kwargs, "fill_opacity", 50);
        let bandwidth = kw_f64(kwargs, "bandwidth", 0.0);
        let categories: Option<&PyAny> = kwargs.and_then(|k| k.get_item("categories").ok().flatten());
        let (vals, step) = fast_f64(values, 100)?;
        let series: Vec<(String, Vec<f64>)> = if let Some(cat_any) = categories {
            let (cats, _) = fast_labels_py(cat_any, 100)?;
            let sampled_cats = fast_labels(cats, step);
            let mut group_order: Vec<String> = Vec::new();
            let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
            for (v, c) in vals.iter().zip(sampled_cats.iter()) {
                group_vals.entry(c.clone()).or_default().push(*v);
                if !group_order.contains(c) { group_order.push(c.clone()); }
            }
            group_order.into_iter().map(|k| {
                let v = group_vals.remove(&k).unwrap_or_default();
                (k, v)
            }).collect()
        } else {
            vec![("Series".to_string(), vals)]
        };
        let hover = o.hover();
        Ok(Chart::new(render_kde_html(&KdeConfig {
            title, series: &series, palette: &o.pal, x_label: &o.xl, y_label: &o.yl,
            bandwidth, filled, fill_opacity, gridlines: o.grid, width: o.w, height: o.h,
            sort_order: &o.srt, hover: &hover, ..KdeConfig::default()
        })))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D KDE chart (WebGL).
///
/// Args: title, values.
///
/// Kwargs: categories (list[str]), width, height, x_label, y_label, z_label.
pub fn build_kde3d_chart(title: &str, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        use crate::plot::statistical::kde::{scott_bw, kde_eval};
        let yl = kw_str(kwargs, "y_label", "Outcome");
        let zl = kw_str(kwargs, "z_label", "Density");
        let cats: Vec<String> = kwargs.and_then(|k| k.get_item("categories").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
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
            title, &xv, &yv, &zv, (&o.xl, &o.yl, &o.zl), &cv, &names, o.w, o.h, None,
        )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Ridgeline (joy) plot.
///
/// Args: title, values (flat), categories (one per value).
///
/// Kwargs: overlap (float, default 0.5), bandwidth (float, 0=auto),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_ridgeline_chart(title: &str, values: &PyAny, categories: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 520);

        use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
        let overlap = kw_f64(kwargs, "overlap", 0.5);
        let bandwidth = kw_f64(kwargs, "bandwidth", 0.0);
        let (vals, _) = fast_f64(values, 200)?;
        let (cats, _) = fast_labels_py(categories, 200)?;
        let hover = o.hover();
        Ok(Chart::new(render_ridgeline_html(&RidgelineConfig {
            title, values: &vals, categories: &cats, palette: &o.pal,
            x_label: &o.xl, y_label: &o.yl, overlap, bandwidth, width: o.w, height: o.h, gridlines: o.grid,
            sort_order: &o.srt, hover: &hover, ..RidgelineConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D ridgeline chart (WebGL).
///
/// Args: title, values (flat), categories (one per value).
///
/// Kwargs: width, height, x_label, y_label, z_label.
pub fn build_ridgeline3d_chart(title: &str, values: &PyAny, categories: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        use crate::plot::statistical::kde::{scott_bw, kde_eval};
        let yl = kw_str(kwargs, "y_label", "Category");
        let zl = kw_str(kwargs, "z_label", "Density");
        let (vals, _) = fast_f64(values, 200)?;
        let (cats, _) = fast_labels_py(categories, 200)?;
        let mut group_order: Vec<String> = Vec::new();
        let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        for (v, c) in vals.iter().zip(cats.iter()) {
            group_vals.entry(c.clone()).or_default().push(*v);
            if !group_order.contains(c) { group_order.push(c.clone()); }
        }
        if group_order.is_empty() { return Ok(Chart::new(String::new())); }
        let xmin = vals.iter().cloned().fold(f64::INFINITY, f64::min);
        let xmax = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
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
            title, &xv, &yv, &zv, (&o.xl, &o.yl, &o.zl), &cv, &group_order, o.w, o.h, None,
        )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D pie chart (WebGL).
///
/// Args: title, labels, values.
///
/// Kwargs: bg_color (str), width, height, palette.
pub fn build_pie3d_chart(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
        use crate::plot::statistical::common::apply_sort;
        let (labels, values) = apply_sort(&labels, &values, &o.srt);
        let n = labels.len().min(values.len());
        let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        Ok(Chart::new(crate::plot::statistical::_3d::render_pie3d_html(
            title, &xv, &yv, &values[..n], ("", "", ""), &cv, &labels[..n].to_vec(), o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D violin chart (WebGL).
///
/// Args: title, values (flat).
///
/// Kwargs: categories (list[str]), width, height, x_label, y_label, z_label.
pub fn build_violin3d_chart(title: &str, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        use crate::plot::statistical::kde::{scott_bw, kde_eval};
        let xl = kw_str(kwargs, "x_label", "Value");
        let yl = kw_str(kwargs, "y_label", "Category");
        let zl = kw_str(kwargs, "z_label", "Density");
        let cats: Vec<String> = kwargs.and_then(|k| k.get_item("categories").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
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
            title, &xv, &yv, &zv, (&o.xl, &o.yl, &o.zl), &cv, &group_order, o.w, o.h, None,
        )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, y_labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D heatmap (WebGL).
///
/// Args: title, x_labels, y_labels, values (list of list[float]).
///
/// Kwargs: bg_color (str), width, height, x_label, y_label, z_label.
pub fn build_heatmap3d_chart(title: &str, x_labels: Vec<String>, y_labels: Vec<String>, values: Vec<Vec<f64>>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
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
            title, &xv, &yv, &zv, (&o.xl, &o.yl, &o.zl), &cv, &cl, o.w, o.h, bg.as_deref(),
        )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, open, high, low, close, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D candlestick chart (WebGL).
///
/// Args: title, labels, open, high, low, close.
///
/// Kwargs: bg_color (str), width, height, x_label, y_label, z_label.
pub fn build_candlestick3d_chart(title: &str, labels: Vec<String>, open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
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
            title, &xv, &yv, &zv, ("Price", "Bar", ""), &[], &labels, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_start, values_end, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D dumbbell chart (WebGL).
///
/// Args: title, labels, values_start, values_end.
///
/// Kwargs: series_names (tuple[str,str], default ("Start","End")), bg_color (str),
/// width, height, x_label, y_label, z_label.
pub fn build_dumbbell3d_chart(title: &str, labels: Vec<String>, values_start: Vec<f64>, values_end: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
        let sl: Option<(String, String)> = kwargs.and_then(|k| k.get_item("series_names").ok().flatten()).and_then(|v| v.extract().ok());
        let axis_names = sl.as_ref().map(|(a, b)| (a.as_str(), "Item", b.as_str())).unwrap_or(("Start", "Item", "End"));
        let n = labels.len().min(values_start.len()).min(values_end.len());
        let mut xv = Vec::new();
        let mut yv = Vec::new();
        let mut zv = Vec::new();
        let mut cv = Vec::new();
        for i in 0..n {
            xv.push(values_start[i]); yv.push(i as f64); zv.push(values_end[i]); cv.push(i as f64);
        }
        Ok(Chart::new(crate::plot::statistical::_3d::render_dumbbell3d_html(
            title, &xv, &yv, &zv, axis_names, &cv, &labels, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D funnel chart (WebGL).
///
/// Args: title, labels, values.
///
/// Kwargs: bg_color (str), width, height.
pub fn build_funnel3d_chart(title: &str, labels: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
        use crate::plot::statistical::common::apply_sort;
        let (labels, values) = apply_sort(&labels, &values, &o.srt);
        let n = labels.len().min(values.len());
        let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        Ok(Chart::new(crate::plot::statistical::_3d::render_funnel3d_html(
            title, &xv, &yv, &values[..n], ("", "Stage", "Value"), &cv, &labels[..n].to_vec(), o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, parents, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D sunburst chart (WebGL).
///
/// Args: title, labels, parents, values.
///
/// Kwargs: bg_color (str), width, height.
pub fn build_sunburst3d_chart(title: &str, labels: Vec<String>, parents: Vec<String>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 700, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
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
            title, &xv, &yv, &zv, ("", "Ring", "Value"), &cv, &cl, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D stacked bar chart (WebGL).
///
/// Args: title, category_labels, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]), bg_color (str), width, height, x_label, y_label, z_label.
pub fn build_stacked_bar3d_chart(title: &str, category_labels: Vec<String>, series_values: Vec<Vec<f64>>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 560);

        let bg = kw_opt_str(kwargs, "bg_color");
        let sn = kw_vec_str(kwargs, "series_names");
        let n_cat = category_labels.len();
        let n_ser = series_values.len();
        let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_ser).map(|_| String::new()).collect() };
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
            title, &xv, &yv, &zv, ("Category", "Series", "Value"), &cv, &names, o.w, o.h, bg.as_deref(),
        )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, latitudes, longitudes, values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// 3D globe chart (WebGL).
///
/// Args: title, latitudes, longitudes, values.
///
/// Kwargs: labels (list[str]), bg_color (str), width, height.
pub fn build_globe3d_chart(title: &str, latitudes: Vec<f64>, longitudes: Vec<f64>, values: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 800, 600);

        let bg = kw_opt_str(kwargs, "bg_color");
        let labels: Vec<String> = kwargs.and_then(|k| k.get_item("labels").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let n = latitudes.len().min(longitudes.len()).min(values.len());
        let cl = if labels.is_empty() { (0..n).map(|i| format!("Point {}", i + 1)).collect() } else { labels };
        let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
        Ok(Chart::new(crate::plot::map::_3d::render_globe3d_html(
            title, &longitudes[..n], &latitudes[..n], &values[..n],
            ("Longitude", "Latitude", "Value"), &cv, &cl, o.w, o.h, bg.as_deref(),
        )))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, words, frequencies, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Word cloud.
///
/// Args: title, words, frequencies.
///
/// Kwargs: min_font (float, default 12), max_font (float, default 72), bg_color (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_wordcloud(title: &str, words: Vec<String>, frequencies: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 500);

        use crate::plot::statistical::wordcloud::{WordCloudConfig, render_wordcloud_html};
        let min_font = kw_f64(kwargs, "min_font", 12.0);
        let max_font = kw_f64(kwargs, "max_font", 72.0);
        let bg = kw_opt_str(kwargs, "bg_color");
        let hover = o.hover();
        Ok(Chart::new(render_wordcloud_html(&WordCloudConfig {
            title, words: &words, frequencies: &frequencies,
            palette: &o.pal, width: o.w, height: o.h, min_font, max_font,
            bg_color: bg.as_deref(), sort_order: &o.srt, hover: &hover, ..WordCloudConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, open, high, low, close, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Candlestick (OHLC) chart.
///
/// Args: title, labels, open, high, low, close.
///
/// Kwargs: width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_candlestick(title: &str, labels: Vec<String>, open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1100, 500);

        use crate::plot::statistical::candlestick::{CandlestickConfig, render_candlestick_html};
        let hover = o.hover();
        Ok(Chart::new(render_candlestick_html(&CandlestickConfig {
            title, labels: &labels, open: &open, high: &high, low: &low, close: &close,
            palette: &o.pal, width: o.w, height: o.h, x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
            sort_order: &o.srt, hover: &hover, ..CandlestickConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_start, values_end, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Dumbbell chart (start-end comparison).
///
/// Args: title, labels, values_start, values_end.
///
/// Kwargs: series_names (tuple[str,str], default ("Start","End")),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_dumbbell(title: &str, labels: Vec<String>, values_start: Vec<f64>, values_end: Vec<f64>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1000, 500);

        use crate::plot::statistical::dumbbell::{DumbbellConfig, render_dumbbell_html};
        let sl: Option<(String, String)> = kwargs.and_then(|k| k.get_item("series_names").ok().flatten()).and_then(|v| v.extract().ok());
        let sl = sl.unwrap_or(("Start".to_string(), "End".to_string()));
        let hover = o.hover();
        Ok(Chart::new(render_dumbbell_html(&DumbbellConfig {
            title, labels: &labels, values_start: &values_start, values_end: &values_end,
            series_names: (&sl.0, &sl.1),
            palette: &o.pal, width: o.w, height: o.h, x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
            sort_order: &o.srt, hover: &hover, ..DumbbellConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, sizes, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Bubble chart.
///
/// Args: title, x_values, y_values, sizes.
///
/// Kwargs: categories (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_bubble(title: &str, x_values: &PyAny, y_values: &PyAny, sizes: &PyAny, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 900, 500);

        use crate::plot::statistical::bubble::{BubbleConfig, render_bubble_html};
        let cats: Vec<String> = kwargs.and_then(|k| k.get_item("categories").ok().flatten()).and_then(|v| v.extract().ok()).unwrap_or_default();
        let (xv, sx) = fast_f64(x_values, 80)?;
        let (yv, _) = fast_f64(y_values, 80)?;
        let (sv, _) = fast_f64(sizes, 80)?;
        let cs: Vec<String> = if sx > 1 && !cats.is_empty() { cats.into_iter().step_by(sx).collect() } else { cats };
        let hover = o.hover();
        Ok(Chart::new(render_bubble_html(&BubbleConfig {
            title, x_values: &xv, y_values: &yv, sizes: &sv,
            categories: &cs, palette: &o.pal, width: o.w, height: o.h, x_label: &o.xl, y_label: &o.yl, gridlines: o.grid,
            sort_order: &o.srt, hover: &hover, ..BubbleConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, value, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Gauge chart.
///
/// Args: title, value (float).
///
/// Kwargs: min_val (float, default 0), max_val (float, default 100), label (str),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_gauge(title: &str, value: f64, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 400, 300);

        use crate::plot::statistical::gauge::{GaugeConfig, render_gauge_html};
        let min_val = kw_f64(kwargs, "min_val", 0.0);
        let max_val = kw_f64(kwargs, "max_val", 100.0);
        let label = kw_str(kwargs, "label", "");
        let hover = o.hover();
        Ok(Chart::new(render_gauge_html(&GaugeConfig {
            title, value, min_val, max_val, label: &label, width: o.w, height: o.h,
            hover: &hover, ..GaugeConfig::default()
        })))
    
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_values, **kwargs))]
#[allow(unused_variables, unused_mut)]
/// Parallel coordinates chart.
///
/// Args: title, axes, series_values (list of list[float]).
///
/// Kwargs: series_names (list[str]),
/// width, height, palette, x_label, y_label, gridlines, sort_order, hover_json, legend_position.
pub fn build_parallel(title: &str, axes: Vec<String>, series_values: Vec<Vec<f64>>, kwargs: Option<&PyDict>) -> PyResult<Chart> {
    let o = ChartOpts::new(kwargs, 1000, 500);

        use crate::plot::statistical::parallel::{ParallelConfig, render_parallel_html};
        let sn = kw_vec_str(kwargs, "series_names");
        let names: Vec<String> = if !sn.is_empty() { sn } else { (0..series_values.len()).map(|_| String::new()).collect() };
        let hover = o.hover();
        Ok(Chart::new(render_parallel_html(&ParallelConfig {
            title, axes: &axes, series_names: &names, series_values: &series_values,
            palette: &o.pal, width: o.w, height: o.h,
            hover: &hover, ..ParallelConfig::default()
        })))
    
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