#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
fn parse_palette(palette: Option<Vec<u32>>) -> Vec<u32> {
    palette.unwrap_or_default()
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
    labels: Vec<String>,
    values: Vec<f64>,
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
) -> Chart {
    use crate::plot::statistical::parse_hover_json;
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let html = crate::plot::default::render_lines_html(
        title, &labels, &values, width, height, &hover, color_hex, x_label, y_label, gridlines, show_points,
    );
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, labels=None, palette=None, x_label="", y_label="", color_hex=0u32, gridlines=true, show_text=false, sort_order="none", width=900, height=540, hover_json=None, sizes=None, color_groups=None, bg_color=None))]
pub fn build_scatter_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
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
) -> Chart {
    use crate::plot::statistical::parse_hover_json;
    let pal = parse_palette(palette);
    let lbls = labels.unwrap_or_default();
    let sz = sizes.unwrap_or_default();
    let cg = color_groups.unwrap_or_default();
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let html = crate::plot::default::render_scatter_html(
        title, &x_values, &y_values, &lbls, width, height, &hover,
        &sz, &cg, &pal, x_label, y_label, color_hex, gridlines, show_text,
    );
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, bins=0usize, color_hex=0x6366F1u32, x_label="", y_label="Count", show_counts=false, gridlines=true, sort_order="none", width=860, height=380, hover_json=None, bg_color=None))]
pub fn build_histogram(
    title: &str,
    values: Vec<f64>,
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
) -> Chart {
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let html = render_histogram_html(&HistogramConfig {
        title, values: &values, bins, color: color_hex, x_label, y_label,
        show_counts, gridlines, width, height, hover: &hover,
        ..HistogramConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
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
    category_labels: Vec<String>,
    values: Vec<f64>,
    palette: Option<Vec<u32>>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{render_boxplot_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    Chart::new(render_boxplot_html(title, &category_labels, &values, width, height, &pal, &hover))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, categories, values, x_label="", y_label="", palette=None, gridlines=true, width=900, height=500, bg_color=None))]
pub fn build_violin(
    title: &str,
    categories: Vec<String>,
    values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    palette: Option<Vec<u32>>,
    gridlines: bool,
    width: i32,
    height: i32,
    bg_color: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{ViolinConfig, render_violin_html};
    let pal = parse_palette(palette);
    let html = render_violin_html(&ViolinConfig {
        title, categories: &categories, values: &values,
        x_label, y_label, palette: &pal, gridlines, width, height,
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
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
    x_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
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
) -> Chart {
    use crate::plot::statistical::{MultiLineConfig, render_multiline_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = series_names.into_iter().zip(series_values.into_iter()).collect();
    let html = render_multiline_html(&MultiLineConfig {
        title, x_labels: &x_labels, series: &series, palette: &pal,
        x_label, y_label, show_points, gridlines, width, height, hover: &hover,
        ..MultiLineConfig::default()
    });
    let html = if let Some(c) = bg_color { crate::html::hover::apply_bg(html, Some(c)) } else { html };
    Chart::new(html)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_names, series_values, stacked=false, palette=None, x_label="", y_label="", gridlines=true, sort_order="none", width=1100, height=480, hover_json=None))]
pub fn build_area_chart(
    title: &str,
    x_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
    stacked: bool,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<&str>,
) -> Chart {
    use crate::plot::statistical::{AreaConfig, render_area_html, parse_hover_json};
    let pal = parse_palette(palette);
    let hover = hover_json.map(|s| parse_hover_json(s)).unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = series_names.into_iter().zip(series_values.into_iter()).collect();
    Chart::new(render_area_html(&AreaConfig {
        title, x_labels: &x_labels, series: &series, stacked, palette: &pal,
        x_label, y_label, gridlines, width, height, hover: &hover,
        ..AreaConfig::default()
    }))
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
    use crate::plot::statistical::{RadarConfig, render_radar_html};
    let pal = parse_palette(palette);
    let series: Vec<(String, Vec<f64>)> = series_names.into_iter().zip(series_values.into_iter()).collect();
    Chart::new(render_radar_html(&RadarConfig {
        title, axes: &axes, series: &series, palette: &pal, filled: true, fill_opacity: 40, width, height,
    }))
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
    Chart::new(crate::plot::default::render_scatter3d_html(
        title, &x_values, &y_values, &z_values,
        (x_label, y_label, z_label), &[], &cl, width, height, None,
    ))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories=None, palette=None, filled=true, fill_opacity=50u8, x_label="", y_label="Density", gridlines=true, bandwidth=0.0f64, width=900, height=420))]
pub fn build_kde_chart(
    title: &str,
    values: Vec<f64>,
    categories: Option<Vec<String>>,
    palette: Option<Vec<u32>>,
    filled: bool,
    fill_opacity: u8,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    bandwidth: f64,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{KdeConfig, render_kde_html};
    let pal = parse_palette(palette);
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
    Chart::new(render_kde_html(&KdeConfig {
        title, series: &series, palette: &pal, x_label, y_label,
        bandwidth, filled, fill_opacity, gridlines, width, height,
        ..KdeConfig::default()
    }))
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
    use crate::plot::statistical::{KdeConfig, render_kde_html};
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
    Chart::new(render_kde_html(&KdeConfig {
        title, series: &series, x_label: x_label, y_label: z_label,
        filled: true, fill_opacity: 50, width, height,
        ..KdeConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, overlap=0.5f64, palette=None, x_label="", bandwidth=0.0f64, width=900, height=520))]
pub fn build_ridgeline_chart(
    title: &str,
    values: Vec<f64>,
    categories: Vec<String>,
    overlap: f64,
    palette: Option<Vec<u32>>,
    x_label: &str,
    bandwidth: f64,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
    let pal = parse_palette(palette);
    Chart::new(render_ridgeline_html(&RidgelineConfig {
        title, values: &values, categories: &categories, palette: &pal,
        x_label, overlap, bandwidth, width, height,
        ..RidgelineConfig::default()
    }))
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, x_label="", y_label="Category", z_label="Density", width=900, height=560))]
pub fn build_ridgeline3d_chart(
    title: &str,
    values: Vec<f64>,
    categories: Vec<String>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    width: i32,
    height: i32,
) -> Chart {
    use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
    Chart::new(render_ridgeline_html(&RidgelineConfig {
        title, values: &values, categories: &categories,
        x_label, width, height,
        ..RidgelineConfig::default()
    }))
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