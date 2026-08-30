pub mod animated;
pub mod arc;
pub mod common;
pub mod config;
pub mod ribbon;
pub mod straight;
pub mod variant;

pub use config::FlowMapConfig;
pub use variant::FlowMapVariant;

use crate::plot::map::regions;
use crate::plot::{apply, parse_all};
#[cfg(test)]
use crate::plot::map::world_data;

pub fn render_flow_map_html(cfg: &FlowMapConfig) -> String {
    use FlowMapVariant::*;
    match cfg.variant {
        Arc => arc::render(cfg),
        Straight => straight::render(cfg),
        Animated => animated::render(cfg),
        Ribbon => ribbon::render(cfg),
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
    fn build_flow_map_stays_fast_with_hundreds_of_edges() {
        let labels: Vec<String> = world_data::all_countries().iter().map(|c| c.id.clone()).collect();
        let n = labels.len();
        let edge_count = 500usize.min(n * n / 4).max(1);
        let mut edges_i = Vec::with_capacity(edge_count);
        let mut edges_j = Vec::with_capacity(edge_count);
        let mut edges_w = Vec::with_capacity(edge_count);
        for k in 0..edge_count {
            edges_i.push((k * 7) % n);
            edges_j.push((k * 13 + 3) % n);
            edges_w.push(((k % 50) + 1) as f64);
        }
        let input = format!(
            r#"{{"title":"t","labels":{:?},"edges_i":{:?},"edges_j":{:?},"edges_w":{:?}}}"#,
            labels, edges_i, edges_j, edges_w
        );
        let start = std::time::Instant::now();
        let out = build_flow_map(&input);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 800, "flow map with {edge_count} edges took too long: {elapsed:?}");
        assert!(out.contains("<svg"), "expected a real svg for {edge_count} edges: {out}");
    }

    #[test]
    fn flow_map_still_accepts_grid_then_show_legend_after_variant_specific_rendering() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("flow_map/arc.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("flow_map arc demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        let html = crate::bindings::method_registry::apply_by_name(&html, "grid", "{}").expect("grid() must apply cleanly to a flow map");
        let html = crate::bindings::method_registry::apply_by_name(&html, "show_legend", "{}").expect("show_legend() must apply cleanly after grid()");
        assert!(html.contains("<svg"), "the map itself must survive both chained calls: {html}");
        assert!(html.contains("g[data-legend],g.sp-leg-grp{display:block"), "show_legend()'s forced-visibility rule must survive chaining after grid(): {html}");
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
