use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, push_hex, truncate};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct MultiLine;

pub struct MultiLineConfig<'a> {
    pub title: &'a str,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub x_labels: &'a [String],
    pub series: &'a [(String, Vec<f64>)],
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
    pub gridlines: bool,
    pub show_points: bool,
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for MultiLineConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            x_label: "",
            y_label: "",
            x_labels: &[],
            series: &[],
            palette: &[],
            width: 1100,
            height: 480,
            gridlines: true,
            show_points: true,
            hover: &[],
        }
    }
}

pub fn render_multiline_html(cfg: &MultiLineConfig) -> String {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser == 0 { return String::new(); }
    let max_val = cfg.series.iter()
        .flat_map(|(_, v)| v.iter().copied())
        .filter(|v| v.is_finite())
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let min_val = cfg.series.iter()
        .flat_map(|(_, v)| v.iter().copied())
        .filter(|v| v.is_finite())
        .fold(f64::INFINITY, f64::min)
        .min(0.0);
    let range = (max_val - min_val).max(1e-12);
    let legend_w: i32 = 160;
    let pad_l: i32 = 56;
    let pad_t: i32 = 42;
    let pad_b: i32 = 52;
    let plot_w = cfg.width - pad_l - legend_w;
    let plot_h = cfg.height - pad_t - pad_b;
    let step_x = plot_w as f64 / (n_pts - 1).max(1) as f64;
    let auto_hover = cfg.hover.is_empty();
    let n_total = n_pts * n_ser;
    let mut b = Vec::<u8>::with_capacity(n_total * 80 + 2048);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" data-sp=\"");
    push_i(&mut b, pad_l); b.push(b','); push_i(&mut b, pad_t);
    b.push(b','); push_i(&mut b, plot_w); b.push(b','); push_i(&mut b, plot_h);
    push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, (cfg.width - legend_w) / 2 + pad_l);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }
    let n_yticks: i32 = 6;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let vval = min_val + frac * range;
        if cfg.gridlines && i > 0 {
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, y);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, y);
            push_b(&mut b, b"\" stroke=\"#e5e7eb\" stroke-width=\".6\" stroke-dasharray=\"3,3\" class=\"sp-gl\"/>");
        }
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 4);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, y + 3);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-yt\">");
        push_f2(&mut b, vval);
        push_b(&mut b, b"</text>");
    }
    if !cfg.y_label.is_empty() {
        let ym = pad_t + plot_h / 2;
        push_b(&mut b, b"<text x=\"14\" y=\""); push_i(&mut b, ym);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" transform=\"rotate(-90,14,");
        push_i(&mut b, ym); push_b(&mut b, b")\">");
        escape_xml(&mut b, cfg.y_label);
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>");
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>");
    let tick_step = ((n_pts as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n_pts).step_by(tick_step) {
        let x = pad_l + (i as f64 * step_x) as i32;
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t + plot_h + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\">");
        escape_xml(&mut b, truncate(&cfg.x_labels[i], 12));
        push_b(&mut b, b"</text>");
    }
    if !cfg.x_label.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l + plot_w / 2);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, cfg.height - 6);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut b, cfg.x_label);
        push_b(&mut b, b"</text>");
    }
    for (si, (sname, svals)) in cfg.series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let mut sname_esc = Vec::with_capacity(sname.len() + 8);
        escape_xml(&mut sname_esc, sname);
        push_b(&mut b, b"<polyline data-series=\""); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" data-pts=\"");
        for i in 0..n_pts {
            let val = svals.get(i).copied().unwrap_or(0.0);
            if i > 0 { b.push(b','); }
            push_f2(&mut b, val);
        }
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\" points=\"");
        for i in 0..n_pts {
            let val = svals.get(i).copied().unwrap_or(0.0);
            let frac = if val.is_finite() { (val - min_val) / range } else { 0.0 };
            let x = pad_l + (i as f64 * step_x) as i32;
            let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
            if i > 0 { b.push(b' '); }
            push_i(&mut b, x); b.push(b','); push_i(&mut b, y);
        }
        push_b(&mut b, b"\"/>");
        if cfg.show_points {
            for i in 0..n_pts {
                let val = svals.get(i).copied().unwrap_or(0.0);
                let frac = if val.is_finite() { (val - min_val) / range } else { 0.0 };
                let x = pad_l + (i as f64 * step_x) as i32;
                let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
                let idx = (si * n_pts + i) as i32;
                push_b(&mut b, b"<circle data-series=\""); push_i(&mut b, si as i32);
                push_b(&mut b, b"\" data-idx=\""); push_i(&mut b, idx);
                push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, val);
                push_b(&mut b, b"\" data-lbl=\""); b.extend_from_slice(&sname_esc);
                push_b(&mut b, b"\" cx=\""); push_i(&mut b, x);
                push_b(&mut b, b"\" cy=\""); push_i(&mut b, y);
                push_b(&mut b, b"\" r=\"3\" fill=\"#"); b.extend_from_slice(&hx);
                push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
            }
        }
    }
    let leg_x = cfg.width - legend_w + 14;
    for (si, (sname, _)) in cfg.series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let ly = pad_t + 6 + si as i32 * 18;
        push_b(&mut b, b"<g data-legend=\"1\" data-series=\""); push_i(&mut b, si as i32);
        push_b(&mut b, b"\"><rect x=\""); push_i(&mut b, leg_x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly);
        push_b(&mut b, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#");
        b.extend_from_slice(&hx); push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, leg_x + 16);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly + 10);
        push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
        escape_xml(&mut b, truncate(sname, 18));
        push_b(&mut b, b"</text></g>");
    }
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}
