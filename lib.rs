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
#[pyclass(module = "seraplot")]
pub struct Chart {
    html: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl Chart {
    #[getter]
    fn html(&self) -> &str {
        &self.html
    }

    fn _repr_html_(&self) -> &str {
        &self.html
    }

    fn __str__(&self) -> &str {
        &self.html
    }

    fn __repr__(&self) -> String {
        format!("SeraPlot.Chart({} bytes)", self.html.len())
    }

    fn __len__(&self) -> usize {
        self.html.len()
    }

    fn __bool__(&self) -> bool {
        !self.html.is_empty()
    }

    fn show(&self, py: Python<'_>) -> PyResult<()> {
        let ipython = py.import("IPython.display")?;
        let html_cls = ipython.getattr("HTML")?;
        let display_fn = ipython.getattr("display")?;
        let html_obj = html_cls.call1((self.html.as_str(),))?;
        display_fn.call1((html_obj,))?;
        Ok(())
    }

    #[pyo3(signature = (path))]
    fn save(&self, path: &str) -> PyResult<()> {
        std::fs::write(path, &self.html)?;
        Ok(())
    }

    #[pyo3(signature = (color=None))]
    fn set_bg(&self, color: Option<&str>) -> Chart {
        Chart {
            html: crate::html::hover::apply_bg(self.html.clone(), color),
        }
    }
}

#[cfg(feature = "python")]
impl Chart {
    fn new(html: String) -> Self {
        Self { html }
    }
}

#[cfg(feature = "python")]
#[pymodule]
fn seraplot(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Chart>()?;
    m.add("__version__", VERSION)?;
    m.add("__doc__", r#"SeraPlot - Rust-Powered Data Visualization Framework

SeraPlot is a framework developed in Rust, meticulously crafted with care. It is a modern 
alternative to Plotly, designed specifically for data visualization. This library is distributed 
across multiple programming languages (Python, C#, C++, JavaScript), regularly maintained and 
updated, offering superior speed and significantly lower memory consumption compared to competitors.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 COVER: src/asset/cover.png
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Submodules
  seraplot.charts   — bar, line, scatter, hbar
  seraplot.stats    — histogram, grouped_bar, violin, heatmap, pie, …
  seraplot.geo      — choropleth, bubble_map
  seraplot.three_d  — scatter3d, bar3d, line3d
  seraplot.engine   — show_chart_value, bench, set_bg, …

Usage
  >>> from seraplot.stats import build_histogram
  >>> from seraplot.charts import build_bar_chart
  >>> import seraplot          # backward compatible — all functions at top level
"#)?;

    let charts = PyModule::new(py, "charts")?;
    charts.add_class::<Chart>()?;
    charts.add_function(wrap_pyfunction!(build_bar_chart, charts)?)?;
    charts.add_function(wrap_pyfunction!(build_line_chart, charts)?)?;
    charts.add_function(wrap_pyfunction!(build_scatter_chart, charts)?)?;
    charts.add_function(wrap_pyfunction!(build_hbar, charts)?)?;
    m.add_submodule(charts)?;

    let stats = PyModule::new(py, "stats")?;
    stats.add_class::<Chart>()?;
    stats.add_function(wrap_pyfunction!(build_pie_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_donut_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_heatmap, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_histogram, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_histogram_overlay, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_grouped_bar, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_stacked_bar, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_multiline_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_area_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_treemap, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_boxplot, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_funnel, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_sunburst, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_waterfall, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_violin, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_slope, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_bullet, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_wordcloud, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_radar_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_lollipop_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_kde_chart, stats)?)?;
    stats.add_function(wrap_pyfunction!(build_ridgeline_chart, stats)?)?;
    m.add_submodule(stats)?;

    let geo = PyModule::new(py, "geo")?;
    geo.add_class::<Chart>()?;
    geo.add_function(wrap_pyfunction!(build_choropleth, geo)?)?;
    geo.add_function(wrap_pyfunction!(build_bubble_map, geo)?)?;
    m.add_submodule(geo)?;

    let three_d = PyModule::new(py, "three_d")?;
    three_d.add_class::<Chart>()?;
    three_d.add_function(wrap_pyfunction!(build_scatter3d_chart, three_d)?)?;
    three_d.add_function(wrap_pyfunction!(build_bar3d_chart, three_d)?)?;
    three_d.add_function(wrap_pyfunction!(build_line3d_chart, three_d)?)?;
    three_d.add_function(wrap_pyfunction!(build_radar3d_chart, three_d)?)?;
    three_d.add_function(wrap_pyfunction!(build_lollipop3d_chart, three_d)?)?;
    three_d.add_function(wrap_pyfunction!(build_kde3d_chart, three_d)?)?;
    three_d.add_function(wrap_pyfunction!(build_ridgeline3d_chart, three_d)?)?;
    m.add_submodule(three_d)?;

    let engine = PyModule::new(py, "engine")?;
    engine.add_function(wrap_pyfunction!(show_chart_value, engine)?)?;
    engine.add_function(wrap_pyfunction!(bench_chart_value, engine)?)?;
    engine.add_function(wrap_pyfunction!(set_chart_kind, engine)?)?;
    engine.add_function(wrap_pyfunction!(set_chart_orientation, engine)?)?;
    engine.add_function(wrap_pyfunction!(build_html_chart, engine)?)?;
    engine.add_function(wrap_pyfunction!(set_bg_fn, engine)?)?;
    engine.add_function(wrap_pyfunction!(bench_pure_rust, engine)?)?;
    m.add_submodule(engine)?;

    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.set_item("seraplot.charts", charts)?;
    sys_modules.set_item("seraplot.stats", stats)?;
    sys_modules.set_item("seraplot.geo", geo)?;
    sys_modules.set_item("seraplot.three_d", three_d)?;
    sys_modules.set_item("seraplot.engine", engine)?;

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
    m.add_function(wrap_pyfunction!(build_funnel, m)?)?;
    m.add_function(wrap_pyfunction!(build_sunburst, m)?)?;
    m.add_function(wrap_pyfunction!(build_waterfall, m)?)?;
    m.add_function(wrap_pyfunction!(build_violin, m)?)?;
    m.add_function(wrap_pyfunction!(build_slope, m)?)?;
    m.add_function(wrap_pyfunction!(build_bullet, m)?)?;
    m.add_function(wrap_pyfunction!(build_wordcloud, m)?)?;
    m.add_function(wrap_pyfunction!(build_radar_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_radar3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_lollipop_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_lollipop3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_kde_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_kde3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_ridgeline_chart, m)?)?;
    m.add_function(wrap_pyfunction!(build_ridgeline3d_chart, m)?)?;
    m.add_function(wrap_pyfunction!(set_bg_fn, m)?)?;
    m.add_function(wrap_pyfunction!(bench_pure_rust, m)?)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(name = "set_bg", signature = (html, color=None), text_signature = "(html, color=None)")]
fn set_bg_fn(html: &PyAny, color: Option<&str>) -> PyResult<Chart> {
    let raw: String = if let Ok(chart) = html.extract::<PyRef<Chart>>() {
        chart.html.clone()
    } else {
        html.extract::<String>()?
    };
    Ok(Chart::new(crate::html::hover::apply_bg(raw, color)))
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
) -> Chart {
    Chart::new({
    let exporter = crate::html::FastHtmlExporter::new(width, height, title.to_string());
    exporter.build_optimized(labels, values)
    })
}

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
) -> Chart {
    Chart::new({
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
    })
}

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
) -> Chart {
    Chart::new({
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
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, flat_matrix, col_labels=None, show_values=true, color_low=0x6366F1, color_mid=0xfafbfc, color_high=0xF43F5E, sort_order="none", width=720, height=440, hover_json=None, bg_color=None), text_signature = "(title, labels, flat_matrix, col_labels=None, show_values=True, color_low=0x6366F1, color_mid=0xfafbfc, color_high=0xF43F5E, sort_order='none', width=720, height=440, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{HeatmapConfig, render_heatmap_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let cl = col_labels.unwrap_or_default();
    crate::html::hover::apply_bg(render_heatmap_html(&HeatmapConfig {
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
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, bins=0, color_hex=0x6366F1, x_label="", y_label="Count", show_counts=false, gridlines=true, sort_order="none", width=860, height=380, hover_json=None, bg_color=None), text_signature = "(title, values, bins=0, color_hex=0x6366F1, x_label='', y_label='Count', show_counts=False, gridlines=True, sort_order='none', width=860, height=380, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::html::hover::apply_bg(render_histogram_html(&HistogramConfig {
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
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, overlay_values, bins=0, color_hex=0x6366F1, overlay_color_hex=0xF43F5E, sort_order="none", width=860, height=380, hover_json=None, bg_color=None), text_signature = "(title, values, overlay_values, bins=0, color_hex=0x6366F1, overlay_color_hex=0xF43F5E, sort_order='none', width=860, height=380, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{HistogramConfig, render_histogram_html, parse_hover_json};
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::html::hover::apply_bg(render_histogram_html(&HistogramConfig {
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
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, palette=None, x_label="", y_label="", show_values=false, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None, bg_color=None), text_signature = "(title, category_labels, series_names, series_values, palette=None, x_label='', y_label='', show_values=False, gridlines=True, sort_order='none', width=1100, height=480, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    if n_cats == 0 || n_ser == 0 { return Chart::new(String::new()); }
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
    crate::html::hover::apply_bg(render_grouped_bar_html(&GroupedBarConfig {
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
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, category_labels, series_names, series_values, palette=None, x_label="", y_label="", show_values=false, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None, bg_color=None), text_signature = "(title, category_labels, series_names, series_values, palette=None, x_label='', y_label='', show_values=False, gridlines=True, sort_order='none', width=1100, height=480, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_cats = category_labels.len();
    let n_ser = series_names.len();
    if n_cats == 0 || n_ser == 0 { return Chart::new(String::new()); }
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
    crate::html::hover::apply_bg(render_grouped_bar_html(&GroupedBarConfig {
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
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, orientation="v", color_groups=None, show_text=false, x_label="", y_label="", palette=None, color_hex=0, gridlines=true, sort_order="none", width=900, height=480, hover_json=None, bg_color=None), text_signature = "(title, labels, values, orientation='v', color_groups=None, show_text=False, x_label='', y_label='', palette=None, color_hex=0, gridlines=True, sort_order='none', width=900, height=480, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let cg = color_groups.unwrap_or_default();
    let ori = if orientation.starts_with('h') || orientation.starts_with('H') { b'h' } else { b'v' };
    crate::html::hover::apply_bg(crate::plot::default::render_bars_html(
        title, &labels, &values, width, height, &hover_slots,
        ori, &cg, show_text, x_label, y_label, &pal, color_hex, gridlines, sort_order,
    ), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, color_hex=0x6366F1, x_label="", y_label="", gridlines=true, show_points=true, sort_order="none", width=900, height=480, hover_json=None, bg_color=None), text_signature = "(title, labels, values, color_hex=0x6366F1, x_label='', y_label='', gridlines=True, show_points=True, sort_order='none', width=900, height=480, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::html::hover::apply_bg(crate::plot::default::render_lines_html(title, &labels, &values, width, height, &hover_slots, color_hex, x_label, y_label, gridlines, show_points), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, labels=None, palette=None, x_label="", y_label="", color_hex=0, gridlines=true, show_text=false, sort_order="none", width=900, height=540, hover_json=None, sizes=None, color_groups=None, bg_color=None), text_signature = "(title, x_values, y_values, labels=None, palette=None, x_label='', y_label='', color_hex=0, gridlines=True, show_text=False, sort_order='none', width=900, height=540, hover_json=None, sizes=None, color_groups=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let empty_labels: Vec<String> = Vec::new();
    let labels_ref = labels.as_deref().unwrap_or(&empty_labels);
    let empty_sizes: Vec<f64> = Vec::new();
    let sizes_ref = sizes.as_deref().unwrap_or(&empty_sizes);
    let empty_groups: Vec<String> = Vec::new();
    let groups_ref = color_groups.as_deref().unwrap_or(&empty_groups);
    let pal = palette.unwrap_or_default();
    crate::html::hover::apply_bg(crate::plot::default::render_scatter_html(title, &x_values, &y_values, labels_ref, width, height, &hover_slots, sizes_ref, groups_ref, &pal, x_label, y_label, color_hex, gridlines, show_text), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560, bg_color=None), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Z', color_values=None, color_labels=None, sort_order='none', width=900, height=560, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    crate::plot::default::render_scatter3d_html(
        title,
        &x_values, &y_values, &z_values,
        (x_label, y_label, z_label),
        &cv, &cl,
        width, height, bg_color.as_deref(),
    )
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560, bg_color=None), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Z', color_values=None, color_labels=None, sort_order='none', width=900, height=560, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    crate::plot::default::render_bar3d_html(
        title,
        &x_values, &y_values, &z_values,
        (x_label, y_label, z_label),
        &cv, &cl,
        width, height, bg_color.as_deref(),
    )
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Z", color_values=None, color_labels=None, sort_order="none", width=900, height=560, bg_color=None), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Z', color_values=None, color_labels=None, sort_order='none', width=900, height=560, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    let cv = color_values.unwrap_or_default();
    let cl = color_labels.unwrap_or_default();
    crate::plot::default::render_line3d_html(
        title,
        &x_values, &y_values, &z_values,
        (x_label, y_label, z_label),
        &cv, &cl,
        width, height, bg_color.as_deref(),
    )
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_labels, series_names, series_values, palette=None, x_label="", y_label="", show_points=true, gridlines=true, sort_order="none", width=1100, height=480, hover_json=None, bg_color=None), text_signature = "(title, x_labels, series_names, series_values, palette=None, x_label='', y_label='', show_points=True, gridlines=True, sort_order='none', width=1100, height=480, hover_json=None, bg_color=None)")]
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{MultiLineConfig, render_multiline_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_pts = x_labels.len();
    let n_ser = series_names.len();
    if n_pts == 0 || n_ser == 0 { return Chart::new(String::new()); }
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
    crate::html::hover::apply_bg(render_multiline_html(&MultiLineConfig {
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
    }), bg_color.as_deref())
    })
}

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
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{AreaConfig, render_area_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let n_pts = x_labels.len();
    let n_ser = series_names.len();
    if n_pts == 0 || n_ser == 0 { return Chart::new(String::new()); }
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
    })
}

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
) -> Chart {
    Chart::new({
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
    })
}

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
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{render_boxplot_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    render_boxplot_html(title, &category_labels, &values, width, height, pal_ref, &hover_slots)
    })
}

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
) -> Chart {
    Chart::new({
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let cg = color_groups.unwrap_or_default();
    crate::plot::default::render_bars_html(
        title, &labels, &values, width, height, &hover_slots,
        b'h', &cg, show_text, x_label, "", &pal, color_hex, gridlines, sort_order,
    )
    })
}

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
) -> Chart {
    Chart::new({
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::map::render_choropleth_html(title, &labels, &values, width, height, &hover_slots)
    })
}

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
) -> Chart {
    Chart::new({
    use crate::html::hover::parse_hover_json;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    crate::plot::map::render_bubble_map_html(title, &labels, &values, width, height, &hover_slots)
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, sort_order="none", show_text=true, palette=None, width=800, height=480, bg_color=None), text_signature = "(title, labels, values, sort_order='none', show_text=True, palette=None, width=800, height=480, bg_color=None)")]
fn build_funnel(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    sort_order: &str,
    show_text: bool,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{FunnelConfig, render_funnel_html};
    use crate::plot::statistical::common::{apply_sort, PALETTE};
    let (labels, values) = apply_sort(&labels, &values, sort_order);
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    crate::html::hover::apply_bg(render_funnel_html(&FunnelConfig {
        title,
        labels: &labels,
        values: &values,
        palette: pal_ref,
        show_text,
        width,
        height,
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, parents, values, width=700, height=700, bg_color=None), text_signature = "(title, labels, parents, values, width=700, height=700, bg_color=None)")]
fn build_sunburst(
    title: &str,
    labels: Vec<String>,
    parents: Vec<String>,
    values: Vec<f64>,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{SunburstConfig, render_sunburst_html};
    crate::html::hover::apply_bg(render_sunburst_html(&SunburstConfig {
        title,
        labels: &labels,
        parents: &parents,
        values: &values,
        width,
        height,
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, x_label="", y_label="", show_text=true, gridlines=true, width=900, height=480), text_signature = "(title, labels, values, x_label='', y_label='', show_text=True, gridlines=True, width=900, height=480)")]
fn build_waterfall(
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
    Chart::new({
    use crate::plot::statistical::{WaterfallConfig, render_waterfall_html};
    render_waterfall_html(&WaterfallConfig {
        title,
        labels: &labels,
        values: &values,
        x_label,
        y_label,
        show_text,
        gridlines,
        width,
        height,
    })
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, categories, values, x_label="", y_label="", palette=None, gridlines=true, width=900, height=500, bg_color=None), text_signature = "(title, categories, values, x_label='', y_label='', palette=None, gridlines=True, width=900, height=500, bg_color=None)")]
fn build_violin(
    title: &str,
    categories: Vec<String>,
    values: Vec<f64>,
    x_label: &str,
    y_label: &str,
    palette: Option<Vec<u32>>,
    gridlines: bool,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{ViolinConfig, render_violin_html};
    use crate::plot::statistical::common::PALETTE;
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    crate::html::hover::apply_bg(render_violin_html(&ViolinConfig {
        title,
        categories: &categories,
        values: &values,
        x_label,
        y_label,
        palette: pal_ref,
        gridlines,
        width,
        height,
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values_left, values_right, left_label="Before", right_label="After", palette=None, show_text=true, width=700, height=500, bg_color=None), text_signature = "(title, labels, values_left, values_right, left_label='Before', right_label='After', palette=None, show_text=True, width=700, height=500, bg_color=None)")]
fn build_slope(
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{SlopeConfig, render_slope_html};
    use crate::plot::statistical::common::PALETTE;
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    crate::html::hover::apply_bg(render_slope_html(&SlopeConfig {
        title,
        labels: &labels,
        values_left: &values_left,
        values_right: &values_right,
        left_label,
        right_label,
        palette: pal_ref,
        show_text,
        width,
        height,
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, targets=None, max_vals=None, ranges=None, width=800, height=300), text_signature = "(title, labels, values, targets=None, max_vals=None, ranges=None, width=800, height=300)")]
fn build_bullet(
    title: &str,
    labels: Vec<String>,
    values: Vec<f64>,
    targets: Option<Vec<f64>>,
    max_vals: Option<Vec<f64>>,
    ranges: Option<Vec<f64>>,
    width: i32,
    height: i32,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{BulletConfig, render_bullet_html};
    let targets = targets.unwrap_or_default();
    let max_vals = max_vals.unwrap_or_default();
    let ranges = ranges.unwrap_or_default();
    render_bullet_html(&BulletConfig {
        title,
        labels: &labels,
        values: &values,
        targets: &targets,
        max_vals: &max_vals,
        ranges: &ranges,
        width,
        height,
    })
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, words, frequencies, palette=None, sort_order="none", width=900, height=500, min_font=12.0, max_font=72.0, bg_color=None, hover_json=None), text_signature = "(title, words, frequencies, palette=None, sort_order='none', width=900, height=500, min_font=12.0, max_font=72.0, bg_color=None, hover_json=None)")]
fn build_wordcloud(
    title: &str,
    words: Vec<String>,
    frequencies: Vec<f64>,
    palette: Option<Vec<u32>>,
    sort_order: &str,
    width: i32,
    height: i32,
    min_font: f64,
    max_font: f64,
    bg_color: Option<String>,
    hover_json: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{WordCloudConfig, render_wordcloud_html, parse_hover_json};
    use crate::plot::statistical::common::PALETTE;
    let hover_slots = hover_json.as_deref().map(parse_hover_json).unwrap_or_default();
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    render_wordcloud_html(&WordCloudConfig {
        title,
        words: &words,
        frequencies: &frequencies,
        palette: pal_ref,
        width,
        height,
        min_font,
        max_font,
        bg_color: bg_color.as_deref(),
        hover: &hover_slots,
        sort_order,
        ..WordCloudConfig::default()
    })
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_names, series_values, palette=None, filled=true, fill_opacity=50, width=700, height=560, bg_color=None), text_signature = "(title, axes, series_names, series_values, palette=None, filled=True, fill_opacity=50, width=700, height=560, bg_color=None)")]
fn build_radar_chart(
    title: &str,
    axes: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
    palette: Option<Vec<u32>>,
    filled: bool,
    fill_opacity: u8,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{RadarConfig, render_radar_html};
    use crate::plot::statistical::common::PALETTE;
    let n_axes = axes.len();
    let n_ser = series_names.len();
    if n_axes < 3 || n_ser == 0 { return Chart::new(String::new()); }
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    let series: Vec<(String, Vec<f64>)> = series_names.into_iter().zip(series_values.into_iter()).map(|(name, vals)| (name, vals)).collect();
    crate::html::hover::apply_bg(render_radar_html(&RadarConfig {
        title, axes: &axes, series: &series, palette: pal_ref,
        width, height, filled, fill_opacity,
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, axes, series_names, series_values, x_label="Axis", y_label="Series", z_label="Value", color_labels=None, width=900, height=560, bg_color=None), text_signature = "(title, axes, series_names, series_values, x_label='Axis', y_label='Series', z_label='Value', color_labels=None, width=900, height=560, bg_color=None)")]
fn build_radar3d_chart(
    title: &str,
    axes: Vec<String>,
    series_names: Vec<String>,
    series_values: Vec<Vec<f64>>,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    color_labels: Option<Vec<String>>,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use std::f64::consts::PI;
    let n_axes = axes.len();
    let n_ser = series_names.len();
    if n_axes < 3 || n_ser == 0 { return Chart::new(String::new()); }
    let global_max = series_values.iter().flat_map(|v| v.iter().copied()).fold(0.0_f64, f64::max).max(1.0);
    let n_total = n_ser * n_axes;
    let mut xv = Vec::with_capacity(n_total);
    let mut yv = Vec::with_capacity(n_total);
    let mut zv = Vec::with_capacity(n_total);
    let mut cv = Vec::with_capacity(n_total);
    for (si, sv) in series_values.iter().enumerate() {
        for ai in 0..n_axes {
            let v = sv.get(ai).copied().unwrap_or(0.0);
            let frac = (v / global_max).clamp(0.0, 1.0);
            let angle = PI / 2.0 - 2.0 * PI * ai as f64 / n_axes as f64;
            xv.push(angle.cos() * frac);
            yv.push(si as f64);
            zv.push(angle.sin() * frac);
            cv.push(si as f64);
        }
    }
    let cl = color_labels.unwrap_or_else(|| series_names.clone());
    crate::html::js_3d::render_radar3d_html(title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &cl, width, height, bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, labels, values, orientation="v", x_label="", y_label="", palette=None, color_hex=0, gridlines=true, show_values=false, sort_order="none", width=900, height=480, bg_color=None), text_signature = "(title, labels, values, orientation='v', x_label='', y_label='', palette=None, color_hex=0, gridlines=True, show_values=False, sort_order='none', width=900, height=480, bg_color=None)")]
