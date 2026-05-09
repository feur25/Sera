use super::config::LollipopConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color, apply_sort, apply_sort_groups, truncate, svg_open_rescalable, svg_title};
use crate::html::hover::{build_chart_html, slots_to_json};

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub groups: Vec<String>,
    pub vmin: f64,
    pub vmax: f64,
}

pub fn prepare(cfg: &LollipopConfig) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return None; }
    let (labels, values, groups) = if cfg.groups.len() == n {
        apply_sort_groups(cfg.labels, cfg.values, cfg.groups, cfg.sort_order)
    } else {
        let (l, v) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
        (l, v, Vec::new())
    };
    let vmin = values[..n].iter().copied().fold(f64::INFINITY, f64::min);
    let vmax = values[..n].iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Some(Prepared { n, labels, values, groups, vmin, vmax })
}

pub fn open(cfg: &LollipopConfig, pad_l: i32, pad_t: i32, pad_r: i32, pad_b: i32) -> (Vec<u8>, i32, i32, i32, i32) {
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let mut b = Vec::<u8>::with_capacity(4096);
    svg_open_rescalable(&mut b, cfg.width, cfg.height, pad_l, pad_t, plot_w, plot_h);
    svg_title(&mut b, cfg.title, cfg.width / 2, 26);
    (b, pad_l, pad_t, plot_w, plot_h)
}

pub fn finalize(buf: Vec<u8>, cfg: &LollipopConfig) -> String {
    let mut buf = buf;
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

pub fn color_for(cfg: &LollipopConfig, p: &Prepared, i: usize) -> u32 {
    if !cfg.palette.is_empty() {
        if !p.groups.is_empty() {
            let mut uniq: Vec<&str> = Vec::new();
            for g in p.groups.iter() {
                let s = g.as_str();
                if !uniq.iter().any(|u| *u == s) { uniq.push(s); }
            }
            if let Some(pos) = uniq.iter().position(|u| *u == p.groups[i].as_str()) {
                return palette_color(cfg.palette, pos);
            }
        }
        return palette_color(cfg.palette, i);
    }
    cfg.color_hex
}

pub fn unique_groups(p: &Prepared) -> Vec<String> {
    let mut uniq: Vec<String> = Vec::new();
    for g in p.groups.iter() {
        if !uniq.iter().any(|u| u == g) { uniq.push(g.clone()); }
    }
    uniq
}

pub fn x_tick_label(buf: &mut Vec<u8>, x: i32, y: i32, label: &str) {
    push_b(buf, b"<text x=\""); push_i(buf, x);
    push_b(buf, b"\" y=\""); push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
    escape_xml(buf, truncate(label, 12));
    push_b(buf, b"</text>");
}

pub fn data_attrs(buf: &mut Vec<u8>, p: &Prepared, i: usize) {
    push_b(buf, b" data-idx=\""); push_i(buf, i as i32);
    push_b(buf, b"\" data-y=\""); push_f2(buf, p.values[i]);
    push_b(buf, b"\" data-lbl=\""); escape_xml(buf, &p.labels[i]); push_b(buf, b"\"");
}

pub fn dot(buf: &mut Vec<u8>, p: &Prepared, i: usize, cx: i32, cy: i32, r: i32, color: u32) {
    let hx = hex6(color);
    push_b(buf, b"<circle");
    data_attrs(buf, p, i);
    push_b(buf, b" cx=\""); push_i(buf, cx);
    push_b(buf, b"\" cy=\""); push_i(buf, cy);
    push_b(buf, b"\" r=\""); push_i(buf, r);
    push_b(buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
    push_b(buf, b"\"/>");
}



