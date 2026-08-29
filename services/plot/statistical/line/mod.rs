use crate::plot::{apply, parse_all};
pub mod band;
pub mod basic;
pub mod config;
pub mod connected_scatter;
pub mod dashed;
pub mod epoch;
pub mod filled;
pub mod gapped;
pub mod momentum;
pub mod multi;
pub mod pace;
pub mod sparkline;
pub mod spline;
pub mod stepped;
pub mod variant;

pub use config::LineConfig;
pub use variant::LineVariant;

const CANVAS_FALLBACK_THRESHOLD: usize = 3_000;

fn canvas_fallback(cfg: &LineConfig) -> Option<String> {
    use LineVariant::*;
    if !cfg.series.is_empty() || cfg.values.len() <= CANVAS_FALLBACK_THRESHOLD {
        return None;
    }
    let mode = match cfg.variant {
        Spline | Dashed | Gapped => crate::plot::canvas_points::MODE_LINE,
        ConnectedScatter if !cfg.show_points => crate::plot::canvas_points::MODE_LINE,
        _ => return None,
    };
    let x_values: Vec<f64> = (0..cfg.values.len()).map(|i| i as f64).collect();
    let spec = crate::plot::canvas_points::CanvasPlotSpec {
        title: cfg.title,
        width: cfg.width,
        height: cfg.height,
        x_label: cfg.x_label,
        y_label: cfg.y_label,
        gridlines: cfg.gridlines,
        mode,
        color_hex: if cfg.color_hex != 0 {
            cfg.color_hex
        } else {
            cfg.palette.first().copied().unwrap_or(0x636EFA)
        },
    };
    Some(crate::plot::canvas_points::render_canvas_points_html(
        &spec, &x_values, cfg.values,
    ))
}

pub fn render_line_html(cfg: &LineConfig) -> String {
    use LineVariant::*;
    if let Some(html) = canvas_fallback(cfg) {
        return html;
    }
    match cfg.variant {
        Basic => basic::render(cfg),
        Multi => multi::render(cfg),
        Stepped => stepped::render(cfg),
        Spline => spline::render(cfg),
        Filled => filled::render(cfg),
        Sparkline => sparkline::render(cfg),
        Dashed => dashed::render(cfg),
        ConnectedScatter => connected_scatter::render(cfg),
        Gapped => gapped::render(cfg),
        Band => band::render(cfg),
        Momentum => momentum::render(cfg),
        Epoch => epoch::render(cfg),
        Pace => pace::render(cfg),
    }
}

pub use build as build_line;

