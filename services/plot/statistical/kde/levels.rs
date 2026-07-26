use super::common::scott_bw;
use super::config::KdeConfig;
use super::contour::{gaussian_kernel_sum, group_hue};
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, Frame};

#[crate::chart_demo(
    "x=[55.4,60.0,51.3,59.0,54.0,54.0,62.6,55.6,54.8,57.9,59.5,54.9,57.4,51.1,53.5,53.2,49.7,49.0,81.6,78.4,82.3,82.6,85.2,78.8,77.0,77.0,75.1,79.8,76.1,85.3,70.7,74.5,75.2,69.5,89.5,68.0], y=[1.51,1.93,1.95,1.9,2.02,1.6,1.98,2.07,2.23,1.75,1.88,1.4,1.85,1.34,1.57,2.33,1.34,2.24,4.3,4.22,4.98,3.71,4.78,4.14,4.35,4.17,4.62,4.0,4.37,4.52,5.04,3.56,4.93,4.73,4.23,4.51], categories=[\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"short\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\",\"long\"], x_label=\"waiting\", y_label=\"duration\", variant=\"levels\""
)]

pub fn render(cfg: &KdeConfig) -> String {
    let multi = cfg.series.len() > 1 && cfg.y_series.len() == cfg.series.len();
    let groups: Vec<(&str, &[f64], &[f64])> = if multi {
        cfg.series
            .iter()
            .zip(cfg.y_series.iter())
            .map(|((name, xv), yv)| {
                let n = xv.len().min(yv.len());
                (name.as_str(), &xv[..n], &yv[..n])
            })
            .filter(|(_, xs, _)| !xs.is_empty())
            .collect()
    } else {
        let xs: &[f64] = cfg.series.first().map(|(_, v)| v.as_slice()).unwrap_or(&[]);
        let ys: &[f64] = cfg.y_values;
        let n = xs.len().min(ys.len());
        if n == 0 {
            vec![]
        } else {
            vec![("Series", &xs[..n], &ys[..n])]
        }
    };
    if groups.is_empty() {
        return String::new();
    }

    let (mut x0, mut x1) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, xs, ys) in &groups {
        for &v in *xs {
            x0 = x0.min(v);
            x1 = x1.max(v);
        }
        for &v in *ys {
            y0 = y0.min(v);
            y1 = y1.max(v);
        }
    }
    let xr = (x1 - x0).max(1e-9);
    let yr = (y1 - y0).max(1e-9);
    x0 -= xr * 0.12;
    x1 += xr * 0.12;
    y0 -= yr * 0.12;
    y1 += yr * 0.12;

    let grid_n = 50usize;
    let pad_l = 56i32;
    let pad_t = 20i32;
    let pad_r = 20i32;
    let pad_b = if groups.len() > 1 { 60i32 } else { 44i32 };
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        pad_l,
        pad_t,
        pad_b,
        pad_r,
        grid_n * grid_n * 90 * groups.len().max(1) + 4096,
    );
    f.open(cfg.title, false);
    f.y_grid(5, y0, y1, cfg.gridlines);
    f.x_grid(5, x0, x1, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cell_w = f.pw as f64 / grid_n as f64;
    let cell_h = f.ph as f64 / grid_n as f64;
    let n_levels = 5usize;

    for (gi, (_, xs, ys)) in groups.iter().enumerate() {
        let bwx = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(xs) };
        let bwy = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(ys) };

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

        let hue = group_hue(cfg, gi);
        let br = ((hue >> 16) & 0xFF) as f64;
        let bg = ((hue >> 8) & 0xFF) as f64;
        let bb = (hue & 0xFF) as f64;

        for gy in 0..grid_n {
            for gx in 0..grid_n {
                let t = (density[gy * grid_n + gx] / dmax).clamp(0.0, 1.0);
                if t < 0.06 {
                    continue;
                }
                let band = ((t * n_levels as f64).floor() as usize).min(n_levels - 1);
                let bt = (band as f64 + 1.0) / n_levels as f64;
                let r = (255.0 + (br - 255.0) * bt) as u32;
                let g = (255.0 + (bg - 255.0) * bt) as u32;
                let b = (255.0 + (bb - 255.0) * bt) as u32;
                let color = (r << 16) | (g << 8) | b;
                let hx = hex6(color);

                let left_band = if gx > 0 {
                    let lt = (density[gy * grid_n + gx - 1] / dmax).clamp(0.0, 1.0);
                    ((lt * n_levels as f64).floor() as usize).min(n_levels - 1)
                } else {
                    band
                };
                let below_band = if gy > 0 {
                    let bt2 = (density[(gy - 1) * grid_n + gx] / dmax).clamp(0.0, 1.0);
                    ((bt2 * n_levels as f64).floor() as usize).min(n_levels - 1)
                } else {
                    band
                };
                let is_edge = left_band != band || below_band != band;

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
                push_f2(&mut f.buf, if groups.len() > 1 { 0.16 + bt * 0.42 } else { 0.3 + bt * 0.65 });
                if is_edge {
                    push_b(&mut f.buf, b"\" stroke=\"#");
                    f.buf.extend_from_slice(&hex6(hue));
                    push_b(&mut f.buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.9");
                }
                push_b(&mut f.buf, b"\"/>");
            }
        }
    }

    let mut idx = 0i32;
    for (gi, (_, xs, ys)) in groups.iter().enumerate() {
        let hue = group_hue(cfg, gi);
        for i in 0..xs.len() {
            let px = f.pl as f64 + (xs[i] - x0) / (x1 - x0) * f.pw as f64;
            let py = f.pt as f64 + (1.0 - (ys[i] - y0) / (y1 - y0)) * f.ph as f64;
            push_b(&mut f.buf, b"<circle data-idx=\"");
            push_i(&mut f.buf, idx);
            push_b(&mut f.buf, b"\" cx=\"");
            push_f2(&mut f.buf, px);
            push_b(&mut f.buf, b"\" cy=\"");
            push_f2(&mut f.buf, py);
            push_b(&mut f.buf, b"\" r=\"2\" fill=\"#");
            f.buf.extend_from_slice(&hex6(hue));
            push_b(&mut f.buf, b"\" fill-opacity=\"0.6\"/>");
            idx += 1;
        }
    }

    if groups.len() > 1 {
        let leg_y = f.pt + f.ph + 34;
        let entry_w = (f.pw / groups.len() as i32).max(70);
        let leg_x0 = f.pl + (f.pw - entry_w * groups.len() as i32) / 2;
        for (gi, (name, _, _)) in groups.iter().enumerate() {
            let lx = leg_x0 + gi as i32 * entry_w;
            let hue = group_hue(cfg, gi);
            push_b(&mut f.buf, b"<rect x=\"");
            push_i(&mut f.buf, lx);
            push_b(&mut f.buf, b"\" y=\"");
            push_i(&mut f.buf, leg_y - 8);
            push_b(&mut f.buf, b"\" width=\"9\" height=\"9\" rx=\"2\" fill=\"#");
            f.buf.extend_from_slice(&hex6(hue));
            push_b(&mut f.buf, b"\"/>");
            push_b(&mut f.buf, b"<text x=\"");
            push_i(&mut f.buf, lx + 13);
            push_b(&mut f.buf, b"\" y=\"");
            push_i(&mut f.buf, leg_y);
            push_b(&mut f.buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"10\" fill=\"#475569\">");
            escape_xml(&mut f.buf, name);
            push_b(&mut f.buf, b"</text>");
        }
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
