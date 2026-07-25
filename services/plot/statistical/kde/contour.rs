use super::common::scott_bw;
use super::config::KdeConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};

fn gaussian_kernel_sum(x: f64, y: f64, xs: &[f64], ys: &[f64], bwx: f64, bwy: f64) -> f64 {
    if xs.is_empty() || bwx <= 0.0 || bwy <= 0.0 {
        return 0.0;
    }
    let n = xs.len() as f64;
    let c = 0.15915494309189535 / (bwx * bwy * n);
    let mut sum = 0.0f64;
    for i in 0..xs.len() {
        let ux = (x - xs[i]) / bwx;
        let uy = (y - ys[i]) / bwy;
        let uu = ux * ux + uy * uy;
        if uu < 30.0 {
            sum += c * (-0.5 * uu).exp();
        }
    }
    sum
}

#[crate::chart_demo("x=[1.2,2.4,2.1,3.6,3.1,3.9,4.2,4.6,4.4,4.9,5.5,5.1,5.8,2.2,3.3,3.7,4.1,1.8,2.6,3.4,4.3,5.2,3.2,3.8], y=[1.1,2.3,3.2,2.4,3.6,4.1,3.3,4.7,5.2,3.9,4.4,5.6,6.1,1.4,2.5,4.2,4.6,2.1,3.1,3.3,5.1,4.5,3.4,5.3], variant=\"contour\"")]

pub fn render(cfg: &KdeConfig) -> String {
    let xs: &[f64] = cfg.series.first().map(|(_, v)| v.as_slice()).unwrap_or(&[]);
    let ys: &[f64] = cfg.y_values;
    let n = xs.len().min(ys.len());
    if n == 0 {
        return String::new();
    }
    let xs = &xs[..n];
    let ys = &ys[..n];

    let (mut x0, mut x1) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY);
    for i in 0..n {
        x0 = x0.min(xs[i]);
        x1 = x1.max(xs[i]);
        y0 = y0.min(ys[i]);
        y1 = y1.max(ys[i]);
    }
    let xr = (x1 - x0).max(1e-9);
    let yr = (y1 - y0).max(1e-9);
    x0 -= xr * 0.1;
    x1 += xr * 0.1;
    y0 -= yr * 0.1;
    y1 += yr * 0.1;

    let bwx = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(xs) };
    let bwy = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(ys) };

    let grid_n = 42usize;
    let mut density = vec![0.0f64; grid_n * grid_n];
    let mut dmax = 1e-12f64;
    for gy in 0..grid_n {
        let yv = y0 + (y1 - y0) * gy as f64 / (grid_n - 1) as f64;
        for gx in 0..grid_n {
            let xv = x0 + (x1 - x0) * gx as f64 / (grid_n - 1) as f64;
            let d = gaussian_kernel_sum(xv, yv, xs, ys, bwx, bwy);
            density[gy * grid_n + gx] = d;
            if d > dmax {
                dmax = d;
            }
        }
    }

    let pad_l = 56i32;
    let pad_t = 20i32;
    let pad_r = 20i32;
    let pad_b = 44i32;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, pad_t, pad_b, pad_r, grid_n * grid_n * 90 + 4096);
    f.open(cfg.title, false);
    f.y_grid(5, y0, y1, cfg.gridlines);
    f.x_grid(5, x0, x1, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cell_w = f.pw as f64 / grid_n as f64;
    let cell_h = f.ph as f64 / grid_n as f64;
    let base_hue = if cfg.palette.is_empty() { 0x636EFA } else { cfg.palette[0] };
    let br = ((base_hue >> 16) & 0xFF) as f64;
    let bg = ((base_hue >> 8) & 0xFF) as f64;
    let bb = (base_hue & 0xFF) as f64;

    for gy in 0..grid_n {
        for gx in 0..grid_n {
            let t = (density[gy * grid_n + gx] / dmax).clamp(0.0, 1.0);
            if t < 0.03 {
                continue;
            }
            let r = (255.0 + (br - 255.0) * t) as u32;
            let g = (255.0 + (bg - 255.0) * t) as u32;
            let b = (255.0 + (bb - 255.0) * t) as u32;
            let color = (r << 16) | (g << 8) | b;
            let hx = hex6(color);
            let px = f.pl as f64 + gx as f64 * cell_w;
            let py = f.pt as f64 + (grid_n - 1 - gy) as f64 * cell_h;
            push_b(&mut f.buf, b"<rect x=\"");
            push_f2(&mut f.buf, px);
            push_b(&mut f.buf, b"\" y=\"");
            push_f2(&mut f.buf, py);
            push_b(&mut f.buf, b"\" width=\"");
            push_f2(&mut f.buf, cell_w + 0.6);
            push_b(&mut f.buf, b"\" height=\"");
            push_f2(&mut f.buf, cell_h + 0.6);
            push_b(&mut f.buf, b"\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\"");
            push_f2(&mut f.buf, 0.25 + t * 0.65);
            push_b(&mut f.buf, b"\"/>");
        }
    }

    for i in 0..n {
        let px = f.pl as f64 + (xs[i] - x0) / (x1 - x0) * f.pw as f64;
        let py = f.pt as f64 + (1.0 - (ys[i] - y0) / (y1 - y0)) * f.ph as f64;
        push_b(&mut f.buf, b"<circle data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" cx=\"");
        push_f2(&mut f.buf, px);
        push_b(&mut f.buf, b"\" cy=\"");
        push_f2(&mut f.buf, py);
        push_b(&mut f.buf, b"\" r=\"2\" fill=\"#");
        f.buf.extend_from_slice(&hex6(base_hue));
        push_b(&mut f.buf, b"\" fill-opacity=\"0.55\"/>");
    }

    let json;
    let hover_json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        json = slots_to_json(cfg.hover);
        &json
    };
    f.html(hover_json)
}
