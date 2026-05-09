use super::config::ParallelConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color, svg_open, svg_title, svg_legend_item};
use crate::html::hover::{build_chart_html, slots_to_json};

pub struct Prepared {
    pub n_axes: usize,
    pub n_series: usize,
    pub mins: Vec<f64>,
    pub maxs: Vec<f64>,
    pub pad_l: i32,
    pub pad_r: i32,
    pub pad_t: i32,
    pub pad_b: i32,
    pub plot_w: i32,
    pub plot_h: i32,
}

pub fn prepare(cfg: &ParallelConfig) -> Option<Prepared> {
    let n_axes = cfg.axes.len();
    let n_series = cfg.series_names.len().min(cfg.series_values.len());
    if n_axes < 2 || n_series == 0 { return None; }
    let mut mins = vec![f64::INFINITY; n_axes];
    let mut maxs = vec![f64::NEG_INFINITY; n_axes];
    for si in 0..n_series {
        let v = &cfg.series_values[si];
        for ai in 0..n_axes.min(v.len()) {
            if v[ai] < mins[ai] { mins[ai] = v[ai]; }
            if v[ai] > maxs[ai] { maxs[ai] = v[ai]; }
        }
    }
    for ai in 0..n_axes {
        if (maxs[ai] - mins[ai]).abs() < 1e-9 { maxs[ai] = mins[ai] + 1.0; }
    }
    let pad_l: i32 = 50;
    let pad_r: i32 = 150;
    let pad_t: i32 = 44;
    let pad_b: i32 = 44;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    Some(Prepared { n_axes, n_series, mins, maxs, pad_l, pad_r, pad_t, pad_b, plot_w, plot_h })
}

pub fn open(cfg: &ParallelConfig, p: &Prepared) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(p.n_series * p.n_axes * 200 + 4096);
    svg_open(&mut buf, cfg.width, cfg.height);
    svg_title(&mut buf, cfg.title, cfg.width / 2, 26);
    buf
}

pub fn axes_grid(buf: &mut Vec<u8>, cfg: &ParallelConfig, p: &Prepared) {
    for ai in 0..p.n_axes {
        let x = p.pad_l + (ai as f64 / (p.n_axes - 1) as f64 * p.plot_w as f64) as i32;
        push_b(buf, b"<line x1=\""); push_i(buf, x);
        push_b(buf, b"\" y1=\""); push_i(buf, p.pad_t);
        push_b(buf, b"\" x2=\""); push_i(buf, x);
        push_b(buf, b"\" y2=\""); push_i(buf, p.pad_t + p.plot_h);
        push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
        push_b(buf, b"<text x=\""); push_i(buf, x);
        push_b(buf, b"\" y=\""); push_i(buf, p.pad_t + p.plot_h + 18);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(buf, &cfg.axes[ai]);
        push_b(buf, b"</text>");
        push_b(buf, b"<text x=\""); push_i(buf, x);
        push_b(buf, b"\" y=\""); push_i(buf, p.pad_t + p.plot_h + 32);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#94a3b8\">");
        let s = format!("{:.1}", p.mins[ai]);
        buf.extend_from_slice(s.as_bytes());
        push_b(buf, b"</text>");
        push_b(buf, b"<text x=\""); push_i(buf, x);
        push_b(buf, b"\" y=\""); push_i(buf, p.pad_t - 6);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#94a3b8\">");
        let s = format!("{:.1}", p.maxs[ai]);
        buf.extend_from_slice(s.as_bytes());
        push_b(buf, b"</text>");
        for t in 0..=4 {
            let frac = t as f64 / 4.0;
            let y = p.pad_t + p.plot_h - (frac * p.plot_h as f64) as i32;
            push_b(buf, b"<line x1=\""); push_i(buf, x - 3);
            push_b(buf, b"\" y1=\""); push_i(buf, y);
            push_b(buf, b"\" x2=\""); push_i(buf, x + 3);
            push_b(buf, b"\" y2=\""); push_i(buf, y);
            push_b(buf, b"\" stroke=\"#94a3b8\" stroke-width=\"0.5\"/>");
        }
    }
}

pub fn legend(buf: &mut Vec<u8>, cfg: &ParallelConfig, p: &Prepared) {
    for (li, name) in cfg.series_names.iter().enumerate() {
        let col = palette_color(cfg.palette, li);
        svg_legend_item(buf, li as i32, name, col, cfg.width - p.pad_r + 14, p.pad_t + 4 + (li as i32) * 20, 20);
    }
}

pub fn point(p: &Prepared, ai: usize, val: f64) -> (f64, f64) {
    let x = p.pad_l as f64 + (ai as f64 / (p.n_axes - 1) as f64 * p.plot_w as f64);
    let f = (val - p.mins[ai]) / (p.maxs[ai] - p.mins[ai]);
    let y = (p.pad_t + p.plot_h) as f64 - f * p.plot_h as f64;
    (x, y)
}

pub fn finalize(buf: Vec<u8>, cfg: &ParallelConfig) -> String {
    let mut buf = buf;
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

pub fn write_polyline(buf: &mut Vec<u8>, p: &Prepared, vals: &[f64], color: u32, stroke_w: f64, opacity: f64, si: usize, name: &str) {
    let hx = hex6(color);
    push_b(buf, b"<polyline data-idx=\""); push_i(buf, si as i32);
    push_b(buf, b"\" data-series=\""); push_i(buf, si as i32);
    push_b(buf, b"\" data-lbl=\""); escape_xml(buf, name);
    push_b(buf, b"\" fill=\"none\" stroke=\"#"); buf.extend_from_slice(&hx);
    push_b(buf, b"\" stroke-width=\""); push_f2(buf, stroke_w);
    push_b(buf, b"\" stroke-opacity=\""); push_f2(buf, opacity);
    push_b(buf, b"\" stroke-linejoin=\"round\" points=\"");
    for ai in 0..p.n_axes {
        if ai >= vals.len() { break; }
        let (x, y) = point(p, ai, vals[ai]);
        if ai > 0 { push_b(buf, b" "); }
        push_f2(buf, x); push_b(buf, b","); push_f2(buf, y);
    }
    push_b(buf, b"\"/>");
}

pub fn write_dots(buf: &mut Vec<u8>, p: &Prepared, vals: &[f64], color: u32, r: f64, opacity: f64, si: usize) {
    let hx = hex6(color);
    for ai in 0..p.n_axes.min(vals.len()) {
        let (x, y) = point(p, ai, vals[ai]);
        push_b(buf, b"<circle data-idx=\""); push_i(buf, (si * p.n_axes + ai) as i32);
        push_b(buf, b"\" data-series=\""); push_i(buf, si as i32);
        push_b(buf, b"\" cx=\""); push_f2(buf, x);
        push_b(buf, b"\" cy=\""); push_f2(buf, y);
        push_b(buf, b"\" r=\""); push_f2(buf, r);
        push_b(buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
        push_b(buf, b"\" fill-opacity=\""); push_f2(buf, opacity);
        push_b(buf, b"\"/>");
    }
}


