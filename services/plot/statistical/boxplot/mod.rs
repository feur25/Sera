use crate::plot::{apply, apply_h, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod grouped;
pub mod horizontal;
pub mod letter_value;
pub mod notched;
pub mod outliers;
pub mod points;
pub mod strip;
pub mod variant;
pub mod violin;

pub use config::BoxplotConfig;
pub use variant::BoxplotVariant;

pub fn render_boxplot_html(cfg: &BoxplotConfig) -> String {
    use BoxplotVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Horizontal => horizontal::render(cfg),
        Notched => notched::render(cfg),
        Grouped => grouped::render(cfg),
        Points => points::render(cfg),
        Outliers => outliers::render(cfg),
        Strip => strip::render(cfg),
        Violin => violin::render(cfg),
        LetterValue => letter_value::render(cfg),
    }
}

pub use build as build_boxplot;

#[crate::sera_alias("boxplot", "box_plot")]
#[crate::sera_builder("build_boxplot")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let series = a.series.unwrap_or_default();
    let series_names = o.series_names.clone().unwrap_or_default();
    let hover = o.hj();
    let mut variant =
        crate::plot::statistical::BoxplotVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    if o.is_horiz() && matches!(variant, crate::plot::statistical::BoxplotVariant::Basic) {
        variant = crate::plot::statistical::BoxplotVariant::Horizontal;
    }
    let (category_labels, values) = if values.is_empty()
        && !series.is_empty()
        && !matches!(variant, crate::plot::statistical::BoxplotVariant::Grouped)
    {
        let flat_len: usize = series.iter().map(|s| s.len()).sum();
        let mut labels_flat = Vec::with_capacity(flat_len);
        let mut values_flat = Vec::with_capacity(flat_len);
        for (i, s) in series.iter().enumerate() {
            let cat = category_labels.get(i).cloned().unwrap_or_default();
            for &v in s {
                labels_flat.push(cat.clone());
                values_flat.push(v);
            }
        }
        (labels_flat, values_flat)
    } else {
        (category_labels, values)
    };
    let html =
        crate::plot::statistical::render_boxplot_html(&crate::plot::statistical::BoxplotConfig {
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
            show_text: o.show_text.unwrap_or(false),
            jitter: o.jitter.unwrap_or(0.35),
            violin_overlay: o.violin_overlay.unwrap_or(false),
            boxen_depth: o.boxen_depth.unwrap_or(4),
            fill_opacity: o.fill_opacity_real.unwrap_or(0.28),
            stroke_width: o.box_stroke_width.unwrap_or(1.6),
        });
    use crate::plot::statistical::BoxplotVariant::*;
    let native = matches!(variant, Horizontal);
    if native {
        apply_h(html, &o)
    } else {
        apply(html, &o)
    }
}

#[cfg(test)]
mod series_regression_tests {
    #[test]
    fn documented_chart_demo_example_renders_non_empty() {
        let input = r#"{"labels":["A","B","C"],"series":[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]}"#;
        let html = super::build(input);
        assert!(!html.is_empty(), "boxplot's own documented series= example must not render empty");
        assert!(html.contains("<svg"));
    }

    #[test]
    fn grouped_variant_still_uses_series_as_multi_series_not_per_category() {
        let input = r#"{"labels":["A","B"],"series":[[1.0,2.0],[3.0,4.0]],"variant":"grouped"}"#;
        let html = super::build(input);
        assert!(!html.is_empty(), "grouped variant should still render from raw series");
    }
}
