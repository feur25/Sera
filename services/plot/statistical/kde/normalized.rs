use super::common::{build_curve, build_xs, integrate, ordered_series, x_range};
use super::config::KdeConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i, Frame};

#[crate::chart_demo(
    "values=[4.74,5.51,4.77,4.68,4.07,4.79,6.11,5.42,6.04,5.25,5.39,5.19,3.33,5.86,5.51,5.5,3.31,3.26,4.11,4.53,5.31,4.95,5.52,4.36,5.31,5.39,4.34,6.72,5.56,6.2,4.38,4.26,4.66,4.89,5.63,5.25,4.55,4.04,4.48,6.22,4.19,5.24,5.43,3.51,5.05,6.31,2.99,4.68,4.89,4.18], categories=[\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Large group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\",\"Small group\"]"
)]

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
    let n_pts = cfg.n_points.max(60);
    let xs = build_xs(x0, x1, n_pts);
    let curves: Vec<Vec<f64>> = series
        .iter()
        .map(|(_, v)| {
            let raw = build_curve(v, &xs, cfg.bandwidth);
            let area = integrate(&raw, &xs);
            raw.iter().map(|y| y / area).collect()
        })
        .collect();
    let y_max = curves
        .iter()
        .flat_map(|c| c.iter().copied())
        .fold(0.0_f64, f64::max)
        .max(1e-12);

    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_ser * n_pts * 24 + 2048,
    );
    f.open(cfg.title, false);
    let yl = if cfg.y_label.is_empty() {
        "Density (PDF)"
    } else {
        cfg.y_label
    };
    f.y_grid(5, 0.0, y_max, cfg.gridlines);
    f.axes(cfg.x_label, yl);

    let base_y = (f.pt + f.ph) as f64;
    for (si, ys) in curves.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" data-idx=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\">");
        let pts: Vec<(f64, f64)> = xs
            .iter()
            .zip(ys.iter())
            .map(|(&x, &y)| {
                let sx = f.pl as f64 + (x - x0) / xr * f.pw as f64;
                let sy = f.pt as f64 + f.ph as f64 - y / y_max * f.ph as f64;
                (sx, sy)
            })
            .collect();
        if cfg.filled {
            push_b(&mut f.buf, b"<path d=\"M");
            push_f2(&mut f.buf, pts[0].0);
            push_b(&mut f.buf, b",");
            push_f2(&mut f.buf, base_y);
            push_b(&mut f.buf, b" L");
            push_f2(&mut f.buf, pts[0].0);
            push_b(&mut f.buf, b",");
            push_f2(&mut f.buf, pts[0].1);
            for &(sx, sy) in pts.iter().skip(1) {
                push_b(&mut f.buf, b" L");
                push_f2(&mut f.buf, sx);
                push_b(&mut f.buf, b",");
                push_f2(&mut f.buf, sy);
            }
            let last = pts.last().unwrap();
            push_b(&mut f.buf, b" L");
            push_f2(&mut f.buf, last.0);
            push_b(&mut f.buf, b",");
            push_f2(&mut f.buf, base_y);
            push_b(&mut f.buf, b" Z\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            let op = cfg.fill_opacity as f64 / 255.0;
            push_b(&mut f.buf, b"\" fill-opacity=\"");
            push_f2(&mut f.buf, op);
            push_b(&mut f.buf, b"\"/>");
        }
        push_b(&mut f.buf, b"<polyline points=\"");
        for (i, &(sx, sy)) in pts.iter().enumerate() {
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
            b"\" stroke-width=\"2\" stroke-linejoin=\"round\"/>",
        );
        push_b(&mut f.buf, b"</g>");
    }

    f.x_grid(6, x0, x1, false);
    if n_ser > 1 {
        let names: Vec<&str> = series.iter().map(|(s, _)| s.as_str()).collect();
        f.legend(&names, cfg.palette, cfg.width - legend_w + 12);
    }
    f.html(&slots_to_json(cfg.hover))
}
