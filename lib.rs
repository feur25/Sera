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
    m.add_function(wrap_pyfunction!(build_multiline_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_area_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_treemap, m)?)?;
    m.add_function(wrap_pyfunction!(build_boxplot, m)?)?;
    m.add_function(wrap_pyfunction!(build_hbar, m)?)?;
    m.add_function(wrap_pyfunction!(bench_pure_rust, m)?)?;
    Ok(())
}

/// Run internal benchmarks to measure rendering performance.
///
/// Returns a tuple of (histogram_ms, bar_ms, scatter_ms, heatmap_ms) representing
/// the average time in milliseconds for each chart type over n iterations.
///
/// Args:
///     n (int): Number of iterations per chart type. Default: 2000.
///
/// Returns:
///     tuple[float, float, float, float]: Average render time in ms for (histogram, bar, scatter, heatmap).
///
/// Example:
///     >>> hist, bar, scatter, hm = seraplot.bench_pure_rust(1000)
///     >>> print(f"Bar: {bar:.4f}ms  Scatter: {scatter:.4f}ms")
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

/// Display a chart in the native SeraPlot viewer window.
///
/// Opens a native GUI window rendering the chart described by the JSON payload.
///
/// Args:
///     chart_json (str): JSON string describing the chart (title, labels, values, hover, group).
///
/// Returns:
///     bool: True if the viewer launched successfully.
///
/// Example:
///     >>> import seraplot, json
///     >>> seraplot.show_chart_value(json.dumps({
///     ...     'title': 'Sales', 'labels': ['Q1','Q2'], 'values': [100,200],
///     ...     'hover': [], 'group': 'default'
///     ... }))
#[cfg(feature = "python")]
#[pyfunction]
fn show_chart_value(chart_json: &str) -> bool {
    let chart_json_c = std::ffi::CString::new(chart_json).unwrap_or_default();
    unsafe {
        crate::viewer::chart::sera_show_chart_value(chart_json_c.as_ptr())
    }
}

/// Validate chart JSON without rendering.
///
/// Parses the JSON string to check if it is valid. Useful for pre-validation before rendering.
///
/// Args:
///     chart_json (str): JSON string to validate.
///
/// Returns:
///     bool: True if the JSON is valid.
#[cfg(feature = "python")]
#[pyfunction]
fn bench_chart_value(chart_json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(chart_json).is_ok()
}

/// Set the chart kind for the native viewer.
///
/// Args:
///     kind (int): Chart type identifier (0=bar, 1=line, 2=pie, 3=scatter, etc.).
///
/// Returns:
///     bool: Always True.
#[cfg(feature = "python")]
#[pyfunction]
fn set_chart_kind(kind: u8) -> bool {
    crate::viewer::chart::sera_set_current_chart_kind(kind);
    true
}

/// Set chart orientation for the native viewer.
///
/// Args:
///     vertical (bool): True for vertical, False for horizontal.
///
/// Returns:
///     bool: Always True.
#[cfg(feature = "python")]
#[pyfunction]
fn set_chart_orientation(vertical: bool) -> bool {
    crate::viewer::chart::sera_set_chart_orientation(vertical);
    true
}

/// Build a basic HTML chart using the fast HTML exporter.
///
/// Generates a standalone HTML string containing an SVG chart with hover interactivity.
///
/// Args:
///     title (str): Chart title displayed at the top.
///     labels (list[str]): Category labels for the X axis.
///     values (list[float]): Numeric values corresponding to each label.
///     width (int): SVG width in pixels. Default: 900.
///     height (int): SVG height in pixels. Default: 480.
///
/// Returns:
///     str: Complete HTML string with embedded SVG and JavaScript hover logic.
///
/// Example:
///     >>> html = seraplot.build_html_chart("Revenue", ["Jan","Feb","Mar"], [100, 150, 130])
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

