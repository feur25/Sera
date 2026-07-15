use super::config::SplomConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i, truncate};

pub struct Prepared {
    pub m: usize,
    pub n: usize,
    pub cols: Vec<Vec<f64>>,
    pub mins: Vec<f64>,
    pub maxs: Vec<f64>,
    pub cell_w: f64,
    pub cell_h: f64,
    pub pad_l: f64,
    pub pad_t: f64,
}

pub fn prepare(cfg: &SplomConfig) -> Option<Prepared> {
    let m = cfg.axes.len();
    if m < 2 {
        return None;
    }
    let n = cfg.series_values.iter().filter(|r| r.len() >= m).count();
    if n == 0 {
        return None;
    }
    let mut cols: Vec<Vec<f64>> = vec![Vec::with_capacity(n); m];
    for row in cfg.series_values {
        if row.len() < m {
            continue;
        }
        for (j, col) in cols.iter_mut().enumerate() {
            col.push(row[j]);
        }
    }
    let mut mins = vec![f64::INFINITY; m];
    let mut maxs = vec![f64::NEG_INFINITY; m];
    for j in 0..m {
        for &v in &cols[j] {
            if v < mins[j] {
                mins[j] = v;
            }
            if v > maxs[j] {
                maxs[j] = v;
            }
        }
        let pad = (maxs[j] - mins[j]).max(1e-9) * 0.08;
        mins[j] -= pad;
        maxs[j] += pad;
    }

    let pad_l = 46.0;
    let pad_t = 46.0;
    let plot_w = (cfg.width as f64 - pad_l - 12.0).max(1.0);
    let plot_h = (cfg.height as f64 - pad_t - 12.0).max(1.0);
    let cell_w = plot_w / m as f64;
    let cell_h = plot_h / m as f64;

    Some(Prepared {
        m,
        n,
        cols,
        mins,
        maxs,
        cell_w,
        cell_h,
        pad_l,
        pad_t,
    })
}

pub fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 {
        return 0.0;
    }
    let mx = x[..n].iter().sum::<f64>() / n as f64;
    let my = y[..n].iter().sum::<f64>() / n as f64;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..n {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= 0.0 || syy <= 0.0 {
        return 0.0;
    }
    (sxy / (sxx.sqrt() * syy.sqrt())).clamp(-1.0, 1.0)
}

pub fn cell_point_px(p: &Prepared, r: usize, c: usize, xv: f64, yv: f64) -> (f64, f64) {
    let x0 = p.pad_l + c as f64 * p.cell_w;
    let y0 = p.pad_t + r as f64 * p.cell_h;
    let xr = (p.maxs[c] - p.mins[c]).max(1e-9);
    let yr = (p.maxs[r] - p.mins[r]).max(1e-9);
    let px = x0 + (xv - p.mins[c]) / xr * p.cell_w;
    let py = y0 + p.cell_h - (yv - p.mins[r]) / yr * p.cell_h;
    (px, py)
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &SplomConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\">");
    push_b(buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn cell_border(buf: &mut Vec<u8>, p: &Prepared, r: usize, c: usize) {
    push_b(buf, b"<rect x=\"");
    push_f2(buf, p.pad_l + c as f64 * p.cell_w);
    push_b(buf, b"\" y=\"");
    push_f2(buf, p.pad_t + r as f64 * p.cell_h);
    push_b(buf, b"\" width=\"");
    push_f2(buf, p.cell_w);
    push_b(buf, b"\" height=\"");
    push_f2(buf, p.cell_h);
    push_b(buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.6\"/>");
}

pub fn diagonal_label(buf: &mut Vec<u8>, p: &Prepared, r: usize, label: &str) {
    let x0 = p.pad_l + r as f64 * p.cell_w;
    let y0 = p.pad_t + r as f64 * p.cell_h;
    push_b(buf, b"<rect x=\"");
    push_f2(buf, x0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y0);
    push_b(buf, b"\" width=\"");
    push_f2(buf, p.cell_w);
    push_b(buf, b"\" height=\"");
    push_f2(buf, p.cell_h);
    push_b(buf, b"\" fill=\"#f1f5f9\"/>");
    let max_chars = ((p.cell_w - 8.0) / 6.5).max(1.0) as usize;
    let short = truncate(label, max_chars);
    push_b(buf, b"<text x=\"");
    push_f2(buf, x0 + p.cell_w / 2.0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y0 + p.cell_h / 2.0 + 4.0);
    push_b(
        buf,
        b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#334155\" pointer-events=\"none\">",
    );
    escape_xml(buf, short);
    push_b(buf, b"</text>");
}

pub fn axis_labels(buf: &mut Vec<u8>, p: &Prepared, cfg: &SplomConfig) {
    for (j, name) in cfg.axes.iter().enumerate() {
        let max_chars = ((p.cell_w - 8.0) / 6.0).max(1.0) as usize;
        let short = truncate(name, max_chars);
        let x = p.pad_l + j as f64 * p.cell_w + p.cell_w / 2.0;
        push_b(buf, b"<text x=\"");
        push_f2(buf, x);
        push_b(buf, b"\" y=\"");
        push_f2(buf, p.pad_t - 8.0);
        push_b(
            buf,
            b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">",
        );
        escape_xml(buf, short);
        push_b(buf, b"</text>");
    }
}

pub fn finalize(mut buf: Vec<u8>, cfg: &SplomConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
