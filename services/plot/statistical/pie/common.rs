use super::config::PieConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};

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
    pub arc_start: f64,
    pub arc_span: f64,
    pub pattern: String,
    pub pattern_id_offset: usize,
    pub center_text: String,
    pub center_subtext: String,
}

impl Default for PiePiece {
    fn default() -> Self {
        Self {
            area_x: 0.0,
            area_y: 0.0,
            area_w: 0.0,
            area_h: 0.0,
            donut: 0.0,
            radius_scale: 1.0,
            draw_legend: true,
            title_top: false,
            title: String::new(),
            palette_offset: 0,
            arc_start: -std::f64::consts::PI / 2.0,
            arc_span: 2.0 * std::f64::consts::PI,
            pattern: String::new(),
            pattern_id_offset: 0,
            center_text: String::new(),
            center_subtext: String::new(),
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
    if n == 0 {
        return;
    }
    let total: f64 = values[..n].iter().sum();
    if total <= 0.0 {
        return;
    }

    let chart_w = if piece.draw_legend {
        piece.area_w * 0.62
    } else {
        piece.area_w
    };
    let cx = piece.area_x + chart_w * 0.50;
    let title_pad = if piece.title_top && !piece.title.is_empty() {
        18.0
    } else {
        0.0
    };
    let span = if piece.arc_span > 0.0 && piece.arc_span <= 2.0 * PI {
        piece.arc_span
    } else {
        2.0 * PI
    };
    let is_partial = span < 2.0 * PI - 0.001;
    let (cy, r_base) = if is_partial {
        let cy_p = piece.area_y + title_pad + (piece.area_h - title_pad) * 0.82;
        let r_p = (chart_w.min((piece.area_h - title_pad) * 1.7) * 0.46).max(1.0);
        (cy_p, r_p)
    } else {
        let cy_n = piece.area_y + title_pad + (piece.area_h - title_pad) * 0.52;
        let r_n = (chart_w.min((piece.area_h - title_pad) * 0.90) * 0.42).max(1.0);
        (cy_n, r_n)
    };
    let r = r_base * piece.radius_scale.max(0.05).min(1.0);
    let r_inner = if piece.donut > 0.0 {
        r * piece.donut.clamp(0.0, 0.90)
    } else {
        0.0
    };

    if !piece.pattern.is_empty() {
        push_b(buf, b"<defs>");
        for i in 0..n {
            let c = palette_color(cfg.palette, i + piece.palette_offset);
            let hx = hex6(c);
            push_b(buf, b"<pattern id=\"sp-pat-");
            push_i(buf, (i + piece.pattern_id_offset) as i32);
            push_b(buf, b"\" patternUnits=\"userSpaceOnUse\" width=\"10\" height=\"10\"><rect width=\"10\" height=\"10\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(buf, b"\"/>");
            match piece.pattern.as_str() {
                "dots" => {
                    push_b(
                        buf,
                        b"<circle cx=\"5\" cy=\"5\" r=\"1.6\" fill=\"#0d1117\" opacity=\"0.55\"/>",
                    );
                }
                "diagonal" => {
                    push_b(buf, b"<path d=\"M0,10 L10,0\" stroke=\"#0d1117\" stroke-width=\"1.4\" opacity=\"0.55\"/>");
                }
                "cross" => {
                    push_b(buf, b"<path d=\"M0,5 L10,5 M5,0 L5,10\" stroke=\"#0d1117\" stroke-width=\"1.2\" opacity=\"0.5\"/>");
                }
                _ => {
                    push_b(buf, b"<path d=\"M0,3 L10,3 M0,7 L10,7\" stroke=\"#0d1117\" stroke-width=\"1.4\" opacity=\"0.55\"/>");
                }
            }
            push_b(buf, b"</pattern>");
        }
        push_b(buf, b"</defs>");
    }

    if piece.title_top && !piece.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_f2(buf, cx);
        push_b(buf, b"\" y=\"");
        push_f2(buf, piece.area_y + 14.0);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#cbd5e1\">");
        escape_xml(buf, &piece.title);
        push_b(buf, b"</text>");
    }

    let mut angle = piece.arc_start;
    for i in 0..n {
        let frac = values[i] / total;
        let sweep = frac * span;
        let end_angle = angle + sweep;
        let mid = angle + sweep / 2.0;
        let pull_v = pull.get(i).copied().unwrap_or(0.0).max(0.0).min(0.6);
        let off_x = pull_v * r * mid.cos();
        let off_y = pull_v * r * mid.sin();

        let c = palette_color(cfg.palette, i + piece.palette_offset);
        let hx = hex6(c);
        let use_pattern = !piece.pattern.is_empty();
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
            push_b(buf, b"<path data-idx=\"");
            push_i(buf, i as i32);
            push_b(buf, b"\" d=\"M");
            push_f2(buf, x1);
            push_b(buf, b",");
            push_f2(buf, y1);
            push_b(buf, b" A");
            push_f2(buf, r);
            push_b(buf, b",");
            push_f2(buf, r);
            push_b(buf, b" 0 ");
            buf.push(large_arc + b'0');
            push_b(buf, b",1 ");
            push_f2(buf, x2);
            push_b(buf, b",");
            push_f2(buf, y2);
            push_b(buf, b" L");
            push_f2(buf, xi2);
            push_b(buf, b",");
            push_f2(buf, yi2);
            push_b(buf, b" A");
            push_f2(buf, r_inner);
            push_b(buf, b",");
            push_f2(buf, r_inner);
            push_b(buf, b" 0 ");
            buf.push(large_arc + b'0');
            push_b(buf, b",0 ");
            push_f2(buf, xi1);
            push_b(buf, b",");
            push_f2(buf, yi1);
            push_b(buf, b" Z\" data-lbl=\"");
            escape_xml(buf, &labels[i]);
            push_b(buf, b"\" data-v=\"");
            push_f2(buf, values[i]);
            push_b(buf, b"\" data-kv-Part=\"");
            push_f2(buf, frac * 100.0);
            if use_pattern {
                push_b(buf, b"%\" fill=\"url(#sp-pat-");
                push_i(buf, (i + piece.pattern_id_offset) as i32);
                push_b(buf, b")\" stroke=\"#0d1117\" stroke-width=\"1.5\"/>");
            } else {
                push_b(buf, b"%\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(buf, b"\" stroke=\"#0d1117\" stroke-width=\"1.5\"/>");
            }
        } else {
            push_b(buf, b"<path data-idx=\"");
            push_i(buf, i as i32);
            push_b(buf, b"\" d=\"M");
            push_f2(buf, cxx);
            push_b(buf, b",");
            push_f2(buf, cyy);
            push_b(buf, b" L");
            push_f2(buf, x1);
            push_b(buf, b",");
            push_f2(buf, y1);
            push_b(buf, b" A");
            push_f2(buf, r);
            push_b(buf, b",");
            push_f2(buf, r);
            push_b(buf, b" 0 ");
            buf.push(large_arc + b'0');
            push_b(buf, b",1 ");
            push_f2(buf, x2);
            push_b(buf, b",");
            push_f2(buf, y2);
            push_b(buf, b" Z\" data-lbl=\"");
            escape_xml(buf, &labels[i]);
            push_b(buf, b"\" data-v=\"");
            push_f2(buf, values[i]);
            push_b(buf, b"\" data-kv-Part=\"");
            push_f2(buf, frac * 100.0);
            if use_pattern {
                push_b(buf, b"%\" fill=\"url(#sp-pat-");
                push_i(buf, (i + piece.pattern_id_offset) as i32);
                push_b(buf, b")\" stroke=\"#0d1117\" stroke-width=\"1.8\"/>");
            } else {
                push_b(buf, b"%\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(buf, b"\" stroke=\"#0d1117\" stroke-width=\"1.8\"/>");
            }
        }

        if cfg.show_pct && frac >= cfg.min_label_frac {
            let lr = if r_inner > 0.0 {
                (r + r_inner) / 2.0
            } else {
                r * 0.66
            };
            push_b(buf, b"<text x=\"");
            push_f2(buf, cxx + lr * mid.cos());
            push_b(buf, b"\" y=\"");
            push_f2(buf, cyy + lr * mid.sin());
            push_b(buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#fff\" pointer-events=\"none\">");
            push_i(buf, (frac * 100.0 + 0.5) as i32);
            push_b(buf, b"%</text>");
        }
        angle = end_angle;
    }

    if !piece.center_text.is_empty() {
        let main_size = if r_inner > 0.0 {
            (r_inner * 0.55).max(14.0).min(36.0)
        } else {
            22.0
        };
        push_b(buf, b"<text x=\"");
        push_f2(buf, cx);
        push_b(buf, b"\" y=\"");
        push_f2(buf, cy);
        push_b(buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"800\" font-size=\"");
        push_f2(buf, main_size);
        push_b(buf, b"\" fill=\"#f1f5f9\" pointer-events=\"none\">");
        escape_xml(buf, &piece.center_text);
        push_b(buf, b"</text>");
        if !piece.center_subtext.is_empty() {
            push_b(buf, b"<text x=\"");
            push_f2(buf, cx);
            push_b(buf, b"\" y=\"");
            push_f2(buf, cy + main_size * 0.85);
            push_b(buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" font-weight=\"600\" font-size=\"11\" fill=\"#94a3b8\" letter-spacing=\"0.08em\" pointer-events=\"none\">");
            escape_xml(buf, &piece.center_subtext);
            push_b(buf, b"</text>");
        }
    }

    if piece.draw_legend {
        let leg_x = (piece.area_x + piece.area_w * 0.66) as i32;
        let leg_top = (piece.area_y + ((piece.area_h - n as f64 * 22.0) / 2.0).max(30.0)) as i32;
        for i in 0..n {
            let frac = values[i] / total;
            let c = palette_color(cfg.palette, i + piece.palette_offset);
            let hx = hex6(c);
            let ly = leg_top + i as i32 * 22;
            push_b(buf, b"<g data-legend=\"1\" style=\"display:none\" data-series=\"");
            push_i(buf, i as i32);
            push_b(buf, b"\"><rect x=\"");
            push_i(buf, leg_x);
            push_b(buf, b"\" y=\"");
            push_i(buf, ly);
            push_b(buf, b"\" width=\"13\" height=\"13\" rx=\"3\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(buf, b"\"/>");
            push_b(buf, b"<text x=\"");
            push_i(buf, leg_x + 17);
            push_b(buf, b"\" y=\"");
            push_i(buf, ly + 11);
            push_b(
                buf,
                b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#cbd5e1\">",
            );
            escape_xml(buf, truncate(&labels[i], 22));
            push_b(buf, b" (");
            push_f2(buf, frac * 100.0);
            push_b(buf, b"%)</text></g>");
        }
    }
}

pub fn open_svg(buf: &mut Vec<u8>, w: i32, h: i32) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, w);
    push_b(buf, b"\" height=\"");
    push_i(buf, h);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, w);
    push_b(buf, b" ");
    push_i(buf, h);
    push_b(buf, b"\">");
    push_b(
        buf,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
}

