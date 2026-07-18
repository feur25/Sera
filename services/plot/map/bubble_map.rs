use super::world_data;
use crate::plot::default::PlotRenderContext;
use crate::plot::{apply, parse_all};

const PALETTE: &[(u8, u8, u8)] = &[
    (99, 102, 241),
    (244, 63, 94),
    (16, 185, 129),
    (245, 158, 11),
    (139, 92, 246),
    (6, 182, 212),
    (236, 72, 153),
    (132, 204, 22),
    (239, 68, 68),
    (20, 184, 166),
    (129, 140, 248),
    (251, 113, 133),
];

pub fn render_bubble_map_fast(
    values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
) -> String {
    let n = values.len().min(labels.len());
    if n == 0 {
        return String::new();
    }

    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let _max_val = max_val.max(1.0);

    let mut svg = String::with_capacity(n * 2000 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0d1117\"/>");

    for shape in world_data::all_countries() {
        let polys = world_data::normalized_polygons(shape);
        for poly in &polys {
            if poly.len() < 3 {
                continue;
            }
            svg.push_str("<path d=\"M");
            for (j, pt) in poly.iter().enumerate() {
                let px = pt[0] * width as f32;
                let py = pt[1] * height as f32;
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }

    for i in 0..n {
        if let Some(shape) = world_data::lookup_country(&labels[i]) {
            let (r, g, b) = PALETTE[i % PALETTE.len()];
            let polys = world_data::normalized_polygons(shape);
            for poly in &polys {
                if poly.len() < 3 {
                    continue;
                }
                svg.push_str("<path d=\"M");
                for (j, pt) in poly.iter().enumerate() {
                    let px = pt[0] * width as f32;
                    let py = pt[1] * height as f32;
                    if j > 0 {
                        svg.push_str(" L");
                    }
                    svg.push_str(&format!("{:.1},{:.1}", px, py));
                }
                svg.push_str(&format!(" Z\" fill=\"rgb({},{},{})\" fill-opacity=\"0.7\" stroke=\"rgb({},{},{})\" stroke-width=\"0.8\" data-index=\"{}\"/>", r, g, b, r, g, b, i));
            }

            let centroid = world_data::shape_centroid(shape);
            let cx = centroid[0] / 1009.6727 * width as f32;
            let cy = centroid[1] / 665.963 * height as f32;
            svg.push_str(&format!(
                "<text x=\"{:.0}\" y=\"{:.0}\" fill=\"white\" font-size=\"8\" text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>",
                cx, cy, shape.id
            ));
        }
    }

    svg.push_str("</svg>");
    svg
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

pub fn render_bubble_map_html(
    title: &str,
    labels: &[String],
    values: &[f64],
    width: i32,
    height: i32,
    hover: &[crate::html::hover::HoverSlot],
) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = values.len().min(labels.len());
    if n == 0 {
        return String::new();
    }
    let auto = hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto {
        Vec::with_capacity(n)
    } else {
        Vec::new()
    };
    if auto {
        for i in 0..n {
            auto_slots
                .push(HoverSlot::new(labels[i].clone()).kv("Valeur", format!("{:.2}", values[i])));
        }
    }
    let mut svg = render_bubble_map_fast(values, labels, width, height);
    svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { hover };
    build_chart_html(title, &svg, &slots_to_json(slots))
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
    let html = crate::plot::map::render_bubble_map_html(
        title,
        &labels,
        &values,
        o.w(1200),
        o.h(600),
        &hover,
    );
    apply(html, &o)
}
