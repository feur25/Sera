pub mod common;
pub mod config;
pub mod globe;
pub mod lines;
pub mod tissot;
pub mod variant;

pub use config::GraticuleMapConfig;
pub use variant::GraticuleMapVariant;

use crate::plot::{apply, parse_all};

pub fn render_graticule_map_html(cfg: &GraticuleMapConfig) -> String {
    use GraticuleMapVariant::*;
    match cfg.variant {
        Lines => lines::render(cfg),
        Globe => globe::render(cfg),
        Tissot => tissot::render(cfg),
    }
}

#[crate::sera_alias(
    "graticule_map",
    "graticulemap",
    "graticule_map_chart",
    "graticule",
    "meridian_map",
    "grid_map"
)]
#[crate::sera_builder]
pub fn build_graticule_map(input: &str) -> String {
    let (title_s, _a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = GraticuleMapVariant::from_str(o.variant.as_deref().unwrap_or("lines"));
    let cfg = GraticuleMapConfig {
        variant,
        title,
        width: o.w(1200),
        height: o.h(650),
        step: o.step.unwrap_or(15.0),
        center_lat: o.center_lat,
        center_lon: o.center_lon,
        color_low: o.color_low.unwrap_or(0x38bdf8),
        color_high: o.color_high.unwrap_or(0xf97316),
    };
    apply(render_graticule_map_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_graticule_map_lines_draws_meridians_and_parallels() {
        let out = build_graticule_map(r#"{"title":"t","step":30}"#);
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.matches("<line").count() > 5, "expected many graticule lines: {out}");
    }

    #[test]
    fn build_graticule_map_defaults_to_lines_variant() {
        let out = build_graticule_map(r#"{"title":"t"}"#);
        assert!(out.contains("<line"), "no variant given must default to lines: {out}");
    }

    #[test]
    fn build_graticule_map_globe_draws_a_disc_and_projected_paths() {
        let out = build_graticule_map(r#"{"title":"t","variant":"globe","step":30}"#);
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("<circle"), "globe variant must draw the sphere disc: {out}");
        assert!(out.matches("<path").count() > 5, "globe variant must draw projected graticule paths: {out}");
    }

    #[test]
    fn build_graticule_map_tissot_draws_distortion_indicatrices() {
        let out = build_graticule_map(r#"{"title":"t","variant":"tissot","step":30}"#);
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.matches("data-index=").count() > 3, "tissot must draw multiple indicatrix circles: {out}");
    }

    #[test]
    fn build_graticule_map_step_changes_line_density() {
        let coarse = build_graticule_map(r#"{"title":"t","step":45}"#);
        let fine = build_graticule_map(r#"{"title":"t","step":10}"#);
        assert!(fine.matches("<line").count() > coarse.matches("<line").count(), "a smaller step must draw more lines: fine={fine} coarse={coarse}");
    }

    #[test]
    fn graticule_map_still_accepts_grid_then_show_legend_after_variant_specific_rendering() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("graticule_map/lines.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("graticule_map lines demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        let html = crate::bindings::method_registry::apply_by_name(&html, "grid", "{}").expect("grid() must apply cleanly to a graticule map");
        let html = crate::bindings::method_registry::apply_by_name(&html, "show_legend", "{}").expect("show_legend() must apply cleanly after grid()");
        assert!(html.contains("<line"), "the graticule itself must survive both chained calls: {html}");
    }

    #[test]
    fn every_registered_chart_demo_for_graticule_map_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/graticule_map/") {
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
            if !path.contains("map/graticule_map/") {
                continue;
            }
            let stem = path.rsplit('/').next().unwrap().trim_end_matches(".rs");
            if stem == "mod" {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            std::fs::write(format!("docs/previews/graticule-map-{stem}.html"), &html).unwrap();
            if stem == GraticuleMapVariant::default_key() {
                std::fs::write("docs/previews/graticule-map.html", &html).unwrap();
            }
        }
    }
}
