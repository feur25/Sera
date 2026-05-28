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
pub struct ChartOpts {
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
    pub orientation_option: Option<u8>,
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
    pub comparisons: Option<Vec<f64>>,
    pub comparison: Option<f64>,
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
    pub variant: Option<String>,
    pub offset_groups: Option<Vec<String>>,
    pub widths: Option<Vec<f64>>,
    pub super_categories: Option<Vec<String>>,
    pub icon_size: Option<i32>,
    pub max_icons_per_column: Option<i32>,
    pub units_per_icon: Option<f64>,
    pub unit_description: Option<String>,
    pub corner_radius: Option<i32>,
    pub bar_gap: Option<f64>,
    pub bargroup_gap: Option<f64>,
    pub line_shape: Option<String>,
    pub step_shape: Option<String>,
    pub spline_tension: Option<f64>,
    pub dash_pattern: Option<String>,
    pub stroke_width: Option<f64>,
    pub marker_size: Option<i32>,
    pub gap_threshold: Option<f64>,
    pub spark_cols: Option<usize>,
    pub spark_cell_w: Option<i32>,
    pub spark_cell_h: Option<i32>,
    pub stack_fill: Option<bool>,
    pub fill_opacity_f: Option<f64>,
    pub min_size: Option<f64>,
    pub max_size: Option<f64>,
    pub point_size: Option<f64>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub pull: Option<Vec<f64>>,
    pub subplot_titles: Option<Vec<String>>,
    pub subplot_cols: Option<usize>,
    pub proportional: Option<bool>,
    pub min_label_frac: Option<f64>,
    pub center_text: Option<String>,
    pub center_subtext: Option<String>,
    pub secondary_values: Option<Vec<f64>>,
    pub secondary_labels: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub notch: Option<bool>,
    pub jitter: Option<f64>,
    pub boxen_depth: Option<usize>,
    pub violin_overlay: Option<bool>,
    pub fill_opacity_real: Option<f64>,
    pub box_stroke_width: Option<f64>,
    pub colorscale: Option<String>,
    pub colorbar_position: Option<String>,
    pub origin_lower: Option<bool>,
    pub show_box: Option<bool>,
    pub show_mean: Option<bool>,
    pub highlight_index: Option<i32>,
    pub color_axis: Option<i32>,
    pub category_indices: Option<Vec<i32>>,
    pub annotations: Option<Vec<Annotation>>,
    pub shape: Option<String>,
    pub mask: Option<Vec<i32>>,
    pub mask_width: Option<i32>,
    pub mask_height: Option<i32>,
    pub points_x: Option<Vec<f64>>,
    pub points_y: Option<Vec<f64>>,
    pub cluster_labels: Option<Vec<String>>,
    pub edges_i: Option<Vec<i32>>,
    pub edges_j: Option<Vec<i32>>,
    pub edges_w: Option<Vec<f64>>,
    pub theme: Option<String>,
}

#[derive(Deserialize, Default, Clone)]
pub struct Annotation {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
    pub text: Option<String>,
    pub color: Option<String>,
    pub stroke_width: Option<f64>,
    pub dash: Option<String>,
    pub frac: Option<bool>,
    pub font_size: Option<f64>,
    pub fill: Option<String>,
    pub opacity: Option<f64>,
}

fn fmt_pos(v: f64, frac: bool) -> String {
    if frac { format!("{:.2}%", v * 100.0) } else { format!("{}", v) }
}

fn ann_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub fn apply_annotations(html: String, o: &ChartOpts) -> String {
    let anns = match &o.annotations {
        Some(a) if !a.is_empty() => a.clone(),
        _ => return html,
    };
    let mut buf = String::with_capacity(256 * anns.len());
    for a in &anns {
        let frac = a.frac.unwrap_or(true);
        let color = a.color.as_deref().unwrap_or("#ef4444");
        let sw = a.stroke_width.unwrap_or(1.5);
        let dash = a.dash.as_deref().unwrap_or("");
        let dash_attr = if dash.is_empty() { String::new() } else { format!(" stroke-dasharray=\"{}\"", dash) };
        let opacity = a.opacity.unwrap_or(1.0);
        let op_attr = if (opacity - 1.0).abs() < 1e-6 { String::new() } else { format!(" opacity=\"{:.2}\"", opacity) };
        match a.kind.as_str() {
            "hline" => {
                let y = fmt_pos(a.y.unwrap_or(0.5), frac);
                buf.push_str(&format!("<line x1=\"0\" x2=\"100%\" y1=\"{y}\" y2=\"{y}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}/>"));
                if let Some(t) = &a.text {
                    let fs = a.font_size.unwrap_or(11.0);
                    buf.push_str(&format!("<text x=\"6\" y=\"{y}\" dy=\"-3\" font-size=\"{fs}\" fill=\"{color}\" font-family=\"Arial,sans-serif\">{}</text>", ann_escape(t)));
                }
            }
            "vline" => {
                let x = fmt_pos(a.x.unwrap_or(0.5), frac);
                buf.push_str(&format!("<line y1=\"0\" y2=\"100%\" x1=\"{x}\" x2=\"{x}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}/>"));
                if let Some(t) = &a.text {
                    let fs = a.font_size.unwrap_or(11.0);
                    buf.push_str(&format!("<text x=\"{x}\" y=\"14\" dx=\"4\" font-size=\"{fs}\" fill=\"{color}\" font-family=\"Arial,sans-serif\">{}</text>", ann_escape(t)));
                }
            }
            "line" | "arrow" => {
                let x1 = fmt_pos(a.x.unwrap_or(0.0), frac);
                let y1 = fmt_pos(a.y.unwrap_or(0.0), frac);
                let x2 = fmt_pos(a.x2.unwrap_or(1.0), frac);
                let y2 = fmt_pos(a.y2.unwrap_or(1.0), frac);
                let arrow_id = format!("sp-ar-{}", buf.len());
                if a.kind == "arrow" {
                    buf.push_str(&format!("<defs><marker id=\"{arrow_id}\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"6\" markerHeight=\"6\" orient=\"auto\"><path d=\"M0,0 L10,5 L0,10 z\" fill=\"{color}\"/></marker></defs>"));
                }
                let m_attr = if a.kind == "arrow" { format!(" marker-end=\"url(#{arrow_id})\"") } else { String::new() };
                buf.push_str(&format!("<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}{m_attr}/>"));
            }
            "rect" => {
                let x = fmt_pos(a.x.unwrap_or(0.0), frac);
                let y = fmt_pos(a.y.unwrap_or(0.0), frac);
                let x2v = a.x2.unwrap_or(1.0);
                let y2v = a.y2.unwrap_or(1.0);
                let w = fmt_pos((x2v - a.x.unwrap_or(0.0)).max(0.0), frac);
                let h = fmt_pos((y2v - a.y.unwrap_or(0.0)).max(0.0), frac);
                let fill = a.fill.as_deref().unwrap_or("none");
                buf.push_str(&format!("<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"{fill}\" stroke=\"{color}\" stroke-width=\"{sw}\"{dash_attr}{op_attr}/>"));
            }
            "text" => {
                let x = fmt_pos(a.x.unwrap_or(0.5), frac);
                let y = fmt_pos(a.y.unwrap_or(0.5), frac);
                let fs = a.font_size.unwrap_or(13.0);
                let t = a.text.clone().unwrap_or_default();
                buf.push_str(&format!("<text x=\"{x}\" y=\"{y}\" font-size=\"{fs}\" font-family=\"Arial,sans-serif\" fill=\"{color}\"{op_attr}>{}</text>", ann_escape(&t)));
            }
            _ => {}
        }
    }
    if buf.is_empty() { return html; }
    if let Some(idx) = html.rfind("</svg>") {
        let mut out = String::with_capacity(html.len() + buf.len() + 64);
        out.push_str(&html[..idx]);
        out.push_str("<g class=\"sp-annotations\" pointer-events=\"none\">");
        out.push_str(&buf);
        out.push_str("</g>");
        out.push_str(&html[idx..]);
        out
    } else {
        html
    }
}

impl ChartOpts {
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
    pub fn is_horiz(&self) -> bool {
        match self.orientation.as_deref() {
            Some(s) => {
                let l = s.to_ascii_lowercase();
                l == "h" || l == "horiz" || l == "horizontal" || l == "rotated" || l == "hbar" || l == "hbox" || l == "barh"
            }
            None => false,
        }
    }
    pub fn orient_byte(&self) -> u8 { if self.is_horiz() { b'h' } else { b'v' } }
    pub fn rotation_deg(&self) -> i32 {
        if let Some(opt) = self.orientation_option {
            return match opt { 2 => 90, 3 => 180, 4 => 270, _ => 0 };
        }
        if self.is_horiz() { 90 } else { 0 }
    }
    pub fn rotation_deg_native(&self) -> i32 {
        match self.orientation_option.unwrap_or(0) {
            2 => 90,
            3 => 180,
            4 => 270,
            _ => 0,
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ChartArgs {
    pub labels: Option<Vec<String>>,
    pub values: Option<Vec<f64>>,
    #[serde(alias = "x_values")]
    pub x: Option<Vec<f64>>,
    #[serde(alias = "y_values")]
    pub y: Option<Vec<f64>>,
    #[serde(alias = "z_values")]
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

pub fn parse_opts(opts: &str) -> ChartOpts {
    serde_json::from_str(opts).unwrap_or_default()
}

pub fn parse_args(args: &str) -> ChartArgs {
    serde_json::from_str(args).unwrap_or_default()
}

fn parse_all(input: &str) -> (String, ChartArgs, ChartOpts) {
    #[derive(Deserialize, Default)]
    struct All {
        #[serde(default)]
        title: String,
        #[serde(flatten)]
        args: ChartArgs,
        #[serde(flatten)]
        opts: ChartOpts,
    }
    let all: All = serde_json::from_str(input).unwrap_or_default();
    (all.title, all.args, all.opts)
}

pub fn apply(html: String, o: &ChartOpts) -> String {
    let bg_str = o.bg_str().or_else(get_global_bg);
    let bg = bg_str.as_deref();
    let h = crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y());
    let h = crate::html::hover::apply_rotation(h, o.rotation_deg());
    let h = apply_annotations(h, o);
    let h = apply_kwarg_chains(h, o);
    if let Some(ref t) = o.theme {
        crate::plot::statistical::apply_chart_theme(h, t)
    } else { h }
}

pub fn apply_h(html: String, o: &ChartOpts) -> String {
    let bg_str = o.bg_str().or_else(get_global_bg);
    let bg = bg_str.as_deref();
    let h = crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y());
    let h = crate::html::hover::apply_rotation(h, o.rotation_deg_native());
    let h = apply_annotations(h, o);
    let h = apply_kwarg_chains(h, o);
    if let Some(ref t) = o.theme {
        crate::plot::statistical::apply_chart_theme(h, t)
    } else { h }
}

#[cfg(feature = "python")]
fn apply_kwarg_chains(html: String, o: &ChartOpts) -> String {
    use crate::{SP_LEGEND_JS, json_str};
    let mut snip = String::new();
    if let Some(ref lp) = o.legend_position {
        let pos = match lp.as_str() { "right" | "left" | "top" | "bottom" => lp.as_str(), _ => "right" };
        snip.push_str(&format!("window.__sp_legend_pos__={};", json_str(pos)));
        snip.push_str(SP_LEGEND_JS);
        snip.push(';');
    }
    if snip.is_empty() {
        return html;
    }
    let block = format!("<script>{}</script></body>", snip);
    html.replacen("</body>", &block, 1)
}

#[cfg(not(feature = "python"))]
fn apply_kwarg_chains(html: String, _o: &ChartOpts) -> String { html }

pub fn apply_bg3d(html: String, o: &ChartOpts) -> String {
    let bg_str = o.bg_str().or_else(get_global_bg);
    let h = if let Some(bg) = bg_str.as_deref() {
        crate::html::hover::apply_bg(html, Some(bg))
    } else {
        html
    };
    apply_annotations(h, o)
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
    let orient = o.orient_byte();
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::default::render_bars_html(
        title, &labels, &values, o.w(900), o.h(480), &hover, orient,
        &groups, o.show_text.unwrap_or(false), &o.xl(), &o.yl(),
        &o.pal(), o.color_hex.unwrap_or(0), o.grid(), &o.srt(),
    );
    apply_h(html, &o)
}

/// Unified bar-family entry point. Dispatches by `variant` keyword:
/// "basic" | "horizontal" | "grouped" | "stacked" | "relative" |
/// "grouped_stacked" | "marimekko" | "pictogram" | "multicategory".
#[crate::sera_alias("bar", "bar_chart", "bars", "bar_unified", "bars_unified", "bar_family")]
pub fn build_bar(input: &str) -> String {
    use crate::plot::statistical::{BarVariant, BarConfig, render_bar_html, ChartTheme};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = BarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));

