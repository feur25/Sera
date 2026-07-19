use super::annotate::apply_annotations;
use super::parse::parse_all;
use super::types::ChartOpts;

pub fn apply(html: String, o: &ChartOpts) -> String {
    let html = crate::apply_responsive(html);
    let bg_str = o.bg_str().or_else(crate::plot::get_global_bg);
    let bg = bg_str.as_deref();
    let h = crate::html::hover::apply_opts(html, bg, !o.no_x(), !o.no_y());
    let h = crate::html::hover::apply_rotation(h, o.rotation_deg());
    let h = apply_annotations(h, o);
    let h = apply_kwarg_chains(h, o);
    let h = crate::apply_global_color_bindings(h);
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
    let h = crate::apply_global_color_bindings(h);
    if let Some(ref t) = o.theme {
        crate::plot::statistical::apply_chart_theme(h, t)
    } else {
        h
    }
}

type KwargStep = fn(String, &ChartOpts, bool) -> String;

const KWARG_STEPS: &[KwargStep] = &[
    |html, o, _is3d| match &o.orientation3d {
        Some(mode) => crate::apply_orient3d(html, mode),
        None => html,
    },
    |html, o, is3d| match &o.colorbar_position {
        Some(pos) => crate::apply_colorbar(html, pos),
        None if is3d && o.scene3d().as_str() == "terrain" => crate::apply_colorbar(html, "right"),
        None => html,
    },
    |html, o, _is3d| {
        if o.despine.unwrap_or_else(crate::plot::utils::get_global_despine) {
            crate::apply_despine(html)
        } else {
            html
        }
    },
    |html, o, _is3d| match &o.watermark_text {
        Some(t) => crate::apply_watermark(html, t, o.watermark_opacity.unwrap_or(0.08)),
        None => match crate::plot::utils::get_global_watermark() {
            Some((t, op)) => crate::apply_watermark(html, &t, op),
            None => html,
        },
    },
    |html, o, _is3d| match &o.caption {
        Some(t) => crate::apply_caption(html, t),
        None => html,
    },
    |html, o, _is3d| match &o.glow_color {
        Some(c) => crate::apply_glow(html, c),
        None => html,
    },
    |html, o, _is3d| match o.highlight_at {
        Some(idx) => crate::apply_highlight(html, idx, o.highlight_color.as_deref().unwrap_or("#f59e0b")),
        None => html,
    },
    |html, o, _is3d| match o.hline_value {
        Some(v) => crate::apply_hline(
            html,
            v,
            o.hline_color.as_deref().unwrap_or("#ef4444"),
            o.hline_label.as_deref(),
        ),
        None => html,
    },
    |html, o, _is3d| match &o.subtitle {
        Some(t) => crate::apply_subtitle(html, t),
        None => html,
    },
    |html, o, _is3d| match o.shadow_blur {
        Some(blur) => crate::apply_shadow(html, blur, o.shadow_color.as_deref().unwrap_or("rgba(0,0,0,.35)")),
        None => match crate::plot::utils::get_global_shadow() {
            Some((blur, color)) => crate::apply_shadow(html, blur, color.as_deref().unwrap_or("rgba(0,0,0,.35)")),
            None => html,
        },
    },
    |html, o, _is3d| match o.pulse_duration {
        Some(dur) => crate::apply_pulse(html, dur, o.pulse_index.as_deref(), o.pulse_above, o.pulse_color.as_deref()),
        None => html,
    },
    |html, o, _is3d| match &o.outline_color {
        Some(c) => crate::apply_outline(html, c, o.outline_width.unwrap_or(2.0)),
        None => html,
    },
    |html, o, _is3d| {
        if o.value_labels.unwrap_or(false) || o.value_labels_decimals.is_some() || o.value_labels_color.is_some() {
            crate::apply_value_labels(
                html,
                o.value_labels_decimals.unwrap_or(0),
                o.value_labels_color.as_deref().unwrap_or("#475569"),
            )
        } else {
            html
        }
    },
    |html, o, _is3d| match o.error_bars_margin {
        Some(margin) => crate::apply_error_bars(html, margin, o.error_bars_color.as_deref().unwrap_or("#475569")),
        None => html,
    },
    |html, o, _is3d| {
        if o.delta_labels.unwrap_or(false) {
            crate::apply_delta_labels(
                html,
                o.delta_labels_pos_color.as_deref().unwrap_or("#22c55e"),
                o.delta_labels_neg_color.as_deref().unwrap_or("#ef4444"),
            )
        } else {
            html
        }
    },
    |html, o, _is3d| match &o.cumulative_line_color {
        Some(c) => crate::apply_cumulative_line(html, c),
        None => html,
    },
    |html, o, _is3d| match o.rank_badges_top_n {
        Some(n) => crate::apply_rank_badges(html, n, o.rank_badges_color.as_deref().unwrap_or("#6366f1")),
        None => html,
    },
    |html, o, _is3d| {
        if o.log_scale.unwrap_or(false) {
            crate::apply_log_scale(html)
        } else {
            html
        }
    },
    |html, o, _is3d| match o.moving_average_window {
        Some(w) => crate::apply_moving_average(html, w, o.moving_average_color.as_deref().unwrap_or("#f59e0b")),
        None => html,
    },
    |html, o, _is3d| match o.outliers_threshold {
        Some(t) => crate::apply_outliers(html, t, o.outliers_color.as_deref().unwrap_or("#ef4444")),
        None => html,
    },
    |html, o, _is3d| match &o.fill_between_color {
        Some(c) => crate::apply_fill_between(html, c, o.fill_between_opacity.unwrap_or(0.15)),
        None => html,
    },
    |html, o, _is3d| match &o.box_annotate_color {
        Some(c) => crate::apply_box_annotate(html, c),
        None => html,
    },
    |html, o, _is3d| {
        if o.pct_of_total.unwrap_or(false) || o.pct_of_total_decimals.is_some() || o.pct_of_total_color.is_some() {
            crate::apply_pct_of_total(
                html,
                o.pct_of_total_decimals.unwrap_or(1),
                o.pct_of_total_color.as_deref().unwrap_or("#475569"),
            )
        } else {
            html
        }
    },
    |html, o, _is3d| match &o.correlation_badge_color {
        Some(c) => crate::apply_correlation_badge(html, c),
        None => html,
    },
    |html, o, _is3d| match (o.highlight_range_low, o.highlight_range_high) {
        (Some(lo), Some(hi)) => crate::apply_highlight_range(
            html,
            lo,
            hi,
            o.highlight_range_color.as_deref().unwrap_or("#6366f1"),
            o.highlight_range_opacity.unwrap_or(0.12),
        ),
        _ => html,
    },
    |html, o, _is3d| match &o.iqr_band_color {
        Some(c) => crate::apply_iqr_band(html, c, o.iqr_band_opacity.unwrap_or(0.10)),
        None => html,
    },
    |html, o, _is3d| match &o.growth_badge_color {
        Some(c) => crate::apply_growth_badge(html, c),
        None => html,
    },
    |html, o, _is3d| {
        if o.zscore_heat.unwrap_or(false) {
            crate::apply_zscore_heat(html)
        } else {
            html
        }
    },
    |html, o, _is3d| match o.pareto_marker_threshold {
        Some(t) => crate::apply_pareto_marker(html, t, o.pareto_marker_color.as_deref().unwrap_or("#ef4444")),
        None => html,
    },
    |html, o, _is3d| {
        if o.diff_from_mean.unwrap_or(false) {
            crate::apply_diff_from_mean(
                html,
                o.diff_from_mean_pos_color.as_deref().unwrap_or("#22c55e"),
                o.diff_from_mean_neg_color.as_deref().unwrap_or("#ef4444"),
            )
        } else {
            html
        }
    },
    |html, o, _is3d| match &o.scatter_regression_color {
        Some(c) => crate::apply_scatter_regression(html, c, o.scatter_regression_width.unwrap_or(2.0)),
        None => html,
    },
    |html, o, _is3d| match o.cluster_eps {
        Some(eps) => crate::apply_cluster(html, eps, o.cluster_min_samples.unwrap_or(3)),
        None => html,
    },
    |html, o, _is3d| match o.rolling_std_band_window {
        Some(w) => crate::apply_rolling_std_band(
            html,
            w,
            o.rolling_std_band_color.as_deref().unwrap_or("#6366f1"),
            o.rolling_std_band_opacity.unwrap_or(0.15),
        ),
        None => html,
    },
    |html, o, _is3d| match o.forecast_line_periods {
        Some(p) => crate::apply_forecast_line(html, p, o.forecast_line_color.as_deref().unwrap_or("#8b5cf6")),
        None => html,
    },
    |html, o, _is3d| match (o.percentile_band_low, o.percentile_band_high) {
        (Some(lo), Some(hi)) => crate::apply_percentile_band(
            html,
            lo,
            hi,
            o.percentile_band_color.as_deref().unwrap_or("#6366f1"),
            o.percentile_band_opacity.unwrap_or(0.10),
        ),
        _ => html,
    },
];

#[cfg(feature = "python")]
fn apply_kwarg_chains(html: String, o: &ChartOpts) -> String {
    use crate::{json_str, SP_LEGEND_JS};
    let is_3d = html.contains("class=\"c3w\"");
    let html = if is_3d {
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
    KWARG_STEPS.iter().fold(html, |h, step| step(h, o, is_3d))
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
    let h = apply_kwarg_chains(h, o);
    crate::apply_global_color_bindings(h)
}

pub fn build_html_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let exporter = crate::html::FastHtmlExporter::new(o.w(900), o.h(480), title.to_string());
    exporter.build_optimized(labels, values)
}
