use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod connected;
pub mod density;
pub mod gradient;
pub mod variant;

pub use config::EventplotConfig;
pub use variant::EventplotVariant;

pub fn render_eventplot_html(cfg: &EventplotConfig) -> String {
    use variant::EventplotVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Density => density::render(cfg),
        Gradient => gradient::render(cfg),
        Connected => connected::render(cfg),
    }
}

pub use build as build_eventplot;

#[crate::sera_alias("eventplot", "event_plot", "raster_plot", "spike_plot")]
#[crate::sera_builder("build_eventplot")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let categories = a.categories.clone().unwrap_or_default();
    use crate::plot::statistical::{render_eventplot_html, EventplotConfig, EventplotVariant};
    let hover = o.hj();
    let variant = EventplotVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();
    let xl = o.xl();
    let yl = o.yl();
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let html = render_eventplot_html(&EventplotConfig {
        variant,
        title,
        x_label: &xl,
        y_label: &yl,
        gridlines: o.grid(),
        x_values: &x_values,
        categories: &categories,
        palette: &palette,
        colorscale: &colorscale,
        hover: &hover,
        width: o.w(900),
        height: o.h(500),
        ..EventplotConfig::default()
    });
    apply(html, &o)
}
