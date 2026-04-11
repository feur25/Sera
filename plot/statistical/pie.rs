use super::common::{palette_color, push_b, push_i, push_f2, hex6, escape_xml, truncate, apply_sort};
use crate::html::hover::{slots_to_json, build_chart_html};

pub struct Pie;

crate::chart_config!(PieConfig, 720, 440;
    struct {
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub donut: f64,
        pub show_pct: bool,
        pub min_label_frac: f64,
        pub palette: &'a [u32],
    }
    defaults {
        labels: &[],
        values: &[],
        donut: 0.0,
        show_pct: true,
        min_label_frac: 0.04,
        palette: &[],
    }
);

pub fn render_pie_html(cfg: &PieConfig) -> String {
    use std::f64::consts::PI;
    let (sorted_labels, sorted_values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let labels = &sorted_labels;
    let values = &sorted_values;
    let n = labels.len().min(values.len());
    if n == 0 { return String::new(); }
    let total: f64 = values[..n].iter().sum();
    if total <= 0.0 { return String::new(); }
    let w = cfg.width;
    let h = cfg.height;
    let chart_w = w as f64 * 0.62;
    let cx = chart_w * 0.50;
    let cy = h as f64 * 0.52;
    let r = (cx.min(cy * 0.90) * 0.84).max(1.0);
    let r_inner = if cfg.donut > 0.0 { r * cfg.donut.clamp(0.0, 0.90) } else { 0.0 };
    let mut buf = Vec::<u8>::with_capacity(n * 380 + 1024);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, w); push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h); push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, w); push_b(&mut buf, b" ");
    push_i(&mut buf, h); push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    let mut angle = -PI / 2.0;
    for i in 0..n {
        let frac = values[i] / total;
        let sweep = frac * 2.0 * PI;
        let end_angle = angle + sweep;
        let c = palette_color(cfg.palette, i);
        let hx = hex6(c);
        let large_arc: u8 = if sweep > PI { 1 } else { 0 };
        let x1 = cx + r * angle.cos();
        let y1 = cy + r * angle.sin();
        let x2 = cx + r * end_angle.cos();
        let y2 = cy + r * end_angle.sin();
        if r_inner > 0.0 {
            let xi1 = cx + r_inner * angle.cos();
            let yi1 = cy + r_inner * angle.sin();
            let xi2 = cx + r_inner * end_angle.cos();
            let yi2 = cy + r_inner * end_angle.sin();
            push_b(&mut buf, b"<path data-idx=\""); push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" d=\"M"); push_f2(&mut buf, x1);
            push_b(&mut buf, b","); push_f2(&mut buf, y1);
            push_b(&mut buf, b" A"); push_f2(&mut buf, r);
            push_b(&mut buf, b","); push_f2(&mut buf, r);
            push_b(&mut buf, b" 0 "); buf.push(large_arc + b'0');
            push_b(&mut buf, b",1 "); push_f2(&mut buf, x2);
            push_b(&mut buf, b","); push_f2(&mut buf, y2);
            push_b(&mut buf, b" L"); push_f2(&mut buf, xi2);
            push_b(&mut buf, b","); push_f2(&mut buf, yi2);
            push_b(&mut buf, b" A"); push_f2(&mut buf, r_inner);
            push_b(&mut buf, b","); push_f2(&mut buf, r_inner);
            push_b(&mut buf, b" 0 "); buf.push(large_arc + b'0');
            push_b(&mut buf, b",0 "); push_f2(&mut buf, xi1);
            push_b(&mut buf, b","); push_f2(&mut buf, yi1);
            push_b(&mut buf, b" Z\" data-lbl=\""); escape_xml(&mut buf, &labels[i]);
            push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, values[i]);
            push_b(&mut buf, b"\" data-kv-Part=\""); push_f2(&mut buf, frac * 100.0);
            push_b(&mut buf, b"%\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.5\"/>");
        } else {
            push_b(&mut buf, b"<path data-idx=\""); push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" d=\"M"); push_f2(&mut buf, cx);
            push_b(&mut buf, b","); push_f2(&mut buf, cy);
            push_b(&mut buf, b" L"); push_f2(&mut buf, x1);
            push_b(&mut buf, b","); push_f2(&mut buf, y1);
            push_b(&mut buf, b" A"); push_f2(&mut buf, r);
            push_b(&mut buf, b","); push_f2(&mut buf, r);
            push_b(&mut buf, b" 0 "); buf.push(large_arc + b'0');
            push_b(&mut buf, b",1 "); push_f2(&mut buf, x2);
            push_b(&mut buf, b","); push_f2(&mut buf, y2);
            push_b(&mut buf, b" Z\" data-lbl=\""); escape_xml(&mut buf, &labels[i]);
            push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, values[i]);
            push_b(&mut buf, b"\" data-kv-Part=\""); push_f2(&mut buf, frac * 100.0);
            push_b(&mut buf, b"%\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.8\"/>");
        }
        if cfg.show_pct && frac >= cfg.min_label_frac {
            let mid = angle + sweep / 2.0;
            let lr = if r_inner > 0.0 { (r + r_inner) / 2.0 } else { r * 0.66 };
            push_b(&mut buf, b"<text x=\""); push_f2(&mut buf, cx + lr * mid.cos());
            push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, cy + lr * mid.sin());
            push_b(&mut buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#fff\">");
            push_i(&mut buf, (frac * 100.0 + 0.5) as i32);
            push_b(&mut buf, b"%</text>");
        }
        angle = end_angle;
    }
    if r_inner > 0.0 {
        push_b(&mut buf, b"<circle cx=\""); push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, r_inner - 1.0);
        push_b(&mut buf, b"\" fill=\"#fff\"/>");
    }
    let leg_x = (w as f64 * 0.66) as i32;
    let leg_top = ((h as f64 - n as f64 * 22.0) / 2.0).max(30.0) as i32;
    for i in 0..n {
        let frac = values[i] / total;
        let c = palette_color(cfg.palette, i);
        let hx = hex6(c);
        let ly = leg_top + i as i32 * 22;
        push_b(&mut buf, b"<g data-legend=\"1\" data-series=\""); push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"><rect x=\""); push_i(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, ly);
        push_b(&mut buf, b"\" width=\"13\" height=\"13\" rx=\"3\" fill=\"#");
        buf.extend_from_slice(&hx); push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, leg_x + 17);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, ly + 11);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, truncate(&labels[i], 22));
        push_b(&mut buf, b" ("); push_f2(&mut buf, frac * 100.0);
        push_b(&mut buf, b"%)</text></g>");
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
