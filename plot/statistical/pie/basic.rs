use super::config::PieConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, hex6, escape_xml, truncate, apply_sort};

pub struct PiePiece {
    pub area_x: f64,
    pub area_y: f64,
    pub area_w: f64,
    pub area_h: f64,
    pub donut: f64,
    pub radius_scale: f64,
    pub draw_legend: bool,
    pub title_top: bool,
    pub title: String,
    pub palette_offset: usize,
}

impl Default for PiePiece {
    fn default() -> Self {
        Self {
            area_x: 0.0, area_y: 0.0, area_w: 0.0, area_h: 0.0,
            donut: 0.0, radius_scale: 1.0,
            draw_legend: true, title_top: false,
            title: String::new(), palette_offset: 0,
        }
    }
}

pub fn render_pie_svg(
    buf: &mut Vec<u8>,
    cfg: &PieConfig,
    labels: &[String],
    values: &[f64],
    pull: &[f64],
    piece: &PiePiece,
) {
    use std::f64::consts::PI;
    let n = labels.len().min(values.len());
    if n == 0 { return; }
    let total: f64 = values[..n].iter().sum();
    if total <= 0.0 { return; }

    let chart_w = if piece.draw_legend { piece.area_w * 0.62 } else { piece.area_w };
    let cx = piece.area_x + chart_w * 0.50;
    let title_pad = if piece.title_top && !piece.title.is_empty() { 18.0 } else { 0.0 };
    let cy = piece.area_y + title_pad + (piece.area_h - title_pad) * 0.52;
    let r_base = (chart_w.min((piece.area_h - title_pad) * 0.90) * 0.42).max(1.0);
    let r = r_base * piece.radius_scale.max(0.05).min(1.0);
    let r_inner = if piece.donut > 0.0 { r * piece.donut.clamp(0.0, 0.90) } else { 0.0 };

    if piece.title_top && !piece.title.is_empty() {
        push_b(buf, b"<text x=\""); push_f2(buf, cx);
        push_b(buf, b"\" y=\""); push_f2(buf, piece.area_y + 14.0);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#cbd5e1\">");
        escape_xml(buf, &piece.title);
        push_b(buf, b"</text>");
    }

    let mut angle = -PI / 2.0;
    for i in 0..n {
        let frac = values[i] / total;
        let sweep = frac * 2.0 * PI;
        let end_angle = angle + sweep;
        let mid = angle + sweep / 2.0;
        let pull_v = pull.get(i).copied().unwrap_or(0.0).max(0.0).min(0.6);
        let off_x = pull_v * r * mid.cos();
        let off_y = pull_v * r * mid.sin();

        let c = palette_color(cfg.palette, i + piece.palette_offset);
        let hx = hex6(c);
        let large_arc: u8 = if sweep > PI { 1 } else { 0 };
        let cxx = cx + off_x;
        let cyy = cy + off_y;
        let x1 = cxx + r * angle.cos();
        let y1 = cyy + r * angle.sin();
        let x2 = cxx + r * end_angle.cos();
        let y2 = cyy + r * end_angle.sin();

        if r_inner > 0.0 {
            let xi1 = cxx + r_inner * angle.cos();
            let yi1 = cyy + r_inner * angle.sin();
            let xi2 = cxx + r_inner * end_angle.cos();
            let yi2 = cyy + r_inner * end_angle.sin();
            push_b(buf, b"<path data-idx=\""); push_i(buf, i as i32);
            push_b(buf, b"\" d=\"M"); push_f2(buf, x1);
            push_b(buf, b","); push_f2(buf, y1);
            push_b(buf, b" A"); push_f2(buf, r);
            push_b(buf, b","); push_f2(buf, r);
            push_b(buf, b" 0 "); buf.push(large_arc + b'0');
            push_b(buf, b",1 "); push_f2(buf, x2);
            push_b(buf, b","); push_f2(buf, y2);
            push_b(buf, b" L"); push_f2(buf, xi2);
            push_b(buf, b","); push_f2(buf, yi2);
            push_b(buf, b" A"); push_f2(buf, r_inner);
            push_b(buf, b","); push_f2(buf, r_inner);
            push_b(buf, b" 0 "); buf.push(large_arc + b'0');
            push_b(buf, b",0 "); push_f2(buf, xi1);
            push_b(buf, b","); push_f2(buf, yi1);
            push_b(buf, b" Z\" data-lbl=\""); escape_xml(buf, &labels[i]);
            push_b(buf, b"\" data-v=\""); push_f2(buf, values[i]);
            push_b(buf, b"\" data-kv-Part=\""); push_f2(buf, frac * 100.0);
            push_b(buf, b"%\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(buf, b"\" stroke=\"#0d1117\" stroke-width=\"1.5\"/>");
        } else {
            push_b(buf, b"<path data-idx=\""); push_i(buf, i as i32);
            push_b(buf, b"\" d=\"M"); push_f2(buf, cxx);
            push_b(buf, b","); push_f2(buf, cyy);
            push_b(buf, b" L"); push_f2(buf, x1);
            push_b(buf, b","); push_f2(buf, y1);
            push_b(buf, b" A"); push_f2(buf, r);
            push_b(buf, b","); push_f2(buf, r);
            push_b(buf, b" 0 "); buf.push(large_arc + b'0');
            push_b(buf, b",1 "); push_f2(buf, x2);
            push_b(buf, b","); push_f2(buf, y2);
            push_b(buf, b" Z\" data-lbl=\""); escape_xml(buf, &labels[i]);
            push_b(buf, b"\" data-v=\""); push_f2(buf, values[i]);
            push_b(buf, b"\" data-kv-Part=\""); push_f2(buf, frac * 100.0);
            push_b(buf, b"%\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(buf, b"\" stroke=\"#0d1117\" stroke-width=\"1.8\"/>");
        }

        if cfg.show_pct && frac >= cfg.min_label_frac {
            let lr = if r_inner > 0.0 { (r + r_inner) / 2.0 } else { r * 0.66 };
            push_b(buf, b"<text x=\""); push_f2(buf, cxx + lr * mid.cos());
            push_b(buf, b"\" y=\""); push_f2(buf, cyy + lr * mid.sin());
            push_b(buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#fff\" pointer-events=\"none\">");
            push_i(buf, (frac * 100.0 + 0.5) as i32);
            push_b(buf, b"%</text>");
        }
        angle = end_angle;
    }

    if piece.draw_legend {
        let leg_x = (piece.area_x + piece.area_w * 0.66) as i32;
        let leg_top = (piece.area_y + ((piece.area_h - n as f64 * 22.0) / 2.0).max(30.0)) as i32;
        for i in 0..n {
            let frac = values[i] / total;
            let c = palette_color(cfg.palette, i + piece.palette_offset);
            let hx = hex6(c);
            let ly = leg_top + i as i32 * 22;
            push_b(buf, b"<g data-legend=\"1\" data-series=\""); push_i(buf, i as i32);
            push_b(buf, b"\"><rect x=\""); push_i(buf, leg_x);
            push_b(buf, b"\" y=\""); push_i(buf, ly);
            push_b(buf, b"\" width=\"13\" height=\"13\" rx=\"3\" fill=\"#");
            buf.extend_from_slice(&hx); push_b(buf, b"\"/>");
            push_b(buf, b"<text x=\""); push_i(buf, leg_x + 17);
            push_b(buf, b"\" y=\""); push_i(buf, ly + 11);
            push_b(buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#cbd5e1\">");
            escape_xml(buf, truncate(&labels[i], 22));
            push_b(buf, b" ("); push_f2(buf, frac * 100.0);
            push_b(buf, b"%)</text></g>");
        }
    }
}

pub fn render_single(cfg: &PieConfig, donut_override: f64) -> String {
    let (sorted_labels, sorted_values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = sorted_labels.len().min(sorted_values.len());
    if n == 0 { return String::new(); }
    let total: f64 = sorted_values[..n].iter().sum();
    if total <= 0.0 { return String::new(); }
    let w = cfg.width;
    let h = cfg.height;

    let donut = if donut_override > 0.0 { donut_override } else { cfg.donut };

    let mut buf = Vec::<u8>::with_capacity(n * 380 + 1024);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, w); push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h); push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, w); push_b(&mut buf, b" ");
    push_i(&mut buf, h); push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#e2e8f0\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let title_pad = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let piece = PiePiece {
        area_x: 0.0,
        area_y: title_pad,
        area_w: w as f64,
        area_h: h as f64 - title_pad,
        donut,
        radius_scale: 1.0,
        draw_legend: true,
        title_top: false,
        title: String::new(),
        palette_offset: 0,
    };
    render_pie_svg(&mut buf, cfg, &sorted_labels, &sorted_values, cfg.pull, &piece);

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
