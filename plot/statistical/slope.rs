use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::build_chart_html;

pub struct SlopeConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub values_left: &'a [f64],
    pub values_right: &'a [f64],
    pub left_label: &'a str,
    pub right_label: &'a str,
    pub palette: &'a [u32],
    pub show_text: bool,
    pub width: i32,
    pub height: i32,
}

impl<'a> Default for SlopeConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            labels: &[],
            values_left: &[],
            values_right: &[],
            left_label: "Before",
            right_label: "After",
            palette: &[],
            show_text: true,
            width: 700,
            height: 500,
        }
    }
}

pub fn render_slope_html(cfg: &SlopeConfig) -> String {
    let n = cfg.labels.len().min(cfg.values_left.len()).min(cfg.values_right.len());
    if n == 0 { return String::new(); }
    let all: Vec<f64> = cfg.values_left[..n].iter()
        .chain(cfg.values_right[..n].iter())
        .copied()
        .collect();
    let min_val = all.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = all.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1e-12);
    let pad_l: i32 = 100;
    let pad_r: i32 = 100;
    let pad_t: i32 = 56;
    let pad_b: i32 = 30;
    let x_left  = pad_l;
    let x_right = cfg.width - pad_r;
    let plot_h  = cfg.height - pad_t - pad_b;
    let val_to_y = |v: f64| -> i32 {
        pad_t + ((1.0 - (v - min_val) / range) * plot_h as f64) as i32
    };
    let mut b = Vec::<u8>::with_capacity(n * 200 + 1024);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, x_left);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, x_left);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"1.2\" class=\"sp-ax-y\"/>");
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, x_right);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, x_right);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"1.2\" class=\"sp-ax-y\"/>");
    push_b(&mut b, b"<text x=\""); push_i(&mut b, x_left);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t - 10);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#374151\">");
    escape_xml(&mut b, cfg.left_label);
    push_b(&mut b, b"</text>");
    push_b(&mut b, b"<text x=\""); push_i(&mut b, x_right);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t - 10);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#374151\">");
    escape_xml(&mut b, cfg.right_label);
    push_b(&mut b, b"</text>");
    for i in 0..n {
        let y1 = val_to_y(cfg.values_left[i]);
        let y2 = val_to_y(cfg.values_right[i]);
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        let up = cfg.values_right[i] >= cfg.values_left[i];
        let stroke_color = if up { b"#10B981" as &[u8] } else { b"#F43F5E" };
        push_b(&mut b, b"<line data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &cfg.labels[i]);
        push_b(&mut b, b"\" x1=\""); push_i(&mut b, x_left);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, y1);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, x_right);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, y2);
        push_b(&mut b, b"\" stroke=\""); b.extend_from_slice(stroke_color);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linecap=\"round\" opacity=\"0.75\"/>");
        let _ = hx;
        let dot_color = if up { b"#10B981" as &[u8] } else { b"#F43F5E" };
        push_b(&mut b, b"<circle cx=\""); push_i(&mut b, x_left);
        push_b(&mut b, b"\" cy=\""); push_i(&mut b, y1);
        push_b(&mut b, b"\" r=\"4\" fill=\""); b.extend_from_slice(dot_color);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<circle cx=\""); push_i(&mut b, x_right);
        push_b(&mut b, b"\" cy=\""); push_i(&mut b, y2);
        push_b(&mut b, b"\" r=\"4\" fill=\""); b.extend_from_slice(dot_color);
        push_b(&mut b, b"\"/>");
        if cfg.show_text {
            push_b(&mut b, b"<text x=\""); push_i(&mut b, x_left - 6);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, y1 + 4);
            push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
            let lbl = &cfg.labels[i];
            let short = if lbl.len() > 14 { &lbl[..14] } else { lbl };
            escape_xml(&mut b, short);
            push_b(&mut b, b" (");
            push_f2(&mut b, cfg.values_left[i]);
            push_b(&mut b, b")");
            push_b(&mut b, b"</text>");
            push_b(&mut b, b"<text x=\""); push_i(&mut b, x_right + 6);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, y2 + 4);
            push_b(&mut b, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
            let lbl = &cfg.labels[i];
            let short = if lbl.len() > 14 { &lbl[..14] } else { lbl };
            escape_xml(&mut b, short);
            push_b(&mut b, b" (");
            push_f2(&mut b, cfg.values_right[i]);
            push_b(&mut b, b")");
            push_b(&mut b, b"</text>");
        }
    }
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