/// Build a pie chart as a standalone HTML string.
///
/// Renders a circular pie chart with optional percentage labels, custom palette,
/// and interactive hover tooltips.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): Slice labels.
///     values (list[float]): Slice values (positive numbers).
///     palette (list[int] | None): Custom colors as hex integers (e.g. [0xFF0000, 0x00FF00]). Default: built-in palette.
///     show_pct (bool): Show percentage text on each slice. Default: True.
///     sort_order (str): Sort slices — "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width in pixels. Default: 720.
///     height (int): SVG height in pixels. Default: 440.
///     hover_json (str | None): JSON string for custom hover tooltips.
///
/// Returns:
///     str: HTML string with the pie chart.
///
/// Example:
///     >>> html = seraplot.build_pie_chart("Market Share", ["Chrome","Firefox","Safari"], [65, 20, 15])
///     >>> html = seraplot.build_pie_chart("Sorted", ["A","B","C"], [30,10,60], sort_order="desc")
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, palette=None, show_pct=true, sort_order="none", width=720, height=440, hover_json=None), text_signature = "(title, labels, values, palette=None, show_pct=True, sort_order='none', width=720, height=440, hover_json=None)")]
fn build_pie_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    palette: Option<Vec<u32>>,
    show_pct: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{PieConfig, render_pie_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    render_pie_html(&PieConfig {
        title,
        labels: &labels,
        values: &values,
        width,
        height,
        show_pct,
        palette: pal_ref,
        hover: &hover_slots,
        sort_order,
        ..PieConfig::default()
    })
}

