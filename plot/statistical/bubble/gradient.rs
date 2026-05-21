use super::common::{compute_layout, lerp_color, make_frame, point_px, radius};
use super::config::BubbleConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

#[crate::chart_demo("x=[1,2,3,4,5,6,7], y=[3,5,2,7,6,8,4], sizes=[20,40,15,55,30,45,25], values=[1,2,3,4,5,6,7]")]

pub fn render(cfg: &BubbleConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let legend_w: i32 = 78;
    let mut f = make_frame(cfg, layout.n, legend_w);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cv: Vec<f64> = if cfg.color_values.len() >= layout.n {
        cfg.color_values[..layout.n].to_vec()
    } else {
        (0..layout.n).map(|i| cfg.sizes[i].abs()).collect()
    };
    let cmin = cv.iter().cloned().fold(f64::INFINITY, f64::min);
    let cmax = cv.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let cr = (cmax - cmin).max(1e-9);
    let lo = cfg.color_low;
    let hi = cfg.color_high;

    for &i in &layout.indices {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let r = radius(cfg, &layout, i);
        let t = (cv[i] - cmin) / cr;
        let col = lerp_color(lo, hi, t);
        let hx = hex6(col);

        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        push_b(&mut f.buf, b"\" data-c=\""); push_f2(&mut f.buf, cv[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.7\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width);
        push_b(&mut f.buf, b"\"/>");
    }

    let bar_x = cfg.width - legend_w + 14;
    let bar_y = f.pt + 6;
    let bar_w = 14;
    let bar_h = (f.ph - 12).max(40);
    let grad_id = "spbg";
    push_b(&mut f.buf, b"<defs><linearGradient id=\""); push_b(&mut f.buf, grad_id.as_bytes());
    push_b(&mut f.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\"><stop offset=\"0\" stop-color=\"#");
    f.buf.extend_from_slice(&hex6(lo));
    push_b(&mut f.buf, b"\"/><stop offset=\"1\" stop-color=\"#");
    f.buf.extend_from_slice(&hex6(hi));
    push_b(&mut f.buf, b"\"/></linearGradient></defs>");
    push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, bar_y);
    push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bar_w);
    push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bar_h);
    push_b(&mut f.buf, b"\" fill=\"url(#"); push_b(&mut f.buf, grad_id.as_bytes());
    push_b(&mut f.buf, b")\" stroke=\"#475569\" stroke-width=\"0.5\"/>");

    let tx = bar_x + bar_w + 4;
    push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, bar_y + 8);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    push_f2(&mut f.buf, cmax); push_b(&mut f.buf, b"</text>");
    push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, bar_y + bar_h);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    push_f2(&mut f.buf, cmin); push_b(&mut f.buf, b"</text>");

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}

