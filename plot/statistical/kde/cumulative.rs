use super::common::{
    build_curve, build_xs, cumulative as cum_integ, integrate, ordered_series, x_range,
};
use super::config::KdeConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i, Frame};

#[crate::chart_demo("values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]")]

pub fn render(cfg: &KdeConfig) -> String {
    let series = ordered_series(cfg);
    let n_ser = series.len();
    if n_ser == 0 {
        return String::new();
    }
    let (x0, x1) = match x_range(&series) {
        Some(v) => v,
        None => return String::new(),
    };
    let xr = x1 - x0;
    let n_pts = cfg.n_points.max(80);
    let xs = build_xs(x0, x1, n_pts);
    let cdfs: Vec<Vec<f64>> = series
        .iter()
        .map(|(_, v)| {
            let raw = build_curve(v, &xs, cfg.bandwidth);
            let area = integrate(&raw, &xs);
            let normed: Vec<f64> = raw.iter().map(|y| y / area).collect();
            cum_integ(&normed, &xs)
        })
        .collect();

    let y_max = 1.0f64;
    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_ser * n_pts * 18 + 2048,
    );
    f.open(cfg.title, false);
    let yl = if cfg.y_label.is_empty() {
        "Cumulative density (CDF)"
    } else {
        cfg.y_label
    };
    f.y_grid(5, 0.0, y_max, cfg.gridlines);
    f.axes(cfg.x_label, yl);

    for (si, ys) in cdfs.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\">");
        push_b(&mut f.buf, b"<polyline points=\"");
        for (i, (&x, &y)) in xs.iter().zip(ys.iter()).enumerate() {
            let sx = f.pl as f64 + (x - x0) / xr * f.pw as f64;
            let yc = y.min(1.0).max(0.0);
            let sy = f.pt as f64 + f.ph as f64 - yc / y_max * f.ph as f64;
            if i > 0 {
                f.buf.push(b' ');
            }
            push_f2(&mut f.buf, sx);
            f.buf.push(b',');
            push_f2(&mut f.buf, sy);
        }
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" stroke-width=\"2.4\" stroke-linejoin=\"round\"/>",
        );
        let mid = ys.iter().position(|&y| y >= 0.5).unwrap_or(ys.len() / 2);
        let mx = f.pl as f64 + (xs[mid] - x0) / xr * f.pw as f64;
        let my = f.pt as f64 + f.ph as f64 / 2.0;
        push_b(&mut f.buf, b"<circle cx=\"");
        push_f2(&mut f.buf, mx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_f2(&mut f.buf, my);
        push_b(&mut f.buf, b"\" r=\"3.2\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke=\"#0b0b18\" stroke-width=\"1\"/>");
        push_b(&mut f.buf, b"</g>");
    }

    f.x_grid(6, x0, x1, false);
    if n_ser > 1 {
        let names: Vec<&str> = series.iter().map(|(s, _)| s.as_str()).collect();
        f.legend(&names, cfg.palette, cfg.width - legend_w + 12);
    }
    f.html(&slots_to_json(cfg.hover))
}
