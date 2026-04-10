#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
fn parse_palette(palette: Option<Vec<u32>>) -> Vec<u32> {
    palette.unwrap_or_default()
}

#[cfg(feature = "python")]
fn fast_f64(data: &PyAny, cap: usize) -> PyResult<(Vec<f64>, usize)> {
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
pub fn build_html_chart(title: &str, labels: Vec<String>, values: Vec<f64>, width: i32, height: i32) -> Chart {
    Chart::new({
        let exporter = crate::html::FastHtmlExporter::new(width, height, title.to_string());
        exporter.build_optimized(labels, values)
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, orientation="v", color_groups=None, show_text=false, x_label="", y_label="", palette=None, color_hex=0u32, gridlines=true, sort_order="none", width=900, height=480, hover_json=None, bg_color=None))]
pub fn build_bar_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    orientation: &str,
    color_groups: Option<Vec<String>>,
    show_text: bool,
    x_label: &str,
    y_label: &str,
    palette: Option<Vec<u32>>,
    color_hex: u32,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::parse_hover_json;
    let pal = parse_palette(palette);
    let groups = color_groups.unwrap_or_default();
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let orient = if orientation == "h" { b'h' } else { b'v' };
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, width, height, &hover,
        orient, &groups, show_text, x_label, y_label, &pal, color_hex, gridlines, sort_order,
    );
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, color_groups=None, show_text=true, x_label="", palette=None, color_hex=0u32, gridlines=true, sort_order="none", width=900, height=500, hover_json=None))]
pub fn build_hbar(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    color_groups: Option<Vec<String>>,
    show_text: bool,
    x_label: &str,
    palette: Option<Vec<u32>>,
    color_hex: u32,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::parse_hover_json;
    let pal = parse_palette(palette);
    let groups = color_groups.unwrap_or_default();
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(crate::plot::default::render_bars_html(
        title, &labels, &values, width, height, &hover,
        b'h', &groups, show_text, x_label, "", &pal, color_hex, gridlines, sort_order,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, color_hex=0x6366F1u32, x_label="", y_label="", gridlines=true, show_points=true, sort_order="none", width=900, height=480, hover_json=None, bg_color=None))]
pub fn build_line_chart(
    title: &str,
    labels: &PyAny,
    values: &PyAny,
    color_hex: u32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    show_points: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::parse_hover_json;
    let (vals, step_v) = fast_f64(values, 100)?;
    let (lbls, _) = fast_labels_py(labels, 100)?;
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let html = crate::plot::default::render_lines_html(
        title, &lbls, &vals, width, height, &hover, color_hex, x_label, y_label, gridlines, show_points,
    );
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Ok(Chart::new(html))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, labels=None, palette=None, x_label="", y_label="", color_hex=0u32, gridlines=true, show_text=false, sort_order="none", width=900, height=540, hover_json=None, sizes=None, color_groups=None, bg_color=None))]
pub fn build_scatter_chart(
    title: &str,
    x_values: &PyAny,
    y_values: &PyAny,
    labels: Option<Vec<String>>,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    color_hex: u32,
    gridlines: bool,
    show_text: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    sizes: Option<Vec<f64>>,
    color_groups: Option<Vec<String>>,
    bg_color: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::parse_hover_json;
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let (x, sx) = fast_f64(x_values, 200)?;
    let (y, sy) = fast_f64(y_values, 200)?;
    let step = sx.max(sy);
    let lbls = fast_labels(labels.unwrap_or_default(), step);
    let sz: Vec<f64> = if step > 1 { sizes.unwrap_or_default().into_iter().step_by(step).collect() } else { sizes.unwrap_or_default() };
    let cg: Vec<String> = if step > 1 { color_groups.unwrap_or_default().into_iter().step_by(step).collect() } else { color_groups.unwrap_or_default() };
    let html = crate::plot::default::render_scatter_html(
        title, &x, &y, &lbls, width, height, &hover,
        &sz, &cg, &pal, x_label, y_label, color_hex, gridlines, show_text,
    );
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Ok(Chart::new(html))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, bins=0usize, color_hex=0x6366F1u32, x_label="", y_label="Count", show_counts=false, gridlines=true, sort_order="none", width=860, height=380, hover_json=None, bg_color=None))]
pub fn build_histogram(
    title: &str,
    values: &PyAny,
    bins: usize,
    color_hex: u32,
    x_label: &str,
    y_label: &str,
    show_counts: bool,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let (vals, scale) = fast_f64(values, 1000)?;
    let html = render_histogram_html(&HistogramConfig {
        title, values: &vals, bins, color: color_hex, x_label, y_label,
        show_counts, gridlines, width, height, hover: &hover, count_scale: scale,
        ..HistogramConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Ok(Chart::new(html))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, overlay_values, bins=0usize, color_hex=0x6366F1u32, overlay_color_hex=0xF43F5Eu32, sort_order="none", width=860, height=380, hover_json=None, bg_color=None))]
