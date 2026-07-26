use super::common::{bin_color, data_bounds, draw_hex_cell, finalize, hist_counts, prepare};
use super::config::HexbinConfig;
use crate::plot::statistical::common::{push_b, push_f2, Frame};

#[crate::chart_demo(
    "gridsize=16, x=[11.03,12.37,13.2,14.37,13.24,14.2,14.39,14.06,14.83,13.86,14.1,14.12,13.75,14.75,14.38,13.63,14.3,13.83,14.19,13.64,14.06,12.93,13.71,12.85,13.5,13.05,13.39,13.3,13.87,14.02,13.73,13.58,13.68,13.76,13.51,13.48,13.28,13.05,13.07,14.22,13.56,13.41,13.88,13.24,12.37,13.34,12.21,12.29,13.86,13.05], y=[2.16,1.63,1.78,1.95,2.59,1.76,1.87,2.15,1.64,1.35,2.16,1.48,1.73,1.68,1.87,1.81,1.92,1.57,2.14,3.1,1.63,3.8,1.79,3.27,1.77,2.05,1.68,1.72,1.9,3.99,4.36,2.58,1.83,1.53,1.8,1.61,1.64,1.65,1.5,3.99,1.71,3.43,1.87,3.98,1.13,1.94,1.19,1.61,1.51,2.05], variant=\"marginals\""
)]

pub fn render(cfg: &HexbinConfig) -> String {
    let bounds = match data_bounds(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        100,
        52,
        60,
        cfg.x_values.len() * 48 + 8192,
    );
    f.open(cfg.title, true);
    f.x_grid(6, bounds.xmin, bounds.xmax, cfg.gridlines);
    f.y_grid(5, bounds.ymin, bounds.ymax, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let p = match prepare(cfg, &f, &bounds) {
        Some(v) => v,
        None => return String::new(),
    };
    for (i, bin) in p.bins.iter().enumerate() {
        let col = bin_color(cfg, &p, bin.count);
        draw_hex_cell(&mut f.buf, i, bin, p.r * 0.98, col, None, false, false);
    }

    let x_bins = 40usize;
    let xh = hist_counts(cfg.x_values, bounds.xmin, bounds.xmax, x_bins);
    let xmax_h = xh.iter().copied().max().unwrap_or(1).max(1) as f64;
    let band_h = 46.0;
    let band_y0 = f.pt as f64 - 10.0 - band_h;
    let bw = f.pw as f64 / x_bins as f64;
    for (i, &c) in xh.iter().enumerate() {
        let h = c as f64 / xmax_h * band_h;
        let bx = f.pl as f64 + i as f64 * bw;
        let by = band_y0 + band_h - h;
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, bx + 0.5);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, by);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, (bw - 1.0).max(0.5));
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, h.max(0.0));
        push_b(&mut f.buf, b"\" fill=\"#64748b\" fill-opacity=\"0.65\"/>");
    }

    let y_bins = 30usize;
    let yh = hist_counts(cfg.y_values, bounds.ymin, bounds.ymax, y_bins);
    let ymax_h = yh.iter().copied().max().unwrap_or(1).max(1) as f64;
    let side_w = 44.0;
    let side_x0 = f.pl as f64 + f.pw as f64 + 8.0;
    let bh = f.ph as f64 / y_bins as f64;
    for (i, &c) in yh.iter().enumerate() {
        let w = c as f64 / ymax_h * side_w;
        let by = f.pt as f64 + f.ph as f64 - (i as f64 + 1.0) * bh;
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, side_x0);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, by + 0.5);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, w.max(0.0));
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, (bh - 1.0).max(0.5));
        push_b(&mut f.buf, b"\" fill=\"#64748b\" fill-opacity=\"0.65\"/>");
    }
    finalize(f, cfg)
}
