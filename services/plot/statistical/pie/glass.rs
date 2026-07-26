use super::common::{open_svg, write_title};
use super::config::PieConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};
use std::f64::consts::PI;

#[crate::chart_demo(
    "labels=[\"Acquisition\",\"Conversion\",\"Retention\",\"Referral\",\"Other\"], values=[42,26,16,11,5], center_text=\"27.3K\", center_subtext=\"TOTAL\", variant=\"glass\""
)]

pub fn render(cfg: &PieConfig) -> String {
    let (labels, values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = labels.len().min(values.len());
    if n == 0 {
        return String::new();
    }
    let total: f64 = values[..n].iter().sum();
    if total <= 0.0 {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let title_pad = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let mut buf = Vec::<u8>::with_capacity(n * 460 + 2048);
    open_svg(&mut buf, w, h);
    write_title(&mut buf, w, cfg.title);

    let cx = w as f64 / 2.0;
    let cy = title_pad + (h as f64 - title_pad) / 2.0;
    let r = ((w.min(h) as f64) * 0.5 - 92.0).max(40.0);
    let r_inner = r * if cfg.donut > 0.0 { cfg.donut.clamp(0.3, 0.85) } else { 0.58 };

    push_b(&mut buf, b"<defs>");
    for i in 0..n {
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        push_b(&mut buf, b"<radialGradient id=\"sp-glass-");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" cx=\"35%\" cy=\"30%\" r=\"75%\">");
        push_b(&mut buf, b"<stop offset=\"0%\" stop-color=\"#ffffff\" stop-opacity=\"0.5\"/>");
        push_b(&mut buf, b"<stop offset=\"55%\" stop-color=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stop-opacity=\"0.88\"/>");
        push_b(&mut buf, b"<stop offset=\"100%\" stop-color=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stop-opacity=\"1\"/></radialGradient>");
    }
    push_b(&mut buf, b"</defs>");

    let mut angle = -PI / 2.0;
    for i in 0..n {
        let frac = values[i] / total;
        let sweep = frac * 2.0 * PI;
        let end = angle + sweep;
        let mid = angle + sweep / 2.0;
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        let large: u8 = if sweep > PI { 1 } else { 0 };
        let x1 = cx + r * angle.cos();
        let y1 = cy + r * angle.sin();
        let x2 = cx + r * end.cos();
        let y2 = cy + r * end.sin();
        let xi1 = cx + r_inner * angle.cos();
        let yi1 = cy + r_inner * angle.sin();
        let xi2 = cx + r_inner * end.cos();
        let yi2 = cy + r_inner * end.sin();

        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &labels[i]);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, values[i]);
        push_b(&mut buf, b"\" data-kv-Part=\"");
        push_f2(&mut buf, frac * 100.0);
        push_b(&mut buf, b"%\" d=\"M");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b" A");
        push_f2(&mut buf, r);
        push_b(&mut buf, b",");
        push_f2(&mut buf, r);
        push_b(&mut buf, b" 0 ");
        buf.push(large + b'0');
        push_b(&mut buf, b",1 ");
        push_f2(&mut buf, x2);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y2);
        push_b(&mut buf, b" L");
        push_f2(&mut buf, xi2);
        push_b(&mut buf, b",");
        push_f2(&mut buf, yi2);
        push_b(&mut buf, b" A");
        push_f2(&mut buf, r_inner);
        push_b(&mut buf, b",");
        push_f2(&mut buf, r_inner);
        push_b(&mut buf, b" 0 ");
        buf.push(large + b'0');
        push_b(&mut buf, b",0 ");
        push_f2(&mut buf, xi1);
        push_b(&mut buf, b",");
        push_f2(&mut buf, yi1);
        push_b(&mut buf, b" Z\" fill=\"url(#sp-glass-");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b")\" stroke=\"#ffffff\" stroke-width=\"1.5\" stroke-opacity=\"0.5\"/>");

        let lr = r + 44.0;
        let lx = cx + lr * mid.cos();
        let ly = cy + lr * mid.sin();
        let ax = cx + (r + 4.0) * mid.cos();
        let ay = cy + (r + 4.0) * mid.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, ay);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" opacity=\"0.7\"/>");
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, ax);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, ay);
        push_b(&mut buf, b"\" r=\"2.5\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\"/>");

        let right = mid.cos() >= 0.0;
        let anchor: &[u8] = if right { b"start" } else { b"end" };
        let tx = if right { lx + 6.0 } else { lx - 6.0 };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly - 3.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#64748b\" letter-spacing=\"0.05em\">");
        escape_xml(&mut buf, &truncate(&labels[i], 16).to_uppercase());
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 13.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#0f172a\">");
        push_i(&mut buf, (frac * 100.0 + 0.5) as i32);
        push_b(&mut buf, b"%</text>");

        angle = end;
    }

    if !cfg.center_text.is_empty() {
        if !cfg.center_subtext.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy - 14.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"600\" font-size=\"10\" fill=\"#64748b\" letter-spacing=\"0.08em\">");
            escape_xml(&mut buf, cfg.center_subtext);
            push_b(&mut buf, b"</text>");
        }
        let main_size = (r_inner * 0.5).max(16.0).min(38.0);
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + main_size * 0.32);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"800\" font-size=\"");
        push_f2(&mut buf, main_size);
        push_b(&mut buf, b"\" fill=\"#0f172a\">");
        escape_xml(&mut buf, cfg.center_text);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
