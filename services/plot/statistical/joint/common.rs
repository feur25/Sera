use super::config::JointConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    hex6, push_b, push_f2, svg_axis_lines, svg_open, svg_title, svg_x_label, svg_y_label, Frame,
};
use crate::plot::statistical::histogram::common::compute_bins;
use crate::plot::statistical::kde::common::{kde_eval, scott_bw};

pub struct Bounds {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

pub fn data_bounds(x: &[f64], y: &[f64]) -> Option<Bounds> {
    let n = x.len().min(y.len());
    if n == 0 {
        return None;
    }
    let (mut xmin, mut xmax) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut ymin, mut ymax) = (f64::INFINITY, f64::NEG_INFINITY);
    for i in 0..n {
        if x[i] < xmin {
            xmin = x[i];
        }
        if x[i] > xmax {
            xmax = x[i];
        }
        if y[i] < ymin {
            ymin = y[i];
        }
        if y[i] > ymax {
            ymax = y[i];
        }
    }
    let xpad = (xmax - xmin).max(1e-9) * 0.06;
    let ypad = (ymax - ymin).max(1e-9) * 0.06;
    Some(Bounds {
        xmin: xmin - xpad,
        xmax: xmax + xpad,
        ymin: ymin - ypad,
        ymax: ymax + ypad,
    })
}

pub struct Layout {
    pub pl: i32,
    pub pt: i32,
    pub pw: i32,
    pub ph: i32,
    pub top_y0: i32,
    pub top_y1: i32,
    pub right_x0: i32,
    pub right_x1: i32,
}

const LEFT_AXIS: i32 = 60;
const BOTTOM_AXIS: i32 = 50;
const TOP_TITLE: i32 = 42;
const RIGHT_PAD: i32 = 24;
const MARGIN_PANEL: i32 = 88;
const GAP: i32 = 10;

pub fn layout(cfg: &JointConfig) -> Layout {
    let pl = LEFT_AXIS;
    let pt = TOP_TITLE + MARGIN_PANEL + GAP;
    let pw = (cfg.width - pl - MARGIN_PANEL - GAP - RIGHT_PAD).max(80);
    let ph = (cfg.height - pt - BOTTOM_AXIS).max(80);
    Layout {
        pl,
        pt,
        pw,
        ph,
        top_y0: TOP_TITLE,
        top_y1: TOP_TITLE + MARGIN_PANEL - 4,
        right_x0: pl + pw + GAP,
        right_x1: pl + pw + GAP + MARGIN_PANEL,
    }
}

pub fn open(cfg: &JointConfig) -> Frame {
    let mut f = Frame::new(cfg.width, cfg.height, 0, 0, 0, 0, cfg.x_values.len() * 32 + 8192);
    svg_open(&mut f.buf, cfg.width, cfg.height);
    svg_title(&mut f.buf, cfg.title, cfg.width / 2, 22);
    f
}

pub fn draw_main_axes(f: &mut Frame, cfg: &JointConfig, l: &Layout) {
    svg_axis_lines(&mut f.buf, l.pl, l.pt, l.pw, l.ph);
    svg_x_label(&mut f.buf, cfg.x_label, l.pl + l.pw / 2, cfg.height - 6);
    svg_y_label(&mut f.buf, cfg.y_label, 16, l.pt, l.ph);
}

pub fn px(l: &Layout, bounds: &Bounds, x: f64) -> f64 {
    l.pl as f64 + (x - bounds.xmin) / (bounds.xmax - bounds.xmin).max(1e-12) * l.pw as f64
}

pub fn py(l: &Layout, bounds: &Bounds, y: f64) -> f64 {
    l.pt as f64 + l.ph as f64 - (y - bounds.ymin) / (bounds.ymax - bounds.ymin).max(1e-12) * l.ph as f64
}

pub fn top_histogram(f: &mut Frame, l: &Layout, bounds: &Bounds, values: &[f64], n_bins: usize, color: u32) {
    let (counts, edges) = compute_bins(values, n_bins.max(4));
    let max_c = counts.iter().copied().max().unwrap_or(1).max(1) as f64;
    let hx = hex6(color);
    let avail_h = (l.top_y1 - l.top_y0) as f64;
    for i in 0..counts.len() {
        if edges[i + 1] < bounds.xmin || edges[i] > bounds.xmax {
            continue;
        }
        let x0 = px(l, bounds, edges[i]).max(l.pl as f64);
        let x1 = px(l, bounds, edges[i + 1]).min((l.pl + l.pw) as f64);
        if x1 <= x0 {
            continue;
        }
        let bh = (counts[i] as f64 / max_c) * avail_h;
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, x0 + 0.5);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, l.top_y1 as f64 - bh);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, (x1 - x0 - 1.0).max(0.5));
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, bh.max(0.5));
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.75\"/>");
    }
}

