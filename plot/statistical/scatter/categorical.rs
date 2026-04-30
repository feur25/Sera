use super::common::{compute_layout, draw_marker, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_legend_item};

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };

    let mut cat_order: Vec<String> = Vec::new();
    for i in 0..layout.n {
        let c = if i < cfg.categories.len() { &cfg.categories[i] } else { "" };
        if !c.is_empty() && !cat_order.iter().any(|x| x == c) { cat_order.push(c.to_string()); }
    }
    let has_legend = !cat_order.is_empty();
    let legend_w: i32 = if has_legend { 140 } else { 20 };

    let mut f = make_frame(cfg, layout.n, legend_w);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let ci = if i < cfg.categories.len() {
            cat_order.iter().position(|c| c == &cfg.categories[i]).unwrap_or(0)
        } else { 0 };
        let col = palette_color(cfg.palette, ci);
        let hx = hex6(col);

        push_b(&mut f.buf, b"<g data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" data-x=\""); push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.categories.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.categories[i]);
        }
        push_b(&mut f.buf, b"\">");
        draw_marker(&mut f.buf, cfg.symbol, cx, cy, cfg.point_size, &hx, &hx, cfg.stroke_width, 0.75);
        push_b(&mut f.buf, b"</g>");
    }

    if has_legend {
        for (li, name) in cat_order.iter().enumerate() {
            let col = palette_color(cfg.palette, li);
            svg_legend_item(&mut f.buf, li as i32, name, col, cfg.width - legend_w + 10, f.pt + 4 + (li as i32) * 20, 20);
        }
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
