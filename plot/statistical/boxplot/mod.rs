use crate::plot::{parse_all, apply, apply_h};
pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod horizontal;
pub mod notched;
pub mod grouped;
pub mod points;
pub mod outliers;
pub mod strip;
pub mod violin;
pub mod letter_value;
pub mod rainbow;

pub use variant::BoxplotVariant;
pub use config::BoxplotConfig;

pub fn render_boxplot_html(cfg: &BoxplotConfig) -> String {
    use BoxplotVariant::*;
    match cfg.variant {
        Basic       => basic::render(cfg),
        Horizontal  => horizontal::render(cfg),
        Notched     => notched::render(cfg),
        Grouped     => grouped::render(cfg),
        Points      => points::render(cfg),
        Outliers    => outliers::render(cfg),
        Strip       => strip::render(cfg),
        Violin      => violin::render(cfg),
        LetterValue => letter_value::render(cfg),
        Rainbow     => rainbow::render(cfg),
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
