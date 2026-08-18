use super::common::{data_bounds, finalize, polygon_area, voronoi_cell};
use super::config::HexbinConfig;
use crate::plot::statistical::common::{
    colorscale_color, hex6, lerp_rgb, push_b, push_f2, push_i, svg_open_rescalable, svg_title, Frame,
};

const STOPS: [u32; 4] = [0x475569, 0xb91c1c, 0xea580c, 0xfacc15];

fn mesh_color(t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let n = STOPS.len() - 1;
    let p = t * n as f64;
    let i0 = (p.floor() as usize).min(n);
    let i1 = (i0 + 1).min(n);
    let f = p - i0 as f64;
    lerp_rgb(STOPS[i0], STOPS[i1], f)
}

#[crate::chart_demo(
    "title=\"Turbidity - Density Voronoi Mesh\", x=[10.19,10.75,47.66,29.71,34.41,55.16,31.78,31.6,22.36,33.01,11.82,23.44,36.68,30.92,25.63,22.21,35.99,7.28,33.52,41.72,34.21,19.83,47.13,22.07,33.31,51.95,37.44,19.02,15.18,39.96,35.63,31.1,27.67,46.26,34.06,32.93,43.01,20.49,35.51,25.05,46.47,25.71,55.42,32.5,29.09,24.27,28.26,5.8,50.48,25.23,43.88,29.15,17.42,45.72,16.32,11.56,18.87,37.06,42.32,29.03,22.14,29.03,18.62,57.39,48.89,7.5,20.18,15.23,27.68,35.64,56.29,46.31,40.53,15.08,22.37,43.55,26.45,24.36,28.61,20.22,29.37,21.87,41.49,17.89,15.45,14.63,17.19,24.57,23.64,22.2,41.78,15.69,46.86,37.0,18.95,27.45,14.82,32.8,8.52,26.86,23.16,32.97,43.91,33.63,30.27,46.64,43.89,35.76,0.5,17.48,2.18,18.76,46.67,53.54,29.49,13.45,20.31,48.6,75.26,26.69,28.47,25.92,16.18,64.23,6.07,62.52,7.03,44.77,39.11,48.87,41.67,43.43,33.88,50.51,54.38,35.65,49.3,16.1,43.7,39.7,39.11,35.18,54.1,52.61,54.54,48.28,52.79,23.25,38.76,28.98,46.1,42.89,18.48,27.45,52.72,49.91,49.36,27.28,41.65,47.74,21.06,32.23,20.93,47.09,22.71,22.81,27.55,44.87,44.25,19.14,34.49,35.36,22.57,51.19,31.72,18.66,58.77,43.39,37.97,38.2,36.77,40.61,49.78,58.18,9.91,51.1,65.53,46.91,49.58,28.66,56.25,24.2,17.29,44.26,23.12,23.58,36.8,31.32,37.61,48.75,17.9,39.48,23.5,18.71,35.91,30.73,45.73,53.31,32.58,29.16,25.4,1.86,22.37,30.11,35.66,3.99,34.72,27.09,27.37,22.0,29.33,53.31,21.45,38.16,26.68,35.43,37.49,35.93,7.53,42.28,47.73,38.33,0.5,10.34,68.46,55.78,21.53,7.53,47.35,54.4,27.39,35.29,21.13,27.53,51.85,43.57,18.8,60.28,28.99,49.72,29.85,22.76,10.72,33.58,29.33,59.28,34.08,63.27,0.5,30.62,26.71,45.36,25.05,43.01,32.91,47.55,35.69,14.03,40.4,36.62,5.3,10.57,50.77,33.21,17.84,53.68,29.34,11.55,33.4,27.28,52.96,12.11,40.38,29.51,8.13,63.33,49.72,46.91,34.88,49.74,2.57,32.77,57.74,16.18,50.29,26.92,31.9,64.74,64.4,29.79,45.96,20.55,60.63,67.61,45.37,57.07,54.42,31.77,44.89,22.82,7.48,35.25,48.4,50.5,13.17,45.99,45.92,45.27,22.67,27.33,22.6,27.03,37.61,48.96,20.01,57.0,15.59,21.72,17.32,41.88,57.79,19.62,40.22,31.06,60.06,21.07,20.07,20.4,54.07,29.53,30.53,62.81,24.24,58.65,38.37,33.69,38.51,25.97,61.29,24.15,20.8,32.13,30.62,30.73,38.17,58.34,31.51,28.41,32.73,63.34,17.23,64.39,49.69,24.08,52.45,46.19,56.67,73.04,27.16,38.87,24.38,51.21,26.52,41.76,42.78,44.35,57.23,10.91,37.19,53.07,28.24,47.34,26.2,37.78,18.8,47.19,57.68,4.02,56.98,54.82,11.2,11.91,25.62,45.87,31.67,34.78,32.41,40.78,21.71,29.47,34.74,30.31,15.02,37.62,35.72,0.5,36.47,55.99,32.91,39.44,36.12,17.82,15.26,21.75,54.37,31.12,35.38,24.44,37.46,44.79,59.55,15.61,23.62,28.36,46.9,44.12,36.31,32.39,59.79,33.78,9.75,22.06,8.82,19.04,33.89,18.58,35.0,43.35,34.51,44.86,44.15,16.68,25.55,72.5,29.82,18.59,47.85,33.82,38.54,36.24,10.08,24.5,23.19,36.49,20.59,14.9,31.36,21.44,19.88,42.2,40.49,41.41,46.65,30.85,18.39,58.05,30.02,27.56,19.69,13.61,32.39,20.49,21.21,32.0,36.55,35.39,18.3,29.88,43.57,63.58,41.83,27.22,26.93,19.67,29.94,58.91,33.98,33.96,29.08,34.24,45.08,36.97,35.33,27.35,25.96,52.66,38.13,51.02,16.6,51.24,33.41,44.88,6.75,39.45,40.93,27.53,16.48,36.41,65.79,1.3,46.26,19.59,50.44,20.18,53.26,53.09,85.93,22.49,64.66,44.39,53.42,23.18,18.52,22.04,17.69,12.75,35.25,35.26,42.94,42.3,38.14,29.93,39.36,25.23,54.19,49.32,22.07,34.79,30.61,18.79,23.18,38.96,50.61,26.44,42.56,48.19,30.8,40.65,37.24,64.2,38.23,61.41,57.46,35.13,52.85,31.6,64.31,33.73,34.53,68.25,17.08,42.96,48.94,13.72,55.81,15.07,33.8,34.25,37.91,39.09,31.88,52.04,35.01,39.25,28.95,50.02,26.4,26.1,56.33,43.4,42.57,35.09,25.68,57.45,33.16,67.3,42.56,27.08,43.65,49.08,41.9,32.78,32.68,42.4,7.67,49.48,39.45,30.28,1.27,54.12,53.08,48.0,50.42,44.84,34.31,37.31,33.52,29.52,41.76,40.78,32.96,22.21,30.57,54.1,26.46,31.25,0.5,48.17,40.16,23.01,30.07,30.45,44.85,21.12,35.74,29.41,41.93,37.52,37.73,56.2,44.11,28.02,35.97,22.26,26.47,42.43,11.77,32.99,42.47,32.09,5.64,12.44,18.03,23.0,26.66,33.44,46.43,38.79,34.48,28.12,23.53,37.06,43.39,18.19,32.12,70.64,37.79,33.68,16.7,36.83,51.01,25.74,52.32,21.02,53.82,5.13,9.37,0.5,35.14,47.77,56.7,8.0,42.37,7.98,31.52,19.11,42.91,28.24,24.49,26.71,0.97,7.25,47.05,56.28,19.29,32.23,24.27,73.13,17.02,26.81,79.5,43.87,16.72,58.13,10.88,8.7,16.56,53.24,5.08,35.3,71.93,0.5,39.05,19.12,61.93,36.11,24.31,0.5,31.66,45.97,36.09,25.94,86.71,12.52,44.38,31.46,42.05,23.71,64.08,0.5,20.58,24.11,31.24,0.5,65.06,39.74,39.9,41.4,19.95,66.55,44.13,25.56,21.04,23.56,48.35,34.92,64.48,25.97,13.65,22.78,13.2,0.5,37.95,7.5,24.93,44.76,15.12,28.74,77.44,66.02,29.47,66.99,13.14,45.76,30.77,18.82,82.53,24.69,46.32,70.68,63.94,16.43,30.67,2.98,25.25,14.88,27.67,58.4,26.78,37.18,40.27,30.07,53.85,40.87,0.5], y=[0.5,13.69,5.94,0.5,8.15,35.85,3.25,4.72,17.2,14.69,9.41,21.6,9.67,13.72,1.96,12.43,0.5,15.26,17.92,8.36,11.74,0.5,13.16,14.69,18.51,25.2,20.2,0.5,11.7,7.91,17.01,0.91,12.22,18.98,19.56,17.47,27.69,2.96,26.38,8.25,29.26,18.03,20.04,21.18,0.5,14.74,3.72,0.5,3.97,7.12,0.5,12.56,8.14,0.5,14.47,11.6,0.5,25.35,12.23,9.33,5.41,13.04,17.8,14.96,8.87,4.85,0.5,11.64,13.08,0.5,10.39,9.2,7.03,0.5,14.0,18.21,19.25,5.78,11.4,0.5,15.73,2.82,0.5,12.74,4.31,11.65,19.66,10.41,4.34,19.76,3.61,17.15,0.5,9.78,8.42,30.02,15.43,21.01,27.65,28.91,34.02,29.44,24.38,20.88,30.57,7.8,25.33,29.38,26.95,32.1,22.81,32.57,30.07,26.48,19.45,25.65,43.32,27.99,20.82,24.19,32.25,31.69,39.0,28.33,42.05,11.87,20.74,30.39,26.09,19.87,43.3,12.05,28.38,33.08,17.15,29.5,10.53,24.72,25.67,23.6,18.69,18.67,32.14,19.8,16.12,14.8,14.72,29.27,21.19,24.61,14.1,15.88,28.27,19.76,27.67,30.81,33.73,22.73,14.08,20.77,24.14,15.8,19.27,46.77,42.02,18.38,28.8,29.68,21.99,24.37,25.76,12.8,18.11,17.72,17.92,31.11,16.8,14.44,7.4,18.88,0.5,10.52,24.68,28.44,10.36,30.35,21.42,30.56,23.57,22.61,27.24,39.96,47.62,32.72,40.94,44.69,29.58,32.69,40.17,55.07,39.41,40.65,25.3,35.3,30.84,36.86,33.99,20.65,39.47,43.43,27.5,26.57,49.54,49.8,37.87,23.28,32.4,30.05,37.87,28.41,43.94,45.62,41.89,36.94,40.01,46.97,55.4,53.12,39.24,21.22,30.2,42.78,45.93,24.76,49.03,28.98,43.74,33.64,38.04,23.26,38.19,27.68,33.86,25.99,40.99,36.51,37.13,47.36,32.57,36.47,29.48,42.08,46.73,40.78,45.47,25.79,31.41,44.35,23.6,34.82,35.2,43.16,29.44,30.07,39.6,31.09,30.79,37.45,37.08,42.05,44.96,36.5,49.14,30.26,27.73,47.98,35.85,36.81,22.82,47.97,33.5,49.05,45.71,30.21,38.64,72.56,47.45,36.15,56.47,37.64,57.08,62.69,62.74,61.43,59.98,62.98,53.29,46.91,40.86,43.7,51.18,43.56,35.83,57.29,61.92,55.01,49.82,49.85,41.46,51.7,50.96,48.61,53.43,57.36,38.77,44.8,67.82,58.71,49.36,53.31,53.94,39.1,50.26,41.69,46.06,47.77,50.4,52.98,43.27,53.75,55.83,62.09,47.62,44.35,43.57,51.22,45.41,59.05,57.47,52.33,62.82,45.89,68.66,60.35,55.46,45.36,75.84,62.04,48.1,58.74,62.97,64.09,55.34,51.68,47.75,55.43,61.48,39.51,51.33,42.05,53.55,70.98,46.65,46.73,60.23,54.05,41.35,61.97,43.26,49.68,45.52,47.51,72.98,46.08,54.6,45.93,53.39,48.08,54.94,58.3,61.82,58.86,51.78,64.07,82.4,58.31,52.0,68.7,79.57,80.29,62.33,67.9,84.75,64.48,58.86,58.84,70.28,62.07,50.38,47.94,63.08,80.88,58.95,58.61,55.1,69.63,64.37,72.72,62.91,61.66,71.75,67.07,56.99,57.07,77.8,54.86,55.55,60.54,65.96,63.28,62.12,65.92,68.74,62.82,72.6,60.73,66.01,62.14,66.63,57.76,68.16,65.02,58.25,69.14,87.71,68.98,69.66,75.43,73.27,70.38,65.16,54.51,65.47,69.85,72.94,59.7,57.41,66.54,77.33,86.1,80.33,57.47,79.02,66.59,54.78,63.79,74.87,79.09,56.33,66.84,73.35,62.11,63.24,64.45,53.67,61.02,71.62,67.31,81.5,68.52,84.85,66.94,73.51,63.28,59.75,89.61,90.76,97.31,76.51,83.68,92.58,83.41,94.56,82.05,64.21,84.15,89.38,87.74,90.42,69.83,80.76,76.68,92.52,81.59,96.01,83.65,83.37,69.8,98.66,92.59,88.97,89.31,98.94,75.17,76.2,77.8,86.09,82.24,74.38,89.77,67.78,60.62,64.12,75.71,91.55,78.87,78.66,70.15,89.21,61.58,76.41,91.43,71.23,87.69,78.13,67.55,85.71,83.99,97.03,56.89,94.12,69.64,79.89,84.85,77.78,91.17,69.25,79.68,71.33,81.14,80.56,77.26,97.44,88.84,77.65,91.63,72.91,71.41,66.88,92.39,96.08,68.45,64.3,88.03,78.59,93.7,69.28,71.51,82.01,79.85,77.53,62.87,73.6,86.08,91.07,56.81,76.4,68.36,80.99,76.99,97.45,95.51,99.5,99.5,96.91,88.01,98.79,84.61,99.5,97.53,96.47,79.53,99.5,74.1,94.59,85.96,93.35,92.38,95.74,98.59,90.64,99.5,98.39,98.99,92.81,83.31,90.32,99.5,91.1,92.91,94.71,88.21,94.65,96.59,89.77,78.92,94.35,88.47,84.15,91.54,97.3,87.65,92.69,99.5,92.77,77.87,97.4,98.12,79.24,97.67,79.4,80.14,88.61,83.23,87.06,87.14,89.58,97.41,99.5,87.29,82.79,99.5,89.43,93.07,99.5,99.5,98.86,97.07,99.5,94.07,79.54,83.45,92.01,90.21,79.61,90.44,87.46,89.2,98.84,99.5,97.67,75.35,86.52,90.49,88.86,86.58,99.5,90.15,89.81,99.5,97.03,84.57,87.12,99.5,99.14,36.67,16.98,85.63,14.41,74.13,66.26,36.69,61.96,74.11,71.72,25.6,61.07,14.29,22.01,45.0,0.5,60.9,99.5,21.19,51.15,99.5,46.51,66.32,0.5,73.07,27.96,90.05,54.08,83.26,13.72,19.58,35.9,50.73,58.25,0.5,32.32,80.47,71.83,40.05,54.88,71.3,9.14,0.5,47.64,0.5,89.4,1.51,99.5,82.83,38.6,60.22,50.05,29.62,24.81,30.41,50.81,29.64,33.16,91.83,0.5,32.17,56.79,99.5,87.22,64.07,20.78,56.84,28.45,43.69,46.38,83.64,22.58,37.6,49.14,38.63,25.85,37.13,99.5,21.63,77.1,46.62,87.33,19.76,22.36,35.71,59.26,43.69,4.63,67.11,90.19,77.13,18.91,45.68,57.37,61.69,69.92,32.07,31.36,58.69,36.74,13.73,32.72,68.57,15.42,3.8,2.22,53.93,97.09,58.2,22.84], width=760, height=980, variant=\"neural\""
)]
pub fn render(cfg: &HexbinConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return String::new();
    }
    let bounds = match data_bounds(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 20, 44, 22, 150, n * 90 + 8192);
    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;

    svg_open_rescalable(&mut f.buf, f.w, f.h, f.pl, f.pt, f.pw, f.ph);
    svg_title(&mut f.buf, cfg.title, f.pl + f.pw / 2, 26);

    let xr = (bounds.xmax - bounds.xmin).max(1e-9);
    let yr = (bounds.ymax - bounds.ymin).max(1e-9);
    let pts: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let px = f.pl as f64 + (cfg.x_values[i] - bounds.xmin) / xr * f.pw as f64;
            let py = f.pt as f64 + f.ph as f64 - (cfg.y_values[i] - bounds.ymin) / yr * f.ph as f64;
            (px, py)
        })
        .collect();

    let rect = (f.pl as f64, f.pt as f64, (f.pl + f.pw) as f64, (f.pt + f.ph) as f64);
    let cells: Vec<Vec<(f64, f64)>> = (0..n).map(|i| voronoi_cell(i, &pts, rect)).collect();
    let areas: Vec<f64> = cells.iter().map(|c| polygon_area(c)).collect();

    let has_values = cfg.values.len() >= n;
    let raw: Vec<f64> = if has_values {
        cfg.values[..n].to_vec()
    } else {
        areas.iter().map(|a| 1.0 / a.max(1.0)).collect()
    };
    let vmin = raw.iter().cloned().fold(f64::INFINITY, f64::min);
    let vmax = raw.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| raw[a].partial_cmp(&raw[b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank_t = vec![0.0_f64; n];
    for (r, &i) in order.iter().enumerate() {
        rank_t[i] = if n > 1 { r as f64 / (n - 1) as f64 } else { 0.0 };
    }

    let color_at = |t: f64| -> u32 {
        if cfg.colorscale.is_empty() {
            mesh_color(t)
        } else {
            colorscale_color(cfg.colorscale, t)
        }
    };

    push_b(&mut f.buf, b"<g fill=\"none\">");
    for &i in order.iter().rev() {
        if cells[i].len() < 3 {
            continue;
        }
        let col = color_at(rank_t[i]);
        let op = 0.35 + rank_t[i] * 0.55;
        let sw = 0.4 + rank_t[i] * 0.5;
        draw_mesh_cell(&mut f.buf, i, &cells[i], col, raw[i], op, sw);
    }
    push_b(&mut f.buf, b"</g>");

    let (mut ccx, mut ccy, mut cn) = (0.0, 0.0, 0.0);
    for (k, &(px, py)) in pts.iter().enumerate() {
        if rank_t[k] > 0.55 {
            ccx += px;
            ccy += py;
            cn += 1.0;
        }
    }
    if cn > 0.0 {
        ccx /= cn;
        ccy /= cn;
    } else {
        ccx = f.pl as f64 + f.pw as f64 / 2.0;
        ccy = f.pt as f64 + f.ph as f64 / 2.0;
    }

    let mut core_r_sum = 0.0;
    let mut core_r_n = 0.0;
    for (k, &(px, py)) in pts.iter().enumerate() {
        if rank_t[k] > 0.55 {
            let dx = px - ccx;
            let dy = py - ccy;
            core_r_sum += (dx * dx + dy * dy).sqrt();
            core_r_n += 1.0;
        }
    }
    let core_r = if core_r_n > 0.0 { core_r_sum / core_r_n } else { f.pw.min(f.ph) as f64 * 0.3 };
    let streak_cutoff = core_r * 2.3;

    push_b(&mut f.buf, b"<g stroke=\"#dc2626\" stroke-width=\"0.9\" stroke-linecap=\"round\">");
    for (k, &(px, py)) in pts.iter().enumerate() {
        if rank_t[k] >= 0.3 {
            continue;
        }
        let dx = px - ccx;
        let dy = py - ccy;
        let d = (dx * dx + dy * dy).sqrt().max(1e-6);
        if d > streak_cutoff {
            continue;
        }
        let ex = px + dx / d * 11.0;
        let ey = py + dy / d * 11.0;
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, px);
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, py);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, ex);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, ey);
        push_b(&mut f.buf, b"\" stroke-opacity=\"0.55\"/>");
    }
    push_b(&mut f.buf, b"</g>");

    legend(&mut f, vmin, vmax, ink, sub, &color_at);
    finalize(f, cfg)
}