    let labels = a.labels.clone().unwrap_or_default();
    let values = a.values.clone().unwrap_or_default();
    let category_labels = a.labels.clone().unwrap_or_default();
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let palette = o.pal();
    let offset_groups = o.offset_groups.clone().unwrap_or_default();
    let widths = o.widths.clone().unwrap_or_default();
    let super_categories = o.super_categories.clone().unwrap_or_default();
    let unit_desc = o.unit_description.clone().unwrap_or_default();
    let xl = o.xl(); let yl = o.yl(); let srt = o.srt(); let lp = o.lp();

    let series: Vec<(String, Vec<f64>)> = {
        let sn = o.series_names.clone().unwrap_or_default();
        let n_cats = category_labels.len();
        if let Some(s) = a.series.as_ref() {
            s.iter().enumerate().map(|(si, vals)| (
                sn.get(si).cloned().unwrap_or_else(|| format!("S{}", si + 1)),
                vals.clone(),
            )).collect()
        } else if !sn.is_empty() && n_cats > 0 {
            let flat = a.values.clone().unwrap_or_default();
            sn.iter().enumerate().map(|(si, name)| {
                let vals: Vec<f64> = (0..n_cats)
                    .map(|ci| flat.get(si * n_cats + ci).copied().unwrap_or(0.0))
                    .collect();
                (name.clone(), vals)
            }).collect()
        } else {
            Vec::new()
        }
    };

    let cfg = BarConfig {
        variant, title,
        x_label: &xl, y_label: &yl,
        width: o.w(900), height: o.h(480),
        gridlines: o.grid(), sort_order: &srt, legend_position: &lp,
        hover: &hover, palette: &palette,
        labels: &labels, values: &values,
        color_hex: o.color_hex.unwrap_or(0),
        color_groups: &groups,
        category_labels: &category_labels, series: &series,
        offset_groups: &offset_groups, widths: &widths,
        super_categories: &super_categories,
        icon_size: o.icon_size.unwrap_or(24),
        max_icons_per_column: o.max_icons_per_column.unwrap_or(10),
        units_per_icon: o.units_per_icon.unwrap_or(1.0),
        unit_description: &unit_desc,
        show_text: o.show_values.or(o.show_text).unwrap_or(false),
        corner_radius: o.corner_radius.unwrap_or(0),
        bar_gap: o.bar_gap.unwrap_or(0.2),
        bargroup_gap: o.bargroup_gap.unwrap_or(0.1),
        orientation: o.orient_byte(),
        theme: ChartTheme::from_str(o.theme.as_deref().unwrap_or("none")),
    };
    let html = render_bar_html(&cfg);
    use crate::plot::statistical::BarVariant::*;
    let native = matches!(variant, Basic | Horizontal | Grouped | Stacked);
    if native { apply_h(html, &o) } else { apply(html, &o) }
}

#[crate::sera_alias("line", "line_chart", "line_unified", "lines_unified", "line_family", "lines_family")]
pub fn build_line(input: &str) -> String {
    use crate::plot::statistical::{LineVariant, LineConfig, render_line_html};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = LineVariant::from_str(o.variant.as_deref().unwrap_or("basic"));

    let x_labels = a.x_labels.clone().unwrap_or_default();
    let labels = a.labels.clone().unwrap_or_else(|| x_labels.clone());
    let values = a.values.clone().unwrap_or_default();
    let hover = o.hj();
    let palette = o.pal();
    let xl = o.xl(); let yl = o.yl(); let srt = o.srt(); let lp = o.lp();

    let series: Vec<(String, Vec<f64>)> = {
        let sn = o.series_names.clone().unwrap_or_default();
        if let Some(s) = a.series.as_ref() {
            s.iter().enumerate().map(|(si, vals)| (
                sn.get(si).cloned().unwrap_or_else(|| format!("S{}", si + 1)),
                vals.clone(),
            )).collect()
        } else {
            Vec::new()
        }
    };

    let step_shape = o.step_shape.clone().or_else(|| o.line_shape.clone()).unwrap_or_else(|| "hv".to_string());
    let dash_pattern = o.dash_pattern.clone().unwrap_or_else(|| "auto".to_string());

    let cfg = LineConfig {
        variant, title,
        x_label: &xl, y_label: &yl,
        width: o.w(900), height: o.h(480),
        gridlines: o.grid(), sort_order: &srt, legend_position: &lp,
        hover: &hover, palette: &palette,
        labels: &labels, values: &values,
        color_hex: o.color_hex.unwrap_or(0),
        show_points: o.show_points.unwrap_or(false),
        series: &series, x_labels: &x_labels,
        step_shape: &step_shape,
        spline_tension: o.spline_tension.unwrap_or(0.5),
        fill_opacity: o.fill_opacity_f.unwrap_or_else(|| o.fill_opacity.map(|i| i as f64 / 100.0).unwrap_or(0.3)),
        stack_fill: o.stack_fill.unwrap_or(false),
        dash_pattern: &dash_pattern,
        stroke_width: o.stroke_width.unwrap_or(2.0),
        marker_size: o.marker_size.unwrap_or(4),
        gap_threshold: o.gap_threshold.unwrap_or(f64::NAN),
        spark_cols: o.spark_cols.unwrap_or(3),
        spark_cell_h: o.spark_cell_h.unwrap_or(60),
        spark_cell_w: o.spark_cell_w.unwrap_or(220),
    };
    let html = render_line_html(&cfg);
    apply(html, &o)
}

#[crate::sera_alias("hbar", "barh", "horizontal_bar")]
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

#[crate::sera_alias("dbscan", "dbscans", "dbscan_chart", "DBSCAN")]
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

#[crate::sera_alias("dbscan3d", "dbscan_3d", "dbscan3d_chart")]
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

#[crate::sera_alias("kmeans", "kmeans_chart")]
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

#[crate::sera_alias("scatter", "scatter_chart", "scatter_family", "scatter_unified", "scatters")]
pub fn build_scatter_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let raw_labels = a.labels.unwrap_or_default();
    let x: Vec<f64> = a.x.unwrap_or_else(|| {
        raw_labels.iter().enumerate().map(|(i, s)| s.parse::<f64>().unwrap_or(i as f64)).collect()
    });
    let y: Vec<f64> = a.y.unwrap_or_else(|| a.values.unwrap_or_default());
    let lbls = raw_labels;
    let sz = a.sizes.unwrap_or_default();
    let cgs = o.color_groups.clone().unwrap_or_default();
    let cats_arg = a.categories.clone().unwrap_or_default();
    let categories: Vec<String> = if !cats_arg.is_empty() { cats_arg } else { cgs.clone() };
    if o.variant.is_some() || !o.color_values.clone().unwrap_or_default().is_empty() || o.symbol.is_some() || o.symbols.is_some() {
        use crate::plot::statistical::scatter::{ScatterConfig, ScatterVariant, render_scatter_variant_html};
        let palette = o.pal();
        let hover = o.hj();
        let xl = o.xl(); let yl = o.yl(); let srt = o.srt(); let lp = o.lp();
        let color_values = o.color_values.clone().unwrap_or_default();
        let syms = o.symbols.clone().unwrap_or_default();
        let symbol = o.symbol.clone().unwrap_or_else(|| "circle".to_string());
        let reg_t = o.regression_type.clone().unwrap_or_else(|| "linear".to_string());
        let mut variant = ScatterVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
        if o.show_regression.unwrap_or(false) && variant == ScatterVariant::Basic {
            variant = ScatterVariant::Regression;
        }
        let cfg = ScatterConfig {
            variant, title,
            x_label: &xl, y_label: &yl,
            width: o.w(900), height: o.h(500),
            gridlines: o.grid(), sort_order: &srt, legend_position: &lp,
            hover: &hover, palette: &palette,
            x_values: &x, y_values: &y,
            categories: &categories, labels: &lbls, color_values: &color_values,
            symbols: &syms,
            color_hex: o.color_hex.unwrap_or(0),
            color_low: o.color_low.unwrap_or(0x6366F1),
            color_high: o.color_high.unwrap_or(0xF43F5E),
            point_size: o.point_size.unwrap_or(5.0),
            stroke_width: o.stroke_width.unwrap_or(1.0),
            show_text: o.show_values.or(o.show_text).unwrap_or(false),
            symbol: &symbol,
            regression_type: &reg_t,
        };
        let html = render_scatter_variant_html(&cfg);
        return apply(html, &o);
    }
    let hover = o.hj();
    let html = crate::plot::default::render_scatter_html(
        title, &x, &y, &lbls, o.w(900), o.h(540), &hover, &sz, &cgs,
        &o.pal(), &o.xl(), &o.yl(), o.color_hex.unwrap_or(0), o.grid(),
        o.show_text.unwrap_or(false), o.show_regression.unwrap_or(false),
        o.regression_type.as_deref().unwrap_or("linear"),
    );
    apply(html, &o)
}

#[crate::sera_alias("hist", "histogram", "histograms", "histogram_family", "histogram_unified")]
pub fn build_histogram(input: &str) -> String {
    use crate::plot::statistical::{HistogramConfig, HistogramVariant, render_histogram_html};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let overlay = a.overlay.clone().unwrap_or_default();
    let cats = o.color_groups.clone().unwrap_or_default();
    let palette = o.pal();
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names = if sn.len() >= 2 { Some((sn[0].as_str(), sn[1].as_str())) } else { None };
    let ref_names: Option<(&str, &str)> = names.as_ref().map(|(a, b)| (*a, *b));
    let mut variant = HistogramVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let overlay_opt: Option<&[f64]> = if overlay.is_empty() { None } else { Some(&overlay) };
    let orient = o.orient_byte();
    if orient == b'h' && matches!(variant, HistogramVariant::Basic) {
        variant = HistogramVariant::Horizontal;
    }
    let color = match o.color_hex {
        Some(0) | None => palette.get(0).copied().unwrap_or(0x6366F1),
        Some(c) => c,
    };
    let html = render_histogram_html(&HistogramConfig {
        title, variant, values: &values, bins: o.bins.unwrap_or(0) as usize,
        color,
        overlay_color: o.overlay_color_hex.unwrap_or(0xF43F5E),
        overlay_values: overlay_opt,
        categories: &cats, palette: &palette,
        stroke_width: o.stroke_width.unwrap_or(1.0),
        gap: o.gap.unwrap_or(2),
        orientation: orient,
        series_names: ref_names,
        x_label: &o.xl(), y_label: &o.yl(),
        show_counts: o.show_counts.unwrap_or(false),
        gridlines: o.grid(), width: o.w(860), height: o.h(380), hover: &hover,
        sort_order: &o.srt(),
        theme: { use crate::plot::statistical::ChartTheme; ChartTheme::from_str(o.theme.as_deref().unwrap_or("none")) },
        ..HistogramConfig::default()
    });
    use crate::plot::statistical::HistogramVariant::*;
    let native = matches!(variant, Basic | Horizontal);
    if native { apply_h(html, &o) } else { apply(html, &o) }
}

pub fn build_histogram_overlay(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let overlay = a.overlay.unwrap_or_default();
    use crate::plot::statistical::{HistogramConfig, HistogramVariant, render_histogram_html};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names = if sn.len() >= 2 { Some((sn[0].as_str(), sn[1].as_str())) } else { None };
    let ref_names: Option<(&str, &str)> = names.as_ref().map(|(a, b)| (*a, *b));
    let html = render_histogram_html(&HistogramConfig {
        title, variant: HistogramVariant::Overlay,
        values: &values, bins: o.bins.unwrap_or(0) as usize,
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

#[crate::sera_alias("grouped_bar", "grouped_bars", "grouped_bar_chart", "group_bar")]
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
        sort_order: &o.srt(), hover: &hover, orientation: o.orient_byte(), ..GroupedBarConfig::default()
    });
    apply_h(html, &o)
}

#[crate::sera_alias("stacked_bar", "stacked_bars", "stacked_bar_chart", "stack_bar")]
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
        sort_order: &o.srt(), hover: &hover, stacked: true, orientation: o.orient_byte(), ..GroupedBarConfig::default()
    });
    apply_h(html, &o)
}