fn build_lollipop_chart(
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{LollipopConfig, render_lollipop_html};
    use crate::plot::statistical::common::PALETTE;
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    let ori = if orientation.starts_with('h') || orientation.starts_with('H') { b'h' } else { b'v' };
    crate::html::hover::apply_bg(render_lollipop_html(&LollipopConfig {
        title, labels: &labels, values: &values, x_label, y_label,
        palette: pal_ref, color_hex, gridlines, show_values,
        orientation: ori, sort_order, width, height,
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, x_values, y_values, z_values, x_label="X", y_label="Y", z_label="Value", color_labels=None, width=900, height=560, bg_color=None), text_signature = "(title, x_values, y_values, z_values, x_label='X', y_label='Y', z_label='Value', color_labels=None, width=900, height=560, bg_color=None)")]
fn build_lollipop3d_chart(
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
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    let cl = color_labels.unwrap_or_default();
    crate::html::js_3d::render_lollipop3d_html(title, &x_values, &y_values, &z_values, (x_label, y_label, z_label), &[], &cl, width, height, bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, bandwidth=0.0, x_label="", y_label="Density", filled=true, gridlines=true, palette=None, width=900, height=420, bg_color=None), text_signature = "(title, values, categories, bandwidth=0.0, x_label='', y_label='Density', filled=True, gridlines=True, palette=None, width=900, height=420, bg_color=None)")]
