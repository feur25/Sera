use crate::plot::{parse_all, apply};
pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod monochrome;
pub mod highlighted;
pub mod bumps;
pub mod curved;
pub mod thick;
pub mod diverging;
pub mod stepped;

pub use variant::SlopeVariant;
pub use config::SlopeConfig;

pub fn render_slope_html(cfg: &SlopeConfig) -> String {
    use variant::SlopeVariant::*;
    match cfg.variant {
        Basic       => basic::render(cfg),
        Monochrome  => monochrome::render(cfg),
        Highlighted => highlighted::render(cfg),
        Bumps       => bumps::render(cfg),
        Curved      => curved::render(cfg),
        Thick       => thick::render(cfg),
        Diverging   => diverging::render(cfg),
        Stepped     => stepped::render(cfg),
    }
}


pub use build as build_slope;

#[crate::sera_alias("slope", "slopes", "slope_chart", "slope_family", "slopegraph")]
#[crate::sera_builder("build_slope")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_left = a.left.unwrap_or_default();
    let values_right = a.right.unwrap_or_default();
    use crate::plot::statistical::{SlopeConfig, SlopeVariant, render_slope_html};
    let hover = o.hj();
    let ll = o.left_label.as_deref().unwrap_or("Before").to_string();
    let rl = o.right_label.as_deref().unwrap_or("After").to_string();
    let variant = SlopeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_slope_html(&SlopeConfig {
        title, variant, labels: &labels, values_left: &values_left, values_right: &values_right,
        left_label: &ll, right_label: &rl, palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true), width: o.w(700), height: o.h(500),
        sort_order: &o.srt(), hover: &hover, ..SlopeConfig::default()
    });
    apply(html, &o)
}
