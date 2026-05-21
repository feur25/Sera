use super::config::LineConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color, truncate, Frame};

#[crate::chart_demo("x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], values=[12,19,15,22,28,24]")]

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
    let color = if cfg.color_hex != 0 { cfg.color_hex } else { 0x6366F1 };
    let hx = hex6(color);
    let shape = match cfg.step_shape { "vh" => 1u8, "hvh" => 2u8, "vhv" => 3u8, _ => 0u8 };

    push_b(&mut f.buf, b"<polyline fill=\"none\" stroke=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width);
    push_b(&mut f.buf, b"\" stroke-linecap=\"round\" stroke-linejoin=\"miter\" points=\"");
    let mut first = true;
    for i in 0..n {
        let v = cfg.values[i];
        let xi = f.pl + (i as f64 * step_x) as i32;
        let yi = f.pt + ((1.0 - (v - min_val) / range) * f.ph as f64) as i32;
        if first {
            push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
            first = false;
        } else {
            let prev_v = cfg.values[i - 1];
            let xp = f.pl + ((i - 1) as f64 * step_x) as i32;
            let yp = f.pt + ((1.0 - (prev_v - min_val) / range) * f.ph as f64) as i32;
            f.buf.push(b' ');
            match shape {
                0 => {
                    push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yp);
                    f.buf.push(b' ');
                    push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
                }
                1 => {
                    push_i(&mut f.buf, xp); f.buf.push(b','); push_i(&mut f.buf, yi);
                    f.buf.push(b' ');
                    push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
                }
                2 => {
                    let xm = (xp + xi) / 2;
                    push_i(&mut f.buf, xm); f.buf.push(b','); push_i(&mut f.buf, yp);
                    f.buf.push(b' ');
                    push_i(&mut f.buf, xm); f.buf.push(b','); push_i(&mut f.buf, yi);
                    f.buf.push(b' ');
                    push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
                }
                _ => {
                    let ym = (yp + yi) / 2;
                    push_i(&mut f.buf, xp); f.buf.push(b','); push_i(&mut f.buf, ym);
                    f.buf.push(b' ');
                    push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, ym);
                    f.buf.push(b' ');
                    push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
                }
            }
        }
    }
    push_b(&mut f.buf, b"\"/>");

    if cfg.show_points {
        for i in 0..n {
            let v = cfg.values[i];
            let xi = f.pl + (i as f64 * step_x) as i32;
            let yi = f.pt + ((1.0 - (v - min_val) / range) * f.ph as f64) as i32;
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

    let _ = palette_color;
    f.html("[]")
}