pub fn write_title(buf: &mut Vec<u8>, w: i32, title: &str) {
    if title.is_empty() {
        return;
    }
    push_b(buf, b"<text x=\"");
    push_i(buf, w / 2);
    push_b(buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#e2e8f0\">");
    escape_xml(buf, title);
    push_b(buf, b"</text>");
}

pub fn render_with<F>(cfg: &PieConfig, pull: &[f64], customize: F) -> String
where
    F: FnOnce(&mut PiePiece, &PieConfig),
{
    let (sorted_labels, sorted_values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = sorted_labels.len().min(sorted_values.len());
    if n == 0 {
        return String::new();
    }
    let total: f64 = sorted_values[..n].iter().sum();
    if total <= 0.0 {
        return String::new();
    }
    let w = cfg.width;
    let h = cfg.height;

    let mut buf = Vec::<u8>::with_capacity(n * 380 + 1024);
    open_svg(&mut buf, w, h);
    write_title(&mut buf, w, cfg.title);

    let title_pad = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let mut piece = PiePiece {
        area_x: 0.0,
        area_y: title_pad,
        area_w: w as f64,
        area_h: h as f64 - title_pad,
        donut: cfg.donut,
        pattern: cfg.pattern.to_string(),
        center_text: cfg.center_text.to_string(),
        center_subtext: cfg.center_subtext.to_string(),
        ..PiePiece::default()
    };
    customize(&mut piece, cfg);
    render_pie_svg(&mut buf, cfg, &sorted_labels, &sorted_values, pull, &piece);

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
