use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod milestone;
pub mod progress;
pub mod variant;

pub use config::GanttConfig;
pub use variant::GanttVariant;

pub fn render_gantt_html(cfg: &GanttConfig) -> String {
    use variant::GanttVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Progress => progress::render(cfg),
        Milestone => milestone::render(cfg),
    }
}

pub use build as build_gantt;

#[crate::sera_alias("gantt", "gantt_chart", "broken_barh", "timeline_chart", "project_timeline")]
#[crate::sera_builder("build_gantt")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    let categories = a.categories.clone().unwrap_or_default();
    let progress = o.color_values.clone().unwrap_or_default();
    let dec = crate::plot::decimate::Decimator::new(o.max_points, &values_start);
    let labels = dec.apply(labels);
    let values_start = dec.apply(values_start);
    let values_end = dec.apply(values_end);
    let categories = dec.apply(categories);
    let progress = dec.apply(progress);
    use crate::plot::statistical::{render_gantt_html, GanttConfig, GanttVariant};
    let hover = o.hj();
    let variant = GanttVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();
    let xl = o.xl();
    let yl = o.yl();
    let srt = o.srt();
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let html = render_gantt_html(&GanttConfig {
        variant,
        title,
        x_label: &xl,
        y_label: &yl,
        sort_order: &srt,
        gridlines: o.grid(),
        labels: &labels,
        values_start: &values_start,
        values_end: &values_end,
        categories: &categories,
        progress: &progress,
        palette: &palette,
        colorscale: &colorscale,
        hover: &hover,
        width: o.w(1000),
        height: o.h(520),
        ..GanttConfig::default()
    });
    apply(html, &o)
}
