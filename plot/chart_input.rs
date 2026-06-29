use serde::Deserialize;

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
    pub scene: Option<String>,
    pub orientation3d: Option<String>,
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
    pub ring_gap: Option<f64>,
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
    pub despine: Option<bool>,
    pub watermark_text: Option<String>,
    pub watermark_opacity: Option<f64>,
    pub caption: Option<String>,
    pub glow_color: Option<String>,
    pub highlight_at: Option<usize>,
    pub highlight_color: Option<String>,
    pub hline_value: Option<f64>,
    pub hline_color: Option<String>,
    pub hline_label: Option<String>,
    pub subtitle: Option<String>,
    pub shadow_blur: Option<i32>,
    pub shadow_color: Option<String>,
    pub pulse_duration: Option<f64>,
    pub pulse_index: Option<Vec<usize>>,
    pub pulse_above: Option<f64>,
    pub outline_color: Option<String>,
    pub outline_width: Option<f64>,
    pub value_labels: Option<bool>,
    pub value_labels_decimals: Option<i32>,
    pub value_labels_color: Option<String>,
    pub error_bars_margin: Option<f64>,
    pub error_bars_color: Option<String>,
    pub delta_labels: Option<bool>,
    pub delta_labels_pos_color: Option<String>,
    pub delta_labels_neg_color: Option<String>,
    pub cumulative_line_color: Option<String>,
    pub rank_badges_top_n: Option<usize>,
    pub rank_badges_color: Option<String>,
    pub title_color: Option<String>,
    pub export_button: Option<bool>,
    pub log_scale: Option<bool>,
    pub moving_average_window: Option<usize>,
    pub moving_average_color: Option<String>,
    pub outliers_threshold: Option<f64>,
    pub outliers_color: Option<String>,
    pub fill_between_color: Option<String>,
    pub fill_between_opacity: Option<f64>,
    pub box_annotate_color: Option<String>,
    pub pct_of_total: Option<bool>,
    pub pct_of_total_decimals: Option<i32>,
    pub pct_of_total_color: Option<String>,
    pub correlation_badge_color: Option<String>,
    pub highlight_range_low: Option<usize>,
    pub highlight_range_high: Option<usize>,
    pub highlight_range_color: Option<String>,
    pub highlight_range_opacity: Option<f64>,
    pub iqr_band_color: Option<String>,
    pub iqr_band_opacity: Option<f64>,
    pub growth_badge_color: Option<String>,
    pub zscore_heat: Option<bool>,
    pub pareto_marker_threshold: Option<f64>,
    pub pareto_marker_color: Option<String>,
    pub diff_from_mean: Option<bool>,
    pub diff_from_mean_pos_color: Option<String>,
    pub diff_from_mean_neg_color: Option<String>,
    pub scatter_regression_color: Option<String>,
    pub scatter_regression_width: Option<f64>,
    pub cluster_eps: Option<f64>,
    pub cluster_min_samples: Option<usize>,
    pub rolling_std_band_window: Option<usize>,
    pub rolling_std_band_color: Option<String>,
    pub rolling_std_band_opacity: Option<f64>,
    pub forecast_line_periods: Option<usize>,
    pub forecast_line_color: Option<String>,
    pub percentile_band_low: Option<f64>,
    pub percentile_band_high: Option<f64>,
    pub percentile_band_color: Option<String>,
    pub percentile_band_opacity: Option<f64>,
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
    if frac {
        format!("{:.2}%", v * 100.0)
    } else {
        format!("{}", v)
    }
}