#[crate::sera_alias("heatmap", "heatmaps", "heatmap_family", "heatmap_unified")]
pub fn build_heatmap(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let flat_matrix = a.values.unwrap_or_default();
    use crate::plot::statistical::{HeatmapConfig, HeatmapVariant, render_heatmap_html};
    let col_lbl = o.col_labels.clone().unwrap_or_default();
    let hover = o.hj();
    let palette = o.pal();
    let variant = HeatmapVariant::from_str(&o.variant.clone().unwrap_or_default());
    let x_widths: Vec<f64> = o.widths.clone().unwrap_or_default();
    let y_heights: Vec<f64> = o.ranges.clone().unwrap_or_default();
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let colorbar_position = o.colorbar_position.clone().unwrap_or_else(|| "right".to_string());
    let html = render_heatmap_html(&HeatmapConfig {
        title, variant,
        row_labels: &labels, col_labels: &col_lbl, flat_matrix: &flat_matrix,
        show_values: o.show_values.unwrap_or(true),
        color_low: o.color_low.unwrap_or(0x6366F1),
        color_mid: o.color_mid.unwrap_or(0xfafbfc),
        color_high: o.color_high.unwrap_or(0xF43F5E),
        palette: &palette,
        discrete_steps: o.bins.unwrap_or(0).max(0) as usize,
        x_widths: &x_widths,
        y_heights: &y_heights,
        colorscale: &colorscale,
        colorbar_position: &colorbar_position,
        origin_lower: o.origin_lower.unwrap_or(false),
        width: o.w(720), height: o.h(440), hover: &hover,
        sort_order: &o.srt(), ..HeatmapConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("pie_chart", "pie_chart_legacy", "basic_pie")]
pub fn build_pie_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{PieConfig, PieVariant, render_pie_html};
    let hover = o.hj();
    let pull = o.pull.clone().unwrap_or_default();
    let html = render_pie_html(&PieConfig {
        variant: PieVariant::Basic,
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true), sort_order: &o.srt(),
        width: o.w(720), height: o.h(440), hover: &hover,
        pull: &pull,
        min_label_frac: o.min_label_frac.unwrap_or(0.04),
        ..PieConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("donut", "donut_chart")]
pub fn build_donut_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{PieConfig, PieVariant, render_pie_html};
    let hover = o.hj();
    let pull = o.pull.clone().unwrap_or_default();
    let html = render_pie_html(&PieConfig {
        variant: PieVariant::Donut,
        title, labels: &labels, values: &values, palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true), sort_order: &o.srt(),
        width: o.w(720), height: o.h(440), hover: &hover,
        donut: o.inner_radius_ratio.unwrap_or(0.55).clamp(0.0, 0.9),
        pull: &pull,
        min_label_frac: o.min_label_frac.unwrap_or(0.04),
        ..PieConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("pie", "pie_unified", "pie_family", "pies", "semi_pie", "half_pie", "kpi_pie", "kpi_donut", "nested_pie", "concentric_pie", "pattern_pie")]
pub fn build_pie(input: &str) -> String {
    use crate::plot::statistical::{PieConfig, PieVariant, render_pie_html};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = PieVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let labels = a.labels.clone().unwrap_or_default();
    let values = a.values.clone().unwrap_or_default();
    let series = a.series.clone().unwrap_or_default();
    let pull = o.pull.clone().unwrap_or_default();
    let subplot_titles = o.subplot_titles.clone().unwrap_or_default();
    let secondary_values = o.secondary_values.clone().unwrap_or_default();
    let secondary_labels = o.secondary_labels.clone().unwrap_or_default();
    let center_text = o.center_text.clone().unwrap_or_default();
    let center_subtext = o.center_subtext.clone().unwrap_or_default();
    let pattern = o.pattern.clone().unwrap_or_default();
    let hover = o.hj();
    let palette = o.pal();
    let srt = o.srt();
    let lp = o.lp();

    let cfg = PieConfig {
        variant, title,
        x_label: "", y_label: "",
        gridlines: false, sort_order: &srt,
        hover: &hover, legend_position: &lp,
        width: o.w(720), height: o.h(440),
        labels: &labels, values: &values,
        donut: o.inner_radius_ratio.unwrap_or(0.0).clamp(0.0, 0.9),
        show_pct: o.show_pct.unwrap_or(true),
        min_label_frac: o.min_label_frac.unwrap_or(0.04),
        palette: &palette,
        pull: &pull,
        series: &series,
        subplot_titles: &subplot_titles,
        subplot_cols: o.subplot_cols.unwrap_or(0),
        proportional: o.proportional.unwrap_or(false),
        center_text: &center_text,
        center_subtext: &center_subtext,
        secondary_values: &secondary_values,
        secondary_labels: &secondary_labels,
        pattern: &pattern,
    };
    let html = render_pie_html(&cfg);
    apply(html, &o)
}

#[crate::sera_alias("boxplot", "box_plot")]
pub fn build_boxplot(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let series = a.series.unwrap_or_default();
    let series_names = o.series_names.clone().unwrap_or_default();
    let hover = o.hj();
    let mut variant = crate::plot::statistical::BoxplotVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    if o.is_horiz() && matches!(variant, crate::plot::statistical::BoxplotVariant::Basic) {
        variant = crate::plot::statistical::BoxplotVariant::Horizontal;
    }
    let html = crate::plot::statistical::render_boxplot_html(&crate::plot::statistical::BoxplotConfig {
        title,
        variant,
        category_labels: &category_labels,
        values: &values,
        series: &series,
        series_names: &series_names,
        palette: &o.pal(),
        hover: &hover,
        x_label: &o.xl(),
        y_label: &o.yl(),
        gridlines: o.grid(),
        width: o.w(900),
        height: o.h(500),
        sort_order: &o.srt(),
        legend_position: &o.lp(),
        notch: o.notch.unwrap_or(false),
        show_points: o.show_points.unwrap_or(false),
        jitter: o.jitter.unwrap_or(0.35),
        violin_overlay: o.violin_overlay.unwrap_or(false),
        boxen_depth: o.boxen_depth.unwrap_or(4),
        fill_opacity: o.fill_opacity_real.unwrap_or(0.28),
        stroke_width: o.box_stroke_width.unwrap_or(1.6),
    });
    use crate::plot::statistical::BoxplotVariant::*;
    let native = matches!(variant, Horizontal);
    if native { apply_h(html, &o) } else { apply(html, &o) }
}

#[crate::sera_alias("violin", "violins", "violin_chart", "violin_family", "violin_unified")]
pub fn build_violin(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let categories = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{ViolinConfig, ViolinVariant, render_violin_html};
    let hover = o.hj();
    let mut variant = ViolinVariant::from_str(o.variant.as_deref().unwrap_or("box"));
    if o.is_horiz() && matches!(variant, ViolinVariant::Basic | ViolinVariant::Box | ViolinVariant::Quartile | ViolinVariant::Mean) {
        variant = ViolinVariant::Horizontal;
    }
    let html = render_violin_html(&ViolinConfig {
        title, variant, categories: &categories, values: &values,
        x_label: &o.xl(), y_label: &o.yl(), palette: &o.pal(), gridlines: o.grid(),
        width: o.w(900), height: o.h(500), sort_order: &o.srt(), hover: &hover,
        legend_position: &o.lp(),
        bandwidth: o.bandwidth.unwrap_or(1.0),
        fill_opacity: o.fill_opacity_real.unwrap_or(0.55),
        stroke_width: o.box_stroke_width.unwrap_or(1.4),
        show_box: o.show_box.unwrap_or(false),
        show_points: o.show_points.unwrap_or(false),
        show_mean: o.show_mean.unwrap_or(false),
        jitter: o.jitter.unwrap_or(0.35),
        kde_steps: 32,
    });
    let native = matches!(variant, ViolinVariant::Horizontal);
    if native { apply_h(html, &o) } else { apply(html, &o) }
}

#[crate::sera_alias("slope", "slopes", "slope_chart", "slope_family", "slopegraph")]
pub fn build_slope(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_left = a.left.unwrap_or_default();
    let values_right = a.right.unwrap_or_default();
    use crate::plot::statistical::{SlopeConfig, SlopeVariant, render_slope_html};
    let hover = o.hj();
    let ll = o.left_label.as_deref().unwrap_or("Before").to_string();
    let rl = o.right_label.as_deref().unwrap_or("After").to_string();
    let variant = SlopeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_slope_html(&SlopeConfig {
        title, variant, labels: &labels, values_left: &values_left, values_right: &values_right,
        left_label: &ll, right_label: &rl, palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true), width: o.w(700), height: o.h(500),
        sort_order: &o.srt(), hover: &hover, ..SlopeConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("sunburst", "sunbursts", "sunburst_chart", "sunburst_family", "sunburst_unified")]
pub fn build_sunburst(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{SunburstConfig, SunburstVariant, render_sunburst_html};
    let hover = o.hj();
    let variant = SunburstVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_sunburst_html(&SunburstConfig {
        title, variant, labels: &labels, parents: &parents, values: &values, palette: &o.pal(),
        width: o.w(700), height: o.h(700), hover: &hover, ..SunburstConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("funnel", "funnels", "funnel_chart", "funnel_family", "funnel_unified")]
pub fn build_funnel(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{FunnelConfig, FunnelVariant, render_funnel_html};
    let hover = o.hj();
    let variant = FunnelVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_funnel_html(&FunnelConfig {
        title, variant, labels: &labels, values: &values, palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true), width: o.w(800), height: o.h(480),
        sort_order: &o.srt(), hover: &hover, ..FunnelConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("treemap", "treemaps", "treemap_chart", "treemap_family", "treemap_unified")]
pub fn build_treemap(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let pars = a.parents.unwrap_or_default();
    use crate::plot::statistical::{TreemapConfig, TreemapVariant, render_treemap_html};
    let hover = o.hj();
    let variant = TreemapVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_treemap_html(&TreemapConfig {
        title, labels: &labels, values: &values, parents: &pars,
        palette: &o.pal(), sort_order: &o.srt(), width: o.w(1100), height: o.h(520),
        hover: &hover, variant, ..TreemapConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("multiline", "multilines", "multiline_chart", "multiline_family", "multi_line", "multi_line_chart")]
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

#[crate::sera_alias("area", "area_chart")]
pub fn build_area_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let values_fallback = a.values;
    let series_flat = a.series.unwrap_or_else(|| values_fallback.map(|v| vec![v]).unwrap_or_default());
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

#[crate::sera_alias("waterfall", "waterfalls", "waterfall_chart", "waterfall_family")]
pub fn build_waterfall(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{WaterfallConfig, WaterfallVariant, render_waterfall_html};
    let hover = o.hj();
    let variant = WaterfallVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_waterfall_html(&WaterfallConfig {
        title, variant, labels: &labels, values: &values, x_label: &o.xl(), y_label: &o.yl(),
        show_text: o.show_text.unwrap_or(true), gridlines: o.grid(),
        width: o.w(900), height: o.h(480), sort_order: &o.srt(), hover: &hover,
        legend_position: &o.lp(), orientation: o.orient_byte(),
    });
    apply_h(html, &o)
}

#[crate::sera_alias("bullet", "bullets", "bullet_chart", "bullet_family", "bullet_graph")]
pub fn build_bullet(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{BulletConfig, BulletVariant, render_bullet_html};
    let variant = BulletVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let targets = o.targets.clone().unwrap_or_default();
    let max_vals = o.max_vals.clone().unwrap_or_default();
    let ranges = o.ranges.clone().unwrap_or_default();
    let comparisons = o.comparisons.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_bullet_html(&BulletConfig {
        variant, title, labels: &labels, values: &values, targets: &targets,
        max_vals: &max_vals, ranges: &ranges, comparisons: &comparisons,
        width: o.w(800), height: o.h(300),
        hover: &hover, ..BulletConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("bubble_map", "bubblemap", "bubble_map_chart", "geo_bubble", "geo_bubble_map")]
pub fn build_bubble_map(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::map::render_bubble_map_html(title, &labels, &values, o.w(1200), o.h(600), &hover);
    apply(html, &o)
}

#[crate::sera_alias("choropleth", "choropleths", "choropleth_map", "choropleth_chart", "geo_map")]
pub fn build_choropleth(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let html = crate::plot::map::render_choropleth_html(title, &labels, &values, o.w(1200), o.h(600), &hover);
    apply(html, &o)
}

#[crate::sera_alias("scatter3d", "scatter_3d", "scatter3d_chart", "scatter3d_family", "scatters3d")]
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

#[crate::sera_alias("bar3d", "bar_3d", "bar3d_chart", "bar3d_family", "bars3d")]
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

#[crate::sera_alias("line3d", "line_3d", "line3d_chart", "line3d_family", "lines3d")]
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

#[crate::sera_alias("radar", "radar_chart")]
pub fn build_radar_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    use crate::plot::statistical::{RadarConfig, RadarVariant, render_radar_html};
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let hover = o.hj();
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let variant = RadarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_radar_html(&RadarConfig {
        title, variant, axes: &axes, series: &series, palette: &o.pal(),
        filled: o.filled.unwrap_or(true), fill_opacity: o.fill_opacity.unwrap_or(50) as u8,
        width: o.w(700), height: o.h(560), hover: &hover, ..RadarConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("radar3d", "radar_3d", "radar3d_chart", "radar3d_family")]
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

#[crate::sera_alias("lollipop", "lollipops", "lollipop_chart", "lollipop_family", "lollipop_unified")]
pub fn build_lollipop_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{LollipopConfig, LollipopVariant, render_lollipop_html};
    let mut variant = LollipopVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let orient = o.orient_byte();
    if o.variant.is_none() && orient == b'h' { variant = LollipopVariant::Cleveland; }
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_lollipop_html(&LollipopConfig {
        variant, title, labels: &labels, values: &values, groups: &groups,
        x_label: &o.xl(), y_label: &o.yl(),
        palette: &o.pal(), color_hex: o.color_hex.unwrap_or(0), gridlines: o.grid(),
        show_values: o.show_values.unwrap_or(false),
        highlight_index: o.highlight_index.unwrap_or(-1),
        sort_order: &o.srt(), width: o.w(900), height: o.h(480), hover: &hover,
        legend_position: &o.lp(),
    });
    apply_h(html, &o)
}

#[crate::sera_alias("lollipop3d", "lollipop_3d", "lollipop3d_chart")]
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

#[crate::sera_alias("kde", "kdes", "kde_chart", "kde_family", "density", "density_plot")]
pub fn build_kde_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{KdeConfig, KdeVariant, render_kde_html};
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
    let variant = KdeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_kde_html(&KdeConfig {
        title, variant, series: &series, palette: &o.pal(), x_label: &xl, y_label: &yl,
        bandwidth: o.bandwidth.unwrap_or(0.0), filled: o.filled.unwrap_or(true),
        fill_opacity: o.fill_opacity.unwrap_or(50) as u8, gridlines: o.grid(),
        bins: o.bins.unwrap_or(0) as usize,
        width: o.w(900), height: o.h(420), sort_order: &o.srt(), hover: &hover,
        ..KdeConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("kde3d", "kde_3d", "kde3d_chart", "density3d")]
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

#[crate::sera_alias("ridgeline", "ridgelines", "ridgeline_chart", "ridgeline_family", "joy_plot", "joyplot")]
pub fn build_ridgeline_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let categories = a.categories.or(a.labels).unwrap_or_default();
    use crate::plot::statistical::{RidgelineConfig, RidgelineVariant, render_ridgeline_html};
    let hover = o.hj();
    let variant = RidgelineVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_ridgeline_html(&RidgelineConfig {
        title, variant, values: &values, categories: &categories, palette: &o.pal(),
        x_label: &o.xl(), y_label: &o.yl(), overlap: o.overlap.unwrap_or(0.5),
        bandwidth: o.bandwidth.unwrap_or(0.0), width: o.w(900), height: o.h(520),
        gridlines: o.grid(), sort_order: &o.srt(), hover: &hover, ..RidgelineConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("ridgeline3d", "ridgeline_3d", "ridgeline3d_chart", "joy_plot3d")]
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

#[crate::sera_alias("bubble3d", "bubble_3d", "bubble3d_chart", "bubble3d_family", "bubbles3d")]
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

#[crate::sera_alias("pie3d", "pie_3d", "pie3d_chart", "pie3d_family", "pies3d")]
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

#[crate::sera_alias("violin3d", "violin_3d", "violin3d_chart", "violins3d")]
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

#[crate::sera_alias("heatmap3d", "heatmap_3d", "heatmap3d_chart", "heatmaps3d")]
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

#[crate::sera_alias("candlestick3d", "candlestick_3d", "candlestick3d_chart", "ohlc3d")]
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

#[crate::sera_alias("dumbbell3d", "dumbbell_3d", "dumbbell3d_chart")]
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

#[crate::sera_alias("funnel3d", "funnel_3d", "funnel3d_chart")]
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

#[crate::sera_alias("sunburst3d", "sunburst_3d", "sunburst3d_chart")]
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

#[crate::sera_alias("stacked_bar3d", "stacked_bar_3d", "stacked_bar3d_chart")]
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

#[crate::sera_alias("globe3d", "globe_3d", "globe3d_chart", "globe")]
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

#[crate::sera_alias("wordcloud", "word_cloud", "wordCloud", "tag_cloud", "tagcloud", "cloud", "token_cloud", "text_cloud")]
pub fn build_wordcloud(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let words = a.words.unwrap_or_default();
    let frequencies = a.frequencies.unwrap_or_else(|| a.values.unwrap_or_default());
    let pal = o.pal();
    let hover = o.hj();
    let bg_str = o.bg_str();
    use crate::plot::statistical::wordcloud::{WordCloudConfig, WordCloudVariant, WordCloudShape, render_wordcloud_html};
    let variant = WordCloudVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let shape = WordCloudShape::from_str(o.shape.as_deref().unwrap_or("rect"));
    let mask = o.mask.clone().unwrap_or_default();
    let points_x = o.points_x.clone().unwrap_or_default();
    let points_y = o.points_y.clone().unwrap_or_default();
    let point_clusters = o.category_indices.clone().unwrap_or_default();
    let cluster_labels = o.cluster_labels.clone().unwrap_or_default();
    let edges_i = o.edges_i.clone().unwrap_or_default();
    let edges_j = o.edges_j.clone().unwrap_or_default();
    let edges_w = o.edges_w.clone().unwrap_or_default();
    let html = render_wordcloud_html(&WordCloudConfig {
        variant, shape, title, words: &words, frequencies: &frequencies,
        palette: &pal, width: o.w(900), height: o.h(500),
        min_font: o.min_font.unwrap_or(12.0), max_font: o.max_font.unwrap_or(72.0),
        bg_color: bg_str.as_deref(), sort_order: &o.srt(), hover: &hover,
        mask: &mask, mask_width: o.mask_width.unwrap_or(0), mask_height: o.mask_height.unwrap_or(0),
        points_x: &points_x, points_y: &points_y, point_clusters: &point_clusters,
        cluster_labels: &cluster_labels,
        edges_i: &edges_i, edges_j: &edges_j, edges_w: &edges_w,
        ..WordCloudConfig::default()
    });
    apply_bg3d(html, &o)
}

#[crate::sera_alias("candlestick", "candlesticks", "candlestick_chart", "candlestick_family", "ohlc", "ohlc_chart")]
pub fn build_candlestick(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
    use crate::plot::statistical::{CandlestickConfig, CandlestickVariant, render_candlestick_html};
    let hover = o.hj();
    let variant = CandlestickVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_candlestick_html(&CandlestickConfig {
        title, labels: &labels, open: &open, high: &high, low: &low, close: &close,
        palette: &o.pal(), width: o.w(1100), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, variant, ..CandlestickConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("dumbbell", "dumbbells", "dumbbell_chart", "dumbbell_family", "dumbbell_plot")]
pub fn build_dumbbell(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    use crate::plot::statistical::dumbbell::{DumbbellConfig, DumbbellVariant, render_dumbbell_html};
    let variant = DumbbellVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let s = o.series_name_start.as_deref().unwrap_or("Start").to_string();
    let e = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let hover = o.hj();
    let html = render_dumbbell_html(&DumbbellConfig {
        variant, title, labels: &labels, values_start: &values_start, values_end: &values_end,
        series_names: (&s, &e), palette: &o.pal(), width: o.w(1000), height: o.h(500),
        x_label: &o.xl(), y_label: &o.yl(), gridlines: o.grid(),
        sort_order: &o.srt(), hover: &hover, ..DumbbellConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("bubble", "bubble_family", "bubble_unified", "bubbles")]
pub fn build_bubble(input: &str) -> String {
    use crate::plot::statistical::bubble::{BubbleConfig, BubbleVariant, render_bubble_html};
    use crate::plot::statistical::ChartTheme;
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = BubbleVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let sizes = a.sizes.or(a.size).unwrap_or_default();
    let categories = a.categories.clone().or_else(|| o.color_groups.clone()).unwrap_or_default();
    let labels = a.labels.clone().unwrap_or_default();
    let color_values = o.color_values.clone().unwrap_or_default();
    let palette = o.pal();
    let hover = o.hj();
    let xl = o.xl(); let yl = o.yl(); let srt = o.srt(); let lp = o.lp();
    let cfg = BubbleConfig {
        variant, title,
        x_label: &xl, y_label: &yl,
        width: o.w(900), height: o.h(500),
        gridlines: o.grid(), sort_order: &srt, legend_position: &lp,
        hover: &hover, palette: &palette,
        x_values: &x_values, y_values: &y_values, sizes: &sizes,
        categories: &categories, labels: &labels, color_values: &color_values,
        color_hex: o.color_hex.unwrap_or(0),
        color_low: o.color_low.unwrap_or(0x6366F1),
        color_high: o.color_high.unwrap_or(0xF43F5E),
        min_size: o.min_size.unwrap_or(4.0),
        max_size: o.max_size.unwrap_or(40.0),
        show_text: o.show_values.or(o.show_text).unwrap_or(false),
        stroke_width: o.stroke_width.unwrap_or(1.5),
        theme: ChartTheme::from_str(o.theme.as_deref().unwrap_or("none")),
    };
    let html = render_bubble_html(&cfg);
    apply(html, &o)
}

#[crate::sera_alias("gauge", "gauges", "gauge_chart", "gauge_family", "speedometer")]
pub fn build_gauge(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let value = a.value.unwrap_or(0.0);
    let hover = o.hj();
    use crate::plot::statistical::gauge::{GaugeConfig, GaugeVariant, render_gauge_html};
    let variant = GaugeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let comparison = o.comparison.unwrap_or(0.0);
    let lbl = o.label.as_deref().unwrap_or("").to_string();
    let html = render_gauge_html(&GaugeConfig {
        variant, title, value, min_val: o.min_val.unwrap_or(0.0), max_val: o.max_val.unwrap_or(100.0),
        label: &lbl, comparison, width: o.w(400), height: o.h(300),
        hover: &hover, ..GaugeConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("parallel", "parallels", "parallel_chart", "parallel_family", "parallel_coordinates", "parcoords")]
pub fn build_parallel(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
    let pal = o.pal();
    let hover = o.hj();
    use crate::plot::statistical::parallel::{ParallelConfig, ParallelVariant, render_parallel_html};
    let variant = ParallelVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_values.len()).map(|_| String::new()).collect());
    let categories = o.category_indices.clone().unwrap_or_default();
    let html = render_parallel_html(&ParallelConfig {
        variant, title, axes: &axes, series_names: &names, series_values: &series_values,
        categories: &categories, palette: &pal, width: o.w(1000), height: o.h(500),
        highlight_index: o.highlight_index.unwrap_or(-1),
        color_axis: o.color_axis.unwrap_or(-1),
        hover: &hover,
        theme: { use crate::plot::statistical::ChartTheme; ChartTheme::from_str(o.theme.as_deref().unwrap_or("none")) },
        ..ParallelConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("plot", "chart", "draw", "render")]
pub fn plot_chart(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct I {
        x: Option<Vec<f64>>,
        y: Option<Vec<f64>>,
        kind: Option<String>,
        title: Option<String>,
        color_hex: Option<u32>,
        width: Option<i32>,
        height: Option<i32>,
        x_label: Option<String>,
        y_label: Option<String>,
        gridlines: Option<bool>,
        palette: Option<Vec<u32>>,
        background: Option<String>,
        show_points: Option<bool>,
    }
    let p: I = serde_json::from_str(input).unwrap_or_default();
    let xv = p.x.unwrap_or_default();
    let title = p.title.unwrap_or_default();
    let color_hex = p.color_hex.unwrap_or(0x6366F1);
    let width = p.width.unwrap_or(900);
    let height = p.height.unwrap_or(480);
    let x_label = p.x_label.unwrap_or_default();
    let y_label = p.y_label.unwrap_or_default();
    let gridlines = p.gridlines.unwrap_or(false);
    let show_points = p.show_points.unwrap_or(true);
    let kind = p.kind.as_deref().unwrap_or("line");
    if let Some(yv) = p.y {
        if kind == "scatter" {
            build_scatter_chart(&serde_json::json!({"title":title,"x":xv,"y":yv,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":gridlines,"palette":p.palette,"background":p.background}).to_string())
        } else {
            let labels: Vec<String> = xv.iter().map(|v| v.to_string()).collect();
            build_line_chart(&serde_json::json!({"title":title,"labels":labels,"values":yv,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":gridlines,"palette":p.palette,"background":p.background}).to_string())
        }
    } else {
        let labels: Vec<String> = (0..xv.len()).map(|idx| idx.to_string()).collect();
        build_line_chart(&serde_json::json!({"title":title,"labels":labels,"values":xv,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":gridlines,"palette":p.palette,"background":p.background}).to_string())
    }
}

#[crate::sera_alias("grid", "grids", "chart_grid", "subplot_grid")]
pub fn build_grid(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let charts = a.charts.unwrap_or_default();
    let cols = o.cols.unwrap_or(2).max(1);
    let gap = o.gap.unwrap_or(16);
    let bg_color = o.background.clone().or_else(get_global_bg).unwrap_or_else(|| "transparent".to_string());
    let bg_color = bg_color.as_str();
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
    let mut frames_html = String::new();
    for (i, html) in charts.iter().enumerate() {
        let esc = html.replace('&', "&amp;").replace('"', "&quot;");
        let vis = if i == 0 { "" } else { "display:none;" };
        frames_html.push_str(&format!(
            "<iframe id=\"sp-s-{i}\" style=\"{vis}width:{width}px;height:{height}px;border:none;border-radius:12px;overflow:hidden;box-shadow:0 2px 12px rgba(0,0,0,.1)\" srcdoc=\"{esc}\"></iframe>"
        ));
    }
    format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        body{{margin:0;padding:24px;background:#f0f2f5;display:flex;flex-direction:column;align-items:center;font-family:system-ui}}\
        .sp-ctrl{{display:flex;gap:10px;margin-top:14px;align-items:center}}\
        .sp-btn{{cursor:pointer;background:#6366f1;color:#fff;border:none;border-radius:8px;padding:7px 20px;font-size:14px;font-weight:600}}\
        .sp-btn:hover{{background:#4f46e5}}\
        .sp-ctr{{color:#64748b;font-size:13px;min-width:64px;text-align:center}}\
        .sp-prog{{width:{width}px;height:4px;background:#e2e8f0;border-radius:2px;margin-top:10px;overflow:hidden}}\
        .sp-bar{{height:100%;background:#6366f1;border-radius:2px;width:0%}}\
        </style></head><body>\
        {title_html}\
        {frames_html}\
        <div class=\"sp-ctrl\">\
        <button class=\"sp-btn\" id=\"sp-p\">&#9664;</button>\
        <div class=\"sp-ctr\" id=\"sp-c\">1 / {n}</div>\
        <button class=\"sp-btn\" id=\"sp-n\">&#9654;</button>\
        </div>\
        <div class=\"sp-prog\"><div class=\"sp-bar\" id=\"sp-b\"></div></div>\
        <script>\
        const slides=document.querySelectorAll('[id^=\"sp-s-\"]');\
        let idx=0,timer;\
        function show(i){{idx=((i%slides.length)+slides.length)%slides.length;\
          slides.forEach((s,j)=>{{s.style.display=j===idx?'':'none';}});\
          document.getElementById('sp-c').textContent=(idx+1)+' / '+slides.length;\
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

#[crate::sera_alias("dbscan_fit_predict", "ml_dbscan", "DBSCAN_fit_predict", "cluster_dbscan")]
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

#[crate::sera_alias("kmeans_fit_predict", "ml_kmeans", "KMeans_fit_predict", "cluster_kmeans")]
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

#[crate::sera_alias("metric_score", "ml_metric", "score_metric")]
pub fn ml_metric_score(input: &str) -> String {
    use crate::ml::metrics::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        y_true: Option<Vec<f64>>,
        y_pred: Option<Vec<f64>>,
        y_score: Option<Vec<f64>>,
        labels: Option<Vec<i32>>,
        labels_true: Option<Vec<i32>>,
        labels_pred: Option<Vec<i32>>,
        x: Option<Vec<Vec<f64>>>,
        average: Option<String>,
        pos_label: Option<i32>,
        beta: Option<f64>,
        alpha: Option<f64>,
        eps: Option<f64>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let name = i.name.unwrap_or_default();
    let to_i32 = |v: &[f64]| v.iter().map(|x| *x as i32).collect::<Vec<i32>>();
    let yt_f = i.y_true.clone().unwrap_or_default();
    let yp_f = i.y_pred.clone().unwrap_or_default();
    let ys_f = i.y_score.clone().unwrap_or_default();
    let yt_i = to_i32(&yt_f);
    let yp_i = to_i32(&yp_f);
    let pos = i.pos_label.unwrap_or(1);
    let avg = match i.average.as_deref().unwrap_or("binary") {
        "macro" => Average::Macro,
        "weighted" => Average::Weighted,
        _ => Average::Binary(pos),
    };
    let value: f64 = match name.as_str() {
        "accuracy_score" => accuracy_score(&yt_i, &yp_i),
        "balanced_accuracy_score" => balanced_accuracy_score(&yt_i, &yp_i),
        "precision_score" => precision_score(&yt_i, &yp_i, avg),
        "recall_score" => recall_score(&yt_i, &yp_i, avg),
        "f1_score" => f1_score(&yt_i, &yp_i, avg),
        "fbeta_score" => fbeta_score(&yt_i, &yp_i, i.beta.unwrap_or(1.0), avg),
        "jaccard_score" => jaccard_score(&yt_i, &yp_i, pos),
        "matthews_corrcoef" => matthews_corrcoef(&yt_i, &yp_i),
        "cohen_kappa_score" => cohen_kappa_score(&yt_i, &yp_i),
        "hamming_loss" => hamming_loss(&yt_i, &yp_i),
        "zero_one_loss" => zero_one_loss(&yt_i, &yp_i),
        "binary_log_loss" => binary_log_loss(&yt_i, &yp_f, i.eps.unwrap_or(1e-15)),
        "brier_score_loss" => brier_score_loss(&yt_i, &yp_f),
        "hinge_loss" => hinge_loss(&yt_i, &yp_f),
        "roc_auc_score" => roc_auc_score(&yt_i, &ys_f, pos),
        "average_precision_score" => average_precision_score(&yt_i, &ys_f, pos),
        "mean_squared_error" => mean_squared_error(&yt_f, &yp_f),
        "root_mean_squared_error" => root_mean_squared_error(&yt_f, &yp_f),
        "mean_absolute_error" => mean_absolute_error(&yt_f, &yp_f),
        "median_absolute_error" => median_absolute_error(&yt_f, &yp_f),
        "r2_score" => r2_score(&yt_f, &yp_f),
        "explained_variance_score" => explained_variance_score(&yt_f, &yp_f),
        "max_error" => max_error(&yt_f, &yp_f),
        "mean_absolute_percentage_error" => mean_absolute_percentage_error(&yt_f, &yp_f),
        "mean_squared_log_error" => mean_squared_log_error(&yt_f, &yp_f),
        "root_mean_squared_log_error" => root_mean_squared_log_error(&yt_f, &yp_f),
        "mean_pinball_loss" => mean_pinball_loss(&yt_f, &yp_f, i.alpha.unwrap_or(0.5)),
        "d2_absolute_error_score" => d2_absolute_error_score(&yt_f, &yp_f),
        "silhouette_score" | "davies_bouldin_score" | "calinski_harabasz_score" => {
            let rows = i.x.clone().unwrap_or_default();
            let n = rows.len();
            let p = if n > 0 { rows[0].len() } else { 0 };
            let mut flat = Vec::with_capacity(n * p);
            for r in &rows { flat.extend_from_slice(&r[..p.min(r.len())]); if r.len() < p { flat.extend(std::iter::repeat(0.0).take(p - r.len())); } }
            let labs = i.labels.clone().unwrap_or_default();
            match name.as_str() {
                "silhouette_score" => silhouette_score(&flat, &labs, n, p),
                "davies_bouldin_score" => davies_bouldin_score(&flat, &labs, n, p),
                _ => calinski_harabasz_score(&flat, &labs, n, p),
            }
        }
        "adjusted_rand_score" | "normalized_mutual_info_score" | "fowlkes_mallows_score"
        | "homogeneity_score" | "completeness_score" | "v_measure_score" => {
            let lt = i.labels_true.clone().unwrap_or_default();
            let lp = i.labels_pred.clone().unwrap_or_default();
            match name.as_str() {
                "adjusted_rand_score" => adjusted_rand_score(&lt, &lp),
                "normalized_mutual_info_score" => normalized_mutual_info_score(&lt, &lp),
                "fowlkes_mallows_score" => fowlkes_mallows_score(&lt, &lp),
                "homogeneity_score" => homogeneity_score(&lt, &lp),
                "completeness_score" => completeness_score(&lt, &lp),
                _ => v_measure_score(&lt, &lp),
            }
        }
        _ => f64::NAN,
    };
    if value.is_nan() { format!("{{\"error\":\"unknown metric: {}\"}}", name) }
    else { format!("{{\"value\":{}}}", value) }
}

#[crate::sera_alias("metric_curve", "ml_curve", "roc_curve", "pr_curve")]
pub fn ml_metric_curve(input: &str) -> String {
    use crate::ml::metrics::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        y_true: Option<Vec<f64>>,
        y_score: Option<Vec<f64>>,
        pos_label: Option<i32>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let yt: Vec<i32> = i.y_true.unwrap_or_default().iter().map(|v| *v as i32).collect();
    let ys = i.y_score.unwrap_or_default();
    let pos = i.pos_label.unwrap_or(1);
    match i.name.as_deref().unwrap_or("") {
        "roc_curve" => {
            let (a, b, c) = roc_curve(&yt, &ys, pos);
            format!("{{\"fpr\":{},\"tpr\":{},\"thresholds\":{}}}",
                serde_json::to_string(&a).unwrap_or_default(),
                serde_json::to_string(&b).unwrap_or_default(),
                serde_json::to_string(&c).unwrap_or_default())
        }
        "precision_recall_curve" => {
            let (a, b, c) = precision_recall_curve(&yt, &ys, pos);
            format!("{{\"precision\":{},\"recall\":{},\"thresholds\":{}}}",
                serde_json::to_string(&a).unwrap_or_default(),
                serde_json::to_string(&b).unwrap_or_default(),
                serde_json::to_string(&c).unwrap_or_default())
        }
        n => format!("{{\"error\":\"unknown curve: {}\"}}", n),
    }
}

#[crate::sera_alias("fit_transform", "ml_transform", "preprocess_fit_transform")]
pub fn ml_fit_transform(input: &str) -> String {
    use crate::ml::preprocessing::transformers::*;
    use crate::ml::preprocessing::scalers::*;
    #[derive(Deserialize, Default)]
    struct I {
        name: Option<String>,
        data: Option<Vec<Vec<f64>>>,
        strategy: Option<String>,
        fill_value: Option<f64>,
        degree: Option<usize>,
        interaction_only: Option<bool>,
        include_bias: Option<bool>,
        n_bins: Option<usize>,
        method: Option<String>,
        n_quantiles: Option<usize>,
        output_distribution: Option<String>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let rows = i.data.unwrap_or_default();
    let n = rows.len();
    let p = if n > 0 { rows[0].len() } else { 0 };
    let mut flat = Vec::with_capacity(n * p);
    for r in &rows {
        flat.extend_from_slice(&r[..p.min(r.len())]);
        if r.len() < p { flat.extend(std::iter::repeat(0.0).take(p - r.len())); }
    }
    let (out, cols, extra): (Vec<f64>, usize, String) = match i.name.as_deref().unwrap_or("") {
        "SimpleImputer" => {
            let mut t = SimpleImputer::new(i.strategy.as_deref().unwrap_or("mean"), i.fill_value.unwrap_or(0.0));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, format!(",\"statistics\":{}", serde_json::to_string(&t.statistics).unwrap_or_default()))
        }
        "PolynomialFeatures" => {
            let mut t = PolynomialFeatures::new(i.degree.unwrap_or(2), i.interaction_only.unwrap_or(false), i.include_bias.unwrap_or(true));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            let nf = t.n_output_features();
            (o, nf, format!(",\"n_features_out\":{}", nf))
        }
        "KBinsDiscretizer" => {
            let mut t = KBinsDiscretizer::new(i.n_bins.unwrap_or(5), i.strategy.as_deref().unwrap_or("quantile"));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "PowerTransformer" => {
            let mut t = PowerTransformer::new(i.method.as_deref().unwrap_or("yeo-johnson"));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, format!(",\"lambdas\":{}", serde_json::to_string(&t.lambdas).unwrap_or_default()))
        }
        "QuantileTransformer" => {
            let mut t = QuantileTransformer::new(i.n_quantiles.unwrap_or(1000), i.output_distribution.as_deref().unwrap_or("uniform"));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "StandardScaler" => {
            let mut t = StandardScaler::new(true, true);
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "MinMaxScaler" => {
            let mut t = MinMaxScaler::new((0.0, 1.0));
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "RobustScaler" => {
            let mut t = RobustScaler::new(true, true);
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        "MaxAbsScaler" => {
            let mut t = MaxAbsScaler::new();
            t.fit(&flat, n, p);
            let o = t.transform(&flat, n, p);
            (o, p, String::new())
        }
        n_ => return format!("{{\"error\":\"unknown transformer: {}\"}}", n_),
    };
    let mut data: Vec<Vec<f64>> = Vec::with_capacity(n);
    for r in 0..n {
        data.push(out[r * cols..(r + 1) * cols].to_vec());
    }
    format!("{{\"data\":{},\"n\":{},\"cols\":{}{}}}",
        serde_json::to_string(&data).unwrap_or_default(), n, cols, extra)
}

#[crate::sera_alias("kfold_split", "ml_kfold", "kfold", "cv_split")]
pub fn ml_kfold_split(input: &str) -> String {
    use crate::ml::model_selection::split::*;
    #[derive(Deserialize, Default)]
    struct I {
        kind: Option<String>,
        n: Option<usize>,
        k: Option<usize>,
        seed: Option<u64>,
        y: Option<Vec<i32>>,
        groups: Option<Vec<i32>>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let kind = i.kind.unwrap_or_else(|| "kfold".to_string());
    let k = i.k.unwrap_or(5);
    let seed = i.seed.unwrap_or(0);
    let folds = match kind.as_str() {
        "stratified" => stratified_kfold_indices(&i.y.unwrap_or_default(), k, seed),
        "group" => group_kfold_indices(&i.groups.unwrap_or_default(), k),
        _ => kfold_indices(i.n.unwrap_or(0), k, seed),
    };
    let payload: Vec<serde_json::Value> = folds.into_iter().map(|(tr, te)| {
        serde_json::json!({"train": tr, "test": te})
    }).collect();
    serde_json::to_string(&payload).unwrap_or_else(|_| "[]".to_string())
}

#[crate::sera_alias("isolation_forest", "ml_iforest", "iforest", "anomaly_isolation_forest")]
pub fn ml_isolation_forest(input: &str) -> String {
    use crate::ml::anomaly::isolation_forest::IsolationForest;
    #[derive(Deserialize, Default)]
    struct I {
        data: Option<Vec<Vec<f64>>>,
        x_test: Option<Vec<Vec<f64>>>,
        n_estimators: Option<usize>,
        max_samples: Option<usize>,
        contamination: Option<f64>,
        seed: Option<u64>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let rows = i.data.unwrap_or_default();
    let n = rows.len();
    if n == 0 { return "{\"labels\":[],\"scores\":[],\"threshold\":0.0}".to_string(); }
    let p = rows[0].len();
    let mut flat = Vec::with_capacity(n * p);
    for r in &rows { flat.extend_from_slice(&r[..p.min(r.len())]); if r.len() < p { flat.extend(std::iter::repeat(0.0).take(p - r.len())); } }
    let mut model = IsolationForest::new(
        i.n_estimators.unwrap_or(100),
        i.max_samples.unwrap_or(256),
        i.contamination.unwrap_or(0.1),
        i.seed.unwrap_or(42),
    );
    model.fit(&flat, n, p);
    let labels = model.predict(&flat, n, p);
    let scores = model.score_samples(&flat, n, p);
    let test_payload = if let Some(rows_t) = i.x_test {
        let nt = rows_t.len();
        if nt == 0 {
            String::new()
        } else {
            let mut flat_t = Vec::with_capacity(nt * p);
            for r in &rows_t { flat_t.extend_from_slice(&r[..p.min(r.len())]); if r.len() < p { flat_t.extend(std::iter::repeat(0.0).take(p - r.len())); } }
            let lt = model.predict(&flat_t, nt, p);
            let st = model.score_samples(&flat_t, nt, p);
            format!(",\"test_labels\":{},\"test_scores\":{}",
                serde_json::to_string(&lt).unwrap_or_default(),
                serde_json::to_string(&st).unwrap_or_default())
        }
    } else { String::new() };
    format!("{{\"labels\":{},\"scores\":{},\"threshold\":{}{}}}",
        serde_json::to_string(&labels).unwrap_or_default(),
        serde_json::to_string(&scores).unwrap_or_default(),
        model.threshold_,
        test_payload)
}

#[crate::sera_alias("permutation_importance", "ml_perm_importance", "perm_importance", "feature_importance")]
pub fn ml_permutation_importance(input: &str) -> String {
    use crate::ml::model_selection::permutation::*;
    #[derive(Deserialize, Default)]
    struct I {
        data: Option<Vec<Vec<f64>>>,
        y: Option<Vec<f64>>,
        baseline_pred: Option<Vec<f64>>,
        perm_preds: Option<Vec<Vec<Vec<f64>>>>,
        task: Option<String>,
        n_repeats: Option<usize>,
        seed: Option<u64>,
    }
    let i: I = serde_json::from_str(input).unwrap_or_default();
    let rows = i.data.unwrap_or_default();
    let n = rows.len();
    if n == 0 { return "{\"importances_mean\":[],\"importances_std\":[]}".to_string(); }
    let p = rows[0].len();
    let mut flat = Vec::with_capacity(n * p);
    for r in &rows { flat.extend_from_slice(&r[..p.min(r.len())]); if r.len() < p { flat.extend(std::iter::repeat(0.0).take(p - r.len())); } }
    let yf = i.y.clone().unwrap_or_default();
    let task = i.task.unwrap_or_else(|| "regression".to_string());
    let n_repeats = i.n_repeats.unwrap_or(5);
    let seed = i.seed.unwrap_or(0);
    let perm_preds = i.perm_preds.unwrap_or_default();
    let baseline_pred = i.baseline_pred.unwrap_or_default();
    let _ = (n_repeats, seed, flat, n);
    if perm_preds.is_empty() || baseline_pred.is_empty() {
        return "{\"importances_mean\":[],\"importances_std\":[],\"error\":\"baseline_pred and perm_preds[p][n_repeats][n] are required\"}".to_string();
    }
    if task == "classification" {
        let yi: Vec<i32> = yf.iter().map(|v| *v as i32).collect();
        let baseline_i: Vec<i32> = baseline_pred.iter().map(|v| *v as i32).collect();
        let baseline = crate::ml::metrics::classification::accuracy_score(&yi, &baseline_i);
        let mut means = vec![0.0; p];
        let mut stds = vec![0.0; p];
        for j in 0..p.min(perm_preds.len()) {
            let reps = &perm_preds[j];
            let mut diffs = Vec::with_capacity(reps.len());
            for rep in reps {
                let pi: Vec<i32> = rep.iter().map(|v| *v as i32).collect();
                let s = crate::ml::metrics::classification::accuracy_score(&yi, &pi);
                diffs.push(baseline - s);
            }
            let m = diffs.iter().sum::<f64>() / diffs.len().max(1) as f64;
            let v = diffs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / diffs.len().max(1) as f64;
            means[j] = m;
            stds[j] = v.sqrt();
        }
        format!("{{\"importances_mean\":{},\"importances_std\":{},\"baseline\":{}}}",
            serde_json::to_string(&means).unwrap_or_default(),
            serde_json::to_string(&stds).unwrap_or_default(),
            baseline)
    } else {
        let baseline = crate::ml::metrics::regression::r2_score(&yf, &baseline_pred);
        let mut means = vec![0.0; p];
        let mut stds = vec![0.0; p];
        for j in 0..p.min(perm_preds.len()) {
            let reps = &perm_preds[j];
            let mut diffs = Vec::with_capacity(reps.len());
            for rep in reps {
                let s = crate::ml::metrics::regression::r2_score(&yf, rep);
                diffs.push(baseline - s);
            }
            let m = diffs.iter().sum::<f64>() / diffs.len().max(1) as f64;
            let v = diffs.iter().map(|x| (x - m).powi(2)).sum::<f64>() / diffs.len().max(1) as f64;
            means[j] = m;
            stds[j] = v.sqrt();
        }
        format!("{{\"importances_mean\":{},\"importances_std\":{},\"baseline\":{}}}",
            serde_json::to_string(&means).unwrap_or_default(),
            serde_json::to_string(&stds).unwrap_or_default(),
            baseline)
    }
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

pub fn export_svg(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { html: String }
    let v: In = match serde_json::from_str(input) { Ok(v) => v, Err(_) => return String::new() };
    let h = v.html;
    let start = match h.find("<svg") { Some(i) => i, None => return String::new() };
    let end = match h[start..].find("</svg>") { Some(i) => start + i + 6, None => return String::new() };
    h[start..end].to_string()
}

pub fn export_data_url(input: &str) -> String {
    use base64::Engine;
    let svg = export_svg(input);
    if svg.is_empty() { return String::new(); }
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    format!("data:image/svg+xml;base64,{b64}")
}

pub fn export_html_file(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { html: String, path: String }
    let v: In = match serde_json::from_str(input) { Ok(v) => v, Err(_) => return "{\"ok\":false}".to_string() };
    match std::fs::write(&v.path, v.html) {
        Ok(_) => format!("{{\"ok\":true,\"path\":{}}}", serde_json::to_string(&v.path).unwrap_or_else(|_| "\"\"".into())),
        Err(e) => format!("{{\"ok\":false,\"error\":{}}}", serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "\"\"".into())),
    }
}

pub fn chart_append(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { kind: String, x: Option<Vec<f64>>, y: Option<Vec<f64>>, title: Option<String>, max_points: Option<usize>, color_hex: Option<u32>, width: Option<i32>, height: Option<i32>, prev_x: Option<Vec<f64>>, prev_y: Option<Vec<f64>> }
    let v: In = match serde_json::from_str(input) { Ok(v) => v, Err(_) => return String::new() };
    let mut xs = v.prev_x.unwrap_or_default();
    let mut ys = v.prev_y.unwrap_or_default();
    if let Some(nx) = v.x { xs.extend(nx); }
    if let Some(ny) = v.y { ys.extend(ny); }
    if let Some(m) = v.max_points {
        if xs.len() > m { let cut = xs.len() - m; xs.drain(..cut); }
        if ys.len() > m { let cut = ys.len() - m; ys.drain(..cut); }
    }
    let title = v.title.unwrap_or_default();
    let color = v.color_hex.unwrap_or(0x6366F1);
    let w = v.width.unwrap_or(900);
    let h = v.height.unwrap_or(420);
    let payload = serde_json::json!({"title":title,"x":xs,"y":ys,"color_hex":color,"width":w,"height":h,"show_points":true});
    let html = match v.kind.as_str() {
        "scatter" => build_scatter_chart(&payload.to_string()),
        _ => build_line_chart(&serde_json::json!({"title":title,"labels":xs.iter().map(|v|v.to_string()).collect::<Vec<_>>(),"values":ys,"color_hex":color,"width":w,"height":h,"show_points":true}).to_string()),
    };
    serde_json::json!({"html":html,"x":xs,"y":ys}).to_string()
}

#[crate::sera_alias("save_model", "ml_save", "export_model")]
pub fn ml_save_model(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { kind: String, state: serde_json::Value, path: Option<String> }
    let v: In = match serde_json::from_str(input) { Ok(v) => v, Err(_) => return "{\"ok\":false}".to_string() };
    let envelope = serde_json::json!({"seraplot_model_v":1,"kind":v.kind,"state":v.state});
    let s = envelope.to_string();
    if let Some(p) = v.path {
        if let Err(e) = std::fs::write(&p, &s) {
            return format!("{{\"ok\":false,\"error\":{}}}", serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "\"\"".into()));
        }
        return format!("{{\"ok\":true,\"path\":{},\"size\":{}}}", serde_json::to_string(&p).unwrap_or_else(|_| "\"\"".into()), s.len());
    }
    serde_json::json!({"ok":true,"data":s}).to_string()
}

#[crate::sera_alias("load_model", "ml_load", "import_model")]
pub fn ml_load_model(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { path: Option<String>, data: Option<String> }
    let v: In = match serde_json::from_str(input) { Ok(v) => v, Err(_) => return "{\"ok\":false}".to_string() };
    let raw = if let Some(p) = v.path {
        match std::fs::read_to_string(&p) { Ok(s) => s, Err(e) => return format!("{{\"ok\":false,\"error\":{}}}", serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "\"\"".into())) }
    } else if let Some(d) = v.data { d } else { return "{\"ok\":false,\"error\":\"need path or data\"}".to_string() };
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(j) => serde_json::json!({"ok":true,"model":j}).to_string(),
        Err(e) => format!("{{\"ok\":false,\"error\":{}}}", serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "\"\"".into())),
    }
}

pub fn chart_info(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { html: String }
    let v: In = match serde_json::from_str(input) { Ok(v) => v, Err(_) => return "{}".to_string() };
    let h = v.html;
    let len = h.len();
    let n_paths = h.matches("<path").count();
    let n_rects = h.matches("<rect").count();
    let n_circles = h.matches("<circle").count();
    let has_svg = h.contains("<svg");
    serde_json::json!({"size":len,"paths":n_paths,"rects":n_rects,"circles":n_circles,"has_svg":has_svg}).to_string()
}

pub fn validate_input(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { labels: Option<Vec<String>>, values: Option<Vec<f64>>, x: Option<Vec<f64>>, y: Option<Vec<f64>>, series: Option<Vec<Vec<f64>>> }
    let v: In = match serde_json::from_str::<In>(input) {
        Ok(v) => v,
        Err(e) => return serde_json::json!({"ok":false,"error":format!("invalid JSON: {e}")}).to_string(),
    };
    if let (Some(l), Some(va)) = (v.labels.as_ref(), v.values.as_ref()) {
        if l.len() != va.len() {
            return serde_json::json!({"ok":false,"error":format!("labels ({}) and values ({}) length mismatch", l.len(), va.len())}).to_string();
        }
    }
    if let (Some(x), Some(y)) = (v.x.as_ref(), v.y.as_ref()) {
        if x.len() != y.len() {
            return serde_json::json!({"ok":false,"error":format!("x ({}) and y ({}) length mismatch", x.len(), y.len())}).to_string();
        }
    }
    if let (Some(l), Some(s)) = (v.labels.as_ref(), v.series.as_ref()) {
        for (i, row) in s.iter().enumerate() {
            if row.len() != l.len() {
                return serde_json::json!({"ok":false,"error":format!("series[{i}] length {} != labels length {}", row.len(), l.len())}).to_string();
            }
        }
    }
    serde_json::json!({"ok":true}).to_string()
}

pub fn downsample_lttb(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { x: Vec<f64>, y: Vec<f64>, threshold: usize }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => return serde_json::json!({"ok":false,"error":format!("invalid JSON: {e}")}).to_string(),
    };
    let n = v.x.len();
    if n != v.y.len() {
        return serde_json::json!({"ok":false,"error":"x and y length mismatch"}).to_string();
    }
    let th = v.threshold;
    if th >= n || th < 3 {
        return serde_json::json!({"ok":true,"x":v.x,"y":v.y}).to_string();
    }
    let bucket_size = (n - 2) as f64 / (th - 2) as f64;
    let mut out_x: Vec<f64> = Vec::with_capacity(th);
    let mut out_y: Vec<f64> = Vec::with_capacity(th);
    out_x.push(v.x[0]); out_y.push(v.y[0]);
    let mut a: usize = 0;
    for i in 0..(th - 2) {
        let avg_start = ((i + 1) as f64 * bucket_size).floor() as usize + 1;
        let avg_end = (((i + 2) as f64 * bucket_size).floor() as usize + 1).min(n);
        let avg_len = (avg_end - avg_start).max(1) as f64;
        let mut avg_x = 0.0; let mut avg_y = 0.0;
        for k in avg_start..avg_end { avg_x += v.x[k]; avg_y += v.y[k]; }
        avg_x /= avg_len; avg_y /= avg_len;
        let range_offs = (i as f64 * bucket_size).floor() as usize + 1;
        let range_to = ((i + 1) as f64 * bucket_size).floor() as usize + 1;
        let pax = v.x[a]; let pay = v.y[a];
        let mut max_area = -1.0f64;
        let mut next_a = range_offs;
        for k in range_offs..range_to.min(n) {
            let area = ((pax - avg_x) * (v.y[k] - pay) - (pax - v.x[k]) * (avg_y - pay)).abs() * 0.5;
            if area > max_area { max_area = area; next_a = k; }
        }
        out_x.push(v.x[next_a]); out_y.push(v.y[next_a]);
        a = next_a;
    }
    out_x.push(v.x[n - 1]); out_y.push(v.y[n - 1]);
    serde_json::json!({"ok":true,"x":out_x,"y":out_y,"reduction":format!("{}->{}", n, out_x.len())}).to_string()
}

pub fn chart_diff(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { a: String, b: String }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return serde_json::json!({"ok":false,"error":"invalid JSON"}).to_string(),
    };
    let svg_a = extract_svg(&v.a);
    let svg_b = extract_svg(&v.b);
    let identical = svg_a == svg_b;
    let len_a = svg_a.len();
    let len_b = svg_b.len();
    let common = svg_a.bytes().zip(svg_b.bytes()).take_while(|(x, y)| x == y).count();
    let similarity = if len_a == 0 && len_b == 0 { 1.0 } else {
        let max_len = len_a.max(len_b) as f64;
        common as f64 / max_len
    };
    serde_json::json!({
        "ok":true,
        "identical":identical,
        "size_a":len_a,
        "size_b":len_b,
        "common_prefix":common,
        "similarity":similarity
    }).to_string()
}

fn extract_svg(html: &str) -> String {
    let start = match html.find("<svg") { Some(i) => i, None => return String::new() };
    let end = match html[start..].find("</svg>") { Some(i) => start + i + 6, None => return String::new() };
    html[start..end].to_string()
}

pub fn drift_ks(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In { reference: Vec<f64>, current: Vec<f64> }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => return serde_json::json!({"ok":false,"error":format!("invalid JSON: {e}")}).to_string(),
    };
    let mut a = v.reference.clone();
    let mut b = v.current.clone();
    if a.is_empty() || b.is_empty() {
        return serde_json::json!({"ok":false,"error":"empty array"}).to_string();
    }
    a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let n = a.len() as f64;
    let m = b.len() as f64;
    let mut i = 0usize; let mut j = 0usize;
    let mut max_d = 0.0f64;
    while i < a.len() && j < b.len() {
        let cdf_a = (i + 1) as f64 / n;
        let cdf_b = (j + 1) as f64 / m;
        let d = (cdf_a - cdf_b).abs();
        if d > max_d { max_d = d; }
        if a[i] <= b[j] { i += 1; } else { j += 1; }
    }
    let coeff = ((n * m) / (n + m)).sqrt();
    let lambda = (coeff + 0.12 + 0.11 / coeff) * max_d;
    let mut p_value = 0.0f64;
    let mut sign = 1.0f64;
    for k in 1..=100 {
        let term = sign * 2.0 * (-2.0 * lambda * lambda * (k * k) as f64).exp();
        p_value += term;
        sign = -sign;
        if term.abs() < 1e-10 { break; }
    }
    p_value = p_value.clamp(0.0, 1.0);
    let drift_detected = p_value < 0.05;
    serde_json::json!({
        "ok":true,
        "ks_statistic":max_d,
        "p_value":p_value,
        "drift_detected":drift_detected,
        "n_reference":a.len(),
        "n_current":b.len()
    }).to_string()
}

pub fn bench_chart_value(s: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(s).is_ok()
}

#[cfg(any(feature = "python", feature = "gui"))]
pub fn set_chart_kind(kind: u8) {
    crate::viewer::chart::sera_set_current_chart_kind(kind);
}
#[cfg(not(any(feature = "python", feature = "gui")))]
pub fn set_chart_kind(_kind: u8) {}

#[cfg(any(feature = "python", feature = "gui"))]
pub fn set_chart_orientation(vertical: bool) {
    crate::viewer::chart::sera_set_chart_orientation(vertical);
}
#[cfg(not(any(feature = "python", feature = "gui")))]
pub fn set_chart_orientation(_vertical: bool) {}

#[cfg(any(feature = "python", feature = "gui"))]
pub fn show_chart_value(s: &str) -> bool {
    let c = std::ffi::CString::new(s).unwrap_or_default();
    crate::viewer::chart::sera_show_chart_value(c.as_ptr())
}
#[cfg(not(any(feature = "python", feature = "gui")))]
pub fn show_chart_value(_s: &str) -> bool { false }

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
        let _ = render_histogram_html(&HistogramConfig { title: "B", values: &ages, bins: 20, width: 900, height: 400, ..HistogramConfig::default() });
    }
    let hist_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_bars_html("B", &labs, &vals, 900, 480, &[], b'v', &[], false, "", "", &[], 0, true, "");
    }
    let bar_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_scatter_html("B", &ages100, &fare100, &[], 900, 540, &[], &[], &[], &[], "", "", 0, true, false, false, "linear");
    }
    let scatter_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_heatmap_html(&HeatmapConfig { title: "B", row_labels: &corr_labels, col_labels: &[], flat_matrix: &flat, width: 800, ..HeatmapConfig::default() });
    }
    let heatmap_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    (hist_ms, bar_ms, scatter_ms, heatmap_ms)
}

pub struct DbscanState {
    pub eps: f64,
    pub min_samples: usize,
    pub labels: Vec<i32>,
    pub n_clusters: usize,
    pub n_noise: usize,
}

impl DbscanState {
    pub fn new(eps: f64, min_samples: usize) -> Self {
        Self { eps, min_samples, labels: Vec::new(), n_clusters: 0, n_noise: 0 }
    }
    pub fn fit(&mut self, x: &[Vec<f64>]) {
        let (labels, n_clusters) = crate::plot::default::scatter::dbscan_core_nd(x, self.eps, self.min_samples);
        self.n_noise = labels.iter().filter(|&&l| l < 0).count();
        self.labels = labels;
        self.n_clusters = n_clusters;
    }
    pub fn fit_flat(&mut self, x: &[f64], n: usize, p: usize) {
        let (labels, n_clusters) = crate::plot::default::scatter::dbscan_core_nd_flat(x, n, p, self.eps, self.min_samples);
        self.n_noise = labels.iter().filter(|&&l| l < 0).count();
        self.labels = labels;
        self.n_clusters = n_clusters;
    }
}

pub struct KMeansState {
    pub k: usize,
    pub max_iter: usize,
    pub tol: f64,
    pub mini_batch: bool,
    pub batch_size: usize,
    pub n_init: usize,
    pub labels: Vec<i32>,
    pub centroids: Vec<Vec<f64>>,
    pub inertia: f64,
    pub n_iter: usize,
}

impl KMeansState {
    pub fn new(k: usize, max_iter: usize, tol: f64, mini_batch: bool, batch_size: usize, n_init: usize) -> Self {
        Self { k, max_iter, tol, mini_batch, batch_size, n_init, labels: Vec::new(), centroids: Vec::new(), inertia: 0.0, n_iter: 0 }
    }
    pub fn fit_flat(&mut self, flat: &[f64], n: usize, dims: usize) {
        if self.mini_batch {
            let (labels, flat_c, inertia) = crate::plot::default::minibatch_kmeans_core_flat(flat, n, dims, self.k, self.max_iter, self.batch_size);
            self.labels = labels;
            self.centroids = (0..self.k.min(n)).map(|ki| flat_c[ki*dims..(ki+1)*dims].to_vec()).collect();
            self.inertia = inertia;
        } else {
            let (labels, flat_c, inertia) = crate::plot::default::kmeans_core_flat_ninit(flat, n, dims, self.k, self.max_iter, self.tol, self.n_init);
            self.labels = labels;
            self.centroids = (0..self.k.min(n)).map(|ki| flat_c[ki*dims..(ki+1)*dims].to_vec()).collect();
            self.inertia = inertia;
        }
        self.n_iter = self.max_iter;
    }
    pub fn predict_flat(&self, flat: &[f64], n: usize, dims: usize) -> Vec<i32> {
        use rayon::prelude::*;
        let kk = self.centroids.len();
        if n == 0 || dims == 0 || kk == 0 { return vec![0i32; n]; }
        let mut c_flat: Vec<f64> = Vec::with_capacity(kk * dims);
        for c in &self.centroids {
            if c.len() == dims { c_flat.extend_from_slice(c); } else { c_flat.extend(std::iter::repeat(0.0).take(dims)); }
        }
        let mut c_norm = vec![0.0f64; kk];
        for ki in 0..kk {
            let row = &c_flat[ki*dims..(ki+1)*dims];
            let mut s = 0.0;
            for &v in row { s += v*v; }
            c_norm[ki] = s;
        }
        let mut d = vec![0.0f64; n * kk];
        if n * dims * kk < 4096 {
            d.par_chunks_mut(kk).enumerate().for_each(|(i, drow)| {
                let xrow = &flat[i*dims..(i+1)*dims];
                for ki in 0..kk {
                    let crow = &c_flat[ki*dims..(ki+1)*dims];
                    let mut s = 0.0;
                    for j in 0..dims { s += xrow[j] * crow[j]; }
                    drow[ki] = s;
                }
            });
        } else {
            unsafe {
                matrixmultiply::dgemm(
                    n, dims, kk,
                    1.0,
                    flat.as_ptr(), dims as isize, 1,
                    c_flat.as_ptr(), 1, dims as isize,
                    0.0,
                    d.as_mut_ptr(), kk as isize, 1,
                );
            }
        }
        let mut labels = vec![0i32; n];
        labels.par_iter_mut().enumerate().for_each(|(i, lab)| {
            let row = &d[i*kk..(i+1)*kk];
            let mut best = 0usize;
            let mut bestv = c_norm[0] - 2.0 * row[0];
            for ki in 1..kk {
                let v = c_norm[ki] - 2.0 * row[ki];
                if v < bestv { bestv = v; best = ki; }
            }
            *lab = best as i32;
        });
        labels
    }
    pub fn transform_flat(&self, flat: &[f64], n: usize, dims: usize) -> Vec<Vec<f64>> {
        use rayon::prelude::*;
        let kk = self.centroids.len();
        if n == 0 || dims == 0 || kk == 0 { return vec![vec![0.0; kk]; n]; }
        let mut c_flat: Vec<f64> = Vec::with_capacity(kk * dims);
        for c in &self.centroids {
            if c.len() == dims { c_flat.extend_from_slice(c); } else { c_flat.extend(std::iter::repeat(0.0).take(dims)); }
        }
        let mut c_norm = vec![0.0f64; kk];
        for ki in 0..kk {
            let row = &c_flat[ki*dims..(ki+1)*dims];
            let mut s = 0.0;
            for &v in row { s += v*v; }
            c_norm[ki] = s;
        }
        let mut x_norm = vec![0.0f64; n];
        x_norm.par_iter_mut().enumerate().for_each(|(i, s)| {
            let row = &flat[i*dims..(i+1)*dims];
            let mut acc = 0.0;
            for &v in row { acc += v*v; }
            *s = acc;
        });
        let mut d = vec![0.0f64; n * kk];
        if n * dims * kk < 4096 {
            d.par_chunks_mut(kk).enumerate().for_each(|(i, drow)| {
                let xrow = &flat[i*dims..(i+1)*dims];
                for ki in 0..kk {
                    let crow = &c_flat[ki*dims..(ki+1)*dims];
                    let mut s = 0.0;
                    for j in 0..dims { s += xrow[j] * crow[j]; }
                    drow[ki] = s;
                }
            });
        } else {
            unsafe {
                matrixmultiply::dgemm(
                    n, dims, kk,
                    1.0,
                    flat.as_ptr(), dims as isize, 1,
                    c_flat.as_ptr(), 1, dims as isize,
                    0.0,
                    d.as_mut_ptr(), kk as isize, 1,
                );
            }
        }
        (0..n).into_par_iter().map(|i| {
            let row = &d[i*kk..(i+1)*kk];
            let xn = x_norm[i];
            (0..kk).map(|ki| {
                let v = xn - 2.0 * row[ki] + c_norm[ki];
                if v <= 0.0 { 0.0 } else { v.sqrt() }
            }).collect()
        }).collect()
    }
}

fn parse_x2d(v: &serde_json::Value, key: &str, alias: &str) -> Vec<Vec<f64>> {
    let raw = v.get(key).or_else(|| v.get(alias));
    if let Some(x) = raw {
        if let Ok(mat) = serde_json::from_value::<Vec<Vec<f64>>>(x.clone()) { return mat; }
        if let Ok(flat) = serde_json::from_value::<Vec<f64>>(x.clone()) { return flat.into_iter().map(|v| vec![v]).collect(); }
    }
    vec![]
}

fn ml_parse(input: &str) -> (serde_json::Value, Vec<f64>, usize, usize, Vec<f64>, usize) {
    let v: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
    let xt = parse_x2d(&v, "x_train", "x");
    let xtest = parse_x2d(&v, "x_test", "test_x");
    let n = xt.len();
    let p = xt.first().map_or(0, |r| r.len());
    let xf: Vec<f64> = xt.into_iter().flatten().collect();
    let nt = xtest.len();
    let xtf: Vec<f64> = xtest.into_iter().flatten().collect();
    (v, xf, n, p, xtf, nt)
}

fn jf(v: &serde_json::Value, k: &str, d: f64) -> f64 { v.get(k).and_then(|x| x.as_f64()).unwrap_or(d) }
fn ju(v: &serde_json::Value, k: &str, d: usize) -> usize { v.get(k).and_then(|x| x.as_u64()).map(|x| x as usize).unwrap_or(d) }
fn jb(v: &serde_json::Value, k: &str, d: bool) -> bool { v.get(k).and_then(|x| x.as_bool()).unwrap_or(d) }
fn js<'a>(v: &'a serde_json::Value, k: &str, d: &'a str) -> &'a str { v.get(k).and_then(|x| x.as_str()).unwrap_or(d) }
fn yf(v: &serde_json::Value) -> Vec<f64> { v.get("y_train").or_else(|| v.get("y")).and_then(|x| serde_json::from_value(x.clone()).ok()).unwrap_or_default() }
fn yi(v: &serde_json::Value) -> Vec<i32> { v.get("y_train").or_else(|| v.get("y")).and_then(|x| serde_json::from_value(x.clone()).ok()).unwrap_or_default() }

fn parse_max_features(v: &serde_json::Value) -> crate::ml::tree::random_forest::MaxFeatures {
    use crate::ml::tree::random_forest::MaxFeatures as MF;
    match v.get("max_features") {
        Some(serde_json::Value::String(s)) => match s.as_str() {
            "log2" => MF::Log2,
            "all"  => MF::All,
            "none" => MF::All,
            _      => MF::Sqrt,
        },
        Some(serde_json::Value::Number(n)) => MF::Fixed(n.as_u64().unwrap_or(0) as usize),
        _ => MF::Sqrt,
    }
}

fn parse_knn_weights(s: &str) -> crate::ml::neighbors::knn::KnnWeights {
    use crate::ml::neighbors::knn::KnnWeights as W;
    match s { "distance" => W::Distance, _ => W::Uniform }
}

fn parse_tree_criterion(s: &str) -> crate::ml::tree::decision_tree::TreeCriterion {
    use crate::ml::tree::decision_tree::TreeCriterion as C;
    match s { "entropy" => C::Entropy, "mse" => C::MSE, _ => C::Gini }
}

#[crate::sera_alias("linear_regression", "ml_linreg", "linreg", "LinearRegression", "ols")]
pub fn ml_linear_regression(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let mut m = crate::ml::linear::ols::LinearRegression::new(jb(&v, "fit_intercept", true));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "coef": m.coef, "intercept": m.intercept}).to_string()
}

#[crate::sera_alias("ridge", "ridge_regression", "Ridge")]
pub fn ml_ridge(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let mut m = crate::ml::linear::ridge::Ridge::new(jf(&v, "alpha", 1.0), jb(&v, "fit_intercept", true));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "coef": m.coef, "intercept": m.intercept}).to_string()
}

#[crate::sera_alias("lasso", "lasso_regression", "Lasso")]
pub fn ml_lasso(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let mut m = crate::ml::linear::lasso::Lasso::new(jf(&v, "alpha", 1.0), ju(&v, "max_iter", 1000), jf(&v, "tol", 1e-4), jb(&v, "fit_intercept", true));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds}).to_string()
}

#[crate::sera_alias("elastic_net", "elasticnet", "ElasticNet")]
pub fn ml_elastic_net(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let mut m = crate::ml::linear::elastic_net::ElasticNet::new(jf(&v, "alpha", 1.0), jf(&v, "l1_ratio", 0.5), ju(&v, "max_iter", 1000), jf(&v, "tol", 1e-4), jb(&v, "fit_intercept", true));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "coef": m.coef, "intercept": m.intercept}).to_string()
}

#[crate::sera_alias("logistic_regression", "ml_logreg", "logreg", "LogisticRegression")]
pub fn ml_logistic_regression(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yi(&v);
    let mut m = crate::ml::linear::logistic::LogisticRegression::new(jf(&v, "c", 1.0), ju(&v, "max_iter", 100), jf(&v, "tol", 1e-4), jb(&v, "fit_intercept", true));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "classes": m.classes}).to_string()
}

#[crate::sera_alias("decision_tree_classifier", "dt_classifier", "DecisionTreeClassifier", "tree_classifier")]
pub fn ml_decision_tree_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yi(&v);
    let max_feat = v.get("max_features").and_then(|x| x.as_u64()).map(|x| x as usize);
    let crit = parse_tree_criterion(js(&v, "criterion", "gini"));
    let mut m = crate::ml::tree::decision_tree::DecisionTreeClassifier::new(ju(&v, "max_depth", 32), ju(&v, "min_samples_split", 2), ju(&v, "min_samples_leaf", 1), max_feat, crit);
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "classes": m.classes, "feature_importances": m.feature_importances}).to_string()
}

#[crate::sera_alias("decision_tree_regressor", "dt_regressor", "DecisionTreeRegressor", "tree_regressor")]
pub fn ml_decision_tree_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let max_feat = v.get("max_features").and_then(|x| x.as_u64()).map(|x| x as usize);
    let mut m = crate::ml::tree::decision_tree::DecisionTreeRegressor::new(ju(&v, "max_depth", 32), ju(&v, "min_samples_split", 2), ju(&v, "min_samples_leaf", 1), max_feat);
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "feature_importances": m.feature_importances}).to_string()
}

#[crate::sera_alias("random_forest_classifier", "rf_classifier", "RandomForestClassifier", "forest_classifier")]
pub fn ml_random_forest_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yi(&v);
    let mf = parse_max_features(&v);
    let mut m = crate::ml::tree::random_forest::RandomForestClassifier::new(ju(&v, "n_estimators", 100), ju(&v, "max_depth", 32), ju(&v, "min_samples_split", 2), ju(&v, "min_samples_leaf", 1), mf);
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "classes": m.classes, "feature_importances": m.feature_importances}).to_string()
}

#[crate::sera_alias("random_forest_regressor", "rf_regressor", "RandomForestRegressor", "forest_regressor")]
pub fn ml_random_forest_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let mf = parse_max_features(&v);
    let mut m = crate::ml::tree::random_forest::RandomForestRegressor::new(ju(&v, "n_estimators", 100), ju(&v, "max_depth", 32), ju(&v, "min_samples_split", 2), ju(&v, "min_samples_leaf", 1), mf);
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "feature_importances": m.feature_importances}).to_string()
}

#[crate::sera_alias("gradient_boosting_classifier", "gb_classifier", "GradientBoostingClassifier", "gbm_classifier")]
pub fn ml_gradient_boosting_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yi(&v);
    let mut m = crate::ml::tree::gradient_boosting::GradientBoostingClassifier::new(ju(&v, "n_estimators", 100), jf(&v, "learning_rate", 0.1), ju(&v, "max_depth", 3), ju(&v, "min_samples_split", 2), ju(&v, "min_samples_leaf", 1));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "classes": m.classes}).to_string()
}

#[crate::sera_alias("gradient_boosting_regressor", "gb_regressor", "GradientBoostingRegressor", "gbm_regressor")]
pub fn ml_gradient_boosting_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let mut m = crate::ml::tree::gradient_boosting::GradientBoostingRegressor::new(ju(&v, "n_estimators", 100), jf(&v, "learning_rate", 0.1), ju(&v, "max_depth", 3), ju(&v, "min_samples_split", 2), ju(&v, "min_samples_leaf", 1));
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds}).to_string()
}

#[crate::sera_alias("knn_classifier", "knn_clf", "KNeighborsClassifier", "kneighbors_classifier")]
pub fn ml_knn_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yi(&v);
    let w = parse_knn_weights(js(&v, "weights", "uniform"));
    let mut m = crate::ml::neighbors::knn::KNeighborsClassifier::new(ju(&v, "k", 5), w);
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds, "classes": m.classes}).to_string()
}

#[crate::sera_alias("knn_regressor", "knn_reg", "KNeighborsRegressor", "kneighbors_regressor")]
pub fn ml_knn_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let yt = yf(&v);
    let w = parse_knn_weights(js(&v, "weights", "uniform"));
    let mut m = crate::ml::neighbors::knn::KNeighborsRegressor::new(ju(&v, "k", 5), w);
    m.fit(&xf, n, p, &yt);
    let preds = m.predict(&xtf, nt, p);
    serde_json::json!({"predictions": preds}).to_string()
}

#[crate::sera_alias("plan", "cloud_plan")]
pub fn scale_plan(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In { n_rows: Option<usize>, n_cols: Option<usize>, mem_budget_mb: Option<u64> }
    let v: In = serde_json::from_str(input).unwrap_or_default();
    let p = crate::cloud::planner::plan(
        v.n_rows.unwrap_or(0),
        v.n_cols.unwrap_or(0),
        v.mem_budget_mb.unwrap_or(512),
    );
    crate::cloud::planner::to_json(&p)
}

#[crate::sera_alias("profile", "cloud_profile", "system_info")]
pub fn system_profile(_input: &str) -> String {
    let r = crate::cloud::profile::current();
    crate::cloud::profile::to_json(&r)
}

#[crate::sera_alias("count_rows", "csv_rows")]
pub fn csv_count_rows(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In { path: String, has_header: Option<bool> }
    let v: In = serde_json::from_str(input).unwrap_or_default();
    if v.path.is_empty() { return "{\"error\":\"path required\"}".to_string(); }
    match crate::cloud::chunker::count_rows(&v.path, v.has_header.unwrap_or(true)) {
        Ok(n) => serde_json::json!({"rows": n}).to_string(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

#[crate::sera_alias("read_chunk", "csv_chunk")]
pub fn csv_chunk_read(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In { path: String, offset_rows: Option<usize>, chunk_rows: Option<usize>, has_header: Option<bool>, delimiter: Option<u8> }
    let v: In = serde_json::from_str(input).unwrap_or_default();
    if v.path.is_empty() { return "{\"error\":\"path required\"}".to_string(); }
    let delim = v.delimiter.unwrap_or(b',');
    let chunk = v.chunk_rows.unwrap_or(1000).max(1);
    let offset = v.offset_rows.unwrap_or(0);
    let has_header = v.has_header.unwrap_or(true);
    let mut reader = match crate::cloud::chunker::CsvChunkReader::open(&v.path, chunk + offset, has_header, delim) {
        Ok(r) => r,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };
    let header = reader.header.clone();
    let all = match reader.next_chunk() {
        Ok(Some(rows)) => rows,
        Ok(None) => vec![],
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };
    let n = all.len();
    let data: Vec<Vec<f64>> = if offset < n { all.into_iter().skip(offset).collect() } else { vec![] };
    let rows = data.len();
    serde_json::json!({"header": header, "data": data, "rows": rows}).to_string()
}

