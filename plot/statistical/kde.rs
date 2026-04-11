use super::common::{palette_color, push_b, push_i, push_f2, hex6, Frame};
use crate::html::hover::slots_to_json;

crate::chart_config!(KdeConfig, 900, 420;
    struct {
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub bandwidth: f64,
        pub filled: bool,
        pub fill_opacity: u8,
        pub n_points: usize,
    }
    defaults {
        series: &[],
        palette: &[],
        bandwidth: 0.0,
        filled: true,
        fill_opacity: 50,
        n_points: 40,
    }
);

pub fn scott_bw(vals: &[f64]) -> f64 {
    let n = vals.len();
    if n < 2 { return 1.0; }
    let mean = vals.iter().sum::<f64>() / n as f64;
    let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    1.06 * var.sqrt() * (n as f64).powf(-0.2)
}

pub fn kde_eval(vals: &[f64], x: f64, bw: f64) -> f64 {
    let inv_n_bw = 1.0 / (bw * vals.len() as f64);
    let c = 0.3989422804014327 * inv_n_bw;
    let mut sum = 0.0f64;
    for &v in vals {
        let u = (x - v) / bw;
        let uu = u * u;
        if uu < 16.0 { sum += c * (-0.5 * uu).exp(); }
    }
    sum
}

pub fn render_kde_html(cfg: &KdeConfig) -> String {
    let n_ser = cfg.series.len();
    if n_ser == 0 { return String::new(); }

    let series: Vec<(String, Vec<f64>)> = if cfg.sort_order != "none" && !cfg.sort_order.is_empty() && n_ser > 1 {
        let means: Vec<f64> = cfg.series.iter().map(|(_, v)| if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 }).collect();
        let names: Vec<String> = cfg.series.iter().map(|(n, _)| n.clone()).collect();
        let idx = super::common::sort_indices(n_ser, &means, &names, cfg.sort_order);
        idx.iter().map(|&i| cfg.series[i].clone()).collect()
    } else {
        cfg.series.to_vec()
    };

    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let n_pts = cfg.n_points.max(30);

    let all_vals: Vec<f64> = series.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if all_vals.is_empty() { return String::new(); }

    let x_min = all_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let x_range = (x_max - x_min).max(1e-12);
    let x_pad = x_range * 0.12;
    let x0 = x_min - x_pad;
    let x1 = x_max + x_pad;
    let xr = x1 - x0;

    let xs: Vec<f64> = (0..n_pts).map(|i| x0 + xr * i as f64 / (n_pts - 1) as f64).collect();

    let curves: Vec<Vec<f64>> = series.iter().map(|(_, vals)| {
        let bw = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(vals).max(1e-12) };
        let cap = 40usize;
        let step = if vals.len() > cap { vals.len() / cap } else { 1 };
        let sampled: Vec<f64> = if step > 1 { vals.iter().step_by(step).copied().collect() } else { vals.to_vec() };
        xs.iter().map(|&x| kde_eval(&sampled, x, bw) * step as f64).collect()
    }).collect();

    let y_max = curves.iter().flat_map(|c| c.iter().copied()).fold(0.0_f64, f64::max).max(1e-12);

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n_ser * n_pts * 24 + 2048);
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
            let op = cfg.fill_opacity as f64 / 255.0;
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

    f.x_grid(6, x0, x1, false);

    if n_ser > 1 {
        let names: Vec<&str> = series.iter().map(|(s, _)| s.as_str()).collect();
        f.legend(&names, cfg.palette, cfg.width - legend_w + 12);
    }

    f.html(&slots_to_json(cfg.hover))
}
