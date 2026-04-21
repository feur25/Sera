use serde::Deserialize;
use std::cell::RefCell;

thread_local! {
    static GLOBAL_BG: RefCell<Option<String>> = RefCell::new(None);
    static GLOBAL_PAL: RefCell<Vec<u32>> = RefCell::new(Vec::new());
    static GLOBAL_GRID: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn set_global_bg(color: Option<String>) {
    GLOBAL_BG.with(|bg| *bg.borrow_mut() = color);
}

pub fn set_global_pal(pal: Vec<u32>) {
    GLOBAL_PAL.with(|p| *p.borrow_mut() = pal);
}

pub fn set_global_grid(v: bool) {
    GLOBAL_GRID.with(|g| g.set(v));
}

fn get_global_bg() -> Option<String> {
    GLOBAL_BG.with(|bg| bg.borrow().clone())
}

fn get_global_pal() -> Vec<u32> {
    GLOBAL_PAL.with(|p| p.borrow().clone())
}

fn get_global_grid() -> bool {
    GLOBAL_GRID.with(|g| g.get())
}

#[derive(Deserialize, Default)]
pub struct CoreOpts {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub z_label: Option<String>,
    pub gridlines: Option<bool>,
    pub sort_order: Option<String>,
    pub hover_json: Option<String>,
    pub legend_position: Option<String>,
    pub palette: Option<Vec<u32>>,
    pub background: Option<String>,
    pub bg_color: Option<String>,
    pub no_x_axis: Option<bool>,
    pub no_y_axis: Option<bool>,
    pub color_hex: Option<u32>,
    pub orientation: Option<String>,
    pub show_text: Option<bool>,
    pub color_groups: Option<Vec<String>>,
    pub show_points: Option<bool>,
    pub show_regression: Option<bool>,
    pub regression_type: Option<String>,
    pub bins: Option<i32>,
    pub show_counts: Option<bool>,
    pub overlay_color_hex: Option<u32>,
    pub show_values: Option<bool>,
    pub color_low: Option<u32>,
    pub color_mid: Option<u32>,
    pub color_high: Option<u32>,
    pub col_labels: Option<Vec<String>>,
    pub show_pct: Option<bool>,
    pub inner_radius_ratio: Option<f64>,
    pub left_label: Option<String>,
    pub right_label: Option<String>,
    pub stacked: Option<bool>,
    pub series_names: Option<Vec<String>>,
    pub targets: Option<Vec<f64>>,
    pub max_vals: Option<Vec<f64>>,
    pub ranges: Option<Vec<f64>>,
    pub color_values: Option<Vec<f64>>,
    pub color_labels: Option<Vec<String>>,
    pub filled: Option<bool>,
    pub fill_opacity: Option<i32>,
    pub bandwidth: Option<f64>,
    pub overlap: Option<f64>,
    pub min_font: Option<f64>,
    pub max_font: Option<f64>,
    pub min_val: Option<f64>,
    pub max_val: Option<f64>,
    pub label: Option<String>,
    pub series_name_start: Option<String>,
    pub series_name_end: Option<String>,
    pub point_labels: Option<Vec<String>>,
    pub interval_ms: Option<u32>,
    pub cols: Option<usize>,
    pub gap: Option<i32>,
    pub cell_height: Option<i32>,
    pub eps: Option<f64>,
    pub min_samples: Option<usize>,
    pub k: Option<usize>,
    pub max_iter: Option<usize>,
    pub normalize: Option<bool>,
    pub images: Option<Vec<Option<String>>>,
    pub descriptions: Option<Vec<Vec<Vec<String>>>>,
}

impl CoreOpts {
    pub fn w(&self, default: i32) -> i32 { self.width.unwrap_or(default) }
    pub fn h(&self, default: i32) -> i32 { self.height.unwrap_or(default) }
    pub fn xl(&self) -> String { self.x_label.clone().unwrap_or_default() }
    pub fn yl(&self) -> String { self.y_label.clone().unwrap_or_default() }
    pub fn zl(&self) -> String { self.z_label.clone().unwrap_or_else(|| "Z".to_string()) }
    pub fn grid(&self) -> bool { self.gridlines.unwrap_or(false) || get_global_grid() }
    pub fn srt(&self) -> String { self.sort_order.clone().unwrap_or_else(|| "none".to_string()) }
    pub fn lp(&self) -> String { self.legend_position.clone().unwrap_or_else(|| "right".to_string()) }
    pub fn pal(&self) -> Vec<u32> {
        if let Some(p) = &self.palette { if !p.is_empty() { return p.clone(); } }
        let g = get_global_pal(); if !g.is_empty() { g } else { Vec::new() }
    }
    pub fn hj(&self) -> Vec<crate::html::hover::HoverSlot> {
        self.hover_json.as_ref().filter(|s| !s.is_empty())
            .map(|s| crate::plot::statistical::parse_hover_json(s))
            .unwrap_or_default()
    }
    pub fn bg_str(&self) -> Option<String> {
        self.background.clone().or_else(|| self.bg_color.clone()).filter(|s| !s.is_empty())
    }
    pub fn no_x(&self) -> bool { self.no_x_axis.unwrap_or(false) }
    pub fn no_y(&self) -> bool { self.no_y_axis.unwrap_or(false) }
}

#[derive(Deserialize, Default)]
pub struct CoreArgs {
    pub labels: Option<Vec<String>>,
    pub values: Option<Vec<f64>>,
    pub x: Option<Vec<f64>>,
    pub y: Option<Vec<f64>>,
    pub z: Option<Vec<f64>>,
    pub x_labels: Option<Vec<String>>,
    pub series: Option<Vec<Vec<f64>>>,
    pub matrix: Option<Vec<Vec<f64>>>,
    pub parents: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub open: Option<Vec<f64>>,
    pub high: Option<Vec<f64>>,
    pub low: Option<Vec<f64>>,
    pub close: Option<Vec<f64>>,
    pub sizes: Option<Vec<f64>>,
    pub overlay: Option<Vec<f64>>,
    pub left: Option<Vec<f64>>,
    pub right: Option<Vec<f64>>,
    pub start: Option<Vec<f64>>,
    pub end: Option<Vec<f64>>,
    pub size: Option<Vec<f64>>,
    pub charts: Option<Vec<String>>,
    pub value: Option<f64>,
    pub lats: Option<Vec<f64>>,
    pub lons: Option<Vec<f64>>,
    pub axes: Option<Vec<String>>,
    pub words: Option<Vec<String>>,
    pub frequencies: Option<Vec<f64>>,
    pub data: Option<Vec<Vec<f64>>>,
}

pub fn parse_opts(opts: &str) -> CoreOpts {
    serde_json::from_str(opts).unwrap_or_default()
}

pub fn parse_args(args: &str) -> CoreArgs {
    serde_json::from_str(args).unwrap_or_default()
}

fn parse_all(input: &str) -> (String, CoreArgs, CoreOpts) {
    #[derive(Deserialize, Default)]
    struct All {
        #[serde(default)]
        title: String,
        #[serde(flatten)]
        args: CoreArgs,
        #[serde(flatten)]
        opts: CoreOpts,
    }
    let all: All = serde_json::from_str(input).unwrap_or_default();
    (all.title, all.args, all.opts)
}

pub fn apply(html: String, o: &CoreOpts) -> String {
    let bg_str = o.bg_str().or_else(get_global_bg);
    let bg = bg_str.as_deref();
    crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y())
}

