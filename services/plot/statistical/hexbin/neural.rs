use super::common::{data_bounds, finalize, polygon_area, voronoi_cell};
use super::config::HexbinConfig;
use crate::plot::statistical::common::{
    colorscale_color, hash01, hex6, lerp_rgb, push_b, push_f2, push_i, svg_open_rescalable, svg_title, Frame,
};

const STOPS: [u32; 5] = [0x475569, 0x7f1d1d, 0xea580c, 0xfbbf24, 0xfef9c3];
const FRAMES: usize = 3;
const JITTER_PX: f64 = 2.6;

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
    "title=\"Turbidity - Density Voronoi Mesh\", x=[32.97,31.22,39.38,15.87,39.48,8.53,57.18,39.41,9.12,33.97,46.31,22.72,31.62,34.35,14.13,48.67,47.93,29.08,40.12,46.85,24.93,12.08,24.7,33.46,46.41,30.18,22.41,12.26,31.26,32.78,26.88,9.19,53.65,43.9,41.66,28.25,28.56,43.55,46.27,26.49,11.53,30.1,34.72,34.8,20.35,34.42,28.63,35.65,57.38,31.36,7.45,26.48,21.33,32.73,23.37,34.37,40.5,37.58,38.47,43.76,29.23,47.22,28.53,22.16,36.68,40.49,35.88,48.34,27.12,39.86,37.13,21.17,38.14,26.9,41.95,35.75,33.93,47.87,41.21,36.56,46.17,23.45,37.78,33.8,25.28,35.3,39.12,23.47,39.68,34.92,24.0,37.63,35.95,15.04,38.12,28.53,26.54,29.95,35.68,13.49,11.68,37.45,35.89,24.62,13.59,27.08,19.63,5.04,14.89,19.82,23.72,1.83,20.82,18.33,30.25,27.51,21.63,34.13,31.98,17.98,27.6,10.99,24.88,34.36,15.4,14.61,38.85,19.04,15.07,44.24,10.84,10.91,39.29,36.15,36.39,29.96,13.07,24.07,40.1,40.92,39.05,23.0,29.7,40.7,34.8,30.61,20.83,25.7,30.91,45.59,23.33,35.94,30.89,30.4,42.88,17.21,35.13,24.85,31.49,43.12,28.0,47.25,39.64,20.9,24.08,20.43,23.82,21.79,43.11,28.5,52.08,26.44,32.4,34.1,48.0,35.39,40.34,39.85,32.14,34.9,34.97,34.85,41.03,21.35,46.09,26.5,34.56,44.39,36.11,27.86,27.71,36.42,45.97,40.56,40.42,25.61,37.02,27.94,16.19,22.43,21.58,11.67,24.48,9.43,10.47,20.67,24.61,21.21,11.74,37.9,30.48,29.52,7.78,10.89,34.58,34.65,31.61,29.57,33.39,8.7,29.28,15.3,25.23,17.38,30.09,17.53,20.39,34.46,26.64,34.1,23.96,33.52,26.17,31.54,34.88,29.76,39.06,50.65,18.9,20.88,25.39,37.57,25.24,41.4,33.47,31.73,26.35,44.56,38.76,50.59,30.0,38.87,43.37,11.31,23.92,34.66,43.86,42.39,32.52,23.77,33.45,34.23,61.97,16.08,29.75,51.89,33.03,30.59,19.74,46.19,43.94,39.69,59.93,24.45,29.44,34.04,48.54,27.73,28.89,33.82,42.22,14.33,43.51,10.05,39.81,7.8,36.35,29.74,31.24,23.7,43.46,22.4,27.83,32.86,12.04,30.62,28.98,45.71,44.72,36.55,46.47,46.84,42.97,44.54,48.99,40.5,35.63,37.79,32.84,31.4,29.73,35.36,41.63,46.82,45.37,32.67,33.67,34.0,47.85,47.19,28.77,38.72,42.06,45.59,35.49,32.31,43.27,43.22,39.55,43.42,20.39,31.25,20.08,44.93,33.85,29.3,31.44,20.14,17.99,31.12,2.8,11.2,32.72,33.37,26.38,28.09,31.14,17.3,25.9,15.13,10.72,36.71,20.43,20.28,25.38,19.14,32.31,18.42,25.06,28.54,15.24,39.83,22.34,30.55,28.34,30.77,28.48,36.38,32.38,21.84,13.68,57.63,19.51,40.83,43.81,41.65,18.97,21.66,51.01,28.14,47.54,31.5,34.26,32.87,30.46,48.92,48.94,52.37,46.57,24.87,8.57,42.32,47.25,34.06,37.22,27.44,42.01,29.26,26.55,35.56,56.76,48.94,35.1,18.91,24.62,31.76,27.86,38.4,38.06,18.9,22.09,21.59,39.11,29.21,28.53,36.42,27.2,38.41,34.5,27.96,24.38,29.05,30.19,28.92,38.94,43.95,39.99,25.05,25.64,24.71,30.32,23.78,24.79,22.36,21.29,25.1,28.9,47.03,23.96,6.54,13.1,27.42,3.11,44.37,13.37,31.59,40.28,30.85,23.16,34.27,28.71,24.55,12.11,26.73,11.39,20.73,34.39,33.95,46.51,21.71,18.05,9.18,30.85,21.97,47.5,35.61,7.12,33.67,39.4,27.24,20.33,14.45,66.43,31.46,21.01,21.8,46.87,24.2,30.24,54.33,23.97,42.31,11.92,47.82,45.99,22.48,41.24,38.7,32.48,38.53,38.08,26.11,29.84,15.12,31.75,45.78,21.13,22.58,51.43,20.2,32.37,41.08,40.57,25.86,34.67,52.34,42.94,22.65,55.0,12.05,44.99,37.21,29.9,29.14,40.61,43.45,23.51,25.9,33.13,41.96,36.24,35.21,40.04,28.65,35.37,29.59,36.59,37.02,49.5,44.03,31.54,43.46,34.33,56.26,36.24,41.85,40.21,34.42,49.46,42.69,36.06,35.13,28.39,37.45,38.96,33.85,39.64,35.19,31.56,22.38,35.5,35.26,48.41,39.96,54.64,25.86,23.04,15.37,31.97,27.56,47.45,45.87,29.42,31.13,25.02,25.57,42.84,28.54,14.62,30.12,32.12,1.55,18.96,24.52,41.0,23.03,24.72,31.45,27.08,18.08,31.18,38.1,8.25,24.42,31.18,37.81,40.01,28.98,13.32,24.16,26.73,24.88,25.04,32.31,18.2,17.02,23.2,24.46,30.81,31.05,18.9,35.62,29.23,19.5,31.13,40.14,30.99,40.82,41.97,47.65,36.48,52.64,21.46,48.57,26.23,29.18,23.46,33.91,40.13,14.85,27.65,36.97,25.49,26.74,41.61,41.95,39.96,31.58,29.52,32.89,37.38,25.31,34.36,23.75,27.43,36.09,25.15,25.69,16.84,22.07,32.71,25.09,16.82,15.79,34.93,28.84,32.47,26.38,34.61,39.65,29.41,17.63,26.58,40.33,23.86,32.29,41.79,21.08,32.49,25.73,21.64,34.45,29.97,26.38,58.9,4.8,21.58,80.24,13.18,0.5,29.82,19.5,75.81,34.58,0.5,10.01,17.02,0.5,29.72,0.5,0.5,83.53,0.5,20.09,0.5,31.81,49.72,64.49,14.59,28.17,13.37,54.2,64.02,49.06,9.74,48.9,0.5,50.13,0.5,5.06,29.1,29.98,6.44,46.47,26.23,4.12,0.5,75.32,9.31,62.94,37.97,43.92,0.5,17.39,36.34,51.79,7.8,27.01,11.16,26.16,2.37,32.18,37.05,0.5,52.76,9.17,48.81,17.71,46.72,0.5,4.21,22.89,63.12,43.79,52.13,11.73,10.38,48.82,16.69,0.5,41.0,21.52,55.17,25.52,83.35,68.86,38.39,41.8,40.5,66.63,62.6,27.34,7.9,18.86], y=[1.3,22.7,13.96,4.13,16.85,7.94,11.52,20.83,7.31,14.36,15.16,6.86,11.04,17.66,12.72,14.99,6.54,0.5,12.95,14.6,7.11,12.67,6.71,6.96,0.5,0.5,6.92,10.25,0.5,7.31,8.06,26.57,23.57,1.21,11.08,4.69,11.32,0.5,1.91,6.7,9.56,6.16,8.18,6.22,13.73,5.15,21.58,10.44,11.13,17.29,23.41,0.5,8.41,4.87,3.41,21.03,22.05,14.54,16.18,8.64,3.85,27.73,18.78,14.73,9.77,7.96,26.95,4.78,26.85,0.5,0.5,15.48,6.25,13.82,9.35,5.68,8.63,18.62,18.83,11.99,9.42,10.35,14.48,5.53,4.73,17.81,13.56,13.11,13.23,11.49,16.58,12.36,10.32,24.3,11.91,20.05,18.02,7.25,4.59,10.02,17.15,23.86,19.57,15.6,21.03,13.69,20.91,12.02,26.9,18.34,15.31,4.87,30.03,28.75,27.92,15.13,24.08,28.99,12.32,22.0,37.16,21.38,24.36,20.35,24.26,12.61,10.93,7.99,17.88,12.26,29.51,25.15,21.47,17.46,12.1,20.37,36.79,17.27,20.82,34.46,24.16,27.33,29.97,23.34,21.83,27.86,21.56,29.41,25.22,22.97,21.17,27.29,17.14,37.57,19.53,25.69,25.44,17.32,21.93,13.49,47.7,30.52,29.89,16.68,27.84,24.37,32.93,28.96,34.04,41.99,23.05,35.62,20.78,24.53,24.78,15.42,19.64,30.18,2.22,36.89,19.77,41.63,32.21,21.33,23.86,30.01,24.47,27.83,20.76,27.75,47.11,24.01,24.31,33.22,25.69,17.84,27.48,31.52,17.38,32.85,39.78,38.42,33.66,16.89,34.02,23.64,31.46,26.36,40.65,43.59,21.64,30.79,25.2,30.36,38.87,34.25,33.95,36.31,40.11,31.1,23.54,28.24,26.17,37.45,33.69,28.51,34.29,43.85,27.2,28.73,29.26,31.09,34.84,40.16,23.59,47.78,36.44,29.81,45.58,45.01,26.49,39.81,50.71,11.66,36.23,33.46,41.54,41.7,34.63,42.57,43.89,58.53,33.58,42.16,47.7,26.38,32.12,24.24,53.35,38.1,49.16,39.19,34.82,47.5,42.6,41.91,51.06,41.27,46.51,37.34,38.29,30.59,43.0,28.66,40.47,37.76,50.14,48.17,50.39,31.83,48.66,55.83,43.73,47.92,55.29,34.41,29.63,35.04,33.54,37.95,36.93,30.74,34.28,30.81,23.09,37.92,46.44,42.62,47.19,46.15,50.32,58.18,49.39,49.14,46.47,48.13,48.26,59.7,54.12,38.82,47.76,43.41,46.76,47.05,49.42,44.13,39.87,52.42,47.52,58.34,43.64,48.3,50.47,56.71,51.78,49.7,53.87,39.56,55.8,53.6,53.0,50.58,61.51,54.7,51.04,49.36,47.55,55.49,41.87,56.41,52.17,52.52,39.82,44.75,47.41,53.74,47.12,52.26,55.15,54.51,48.85,44.12,49.53,55.75,47.06,51.96,47.93,47.88,50.01,44.58,57.44,46.69,50.49,55.27,49.77,44.4,42.18,47.0,57.48,48.51,39.27,51.75,59.92,46.04,46.34,42.44,50.67,47.62,62.45,60.53,65.03,58.87,60.23,62.19,59.46,58.65,53.88,70.8,43.04,63.78,43.62,55.82,59.39,55.23,56.08,46.3,57.0,57.97,40.29,57.5,74.18,53.12,50.46,46.74,60.32,65.75,55.34,62.02,60.83,63.57,64.6,58.05,56.17,53.05,67.22,57.9,72.98,65.29,61.86,43.91,49.0,71.7,62.35,55.22,51.65,50.61,57.2,72.83,72.31,73.66,50.26,59.85,78.36,61.45,59.33,61.51,59.35,62.12,75.98,73.22,68.19,63.18,63.16,60.2,57.31,58.51,66.15,58.01,62.53,60.82,68.79,70.29,67.77,61.42,59.04,69.02,67.7,70.83,66.16,60.68,63.08,68.3,56.28,55.72,68.18,71.83,63.73,84.31,65.67,86.33,72.44,63.55,71.43,70.11,65.32,78.42,76.53,82.28,77.46,69.41,64.39,78.95,76.19,70.21,72.06,74.94,72.63,74.56,63.24,63.09,69.82,68.63,65.07,82.84,78.76,79.36,67.71,60.01,56.01,61.15,70.27,78.9,58.4,75.34,68.81,65.99,84.1,63.43,86.88,73.94,75.73,72.93,58.63,70.01,57.9,70.6,65.72,72.43,63.09,64.71,68.81,84.52,78.67,82.78,67.29,75.82,81.74,83.59,71.92,73.65,75.18,72.88,75.69,76.49,80.2,74.2,77.86,72.95,96.92,70.42,73.38,66.81,80.13,78.15,79.62,72.13,68.58,74.3,74.05,66.25,65.54,72.22,79.45,76.56,94.64,74.31,84.63,80.47,82.86,89.53,68.28,86.34,90.79,74.76,70.15,87.13,80.43,84.77,85.29,79.15,78.18,73.64,83.51,80.4,89.53,90.26,92.7,83.02,90.72,73.94,87.57,69.97,80.75,78.41,79.19,91.89,74.09,76.01,90.21,84.35,85.34,86.98,84.97,81.1,95.7,79.45,63.91,90.83,89.64,90.04,89.68,86.09,92.35,82.94,86.11,89.85,91.45,92.02,98.05,87.7,92.69,88.93,89.6,86.4,83.47,94.61,93.24,87.43,80.29,84.57,90.37,94.37,84.88,95.24,76.06,88.54,85.28,89.34,80.54,75.89,91.1,85.03,95.0,95.54,89.71,90.96,92.64,96.82,86.2,91.9,99.5,87.71,91.58,99.14,93.32,89.01,94.73,99.5,92.73,99.5,95.49,96.74,86.44,90.76,80.54,91.41,98.37,93.85,92.12,96.04,90.77,99.5,96.98,99.5,72.39,53.07,0.5,77.4,69.5,0.5,24.09,32.98,99.5,52.16,87.67,71.14,82.58,88.53,22.52,76.49,73.67,25.42,88.86,50.46,51.54,35.85,53.4,9.86,99.5,39.63,11.5,48.77,95.85,99.5,20.97,62.02,48.92,39.8,5.03,80.17,74.3,99.5,86.27,99.5,16.6,62.49,18.26,61.93,63.38,63.19,78.95,99.5,71.76,84.27,88.24,80.08,38.86,52.95,0.5,31.62,99.5,11.12,61.03,63.47,86.86,55.03,99.5,55.67,48.2,14.03,58.82,14.6,27.36,0.5,80.06,50.26,87.18,24.41,34.63,50.75,21.38,83.6,60.07,5.11,37.28,99.5,47.41,67.06,24.73,0.5,51.03,7.86,39.51,52.9], width=820, height=1060, variant=\"neural\""
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
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 20, 46, 24, 160, n * FRAMES * 90 + 8192);
    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;

    svg_open_rescalable(&mut f.buf, f.w, f.h, f.pl, f.pt, f.pw, f.ph);
    svg_title(&mut f.buf, cfg.title, f.pl + f.pw / 2, 28);
    push_b(
        &mut f.buf,
        b"<defs><filter id=\"nglow\" x=\"-80%\" y=\"-80%\" width=\"260%\" height=\"260%\" color-interpolation-filters=\"sRGB\"><feGaussianBlur stdDeviation=\"2.1\" result=\"b\"/><feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter></defs>",
    );

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

    let mut frame_cells: Vec<Vec<Vec<(f64, f64)>>> = Vec::with_capacity(FRAMES);
    frame_cells.push(cells);
    for k in 1..FRAMES {
        let jpts: Vec<(f64, f64)> = (0..n)
            .map(|i| {
                let jx = (hash01(i * 7919 + k * 104729) * 2.0 - 1.0) * JITTER_PX;
                let jy = (hash01(i * 7919 + k * 104729 + 3) * 2.0 - 1.0) * JITTER_PX;
                (pts[i].0 + jx, pts[i].1 + jy)
            })
            .collect();
        let jcells: Vec<Vec<(f64, f64)>> = (0..n).map(|i| voronoi_cell(i, &jpts, rect)).collect();
        frame_cells.push(jcells);
    }

    push_b(&mut f.buf, b"<g fill=\"none\" filter=\"url(#nglow)\">");
    for fk in 0..FRAMES {
        for &i in order.iter().rev() {
            if rank_t[i] < 0.5 {
                break;
            }
            if frame_cells[fk][i].len() < 3 {
                continue;
            }
            let col = color_at(rank_t[i]);
            let sw = 1.5 + rank_t[i] * 1.5;
            draw_glow_cell(&mut f.buf, &frame_cells[fk][i], col, sw);
        }
    }
    push_b(&mut f.buf, b"</g>");

    push_b(&mut f.buf, b"<g fill=\"none\">");
    for fk in 0..FRAMES {
        for &i in order.iter().rev() {
            if frame_cells[fk][i].len() < 3 {
                continue;
            }
            let col = color_at(rank_t[i]);
            let op = (0.32 + rank_t[i] * 0.5) * if FRAMES > 1 { 0.6 } else { 1.0 };
            let sw = 0.35 + rank_t[i] * 0.45;
            if fk == 0 {
                draw_mesh_cell(&mut f.buf, i, &frame_cells[fk][i], col, raw[i], op, sw);
            } else {
                draw_mesh_cell_plain(&mut f.buf, &frame_cells[fk][i], col, op, sw);
            }
        }
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

fn path_d(buf: &mut Vec<u8>, poly: &[(f64, f64)]) {
    push_b(buf, b"M ");
    push_f2(buf, poly[0].0);
    buf.push(b' ');
    push_f2(buf, poly[0].1);
    for &(x, y) in &poly[1..] {
        push_b(buf, b" L ");
        push_f2(buf, x);
        buf.push(b' ');
        push_f2(buf, y);
    }
    push_b(buf, b" Z");
}

fn draw_mesh_cell(buf: &mut Vec<u8>, idx: usize, poly: &[(f64, f64)], col: u32, val: f64, op: f64, sw: f64) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, val);
    push_b(buf, b"\" d=\"");
    path_d(buf, poly);
    push_b(buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, sw);
    push_b(buf, b"\" stroke-opacity=\"");
    push_f2(buf, op);
    push_b(buf, b"\"/>");
}

