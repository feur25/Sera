use super::config::BubbleMapConfig;
use crate::plot::map::regions;

pub const PALETTE: &[(u8, u8, u8)] = &[
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

pub fn svg_open(width: i32, height: i32) -> String {
    let mut svg = String::with_capacity(4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0d1117\"/>");
    svg
}

pub fn push_outlines(svg: &mut String, cfg: &BubbleMapConfig, shapes: &[&'static crate::plot::map::svg_parser::CountryShape]) {
    for shape in shapes {
        let polys = (cfg.region.normalize)(shape);
        for poly in &polys {
            if poly.len() < 3 {
                continue;
            }
            svg.push_str("<path d=\"M");
            for (j, pt) in poly.iter().enumerate() {
                let px = pt[0] * cfg.width as f32;
                let py = pt[1] * cfg.height as f32;
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }
}

pub fn visible_shapes(cfg: &BubbleMapConfig) -> Vec<&'static crate::plot::map::svg_parser::CountryShape> {
    regions::shapes_in_group(cfg.region, cfg.group)
}

pub fn to_html(cfg: &BubbleMapConfig, svg: String) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.values.len().min(cfg.labels.len());
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(HoverSlot::new(cfg.labels[i].clone()).kv("Valeur", format!("{:.2}", cfg.values[i])));
        }
    }
    let svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