fn build_kde_chart(
    title: &str,
    values: Vec<f64>,
    categories: Vec<String>,
    bandwidth: f64,
    x_label: &str,
    y_label: &str,
    filled: bool,
    gridlines: bool,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{KdeConfig, render_kde_html};
    use crate::plot::statistical::common::PALETTE;
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    let n = values.len().min(categories.len());
    let mut group_order: Vec<String> = Vec::new();
    for cat in categories[..n].iter() {
        if !group_order.contains(cat) { group_order.push(cat.clone()); }
    }
    let mut group_vals: Vec<Vec<f64>> = vec![Vec::new(); group_order.len()];
    for i in 0..n {
        if let Some(gi) = group_order.iter().position(|g| g == &categories[i]) {
            group_vals[gi].push(values[i]);
        }
    }
    let series: Vec<(String, Vec<f64>)> = group_order.into_iter().zip(group_vals.into_iter()).collect();
    crate::html::hover::apply_bg(render_kde_html(&KdeConfig {
        title, series: &series, palette: pal_ref, x_label, y_label,
        bandwidth, filled, gridlines, width, height, ..KdeConfig::default()
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, bandwidth=0.0, x_label="Value", y_label="Group", z_label="Density", width=900, height=560, bg_color=None), text_signature = "(title, values, categories, bandwidth=0.0, x_label='Value', y_label='Group', z_label='Density', width=900, height=560, bg_color=None)")]
fn build_kde3d_chart(
    title: &str,
    values: Vec<f64>,
    categories: Vec<String>,
    bandwidth: f64,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let n = values.len().min(categories.len());
    let mut group_order: Vec<String> = Vec::new();
    for cat in categories[..n].iter() {
        if !group_order.contains(cat) { group_order.push(cat.clone()); }
    }
    let n_groups = group_order.len();
    let mut group_vals: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
    for i in 0..n {
        if let Some(gi) = group_order.iter().position(|g| g == &categories[i]) {
            group_vals[gi].push(values[i]);
        }
    }
    let all_vals: Vec<f64> = values[..n].to_vec();
    let x_min = all_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let xr = (x_max - x_min).max(1e-12);
    let x_pad = xr * 0.12;
    let x0 = x_min - x_pad;
    let x1 = x_max + x_pad;
    let n_pts = 80usize;
    let xs: Vec<f64> = (0..n_pts).map(|i| x0 + (x1 - x0) * i as f64 / (n_pts - 1) as f64).collect();
    let mut xv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    let mut yv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    let mut zv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    let mut cv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    for (gi, gvals) in group_vals.iter().enumerate() {
        let bw = if bandwidth > 0.0 { bandwidth } else { scott_bw(gvals).max(1e-12) };
        for (xi, &x) in xs.iter().enumerate() {
            let d = if gvals.is_empty() { 0.0 } else { kde_eval(gvals, x, bw) };
            xv.push(x);
            yv.push(gi as f64);
            zv.push(d);
            cv.push(gi as f64);
        }
    }
    crate::html::js_3d::render_kde3d_html(title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &group_order, width, height, bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, bandwidth=0.0, overlap=0.5, x_label="", palette=None, width=900, height=520, bg_color=None), text_signature = "(title, values, categories, bandwidth=0.0, overlap=0.5, x_label='', palette=None, width=900, height=520, bg_color=None)")]
