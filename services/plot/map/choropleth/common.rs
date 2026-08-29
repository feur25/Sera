use super::config::ChoroplethConfig;
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
