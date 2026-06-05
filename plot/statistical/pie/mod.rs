use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod donut;
pub mod exploded;
pub mod kpi;
pub mod nested;
pub mod pattern;
pub mod proportional;
pub mod semi;
pub mod subplots;
pub mod variant;

pub use config::{Pie, PieConfig};
pub use variant::PieVariant;

use crate::html::hover::{build_chart_html, slots_to_json};

pub fn render_pie_html(cfg: &PieConfig) -> String {
    let svg = match cfg.variant {
        PieVariant::Basic => basic::render(cfg),
        PieVariant::Donut => donut::render(cfg),
        PieVariant::Exploded => exploded::render(cfg),
        PieVariant::Subplots => subplots::render(cfg),
        PieVariant::Proportional => proportional::render(cfg),
        PieVariant::Semi => semi::render(cfg),
        PieVariant::Kpi => kpi::render(cfg),
        PieVariant::Nested => nested::render(cfg),
        PieVariant::Pattern => pattern::render(cfg),
    };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

#[crate::sera_alias("pie_chart", "pie_chart_legacy", "basic_pie")]
#[crate::sera_builder]
pub fn build_pie_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_pie_html, PieConfig, PieVariant};
    let hover = o.hj();
    let pull = o.pull.clone().unwrap_or_default();
    let html = render_pie_html(&PieConfig {
        variant: PieVariant::Basic,
        title,
        labels: &labels,
        values: &values,
        palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true),
        sort_order: &o.srt(),
        width: o.w(720),
        height: o.h(440),
        hover: &hover,
        pull: &pull,
        min_label_frac: o.min_label_frac.unwrap_or(0.04),
        ..PieConfig::default()
    });
    apply(html, &o)
}

#[crate::sera_alias("donut", "donut_chart")]
#[crate::sera_builder]
pub fn build_donut_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_pie_html, PieConfig, PieVariant};
    let hover = o.hj();
    let pull = o.pull.clone().unwrap_or_default();
    let html = render_pie_html(&PieConfig {
        variant: PieVariant::Donut,
        title,
        labels: &labels,
        values: &values,
        palette: &o.pal(),
        show_pct: o.show_pct.unwrap_or(true),
        sort_order: &o.srt(),
        width: o.w(720),
        height: o.h(440),
        hover: &hover,
        donut: o.inner_radius_ratio.unwrap_or(0.55).clamp(0.0, 0.9),
        pull: &pull,
        min_label_frac: o.min_label_frac.unwrap_or(0.04),
        ..PieConfig::default()
    });
    apply(html, &o)
}

pub use build as build_pie;

#[crate::sera_alias(
    "pie",
    "pie_unified",
    "pie_family",
    "pies",
    "semi_pie",
    "half_pie",
    "kpi_pie",
    "kpi_donut",
    "nested_pie",
    "concentric_pie",
    "pattern_pie"
)]
#[crate::sera_builder("build_pie")]
pub fn build(input: &str) -> String {
    use crate::plot::statistical::{render_pie_html, PieConfig, PieVariant};
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
        variant,
        title,
        x_label: "",
        y_label: "",
        gridlines: false,
        sort_order: &srt,
        hover: &hover,
        legend_position: &lp,
        width: o.w(720),
        height: o.h(440),
        labels: &labels,
        values: &values,
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