/// Build a donut chart (pie chart with a hole) as a standalone HTML string.
///
/// Same as build_pie_chart but with an inner radius creating a donut shape.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): Slice labels.
///     values (list[float]): Slice values.
///     inner_radius_ratio (float): Ratio of inner hole (0.0 to 0.9). Default: 0.55.
///     palette (list[int] | None): Custom hex color palette.
///     show_pct (bool): Show percentage on slices. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 720.
///     height (int): SVG height. Default: 440.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the donut chart.
///
/// Example:
///     >>> html = seraplot.build_donut_chart("OS Share", ["Windows","Mac","Linux"], [75, 15, 10])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, inner_radius_ratio=0.55, palette=None, show_pct=true, sort_order="none", width=720, height=440, hover_json=None), text_signature = "(title, labels, values, inner_radius_ratio=0.55, palette=None, show_pct=True, sort_order='none', width=720, height=440, hover_json=None)")]
fn build_donut_chart(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    inner_radius_ratio: f64,
    palette: Option<Vec<u32>>,
    show_pct: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{PieConfig, render_pie_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    render_pie_html(&PieConfig {
        title,
        labels: &labels,
        values: &values,
        width,
        height,
        donut: inner_radius_ratio.clamp(0.0, 0.9),
        show_pct,
        palette: pal_ref,
        hover: &hover_slots,
        sort_order,
        ..PieConfig::default()
    })
}

/// Build a heatmap chart as a standalone HTML string.
///
/// Renders a color-coded matrix with diverging color scale.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): Row labels.
///     flat_matrix (list[float]): Flat array of values (row-major order, len = rows * cols).
///     col_labels (list[str] | None): Column labels. If None, uses row labels.
///     show_values (bool): Display numeric values in each cell. Default: True.
///     color_low (int): Hex color for minimum values. Default: 0x2166ac (blue).
///     color_mid (int): Hex color for midpoint values. Default: 0xffffff (white).
///     color_high (int): Hex color for maximum values. Default: 0xd6604d (red).
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 720.
///     height (int): SVG height. Default: 440.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the heatmap.
///
/// Example:
///     >>> html = seraplot.build_heatmap("Correlation", ["A","B","C"], [1, 0.5, -0.3, 0.5, 1, 0.8, -0.3, 0.8, 1])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, flat_matrix, col_labels=None, show_values=true, color_low=0x2166ac, color_mid=0xffffff, color_high=0xd6604d, sort_order="none", width=720, height=440, hover_json=None), text_signature = "(title, labels, flat_matrix, col_labels=None, show_values=True, color_low=0x2166ac, color_mid=0xffffff, color_high=0xd6604d, sort_order='none', width=720, height=440, hover_json=None)")]
fn build_heatmap(
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
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{HeatmapConfig, render_heatmap_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let cl = col_labels.unwrap_or_default();
    render_heatmap_html(&HeatmapConfig {
        title,
        row_labels: &labels,
        col_labels: if cl.is_empty() { &[] } else { &cl },
        flat_matrix: &flat_matrix,
        show_values,
        color_low,
        color_mid,
        color_high,
        width,
        height,
        hover: &hover_slots,
        ..HeatmapConfig::default()
    })
}

/// Build a histogram as a standalone HTML string.
///
/// Automatically bins continuous data into equal-width intervals.
///
/// Args:
///     title (str): Chart title.
///     values (list[float]): Raw numeric data to bin.
///     bins (int): Number of bins. 0 = auto (Sturges' rule). Default: 0.
///     color_hex (int): Bar color as hex integer. Default: 0x4C72B0.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "Count".
///     show_counts (bool): Display count on top of each bar. Default: False.
///     gridlines (bool): Show horizontal gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 860.
///     height (int): SVG height. Default: 380.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the histogram.
///
/// Example:
///     >>> html = seraplot.build_histogram("Age Distribution", ages, bins=20)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, bins=0, color_hex=0x4C72B0, x_label="", y_label="Count", show_counts=false, gridlines=true, sort_order="none", width=860, height=380, hover_json=None), text_signature = "(title, values, bins=0, color_hex=0x4C72B0, x_label='', y_label='Count', show_counts=False, gridlines=True, sort_order='none', width=860, height=380, hover_json=None)")]
fn build_histogram(
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
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    render_histogram_html(&HistogramConfig {
        title,
        values: &values,
        bins,
        color: color_hex,
        x_label,
        y_label,
        show_counts,
        gridlines,
        width,
        height,
        hover: &hover_slots,
        ..HistogramConfig::default()
    })
}

/// Build an overlay histogram comparing two distributions.
///
/// Renders two semi-transparent histograms on the same axes for visual comparison.
///
/// Args:
///     title (str): Chart title.
///     values (list[float]): Primary distribution data.
///     overlay_values (list[float]): Secondary distribution data.
///     bins (int): Number of bins. 0 = auto. Default: 0.
///     color_hex (int): Primary color. Default: 0x4C72B0.
///     overlay_color_hex (int): Overlay color. Default: 0xC44E52.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 860.
///     height (int): SVG height. Default: 380.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with both histograms overlaid.
///
/// Example:
///     >>> html = seraplot.build_histogram_overlay("Age by Gender", male_ages, female_ages, bins=15)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, overlay_values, bins=0, color_hex=0x4C72B0, overlay_color_hex=0xC44E52, sort_order="none", width=860, height=380, hover_json=None), text_signature = "(title, values, overlay_values, bins=0, color_hex=0x4C72B0, overlay_color_hex=0xC44E52, sort_order='none', width=860, height=380, hover_json=None)")]
fn build_histogram_overlay(
    title: &str,
    values: Vec<f64>,
    overlay_values: Vec<f64>,
    bins: usize,
    color_hex: u32,
    overlay_color_hex: u32,
    sort_order: &str,
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

/// Build a grouped bar chart as a standalone HTML string.
///
/// Renders multiple series as side-by-side bars for each category. Supports sorting
/// categories by total value or alphabetically.
///
/// Args:
///     title (str): Chart title.
///     category_labels (list[str]): Category names for the X axis.
///     series_names (list[str]): Name of each series (appears in legend).
///     series_values (list[float]): Flat array of values. Layout: [series0_cat0, series0_cat1, ..., series1_cat0, ...].
///     palette (list[int] | None): Custom hex colors. Default: built-in palette.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     show_values (bool): Display values above bars. Default: False.
///     gridlines (bool): Show horizontal gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1100.
///     height (int): SVG height. Default: 480.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the grouped bar chart.
///
/// Example:
///     >>> html = seraplot.build_grouped_bar("Sales", ["Q1","Q2"], ["Product A","Product B"], [10,20,15,25])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, palette=None, x_label="", y_label="", show_values=false, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None), text_signature = "(title, category_labels, series_names, series_values, palette=None, x_label='', y_label='', show_values=False, gridlines=True, sort_order='none', width=1100, height=480, hover_json=None)")]
fn build_grouped_bar(
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
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
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
        x_label,
        y_label,
        category_labels: &category_labels,
        series: &series,
        palette: pal_ref,
        show_values,
        gridlines,
        width,
        height,
        hover: &hover_slots,
        sort_order,
        ..GroupedBarConfig::default()
    })
}

/// Build a stacked bar chart as a standalone HTML string.
///
/// Same layout as build_grouped_bar but with bars stacked on top of each other.
///
/// Args:
///     title (str): Chart title.
///     category_labels (list[str]): Category names.
///     series_names (list[str]): Series names (legend entries).
///     series_values (list[float]): Flat value array [s0_c0, s0_c1, ..., s1_c0, ...].
///     palette (list[int] | None): Custom hex colors.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     show_values (bool): Display values on bars. Default: False.
///     gridlines (bool): Show gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1100.
///     height (int): SVG height. Default: 480.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the stacked bar chart.
///
/// Example:
///     >>> html = seraplot.build_stacked_bar("Revenue", ["2022","2023"], ["Online","Store"], [50,60,40,55])
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, palette=None, x_label="", y_label="", show_values=false, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None), text_signature = "(title, category_labels, series_names, series_values, palette=None, x_label='', y_label='', show_values=False, gridlines=True, sort_order='none', width=1100, height=480, hover_json=None)")]
fn build_stacked_bar(
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
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
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
        x_label,
        y_label,
        category_labels: &category_labels,
        series: &series,
        palette: pal_ref,
        show_values,
        gridlines,
        width,
        height,
        stacked: true,
        hover: &hover_slots,
        sort_order,
        ..GroupedBarConfig::default()
    })
}