fn draw_mesh_cell(buf: &mut Vec<u8>, idx: usize, poly: &[(f64, f64)], col: u32, val: f64, op: f64, sw: f64) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, val);
    push_b(buf, b"\" d=\"M ");
    push_f2(buf, poly[0].0);
    buf.push(b' ');
    push_f2(buf, poly[0].1);
    for &(x, y) in &poly[1..] {
        push_b(buf, b" L ");
        push_f2(buf, x);
        buf.push(b' ');
        push_f2(buf, y);
    }
    push_b(buf, b" Z\" stroke=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, sw);
    push_b(buf, b"\" stroke-opacity=\"");
    push_f2(buf, op);
    push_b(buf, b"\"/>");
}

fn legend(f: &mut Frame, vmin: f64, vmax: f64, ink: u32, sub: u32, color_at: &dyn Fn(f64) -> u32) {
    let bar_x = f.pl + f.pw + 22;
    let bar_y = f.pt + 10;
    let bar_w = 14;
    let bar_h = (f.ph - 70).max(60);

    push_b(&mut f.buf, b"<defs><linearGradient id=\"spmesh\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    for s in 0..8 {
        let t = s as f64 / 7.0;
        let col = color_at(t);
        push_b(&mut f.buf, b"<stop offset=\"");
        push_f2(&mut f.buf, t);
        push_b(&mut f.buf, b"\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\"/>");
    }
    push_b(&mut f.buf, b"</linearGradient></defs>");

    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y - 8);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#");
    f.buf.extend_from_slice(&hex6(sub));
    push_b(&mut f.buf, b"\" letter-spacing=\"0.6\">DENSITY</text>");

    push_b(&mut f.buf, b"<rect x=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y);
    push_b(&mut f.buf, b"\" width=\"");
    push_i(&mut f.buf, bar_w);
    push_b(&mut f.buf, b"\" height=\"");
    push_i(&mut f.buf, bar_h);
    push_b(&mut f.buf, b"\" fill=\"url(#spmesh)\" stroke=\"#334155\" stroke-width=\"0.6\" rx=\"2\"/>");

    let tx = bar_x + bar_w + 6;
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + 9);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\">");
    push_f2(&mut f.buf, vmax);
    push_b(&mut f.buf, b"</text>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + bar_h);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\">");
    push_f2(&mut f.buf, vmin);
    push_b(&mut f.buf, b"</text>");

    let py = bar_y + bar_h + 30;
    push_b(&mut f.buf, b"<line x1=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y1=\"");
    push_i(&mut f.buf, py);
    push_b(&mut f.buf, b"\" x2=\"");
    push_i(&mut f.buf, bar_x + 14);
    push_b(&mut f.buf, b"\" y2=\"");
    push_i(&mut f.buf, py);
    push_b(&mut f.buf, b"\" stroke=\"#dc2626\" stroke-width=\"1.1\" stroke-linecap=\"round\"/>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, py + 3);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\">Particle tracking</text>");
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::common::make_frame;

    fn cfg<'a>(x: &'a [f64], y: &'a [f64]) -> HexbinConfig<'a> {
        HexbinConfig {
            title: "Test",
            x_values: x,
            y_values: y,
            width: 760,
            height: 980,
            ..HexbinConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<f64>, Vec<f64>) {
        let x: Vec<f64> = (0..n).map(|i| ((i as f64 * 0.31).sin() * 0.5 + 0.5) * 100.0).collect();
        let y: Vec<f64> = (0..n).map(|i| ((i as f64 * 0.47).cos() * 0.5 + 0.5) * 100.0).collect();
        (x, y)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("hexbin/neural.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/hexbin-neural.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_edge_path_per_site() {
        let (x, y) = synth(40);
        let html = render(&cfg(&x, &y));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=\"").count(), x.len());
    }

    #[test]
    fn mesh_cells_have_no_fill_and_carry_a_stroke() {
        let (x, y) = synth(30);
        let html = render(&cfg(&x, &y));
        assert!(html.contains("<g fill=\"none\">"));
        assert!(html.matches("stroke-opacity=\"").count() >= x.len());
    }

    #[test]
    fn every_mesh_ramp_stop_stays_within_valid_hex_bounds() {
        for i in 0..=20 {
            let t = i as f64 / 20.0;
            let hx = hex6(mesh_color(t));
            assert_eq!(hx.len(), 6);
        }
    }

    #[test]
    fn voronoi_cells_still_tile_the_plot_rect_without_gaps() {
        let (x, y) = synth(24);
        let bounds = data_bounds(&cfg(&x, &y)).unwrap();
        let f = make_frame(&cfg(&x, &y));
        let xr = (bounds.xmax - bounds.xmin).max(1e-9);
        let yr = (bounds.ymax - bounds.ymin).max(1e-9);
        let pts: Vec<(f64, f64)> = (0..x.len())
            .map(|i| {
                let px = f.pl as f64 + (x[i] - bounds.xmin) / xr * f.pw as f64;
                let py = f.pt as f64 + f.ph as f64 - (y[i] - bounds.ymin) / yr * f.ph as f64;
                (px, py)
            })
            .collect();
        let rect = (f.pl as f64, f.pt as f64, (f.pl + f.pw) as f64, (f.pt + f.ph) as f64);
        let rect_area = (rect.2 - rect.0) * (rect.3 - rect.1);
        let total: f64 = (0..pts.len()).map(|i| polygon_area(&voronoi_cell(i, &pts, rect))).sum();
        assert!((total - rect_area).abs() / rect_area < 1e-6, "cells do not tile the rect: {total} vs {rect_area}");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty: Vec<f64> = vec![];
        assert!(render(&cfg(&empty, &empty)).is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_particle_field_stays_fast() {
        let (x, y) = synth(800);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &y));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 1500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