pub fn build_histogram_overlay(
    title: &str,
    values: Vec<f64>,
    overlay_values: Vec<f64>,
    bins: usize,
    color_hex: u32,
    overlay_color_hex: u32,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let html = render_histogram_html(&HistogramConfig {
        title, values: &values, bins, color: color_hex, overlay_color: overlay_color_hex,
        overlay_values: Some(&overlay_values), width, height, hover: &hover,
        ..HistogramConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, palette=None, x_label="", y_label="", show_values=false, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None, bg_color=None))]
pub fn build_grouped_bar(
    title: &str,
    category_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<f64>,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    show_values: bool,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let n_cats = category_labels.len();
    let n_ser = series_names.len();

    let series: Vec<(String, Vec<f64>)> = series_names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| {
            series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)
        }).collect();
        (name.clone(), vals)
    }).collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title, category_labels: &category_labels, series: &series,
        palette: &pal, x_label, y_label, show_values, gridlines, sort_order,
        hover: &hover, ..GroupedBarConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, palette=None, x_label="", y_label="", show_values=false, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None, bg_color=None))]
pub fn build_stacked_bar(
    title: &str,
    category_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<f64>,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    show_values: bool,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    let series: Vec<(String, Vec<f64>)> = series_names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| {
            series_values.get(si * n_cats + ci).copied().unwrap_or(0.0)
        }).collect();
        (name.clone(), vals)
    }).collect();
    let html = render_grouped_bar_html(&GroupedBarConfig {
        title, category_labels: &category_labels, series: &series,
        palette: &pal, x_label, y_label, show_values, gridlines, sort_order,
        hover: &hover, stacked: true, ..GroupedBarConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, flat_matrix, col_labels=None, show_values=true, color_low=0x6366F1u32, color_mid=0xfafbfcu32, color_high=0xF43F5Eu32, sort_order="none", width=720, height=440, hover_json=None, bg_color=None))]
pub fn build_heatmap(
    title: &str,
    labels: Vec<String>,
    flat_matrix: Vec<f64>,
    col_labels: Option<Vec<String>>,
    show_values: bool,
    color_low: u32,
    color_mid: u32,
    color_high: u32,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{HeatmapConfig, render_heatmap_html, parse_hover_json};
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let col_lbls = col_labels.unwrap_or_default();
    let html = render_heatmap_html(&HeatmapConfig {
        title, row_labels: &labels, col_labels: &col_lbls, flat_matrix: &flat_matrix,
        show_values, color_low, color_mid, color_high, width, height, hover: &hover,
        ..HeatmapConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, palette=None, show_pct=true, sort_order="none", width=720, height=440, hover_json=None))]
pub fn build_pie_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    palette: Option<Vec<u32>>,
    show_pct: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{PieConfig, render_pie_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &pal,
        show_pct, sort_order, width, height, hover: &hover,
        ..PieConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, inner_radius_ratio=0.55f64, palette=None, show_pct=true, sort_order="none", width=720, height=440, hover_json=None))]
pub fn build_donut_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    inner_radius_ratio: f64,
    palette: Option<Vec<u32>>,
    show_pct: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{PieConfig, render_pie_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &pal,
        show_pct, sort_order, width, height, hover: &hover,
        donut: inner_radius_ratio.clamp(0.0, 0.9),
        ..PieConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, values, palette=None, sort_order="none", width=900, height=500, hover_json=None))]
