use super::common::{compute_layout, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, lerp_rgb, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x=[16.99,10.34,21.01,23.68,24.59,25.29,8.77,26.88,15.04,14.78,10.27,35.26,15.42,18.43,14.83,21.58,10.33,16.29,16.97,20.65,17.92,20.29,15.77,39.42,19.82,17.81,13.37,12.69,21.7,19.65], y=[1.01,1.66,3.5,3.31,3.61,4.71,2.0,3.12,1.96,3.23,1.71,5.0,1.57,3.0,3.02,3.92,1.67,3.71,3.5,3.35,4.08,3.21,2.23,7.58,3.18,2.34,2.0,2.0,4.3,3.0], color_values=[2,3,3,2,4,4,2,4,2,2,2,4,2,4,2,2,3,3,3,3,2,2,2,4,2,4,2,2,2,2], variant=\"sized\""
)]

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) {
        Some(l) => l,
        None => return String::new(),
    };
    let legend_w: i32 = 130;
    let mut f = make_frame(cfg, layout.n, legend_w);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for i in 0..layout.n {
        if let Some(&v) = cfg.color_values.get(i) {
            vmin = vmin.min(v);
            vmax = vmax.max(v);
        }
    }
    if !vmin.is_finite() {
        vmin = 0.0;
        vmax = 1.0;
    }
    let span = (vmax - vmin).max(1e-9);
    let size_min = cfg.size_min.max(1.0);
    let size_max = cfg.size_max.max(size_min + 1.0);

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let v = cfg.color_values.get(i).copied().unwrap_or(vmin);
        let t = ((v - vmin) / span).clamp(0.0, 1.0);
        let r = size_min + t * (size_max - size_min);
        let col = lerp_rgb(cfg.color_low, cfg.color_high, t);
        let hx = hex6(col);
        push_b(&mut f.buf, b"<circle data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\"");
        push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.y_values[i]);
        push_b(&mut f.buf, b"\" data-v=\"");
        push_f2(&mut f.buf, v);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\" cx=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.78\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"");
        push_f2(&mut f.buf, cfg.stroke_width.max(1.0));
        push_b(&mut f.buf, b"\"/>");
    }

    let lx = cfg.width - legend_w + 24;
    let mut ly = f.pt + 14;
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, lx - 12);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, ly);
    push_b(
        &mut f.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#94a3b8\">size</text>",
    );
    ly += 24;
    for k in 0..4 {
        let t = k as f64 / 3.0;
        let v = vmin + t * span;
        let r = size_min + t * (size_max - size_min);
        let col = lerp_rgb(cfg.color_low, cfg.color_high, t);
        let hx = hex6(col);
        let cy = ly + size_max as i32;
        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, lx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.78\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1\"/>");
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, lx + size_max as i32 + 10);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, cy + 4);
        push_b(
            &mut f.buf,
            b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#cbd5e1\">",
        );
        push_f2(&mut f.buf, v);
        push_b(&mut f.buf, b"</text>");
        ly = cy + size_max as i32 + 10;
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
