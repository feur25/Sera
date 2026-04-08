pub mod core;
pub mod data;
pub mod plot;
pub mod viewer;
pub mod bindings;
pub mod wiki;
pub mod html;

pub use core::math::{self, mean, median, std_dev};
pub use data::loader;
pub use data::processor;
pub use data::conversion;
pub use data::index;
pub use plot::canvas::Canvas;
pub use viewer::gui;
pub use wiki::{WikiExport, MethodDoc, ModuleDoc, WikiExtractor};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct SeraPlot;

impl SeraPlot {
    pub fn version() -> &'static str {
        VERSION
    }

    pub fn new_canvas(width: f32, height: f32, labels: Vec<String>, values: Vec<f64>, type_id: u8) -> Canvas {
        Canvas::new(width, height, labels, values, type_id)
    }

    pub fn load_csv<P: AsRef<std::path::Path>>(path: P) -> Result<crate::data::loader::CsvData, Box<dyn std::error::Error>> {
        crate::data::loader::CsvData::load(path).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add("__doc__", r#"SeraPlot - Rust-Powered Data Visualization Framework

SeraPlot is a framework developed in Rust, meticulously crafted with care. It is a modern 
alternative to Plotly, designed specifically for data visualization. This library is distributed 
across multiple programming languages (Python, C#, C++, JavaScript), regularly maintained and 
updated, offering superior speed and significantly lower memory consumption compared to competitors.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 COVER: src/asset/cover.png
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Why Choose SeraPlot
  • More fast than plotly; generation across all dataset sizes
  • Minimal memory footprint - ideal for resource-constrained environments
  • Production-ready with enterprise-grade stability
  • Multi-language support (Python, C#, C++, JavaScript, and more if necessary send me a message)
  • Regularly updated with new plots - new features and improvements
  • Perfect for real-time dashboards and batch processing

Install Command
Seraplot may be installed using pip
    >>> pip install seraplot

or you can also install in conda
    >>> conda install -c conda-forge seraplot

Simple Usage 
  >>> import seraplot, json
  >>> seraplot.show_chart_value(json.dumps({
  ...     'title': 'My Chart',
  ...     'labels': ['A', 'B', 'C', 'D'],
  ...     'values': [45.2, 38.9, 52.1, 41.7],
  ...     'hover': [{'index': i} for i in range(4)],
  ...     'group': 'default'
  ... }))
"#)?;
    m.add_function(wrap_pyfunction!(show_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(bench_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(set_chart_kind, m)?)?;
    m.add_function(wrap_pyfunction!(set_chart_orientation, m)?)?;
    m.add_function(wrap_pyfunction!(build_html_chart, m)?)?;
    
    m.add_function(wrap_pyfunction!(build_pie_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_donut_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_heatmap, m)?)?;
    m.add_function(wrap_pyfunction!(build_histogram, m)?)?;
    m.add_function(wrap_pyfunction!(build_histogram_overlay, m)?)?;
    m.add_function(wrap_pyfunction!(build_grouped_bar, m)?)?;
    m.add_function(wrap_pyfunction!(build_stacked_bar, m)?)?;
    m.add_function(wrap_pyfunction!(build_bar_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_line_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_scatter_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_choropleth, m)?)?;
    m.add_function(wrap_pyfunction!(build_bubble_map, m)?)?;
    m.add_function(wrap_pyfunction!(build_scatter3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_bar3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_line3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(bench_pure_rust, m)?)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (n=2000))]
fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) {
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
        let _ = crate::plot::default::render_bars_html("B", &labs, &vals, 900, 480, &[]);
    }
    let bar_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;

    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_scatter_html("B", &ages100, &fare100, &[], 900, 540, &[]);
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
fn show_chart_value(chart_json: &str) -> bool {
    let chart_json_c = std::ffi::CString::new(chart_json).unwrap_or_default();
    unsafe {
        crate::viewer::chart::sera_show_chart_value(chart_json_c.as_ptr())
    }
}

#[cfg(feature = "python")]
#[pyfunction]
fn bench_chart_value(chart_json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(chart_json).is_ok()
}

#[cfg(feature = "python")]
#[pyfunction]
fn set_chart_kind(kind: u8) -> bool {
    crate::viewer::chart::sera_set_current_chart_kind(kind);
    true
}

#[cfg(feature = "python")]
#[pyfunction]
fn set_chart_orientation(vertical: bool) -> bool {
    crate::viewer::chart::sera_set_chart_orientation(vertical);
    true
}

#[cfg(feature = "python")]
#[pyfunction]
fn build_html_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
) -> String {
    let exporter = crate::html::FastHtmlExporter::new(width, height, title.to_string());
    exporter.build_optimized(labels, values)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, width=720, height=440, hover_json=None))]
fn build_pie_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{PieConfig, render_pie_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    render_pie_html(&PieConfig {
        title,
        labels: &labels,
        values: &values,
        width,
        height,
        hover: &hover_slots,
        ..PieConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, width=720, height=440, inner_radius_ratio=0.55, hover_json=None))]
fn build_donut_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    inner_radius_ratio: f64,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{PieConfig, render_pie_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    render_pie_html(&PieConfig {
        title,
        labels: &labels,
        values: &values,
        width,
        height,
        donut: inner_radius_ratio.clamp(0.0, 0.9),
        hover: &hover_slots,
        ..PieConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, flat_matrix, width=720, height=440, hover_json=None))]
fn build_heatmap(
    title: &str,
    labels: Vec<String>,
    flat_matrix: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{HeatmapConfig, render_heatmap_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    render_heatmap_html(&HeatmapConfig {
        title,
        row_labels: &labels,
        col_labels: &[],
        flat_matrix: &flat_matrix,
        width,
        height,
        hover: &hover_slots,
        ..HeatmapConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, bins=0, color_hex=0x4C72B0, width=860, height=380, hover_json=None))]
fn build_histogram(
    title: &str,
    values: Vec<f64>,
    bins: usize,
    color_hex: u32,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    render_histogram_html(&HistogramConfig {
        title,
        values: &values,
        bins,
        color: color_hex,
        width,
        height,
        hover: &hover_slots,
        ..HistogramConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, overlay_values, bins=0, color_hex=0x4C72B0, overlay_color_hex=0xC44E52, width=860, height=380, hover_json=None))]
fn build_histogram_overlay(
    title: &str,
    values: Vec<f64>,
    overlay_values: Vec<f64>,
    bins: usize,
    color_hex: u32,
    overlay_color_hex: u32,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    render_histogram_html(&HistogramConfig {
        title,
        values: &values,
        bins,
        color: color_hex,
        overlay_values: Some(&overlay_values),
        overlay_color: overlay_color_hex,
        width,
        height,
        hover: &hover_slots,
        ..HistogramConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, width=1100, height=480, hover_json=None))]
fn build_grouped_bar(
    title: &str,
    category_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = series_names
        .into_iter()
        .enumerate()
        .map(|(si, name)| {
            let start = si * n_cats;
            let end = (start + n_cats).min(series_values.len());
            let vals = if start < series_values.len() {
                series_values[start..end].to_vec()
            } else {
                vec![0.0; n_cats]
            };
            (name, vals)
        })
        .collect();
    render_grouped_bar_html(&GroupedBarConfig {
        title,
        category_labels: &category_labels,
        series: &series,
        width,
        height,
        hover: &hover_slots,
        ..GroupedBarConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, width=1100, height=480, hover_json=None))]
fn build_stacked_bar(
    title: &str,
    category_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = series_names
        .into_iter()
        .enumerate()
        .map(|(si, name)| {
            let start = si * n_cats;
            let end = (start + n_cats).min(series_values.len());
            let vals = if start < series_values.len() {
                series_values[start..end].to_vec()
            } else {
                vec![0.0; n_cats]
            };
            (name, vals)
        })
        .collect();
    render_grouped_bar_html(&GroupedBarConfig {
        title,
        category_labels: &category_labels,
        series: &series,
        width,
        height,
        stacked: true,
        hover: &hover_slots,
        ..GroupedBarConfig::default()
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, width=900, height=480, hover_json=None))]
fn build_bar_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::default::render_bars_html(title, &labels, &values, width, height, &hover_slots)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, width=900, height=480, hover_json=None))]
fn build_line_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::default::render_lines_html(title, &labels, &values, width, height, &hover_slots)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, labels=None, width=900, height=540, hover_json=None))]
fn build_scatter_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    labels: Option<Vec<String>>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let empty_labels: Vec<String> = Vec::new();
    let labels_ref = labels.as_deref().unwrap_or(&empty_labels);
    crate::plot::default::render_scatter_html(title, &x_values, &y_values, labels_ref, width, height, &hover_slots)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, width=900, height=560))]
