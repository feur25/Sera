use super::common::{build_curve, build_xs, ordered_series, x_range};
use super::config::KdeConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i, Frame};

pub fn render(cfg: &KdeConfig) -> String {
    let series = ordered_series(cfg);
    let n_ser = series.len();
    if n_ser == 0 { return String::new(); }
    let (x0, x1) = match x_range(&series) { Some(v) => v, None => return String::new() };
    let xr = x1 - x0;
    let n_pts = cfg.n_points.max(40);
    let xs = build_xs(x0, x1, n_pts);
    let curves: Vec<Vec<f64>> = series.iter().map(|(_, v)| build_curve(v, &xs, cfg.bandwidth)).collect();
    let y_max = curves.iter().flat_map(|c| c.iter().copied()).fold(0.0_f64, f64::max).max(1e-12);

    let rug_band: i32 = 18;
    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52 + rug_band, legend_w, n_ser * n_pts * 22 + 4096);
    f.open(cfg.title, false);
    f.y_grid(5, 0.0, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let base_y = (f.pt + f.ph) as f64;
    for (si, ys) in curves.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, si as i32); push_b(&mut f.buf, b"\">");
        let pts: Vec<(f64, f64)> = xs.iter().zip(ys.iter()).map(|(&x, &y)| {
            let sx = f.pl as f64 + (x - x0) / xr * f.pw as f64;
            let sy = f.pt as f64 + f.ph as f64 - y / y_max * f.ph as f64;
            (sx, sy)
        }).collect();
        if cfg.filled {
            push_b(&mut f.buf, b"<path d=\"M");
            push_f2(&mut f.buf, pts[0].0); push_b(&mut f.buf, b","); push_f2(&mut f.buf, base_y);
            push_b(&mut f.buf, b" L"); push_f2(&mut f.buf, pts[0].0); push_b(&mut f.buf, b","); push_f2(&mut f.buf, pts[0].1);
            for &(sx, sy) in pts.iter().skip(1) {
                push_b(&mut f.buf, b" L"); push_f2(&mut f.buf, sx); push_b(&mut f.buf, b","); push_f2(&mut f.buf, sy);
            }
            let last = pts.last().unwrap();
            push_b(&mut f.buf, b" L"); push_f2(&mut f.buf, last.0); push_b(&mut f.buf, b","); push_f2(&mut f.buf, base_y);
            push_b(&mut f.buf, b" Z\" fill=\"#"); f.buf.extend_from_slice(&hx);
            let op = cfg.fill_opacity as f64 / 255.0 * 0.6;
            push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, op); push_b(&mut f.buf, b"\"/>");
        }
        push_b(&mut f.buf, b"<polyline points=\"");
        for (i, &(sx, sy)) in pts.iter().enumerate() {
            if i > 0 { f.buf.push(b' '); }
            push_f2(&mut f.buf, sx); f.buf.push(b','); push_f2(&mut f.buf, sy);
        }
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2\" stroke-linejoin=\"round\"/>");
        push_b(&mut f.buf, b"</g>");
    }

    let rug_y0 = f.pt + f.ph + 6;
    let row_h = (rug_band - 6) / (n_ser.max(1) as i32);
    for (si, (_, vals)) in series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let y_top = rug_y0 + (si as i32) * row_h;
        let y_bot = y_top + row_h - 2;
        let cap = 200usize;
        let step = if vals.len() > cap { vals.len() / cap } else { 1 };
        push_b(&mut f.buf, b"<g stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.55\">");
        for v in vals.iter().step_by(step.max(1)) {
            if !v.is_finite() { continue; }
            let sx = f.pl as f64 + (v - x0) / xr * f.pw as f64;
            push_b(&mut f.buf, b"<line x1=\""); push_f2(&mut f.buf, sx);
            push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_top);
            push_b(&mut f.buf, b"\" x2=\""); push_f2(&mut f.buf, sx);
            push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_bot);
            push_b(&mut f.buf, b"\"/>");
        }
        push_b(&mut f.buf, b"</g>");
    }

    f.x_grid(6, x0, x1, false);
    if n_ser > 1 {
        let names: Vec<&str> = series.iter().map(|(s, _)| s.as_str()).collect();
        f.legend(&names, cfg.palette, cfg.width - legend_w + 12);
    }
    f.html(&slots_to_json(cfg.hover))
}