fn ann_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
        let dash_attr = if dash.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{}\"", dash)
        };
        let opacity = a.opacity.unwrap_or(1.0);
        let op_attr = if (opacity - 1.0).abs() < 1e-6 {
            String::new()
        } else {
            format!(" opacity=\"{:.2}\"", opacity)
        };
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
                let m_attr = if a.kind == "arrow" {
                    format!(" marker-end=\"url(#{arrow_id})\"")
                } else {
                    String::new()
                };
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
    if buf.is_empty() {
        return html;
    }
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
    pub fn w(&self, default: i32) -> i32 {
        self.width.unwrap_or(default)
    }
    pub fn h(&self, default: i32) -> i32 {
        self.height.unwrap_or(default)
    }
    pub fn xl(&self) -> String {
        self.x_label.clone().unwrap_or_default()
    }
    pub fn yl(&self) -> String {
        self.y_label.clone().unwrap_or_default()
    }
    pub fn zl(&self) -> String {
        self.z_label.clone().unwrap_or_else(|| "Z".to_string())
    }
    pub fn grid(&self) -> bool {
        self.gridlines.unwrap_or(false) || crate::plot::get_global_grid()
    }
    pub fn srt(&self) -> String {
        self.sort_order
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }
    pub fn lp(&self) -> String {
        self.legend_position
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }
    pub fn scene3d(&self) -> String {
        crate::plot::scene3d::Scene3DVariant::from_str(self.scene.as_deref().unwrap_or("studio"))
            .name()
            .to_string()
    }
    pub fn pal(&self) -> Vec<u32> {
        if let Some(p) = &self.palette {
            if !p.is_empty() {
                return p.clone();
            }
        }
        let g = crate::plot::get_global_pal();
        if !g.is_empty() {
            g
        } else {
            Vec::new()
        }
    }
    pub fn hj(&self) -> Vec<crate::html::hover::HoverSlot> {
        self.hover_json
            .as_ref()
            .filter(|s| !s.is_empty())
            .map(|s| crate::plot::statistical::parse_hover_json(s))
            .unwrap_or_default()
    }
    pub fn bg_str(&self) -> Option<String> {
        self.background
            .clone()
            .or_else(|| self.bg_color.clone())
            .filter(|s| !s.is_empty())
    }

    pub fn no_x(&self) -> bool {
        self.no_x_axis.unwrap_or(false)
    }
    pub fn no_y(&self) -> bool {
        self.no_y_axis.unwrap_or(false)
    }
    pub fn no_background(&self) -> bool {
        self.background.is_none() && self.bg_color.is_none()
    }

    pub fn is_horiz(&self) -> bool {
        match self.orientation.as_deref() {
            Some(s) => {
                let l = s.to_ascii_lowercase();
                l == "h"
                    || l == "horiz"
                    || l == "horizontal"
                    || l == "rotated"
                    || l == "hbar"
                    || l == "hbox"
                    || l == "barh"
            }
            None => false,
        }
    }
    pub fn orient_byte(&self) -> u8 {
        if self.is_horiz() {
            b'h'
        } else {
            b'v'
        }
    }
    pub fn rotation_deg(&self) -> i32 {
        if let Some(opt) = self.orientation_option {
            return match opt {
                2 => 90,
                3 => 180,
                4 => 270,
                _ => 0,
            };
        }
        if self.is_horiz() {
            90
        } else {
            0
        }
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

pub fn parse_all(input: &str) -> (String, ChartArgs, ChartOpts) {
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
    let html = crate::apply_responsive(html);
    let bg_str = o.bg_str().or_else(crate::plot::get_global_bg);
    let bg = bg_str.as_deref();
    let h = crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y());
    let h = crate::html::hover::apply_rotation(h, o.rotation_deg());
    let h = apply_annotations(h, o);
    let h = apply_kwarg_chains(h, o);
    if let Some(ref t) = o.theme {
        crate::plot::statistical::apply_chart_theme(h, t)
    } else {
        h
    }
}

pub fn apply_h(html: String, o: &ChartOpts) -> String {
    let html = crate::apply_responsive(html);
    let bg_str = o.bg_str().or_else(crate::plot::get_global_bg);
    let bg = bg_str.as_deref();
    let h = crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y());
    let h = crate::html::hover::apply_rotation(h, o.rotation_deg_native());
    let h = apply_annotations(h, o);
    let h = apply_kwarg_chains(h, o);
    if let Some(ref t) = o.theme {
        crate::plot::statistical::apply_chart_theme(h, t)
    } else {
        h
    }
}

