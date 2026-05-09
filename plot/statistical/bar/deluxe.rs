use super::config::BarConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, Frame};
use crate::html::hover::slots_to_json;

fn lerp(a: u32, b: u32, t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let r = (((a >> 16) & 0xFF) as f64 * (1.0 - t) + ((b >> 16) & 0xFF) as f64 * t).round() as u32;
    let g = (((a >> 8) & 0xFF) as f64 * (1.0 - t) + ((b >> 8) & 0xFF) as f64 * t).round() as u32;
    let bl = ((a & 0xFF) as f64 * (1.0 - t) + (b & 0xFF) as f64 * t).round() as u32;
    (r << 16) | (g << 8) | bl
}

fn lighten(c: u32) -> u32 { lerp(c, 0xFFFFFF, 0.55) }

pub fn render(cfg: &BarConfig, orient: u8) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }

    let max_v = cfg.values[..n].iter().cloned().fold(0.0_f64, f64::max).max(1e-9);
    let legend_w: i32 = 20;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 52, 38, 52, legend_w, n * 300 + 4096);
    f.open(cfg.title, false);

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"dlxgf\" x=\"-30%\" y=\"-30%\" width=\"160%\" height=\"160%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"3\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");

    let base_colors = [0x06B6D4_u32, 0x6366F1, 0x8B5CF6, 0xF43F5E, 0xF59E0B, 0x10B981];
    for i in 0..n {
        let base = if !cfg.palette.is_empty() { palette_color(cfg.palette, i) }
                   else if cfg.color_hex != 0 { cfg.color_hex }
                   else { base_colors[i % base_colors.len()] };
        let top = lighten(base);
        push_b(&mut f.buf, b"<linearGradient id=\"dlxg");
        push_i(&mut f.buf, i as i32);
        if orient == b'h' {
            push_b(&mut f.buf, b"\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
        } else {
            push_b(&mut f.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
        }
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(base)); push_b(&mut f.buf, b"\" stop-opacity=\"0.9\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.6\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(lerp(base, top, 0.6))); push_b(&mut f.buf, b"\" stop-opacity=\"0.95\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(top)); push_b(&mut f.buf, b"\" stop-opacity=\"1\"/>");
        push_b(&mut f.buf, b"</linearGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    if orient == b'h' {
        let slot_h = f.ph as f64 / n as f64;
        let bar_h = (slot_h * 0.62) as i32;
        let tick_max = max_v;
        for ti in 0..=5 {
            let frac = ti as f64 / 5.0;
            let x = f.pl + (frac * f.pw as f64) as i32;
            let v = tick_max * frac;
            if cfg.gridlines {
                push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x);
                push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, f.pt);
                push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x);
                push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, f.pt + f.ph);
                push_b(&mut f.buf, b"\" stroke=\"#1e293b\" stroke-width=\"1\"/>");
            }
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
            push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
            push_f2(&mut f.buf, v); push_b(&mut f.buf, b"</text>");
        }
        for i in 0..n {
            let bar_w = (cfg.values[i] / max_v * f.pw as f64) as i32;
            let y = f.pt + (i as f64 * slot_h + (slot_h - bar_h as f64) / 2.0) as i32;
            push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
            push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.values[i]);
            push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, f.pl);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bar_w.max(1));
            push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bar_h);
            push_b(&mut f.buf, b"\" fill=\"url(#dlxg"); push_i(&mut f.buf, i as i32);
            push_b(&mut f.buf, b")\" rx=\"4\" filter=\"url(#dlxgf)\"/>");
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 6);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y + bar_h / 2 + 4);
            push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
            escape_xml(&mut f.buf, &cfg.labels[i]); push_b(&mut f.buf, b"</text>");
        }
    } else {
        let slot_w = f.pw as f64 / n as f64;
        let bar_w = (slot_w * 0.68) as i32;
        f.y_grid(5, 0.0, max_v, cfg.gridlines);
        for i in 0..n {
            let bar_h = (cfg.values[i] / max_v * f.ph as f64) as i32;
            let x = f.pl + (i as f64 * slot_w + (slot_w - bar_w as f64) / 2.0) as i32;
            let y = f.pt + f.ph - bar_h;
            push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
            push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.values[i]);
            push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bar_w.max(1));
            push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bar_h.max(1));
            push_b(&mut f.buf, b"\" fill=\"url(#dlxg"); push_i(&mut f.buf, i as i32);
            push_b(&mut f.buf, b")\" rx=\"5\" filter=\"url(#dlxgf)\"/>");
            push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bar_w.max(1));
            push_b(&mut f.buf, b"\" height=\"3\" fill=\"#ffffff\" fill-opacity=\"0.25\" rx=\"2\"/>");
            let cx = x + bar_w / 2;
            let ty = f.pt + f.ph + 14;
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, ty);
            push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
            escape_xml(&mut f.buf, &cfg.labels[i]); push_b(&mut f.buf, b"</text>");
            if cfg.show_text && bar_h > 16 {
                push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
                push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y + 12);
                push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#ffffff\" fill-opacity=\"0.85\">");
                push_f2(&mut f.buf, cfg.values[i]); push_b(&mut f.buf, b"</text>");
            }
        }
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, f.pl);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, f.pt);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, f.pl);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, f.pt + f.ph);
        push_b(&mut f.buf, b"\" stroke=\"#64748b\" stroke-width=\"1.5\"/>");
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, f.pl);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, f.pt + f.ph);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, f.pl + f.pw);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, f.pt + f.ph);
        push_b(&mut f.buf, b"\" stroke=\"#64748b\" stroke-width=\"1.5\"/>");
    }
    f.html(&slots_to_json(cfg.hover))
}


