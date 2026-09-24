pub mod common;
pub mod config;
pub mod extrema;
pub mod filled;
pub mod isolines;
pub mod layout3d;
pub mod variant;

pub use config::ContourMapConfig;
pub use variant::ContourMapVariant;

use crate::plot::{apply, parse_all};

pub fn render_contour_map_html(cfg: &ContourMapConfig) -> String {
    use ContourMapVariant::*;
    match cfg.variant {
        Filled => filled::render(cfg),
        Isolines => isolines::render(cfg),
        Extrema => extrema::render(cfg),
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
    fn build_contour_map_extrema_labels_at_least_one_high_and_one_low() {
        let out = build_contour_map(
            r#"{"title":"t","variant":"extrema","lats":[65,55,35,55,32,-28,-28,-30,55,2,2,0,-57,-57,-57,40,50,35,-25,40,-5,55],"lons":[-20,-165,-25,90,-140,-105,-5,70,-100,20,-60,110,-60,90,170,15,-35,140,135,-95,25,-10],"field":[-12.0,-10.0,11.0,15.0,9.0,10.0,8.0,9.0,6.0,-3.0,-4.0,-5.0,-9.0,-8.0,-7.0,3.0,-2.0,2.0,7.0,1.0,-2.0,-4.0]}"#,
        );
        assert!(out.contains(">H<"), "expected at least one high pressure label: {out}");
        assert!(out.contains(">L<"), "expected at least one low pressure label: {out}");
    }

    #[test]
    fn build_contour_map_extrema_handles_empty_input_without_panicking() {
        let out = build_contour_map(r#"{"title":"t","variant":"extrema"}"#);
        assert!(out.is_empty() || out.contains("<svg"));
    }

    #[test]
    fn every_registered_chart_demo_for_contour_map_extrema_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("contour_map/extrema.rs") {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            assert!(html.contains("<svg"), "{} must render a real svg: {html}", entry.file);
        }
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

    #[test]
    #[ignore]
    fn write_preview_assets() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            let path = entry.file.replace('\\', "/");
            if !path.contains("map/contour_map/") {
                continue;
            }
            let stem = path.rsplit('/').next().unwrap().trim_end_matches(".rs");
            if stem == "mod" {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            std::fs::write(format!("docs/previews/contour-map-{stem}.html"), &html).unwrap();
            if stem == ContourMapVariant::default_key() {
                std::fs::write("docs/previews/contour-map.html", &html).unwrap();
            }
        }
    }
}