pub fn build_boxplot(
    title: &str,
    category_labels: &PyAny,
    values: &PyAny,
    palette: Option<Vec<u32>>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::{render_boxplot_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let (v, sv) = fast_f64(values, 100)?;
    let (cl, _) = fast_labels_py(category_labels, 100)?;
    Ok(Chart::new(render_boxplot_html(title, &cl, &v, width, height, &pal, &hover)))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, categories, values, x_label="", y_label="", palette=None, gridlines=true, width=900, height=500, bg_color=None))]
pub fn build_violin(
    title: &str,
    categories: &PyAny,
    values: &PyAny,
    x_label: &str,
    y_label: &str,
    palette: Option<Vec<u32>>,
    gridlines: bool,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::{ViolinConfig, render_violin_html};
    let pal = parse_palette(palette);
    let (v, sv) = fast_f64(values, 50)?;
    let (cats, _) = fast_labels_py(categories, 50)?;
    let html = render_violin_html(&ViolinConfig {
        title, categories: &cats, values: &v,
        x_label, y_label, palette: &pal, gridlines, width, height,
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Ok(Chart::new(html))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_left, values_right, left_label="Before", right_label="After", palette=None, show_text=true, width=700, height=500, bg_color=None))]
pub fn build_slope(
    title: &str,
    labels: Vec<String>,
    values_left: Vec<f64>,
    values_right: Vec<f64>,
    left_label: &str,
    right_label: &str,
    palette: Option<Vec<u32>>,
    show_text: bool,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{SlopeConfig, render_slope_html};
    let pal = parse_palette(palette);
    let html = render_slope_html(&SlopeConfig {
        title, labels: &labels, values_left: &values_left, values_right: &values_right,
        left_label, right_label, palette: &pal, show_text, width, height,
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, parents, values, width=700, height=700, bg_color=None))]
pub fn build_sunburst(
    title: &str,
    labels: Vec<String>,
    parents: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{SunburstConfig, render_sunburst_html};
    let html = render_sunburst_html(&SunburstConfig {
        title, labels: &labels, parents: &parents, values: &values, width, height,
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, sort_order="none", show_text=true, palette=None, width=800, height=480, bg_color=None))]
pub fn build_funnel(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    sort_order: &str,
    show_text: bool,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{FunnelConfig, render_funnel_html};
    let pal = parse_palette(palette);
    let html = render_funnel_html(&FunnelConfig {
        title, labels: &labels, values: &values, palette: &pal, show_text, width, height,
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, parents=None, palette=None, sort_order="none", width=1100, height=520, hover_json=None))]
pub fn build_treemap(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    parents: Option<Vec<String>>,
    palette: Option<Vec<u32>>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{TreemapConfig, render_treemap_html, parse_hover_json};
    let pal = parse_palette(palette);
    let pars = parents.unwrap_or_default();
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(render_treemap_html(&TreemapConfig {
        title, labels: &labels, values: &values, parents: &pars,
        palette: &pal, sort_order, width, height, hover: &hover,
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_names, series_values, palette=None, x_label="", y_label="", show_points=true, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None, bg_color=None))]
pub fn build_multiline_chart(
    title: &str,
    x_labels: &PyAny,
    series_names: Vec<String>,
    series_values: &PyAny,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    show_points: bool,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
    bg_color: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::{MultiLineConfig, render_multiline_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let (names, vecs, step) = fast_vecs_py(series_names, series_values, 60)?;
    let (xlabels, _) = fast_labels_py(x_labels, 60)?;
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(vecs.into_iter()).collect();
    let html = render_multiline_html(&MultiLineConfig {
        title, x_labels: &xlabels, series: &series, palette: &pal,
        x_label, y_label, show_points, gridlines, width, height, hover: &hover,
        ..MultiLineConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Ok(Chart::new(html))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_names, series_values, stacked=false, palette=None, x_label="", y_label="", gridlines=true, sort_order="none", width=1100, height=480, hover_json=None))]
pub fn build_area_chart(
    title: &str,
    x_labels: &PyAny,
    series_names: Vec<String>,
    series_values: &PyAny,
    stacked: bool,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> PyResult<Chart> {
    use crate::plot::statistical::{AreaConfig, render_area_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let (names, vecs, _) = fast_vecs_py(series_names, series_values, 60)?;
    let (xlabels, _) = fast_labels_py(x_labels, 60)?;
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(vecs.into_iter()).collect();
    Ok(Chart::new(render_area_html(&AreaConfig {
        title, x_labels: &xlabels, series: &series, stacked, palette: &pal,
        x_label, y_label, gridlines, width, height, hover: &hover,
        ..AreaConfig::default()
    })))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, x_label="", y_label="", show_text=true, gridlines=true, width=900, height=480))]
pub fn build_waterfall(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    show_text: bool,
    gridlines: bool,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{WaterfallConfig, render_waterfall_html};
    Chart::new(render_waterfall_html(&WaterfallConfig {
        title, labels: &labels, values: &values, x_label, y_label, show_text, gridlines, width, height,
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, targets=None, max_vals=None, ranges=None, width=800, height=300))]
pub fn build_bullet(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    targets: Option<Vec<f64>>,
    max_vals: Option<Vec<f64>>,
    ranges: Option<Vec<f64>>,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{BulletConfig, render_bullet_html};
    let tgts = targets.unwrap_or_default();
    let maxs = max_vals.unwrap_or_default();
    let rngs = ranges.unwrap_or_default();
    Chart::new(render_bullet_html(&BulletConfig {
        title, labels: &labels, values: &values, targets: &tgts, max_vals: &maxs, ranges: &rngs, width, height,
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, sort_order="none", width=1200, height=600, hover_json=None))]
pub fn build_bubble_map(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::parse_hover_json;
    use crate::plot::map::render_bubble_map_html;
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(render_bubble_map_html(title, &labels, &values, width, height, &hover))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, sort_order="none", width=1200, height=600, hover_json=None))]
pub fn build_choropleth(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::parse_hover_json;
    use crate::plot::map::render_choropleth_html;
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(render_choropleth_html(title, &labels, &values, width, height, &hover))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560, bg_color=None))]
pub fn build_scatter3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    sort_order: &str,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    Chart::new(crate::plot::default::render_scatter3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &cv, &cl, width, height, bg_color,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560, bg_color=None))]
pub fn build_bar3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    sort_order: &str,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    Chart::new(crate::plot::default::render_bar3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &cv, &cl, width, height, bg_color,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560, bg_color=None))]
pub fn build_line3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    sort_order: &str,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    Chart::new(crate::plot::default::render_line3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &cv, &cl, width, height, bg_color,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_names, series_values, palette=None, filled=true, fill_opacity=50u8, width=700, height=560))]
pub fn build_radar_chart(
    title: &str,
    axes: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
    palette: Option<Vec<u32>>,
    filled: bool,
    fill_opacity: u8,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{RadarConfig, render_radar_html};
    let pal = parse_palette(palette);
    let series: Vec<(String, Vec<f64>)> = series_names.into_iter().zip(series_values.into_iter()).collect();
    Chart::new(render_radar_html(&RadarConfig {
        title, axes: &axes, series: &series, palette: &pal, filled, fill_opacity, width, height,
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_names, series_values, palette=None, width=700, height=560))]
pub fn build_radar3d_chart(
    title: &str,
    axes: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
) -> Chart {
    let n_axes = axes.len();
    if n_axes == 0 { return Chart::new(String::new()); }
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let n_series = series_names.len().min(series_values.len());
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
    Chart::new(crate::html::js_3d::render_radar3d_html(
        title, &xv, &yv, &zv,
        ("Axis", "Series", "Axis"),
        &cv, &series_names, width, height, None,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, orientation="v", x_label="", y_label="", palette=None, color_hex=0u32, gridlines=true, show_values=false, sort_order="none", width=900, height=480))]
pub fn build_lollipop_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    orientation: &str,
    x_label: &str,
    y_label: &str,
    palette: Option<Vec<u32>>,
    color_hex: u32,
    gridlines: bool,
    show_values: bool,
    sort_order: &str,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{LollipopConfig, render_lollipop_html};
    let pal = parse_palette(palette);
    let orient = if orientation == "h" { b'h' } else { b'v' };
    Chart::new(render_lollipop_html(&LollipopConfig {
        title, labels: &labels, values: &values, x_label, y_label,
        palette: &pal, color_hex, gridlines, show_values, orientation: orient, sort_order, width, height,
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="Feature Index", y_label="|Correlation|", z_label="Correlation", color_labels=None, width=900, height=560))]
pub fn build_lollipop3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_labels: Option<Vec<String>>,
    width: i32,
    height: i32,
) -> Chart {
    let cl = color_labels.unwrap_or_default();
    Chart::new(crate::html::js_3d::render_lollipop3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &[], &cl, width, height, None,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories=None, palette=None, filled=true, fill_opacity=50u8, x_label="", y_label="Density", gridlines=true, bandwidth=0.0f64, width=900, height=420))]