fn build_scatter3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    width: i32,
    height: i32,
) -> String {
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    crate::plot::default::render_scatter3d_html(
        title,
        &x_values, &y_values, &z_values,
        (x_label, y_label, z_label),
        &cv, &cl,
        width, height,
    )
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, width=900, height=560))]
fn build_bar3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    width: i32,
    height: i32,
) -> String {
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    crate::plot::default::render_bar3d_html(
        title,
        &x_values, &y_values, &z_values,
        (x_label, y_label, z_label),
        &cv, &cl,
        width, height,
    )
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, width=900, height=560))]
fn build_line3d_chart(
    title: &str,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    z_values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_values: Option<Vec<f64>>,
    color_labels: Option<Vec<String>>,
    width: i32,
    height: i32,
) -> String {
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    crate::plot::default::render_line3d_html(
        title,
        &x_values, &y_values, &z_values,
        (x_label, y_label, z_label),
        &cv, &cl,
        width, height,
    )
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, width=1200, height=600, hover_json=None))]
fn build_choropleth(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::map::render_choropleth_html(title, &labels, &values, width, height, &hover_slots)
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, width=1200, height=600, hover_json=None))]
fn build_bubble_map(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::map::render_bubble_map_html(title, &labels, &values, width, height, &hover_slots)
}
