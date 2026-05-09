use super::config::LineConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color, truncate, Frame};

pub fn render(cfg: &LineConfig) -> String {
    let multi = !cfg.series.is_empty();
    let (n, mx, mn) = if multi {
        let n = cfg.x_labels.len();
        if n < 2 { return String::new(); }
        let mx = cfg.series.iter().flat_map(|(_, v)| v.iter().copied()).filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0);
        let mn = cfg.series.iter().flat_map(|(_, v)| v.iter().copied()).filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min).min(0.0);
        (n, mx, mn)
    } else {
        let n = cfg.values.len().min(cfg.labels.len());
        if n < 2 { return String::new(); }
        let mx = cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0);
        let mn = cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min).min(0.0);
        (n, mx, mn)
    };
    let range = (mx - mn).max(1e-12);

    let legend_w = if multi { 160 } else { 0 };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n * 140 + 2048);
    f.open(cfg.title, true);
    f.y_grid(6, mn, mx, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let step_x = f.pw as f64 / (n - 1).max(1) as f64;
    let series_iter: Vec<(String, Vec<f64>)> = if multi {
        cfg.series.to_vec()
    } else {
        vec![(String::new(), cfg.values.to_vec())]
    };

    let r_marker = cfg.marker_size.max(5);

    for (si, (_, vals)) in series_iter.iter().enumerate() {
        let color = if multi { palette_color(cfg.palette, si) } else if cfg.color_hex != 0 { cfg.color_hex } else { 0x6366F1 };
        let hx = hex6(color);

        push_b(&mut f.buf, b"<polyline fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width);
        push_b(&mut f.buf, b"\" stroke-opacity=\"0.7\" stroke-linecap=\"round\" stroke-linejoin=\"round\" points=\"");
        for i in 0..n {
            let v = vals.get(i).copied().unwrap_or(0.0);
            let xi = f.pl + (i as f64 * step_x) as i32;
            let yi = f.pt + ((1.0 - (v - mn) / range) * f.ph as f64) as i32;
            if i > 0 { f.buf.push(b' '); }
            push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
        }
        push_b(&mut f.buf, b"\"/>");

        for i in 0..n {
            let v = vals.get(i).copied().unwrap_or(0.0);
            let xi = f.pl + (i as f64 * step_x) as i32;
            let yi = f.pt + ((1.0 - (v - mn) / range) * f.ph as f64) as i32;
            push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, xi);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, yi);
            push_b(&mut f.buf, b"\" r=\""); push_i(&mut f.buf, r_marker);
            push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.5\"/>");
        }
    }

    let labels_src = if multi { cfg.x_labels } else { cfg.labels };
    let tick_step = ((n as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n).step_by(tick_step) {
        let xi = f.pl + (i as f64 * step_x) as i32;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, xi);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        if let Some(l) = labels_src.get(i) { escape_xml(&mut f.buf, truncate(l, 10)); }
        push_b(&mut f.buf, b"</text>");
    }

    if multi {
        let names: Vec<&str> = cfg.series.iter().map(|(n, _)| n.as_str()).collect();
        f.legend_pos(&names, cfg.palette, cfg.legend_position);
    }
    f.html("[]")
}