pub fn build_kde_chart(
    title: &str,
    values: &PyAny,
    categories: Option<&PyAny>,
    palette: Option<Vec<u32>>,
    filled: bool,
    fill_opacity: u8,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    bandwidth: f64,
    width: i32,
    height: i32,
) -> PyResult<Chart> {
    use crate::plot::statistical::{KdeConfig, render_kde_html};
    let pal = parse_palette(palette);
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
    Ok(Chart::new(render_kde_html(&KdeConfig {
        title, series: &series, palette: &pal, x_label, y_label,
        bandwidth, filled, fill_opacity, gridlines, width, height,
        ..KdeConfig::default()
    })))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories=None, x_label="", y_label="Outcome", z_label="Density", width=900, height=560))]
pub fn build_kde3d_chart(
    title: &str,
    values: Vec<f64>,
    categories: Option<Vec<String>>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let cats = categories.unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = if cats.is_empty() {
        vec![("Series".to_string(), values)]
    } else {
        let mut group_order: Vec<String> = Vec::new();
        let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        for (v, c) in values.iter().zip(cats.iter()) {
            group_vals.entry(c.clone()).or_default().push(*v);
            if !group_order.contains(c) { group_order.push(c.clone()); }
        }
        group_order.into_iter().map(|k| {
            let v = group_vals.remove(&k).unwrap_or_default();
            (k, v)
        }).collect()
    };
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if all_vals.is_empty() { return Chart::new(String::new()); }
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
            xv.push(t);
            yv.push(si as f64);
            zv.push(d);
            cv.push(si as f64);
        }
    }
    Chart::new(crate::html::js_3d::render_kde3d_html(
        title, &xv, &yv, &zv,
        (x_label, y_label, z_label),
        &cv, &names, width, height, None,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, overlap=0.5f64, palette=None, x_label="", bandwidth=0.0f64, width=900, height=520))]
