use super::config::FunnelConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};

pub struct Layout {
    pub pad_l: i32,
    pub pad_r: i32,
    pub pad_t: i32,
    pub pad_b: i32,
    pub plot_w: i32,
    pub plot_h: i32,
    pub gap: i32,
    pub step_h: i32,
    pub cx: i32,
}

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub max_val: f64,
    pub layout: Layout,
}

pub fn prepare(cfg: &FunnelConfig) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return None;
    }
    let (labels, values) = apply_sort(&cfg.labels[..n], &cfg.values[..n], cfg.sort_order);
    let max_val = values.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
    let pad_l: i32 = 80;
    let pad_r: i32 = 80;
    let pad_t: i32 = 46;
    let pad_b: i32 = 20;
    let gap: i32 = 3;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let step_h = (plot_h - gap * (n as i32 - 1).max(0)) / n as i32;
    let cx = cfg.width / 2;
    Some(Prepared {
        n,
        labels,
        values,
        max_val,
        layout: Layout {
            pad_l,
            pad_r,
            pad_t,
            pad_b,
            plot_w,
            plot_h,
            gap,
            step_h,
            cx,
        },
    })
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &FunnelConfig) {
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

pub fn fmt_value(buf: &mut Vec<u8>, v: f64) {
    if v >= 1_000_000.0 {
        push_f2(buf, v / 1_000_000.0);
        push_b(buf, b"M");
    } else if v >= 1000.0 {
        push_f2(buf, v / 1000.0);
        push_b(buf, b"k");
    } else {
        push_f2(buf, v);
    }
}

pub fn label_inside(buf: &mut Vec<u8>, cx: i32, cy: i32, lbl: &str) {
    push_b(buf, b"<text x=\"");
    push_i(buf, cx);
    push_b(buf, b"\" y=\"");
    push_i(buf, cy + 4);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#ffffff\" pointer-events=\"none\">");
    escape_xml(buf, truncate(lbl, 18));
    push_b(buf, b"</text>");
}

pub fn label_left_pct(buf: &mut Vec<u8>, x: i32, cy: i32, val: f64, max_val: f64) {
    let pct = if max_val > 0.0 {
        (val / max_val) * 100.0
    } else {
        0.0
    };
    push_b(buf, b"<text x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, cy + 4);
    push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
    push_f2(buf, pct);
    push_b(buf, b"%</text>");
}

pub fn label_right_val(buf: &mut Vec<u8>, x: i32, cy: i32, val: f64) {
    push_b(buf, b"<text x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, cy + 4);
    push_b(buf, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
    fmt_value(buf, val);
    push_b(buf, b"</text>");
}

pub fn finalize(mut buf: Vec<u8>, cfg: &FunnelConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

pub fn step_color(palette: &[u32], i: usize) -> [u8; 6] {
    hex6(palette_color(palette, i))
}
