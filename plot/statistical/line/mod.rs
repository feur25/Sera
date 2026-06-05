use crate::plot::{apply, parse_all};
pub mod basic;
pub mod config;
pub mod connected_scatter;
pub mod dashed;
pub mod filled;
pub mod gapped;
pub mod multi;
pub mod neon;
pub mod sparkline;
pub mod spline;
pub mod stepped;
pub mod variant;

pub use config::LineConfig;
pub use variant::LineVariant;

pub fn render_line_html(cfg: &LineConfig) -> String {
    use LineVariant::*;
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
        Neon => neon::render(cfg),
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

    let x_labels = a.x_labels.clone().unwrap_or_default();
    let labels = a.labels.clone().unwrap_or_else(|| x_labels.clone());
    let values = a.values.clone().unwrap_or_default();
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
    };
    let html = render_line_html(&cfg);
    apply(html, &o)
}
