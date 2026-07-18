use crate::plot::{apply, parse_all};

pub mod basic;
pub mod common;
pub mod config;
pub mod directed;
pub mod mono;
pub mod variant;

pub use config::ChordConfig;
pub use variant::ChordVariant;

pub fn render_chord_html(cfg: &ChordConfig) -> String {
    use ChordVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Ribbon   => basic::render_ribbon(cfg),
        Arc      => basic::render_arc(cfg),
        Mono     => mono::render(cfg),
        Directed => directed::render(cfg),
    }
}

pub use build as build_chord;

#[crate::sera_alias("chord", "chords", "chord_chart", "chord_diagram", "chord_plot")]
#[crate::sera_builder("build_chord")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let matrix  = a.matrix.unwrap_or_default();
    let hover   = o.hj();
    let variant = ChordVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();

    let html = render_chord_html(&ChordConfig {
        variant,
        title,
        labels:  &labels,
        matrix:  &matrix,
        palette: &palette,
        hover:   &hover,
        width:   o.w(700),
        height:  o.h(700),
        ..ChordConfig::default()
    });
    apply(html, &o)
}
