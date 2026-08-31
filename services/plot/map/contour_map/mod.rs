pub mod common;
pub mod config;
pub mod filled;
pub mod isolines;
pub mod variant;

pub use config::ContourMapConfig;
pub use variant::ContourMapVariant;

use crate::plot::{apply, parse_all};

pub fn render_contour_map_html(cfg: &ContourMapConfig) -> String {
    use ContourMapVariant::*;
    match cfg.variant {
        Filled => filled::render(cfg),
        Isolines => isolines::render(cfg),
    }
}

#[crate::sera_alias(
    "contour_map",
    "contourmap",
    "contour_map_chart",
    "geo_contour",
    "isarithmic_map",
    "scalar_field_map"
)]
#[crate::sera_builder]
pub fn build_contour_map(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let lats = a.lats.unwrap_or_default();
    let lons = a.lons.unwrap_or_default();
    let field = a.field.clone().or(a.values.clone()).unwrap_or_default();
    let variant = ContourMapVariant::from_str(o.variant.as_deref().unwrap_or("filled"));
    let cfg = ContourMapConfig {
        variant,
        title,
        lats: &lats,
        lons: &lons,
        field: &field,
        width: o.w(1200),
        height: o.h(650),
        levels: o.bins.unwrap_or(6).max(2) as usize,
        color_low: o.color_low.unwrap_or(0x1e3a8a),
        color_high: o.color_high.unwrap_or(0xdc2626),
    };
    apply(render_contour_map_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input(variant: &str) -> String {
        format!(
            r#"{{"title":"t","lats":[40.0,42.0,38.0],"lons":[-75.0,-73.0,-77.0],"field":[10.0,20.0,15.0],"variant":"{variant}"}}"#
        )
    }

    #[test]
    fn build_contour_map_filled_draws_a_real_svg_with_grid_cells() {
        let out = build_contour_map(&sample_input("filled"));
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.matches("<rect").count() > 10, "expected many interpolated grid cells: {out}");
    }

    #[test]
    fn build_contour_map_isolines_draws_stroked_paths_not_filled_cells() {
        let out = build_contour_map(&sample_input("isolines"));
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("fill=\"none\" stroke="), "isolines must draw stroked contour paths: {out}");
    }

    #[test]
    fn build_contour_map_defaults_to_filled_variant() {
        let out = build_contour_map(r#"{"title":"t","lats":[1.0,2.0],"lons":[3.0,4.0],"field":[5.0,6.0]}"#);
        assert!(out.matches("<rect").count() > 10, "no variant given must default to filled: {out}");
    }

    #[test]
    fn build_contour_map_handles_empty_input_without_panicking() {
        let out = build_contour_map(r#"{"title":"t"}"#);
        assert!(out.is_empty() || out.contains("<svg"));
    }

    #[test]
    fn every_registered_chart_demo_for_contour_map_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/contour_map/") {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            assert!(html.contains("<svg"), "{} must render a real svg: {html}", entry.file);
        }
    }
}
