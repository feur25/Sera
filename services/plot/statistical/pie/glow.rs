use super::common::{open_svg, write_title};
use super::config::PieConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};
use std::f64::consts::PI;

#[crate::chart_demo(
    "title=\"Project Time Allocation\", labels=[\"Ecrire du code\",\"Chercher le bug\",\"Reunions utiles\",\"Pause cafe\"], values=[45,30,15,10], center_text=\"100%\", center_subtext=\"TOTAL PROJET\", variant=\"glow\""
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
    let mut buf = Vec::<u8>::with_capacity(n * 420 + 2048);
    open_svg(&mut buf, w, h);
    write_title(&mut buf, w, cfg.title);

    let cx = w as f64 / 2.0;
    let cy = title_pad + (h as f64 - title_pad) / 2.0;
    let r = ((w.min(h) as f64) * 0.5 - 88.0).max(40.0);
    let r_inner = r * if cfg.donut > 0.0 { cfg.donut.clamp(0.3, 0.85) } else { 0.62 };

    push_b(
        &mut buf,
        b"<defs><filter id=\"sp-glow\" x=\"-60%\" y=\"-60%\" width=\"220%\" height=\"220%\">\
        <feGaussianBlur stdDeviation=\"6\" result=\"b\"/>\
        <feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter></defs>",
    );

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

        push_b(&mut buf, b"<path filter=\"url(#sp-glow)\" data-idx=\"");
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
        push_b(&mut buf, b" Z\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"none\"/>");

        let lr = r + 46.0;
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
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1\" opacity=\"0.6\"/>");

        let right = mid.cos() >= 0.0;
        let anchor: &[u8] = if right { b"start" } else { b"end" };
        let tx = if right { lx + 6.0 } else { lx - 6.0 };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly - 3.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\">");
        push_i(&mut buf, (frac * 100.0 + 0.5) as i32);
        push_b(&mut buf, b"%</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 11.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#64748b\">");
        escape_xml(&mut buf, truncate(&labels[i], 16));
        push_b(&mut buf, b"</text>");

        angle = end;
    }

    if !cfg.center_text.is_empty() {
        let main_size = (r_inner * 0.55).max(16.0).min(40.0);
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy - 6.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"800\" font-size=\"");
        push_f2(&mut buf, main_size);
        push_b(&mut buf, b"\" fill=\"#0f172a\">");
        escape_xml(&mut buf, cfg.center_text);
        push_b(&mut buf, b"</text>");
        if !cfg.center_subtext.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy + main_size * 0.7);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"600\" font-size=\"10\" fill=\"#64748b\" letter-spacing=\"0.06em\">");
            escape_xml(&mut buf, cfg.center_subtext);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