pub fn build_ridgeline_chart(
    title: &str,
    values: &PyAny,
    categories: &PyAny,
    overlap: f64,
    palette: Option<Vec<u32>>,
    x_label: &str,
    bandwidth: f64,
    width: i32,
    height: i32,
) -> PyResult<Chart> {
    use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
    let pal = parse_palette(palette);
    let (vals, _) = fast_f64(values, 200)?;
    let (cats, _) = fast_labels_py(categories, 200)?;
    Ok(Chart::new(render_ridgeline_html(&RidgelineConfig {
        title, values: &vals, categories: &cats, palette: &pal,
        x_label, overlap, bandwidth, width, height,
        ..RidgelineConfig::default()
    })))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, x_label="", y_label="Category", z_label="Density", width=900, height=560))]
pub fn build_ridgeline3d_chart(
    title: &str,
    values: &PyAny,
    categories: &PyAny,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    width: i32,
    height: i32,
) -> PyResult<Chart> {
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
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
            xv.push(t);
            yv.push(gi as f64);
            zv.push(d);
            cv.push(gi as f64);
        }
    }
    Ok(Chart::new(crate::html::js_3d::render_ridgeline3d_html(
        title, &xv, &yv, &zv,
        (x_label, y_label, z_label),
        &cv, &group_order, width, height, None,
    )))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, words, frequencies, palette=None, width=900, height=500, min_font=12.0, max_font=72.0, bg_color=None))]
