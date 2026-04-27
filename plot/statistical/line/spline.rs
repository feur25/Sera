use super::config::LineConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, truncate, Frame};

fn catmull_rom_path(buf: &mut Vec<u8>, pts: &[(i32, i32)], tension: f64) {
    if pts.len() < 2 { return; }
    push_b(buf, b"M "); push_i(buf, pts[0].0); buf.push(b' '); push_i(buf, pts[0].1);
    let n = pts.len();
    for i in 0..(n - 1) {
        let p0 = if i == 0 { pts[0] } else { pts[i - 1] };
        let p1 = pts[i];
        let p2 = pts[i + 1];
        let p3 = if i + 2 < n { pts[i + 2] } else { pts[i + 1] };
        let t = tension.clamp(0.0, 1.0) / 6.0;
        let c1x = p1.0 as f64 + (p2.0 - p0.0) as f64 * t;
        let c1y = p1.1 as f64 + (p2.1 - p0.1) as f64 * t;
        let c2x = p2.0 as f64 - (p3.0 - p1.0) as f64 * t;
        let c2y = p2.1 as f64 - (p3.1 - p1.1) as f64 * t;
        push_b(buf, b" C ");
        push_i(buf, c1x as i32); buf.push(b' ');
        push_i(buf, c1y as i32); buf.push(b' ');
        push_i(buf, c2x as i32); buf.push(b' ');
        push_i(buf, c2y as i32); buf.push(b' ');
        push_i(buf, p2.0); buf.push(b' '); push_i(buf, p2.1);
    }
}

pub fn render(cfg: &LineConfig) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n < 2 { return String::new(); }
    let max_val = cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0);
    let min_val = cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min).min(0.0);
    let range = (max_val - min_val).max(1e-12);

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, 0, n * 100 + 2048);
    f.open(cfg.title, true);
    f.y_grid(6, min_val, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let step_x = f.pw as f64 / (n - 1).max(1) as f64;
    let pts: Vec<(i32, i32)> = (0..n).map(|i| {
        let v = cfg.values[i];
        (f.pl + (i as f64 * step_x) as i32,
         f.pt + ((1.0 - (v - min_val) / range) * f.ph as f64) as i32)
    }).collect();

    let color = if cfg.color_hex != 0 { cfg.color_hex } else { 0x6366F1 };
    let hx = hex6(color);
    push_b(&mut f.buf, b"<path fill=\"none\" stroke=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width);
    push_b(&mut f.buf, b"\" stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"");
    catmull_rom_path(&mut f.buf, &pts, cfg.spline_tension);
    push_b(&mut f.buf, b"\"/>");

    if cfg.show_points {
        for &(xi, yi) in &pts {
            push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, xi);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, yi);
            push_b(&mut f.buf, b"\" r=\""); push_i(&mut f.buf, cfg.marker_size);
            push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        }
    }

    let tick_step = ((n as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n).step_by(tick_step) {
        let xi = f.pl + (i as f64 * step_x) as i32;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, xi);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        escape_xml(&mut f.buf, truncate(&cfg.labels[i], 10));
        push_b(&mut f.buf, b"</text>");
    }

    f.html("[]")
}