/// Build a bar chart (vertical or horizontal) as a standalone HTML string.
///
/// Supports single-color bars, color groups (one color per group), optional value labels,
/// axis labels, gridlines, and data sorting.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): Category labels.
///     values (list[float]): Numeric values for each bar.
///     orientation (str): "v" for vertical (default), "h" for horizontal.
///     color_groups (list[str] | None): Group name per bar for automatic coloring.
///     show_text (bool): Show value text on bars. Default: False.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     palette (list[int] | None): Custom hex color palette.
///     color_hex (int): Single color for all bars if no groups. Default: 0 (auto).
///     gridlines (bool): Show gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 900.
///     height (int): SVG height. Default: 480.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the bar chart.
///
/// Example:
///     >>> html = seraplot.build_bar_chart("Top Cities", ["Paris","London","NYC"], [2.1, 8.9, 8.3])
///     >>> html = seraplot.build_bar_chart("Sorted", labels, values, sort_order="desc")
///     >>> html = seraplot.build_bar_chart("Horizontal", labels, values, orientation="h")
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, orientation="v", color_groups=None, show_text=false, x_label="", y_label="", palette=None, color_hex=0, gridlines=true, sort_order="none", width=900, height=480, hover_json=None), text_signature = "(title, labels, values, orientation='v', color_groups=None, show_text=False, x_label='', y_label='', palette=None, color_hex=0, gridlines=True, sort_order='none', width=900, height=480, hover_json=None)")]
fn build_bar_chart(
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
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let cg = color_groups.unwrap_or_default();
    let ori = if orientation.starts_with('h') || orientation.starts_with('H') { b'h' } else { b'v' };
    crate::plot::default::render_bars_html(
        title, &labels, &values, width, height, &hover_slots,
        ori, &cg, show_text, x_label, y_label, &pal, color_hex, gridlines, sort_order,
    )
}