pub fn right_histogram(f: &mut Frame, l: &Layout, bounds: &Bounds, values: &[f64], n_bins: usize, color: u32) {
    let (counts, edges) = compute_bins(values, n_bins.max(4));
    let max_c = counts.iter().copied().max().unwrap_or(1).max(1) as f64;
    let hx = hex6(color);
    let avail_w = (l.right_x1 - l.right_x0) as f64;
    for i in 0..counts.len() {
        if edges[i + 1] < bounds.ymin || edges[i] > bounds.ymax {
            continue;
        }
        let y0 = py(l, bounds, edges[i + 1]).max(l.pt as f64);
        let y1 = py(l, bounds, edges[i]).min((l.pt + l.ph) as f64);
        if y1 <= y0 {
            continue;
        }
        let bw = (counts[i] as f64 / max_c) * avail_w;
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, l.right_x0 as f64);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, y0 + 0.5);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, bw.max(0.5));
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, (y1 - y0 - 1.0).max(0.5));
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.75\"/>");
    }
}

pub fn top_kde(f: &mut Frame, l: &Layout, bounds: &Bounds, values: &[f64], color: u32) {
    if values.len() < 2 {
        return;
    }
    let bw = scott_bw(values).max(1e-9);
    let n_pts = 64;
    let mut ys = Vec::with_capacity(n_pts);
    let mut max_d = 1e-12f64;
    for i in 0..n_pts {
        let x = bounds.xmin + (bounds.xmax - bounds.xmin) * i as f64 / (n_pts - 1) as f64;
        let d = kde_eval(values, x, bw);
        ys.push(d);
        if d > max_d {
            max_d = d;
        }
    }
    let avail_h = (l.top_y1 - l.top_y0) as f64;
    let hx = hex6(color);
    push_b(&mut f.buf, b"<path d=\"M");
    for i in 0..n_pts {
        let x = bounds.xmin + (bounds.xmax - bounds.xmin) * i as f64 / (n_pts - 1) as f64;
        let px_ = px(l, bounds, x);
        let py_ = l.top_y1 as f64 - (ys[i] / max_d) * avail_h;
        if i > 0 {
            push_b(&mut f.buf, b"L");
        }
        push_f2(&mut f.buf, px_);
        push_b(&mut f.buf, b",");
        push_f2(&mut f.buf, py_);
    }
    push_b(&mut f.buf, b" L");
    push_f2(&mut f.buf, px(l, bounds, bounds.xmax));
    push_b(&mut f.buf, b",");
    push_f2(&mut f.buf, l.top_y1 as f64);
    push_b(&mut f.buf, b" L");
    push_f2(&mut f.buf, px(l, bounds, bounds.xmin));
    push_b(&mut f.buf, b",");
    push_f2(&mut f.buf, l.top_y1 as f64);
    push_b(&mut f.buf, b" Z\" fill=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" fill-opacity=\"0.55\" stroke=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\"1.4\"/>");
}

pub fn right_kde(f: &mut Frame, l: &Layout, bounds: &Bounds, values: &[f64], color: u32) {
    if values.len() < 2 {
        return;
    }
    let bw = scott_bw(values).max(1e-9);
    let n_pts = 64;
    let mut xs = Vec::with_capacity(n_pts);
    let mut max_d = 1e-12f64;
    for i in 0..n_pts {
        let y = bounds.ymin + (bounds.ymax - bounds.ymin) * i as f64 / (n_pts - 1) as f64;
        let d = kde_eval(values, y, bw);
        xs.push(d);
        if d > max_d {
            max_d = d;
        }
    }
    let avail_w = (l.right_x1 - l.right_x0) as f64;
    let hx = hex6(color);
    push_b(&mut f.buf, b"<path d=\"M");
    for i in 0..n_pts {
        let y = bounds.ymin + (bounds.ymax - bounds.ymin) * i as f64 / (n_pts - 1) as f64;
        let px_ = l.right_x0 as f64 + (xs[i] / max_d) * avail_w;
        let py_ = py(l, bounds, y);
        if i > 0 {
            push_b(&mut f.buf, b"L");
        }
        push_f2(&mut f.buf, px_);
        push_b(&mut f.buf, b",");
        push_f2(&mut f.buf, py_);
    }
    push_b(&mut f.buf, b" L");
    push_f2(&mut f.buf, l.right_x0 as f64);
    push_b(&mut f.buf, b",");
    push_f2(&mut f.buf, py(l, bounds, bounds.ymax));
    push_b(&mut f.buf, b" L");
    push_f2(&mut f.buf, l.right_x0 as f64);
    push_b(&mut f.buf, b",");
    push_f2(&mut f.buf, py(l, bounds, bounds.ymin));
    push_b(&mut f.buf, b" Z\" fill=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" fill-opacity=\"0.55\" stroke=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\"1.4\"/>");
}

pub fn finalize(f: Frame, cfg: &JointConfig) -> String {
    f.html(&slots_to_json(cfg.hover))
}
