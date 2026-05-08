use super::common::{compute_layout, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let mut f = make_frame(cfg, layout.n, 20);
    f.open(cfg.title, true);

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"stlgf\" x=\"-150%\" y=\"-150%\" width=\"400%\" height=\"400%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"4.5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    push_b(&mut f.buf, b"<radialGradient id=\"stlhaze\" cx=\"30%\" cy=\"70%\" r=\"55%\">");
    push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#F59E0B\" stop-opacity=\"0.18\"/>");
    push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#0f172a\" stop-opacity=\"0\"/>");
    push_b(&mut f.buf, b"</radialGradient>");
    push_b(&mut f.buf, b"</defs>");

    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cx_mid = (layout.xmin2 + layout.xmax2) / 2.0;
    let cy_mid = (layout.ymin2 + layout.ymax2) / 2.0;
    let xr2 = layout.xr2.max(1e-9);
    let yr2 = layout.yr2.max(1e-9);

    for i in 0..layout.n {
        let (px, py) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let dx = (cfg.x_values[i] - cx_mid) / xr2;
        let dy = (cfg.y_values[i] - cy_mid) / yr2;
        let dist = (dx * dx + dy * dy).sqrt().min(0.7);
        let t = 1.0 - dist / 0.7;
        let r_base = cfg.point_size.max(2.0);
        let r = r_base + t * 2.5;

        let col = if t > 0.7 {
            let s = (t - 0.7) / 0.3;
            let r_c = (0xFBu32 + (s * (0xFF - 0xFB) as f64) as u32).min(255);
            let g_c = (0xBF + (s * (0xFF - 0xBF) as f64) as u32).min(255);
            (r_c << 16) | (g_c << 8) | 0x24
        } else if t > 0.35 {
            let s = (t - 0.35) / 0.35;
            0xF59E0B_u32.saturating_add(((s * 6.0) as u32) << 16)
        } else {
            let s = t / 0.35;
            let _ = s;
            0xD97706
        };
        let hx = hex6(col);

        push_b(&mut f.buf, b"<g data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\""); push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, px);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, py);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.93\" filter=\"url(#stlgf)\"/>");
        push_b(&mut f.buf, b"</g>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
