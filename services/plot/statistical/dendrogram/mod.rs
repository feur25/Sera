use crate::plot::{apply, parse_all};

pub mod common;
pub mod compact;
pub mod config;
pub mod elegant;
pub mod horizontal;
pub mod radial;
pub mod triangular;
pub mod variant;
pub mod vertical;

pub use config::DendrogramConfig;
pub use variant::DendrogramVariant;

pub fn render_dendrogram_html(cfg: &DendrogramConfig) -> String {
    use DendrogramVariant::*;
    match cfg.variant {
        Vertical   => vertical::render(cfg),
        Horizontal => horizontal::render(cfg),
        Radial     => radial::render(cfg),
        Compact    => compact::render(cfg),
        Elegant    => elegant::render(cfg),
        Triangular => triangular::render(cfg),
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
    let values  = a.matrix.unwrap_or_default();
    let clusters = o.k.unwrap_or(3);
    let palette = o.pal();
    let variant = DendrogramVariant::from_str(o.variant.as_deref().unwrap_or("vertical"));
    let hover   = o.hj();

    let html = render_dendrogram_html(&DendrogramConfig {
        variant,
        title,
        labels:      &labels,
        parents:     &parents,
        values:      &values,
        clusters,
        palette:     &palette,
        hover:       &hover,
        width:       o.w(820),
        height:      o.h(480),
        ..DendrogramConfig::default()
    });
    apply(html, &o)
}
