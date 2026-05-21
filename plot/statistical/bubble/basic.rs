use super::common::{compute_layout, make_frame, point_px, radius};
use super::config::BubbleConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

#[crate::chart_demo("x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"]")]

pub fn render(cfg: &BubbleConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let mut f = make_frame(cfg, layout.n, 20);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let color = if cfg.color_hex != 0 { cfg.color_hex } else { 0x6366F1 };
    let hx = hex6(color);

    for &i in &layout.indices {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let r = radius(cfg, &layout, i);
        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.6\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width);
        push_b(&mut f.buf, b"\"/>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}

