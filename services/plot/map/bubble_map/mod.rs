pub mod common;
pub mod config;
pub mod filled;
pub mod proportional;
pub mod variant;

pub use config::BubbleMapConfig;
pub use variant::BubbleMapVariant;

use crate::plot::default::PlotRenderContext;
use crate::plot::map::{regions, world_data};
use crate::plot::{apply, parse_all};

const PALETTE: &[(u8, u8, u8)] = common::PALETTE;

pub fn render_bubble_map_html(cfg: &BubbleMapConfig) -> String {
    use BubbleMapVariant::*;
    match cfg.variant {
        Filled => filled::render(cfg),
        Proportional => proportional::render(cfg),
    }
}

pub fn render_bubble_map(ctx: PlotRenderContext) {
    let _n = ctx.visible_indices.len();

    ctx.painter
        .rect_filled(ctx.plot_rect, 0.0, egui::Color32::from_rgb(13, 17, 23));

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
        egui::Stroke::new(0.3, egui::Color32::from_rgba_premultiplied(42, 42, 74, 100));
    let base_fill = egui::Color32::from_rgb(21, 27, 35);

    for shape in world_data::all_countries() {
        let polys = world_data::normalized_polygons(shape);
        let entry = label_map
            .get(&shape.id)
            .or_else(|| label_map.get(&shape.name.to_uppercase()));

        let (fill, stroke, is_data) = if let Some(&(idx, _)) = entry {
            let is_hov = ctx.hovered_idx.map(|h| h == idx).unwrap_or(false);
            let pal = PALETTE[idx % PALETTE.len()];
            if is_hov {
                (
                    egui::Color32::from_rgb(255, 220, 50),
                    egui::Stroke::new(1.5, egui::Color32::WHITE),
                    true,
                )
            } else {
                (
                    egui::Color32::from_rgba_premultiplied(pal.0, pal.1, pal.2, 180),
                    egui::Stroke::new(0.8, egui::Color32::from_rgb(pal.0, pal.1, pal.2)),
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
            let mut path = egui::epaint::PathShape::closed_line(points, stroke);
            path.fill = fill;
            ctx.painter.add(egui::Shape::Path(path));
        }

        if is_data {
            if let Some(&(idx, _)) = entry {
                let centroid = world_data::shape_centroid(shape);
                let cx = ox + centroid[0] / 1009.6727 * w;
                let cy = oy + centroid[1] / 665.963 * h;
                let font = egui::FontId::proportional(9.0);

                ctx.painter.text(
                    egui::pos2(cx, cy),
                    egui::Align2::CENTER_CENTER,
                    &shape.id,
                    font.clone(),
                    egui::Color32::WHITE,
                );

                if ctx.hovered_idx == Some(idx) {
                    let font_big = egui::FontId::proportional(12.0);
                    let text = format!("{} ({})", shape.name, shape.id);
                    ctx.painter.text(
                        egui::pos2(cx, cy - 14.0),
                        egui::Align2::CENTER_BOTTOM,
                        &text,
                        font_big,
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }
}

pub fn render_svg_bubble_map(
    svg: &mut String,
    values: &[f64],
    colors: &[&'static str],
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
        let nx = (i % 20) as f64 / 20.0;
        let ny = (i / 20) as f64 / 10.0;
        let px = (nx * plot_width as f64) as i32;
        let py = (ny * plot_height as f64) as i32;
        let norm = (values[i] / max_val).clamp(0.0, 1.0);
        let radius = 3 + (norm.sqrt() * 18.0) as i32;
        let color = colors.get(i % colors.len()).unwrap_or(&"#4a90e2");

        svg.push_str("<circle cx=\"");
        svg.push_str(&px.to_string());
        svg.push_str("\" cy=\"");
        svg.push_str(&py.to_string());
        svg.push_str("\" r=\"");
        svg.push_str(&radius.to_string());
        svg.push_str("\" fill=\"");
        svg.push_str(color);
        svg.push_str("\" fill-opacity=\"0.6\" stroke=\"");
        svg.push_str(color);
        svg.push_str("\" stroke-width=\"1\" class=\"interactive-point\" data-index=\"");
        svg.push_str(&i.to_string());
        svg.push_str("\"/>");
    }
}

#[allow(dead_code)]
fn parse_label_coords(label: &str) -> (f64, f64) {
    if let Some((lat_str, lon_str)) = label.split_once(',') {
        if let (Ok(lat), Ok(lon)) = (lat_str.trim().parse::<f64>(), lon_str.trim().parse::<f64>()) {
            return (lat, lon);
        }
    }
    let centroid = crate::core::math::sub_region_centroid(label);
    if centroid != (0.0, 0.0) {
        return centroid;
    }
    crate::core::math::region_centroid(label)
}

#[crate::sera_alias(
    "bubble_map",
    "bubblemap",
    "bubble_map_chart",
    "geo_bubble",
    "geo_bubble_map"
)]
#[crate::sera_builder]
pub fn build_bubble_map(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let hover = o.hj();
    let region = regions::resolve(o.map.as_deref().unwrap_or(""))
        .or_else(regions::default_region_set)
        .expect("world region set must be registered");
    let variant = BubbleMapVariant::from_str(o.variant.as_deref().unwrap_or("filled"));
    let cfg = BubbleMapConfig {
        variant,
        title,
        labels: &labels,
        values: &values,
        width: o.w(1200),
        height: o.h(600),
        hover: &hover,
        region,
        group: o.region.as_deref().unwrap_or(""),
        min_bubble_size: o.min_size.unwrap_or(5.0),
        max_bubble_size: o.max_size.unwrap_or(42.0),
    };
    apply(render_bubble_map_html(&cfg), &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_bubble_map_defaults_to_world_and_still_matches_iso_country_codes() {
        let out = build_bubble_map(r#"{"title":"t","labels":["FRA","DEU"],"values":[1.0,2.0]}"#);
        assert!(out.contains("<svg"), "expected a real svg for the default world map: {out}");
    }

    #[test]
    fn build_bubble_map_switches_to_usa_states_via_the_map_option() {
        let out = build_bubble_map(r#"{"title":"t","labels":["CA","TX"],"values":[10.0,20.0],"map":"usa_states"}"#);
        assert!(out.contains("<svg"), "expected a real svg for the usa_states map: {out}");
    }

    #[test]
    fn build_bubble_map_switches_to_proportional_variant_and_draws_real_circles() {
        let out = build_bubble_map(r#"{"title":"t","labels":["CA","TX"],"values":[10.0,20.0],"map":"usa_states","variant":"proportional"}"#);
        assert!(out.contains("<circle"), "proportional bubble_map must draw real circle bubbles: {out}");
    }

    #[test]
    fn build_bubble_map_restricts_rendering_to_one_named_region_group() {
        let all = build_bubble_map(r#"{"title":"t","labels":["CA","NY"],"values":[1.0,2.0],"map":"usa_states"}"#);
        let west_only = build_bubble_map(r#"{"title":"t","labels":["CA","NY"],"values":[1.0,2.0],"map":"usa_states","region":"West"}"#);
        let path_count = |s: &str| s.matches("<path").count();
        assert!(path_count(&west_only) < path_count(&all), "restricting to one census region must draw fewer outline paths");
    }

    #[test]
    fn every_registered_chart_demo_for_bubble_map_renders_non_empty_html() {
        for entry in crate::plot::chart_demo_registry::iter_entries() {
            if !entry.file.replace('\\', "/").contains("map/bubble_map/") {
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
            if !path.contains("map/bubble_map/") {
                continue;
            }
            let stem = path.rsplit('/').next().unwrap().trim_end_matches(".rs");
            if stem == "mod" {
                continue;
            }
            let html = crate::plot::chart_demo_registry::render_demo_html(entry).expect("demo html");
            std::fs::write(format!("docs/previews/bubble-map-{stem}.html"), html).unwrap();
        }
    }
}
