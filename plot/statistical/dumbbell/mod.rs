use crate::plot::{parse_all, apply};
pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod arrow;
pub mod delta;
pub mod barbell;
pub mod glow;
pub mod dotted;
pub mod ranked;

pub use variant::DumbbellVariant;
pub use config::DumbbellConfig;

pub fn render_dumbbell_html(cfg: &DumbbellConfig) -> String {
    use variant::DumbbellVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Arrow    => arrow::render(cfg),
        Delta    => delta::render(cfg),
        Barbell  => barbell::render(cfg),
        Glow     => glow::render(cfg),
        Dotted   => dotted::render(cfg),
        Ranked   => ranked::render(cfg),
    }
}


pub use build as build_dumbbell;

#[crate::sera_alias("dumbbell", "dumbbells", "dumbbell_chart", "dumbbell_family", "dumbbell_plot")]
#[crate::sera_builder("build_dumbbell")]
pub fn build(input: &str) -> String {
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