fn draw_mesh_cell_plain(buf: &mut Vec<u8>, poly: &[(f64, f64)], col: u32, op: f64, sw: f64) {
    push_b(buf, b"<path d=\"");
    path_d(buf, poly);
    push_b(buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, sw);
    push_b(buf, b"\" stroke-opacity=\"");
    push_f2(buf, op);
    push_b(buf, b"\"/>");
}

fn draw_glow_cell(buf: &mut Vec<u8>, poly: &[(f64, f64)], col: u32, sw: f64) {
    push_b(buf, b"<path d=\"");
    path_d(buf, poly);
    push_b(buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, sw);
    push_b(buf, b"\" stroke-opacity=\"0.14\"/>");
}

fn legend(f: &mut Frame, vmin: f64, vmax: f64, ink: u32, sub: u32, color_at: &dyn Fn(f64) -> u32) {
    let bar_x = f.pl + f.pw + 24;
    let bar_y = f.pt + 12;
    let bar_w = 14;
    let bar_h = (f.ph - 76).max(60);

    push_b(&mut f.buf, b"<defs><linearGradient id=\"spmesh\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    for s in 0..10 {
        let t = s as f64 / 9.0;
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
            width: 820,
            height: 1060,
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
    fn renders_one_hoverable_edge_path_per_site() {
        let (x, y) = synth(40);
        let html = render(&cfg(&x, &y));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=\"").count(), x.len());
    }

    #[test]
    fn renders_multiple_jittered_frames_per_site() {
        let (x, y) = synth(30);
        let html = render(&cfg(&x, &y));
        let total_paths = html.matches("<path ").count();
        assert!(total_paths > x.len() * FRAMES, "expected glow + {FRAMES} frames of edges, got {total_paths} paths for {} sites", x.len());
    }

    #[test]
    fn mesh_cells_have_no_fill_and_carry_a_stroke() {
        let (x, y) = synth(30);
        let html = render(&cfg(&x, &y));
        assert!(html.contains("<g fill=\"none\""));
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
    fn perf_rendering_a_dense_multi_frame_field_stays_fast() {
        let (x, y) = synth(760);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &y));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 4000, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