/// Build a single-series line chart as a standalone HTML string.
///
/// Renders a connected line with optional data points and gridlines.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): X-axis labels (typically dates or categories).
///     values (list[float]): Y values for each point.
///     color_hex (int): Line color as hex. Default: 0x4C72B0.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     gridlines (bool): Show gridlines. Default: True.
///     show_points (bool): Show circles at data points. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 900.
///     height (int): SVG height. Default: 480.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the line chart.
///
/// Example:
///     >>> html = seraplot.build_line_chart("Temperature", months, temps, color_hex=0xDD8452)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, color_hex=0x4C72B0, x_label="", y_label="", gridlines=true, show_points=true, sort_order="none", width=900, height=480, hover_json=None), text_signature = "(title, labels, values, color_hex=0x4C72B0, x_label='', y_label='', gridlines=True, show_points=True, sort_order='none', width=900, height=480, hover_json=None)")]
fn build_line_chart(
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
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::default::render_lines_html(title, &labels, &values, width, height, &hover_slots, color_hex, x_label, y_label, gridlines, show_points)
}

/// Build a scatter chart as a standalone HTML string.
///
/// Renders points at (x, y) coordinates with optional sizes, color groups, and labels.
///
/// Args:
///     title (str): Chart title.
///     x_values (list[float]): X coordinates.
///     y_values (list[float]): Y coordinates.
///     labels (list[str] | None): Point labels for hover. Default: None.
///     palette (list[int] | None): Custom hex color palette.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     color_hex (int): Single color for all points. Default: 0 (auto).
///     gridlines (bool): Show gridlines. Default: True.
///     show_text (bool): Display labels next to each point. Default: False.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 900.
///     height (int): SVG height. Default: 540.
///     hover_json (str | None): Custom hover tooltips JSON.
///     sizes (list[float] | None): Bubble sizes for each point.
///     color_groups (list[str] | None): Group name per point for coloring.
///
/// Returns:
///     str: HTML string with the scatter chart.
///
/// Example:
///     >>> html = seraplot.build_scatter_chart("Height vs Weight", heights, weights, x_label="Height", y_label="Weight")
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, labels=None, palette=None, x_label="", y_label="", color_hex=0, gridlines=true, show_text=false, sort_order="none", width=900, height=540, hover_json=None, sizes=None, color_groups=None), text_signature = "(title, x_values, y_values, labels=None, palette=None, x_label='', y_label='', color_hex=0, gridlines=True, show_text=False, sort_order='none', width=900, height=540, hover_json=None, sizes=None, color_groups=None)")]
fn build_scatter_chart(
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
    hover_json: Option<String>,
    sizes: Option<Vec<f64>>,
    color_groups: Option<Vec<String>>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let empty_labels: Vec<String> = Vec::new();
    let labels_ref = labels.as_deref().unwrap_or(&empty_labels);
    let empty_sizes: Vec<f64> = Vec::new();
    let sizes_ref = sizes.as_deref().unwrap_or(&empty_sizes);
    let empty_groups: Vec<String> = Vec::new();
    let groups_ref = color_groups.as_deref().unwrap_or(&empty_groups);
    let pal = palette.unwrap_or_default();
    crate::plot::default::render_scatter_html(title, &x_values, &y_values, labels_ref, width, height, &hover_slots, sizes_ref, groups_ref, &pal, x_label, y_label, color_hex, gridlines, show_text)
}

/// Build a 3D scatter chart as a standalone HTML string.
///
/// Renders an interactive 3D scatter plot using WebGL with rotation and zoom controls.
///
/// Args:
///     title (str): Chart title.
///     x_values (list[float]): X coordinates.
///     y_values (list[float]): Y coordinates.
///     z_values (list[float]): Z coordinates.
///     x_label (str): X-axis label. Default: "X".
///     y_label (str): Y-axis label. Default: "Y".
///     z_label (str): Z-axis label. Default: "Z".
///     color_values (list[float] | None): Values for color mapping.
///     color_labels (list[str] | None): Labels for color legend.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): Canvas width. Default: 900.
///     height (int): Canvas height. Default: 560.
///
/// Returns:
///     str: HTML string with the 3D scatter chart.
///
/// Example:
///     >>> html = seraplot.build_scatter3d_chart("3D Clusters", x, y, z)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Z', color_values=None, color_labels=None, sort_order='none', width=900, height=560)")]
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
    sort_order: &str,
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

