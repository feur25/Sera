use super::common::{compute_layout, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn galaxy_color(t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let stops: [(f64, u32); 6] = [
        (0.00, 0x22D3EE),
        (0.20, 0x818CF8),
        (0.40, 0xEC4899),
        (0.60, 0xF97316),
        (0.80, 0xFACC15),
        (1.00, 0x34D399),
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
    0x34D399
}

#[crate::chart_demo("x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13]")]

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let mut f = make_frame(cfg, layout.n, 20);
    f.open(cfg.title, true);

    push_b(&mut f.buf, b"<defs><filter id=\"spgf\" x=\"-120%\" y=\"-120%\" width=\"340%\" height=\"340%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"3.5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter></defs>");

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
        let col = galaxy_color(t);
        let hx = hex6(col);
        let r = cfg.point_size.max(2.5);

        push_b(&mut f.buf, b"<g data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\""); push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.92\" filter=\"url(#spgf)\"/>");
        push_b(&mut f.buf, b"</g>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}