fn build_ridgeline_chart(
    title: &str,
    values: Vec<f64>,
    categories: Vec<String>,
    bandwidth: f64,
    overlap: f64,
    x_label: &str,
    palette: Option<Vec<u32>>,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::{RidgelineConfig, render_ridgeline_html};
    use crate::plot::statistical::common::PALETTE;
    let pal = palette.unwrap_or_default();
    let pal_ref: &[u32] = if pal.is_empty() { PALETTE } else { &pal };
    crate::html::hover::apply_bg(render_ridgeline_html(&RidgelineConfig {
        title, values: &values, categories: &categories, palette: pal_ref,
        x_label, overlap, bandwidth, width, height, ..RidgelineConfig::default()
    }), bg_color.as_deref())
    })
}

#[cfg(feature = "python")]
#[pyfunction]
#[pyo3(signature = (title, values, categories, bandwidth=0.0, x_label="Value", y_label="Group", z_label="Density", width=900, height=560, bg_color=None), text_signature = "(title, values, categories, bandwidth=0.0, x_label='Value', y_label='Group', z_label='Density', width=900, height=560, bg_color=None)")]
fn build_ridgeline3d_chart(
    title: &str,
    values: Vec<f64>,
    categories: Vec<String>,
    bandwidth: f64,
    x_label: &str,
    y_label: &str,
    z_label: &str,
    width: i32,
    height: i32,
    bg_color: Option<String>,
) -> Chart {
    Chart::new({
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let n = values.len().min(categories.len());
    let mut group_order: Vec<String> = Vec::new();
    for cat in categories[..n].iter() {
        if !group_order.contains(cat) { group_order.push(cat.clone()); }
    }
    let n_groups = group_order.len();
    let mut group_vals: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
    for i in 0..n {
        if let Some(gi) = group_order.iter().position(|g| g == &categories[i]) {
            group_vals[gi].push(values[i]);
        }
    }
    let all_vals: Vec<f64> = values[..n].to_vec();
    let x_min = all_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let x_pad = (x_max - x_min).max(1e-12) * 0.12;
    let x0 = x_min - x_pad;
    let x1 = x_max + x_pad;
    let n_pts = 80usize;
    let xs: Vec<f64> = (0..n_pts).map(|i| x0 + (x1 - x0) * i as f64 / (n_pts - 1) as f64).collect();
    let mut xv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    let mut yv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    let mut zv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    let mut cv: Vec<f64> = Vec::with_capacity(n_groups * n_pts);
    for (gi, gvals) in group_vals.iter().enumerate() {
        let bw = if bandwidth > 0.0 { bandwidth } else { scott_bw(gvals).max(1e-12) };
        for &x in xs.iter() {
            let d = if gvals.is_empty() { 0.0 } else { kde_eval(gvals, x, bw) };
            xv.push(x); yv.push(gi as f64); zv.push(d); cv.push(gi as f64);
        }
    }
    crate::html::js_3d::render_ridgeline3d_html(title, &xv, &yv, &zv, (x_label, y_label, z_label), &cv, &group_order, width, height, bg_color.as_deref())
    })
}
