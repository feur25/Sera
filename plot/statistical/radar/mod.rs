use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod dashed;
pub mod filled;
pub mod gradient;
pub mod lines;
pub mod markers;
pub mod polar_bar;
pub mod stacked;
pub mod variant;

pub use config::RadarConfig;
pub use variant::RadarVariant;

pub fn render_radar_html(cfg: &RadarConfig) -> String {
    use variant::RadarVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Lines => lines::render(cfg),
        Filled => filled::render(cfg),
        Markers => markers::render(cfg),
        Dashed => dashed::render(cfg),
        Stacked => stacked::render(cfg),
        PolarBar => polar_bar::render(cfg),
        Gradient => gradient::render(cfg),
    }
}

pub use build as build_radar_chart;

#[crate::sera_alias("radar", "radar_chart")]
#[crate::sera_builder("build_radar_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    use crate::plot::statistical::{render_radar_html, RadarConfig, RadarVariant};
    let names: Vec<String> = o
        .series_names
        .clone()
        .unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let hover = o.hj();
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();
    let variant = RadarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_radar_html(&RadarConfig {
        title,
        variant,
        axes: &axes,
        series: &series,
        palette: &o.pal(),
        filled: o.filled.unwrap_or(true),
        fill_opacity: o.fill_opacity.unwrap_or(50) as u8,
        width: o.w(700),
        height: o.h(560),
        hover: &hover,
        ..RadarConfig::default()
    });
    apply(html, &o)
}
