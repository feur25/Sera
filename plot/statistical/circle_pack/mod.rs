use crate::plot::{apply, parse_all};

pub mod basic;
pub mod common;
pub mod config;
pub mod flat;
pub mod variant;

pub use config::CirclePackConfig;
pub use variant::CirclePackVariant;

pub fn render_circle_pack_html(cfg: &CirclePackConfig) -> String {
    use CirclePackVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Flat     => flat::render(cfg),
        Outlined => basic::render_outlined(cfg),
        Gradient => basic::render_gradient(cfg),
        Bubble   => flat::render(cfg),
        LeafFocus => basic::render_leaf_focus(cfg),
    }
}

pub use build as build_circle_pack;

#[crate::sera_alias(
    "circle_pack", "circle_packing", "pack", "packed_circles",
    "bubble_pack", "circle_pack_chart"
)]
#[crate::sera_builder("build_circle_pack")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values  = a.values.unwrap_or_default();
    let hover   = o.hj();
    let variant = CirclePackVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();

    let html = render_circle_pack_html(&CirclePackConfig {
        variant,
        title,
        labels:  &labels,
        parents: &parents,
        values:  &values,
        palette: &palette,
        hover:   &hover,
        width:   o.w(700),
        height:  o.h(700),
        show_labels: o.show_text.unwrap_or(true),
        ..CirclePackConfig::default()
    });
    apply(html, &o)
}
