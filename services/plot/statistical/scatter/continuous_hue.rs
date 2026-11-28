use super::common::{compute_layout, draw_marker, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, lerp_rgb, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13], color_values=[12,18,9,27,22,31,15,38,24,45], variant=\"continuous_hue\""
)]

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) {
        Some(l) => l,
        None => return String::new(),
    };

    let legend_w: i32 = 80;
    let mut f = make_frame(cfg, layout.n, legend_w);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let vmin = cfg.color_values[..layout.n.min(cfg.color_values.len())]
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let vmax = cfg.color_values[..layout.n.min(cfg.color_values.len())]
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let vrange = (vmax - vmin).max(1e-9);

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let v = cfg.color_values.get(i).copied().unwrap_or(vmin);
        let t = ((v - vmin) / vrange).clamp(0.0, 1.0);
        let col = lerp_rgb(cfg.color_low, cfg.color_high, t);
        let hx = hex6(col);

        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\"");
        push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.y_values[i]);
        push_b(&mut f.buf, b"\" data-v=\"");
        push_f2(&mut f.buf, v);
        push_b(&mut f.buf, b"\">");
        draw_marker(
            &mut f.buf,
            cfg.symbol,
            cx,
            cy,
            cfg.point_size,
            &hx,
            &hx,
            cfg.stroke_width,
            0.82,
        );
        push_b(&mut f.buf, b"</g>");
    }

    let bar_x = cfg.width - legend_w + 14;
    let bar_y = f.pt + 6;
    let bar_w = 14;
    let bar_h = (f.ph - 12).max(40);
    let grad_id = "spschg";
    push_b(&mut f.buf, b"<defs><linearGradient id=\"");
    push_b(&mut f.buf, grad_id.as_bytes());
    push_b(&mut f.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    let stops = 8usize;
    for s in 0..stops {
        let t = s as f64 / (stops - 1) as f64;
        let col = lerp_rgb(cfg.color_low, cfg.color_high, t);
        push_b(&mut f.buf, b"<stop offset=\"");
        push_f2(&mut f.buf, t);
        push_b(&mut f.buf, b"\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\"/>");
    }
    push_b(&mut f.buf, b"</linearGradient></defs>");
    push_b(&mut f.buf, b"<rect x=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y);
    push_b(&mut f.buf, b"\" width=\"");
    push_i(&mut f.buf, bar_w);
    push_b(&mut f.buf, b"\" height=\"");
    push_i(&mut f.buf, bar_h);
    push_b(&mut f.buf, b"\" fill=\"url(#");
    push_b(&mut f.buf, grad_id.as_bytes());
    push_b(&mut f.buf, b")\" stroke=\"#475569\" stroke-width=\"0.5\"/>");
    let tx = bar_x + bar_w + 4;
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + 8);
    push_b(
        &mut f.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
    );
    push_f2(&mut f.buf, vmax);
    push_b(&mut f.buf, b"</text>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + bar_h);
    push_b(
        &mut f.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
    );
    push_f2(&mut f.buf, vmin);
    push_b(&mut f.buf, b"</text>");

    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
