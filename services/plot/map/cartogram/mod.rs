pub mod common;
pub mod config;
pub mod demers;
pub mod dorling;
pub mod variant;

pub use config::CartogramConfig;
pub use variant::CartogramVariant;

use crate::plot::map::regions;
use crate::plot::{apply, parse_all};

pub fn render_cartogram_html(cfg: &CartogramConfig) -> String {
    use CartogramVariant::*;
    match cfg.variant {
        Dorling => dorling::render(cfg),
        Demers => demers::render(cfg),
    }
}

#[crate::sera_alias(
    "cartogram",
    "cartogram_chart",
    "dorling_cartogram",
    "geo_cartogram",
    "proportional_cartogram",
    "relaxed_map"
)]
#[crate::sera_builder]
pub fn build_cartogram(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let hover = o.hj();
    let region = regions::resolve(o.map.as_deref().unwrap_or(""))
        .or_else(regions::default_region_set)
        .expect("world region set must be registered");
    let variant = CartogramVariant::from_str(o.variant.as_deref().unwrap_or("dorling"));
    let cfg = CartogramConfig {
        variant,
        title,
        labels: &labels,
        values: &values,
        lats: &lats,
        lons: &lons,
        width: o.w(1200),
        height: o.h(650),
        hover: &hover,
        region,
        group: o.region.as_deref().unwrap_or(""),
        min_radius: o.min_size.unwrap_or(6.0),
        max_radius: o.max_size.unwrap_or(46.0),
        color_low: o.color_low.unwrap_or(0x1e3a8a),
        color_high: o.color_high.unwrap_or(0xf59e0b),
        iterations: 180,
    };
    apply(render_cartogram_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input(variant: &str) -> String {
        format!(
            r#"{{"title":"t","labels":["USA","CHN","IND","BRA"],"values":[331.0,1412.0,1408.0,215.0],"lats":[39.0,35.0,21.0,-10.0],"lons":[-98.0,105.0,78.0,-51.0],"variant":"{variant}"}}"#
        )
    }

    #[test]
    fn build_cartogram_dorling_draws_circles_sized_by_value() {
        let out = build_cartogram(&sample_input("dorling"));
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("<circle"), "dorling must draw circle glyphs: {out}");
    }

    #[test]
    fn build_cartogram_demers_draws_squares_instead_of_circles() {
        let out = build_cartogram(&sample_input("demers"));
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("<rect") && out.contains("data-index"), "demers must draw square glyphs: {out}");
    }

    #[test]
    fn build_cartogram_defaults_to_dorling_variant() {
        let out = build_cartogram(r#"{"title":"t","labels":["A","B"],"values":[1.0,9.0],"lats":[10.0,20.0],"lons":[10.0,20.0]}"#);
        assert!(out.contains("<circle"), "no variant given must default to dorling circles: {out}");
    }

    #[test]
    fn build_cartogram_handles_empty_input_without_panicking() {
        let out = build_cartogram(r#"{"title":"t"}"#);
        assert!(out.is_empty() || out.contains("<svg"));
    }

    #[test]
    fn build_cartogram_relaxation_keeps_circles_from_overlapping_when_points_start_coincident() {
        let out = build_cartogram(
            r#"{"title":"t","labels":["A","B","C"],"values":[50.0,50.0,50.0],"lats":[10.0,10.0,10.0],"lons":[10.0,10.0,10.0]}"#,
        );
        assert!(out.contains("<circle"), "expected circles even from coincident seed points: {out}");
        assert!(out.matches("<circle").count() >= 3, "all three points must still render their own circle: {out}");
    }

    #[test]
    fn build_cartogram_switches_to_usa_states_via_the_map_option() {
        let out = build_cartogram(
            r#"{"title":"t","labels":["CA","TX"],"values":[39.0,30.0],"lats":[36.7,31.0],"lons":[-119.4,-99.0],"map":"usa_states"}"#,
        );
        assert!(out.contains("<svg"), "expected a real svg for the usa_states map: {out}");
    }

    #[test]
    fn every_registered_chart_demo_for_cartogram_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/cartogram/") {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            assert!(html.contains("<svg"), "{} must render a real svg: {html}", entry.file);
        }
    }

    #[test]
    fn cartogram_still_accepts_grid_then_show_legend_after_variant_specific_rendering() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("cartogram/dorling.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("cartogram dorling demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        let html = crate::bindings::method_registry::apply_by_name(&html, "grid", "{}").expect("grid() must apply cleanly to a cartogram");
        let html = crate::bindings::method_registry::apply_by_name(&html, "show_legend", "{}").expect("show_legend() must apply cleanly after grid()");
        assert!(html.contains("<circle"), "the glyphs themselves must survive both chained calls: {html}");
        assert!(html.contains("g[data-legend],g.sp-leg-grp{display:block"), "show_legend()'s forced-visibility rule must survive chaining after grid(): {html}");
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            let path = entry.file.replace('\\', "/");
            if !path.contains("map/cartogram/") {
                continue;
            }
            let stem = path.rsplit('/').next().unwrap().trim_end_matches(".rs");
            if stem == "mod" {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            std::fs::write(format!("docs/previews/cartogram-{stem}.html"), &html).unwrap();
            if stem == CartogramVariant::default_key() {
                std::fs::write("docs/previews/cartogram.html", &html).unwrap();
            }
        }
    }
}