pub fn build_wordcloud(
    title: &str,
    words: Vec<String>,
    frequencies: Vec<f64>,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    min_font: f64,
    max_font: f64,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::wordcloud::{WordCloudConfig, render_wordcloud_html};
    let pal = parse_palette(palette);
    Chart::new(render_wordcloud_html(&WordCloudConfig {
        title, words: &words, frequencies: &frequencies,
        palette: &pal, width, height, min_font, max_font,
        bg_color, ..WordCloudConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, open, high, low, close, palette=None, width=1100, height=500, x_label="Date", y_label="Price", gridlines=true))]
pub fn build_candlestick(
    title: &str,
    labels: Vec<String>,
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
) -> Chart {
    use crate::plot::statistical::candlestick::{CandlestickConfig, render_candlestick_html};
    let pal = parse_palette(palette);
    Chart::new(render_candlestick_html(&CandlestickConfig {
        title, labels: &labels, open: &open, high: &high, low: &low, close: &close,
        palette: &pal, width, height, x_label, y_label, gridlines,
        ..CandlestickConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_start, values_end, series_labels=None, palette=None, width=1000, height=500, x_label="", y_label="", gridlines=true))]
pub fn build_dumbbell(
    title: &str,
    labels: Vec<String>,
    values_start: Vec<f64>,
    values_end: Vec<f64>,
    series_labels: Option<(String, String)>,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
) -> Chart {
    use crate::plot::statistical::dumbbell::{DumbbellConfig, render_dumbbell_html};
    let pal = parse_palette(palette);
    let sl = series_labels.unwrap_or(("Start".to_string(), "End".to_string()));
    Chart::new(render_dumbbell_html(&DumbbellConfig {
        title, labels: &labels, values_start: &values_start, values_end: &values_end,
        series_labels: (&sl.0, &sl.1),
        palette: &pal, width, height, x_label, y_label, gridlines,
        ..DumbbellConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, sizes, categories=None, palette=None, width=900, height=500, x_label="", y_label="", gridlines=true))]
pub fn build_bubble(
    title: &str,
    x_values: &PyAny,
    y_values: &PyAny,
    sizes: &PyAny,
    categories: Option<Vec<String>>,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
) -> PyResult<Chart> {
    use crate::plot::statistical::bubble::{BubbleConfig, render_bubble_html};
    let pal = parse_palette(palette);
    let cats = categories.unwrap_or_default();
    let (xv, sx) = fast_f64(x_values, 80)?;
    let (yv, _) = fast_f64(y_values, 80)?;
    let (sv, _) = fast_f64(sizes, 80)?;
    let cs: Vec<String> = if sx > 1 && !cats.is_empty() { cats.into_iter().step_by(sx).collect() } else { cats };
    Ok(Chart::new(render_bubble_html(&BubbleConfig {
        title, x_values: &xv, y_values: &yv, sizes: &sv,
        categories: &cs, palette: &pal, width, height, x_label, y_label, gridlines,
        ..BubbleConfig::default()
    })))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, value, min_val=0.0f64, max_val=100.0f64, label="", width=400, height=300))]
pub fn build_gauge(
    title: &str,
    value: f64,
    min_val: f64,
    max_val: f64,
    label: &str,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::gauge::{GaugeConfig, render_gauge_html};
    Chart::new(render_gauge_html(&GaugeConfig {
        title, value, min_val, max_val, label, width, height,
        ..GaugeConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_names, series_values, palette=None, width=1000, height=500))]
pub fn build_parallel(
    title: &str,
    axes: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::parallel::{ParallelConfig, render_parallel_html};
    let pal = parse_palette(palette);
    Chart::new(render_parallel_html(&ParallelConfig {
        title, axes: &axes, series_names: &series_names, series_values: &series_values,
        palette: &pal, width, height,
        ..ParallelConfig::default()
    }))
}