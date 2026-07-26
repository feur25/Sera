use super::config::CorrelogramConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

pub fn corr_color(v: f64) -> [u8; 6] {
    let t = v.clamp(-1.0, 1.0);
    let (r, g, b): (u8, u8, u8) = if t >= 0.0 {
        let u = (t * 255.0) as u8;
        (u, (u as f64 * 0.35) as u8, (u as f64 * 0.35) as u8)
    } else {
        let u = ((-t) * 255.0) as u8;
        ((u as f64 * 0.35) as u8, (u as f64 * 0.55) as u8, u)
    };
    hex6(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
}

fn draw_value_text(buf: &mut Vec<u8>, cx: f64, cy: f64, cell: f64, v: f64) {
    let label = format!("{:.2}", v);
    push_b(buf, b"<text x=\"");
    push_f2(buf, cx);
    push_b(buf, b"\" y=\"");
    push_f2(buf, cy + 4.0);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
    push_f2(buf, (cell * 0.28).clamp(7.0, 11.0));
    push_b(buf, b"\" fill=\"#1e293b\">");
    buf.extend_from_slice(label.as_bytes());
    push_b(buf, b"</text>");
}

fn draw_cell(
    buf: &mut Vec<u8>,
    shape: &str,
    idx: i32,
    v: f64,
    cx: f64,
    cy: f64,
    cell: f64,
    show_values: bool,
) {
    let fill = corr_color(v);
    match shape {
        "square" | "color" => {
            let s = cell * 0.94;
            push_b(buf, b"<rect x=\"");
            push_f2(buf, cx - s / 2.0);
            push_b(buf, b"\" y=\"");
            push_f2(buf, cy - s / 2.0);
            push_b(buf, b"\" width=\"");
            push_f2(buf, s);
            push_b(buf, b"\" height=\"");
            push_f2(buf, s);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(&fill);
            push_b(buf, b"\" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"0.5\" data-idx=\"");
            push_i(buf, idx);
            push_b(buf, b"\"/>");
        }
        "ellipse" => {
            let rx = cell * 0.44;
            let ry = (cell * 0.44 * (1.0 - v.abs() * 0.82)).max(cell * 0.05);
            let deg = if v >= 0.0 { -45.0 } else { 45.0 };
            push_b(buf, b"<ellipse cx=\"");
            push_f2(buf, cx);
            push_b(buf, b"\" cy=\"");
            push_f2(buf, cy);
            push_b(buf, b"\" rx=\"");
            push_f2(buf, rx);
            push_b(buf, b"\" ry=\"");
            push_f2(buf, ry);
            push_b(buf, b"\" transform=\"rotate(");
            push_f2(buf, deg);
            push_b(buf, b" ");
            push_f2(buf, cx);
            push_b(buf, b" ");
            push_f2(buf, cy);
            push_b(buf, b")\" fill=\"#");
            buf.extend_from_slice(&fill);
            push_b(buf, b"\" fill-opacity=\"0.85\" stroke=\"#1e293b\" stroke-width=\"0.5\" data-idx=\"");
            push_i(buf, idx);
            push_b(buf, b"\"/>");
        }
        "pie" => {
            let r = cell * 0.42;
            let frac = v.abs().clamp(0.0, 1.0);
            if frac >= 0.999 {
                push_b(buf, b"<circle cx=\"");
                push_f2(buf, cx);
                push_b(buf, b"\" cy=\"");
                push_f2(buf, cy);
                push_b(buf, b"\" r=\"");
                push_f2(buf, r);
                push_b(buf, b"\" fill=\"#");
                buf.extend_from_slice(&fill);
                push_b(buf, b"\" fill-opacity=\"0.85\" data-idx=\"");
                push_i(buf, idx);
                push_b(buf, b"\"/>");
            } else {
                let start = -std::f64::consts::FRAC_PI_2;
                let end = start + frac * std::f64::consts::TAU;
                let x0 = cx + r * start.cos();
                let y0 = cy + r * start.sin();
                let x1 = cx + r * end.cos();
                let y1 = cy + r * end.sin();
                let large = if frac > 0.5 { 1 } else { 0 };
                push_b(buf, b"<circle cx=\"");
                push_f2(buf, cx);
                push_b(buf, b"\" cy=\"");
                push_f2(buf, cy);
                push_b(buf, b"\" r=\"");
                push_f2(buf, r);
                push_b(buf, b"\" fill=\"none\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
                push_b(buf, b"<path data-idx=\"");
                push_i(buf, idx);
                push_b(buf, b"\" d=\"M");
                push_f2(buf, cx);
                push_b(buf, b",");
                push_f2(buf, cy);
                push_b(buf, b" L");
                push_f2(buf, x0);
                push_b(buf, b",");
                push_f2(buf, y0);
                push_b(buf, b" A");
                push_f2(buf, r);
                push_b(buf, b",");
                push_f2(buf, r);
                push_b(buf, b" 0 ");
                buf.push(large + b'0');
                push_b(buf, b",1 ");
                push_f2(buf, x1);
                push_b(buf, b",");
                push_f2(buf, y1);
                push_b(buf, b" Z\" fill=\"#");
                buf.extend_from_slice(&fill);
                push_b(buf, b"\" fill-opacity=\"0.85\"/>");
            }
        }
        "number" => {
            draw_value_text(buf, cx, cy, cell, v);
            return;
        }
        _ => {
            let r = (v.abs() * cell * 0.46).max(1.5);
            push_b(buf, b"<circle cx=\"");
            push_f2(buf, cx);
            push_b(buf, b"\" cy=\"");
            push_f2(buf, cy);
            push_b(buf, b"\" r=\"");
            push_f2(buf, r);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(&fill);
            push_b(buf, b"\" fill-opacity=\"0.80\" data-idx=\"");
            push_i(buf, idx);
            push_b(buf, b"\"/>");
        }
    }
    if show_values && shape != "number" {
        draw_value_text(buf, cx, cy, cell, v);
    }
}

fn draw_legend(buf: &mut Vec<u8>, x: f64, y_top: f64, y_bot: f64) {
    let steps = 40;
    let h = (y_bot - y_top) / steps as f64;
    for i in 0..steps {
        let t = 1.0 - 2.0 * (i as f64 / (steps - 1) as f64);
        let fill = corr_color(t);
        push_b(buf, b"<rect x=\"");
        push_f2(buf, x);
        push_b(buf, b"\" y=\"");
        push_f2(buf, y_top + i as f64 * h);
        push_b(buf, b"\" width=\"14\" height=\"");
        push_f2(buf, h + 0.5);
        push_b(buf, b"\" fill=\"#");
        buf.extend_from_slice(&fill);
        push_b(buf, b"\"/>");
    }
    push_b(buf, b"<rect x=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y_top);
    push_b(buf, b"\" width=\"14\" height=\"");
    push_f2(buf, y_bot - y_top);
    push_b(buf, b"\" fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"0.6\"/>");
    for (t, lbl) in [(1.0, "1"), (0.0, "0"), (-1.0, "-1")] {
        let ly = y_top + (1.0 - (t + 1.0) / 2.0) * (y_bot - y_top);
        push_b(buf, b"<text x=\"");
        push_f2(buf, x + 19.0);
        push_b(buf, b"\" y=\"");
        push_f2(buf, ly + 3.5);
        push_b(buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
        push_b(buf, lbl.as_bytes());
        push_b(buf, b"</text>");
    }
}

pub fn render_impl(cfg: &CorrelogramConfig, shape: &str, shape2: &str, layout: &str, legend: bool) -> String {
    let n = cfg.labels.len();
    if n == 0 || cfg.matrix.len() < n * n {
        return String::new();
    }

    let title_h = if cfg.title.is_empty() { 0.0f64 } else { 22.0 };
    let pad_l = 52.0f64;
    let pad_t = title_h + 12.0;
    let pad_b = 52.0f64;
    let pad_r = if legend { 44.0 } else { 12.0 };
    let cell = ((cfg.width as f64 - pad_l - pad_r) / n as f64)
        .min((cfg.height as f64 - pad_t - pad_b) / n as f64);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * n * 220 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cfg.width as f64 / 2.0);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let shape2 = if shape2.is_empty() { shape } else { shape2 };

    for row in 0..n {
        for col in 0..n {
            let visible = match layout {
                "upper" => col > row,
                "lower" => col < row,
                "mixed" => col != row,
                _ => true,
            };
            if !visible {
                continue;
            }
            let v = cfg.matrix[row * n + col];
            let cx = pad_l + (col as f64 + 0.5) * cell;
            let cy = pad_t + (row as f64 + 0.5) * cell;
            let idx = (row * n + col) as i32;
            let cell_shape = match layout {
                "mixed" if col < row => shape2,
                _ => shape,
            };
            draw_cell(&mut buf, cell_shape, idx, v, cx, cy, cell, cfg.show_values);
        }
    }

    let fs_axis = (cell * 0.3).clamp(7.5, 10.5);
    for i in 0..n {
        let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
        let cx = pad_l + (i as f64 + 0.5) * cell;
        let cy = pad_t + (i as f64 + 0.5) * cell;

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, pad_l - 4.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 3.5);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, fs_axis);
        push_b(&mut buf, b"\" fill=\"#374151\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, pad_t + n as f64 * cell + 14.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, fs_axis);
        push_b(&mut buf, b"\" fill=\"#374151\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text>");
    }

    if legend {
        let lx = pad_l + n as f64 * cell + 14.0;
        draw_legend(&mut buf, lx, pad_t + 4.0, pad_t + n as f64 * cell - 4.0);
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
