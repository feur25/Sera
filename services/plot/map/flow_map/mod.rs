pub mod arc;
pub mod common;
pub mod config;
pub mod straight;
pub mod variant;

pub use config::FlowMapConfig;
pub use variant::FlowMapVariant;

use crate::plot::map::regions;
use crate::plot::{apply, parse_all};

pub fn render_flow_map_html(cfg: &FlowMapConfig) -> String {
    use FlowMapVariant::*;
    match cfg.variant {
        Arc => arc::render(cfg),
        Straight => straight::render(cfg),
    }
}

#[crate::sera_alias("flow_map", "flowmap", "flow_map_chart", "geo_flow", "connection_map", "great_circle_map")]
#[crate::sera_builder]
pub fn build_flow_map(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_default();
    let hover = o.hj();
    let region = regions::resolve(o.map.as_deref().unwrap_or(""))
        .or_else(regions::default_region_set)
        .expect("world region set must be registered");
    let variant = FlowMapVariant::from_str(o.variant.as_deref().unwrap_or("arc"));
    let cfg = FlowMapConfig {
        variant,
        title,
        labels: &labels,
        sources: &sources,
        targets: &targets,
        weights: &weights,
        width: o.w(1200),
        height: o.h(600),
        hover: &hover,
        region,
        group: o.region.as_deref().unwrap_or(""),
        min_width: o.min_size.unwrap_or(1.0),
        max_width: o.max_size.unwrap_or(7.0),
    };
    apply(render_flow_map_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_flow_map_renders_a_real_svg_with_default_arc_variant() {
        let out = build_flow_map(r#"{"title":"t","labels":["FR","DE","GB"],"edges_i":[0,1],"edges_j":[1,2],"edges_w":[10.0,5.0]}"#);
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("<path d=\"M"), "arc variant must draw at least one flow path: {out}");
    }

    #[test]
    fn build_flow_map_switches_to_straight_variant() {
        let out = build_flow_map(r#"{"title":"t","labels":["FR","DE"],"edges_i":[0],"edges_j":[1],"edges_w":[10.0],"variant":"straight"}"#);
        assert!(out.contains("<svg"), "expected a real svg: {out}");
    }

    #[test]
    fn build_flow_map_ignores_edges_referencing_an_unknown_label() {
        let out = build_flow_map(r#"{"title":"t","labels":["FR","ATLANTIS"],"edges_i":[0],"edges_j":[1],"edges_w":[10.0]}"#);
        assert!(out.is_empty() || out.contains("<svg"), "must not panic on an unresolvable edge: {out}");
    }

    #[test]
    fn build_flow_map_switches_to_usa_states_via_the_map_option() {
        let out = build_flow_map(r#"{"title":"t","labels":["CA","TX","NY"],"edges_i":[0,1],"edges_j":[1,2],"edges_w":[5.0,3.0],"map":"usa_states"}"#);
        assert!(out.contains("<svg"), "expected a real svg for usa_states: {out}");
    }

    #[test]
    fn every_registered_chart_demo_for_flow_map_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/flow_map/") {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            assert!(html.contains("<svg"), "{} must render a real svg: {html}", entry.file);
        }
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            let path = entry.file.replace('\\', "/");
            if !path.contains("map/flow_map/") {
                continue;
            }
            let stem = path.rsplit('/').next().unwrap().trim_end_matches(".rs");
            if stem == "mod" {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            std::fs::write(format!("docs/previews/flow-map-{stem}.html"), &html).unwrap();
            if stem == FlowMapVariant::default_key() {
                std::fs::write("docs/previews/flow-map.html", &html).unwrap();
            }
        }
    }
}
