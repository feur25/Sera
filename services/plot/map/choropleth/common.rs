use super::config::ChoroplethConfig;
use crate::plot::map::projections::{self, Projection};
use crate::plot::map::regions;

pub fn render_svg(cfg: &ChoroplethConfig, color_for: impl Fn(f64, f64, f64) -> (u8, u8, u8)) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }

    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let visible_ids: std::collections::HashSet<&str> = visible.iter().map(|s| s.id.as_str()).collect();

    let mut matched: Vec<(usize, f64)> = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) {
            if visible_ids.contains(shape.id.as_str()) {
                matched.push((i, cfg.values[i]));
            }
        }
    }
    let min_val = matched.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min);
    let max_val = matched.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max);
    let (min_val, max_val) = if min_val.is_finite() && max_val.is_finite() {
        (min_val, max_val)
    } else {
        (0.0, 1.0)
    };

    let width = cfg.width;
    let height = cfg.height;
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

    for shape in &visible {
        let polys = (cfg.region.normalize)(shape);
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
            svg.push_str(" Z\" fill=\"#1a1a2e\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }

    for &(i, value) in &matched {
        let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) else {
            continue;
        };
        let (r, g, b) = color_for(value, min_val, max_val);
        let polys = (cfg.region.normalize)(shape);
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
            svg.push_str(&format!(
                " Z\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.85\" stroke=\"rgba(255,255,255,0.3)\" stroke-width=\"0.5\" data-index=\"{i}\"/>"
            ));
        }
    }

    svg.push_str("</svg>");
    svg
}

pub fn render_svg_projected(
    cfg: &ChoroplethConfig,
    color_for: impl Fn(f64, f64, f64) -> (u8, u8, u8),
    projection: Projection,
    center_lat: f64,
    center_lon: f64,
) -> String {
    let Some(to_latlon) = cfg.region.to_latlon else {
        return render_svg(cfg, color_for);
    };
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }

    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let visible_ids: std::collections::HashSet<&str> = visible.iter().map(|s| s.id.as_str()).collect();
    let raw = projections::project_shapes(&visible, to_latlon, projection, center_lat, center_lon);
    let (projected, disc) = projections::normalize_projected(&raw, cfg.width, cfg.height, 0.92);

    let mut matched: Vec<(usize, f64)> = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) {
            if visible_ids.contains(shape.id.as_str()) {
                matched.push((i, cfg.values[i]));
            }
        }
    }
    let min_val = matched.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min);
    let max_val = matched.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max);
    let (min_val, max_val) = if min_val.is_finite() && max_val.is_finite() {
        (min_val, max_val)
    } else {
        (0.0, 1.0)
    };
    let data_for_shape_id: std::collections::HashMap<&str, (usize, f64)> = matched
        .iter()
        .filter_map(|&(i, v)| (cfg.region.lookup)(&cfg.labels[i]).map(|s| (s.id.as_str(), (i, v))))
        .collect();

    let width = cfg.width;
    let height = cfg.height;
    let mut svg = String::with_capacity(projected.len() * 2000 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0f0f1e\"/>");

    if projection.draws_disc() {
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#111633\" stroke=\"#33406e\" stroke-width=\"1\"/>",
            disc.cx, disc.cy, disc.radius
        ));
    }

    for (idx, polys) in &projected {
        let shape_id = visible[*idx].id.as_str();
        let (fill, opacity, data_attr) = match data_for_shape_id.get(shape_id) {
            Some(&(i, v)) => {
                let (r, g, b) = color_for(v, min_val, max_val);
                (format!("rgb({r},{g},{b})"), 0.85, format!(" data-index=\"{i}\""))
            }
            None => ("#1a1a2e".to_string(), 1.0, String::new()),
        };
        for poly in polys {
            if poly.len() < 3 {
                continue;
            }
            svg.push_str("<path d=\"M");
            for (j, pt) in poly.iter().enumerate() {
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", pt[0], pt[1]));
            }
            svg.push_str(&format!(
                " Z\" fill=\"{fill}\" fill-opacity=\"{opacity}\" stroke=\"#2a2a4a\" stroke-width=\"0.4\"{data_attr}/>"
            ));
        }
    }

    svg.push_str("</svg>");
    svg
}

pub fn render_html_projected(
    cfg: &ChoroplethConfig,
    color_for: impl Fn(f64, f64, f64) -> (u8, u8, u8),
    projection: Projection,
    center_lat: f64,
    center_lon: f64,
) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(HoverSlot::new(cfg.labels[i].clone()).kv("Valeur", format!("{:.2}", cfg.values[i])));
        }
    }
    let mut svg = render_svg_projected(cfg, color_for, projection, center_lat, center_lon);
    svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}

pub fn render_html(cfg: &ChoroplethConfig, color_for: impl Fn(f64, f64, f64) -> (u8, u8, u8)) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(HoverSlot::new(cfg.labels[i].clone()).kv("Valeur", format!("{:.2}", cfg.values[i])));
        }
    }
    let mut svg = render_svg(cfg, color_for);
    svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
