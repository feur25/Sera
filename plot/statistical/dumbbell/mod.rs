use crate::plot::{apply, parse_all};
pub mod arrow;
pub mod barbell;
pub mod basic;
pub mod common;
pub mod config;
pub mod delta;
pub mod dotted;
pub mod glow;
pub mod ranked;
pub mod variant;

pub use config::DumbbellConfig;
pub use variant::DumbbellVariant;

pub fn render_dumbbell_html(cfg: &DumbbellConfig) -> String {
    use variant::DumbbellVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Arrow => arrow::render(cfg),
        Delta => delta::render(cfg),
        Barbell => barbell::render(cfg),
        Glow => glow::render(cfg),
        Dotted => dotted::render(cfg),
        Ranked => ranked::render(cfg),
    }
}

pub use build as build_dumbbell;

#[crate::sera_alias(
    "dumbbell",
    "dumbbells",
    "dumbbell_chart",
    "dumbbell_family",
    "dumbbell_plot"
)]
#[crate::sera_builder("build_dumbbell")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    let dec = crate::plot::decimate::Decimator::new(o.max_points, &values_start);
    let labels = dec.apply(labels);
    let values_start = dec.apply(values_start);
    let values_end = dec.apply(values_end);
    use crate::plot::statistical::dumbbell::{
        render_dumbbell_html, DumbbellConfig, DumbbellVariant,
    };
    let variant = DumbbellVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let s = o
        .series_name_start
        .as_deref()
        .unwrap_or("Start")
        .to_string();
    let e = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let hover = o.hj();
    let html = render_dumbbell_html(&DumbbellConfig {
        variant,
        title,
        labels: &labels,
        values_start: &values_start,
        values_end: &values_end,
        series_names: (&s, &e),
        palette: &o.pal(),
        width: o.w(1000),
        height: o.h(500),
        x_label: &o.xl(),
        y_label: &o.yl(),
        gridlines: o.grid(),
        sort_order: &o.srt(),
        hover: &hover,
        show_text: o.show_text.unwrap_or(false),
        ..DumbbellConfig::default()
    });
    apply(html, &o)
}
