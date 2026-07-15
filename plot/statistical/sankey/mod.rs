use crate::plot::{apply, parse_all};

pub mod basic;
pub mod common;
pub mod config;
pub mod gapped;
pub mod ribbon;
pub mod variant;

pub use config::SankeyConfig;
pub use variant::SankeyVariant;

pub fn render_sankey_html(cfg: &SankeyConfig) -> String {
    use SankeyVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Gradient => basic::render_gradient(cfg),
        Gapped   => gapped::render(cfg),
        Ribbon   => ribbon::render(cfg),
        Minimal  => basic::render_minimal(cfg),
    }
}

pub use build as build_sankey;

#[crate::sera_alias("sankey", "sankeys", "sankey_chart", "sankey_diagram", "flow_chart")]
#[crate::sera_builder("build_sankey")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_else(|| vec![1.0; sources.len()]);
    let hover   = o.hj();
    let variant = SankeyVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();

    let html = render_sankey_html(&SankeyConfig {
        variant,
        title,
        labels:     &labels,
        sources:    &sources,
        targets:    &targets,
        weights:    &weights,
        palette:    &palette,
        hover:      &hover,
        width:      o.w(900),
        height:     o.h(520),
        node_width: 18,
        node_gap:   10,
        ..SankeyConfig::default()
    });
    apply(html, &o)
}