use super::common::{build_curve, build_xs, histogram_normalized, ordered_series, x_range};
use super::config::KdeConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i, Frame};

pub const DEMO_KWARGS: &str = "values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]";
pub fn render(cfg: &KdeConfig) -> String {
    let series = ordered_series(cfg);
    let n_ser = series.len();
    if n_ser == 0 { return String::new(); }
    let (x0, x1) = match x_range(&series) { Some(v) => v, None => return String::new() };
    let xr = x1 - x0;
    let n_pts = cfg.n_points.max(60);
    let xs = build_xs(x0, x1, n_pts);

    let curves: Vec<Vec<f64>> = series.iter().map(|(_, v)| {
        let raw = build_curve(v, &xs, cfg.bandwidth);
        let area: f64 = (1..raw.len()).map(|i| 0.5 * (raw[i] + raw[i - 1]) * (xs[i] - xs[i - 1])).sum::<f64>().max(1e-12);
        raw.iter().map(|y| y / area).collect()
    }).collect();

    let bins = cfg.bins.max(10);
    let hists: Vec<(Vec<f64>, f64)> = series.iter().map(|(_, v)| histogram_normalized(v, x0, x1, bins)).collect();

    let y_max_curve = curves.iter().flat_map(|c| c.iter().copied()).fold(0.0_f64, f64::max);
    let y_max_hist = hists.iter().map(|h| h.1).fold(0.0_f64, f64::max);
    let y_max = y_max_curve.max(y_max_hist).max(1e-12);

    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n_ser * (n_pts + bins) * 28 + 4096);
    f.open(cfg.title, false);
    f.y_grid(5, 0.0, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let bin_w_data = xr / bins as f64;
    let bin_w_px = bin_w_data / xr * f.pw as f64;
    let gap = (bin_w_px * 0.06).max(1.0);

    for (si, (densities, _)) in hists.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let op = if n_ser >= 2 { 0.32 } else { 0.45 };
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, si as i32); push_b(&mut f.buf, b"\">");
        for (i, &d) in densities.iter().enumerate() {
            let bx0 = f.pl as f64 + (i as f64 * bin_w_px);
            let bw = bin_w_px - gap;
            let bh = d / y_max * f.ph as f64;
            let by = f.pt as f64 + f.ph as f64 - bh;
            push_b(&mut f.buf, b"<rect x=\""); push_f2(&mut f.buf, bx0);
            push_b(&mut f.buf, b"\" y=\""); push_f2(&mut f.buf, by);
            push_b(&mut f.buf, b"\" width=\""); push_f2(&mut f.buf, bw.max(0.5));
            push_b(&mut f.buf, b"\" height=\""); push_f2(&mut f.buf, bh);
            push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, op); push_b(&mut f.buf, b"\" rx=\"1\"/>");
        }
        push_b(&mut f.buf, b"</g>");
    }

    for (si, ys) in curves.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<polyline points=\"");
        for (i, (&x, &y)) in xs.iter().zip(ys.iter()).enumerate() {
            let sx = f.pl as f64 + (x - x0) / xr * f.pw as f64;
            let sy = f.pt as f64 + f.ph as f64 - y / y_max * f.ph as f64;
            if i > 0 { f.buf.push(b' '); }
            push_f2(&mut f.buf, sx); f.buf.push(b','); push_f2(&mut f.buf, sy);
        }
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2.4\" stroke-linejoin=\"round\"/>");
    }

    f.x_grid(6, x0, x1, false);
    if n_ser > 1 {
        let names: Vec<&str> = series.iter().map(|(s, _)| s.as_str()).collect();
        f.legend(&names, cfg.palette, cfg.width - legend_w + 12);
    }
    f.html(&slots_to_json(cfg.hover))
}