#[crate::sera_alias(
    "line",
    "line_chart",
    "line_unified",
    "lines_unified",
    "line_family",
    "lines_family"
)]
#[crate::sera_builder("build_line")]
pub fn build(input: &str) -> String {
    use crate::plot::statistical::{render_line_html, LineConfig, LineVariant};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = LineVariant::from_str(o.variant.as_deref().unwrap_or("basic"));

    let x_labels = a.x_labels.clone().unwrap_or_else(|| {
        a.x.as_ref()
            .map(|xs| xs.iter().map(|&v| crate::plot::statistical::common::format_axis_label(v)).collect())
            .unwrap_or_default()
    });
    let labels = a.labels.clone().unwrap_or_else(|| x_labels.clone());
    let values = a.values.clone().unwrap_or_else(|| a.y.clone().unwrap_or_default());
    let hover = o.hj();
    let palette = o.pal();
    let xl = o.xl();
    let yl = o.yl();
    let srt = o.srt();
    let lp = o.lp();

    let series: Vec<(String, Vec<f64>)> = {
        let sn = o.series_names.clone().unwrap_or_default();
        if let Some(s) = a.series.as_ref() {
            s.iter()
                .enumerate()
                .map(|(si, vals)| {
                    (
                        sn.get(si)
                            .cloned()
                            .unwrap_or_else(|| format!("S{}", si + 1)),
                        vals.clone(),
                    )
                })
                .collect()
        } else {
            Vec::new()
        }
    };

    let (labels, values, x_labels, series) = if !series.is_empty() {
        let dec = crate::plot::decimate::Decimator::for_series(o.max_points, &series);
        (dec.apply(labels), values, dec.apply(x_labels), dec.apply_each(series))
    } else {
        let dec = crate::plot::decimate::Decimator::new(o.max_points, &values);
        (dec.apply(labels), dec.apply(values), dec.apply(x_labels), series)
    };

    let step_shape = o
        .step_shape
        .clone()
        .or_else(|| o.line_shape.clone())
        .unwrap_or_else(|| "hv".to_string());
    let dash_pattern = o.dash_pattern.clone().unwrap_or_else(|| "auto".to_string());

    let cfg = LineConfig {
        variant,
        title,
        x_label: &xl,
        y_label: &yl,
        width: o.w(900),
        height: o.h(480),
        gridlines: o.grid(),
        sort_order: &srt,
        legend_position: &lp,
        hover: &hover,
        palette: &palette,
        labels: &labels,
        values: &values,
        color_hex: o.color_hex.unwrap_or(0),
        show_points: o.show_points.unwrap_or(false),
        series: &series,
        x_labels: &x_labels,
        step_shape: &step_shape,
        spline_tension: o.spline_tension.unwrap_or(0.5),
        fill_opacity: o
            .fill_opacity_f
            .unwrap_or_else(|| o.fill_opacity.map(|i| i as f64 / 100.0).unwrap_or(0.3)),
        stack_fill: o.stack_fill.unwrap_or(false),
        dash_pattern: &dash_pattern,
        stroke_width: o.stroke_width.unwrap_or(2.0),
        marker_size: o.marker_size.unwrap_or(4),
        gap_threshold: o.gap_threshold.unwrap_or(f64::NAN),
        spark_cols: o.spark_cols.unwrap_or(3),
        spark_cell_h: o.spark_cell_h.unwrap_or(60),
        spark_cell_w: o.spark_cell_w.unwrap_or(220),
        epoch_pos_color: o.epoch_pos_color.unwrap_or(0xB91C1C),
        epoch_neg_color: o.epoch_neg_color.unwrap_or(0x1D4ED8),
        epoch_flat_color: o.epoch_flat_color.unwrap_or(0x64748B),
        pace_target: o.pace_target,
        pace_ahead_color: o.pace_ahead_color.unwrap_or(0x16A34A),
        pace_behind_color: o.pace_behind_color.unwrap_or(0xDC2626),
    };
    let html = render_line_html(&cfg);
    apply(html, &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_line_accepts_x_y_as_an_alias_for_labels_values() {
        let via_xy = build(r#"{"title":"t","x":[2010.0,2011.0,2012.0],"y":[10.0,20.0,15.0]}"#);
        for needle in ["2010", "2011", "2012"] {
            assert!(via_xy.contains(needle), "x/y input should carry the year {needle} through as a visible tick label: {via_xy}");
        }
    }

    #[test]
    fn build_line_prefers_explicit_labels_values_over_x_y_when_both_are_present() {
        let out = build(r#"{"title":"t","x":[8172.0,8173.0],"y":[8172.0,8173.0],"labels":["Alpha","Beta"],"values":[10.0,20.0]}"#);
        assert!(!out.contains("8172") && !out.contains("8173"), "explicit labels/values must take priority over x/y when both are present, but the x-derived label leaked through: {out}");
        assert!(out.contains("Alpha") && out.contains("Beta"), "the explicit labels must appear when labels/values win over x/y: {out}");
    }

    #[test]
    fn build_line_with_x_y_and_connected_scatter_variant_renders_real_markers_not_an_empty_chart() {
        let out = build(r#"{"title":"t","x":[1.0,2.0,3.0,4.0],"y":[5.0,9.0,3.0,7.0],"variant":"connected_scatter","show_points":true}"#);
        assert!(out.contains("<circle"), "connected_scatter constructed from x/y must produce real point markers, not an empty fallback: {out}");
    }

    #[test]
    fn build_line_with_x_y_still_works_below_the_canvas_fallback_threshold_for_every_variant() {
        let x: Vec<f64> = (0..10).map(|i| 2000.0 + i as f64).collect();
        let y: Vec<f64> = (0..10).map(|i| (i as f64) * 1.5).collect();
        let input = serde_json::json!({"title": "t", "x": x, "y": y}).to_string();
        let out = build(&input);
        assert!(out.contains("2000"), "a small x/y-constructed line chart must carry real year labels through to the rendered output: {out}");
    }

    #[test]
    fn build_line_with_stepped_variant_above_the_canvas_fallback_threshold_still_renders_the_real_step_shaped_polyline() {
        let n = CANVAS_FALLBACK_THRESHOLD + 500;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 0.05).collect();
        let y: Vec<f64> = (0..n).map(|i| if (i / 400) % 2 == 0 { 18.0 } else { 21.0 }).collect();
        let input = serde_json::json!({"title": "t", "x": x, "y": y, "variant": "stepped"}).to_string();
        let out = build(&input);
        assert!(out.contains("data-idx=\"0\""), "a large stepped chart must still go through stepped::render's own polyline, not silently disappear: {out}");
        assert!(!out.contains("<canvas id="), "a large stepped chart must not fall back to the generic connect-the-dots canvas renderer, which cannot represent instant value jumps: {out}");
    }

    #[test]
    fn perf_build_line_with_stepped_variant_on_a_large_dataset_stays_fast_without_the_canvas_fallback() {
        let n = 8_000usize;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 0.05).collect();
        let y: Vec<f64> = (0..n).map(|i| if (i / 400) % 2 == 0 { 18.0 } else { 21.0 }).collect();
        let input = serde_json::json!({"title": "t", "x": x, "y": y, "variant": "stepped"}).to_string();

        let start = std::time::Instant::now();
        let iterations = 20;
        for _ in 0..iterations {
            std::hint::black_box(build(&input));
        }
        let elapsed = start.elapsed();
        let per_call = elapsed / iterations;
        assert!(
            per_call.as_millis() < 200,
            "stepped::render on {n} points took {per_call:?}/call without the canvas fallback, expected comfortably under 200ms"
        );
    }

    #[test]
    fn build_line_with_basic_variant_above_the_canvas_fallback_threshold_still_renders_a_real_svg_polyline() {
        let n = CANVAS_FALLBACK_THRESHOLD + 500;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 0.02).collect();
        let y: Vec<f64> = (0..n).map(|i| 50.0 + (i as f64 * 0.01).sin() * 10.0).collect();
        let input = serde_json::json!({"title": "t", "x": x, "y": y, "variant": "basic"}).to_string();
        let out = build(&input);
        assert!(out.contains("data-idx=\"0\""), "a large basic line chart must still go through the real SVG polyline, not silently disappear: {out}");
        assert!(!out.contains("<canvas id="), "a large basic line chart must not fall back to the non-rasterizable canvas+JS renderer, which SeraStudio's static export cannot execute: {out}");
    }

    #[test]
    fn perf_build_line_with_basic_variant_on_a_large_dataset_stays_fast_without_the_canvas_fallback() {
        let n = 8_000usize;
        let x: Vec<f64> = (0..n).map(|i| i as f64 * 0.02).collect();
        let y: Vec<f64> = (0..n).map(|i| 50.0 + (i as f64 * 0.01).sin() * 10.0).collect();
        let input = serde_json::json!({"title": "t", "x": x, "y": y, "variant": "basic"}).to_string();

        let start = std::time::Instant::now();
        let iterations = 20;
        for _ in 0..iterations {
            std::hint::black_box(build(&input));
        }
        let elapsed = start.elapsed();
        let per_call = elapsed / iterations;
        assert!(
            per_call.as_millis() < 200,
            "basic::render on {n} points took {per_call:?}/call without the canvas fallback, expected comfortably under 200ms"
        );
    }
}