pub fn apply_bg3d(html: String, o: &CoreOpts) -> String {
    let bg_str = o.bg_str().or_else(get_global_bg);
    if let Some(bg) = bg_str.as_deref() {
        crate::html::hover::apply_bg(html, Some(bg))
    } else {
        html
    }
}

pub fn build_html_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let exporter = crate::html::FastHtmlExporter::new(o.w(900), o.h(480), title.to_string());
    exporter.build_optimized(labels, values)
}

pub fn build_bar_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_hbar(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, o.w(900), o.h(500), &hover, b'h',
        &groups, o.show_text.unwrap_or(true), &o.xl(), &o.yl(),
        &o.pal(), o.color_hex.unwrap_or(0), o.grid(), &o.srt(),
    );
    apply(html, &o)
}

pub fn build_line_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_lines_html(
        title, &labels, &values, o.w(900), o.h(480), &hover,
        o.color_hex.unwrap_or(0x6366F1), &o.xl(), &o.yl(),
        o.grid(), o.show_points.unwrap_or(true), &o.srt(),
    );
    apply(html, &o)
}

pub fn build_dbscan_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let eps = o.eps.unwrap_or(0.5);
    let min_samples = o.min_samples.unwrap_or(5);
    let normalize = o.normalize.unwrap_or(false);
    let pal = o.pal();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_dbscan_html(
        title, &x, &y, eps, min_samples, &o.xl(), &o.yl(),
        o.w(900), o.h(540), o.grid(), normalize, &pal,
    );
    crate::html::hover::apply_opts(html, bg_str.as_deref(), !o.no_x(), !o.no_y())
}

