pub mod binned;
pub mod bivariate;
pub mod common;
pub mod config;
pub mod daynight;
pub mod diverging;
pub mod dot_density;
pub mod orthographic;
pub mod polar;
pub mod sequential;
pub mod variant;

pub use config::ChoroplethConfig;
pub use variant::ChoroplethVariant;

use crate::plot::default::PlotRenderContext;
use crate::plot::map::{regions, world_data};
use crate::plot::{apply, parse_all};

pub fn render_choropleth_html(cfg: &ChoroplethConfig) -> String {
    use ChoroplethVariant::*;
    match cfg.variant {
        Sequential => sequential::render(cfg),
        Binned => binned::render(cfg),
        Diverging => diverging::render(cfg),
        Orthographic => orthographic::render(cfg),
        Polar => polar::render(cfg),
        Bivariate => bivariate::render(cfg),
        DotDensity => dot_density::render(cfg),
        DayNight => daynight::render(cfg),
    }
}

pub fn render_choropleth(ctx: PlotRenderContext) {
    let _n = ctx.visible_indices.len();

    ctx.painter
        .rect_filled(ctx.plot_rect, 0.0, egui::Color32::from_rgb(15, 15, 30));

    let max_val = ctx.max_val.max(1.0);
    let w = ctx.plot_rect.width();
    let h = ctx.plot_rect.height();
    let ox = ctx.plot_rect.left();
    let oy = ctx.plot_rect.top();

    let mut label_map: std::collections::HashMap<String, (usize, f64)> =
        std::collections::HashMap::new();
    for &actual_idx in ctx.visible_indices.iter() {
        if actual_idx >= ctx.labels.len() {
            continue;
        }
        let key = ctx.labels[actual_idx].to_uppercase();
        label_map.insert(key, (actual_idx, ctx.values[actual_idx]));
    }

    let border_stroke =
        egui::Stroke::new(0.4_f32, egui::Color32::from_rgba_premultiplied(50, 50, 80, 100));
    let base_fill = egui::Color32::from_rgb(26, 26, 46);

    for shape in world_data::all_countries() {
        let polys = world_data::normalized_polygons(shape);
        let entry = label_map
            .get(&shape.id)
            .or_else(|| label_map.get(&shape.name.to_uppercase()));

        let (fill, stroke, is_data) = if let Some(&(idx, value)) = entry {
            let is_hov = ctx.hovered_idx.map(|h| h == idx).unwrap_or(false);
            if is_hov {
                (
                    egui::Color32::from_rgb(255, 220, 50),
                    egui::Stroke::new(1.5_f32, egui::Color32::WHITE),
                    true,
                )
            } else {
                let (r, g, b) = crate::core::math::heat_color(value, max_val);
                (
                    egui::Color32::from_rgb(r, g, b),
                    egui::Stroke::new(
                        0.6_f32,
                        egui::Color32::from_rgba_premultiplied(255, 255, 255, 60),
                    ),
                    true,
                )
            }
        } else {
            (base_fill, border_stroke, false)
        };

        for poly in &polys {
            if poly.len() < 3 {
                continue;
            }
            let points: Vec<egui::Pos2> = poly
                .iter()
                .map(|pt| egui::pos2(ox + pt[0] * w, oy + pt[1] * h))
                .collect();
            let path = egui::epaint::PathShape::closed_line(points.clone(), stroke);
            let mut path = path;
            path.fill = fill;
            ctx.painter.add(egui::Shape::Path(path));
        }

        if is_data {
            if let Some(&(idx, value)) = entry {
                let centroid = world_data::shape_centroid(shape);
                let cx = ox + centroid[0] / 1009.6727 * w;
                let cy = oy + centroid[1] / 665.963 * h;

                if ctx.hovered_idx == Some(idx) {
                    let font = egui::FontId::proportional(11.0);
                    let text = format!("{}: {:.0}", shape.name, value);
                    ctx.painter.text(
                        egui::pos2(cx, cy - 12.0),
                        egui::Align2::CENTER_BOTTOM,
                        &text,
                        font,
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }
}

pub fn render_svg_choropleth(
    svg: &mut String,
    values: &[f64],
    _colors: &[&'static str],
    _pad: i32,
    plot_width: i32,
    plot_height: i32,
    max_val: f64,
    _vertical: bool,
) {
    let n = values.len();
    if n == 0 {
        return;
    }
    let max_val = max_val.max(1.0);

    for i in 0..n {
        let (r, g, b) = crate::core::math::heat_color(values[i], max_val);
        let radius = 4 + ((values[i] / max_val) * 8.0) as i32;
        let px = ((i % 20) as i32) * (plot_width / 20);
        let py = ((i / 20) as i32) * (plot_height / 10);

        svg.push_str("<circle cx=\"");
        svg.push_str(&px.to_string());
        svg.push_str("\" cy=\"");
        svg.push_str(&py.to_string());
        svg.push_str("\" r=\"");
        svg.push_str(&radius.to_string());
        svg.push_str("\" fill=\"rgb(");
        svg.push_str(&r.to_string());
        svg.push(',');
        svg.push_str(&g.to_string());
        svg.push(',');
        svg.push_str(&b.to_string());
        svg.push_str(")\" fill-opacity=\"0.85\" stroke=\"white\" stroke-width=\"0.5\" class=\"interactive-point\" data-index=\"");
        svg.push_str(&i.to_string());
        svg.push_str("\"/>");
    }
}

#[crate::sera_alias(
    "choropleth",
    "choropleths",
    "choropleth_map",
    "choropleth_chart",
    "geo_map"
)]
#[crate::sera_builder]
pub fn build_choropleth(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let secondary_values = o.secondary_values.clone().unwrap_or_default();
    let hover = o.hj();
    let region = regions::resolve(o.map.as_deref().unwrap_or(""))
        .or_else(regions::default_region_set)
        .expect("world region set must be registered");
    let variant = ChoroplethVariant::from_str(o.variant.as_deref().unwrap_or("sequential"));
    let cfg = ChoroplethConfig {
        variant,
        title,
        labels: &labels,
        values: &values,
        secondary_values: &secondary_values,
        width: o.w(1200),
        height: o.h(600),
        hover: &hover,
        region,
        group: o.region.as_deref().unwrap_or(""),
        bins: o.bins.map(|b| b as usize).unwrap_or(5),
        diverging_midpoint: o.diverging_midpoint.unwrap_or(0.0),
        center_lat: o.center_lat,
        center_lon: o.center_lon,
    };
    apply(render_choropleth_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_choropleth_defaults_to_world_and_still_matches_iso_country_codes() {
        let out = build_choropleth(r#"{"title":"t","labels":["FRA","DEU"],"values":[1.0,2.0]}"#);
        assert!(out.contains("<svg"), "expected a real svg for the default world map: {out}");
    }

    #[test]
    fn build_choropleth_switches_to_usa_states_via_the_map_option() {
        let out = build_choropleth(r#"{"title":"t","labels":["CA","TX"],"values":[10.0,20.0],"map":"usa_states"}"#);
        assert!(out.contains("<svg"), "expected a real svg for the usa_states map: {out}");
    }

    #[test]
    fn build_choropleth_renders_every_newly_registered_country_region_set() {
        for region in crate::plot::map::regions::all_region_sets() {
            let map = region.key;
            let shapes = (region.all)();
            let n = shapes.len().min(3);
            let labels: Vec<&str> = shapes[..n].iter().map(|s| s.id.as_str()).collect();
            let input = format!(
                r#"{{"title":"t","labels":{:?},"values":[1.0,2.0,3.0],"map":"{map}"}}"#,
                labels
            );
            let out = build_choropleth(&input);
            assert!(out.contains("<svg"), "{map} must render a real svg: {out}");
            for i in 0..n {
                let needle = format!("data-idx=\"{i}\"");
                assert!(out.contains(&needle), "{map} must color region index {i}: {out}");
            }
        }
    }

    #[test]
    fn build_choropleth_falls_back_to_world_for_an_unknown_map_key() {
        let world = build_choropleth(r#"{"title":"t","labels":["FRA"],"values":[1.0]}"#);
        let unknown = build_choropleth(r#"{"title":"t","labels":["FRA"],"values":[1.0],"map":"atlantis"}"#);
        let path_count = |s: &str| s.matches("<path").count();
        assert_eq!(path_count(&world), path_count(&unknown), "an unrecognized map key must silently fall back to world, not fail: got a different number of drawn paths");
        assert!(unknown.contains("<svg"), "unknown map key must still render a real svg: {unknown}");
    }

    #[test]
    fn build_choropleth_switches_variant_via_the_variant_option() {
        let out = build_choropleth(r#"{"title":"t","labels":["CA","TX","NY"],"values":[10.0,20.0,30.0],"map":"usa_states","variant":"binned"}"#);
        assert!(out.contains("<svg"), "expected a real svg for the binned variant: {out}");
    }

    #[test]
    fn build_choropleth_restricts_rendering_to_one_named_region_group() {
        let all = build_choropleth(r#"{"title":"t","labels":["CA","NY"],"values":[1.0,2.0],"map":"usa_states"}"#);
        let west_only = build_choropleth(r#"{"title":"t","labels":["CA","NY"],"values":[1.0,2.0],"map":"usa_states","region":"West"}"#);
        assert!(west_only.len() < all.len(), "restricting to one census region must draw fewer outline paths than the full map: all={} west={}", all.len(), west_only.len());
        assert!(west_only.contains("data-idx=\"0\""), "California (in West) must still be colored: {west_only}");
    }

    #[test]
    fn build_choropleth_stays_fast_with_every_world_country_populated() {
        let labels: Vec<String> = world_data::all_countries().iter().map(|c| c.id.clone()).collect();
        let values: Vec<f64> = (0..labels.len()).map(|i| ((i as f64 * 37.0) % 100.0) + 1.0).collect();
        let input = format!(r#"{{"title":"t","labels":{:?},"values":{:?}}}"#, labels, values);
        let start = std::time::Instant::now();
        let out = build_choropleth(&input);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 800, "choropleth over the full world country set took too long: {elapsed:?}");
        assert!(out.contains("<svg"), "expected a real svg for the full world country set: {out}");
    }

    #[test]
    fn choropleth_still_accepts_grid_then_show_legend_after_variant_specific_rendering() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("choropleth/sequential.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("choropleth sequential demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        let html = crate::bindings::method_registry::apply_by_name(&html, "grid", "{}").expect("grid() must apply cleanly to a choropleth");
        let html = crate::bindings::method_registry::apply_by_name(&html, "show_legend", "{}").expect("show_legend() must apply cleanly after grid()");
        assert!(html.contains("<svg"), "the map itself must survive both chained calls: {html}");
        assert!(html.contains("g[data-legend],g.sp-leg-grp{display:block"), "show_legend()'s forced-visibility rule must survive chaining after grid(): {html}");
    }

    #[test]
    fn every_registered_chart_demo_for_choropleth_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/choropleth/") {
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
            if !path.contains("map/choropleth/") {
                continue;
            }
            let stem = path.rsplit('/').next().unwrap().trim_end_matches(".rs");
            if stem == "mod" {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            std::fs::write(format!("docs/previews/choropleth-{stem}.html"), &html).unwrap();
            if stem == ChoroplethVariant::default_key() {
                std::fs::write("docs/previews/choropleth.html", &html).unwrap();
            }
        }
    }

    #[test]
    #[ignore]
    fn write_new_region_set_visual_checks() {
        let out_dir = std::env::var("SP_VISUAL_CHECK_DIR").unwrap_or_else(|_| ".".to_string());
        for region in crate::plot::map::regions::all_region_sets() {
            let map = region.key;
            let shapes = (region.all)();
            let labels: Vec<String> = shapes.iter().map(|s| s.id.clone()).collect();
            let values: Vec<f64> = (0..labels.len()).map(|i| i as f64).collect();
            let input = format!(
                r#"{{"title":"{map}","labels":{:?},"values":{:?},"map":"{map}","variant":"binned"}}"#,
                labels, values
            );
            let out = build_choropleth(&input);
            assert!(out.contains("<svg"), "{map} must render a real svg");
            std::fs::write(format!("{out_dir}/visual_{map}.html"), out).unwrap();
        }
    }

    #[test]
    fn build_choropleth_daynight_draws_the_terminator_and_a_sun_marker() {
        let out = build_choropleth(r#"{"title":"t","variant":"daynight","center_lat":15.0,"center_lon":-40.0}"#);
        assert!(out.contains("<svg"), "expected a real svg: {out}");
        assert!(out.contains("fill-opacity=\"0.62\""), "must draw the night-side shading: {out}");
        assert!(out.contains("#facc15"), "must draw a sun marker at the subsolar point: {out}");
    }

    #[test]
    fn build_choropleth_daynight_works_with_no_center_given() {
        let out = build_choropleth(r#"{"title":"t","variant":"daynight"}"#);
        assert!(out.contains("<svg"), "must default center_lat/lon rather than panic: {out}");
    }

    #[test]
    fn every_registered_chart_demo_for_choropleth_daynight_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("choropleth/daynight.rs") {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            assert!(html.contains("<svg"), "{} must render a real svg: {html}", entry.file);
        }
    }
}
