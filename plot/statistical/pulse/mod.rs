use crate::plot::{apply, parse_all};

pub mod basic;
pub mod config;
pub mod variant;

pub use config::PulseConfig;
pub use variant::PulseVariant;

pub fn render_pulse_html(cfg: &PulseConfig) -> String {
    use PulseVariant::*;
    match cfg.variant {
        Radial   => basic::render(cfg),
        Wave     => basic::render_wave(cfg),
        Dot      => basic::render_dot(cfg),
        Filled   => basic::render_filled(cfg),
        Gradient => basic::render_gradient(cfg),
        Outlined => basic::render_outlined(cfg),
    }
}

pub use build as build_pulse;

#[crate::sera_alias("pulse", "pulse_chart", "radial_bar", "clock_chart", "rhythm", "radial_rhythm")]
#[crate::sera_builder("build_pulse")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let values  = a.values.unwrap_or_default();
    let palette = o.pal();
    let variant = PulseVariant::from_str(o.variant.as_deref().unwrap_or("radial"));
    let hover   = o.hj();

    let html = render_pulse_html(&PulseConfig {
        variant,
        title,
        labels:  &labels,
        values:  &values,
        palette: &palette,
        hover:   &hover,
        width:   o.w(560),
        height:  o.h(560),
        ..PulseConfig::default()
    });
    apply(html, &o)
}