#[cfg(feature = "python")]
fn apply_kwarg_chains(html: String, o: &ChartOpts) -> String {
    use crate::{json_str, SP_LEGEND_JS};
    let is_3d = html.contains("class=\"c3w\"");
    let mut html = if is_3d {
        let mut cfg = String::new();
        if let Some(ref lp) = o.legend_position {
            cfg.push_str(&format!("\"legend\":true,\"legendPos\":{},", json_str(lp.as_str())));
        }
        if let Some(ref tc) = o.title_color {
            cfg.push_str(&format!("\"titleColor\":{},", json_str(tc.as_str())));
        }
        if o.export_button.unwrap_or(false) {
            cfg.push_str("\"exportBtn\":true,");
        }
        if cfg.is_empty() {
            html
        } else {
            cfg.pop();
            crate::apply_3d_cfg(html, &format!("{{{}}}", cfg))
        }
    } else {
        let mut snip = String::new();
        if let Some(ref lp) = o.legend_position {
            let pos = match lp.as_str() {
                "right" | "left" | "top" | "bottom" => lp.as_str(),
                _ => "right",
            };
            snip.push_str(&format!("window.__sp_legend_pos__={};", json_str(pos)));
            snip.push_str(SP_LEGEND_JS);
            snip.push(';');
        }
        let html = if snip.is_empty() {
            html
        } else {
            let block = format!("<script>{}</script></body>", snip);
            html.replacen("</body>", &block, 1)
        };
        if let Some(ref tc) = o.title_color {
            let css = format!("<style>.sp-ttl{{fill:{}!important}}</style></head>", tc);
            html.replacen("</head>", &css, 1)
        } else {
            html
        }
    };
    if let Some(ref mode) = o.orientation3d {
        html = crate::apply_orient3d(html, mode);
    }
    if let Some(ref pos) = o.colorbar_position {
        html = crate::apply_colorbar(html, pos);
    } else if is_3d {
        let scene = o.scene3d();
        if scene.as_str() == "terrain" {
            html = crate::apply_colorbar(html, "right");
        }
    }
    if o.despine.unwrap_or_else(crate::plot::utils::get_global_despine) {
        html = crate::apply_despine(html);
    }
    if let Some(ref t) = o.watermark_text {
        html = crate::apply_watermark(html, t, o.watermark_opacity.unwrap_or(0.08));
    } else if let Some((t, op)) = crate::plot::utils::get_global_watermark() {
        html = crate::apply_watermark(html, &t, op);
    }
    if let Some(ref t) = o.caption {
        html = crate::apply_caption(html, t);
    }
    if let Some(ref c) = o.glow_color {
        html = crate::apply_glow(html, c);
    }
    if let Some(idx) = o.highlight_at {
        html = crate::apply_highlight(html, idx, o.highlight_color.as_deref().unwrap_or("#f59e0b"));
    }
    if let Some(v) = o.hline_value {
        html = crate::apply_hline(
            html,
            v,
            o.hline_color.as_deref().unwrap_or("#ef4444"),
            o.hline_label.as_deref(),
        );
    }
    if let Some(ref t) = o.subtitle {
        html = crate::apply_subtitle(html, t);
    }
    if let Some(blur) = o.shadow_blur {
        html = crate::apply_shadow(html, blur, o.shadow_color.as_deref().unwrap_or("rgba(0,0,0,.35)"));
    } else if let Some((blur, color)) = crate::plot::utils::get_global_shadow() {
        html = crate::apply_shadow(html, blur, color.as_deref().unwrap_or("rgba(0,0,0,.35)"));
    }
    if let Some(dur) = o.pulse_duration {
        html = crate::apply_pulse(html, dur, o.pulse_index.as_deref(), o.pulse_above);
    }
    if let Some(ref c) = o.outline_color {
        html = crate::apply_outline(html, c, o.outline_width.unwrap_or(2.0));
    }
    if o.value_labels.unwrap_or(false) || o.value_labels_decimals.is_some() || o.value_labels_color.is_some() {
        html = crate::apply_value_labels(
            html,
            o.value_labels_decimals.unwrap_or(0),
            o.value_labels_color.as_deref().unwrap_or("#475569"),
        );
    }
    if let Some(margin) = o.error_bars_margin {
        html = crate::apply_error_bars(html, margin, o.error_bars_color.as_deref().unwrap_or("#475569"));
    }
    if o.delta_labels.unwrap_or(false) {
        html = crate::apply_delta_labels(
            html,
            o.delta_labels_pos_color.as_deref().unwrap_or("#22c55e"),
            o.delta_labels_neg_color.as_deref().unwrap_or("#ef4444"),
        );
    }
    if let Some(ref c) = o.cumulative_line_color {
        html = crate::apply_cumulative_line(html, c);
    }
    if let Some(n) = o.rank_badges_top_n {
        html = crate::apply_rank_badges(html, n, o.rank_badges_color.as_deref().unwrap_or("#6366f1"));
    }
    if o.log_scale.unwrap_or(false) {
        html = crate::apply_log_scale(html);
    }
    if let Some(w) = o.moving_average_window {
        html = crate::apply_moving_average(html, w, o.moving_average_color.as_deref().unwrap_or("#f59e0b"));
    }
    if let Some(t) = o.outliers_threshold {
        html = crate::apply_outliers(html, t, o.outliers_color.as_deref().unwrap_or("#ef4444"));
    }
    if let Some(ref c) = o.fill_between_color {
        html = crate::apply_fill_between(html, c, o.fill_between_opacity.unwrap_or(0.15));
    }
    if let Some(ref c) = o.box_annotate_color {
        html = crate::apply_box_annotate(html, c);
    }
    if o.pct_of_total.unwrap_or(false) || o.pct_of_total_decimals.is_some() || o.pct_of_total_color.is_some() {
        html = crate::apply_pct_of_total(
            html,
            o.pct_of_total_decimals.unwrap_or(1),
            o.pct_of_total_color.as_deref().unwrap_or("#475569"),
        );
    }
    if let Some(ref c) = o.correlation_badge_color {
        html = crate::apply_correlation_badge(html, c);
    }
    if let (Some(lo), Some(hi)) = (o.highlight_range_low, o.highlight_range_high) {
        html = crate::apply_highlight_range(
            html,
            lo,
            hi,
            o.highlight_range_color.as_deref().unwrap_or("#6366f1"),
            o.highlight_range_opacity.unwrap_or(0.12),
        );
    }
    if let Some(ref c) = o.iqr_band_color {
        html = crate::apply_iqr_band(html, c, o.iqr_band_opacity.unwrap_or(0.10));
    }
    if let Some(ref c) = o.growth_badge_color {
        html = crate::apply_growth_badge(html, c);
    }
    if o.zscore_heat.unwrap_or(false) {
        html = crate::apply_zscore_heat(html);
    }
    if let Some(t) = o.pareto_marker_threshold {
        html = crate::apply_pareto_marker(html, t, o.pareto_marker_color.as_deref().unwrap_or("#ef4444"));
    }
    if o.diff_from_mean.unwrap_or(false) {
        html = crate::apply_diff_from_mean(
            html,
            o.diff_from_mean_pos_color.as_deref().unwrap_or("#22c55e"),
            o.diff_from_mean_neg_color.as_deref().unwrap_or("#ef4444"),
        );
    }
    if let Some(ref c) = o.scatter_regression_color {
        html = crate::apply_scatter_regression(html, c, o.scatter_regression_width.unwrap_or(2.0));
    }
    if let Some(eps) = o.cluster_eps {
        html = crate::apply_cluster(html, eps, o.cluster_min_samples.unwrap_or(3));
    }
    if let Some(w) = o.rolling_std_band_window {
        html = crate::apply_rolling_std_band(
            html,
            w,
            o.rolling_std_band_color.as_deref().unwrap_or("#6366f1"),
            o.rolling_std_band_opacity.unwrap_or(0.15),
        );
    }
    if let Some(p) = o.forecast_line_periods {
        html = crate::apply_forecast_line(html, p, o.forecast_line_color.as_deref().unwrap_or("#8b5cf6"));
    }
    if let (Some(lo), Some(hi)) = (o.percentile_band_low, o.percentile_band_high) {
        html = crate::apply_percentile_band(
            html,
            lo,
            hi,
            o.percentile_band_color.as_deref().unwrap_or("#6366f1"),
            o.percentile_band_opacity.unwrap_or(0.10),
        );
    }
    html
}

#[cfg(not(feature = "python"))]
fn apply_kwarg_chains(html: String, o: &ChartOpts) -> String {
    let is_3d = html.contains("class=\"c3w\"");
    let html = match o.orientation3d {
        Some(ref mode) => crate::apply_orient3d(html, mode),
        None => html,
    };
    match o.colorbar_position {
        Some(ref pos) => crate::apply_colorbar(html, pos),
        None => {
            let scene = o.scene3d();
            if is_3d && scene.as_str() == "terrain" {
                crate::apply_colorbar(html, "right")
            } else {
                html
            }
        }
    }
}

pub fn apply_bg3d(html: String, o: &ChartOpts) -> String {
    let bg_str = o.bg_str().or_else(crate::plot::get_global_bg);
    let h = if let Some(bg) = bg_str.as_deref() {
        crate::html::hover::apply_bg(html, Some(bg))
    } else {
        html
    };
    let h = apply_annotations(h, o);
    apply_kwarg_chains(h, o)
}

pub fn build_html_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let exporter = crate::html::FastHtmlExporter::new(o.w(900), o.h(480), title.to_string());
    exporter.build_optimized(labels, values)
}
