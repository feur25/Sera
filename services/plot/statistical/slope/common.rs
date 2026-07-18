use super::config::SlopeConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, push_b, push_f2, push_i, sort_indices, sorted, truncate,
};

pub struct Layout {
    pub pad_l: i32,
    pub pad_r: i32,
    pub pad_t: i32,
    pub pad_b: i32,
    pub x_left: i32,
    pub x_right: i32,
    pub plot_h: i32,
}

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values_left: Vec<f64>,
    pub values_right: Vec<f64>,
    pub min_val: f64,
    pub max_val: f64,
    pub range: f64,
    pub layout: Layout,
}

pub fn prepare(cfg: &SlopeConfig) -> Option<Prepared> {
    let n = cfg
        .labels
        .len()
        .min(cfg.values_left.len())
        .min(cfg.values_right.len());
    if n == 0 {
        return None;
    }
    let sort_idx = sort_indices(n, cfg.values_left, cfg.labels, cfg.sort_order);
    let labels = sorted(&sort_idx, cfg.labels);
    let values_left = sorted(&sort_idx, cfg.values_left);
    let values_right = sorted(&sort_idx, cfg.values_right);
    let all: Vec<f64> = values_left
        .iter()
        .chain(values_right.iter())
        .copied()
        .collect();
    let min_val = all.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = all.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1e-12);
    let pad_l: i32 = 100;
    let pad_r: i32 = 100;
    let pad_t: i32 = 56;
    let pad_b: i32 = 30;
    let x_left = pad_l;
    let x_right = cfg.width - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    Some(Prepared {
        n,
        labels,
        values_left,
        values_right,
        min_val,
        max_val,
        range,
        layout: Layout {
            pad_l,
            pad_r,
            pad_t,
            pad_b,
            x_left,
            x_right,
            plot_h,
        },
    })
}

pub fn val_to_y(p: &Prepared, v: f64) -> i32 {
    p.layout.pad_t + ((1.0 - (v - p.min_val) / p.range) * p.layout.plot_h as f64) as i32
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &SlopeConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\">");
    push_b(
        buf,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn draw_axes(buf: &mut Vec<u8>, cfg: &SlopeConfig, p: &Prepared) {
    let l = &p.layout;
    push_b(buf, b"<line x1=\"");
    push_i(buf, l.x_left);
    push_b(buf, b"\" y1=\"");
    push_i(buf, l.pad_t);
    push_b(buf, b"\" x2=\"");
    push_i(buf, l.x_left);
    push_b(buf, b"\" y2=\"");
    push_i(buf, l.pad_t + l.plot_h);
    push_b(
        buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"1.2\" class=\"sp-ax-y\"/>",
    );
    push_b(buf, b"<line x1=\"");
    push_i(buf, l.x_right);
    push_b(buf, b"\" y1=\"");
    push_i(buf, l.pad_t);
    push_b(buf, b"\" x2=\"");
    push_i(buf, l.x_right);
    push_b(buf, b"\" y2=\"");
    push_i(buf, l.pad_t + l.plot_h);
    push_b(
        buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"1.2\" class=\"sp-ax-y\"/>",
    );
    push_b(buf, b"<text x=\"");
    push_i(buf, l.x_left);
    push_b(buf, b"\" y=\"");
    push_i(buf, l.pad_t - 10);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#374151\">");
    escape_xml(buf, cfg.left_label);
    push_b(buf, b"</text>");
    push_b(buf, b"<text x=\"");
    push_i(buf, l.x_right);
    push_b(buf, b"\" y=\"");
    push_i(buf, l.pad_t - 10);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#374151\">");
    escape_xml(buf, cfg.right_label);
    push_b(buf, b"</text>");
}

pub fn dot(buf: &mut Vec<u8>, cx: i32, cy: i32, color: &[u8], r: f64) {
    push_b(buf, b"<circle cx=\"");
    push_i(buf, cx);
    push_b(buf, b"\" cy=\"");
    push_i(buf, cy);
    push_b(buf, b"\" r=\"");
    push_f2(buf, r);
    push_b(buf, b"\" fill=\"");
    buf.extend_from_slice(color);
    push_b(buf, b"\"/>");
}

pub fn label_left(buf: &mut Vec<u8>, x: i32, y: i32, lbl: &str, val: f64) {
    push_b(buf, b"<text x=\"");
    push_i(buf, x - 6);
    push_b(buf, b"\" y=\"");
    push_i(buf, y + 4);
    push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
    escape_xml(buf, truncate(lbl, 14));
    push_b(buf, b" (");
    push_f2(buf, val);
    push_b(buf, b")</text>");
}

pub fn label_right(buf: &mut Vec<u8>, x: i32, y: i32, lbl: &str, val: f64) {
    push_b(buf, b"<text x=\"");
    push_i(buf, x + 6);
    push_b(buf, b"\" y=\"");
    push_i(buf, y + 4);
    push_b(buf, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
    escape_xml(buf, truncate(lbl, 14));
    push_b(buf, b" (");
    push_f2(buf, val);
    push_b(buf, b")</text>");
}

pub fn finalize(mut buf: Vec<u8>, cfg: &SlopeConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
