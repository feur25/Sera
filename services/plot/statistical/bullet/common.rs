use super::config::BulletConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, hex6, push_b, push_f2, push_i, sort_indices, sorted, truncate,
};

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub targets: Vec<f64>,
    pub max_vals: Vec<f64>,
    pub ranges: Vec<f64>,
    pub comparisons: Vec<f64>,
    pub auto_h: i32,
    pub pad_l: i32,
    pub pad_r: i32,
    pub pad_t: i32,
    pub row_h: i32,
    pub plot_w: i32,
}

pub fn prepare(cfg: &BulletConfig) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return None;
    }
    let idx = sort_indices(n, cfg.values, cfg.labels, cfg.sort_order);
    let labels = sorted(&idx, cfg.labels);
    let values = sorted(&idx, cfg.values);
    let targets: Vec<f64> = idx
        .iter()
        .map(|&i| cfg.targets.get(i).copied().unwrap_or(0.0))
        .collect();
    let max_vals: Vec<f64> = idx
        .iter()
        .map(|&i| cfg.max_vals.get(i).copied().unwrap_or(0.0))
        .collect();
    let ranges: Vec<f64> = idx
        .iter()
        .map(|&i| cfg.ranges.get(i).copied().unwrap_or(0.0))
        .collect();
    let comparisons: Vec<f64> = idx
        .iter()
        .map(|&i| cfg.comparisons.get(i).copied().unwrap_or(0.0))
        .collect();
    let auto_h = if cfg.height < n as i32 * 50 + 60 {
        n as i32 * 50 + 60
    } else {
        cfg.height
    };
    let pad_l = 130;
    let pad_r = 30;
    let pad_t = if cfg.title.is_empty() { 20 } else { 46 };
    let row_h = (auto_h - pad_t - 20) / n as i32;
    let plot_w = cfg.width - pad_l - pad_r;
    Some(Prepared {
        n,
        labels,
        values,
        targets,
        max_vals,
        ranges,
        comparisons,
        auto_h,
        pad_l,
        pad_r,
        pad_t,
        row_h,
        plot_w,
    })
}

pub fn max_for(p: &Prepared, i: usize) -> f64 {
    if p.max_vals[i] > 0.0 {
        p.max_vals[i]
    } else {
        (p.values[i].max(p.targets[i]).max(p.comparisons[i]) * 1.2).max(1.0)
    }
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &BulletConfig, p: &Prepared) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, p.auto_h);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, p.auto_h);
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

pub fn label_left(buf: &mut Vec<u8>, p: &Prepared, i: usize, by: i32, bar_h: i32) {
    push_b(buf, b"<text x=\"");
    push_i(buf, p.pad_l - 6);
    push_b(buf, b"\" y=\"");
    push_i(buf, by + bar_h / 2 + 4);
    push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
    escape_xml(buf, truncate(&p.labels[i], 16));
    push_b(buf, b"</text>");
}

pub fn value_text(buf: &mut Vec<u8>, v: f64, x: i32, y: i32) {
    push_b(buf, b"<text x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(
        buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">",
    );
    if v.abs() >= 1_000_000.0 {
        push_f2(buf, v / 1_000_000.0);
        push_b(buf, b"M");
    } else if v.abs() >= 1_000.0 {
        push_f2(buf, v / 1_000.0);
        push_b(buf, b"k");
    } else {
        push_f2(buf, v);
    }
    push_b(buf, b"</text>");
}

pub fn data_value_rect(
    buf: &mut Vec<u8>,
    p: &Prepared,
    i: usize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: u32,
    rx: i32,
    opacity: f32,
) {
    let hx = hex6(color);
    push_b(buf, b"<rect data-idx=\"");
    push_i(buf, i as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, p.values[i]);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, &p.labels[i]);
    push_b(buf, b"\" x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" width=\"");
    push_i(buf, w.max(2));
    push_b(buf, b"\" height=\"");
    push_i(buf, h);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hx);
    push_b(buf, b"\" rx=\"");
    push_i(buf, rx);
    push_b(buf, b"\" opacity=\"");
    push_f2(buf, opacity as f64);
    push_b(buf, b"\"/>");
}

pub fn finalize(buf: Vec<u8>, cfg: &BulletConfig) -> String {
    let mut buf = buf;
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
