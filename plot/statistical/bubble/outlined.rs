use super::common::{compute_layout, make_frame, point_px, radius};
use super::config::BubbleConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_legend_item};

#[crate::chart_demo("x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"]")]

pub fn render(cfg: &BubbleConfig) -> String {
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

    let fixed_hex = if cfg.color_hex != 0 { cfg.color_hex } else { 0x6366F1 };
    let stroke_w = cfg.stroke_width.max(2.5);

    for &i in &layout.indices {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let r = radius(cfg, &layout, i);
        let col = if has_legend && i < cfg.categories.len() {
            let ci = cat_order.iter().position(|c| c == &cfg.categories[i]).unwrap_or(0);
            palette_color(cfg.palette, ci)
        } else { fixed_hex };
        let hx = hex6(col);

        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, stroke_w);
        push_b(&mut f.buf, b"\"/>");
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

