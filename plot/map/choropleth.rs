use crate::plot::default::PlotRenderContext;
use crate::core::math::heat_color;
use super::world_data;

pub fn render_choropleth_fast(
    values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
) -> String {
    let n = values.len().min(labels.len());
    if n == 0 { return String::new(); }

    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let max_val = max_val.max(1.0);

    let mut svg = String::with_capacity(n * 2000 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0f0f1e\"/>");

    render_svg_country_outlines(&mut svg, width, height);

    for i in 0..n {
        if let Some(shape) = world_data::lookup_country(&labels[i]) {
            let (r, g, b) = heat_color(values[i], max_val);
            let polys = world_data::normalized_polygons(shape);
            for poly in &polys {
                svg.push_str("<path d=\"M");
                for (j, pt) in poly.iter().enumerate() {
                    let px = pt[0] * width as f32;
                    let py = pt[1] * height as f32;
                    if j > 0 { svg.push_str(" L"); }
                    svg.push_str(&format!("{:.1},{:.1}", px, py));
                }
                svg.push_str(" Z\" fill=\"rgb(");
                svg.push_str(&r.to_string());
                svg.push(',');
                svg.push_str(&g.to_string());
                svg.push(',');
                svg.push_str(&b.to_string());
                svg.push_str(")\" fill-opacity=\"0.85\" stroke=\"rgba(255,255,255,0.3)\" stroke-width=\"0.5\" data-index=\"");
                svg.push_str(&i.to_string());
                svg.push_str("\"/>");
            }
        }
    }

    svg.push_str("</svg>");
    svg
}

fn render_svg_country_outlines(svg: &mut String, width: i32, height: i32) {
    for shape in world_data::all_countries() {
        let polys = world_data::normalized_polygons(shape);
        for poly in &polys {
            if poly.len() < 3 { continue; }
            svg.push_str("<path d=\"M");
            for (j, pt) in poly.iter().enumerate() {
                let px = pt[0] * width as f32;
                let py = pt[1] * height as f32;
                if j > 0 { svg.push_str(" L"); }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(" Z\" fill=\"#1a1a2e\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }
}

pub fn render_choropleth(ctx: PlotRenderContext) {
    let n = ctx.visible_indices.len();

    ctx.painter.rect_filled(ctx.plot_rect, 0.0, egui::Color32::from_rgb(15, 15, 30));

    let max_val = ctx.max_val.max(1.0);
    let w = ctx.plot_rect.width();
    let h = ctx.plot_rect.height();
    let ox = ctx.plot_rect.left();
    let oy = ctx.plot_rect.top();

    let mut label_map: std::collections::HashMap<String, (usize, f64)> = std::collections::HashMap::new();
    for &actual_idx in ctx.visible_indices.iter() {
        if actual_idx >= ctx.labels.len() { continue; }
        let key = ctx.labels[actual_idx].to_uppercase();
        label_map.insert(key, (actual_idx, ctx.values[actual_idx]));
    }

    let border_stroke = egui::Stroke::new(0.4, egui::Color32::from_rgba_premultiplied(50, 50, 80, 100));
    let base_fill = egui::Color32::from_rgb(26, 26, 46);

    for shape in world_data::all_countries() {
        let polys = world_data::normalized_polygons(shape);
        let entry = label_map.get(&shape.id).or_else(|| label_map.get(&shape.name.to_uppercase()));

        let (fill, stroke, is_data) = if let Some(&(idx, value)) = entry {
            let is_hov = ctx.hovered_idx.map(|h| h == idx).unwrap_or(false);
            if is_hov {
                (egui::Color32::from_rgb(255, 220, 50), egui::Stroke::new(1.5, egui::Color32::WHITE), true)
            } else {
                let (r, g, b) = heat_color(value, max_val);
                (egui::Color32::from_rgb(r, g, b), egui::Stroke::new(0.6, egui::Color32::from_rgba_premultiplied(255, 255, 255, 60)), true)
            }
        } else {
            (base_fill, border_stroke, false)
        };

        for poly in &polys {
            if poly.len() < 3 { continue; }
            let points: Vec<egui::Pos2> = poly.iter().map(|pt| {
                egui::pos2(ox + pt[0] * w, oy + pt[1] * h)
            }).collect();
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
    if n == 0 { return; }
    let max_val = max_val.max(1.0);

    for i in 0..n {
        let (r, g, b) = heat_color(values[i], max_val);
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

pub fn render_choropleth_html(
    title: &str,
    labels: &[String],
    values: &[f64],
    width: i32,
    height: i32,
    hover: &[crate::html::hover::HoverSlot],
) -> String {
    use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};
    let n = values.len().min(labels.len());
    if n == 0 { return String::new(); }
    let auto = hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(HoverSlot::new(labels[i].clone()).kv("Valeur", format!("{:.2}", values[i])));
        }
    }
    let mut svg = render_choropleth_fast(values, labels, width, height);
    svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { hover };
    build_chart_html(title, &svg, &slots_to_json(slots))
}