/// Build a 3D bar chart as a standalone HTML string.
///
/// Renders 3D bars using WebGL with rotation and zoom.
///
/// Args:
///     title (str): Chart title.
///     x_values (list[float]): X positions.
///     y_values (list[float]): Y positions.
///     z_values (list[float]): Bar heights (Z axis).
///     x_label (str): X-axis label. Default: "X".
///     y_label (str): Y-axis label. Default: "Y".
///     z_label (str): Z-axis label. Default: "Z".
///     color_values (list[float] | None): Values for color mapping.
///     color_labels (list[str] | None): Labels for color legend.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): Canvas width. Default: 900.
///     height (int): Canvas height. Default: 560.
///
/// Returns:
///     str: HTML string with the 3D bar chart.
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Z', color_values=None, color_labels=None, sort_order='none', width=900, height=560)")]
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
    sort_order: &str,
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

/// Build a 3D line chart as a standalone HTML string.
///
/// Renders a 3D line path using WebGL with rotation and zoom.
///
/// Args:
///     title (str): Chart title.
///     x_values (list[float]): X coordinates.
///     y_values (list[float]): Y coordinates.
///     z_values (list[float]): Z coordinates.
///     x_label (str): X-axis label. Default: "X".
///     y_label (str): Y-axis label. Default: "Y".
///     z_label (str): Z-axis label. Default: "Z".
///     color_values (list[float] | None): Values for color mapping.
///     color_labels (list[str] | None): Labels for color legend.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): Canvas width. Default: 900.
///     height (int): Canvas height. Default: 560.
///
/// Returns:
///     str: HTML string with the 3D line chart.
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Z', color_values=None, color_labels=None, sort_order='none', width=900, height=560)")]
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
    sort_order: &str,
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

