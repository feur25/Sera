pub mod arrows;
pub mod common;
pub mod config;
pub mod streamlines;
pub mod variant;

pub use config::VectorFieldMapConfig;
pub use variant::VectorFieldMapVariant;

use crate::plot::{apply, parse_all};

pub fn render_vector_field_map_html(cfg: &VectorFieldMapConfig) -> String {
    use VectorFieldMapVariant::*;
    match cfg.variant {
        Arrows => arrows::render(cfg),
        Streamlines => streamlines::render(cfg),
    }
}

#[crate::sera_alias(
    "vector_field_map",
    "vectorfieldmap",
    "vector_field_map_chart",
    "wind_map",
    "quiver_map",
    "geo_vector_field"
)]
#[crate::sera_builder]
pub fn build_vector_field_map(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let u = a.u.unwrap_or_default();
    let v = a.v.unwrap_or_default();
    let variant = VectorFieldMapVariant::from_str(o.variant.as_deref().unwrap_or("arrows"));
    let cfg = VectorFieldMapConfig {
        variant,
        title,
        lats: &lats,
        lons: &lons,
        u: &u,
        v: &v,
        width: o.w(1200),
        height: o.h(650),
        color_low: o.color_low.unwrap_or(0x38bdf8),
        color_high: o.color_high.unwrap_or(0xf97316),
    };
    apply(render_vector_field_map_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input(variant: &str) -> String {
        format!(
            r#"{{"title":"t","lats":[40.0,42.0,38.0],"lons":[-75.0,-73.0,-77.0],"u":[5.0,-3.0,2.0],"v":[2.0,4.0,-6.0],"variant":"{variant}"}}"#
        )
    }

    #[test]
    fn build_vector_field_map_arrows_draws_lines_and_arrowheads() {
        let out = build_vector_field_map(&sample_input("arrows"));
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("<polygon"), "arrows must draw arrowhead polygons: {out}");
        assert!(out.contains("<line"), "arrows must draw shaft lines: {out}");
    }

    #[test]
    fn build_vector_field_map_streamlines_draws_flowing_paths() {
        let out = build_vector_field_map(&sample_input("streamlines"));
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.matches("<path").count() > 5, "streamlines must draw many flowing paths: {out}");
    }

    #[test]
    fn build_vector_field_map_defaults_to_arrows_variant() {
        let out = build_vector_field_map(r#"{"title":"t","lats":[1.0,2.0],"lons":[3.0,4.0],"u":[5.0,6.0],"v":[7.0,8.0]}"#);
        assert!(out.contains("<polygon"), "no variant given must default to arrows: {out}");
    }

    #[test]
    fn build_vector_field_map_handles_empty_input_without_panicking() {
        let out = build_vector_field_map(r#"{"title":"t"}"#);
        assert!(out.is_empty() || out.contains("<svg"));
    }

    #[test]
    fn every_registered_chart_demo_for_vector_field_map_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/vector_field_map/") {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            assert!(html.contains("<svg"), "{} must render a real svg: {html}", entry.file);
        }
    }
}
