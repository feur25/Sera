use super::config::DumbbellConfig;
use crate::plot::statistical::common::{sorted, sort_indices, push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_legend_item, Frame};
use crate::html::hover::slots_to_json;

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub vmin: f64,
    pub vmax: f64,
    pub vrange: f64,
    pub c1: u32,
    pub c2: u32,
}

pub fn prepare(cfg: &DumbbellConfig) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values_start.len()).min(cfg.values_end.len());
    if n == 0 { return None; }
    let idx = sort_indices(n, cfg.values_start, cfg.labels, cfg.sort_order);
    let labels = sorted(&idx, cfg.labels);
    let start = sorted(&idx, cfg.values_start);
    let end = sorted(&idx, cfg.values_end);
    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for i in 0..n {
        let lo = start[i].min(end[i]);
        let hi = start[i].max(end[i]);
        if lo < vmin { vmin = lo; }
        if hi > vmax { vmax = hi; }
    }
    if vmin > 0.0 { vmin = 0.0; }
    let vrange = (vmax - vmin).max(1.0);
    let c1 = if !cfg.palette.is_empty() { cfg.palette[0] } else { 0x6366F1 };
    let c2 = if cfg.palette.len() >= 2 { cfg.palette[1] } else { 0x22D3EE };
    Some(Prepared { n, labels, start, end, vmin, vmax: vmin + vrange, vrange, c1, c2 })
}

pub fn open_frame(cfg: &DumbbellConfig, p: &Prepared) -> Frame {
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 132, 38, 52, 20, p.n * 400 + 2048);
    f.open(cfg.title, true);
    f.x_grid(6, p.vmin, p.vmin + p.vrange, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    f
}

pub fn finalize(f: Frame, cfg: &DumbbellConfig, p: &Prepared) -> String {
    let mut f = f;
    svg_legend_item(&mut f.buf, 0, cfg.series_names.0, p.c1, cfg.width - 140, f.pt + 4, 20);
    svg_legend_item(&mut f.buf, 1, cfg.series_names.1, p.c2, cfg.width - 140, f.pt + 22, 20);
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}

pub fn label_left(f: &mut Frame, p: &Prepared, i: usize, cy: i32) {
    push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 6);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy + 3);
    push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
    escape_xml(&mut f.buf, truncate(&p.labels[i], 18));
    push_b(&mut f.buf, b"</text>");
}

pub fn data_dot(f: &mut Frame, p: &Prepared, i: usize, series: i32, name: &str, cx: i32, cy: i32, r: i32, color: u32) {
    let hx = hex6(color);
    push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, (i as i32) * 2 + series);
    push_b(&mut f.buf, b"\" data-series=\""); push_i(&mut f.buf, series);
    push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, if series == 0 { p.start[i] } else { p.end[i] });
    push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &p.labels[i]);
    push_b(&mut f.buf, b" ("); escape_xml(&mut f.buf, name);
    push_b(&mut f.buf, b")\" cx=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
    push_b(&mut f.buf, b"\" r=\""); push_i(&mut f.buf, r);
    push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\"/>");
}

pub fn x_at(f: &Frame, p: &Prepared, v: f64) -> i32 {
    f.pl + (((v - p.vmin) / p.vrange) * f.pw as f64) as i32
}
