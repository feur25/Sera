use crate::plot::{apply, parse_all};

pub mod basic;
pub mod config;
pub mod variant;

pub use config::ArcDiagramConfig;
pub use variant::ArcDiagramVariant;

pub fn render_arc_diagram_html(cfg: &ArcDiagramConfig) -> String {
    use ArcDiagramVariant::*;
    match cfg.variant {
        Basic     => basic::render(cfg),
        Bilateral => basic::render_bilateral(cfg),
        Weighted  => basic::render_weighted(cfg),
        Minimal   => basic::render_minimal(cfg),
        Directed  => basic::render_directed(cfg),
    }
}

pub use build as build_arc_diagram;

#[crate::sera_alias("arc_diagram", "arc_chart", "arc_plot", "arc_graph", "linear_network")]
#[crate::sera_builder("build_arc_diagram")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title   = title_s.as_str();
    let labels  = a.labels.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_else(|| vec![1.0; sources.len()]);
    let hover   = o.hj();
    let variant = ArcDiagramVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();

    let html = render_arc_diagram_html(&ArcDiagramConfig {
        variant,
        title,
        labels:  &labels,
        sources: &sources,
        targets: &targets,
        weights: &weights,
        palette: &palette,
        hover:   &hover,
        width:   o.w(900),
        height:  o.h(400),
        ..ArcDiagramConfig::default()
    });
    apply(html, &o)
}