/// Build a multi-series line chart as a standalone HTML string.
///
/// Renders multiple lines on the same axes with a legend. Supports toggle-to-hide series
/// via the interactive legend (with spRescale for Y-axis auto-rescaling).
///
/// Args:
///     title (str): Chart title.
///     x_labels (list[str]): Shared X-axis labels.
///     series_names (list[str]): Name for each line series.
///     series_values (list[float]): Flat array [s0_x0, s0_x1, ..., s1_x0, ...].
///     palette (list[int] | None): Custom hex colors.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     show_points (bool): Show circles on data points. Default: True.
///     gridlines (bool): Show gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1100.
///     height (int): SVG height. Default: 480.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the multiline chart.
///
/// Example:
///     >>> html = seraplot.build_multiline_chart("Trends", months, ["2022","2023"], values_flat)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_names, series_values, palette=None, x_label="", y_label="", show_points=true, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None), text_signature = "(title, x_labels, series_names, series_values, palette=None, x_label='', y_label='', show_points=True, gridlines=True, sort_order='none', width=1100, height=480, hover_json=None)")]
fn build_multiline_chart(
    title: &str,
    x_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<f64>,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    show_points: bool,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{MultiLineConfig, render_multiline_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_pts = x_labels.len();
    let n_ser = series_names.len();
    if n_pts == 0 || n_ser == 0 { return String::new(); }
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    let series: Vec<(String, Vec<f64>)> = series_names
        .into_iter()
        .enumerate()
        .map(|(si, name)| {
            let start = si * n_pts;
            let end = (start + n_pts).min(series_values.len());
            let vals = if start < series_values.len() {
                series_values[start..end].to_vec()
            } else {
                vec![0.0; n_pts]
            };
            (name, vals)
        })
        .collect();
    render_multiline_html(&MultiLineConfig {
        title,
        x_label,
        y_label,
        x_labels: &x_labels,
        series: &series,
        palette: pal_ref,
        show_points,
        gridlines,
        width,
        height,
        hover: &hover_slots,
        ..MultiLineConfig::default()
    })
}

/// Build an area chart as a standalone HTML string.
///
/// Renders filled areas under each series line. Supports stacked mode where areas
/// are layered on top of each other.
///
/// Args:
///     title (str): Chart title.
///     x_labels (list[str]): Shared X-axis labels.
///     series_names (list[str]): Name for each area series.
///     series_values (list[float]): Flat array [s0_x0, s0_x1, ..., s1_x0, ...].
///     stacked (bool): If True, areas are stacked. Default: False.
///     palette (list[int] | None): Custom hex colors.
///     x_label (str): X-axis label. Default: "".
///     y_label (str): Y-axis label. Default: "".
///     gridlines (bool): Show gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1100.
///     height (int): SVG height. Default: 480.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the area chart.
///
/// Example:
///     >>> html = seraplot.build_area_chart("Traffic", months, ["Desktop","Mobile"], vals, stacked=True)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_names, series_values, stacked=false, palette=None, x_label="", y_label="", gridlines=true, sort_order="none", width=1100, height=480, hover_json=None), text_signature = "(title, x_labels, series_names, series_values, stacked=False, palette=None, x_label='', y_label='', gridlines=True, sort_order='none', width=1100, height=480, hover_json=None)")]
fn build_area_chart(
    title: &str,
    x_labels: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<f64>,
    stacked: bool,
    palette: Option<Vec<u32>>,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{AreaConfig, render_area_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_pts = x_labels.len();
    let n_ser = series_names.len();
    if n_pts == 0 || n_ser == 0 { return String::new(); }
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    let series: Vec<(String, Vec<f64>)> = series_names
        .into_iter()
        .enumerate()
        .map(|(si, name)| {
            let start = si * n_pts;
            let end = (start + n_pts).min(series_values.len());
            let vals = if start < series_values.len() {
                series_values[start..end].to_vec()
            } else {
                vec![0.0; n_pts]
            };
            (name, vals)
        })
        .collect();
    render_area_html(&AreaConfig {
        title,
        x_label,
        y_label,
        x_labels: &x_labels,
        series: &series,
        palette: pal_ref,
        stacked,
        gridlines,
        width,
        height,
        hover: &hover_slots,
        ..AreaConfig::default()
    })
}

/// Build a treemap chart as a standalone HTML string.
///
/// Renders a squarify treemap layout with nested rectangles proportional to values.
/// Supports parent-child hierarchy for color grouping.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): Node labels.
///     values (list[float]): Node values (positive numbers).
///     parents (list[str] | None): Parent group for each node (for color grouping).
///     palette (list[int] | None): Custom hex colors.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1100.
///     height (int): SVG height. Default: 520.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the treemap.
///
/// Example:
///     >>> html = seraplot.build_treemap("Disk Usage", ["Photos","Videos","Docs"], [4500,2300,800])
///     >>> html = seraplot.build_treemap("Files", labels, values, parents=categories, sort_order="desc")
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, parents=None, palette=None, sort_order="none", width=1100, height=520, hover_json=None), text_signature = "(title, labels, values, parents=None, palette=None, sort_order='none', width=1100, height=520, hover_json=None)")]
fn build_treemap(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    parents: Option<Vec<String>>,
    palette: Option<Vec<u32>>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{TreemapConfig, render_treemap_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let parents_vec = parents.unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    render_treemap_html(&TreemapConfig {
        title,
        labels: &labels,
        values: &values,
        parents: &parents_vec,
        palette: pal_ref,
        width,
        height,
        hover: &hover_slots,
        sort_order,
        ..TreemapConfig::default()
    })
}

/// Build a box plot as a standalone HTML string.
///
/// Computes quartiles, median, whiskers, and outliers for each category automatically.
/// Values are split evenly across categories.
///
/// Args:
///     title (str): Chart title.
///     category_labels (list[str]): Category names for the X axis.
///     values (list[float]): Flat array of all values, split evenly across categories.
///     palette (list[int] | None): Custom hex colors per category.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 900.
///     height (int): SVG height. Default: 500.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the box plot.
///
/// Example:
///     >>> html = seraplot.build_boxplot("Score by Class", ["A","B","C"], all_scores)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, values, palette=None, sort_order="none", width=900, height=500, hover_json=None), text_signature = "(title, category_labels, values, palette=None, sort_order='none', width=900, height=500, hover_json=None)")]
fn build_boxplot(
    title: &str,
    category_labels: Vec<String>,
    values: Vec<f64>,
    palette: Option<Vec<u32>>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::plot::statistical::{render_boxplot_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    render_boxplot_html(title, &category_labels, &values, width, height, pal_ref, &hover_slots)
}

/// Build a horizontal bar chart as a standalone HTML string.
///
/// Shortcut for build_bar_chart with orientation="h". Kept for backward compatibility.
///
/// Args:
///     title (str): Chart title.
///     labels (list[str]): Category labels (Y axis for horizontal bars).
///     values (list[float]): Bar values.
///     color_groups (list[str] | None): Group name per bar for coloring.
///     show_text (bool): Show value text on bars. Default: True.
///     x_label (str): X-axis label (value axis). Default: "".
///     palette (list[int] | None): Custom hex colors.
///     color_hex (int): Single color. Default: 0 (auto).
///     gridlines (bool): Show gridlines. Default: True.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 900.
///     height (int): SVG height. Default: 500.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the horizontal bar chart.
///
/// Example:
///     >>> html = seraplot.build_hbar("Top Languages", langs, popularity, sort_order="desc")
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, color_groups=None, show_text=true, x_label="", palette=None, color_hex=0, gridlines=true, sort_order="none", width=900, height=500, hover_json=None), text_signature = "(title, labels, values, color_groups=None, show_text=True, x_label='', palette=None, color_hex=0, gridlines=True, sort_order='none', width=900, height=500, hover_json=None)")]
fn build_hbar(
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
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let cg = color_groups.unwrap_or_default();
    crate::plot::default::render_bars_html(
        title, &labels, &values, width, height, &hover_slots,
        b'h', &cg, show_text, x_label, "", &pal, color_hex, gridlines, sort_order,
    )
}

