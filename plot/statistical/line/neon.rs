use super::config::LineConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate, Frame};

fn neon_col(i: usize, palette: &[u32]) -> u32 {
    if !palette.is_empty() { return palette_color(palette, i); }
    const COLS: [u32; 8] = [
        0x00F5FF, 0xFF00C8, 0x00FF88, 0xFFD600,
        0xFF4D00, 0x7B00FF, 0x40FFAA, 0xFF0055,
    ];
    COLS[i % COLS.len()]
}

#[crate::chart_demo("x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], values=[12,19,15,22,28,24]")]

pub fn render(cfg: &LineConfig) -> String {
    let multi = !cfg.series.is_empty();
    let (n, max_val, min_val) = if multi {
        let n = cfg.x_labels.len();
        if n < 2 { return String::new(); }
        let mx = cfg.series.iter().flat_map(|(_, v)| v.iter().copied())
            .filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0);
        let mn = cfg.series.iter().flat_map(|(_, v)| v.iter().copied())
            .filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min).min(0.0);
        (n, mx, mn)
    } else {
        let n = cfg.values.len().min(cfg.labels.len());
        if n < 2 { return String::new(); }
        let mx = cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0);
        let mn = cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min).min(0.0);
        (n, mx, mn)
    };
    let range = (max_val - min_val).max(1e-12);

    let legend_w = if multi { 140 } else { 0 };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n * 160 + 2048);
    f.open(cfg.title, true);

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"nnglw\" x=\"-80%\" y=\"-80%\" width=\"360%\" height=\"360%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    push_b(&mut f.buf, b"<filter id=\"nndot\" x=\"-100%\" y=\"-100%\" width=\"400%\" height=\"400%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"2.5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    push_b(&mut f.buf, b"</defs>");

    f.y_grid(6, min_val, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let step_x = f.pw as f64 / (n - 1).max(1) as f64;
    let baseline_y = f.pt + ((1.0 - (0.0 - min_val) / range) * f.ph as f64) as i32;

    let series_iter: Vec<(String, Vec<f64>)> = if multi {
        cfg.series.to_vec()
    } else {
        vec![(String::new(), cfg.values.to_vec())]
    };

    for (si, (_, vals)) in series_iter.iter().enumerate() {
        let col = neon_col(si, cfg.palette);
        let hx = hex6(col);

        push_b(&mut f.buf, b"<path fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.07\" stroke=\"none\" d=\"M ");
        push_i(&mut f.buf, f.pl); f.buf.push(b' '); push_i(&mut f.buf, baseline_y);
        let mut pts: Vec<(i32, i32)> = Vec::with_capacity(n);
        for i in 0..n {
            let v = vals.get(i).copied().unwrap_or(0.0);
            let xi = f.pl + (i as f64 * step_x) as i32;
            let frac = (v - min_val) / range;
            let yi = f.pt + ((1.0 - frac) * f.ph as f64) as i32;
            pts.push((xi, yi));
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, xi); f.buf.push(b' '); push_i(&mut f.buf, yi);
        }
        let last_x = f.pl + f.pw;
        push_b(&mut f.buf, b" L "); push_i(&mut f.buf, last_x); f.buf.push(b' '); push_i(&mut f.buf, baseline_y);
        push_b(&mut f.buf, b" Z\"/>");

        push_b(&mut f.buf, b"<polyline fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-opacity=\"0.35\" filter=\"url(#nnglw)\" points=\"");
        for (i, &(xi, yi)) in pts.iter().enumerate() {
            if i > 0 { f.buf.push(b' '); }
            push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
        }
        push_b(&mut f.buf, b"\"/>");

        push_b(&mut f.buf, b"<polyline fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.8\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-opacity=\"0.95\" points=\"");
        for (i, &(xi, yi)) in pts.iter().enumerate() {
            if i > 0 { f.buf.push(b' '); }
            push_i(&mut f.buf, xi); f.buf.push(b','); push_i(&mut f.buf, yi);
        }
        push_b(&mut f.buf, b"\"/>");

        if cfg.show_points {
            for &(xi, yi) in &pts {
                push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, xi);
                push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, yi);
                push_b(&mut f.buf, b"\" r=\"3\" fill=\"#"); f.buf.extend_from_slice(&hx);
                push_b(&mut f.buf, b"\" fill-opacity=\"1\" filter=\"url(#nndot)\"/>");
            }
        }
    }

    let labels_src = if multi { cfg.x_labels } else { cfg.labels };
    for i in 0..n {
        let xi = f.pl + (i as f64 * step_x) as i32;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, xi);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        if let Some(l) = labels_src.get(i) { escape_xml(&mut f.buf, truncate(l, 10)); }
        push_b(&mut f.buf, b"</text>");
    }

    if multi {
        let names: Vec<&str> = series_iter.iter().map(|(n, _)| n.as_str()).collect();
        f.legend_pos(&names, cfg.palette, cfg.legend_position);
    }
    f.html("[]")
}

