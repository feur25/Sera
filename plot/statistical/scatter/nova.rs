use super::common::{compute_layout, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

pub const DEMO_KWARGS: &str = "x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]";
fn nova_color(t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let stops: [(f64, u32); 7] = [
        (0.00, 0xFF0080),
        (0.17, 0xFF6B00),
        (0.33, 0xFFD700),
        (0.50, 0x00FF88),
        (0.67, 0x00CFFF),
        (0.83, 0x8A00FF),
        (1.00, 0xFF0080),
    ];
    for w in stops.windows(2) {
        let (t0, c0) = w[0];
        let (t1, c1) = w[1];
        if t <= t1 {
            let s = if (t1 - t0).abs() < 1e-12 { 0.0 } else { ((t - t0) / (t1 - t0)).clamp(0.0, 1.0) };
            let r = (((c0 >> 16) & 0xFF) as f64 * (1.0 - s) + ((c1 >> 16) & 0xFF) as f64 * s).round() as u32;
            let g = (((c0 >> 8) & 0xFF) as f64 * (1.0 - s) + ((c1 >> 8) & 0xFF) as f64 * s).round() as u32;
            let b = ((c0 & 0xFF) as f64 * (1.0 - s) + (c1 & 0xFF) as f64 * s).round() as u32;
            return (r << 16) | (g << 8) | b;
        }
    }
    0xFF0080
}

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let mut f = make_frame(cfg, layout.n, 20);
    f.open(cfg.title, true);

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"nvgf\" x=\"-200%\" y=\"-200%\" width=\"600%\" height=\"600%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    push_b(&mut f.buf, b"<filter id=\"nvcore\" x=\"-120%\" y=\"-120%\" width=\"340%\" height=\"340%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"2\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    push_b(&mut f.buf, b"</defs>");

    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cv: Vec<f64> = if cfg.color_values.len() >= layout.n {
        cfg.color_values[..layout.n].to_vec()
    } else {
        cfg.y_values[..layout.n].to_vec()
    };
    let cmin = cv.iter().cloned().fold(f64::INFINITY, f64::min);
    let cmax = cv.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let cr = (cmax - cmin).max(1e-9);

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let t = (cv[i] - cmin) / cr;
        let col = nova_color(t);
        let hx = hex6(col);
        let r = cfg.point_size.max(3.0);
        let spike = r * 3.8;
        let spike2 = r * 2.4;

        push_b(&mut f.buf, b"<g data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\""); push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\""); push_f2(&mut f.buf, cy as f64 - spike);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\""); push_f2(&mut f.buf, cy as f64 + spike);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.2\" stroke-opacity=\"0.6\" filter=\"url(#nvgf)\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_f2(&mut f.buf, cx as f64 - spike);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\""); push_f2(&mut f.buf, cx as f64 + spike);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.2\" stroke-opacity=\"0.6\" filter=\"url(#nvgf)\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_f2(&mut f.buf, cx as f64 - spike2 * 0.707);
        push_b(&mut f.buf, b"\" y1=\""); push_f2(&mut f.buf, cy as f64 - spike2 * 0.707);
        push_b(&mut f.buf, b"\" x2=\""); push_f2(&mut f.buf, cx as f64 + spike2 * 0.707);
        push_b(&mut f.buf, b"\" y2=\""); push_f2(&mut f.buf, cy as f64 + spike2 * 0.707);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"0.8\" stroke-opacity=\"0.45\" filter=\"url(#nvgf)\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_f2(&mut f.buf, cx as f64 + spike2 * 0.707);
        push_b(&mut f.buf, b"\" y1=\""); push_f2(&mut f.buf, cy as f64 - spike2 * 0.707);
        push_b(&mut f.buf, b"\" x2=\""); push_f2(&mut f.buf, cx as f64 - spike2 * 0.707);
        push_b(&mut f.buf, b"\" y2=\""); push_f2(&mut f.buf, cy as f64 + spike2 * 0.707);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"0.8\" stroke-opacity=\"0.45\" filter=\"url(#nvgf)\"/>");

        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"1\" filter=\"url(#nvcore)\"/>");

        push_b(&mut f.buf, b"</g>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}