/// Build a choropleth world map as a standalone HTML string.
///
/// Colors countries based on values using a diverging color scale.
///
/// Args:
///     title (str): Map title.
///     labels (list[str]): Country names or ISO codes.
///     values (list[float]): Values per country.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1200.
///     height (int): SVG height. Default: 600.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the choropleth map.
///
/// Example:
///     >>> html = seraplot.build_choropleth("GDP per Capita", countries, gdp_values)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, sort_order="none", width=1200, height=600, hover_json=None), text_signature = "(title, labels, values, sort_order='none', width=1200, height=600, hover_json=None)")]
fn build_choropleth(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::map::render_choropleth_html(title, &labels, &values, width, height, &hover_slots)
}

/// Build a bubble map as a standalone HTML string.
///
/// Places sized bubbles on a world map at country locations.
///
/// Args:
///     title (str): Map title.
///     labels (list[str]): Country names or ISO codes.
///     values (list[float]): Values controlling bubble size.
///     sort_order (str): "asc", "desc", "alpha", "alpha_desc", or "none". Default: "none".
///     width (int): SVG width. Default: 1200.
///     height (int): SVG height. Default: 600.
///     hover_json (str | None): Custom hover tooltips JSON.
///
/// Returns:
///     str: HTML string with the bubble map.
///
/// Example:
///     >>> html = seraplot.build_bubble_map("Population", countries, populations)
#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, sort_order="none", width=1200, height=600, hover_json=None), text_signature = "(title, labels, values, sort_order='none', width=1200, height=600, hover_json=None)")]
fn build_bubble_map(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    sort_order: &str,
    width: i32,
    height: i32,
    hover_json: Option<String>,
) -> String {
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::map::render_bubble_map_html(title, &labels, &values, width, height, &hover_slots)
}
