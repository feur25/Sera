use crate::plot::{apply, parse_all};

pub mod basic;
pub mod common;
pub mod config;
pub mod variant;

pub use config::DendrogramConfig;
pub use variant::DendrogramVariant;

pub fn render_dendrogram_html(cfg: &DendrogramConfig) -> String {
    use DendrogramVariant::*;
    match cfg.variant {
        Vertical   => basic::render(cfg),
        Horizontal => basic::render_horizontal(cfg),
        Radial     => basic::render_radial(cfg),
        Compact    => basic::render_compact(cfg),
        Elegant    => basic::render_elegant(cfg),
    }
}

pub use build as build_dendrogram;

#[crate::sera_alias("dendrogram", "dendro", "tree", "tree_diagram", "hierarchy", "hierarchical")]
#[crate::sera_builder("build_dendrogram")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let palette = o.pal();
    let variant = DendrogramVariant::from_str(o.variant.as_deref().unwrap_or("vertical"));
    let hover   = o.hj();

    let html = render_dendrogram_html(&DendrogramConfig {
        variant,
        title,
        labels:      &labels,
        parents:     &parents,
        palette:     &palette,
        hover:       &hover,
        width:       o.w(820),
        height:      o.h(480),
        ..DendrogramConfig::default()
    });
    apply(html, &o)
}