pub fn build_dbscan_chart_3d(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let eps = o.eps.unwrap_or(0.5);
    let min_samples = o.min_samples.unwrap_or(5);
    let normalize = o.normalize.unwrap_or(false);
    let n = x.len().min(y.len()).min(z.len());
    let (xn, yn, zn) = if normalize && n > 0 {
        let norm = |v: &[f64]| {
            let mn = v.iter().cloned().fold(f64::INFINITY, f64::min);
            let mx = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let r = (mx - mn).max(1e-12);
            v.iter().map(|&val| (val - mn) / r).collect::<Vec<_>>()
        };
        (norm(&x[..n]), norm(&y[..n]), norm(&z[..n]))
    } else {
        (x[..n].to_vec(), y[..n].to_vec(), z[..n].to_vec())
    };
    let data: Vec<Vec<f64>> = (0..n).map(|i| vec![xn[i], yn[i], zn[i]]).collect();
    let (labels, _) = crate::plot::default::scatter::dbscan_core_nd(&data, eps, min_samples);
    let color_labels: Vec<String> = labels.iter().map(|&l| {
        if l < 0 { "Noise".to_string() } else { format!("Cluster {}", l + 1) }
    }).collect();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_scatter3d_html(
        title, &xn, &yn, &zn, (&o.xl(), &o.yl(), &o.zl()), &[], &color_labels,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

pub fn build_kmeans_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let k = o.k.unwrap_or(3);
    let max_iter = o.max_iter.unwrap_or(300);
    let pal = o.pal();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_kmeans_html(&crate::plot::default::KMeansConfig {
        title, x_values: &x, y_values: &y, k, max_iter, tol: 1e-4,
        mini_batch: false, batch_size: 1000, width: o.w(1000), height: o.h(580),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(), palette: &pal,
        ..Default::default()
    });
    crate::html::hover::apply_opts(html, bg_str.as_deref(), !o.no_x(), !o.no_y())
}

pub fn build_scatter_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let lbls = a.labels.unwrap_or_default();
    let sz = a.sizes.unwrap_or_default();
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

pub fn build_histogram(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
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

pub fn build_histogram_overlay(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let overlay = a.overlay.unwrap_or_default();
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

pub fn build_grouped_bar(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let series_flat: Vec<f64> = a.values.unwrap_or_default();
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_flat.len() + n_cats - 1) / n_cats } else { 0 };
    let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
    let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| series_flat.get(si * n_cats + ci).copied().unwrap_or(0.0)).collect();
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

pub fn build_stacked_bar(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let series_flat: Vec<f64> = a.values.unwrap_or_default();
    use crate::plot::statistical::{GroupedBarConfig, render_grouped_bar_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    let n_series = if !sn.is_empty() { sn.len() } else if n_cats > 0 { (series_flat.len() + n_cats - 1) / n_cats } else { 0 };
    let names: Vec<String> = if !sn.is_empty() { sn } else { (0..n_series).map(|_| String::new()).collect() };
    let series: Vec<(String, Vec<f64>)> = names.iter().enumerate().map(|(si, name)| {
        let vals: Vec<f64> = (0..n_cats).map(|ci| series_flat.get(si * n_cats + ci).copied().unwrap_or(0.0)).collect();
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

pub fn build_heatmap(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let flat_matrix = a.values.unwrap_or_default();
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

pub fn build_pie_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{PieConfig, render_pie_html};
    let hover = o.hj();
    let html = render_pie_html(&PieConfig {
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true), sort_order: &o.srt(),
        width: o.w(720), height: o.h(440), hover: &hover, ..PieConfig::default()
    });
    apply(html, &o)
}

pub fn build_donut_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_boxplot(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::statistical::render_boxplot_html(&crate::plot::statistical::BoxplotConfig {
        title, category_labels: &category_labels, values: &values, palette: &o.pal(),
        hover: &hover, x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        width: o.w(900), height: o.h(500), sort_order: &o.srt(), legend_position: &o.lp(),
    });
    apply(html, &o)
}

pub fn build_violin(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let categories = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_slope(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_left = a.left.unwrap_or_default();
    let values_right = a.right.unwrap_or_default();
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

pub fn build_sunburst(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{SunburstConfig, render_sunburst_html};
    let hover = o.hj();
    let html = render_sunburst_html(&SunburstConfig {
        title, labels: &labels, parents: &parents, values: &values,
        width: o.w(700), height: o.h(700), hover: &hover, ..SunburstConfig::default()
    });
    apply(html, &o)
}

pub fn build_funnel(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{FunnelConfig, render_funnel_html};
    let hover = o.hj();
    let html = render_funnel_html(&FunnelConfig {
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true), width: o.w(800), height: o.h(480),
        sort_order: &o.srt(), hover: &hover, ..FunnelConfig::default()
    });
    apply(html, &o)
}

pub fn build_treemap(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let pars = a.parents.unwrap_or_default();
    use crate::plot::statistical::{TreemapConfig, render_treemap_html};
    let hover = o.hj();
    let html = render_treemap_html(&TreemapConfig {
        title, labels: &labels, values: &values, parents: &pars,
        palette: &o.pal(), sort_order: &o.srt(), width: o.w(1100), height: o.h(520),
        hover: &hover, ..TreemapConfig::default()
    });
    apply(html, &o)
}

pub fn build_multiline_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
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

pub fn build_area_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
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

pub fn build_waterfall(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_bullet(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_bubble_map(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::map::render_bubble_map_html(title, &labels, &values, o.w(1200), o.h(600), &hover);
    apply(html, &o)
}

pub fn build_choropleth(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::map::render_choropleth_html(title, &labels, &values, o.w(1200), o.h(600), &hover);
    apply(html, &o)
}

pub fn build_scatter3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_scatter3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

pub fn build_bar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_bar3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

pub fn build_line3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_line3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}

pub fn build_radar_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
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

pub fn build_radar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let n_axes = axes.len();
    if n_axes == 0 { return String::new(); }
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let n_series = names.len().min(series_flat.len());
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new(); let mut cv = Vec::new();
    for si in 0..n_series {
        let vals = &series_flat[si];
        let max_val = vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);
        for ai in 0..n_axes.min(vals.len()) {
            let angle = std::f64::consts::TAU * ai as f64 / n_axes as f64;
            let r = vals[ai] / max_val;
            xv.push(angle.cos() * r); yv.push(si as f64); zv.push(angle.sin() * r); cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_radar3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_lollipop_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_lollipop3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_lollipop3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &[], &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_kde_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{KdeConfig, render_kde_html};
    let series: Vec<(String, Vec<f64>)> = if let Some(cats) = a.categories {
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

pub fn build_kde3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let cats = a.categories.unwrap_or_default();
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
    if all_vals.is_empty() { return String::new(); }
    let xmin = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad; let hi = xmax + pad;
    let n_pts: usize = 120;
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new(); let mut cv = Vec::new();
    let names: Vec<String> = series.iter().map(|(n, _)| n.clone()).collect();
    for (si, (_, vals)) in series.iter().enumerate() {
        let bw = scott_bw(vals);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(vals, t, bw);
            xv.push(t); yv.push(si as f64); zv.push(d); cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_kde3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_ridgeline_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let categories = a.categories.or(a.labels).unwrap_or_default();
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

pub fn build_ridgeline3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let categories = a.categories.or(a.labels).unwrap_or_default();
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let mut group_order: Vec<String> = Vec::new();
    let mut group_vals: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    for (v, c) in values.iter().zip(categories.iter()) {
        group_vals.entry(c.clone()).or_default().push(*v);
        if !group_order.contains(c) { group_order.push(c.clone()); }
    }
    let series: Vec<(String, Vec<f64>)> = group_order.into_iter().map(|k| { let v = group_vals.remove(&k).unwrap_or_default(); (k, v) }).collect();
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if all_vals.is_empty() { return String::new(); }
    let xmin = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad; let hi = xmax + pad;
    let n_pts: usize = 80;
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new(); let mut cv = Vec::new();
    let names: Vec<String> = series.iter().map(|(n, _)| n.clone()).collect();
    for (si, (_, vals)) in series.iter().enumerate() {
        let bw = scott_bw(vals);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(vals, t, bw);
            xv.push(t); yv.push(si as f64); zv.push(d); cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_ridgeline3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_bubble3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let size_values = a.size.or(a.sizes).unwrap_or_default();
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

pub fn build_pie3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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

pub fn build_violin3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::kde::{scott_bw, kde_eval};
    let cats = a.categories.or(a.labels).unwrap_or_default();
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
    if all_vals.is_empty() { return String::new(); }
    let xmin = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad; let hi = xmax + pad;
    let n_pts: usize = 100;
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new(); let mut cv = Vec::new();
    let names: Vec<String> = series.iter().map(|(n, _)| n.clone()).collect();
    for (si, (_, vals)) in series.iter().enumerate() {
        let bw = scott_bw(vals);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(vals, t, bw);
            xv.push(t); yv.push(si as f64); zv.push(d); cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_violin3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_heatmap3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or_else(|| a.labels.clone()).unwrap_or_default();
    let y_labels = a.categories.or(a.labels).unwrap_or_default();
    let values = a.matrix.unwrap_or_default();
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
    apply_bg3d(crate::plot::statistical::_3d::render_heatmap3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_candlestick3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
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
    apply_bg3d(crate::plot::statistical::_3d::render_candlestick3d_html(
        title, &xv, &yv, &zv, (x_lbl, y_lbl, &zl), &[], &labels,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_dumbbell3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
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
    apply_bg3d(crate::plot::statistical::_3d::render_dumbbell3d_html(
        title, &xv, &yv, &zv, (&s_name, y_lbl, &e_name), &cv, &labels,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_funnel3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::common::apply_sort;
    let srt = o.srt();
    let (labels, values) = apply_sort(&labels, &values, &srt);
    let n = labels.len().min(values.len());
    let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_funnel3d_html(
        title, &xv, &yv, &values[..n], ("", "Stage", "Value"),
        &cv, &labels[..n].to_vec(), o.w(700), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_sunburst3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
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
    apply_bg3d(crate::plot::statistical::_3d::render_sunburst3d_html(
        title, &xv, &yv, &zv, ("", "Ring", "Value"), &cv, &cl,
        o.w(700), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_stacked_bar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
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
    apply_bg3d(crate::plot::statistical::_3d::render_stacked_bar3d_html(
        title, &xv, &yv, &zv, (x_lbl, y_lbl, &z_lbl), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}

pub fn build_globe3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let latitudes = a.lats.unwrap_or_default();
    let longitudes = a.lons.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let n = latitudes.len().min(longitudes.len()).min(values.len());
    let lbl = o.point_labels.clone().unwrap_or_default();
    let cl = if lbl.is_empty() { (0..n).map(|i| format!("Point {}", i + 1)).collect() } else { lbl };
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::map::_3d::render_globe3d_html(
        title, &longitudes[..n], &latitudes[..n], &values[..n],
        ("Longitude", "Latitude", "Value"), &cv, &cl, o.w(800), o.h(600), bg_str.as_deref(),
    ), &o)
}

pub fn build_wordcloud(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let words = a.words.unwrap_or_default();
    let frequencies = a.frequencies.unwrap_or_else(|| a.values.unwrap_or_default());
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

pub fn build_candlestick(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
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

pub fn build_dumbbell(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    use crate::plot::statistical::dumbbell::{DumbbellConfig, render_dumbbell_html};
    let s = o.series_name_start.as_deref().unwrap_or("Start").to_string();
    let e = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let hover = o.hj();
    let html = render_dumbbell_html(&DumbbellConfig {
        title, labels: &labels, values_start: &values_start, values_end: &values_end,
        series_names: (&s, &e), palette: &o.pal(), width: o.w(1000), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..DumbbellConfig::default()
    });
    apply(html, &o)
}

pub fn build_bubble(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let sizes = a.sizes.or(a.size).unwrap_or_default();
    use crate::plot::statistical::bubble::{BubbleConfig, render_bubble_html};
    let cs = a.categories.unwrap_or_default();
    let hover = o.hj();
    let html = render_bubble_html(&BubbleConfig {
        title, x_values: &x_values, y_values: &y_values, sizes: &sizes,
        categories: &cs, palette: &o.pal(), width: o.w(900), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..BubbleConfig::default()
    });
    apply(html, &o)
}

pub fn build_gauge(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let value = a.value.unwrap_or(0.0);
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

pub fn build_parallel(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
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

pub fn build_grid(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let charts = a.charts.unwrap_or_default();
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

pub fn build_slideshow(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let charts = a.charts.unwrap_or_default();
    if charts.is_empty() { return String::new(); }
    let ivms = o.interval_ms.unwrap_or(2500);
    let width = o.w(900);
    let height = o.h(520);
    let show_title = if !title.is_empty() { title } else { o.label.as_deref().unwrap_or("") };
    let title_html = if show_title.is_empty() { String::new() } else {
        format!("<div style=\"color:#1e293b;font-family:system-ui;font-size:22px;font-weight:700;text-align:center;margin-bottom:16px\">{}</div>", show_title)
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

pub fn build_hover_json(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct I {
        labels: Option<Vec<String>>,
        images: Option<Vec<Option<String>>>,
        descriptions: Option<Vec<Vec<Vec<String>>>>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let labels = i.labels.unwrap_or_default();
    use crate::html::hover::{HoverSlot, slots_to_json};
    let n = labels.len();
    let mut slots = Vec::with_capacity(n);
    for idx in 0..n {
        let mut slot = HoverSlot::new(&labels[idx]);
        if let Some(ref imgs) = i.images {
            if let Some(Some(ref url)) = imgs.get(idx) {
                slot = slot.image(url.clone());
            }
        }
        if let Some(ref descs) = i.descriptions {
            if let Some(row) = descs.get(idx) {
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

pub fn ml_dbscan_fit_predict(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct I { data: Option<Vec<Vec<f64>>>, eps: Option<f64>, min_samples: Option<usize> }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let data = i.data.unwrap_or_default();
    let eps = i.eps.unwrap_or(0.5);
    let min_samples = i.min_samples.unwrap_or(5);
    let (labels, n_clusters) = crate::plot::default::scatter::dbscan_core_nd(&data, eps, min_samples);
    let n_noise = labels.iter().filter(|&&l| l < 0).count();
    format!("{{\"labels\":{},\"n_clusters\":{},\"n_noise\":{}}}",
        serde_json::to_string(&labels).unwrap_or_default(),
        n_clusters, n_noise)
}

pub fn ml_kmeans_fit_predict(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct I { data: Option<Vec<Vec<f64>>>, k: Option<usize>, max_iter: Option<usize>, n_init: Option<usize> }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let rows = i.data.unwrap_or_default();
    let k = i.k.unwrap_or(3);
    let max_iter = i.max_iter.unwrap_or(300);
    let n_init = i.n_init.unwrap_or(10);
    let n = rows.len();
    if n == 0 { return "{\"labels\":[],\"inertia\":0.0}".to_string(); }
    let dims = rows[0].len();
    let mut flat = vec![0.0f64; n * dims];
    for (ri, row) in rows.iter().enumerate() {
        let len = row.len().min(dims);
        flat[ri * dims..ri * dims + len].copy_from_slice(&row[..len]);
    }
    let (labels, _, inertia) = crate::plot::default::kmeans_core_flat_ninit(&flat, n, dims, k, max_iter, 1e-4, n_init);
    format!("{{\"labels\":{},\"inertia\":{:.6}}}",
        serde_json::to_string(&labels).unwrap_or_default(), inertia)
}

pub fn set_global_background(input: &str) -> String {
    let color = input.trim().trim_matches('"');
    set_global_bg(if color.is_empty() { None } else { Some(color.to_string()) });
    String::new()
}

pub fn reset_global_background(_: &str) -> String {
    set_global_bg(None);
    String::new()
}

pub fn set_theme(input: &str) -> String {
    let name = input.trim().trim_matches('"');
    let (bg, pal, grid): (Option<&str>, &[u32], bool) = match name {
        "dark"       => (Some("#0f172a"), &[0x818CF8,0x34D399,0xFBBF24,0xF87171,0x60A5FA,0xA78BFA,0xFB7185,0x2DD4BF,0xF472B6,0xFACC15], true),
        "light"      => (None,            &[0x6366F1,0x10B981,0xF59E0B,0xEF4444,0x3B82F6,0x8B5CF6,0xEC4899,0x14B8A6,0xF97316,0xEAB308], false),
        "scientific" => (Some("#fafafa"),  &[0x1F77B4,0xFF7F0E,0x2CA02C,0xD62728,0x9467BD,0x8C564B,0xE377C2,0x7F7F7F,0xBCBD22,0x17BECF], true),
        "apple"      => (Some("#000000"),  &[0xFF375F,0x30D158,0x0A84FF,0xFFD60A,0xFF9F0A,0x5E5CE6,0x64D2FF,0xBF5AF2,0xFF6961,0x32ADE6], false),
        "notion"     => (Some("#191919"),  &[0xE3E3E3,0xA0A0A0,0xCB9D6D,0x7C9E7E,0x7B8FC4,0xC17B7B,0xD4A76A,0x8BA4B0,0xB39DDB,0x80CBC4], false),
        "minimal"    => (None,            &[0x222222,0x444444,0x666666,0x888888,0xAAAAAA,0xCCCCCC,0x111111,0x333333,0x555555,0x777777], false),
        "neon"       => (Some("#0a0a0a"),  &[0x00FFF0,0xFF00FF,0x00FF41,0xFF6B00,0xFFFF00,0xFF1493,0x00BFFF,0xFF4500,0x7FFF00,0xDA70D6], false),
        _ => return String::new(),
    };
    set_global_bg(bg.map(str::to_string));
    set_global_pal(pal.to_vec());
    set_global_grid(grid);
    String::new()
}

pub fn reset_theme(_: &str) -> String {
    set_global_bg(None);
    set_global_pal(Vec::new());
    set_global_grid(false);
    String::new()
}

pub fn themes(_: &str) -> String {
    "[\"dark\",\"light\",\"scientific\",\"apple\",\"notion\",\"minimal\",\"neon\"]".to_string()
}


