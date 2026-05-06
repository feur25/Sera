use super::common::{prepare, open, finalize, dot, data_attrs, x_tick_label};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, svg_axis_lines, svg_y_label};

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let (mut b, pl, pt, pw, ph) = open(cfg, 64, 42, 20, 52);
    let mean: f64 = p.values.iter().sum::<f64>() / p.n as f64;
    let mut max_dev: f64 = 0.0;
    for v in p.values.iter() {
        let d = (v - mean).abs();
        if d > max_dev { max_dev = d; }
    }
    if max_dev < 1e-9 { max_dev = 1.0; }
    let pos_color: u32 = 0x10B981;
    let neg_color: u32 = 0xEF4444;
    svg_y_label(&mut b, cfg.y_label, 12, pt, ph);
    svg_axis_lines(&mut b, pl, pt, pw, ph);
    let mid_y = pt + ph / 2;
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pl);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, mid_y);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pl + pw);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, mid_y);
    push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" stroke-dasharray=\"4,3\"/>");
    push_b(&mut b, b"<text x=\""); push_i(&mut b, pl + pw - 4);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, mid_y - 6);
    push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#64748b\">mean ");
    push_f2(&mut b, mean);
    push_b(&mut b, b"</text>");
    let step = pw as f64 / p.n as f64;
    let tick_step = ((p.n as f64 / 14.0).ceil() as usize).max(1);
    for i in 0..p.n {
        let cx = pl + (step * 0.5 + step * i as f64) as i32;
        let dev = p.values[i] - mean;
        let frac = dev / max_dev;
        let y_v = mid_y - (frac * (ph as f64 / 2.0 - 8.0)) as i32;
        let col = if dev >= 0.0 { pos_color } else { neg_color };
        let hx = hex6(col);
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, mid_y);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, y_v);
        push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\" opacity=\"0.85\"/>");
        dot(&mut b, &p, i, cx, y_v, 5, col);
        if cfg.show_values {
            let dy = if dev >= 0.0 { y_v - 9 } else { y_v + 14 };
            push_b(&mut b, b"<text x=\""); push_i(&mut b, cx);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, dy);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            if dev >= 0.0 { push_b(&mut b, b"+"); }
            push_f2(&mut b, dev);
            push_b(&mut b, b"</text>");
        }
        if i % tick_step == 0 {
            x_tick_label(&mut b, cx, pt + ph + 14, &p.labels[i]);
        }
    }
    finalize(b, cfg)
}
