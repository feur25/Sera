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
    "title=\"Turbidity - Density Voronoi Mesh\", x=[36.18,16.67,33.75,27.28,44.03,27.41,16.18,33.26,27.39,24.51,34.76,26.47,29.95,41.33,41.62,29.52,37.34,43.11,23.37,41.8,30.21,57.03,30.76,29.11,29.92,44.4,32.29,11.28,44.28,17.52,16.18,25.48,13.66,18.8,41.37,24.28,21.34,0.5,20.57,26.32,31.35,30.61,33.62,37.63,39.68,21.75,14.87,19.31,7.93,15.02,6.49,27.26,38.34,29.59,39.27,51.36,41.94,37.49,23.77,30.2,41.69,33.14,42.62,23.1,37.16,39.79,40.19,38.46,34.48,26.13,12.71,30.84,25.5,33.98,47.49,33.09,22.89,32.32,59.8,46.5,14.39,29.91,35.73,31.94,25.88,31.77,29.48,35.35,14.27,45.75,2.33,40.49,46.33,22.66,59.08,13.64,26.28,19.92,28.03,16.14,18.57,28.29,26.93,25.82,33.88,27.04,30.08,48.62,29.24,39.76,26.78,22.3,0.5,21.75,9.39,3.88,7.86,13.99,22.82,17.0,22.07,27.07,30.35,21.01,34.4,25.17,7.83,23.05,17.56,29.82,34.17,26.65,18.54,20.6,26.71,33.93,33.71,25.35,44.56,25.2,27.15,28.36,42.52,24.37,36.18,42.76,35.49,30.13,39.03,45.35,35.87,12.91,61.22,36.7,28.21,59.01,33.74,16.83,29.09,38.41,21.47,25.64,26.08,30.98,14.29,32.04,21.53,17.55,52.97,0.5,14.91,8.23,11.65,23.96,29.46,11.3,25.39,47.89,16.63,16.05,26.16,29.22,31.01,12.52,29.92,30.57,18.88,30.0,31.43,21.45,42.04,44.81,41.93,20.07,0.5,14.14,29.86,27.72,37.84,22.4,19.89,53.03,4.62,34.11,33.65,41.25,35.77,37.1,30.81,22.44,43.47,33.79,39.98,16.51,38.32,37.03,49.69,39.96,32.64,37.59,65.19,39.83,28.01,37.04,33.57,30.21,30.02,19.7,38.21,37.38,16.67,1.35,27.43,20.49,29.65,17.87,26.3,17.64,45.21,46.81,22.56,29.54,27.74,2.31,28.43,15.71,28.12,37.81,28.59,26.06,34.49,18.76,15.59,15.99,16.38,29.4,29.89,41.24,48.24,38.58,27.81,39.19,47.3,41.08,8.81,25.61,18.33,58.36,17.66,35.18,51.55,12.15,19.8,33.67,53.39,11.44,42.91,11.16,33.37,19.24,18.25,48.85,40.61,30.77,16.56,52.92,23.72,29.9,17.85,29.57,28.43,3.22,45.89,29.14,46.31,43.59,19.24,38.29,24.83,21.45,16.88,19.19,19.86,28.61,24.22,29.32,28.12,33.15,19.84,18.1,21.0,36.06,14.3,12.59,15.5,29.12,21.42,20.46,13.78,36.46,23.76,36.88,37.32,25.64,32.76,28.28,30.5,25.38,48.0,23.68,27.94,26.23,22.13,21.22,52.24,27.3,26.04,22.88,36.91,36.61,6.51,38.66,19.4,45.2,34.45,22.61,27.35,40.06,46.66,36.33,50.84,30.66,35.68,43.67,6.8,15.98,30.28,36.92,33.1,40.19,27.37,26.79,49.34,48.2,42.39,25.15,22.12,41.26,14.07,24.21,43.68,46.78,37.11,42.57,31.77,21.16,36.55,33.42,35.85,60.53,9.54,33.25,15.85,8.03,32.75,27.56,40.1,15.09,52.64,14.99,26.07,34.59,17.73,50.97,25.45,29.17,40.74,34.96,38.72,45.16,23.37,6.64,12.28,31.84,24.22,11.06,19.42,26.75,10.38,34.96,44.83,17.93,6.25,21.49,0.5,25.2,23.2,41.25,24.61,20.49,23.56,22.26,17.11,30.35,29.87,39.16,19.09,31.3,32.46,17.85,26.17,40.21,40.06,32.51,47.83,25.47,27.38,43.22,51.11,43.83,54.79,39.76,36.12,38.6,10.67,40.26,41.74,16.73,12.43,28.16,35.58,28.48,39.19,26.49,33.26,48.39,30.31,18.52,32.07,17.05,38.21,18.73,38.24,12.65,28.0,12.86,17.54,5.31,31.79,24.54,30.29,23.19,12.25,23.53,25.71,11.96,21.57,18.01,23.17,4.01,25.62,27.86,21.46,42.92,37.38,25.45,19.08,23.85,13.33,48.9,37.82,32.6,24.34,31.57,41.33,40.67,31.2,22.42,49.24,48.05,25.24,37.49,35.6,32.32,39.57,23.08,36.04,27.04,33.26,40.1,32.85,39.07,31.04,27.01,30.16,32.44,33.47,11.75,11.16,30.83,4.57,18.63,39.59,46.83,8.25,13.33,31.16,35.3,28.61,45.19,35.93,17.27,20.2,19.42,43.65,35.57,27.87,26.18,20.3,26.15,17.9,38.96,17.09,19.3,11.03,44.14,27.09,29.46,13.1,19.94,3.71,23.34,17.57,37.11,24.09,16.15,20.49,21.13,17.1,19.56,24.98,14.54,20.3,20.47,19.57,12.05,28.39,31.73,22.59,2.58,22.73,15.05,13.99,38.91,40.82,29.17,28.3,36.93,20.95,31.35,28.71,56.27,41.47,27.69,10.35,44.82,42.84,45.92,37.26,29.29,58.0,24.92,15.67,18.87,31.92,28.19,43.35,13.08,46.28,33.56,41.18,29.77,23.51,11.55,14.03,17.73,21.08,19.27,23.81,33.89,0.5,17.85,15.55,10.09,16.78,31.17,12.37,21.04,6.02,29.64,52.12,20.99,65.21,18.49,50.11,32.86,42.55,38.22,22.24,19.76,33.29,26.55,27.14,29.87,31.17,29.65,14.69,57.24,12.94,27.79,57.45,29.84,33.61,27.47,25.36,32.73,30.33,30.48,28.47,39.0,33.27,26.78,46.97,30.15,42.61,28.82,39.16,27.94,39.29,45.93,34.86,36.12,40.38,24.59,32.71,14.8,23.42,31.88,16.22,13.9,32.76,20.76,8.13,27.07,13.0,13.21,29.66,20.98,7.34,27.79,20.28,23.66,31.68,35.65,20.18,20.96,3.63,8.38,39.04,17.28,9.7,29.45,33.55,30.79,29.15,35.18,26.34,30.07,23.12,42.29,28.71,14.08,24.45,25.91,35.45,43.46,36.16,39.87,24.1,16.75,49.82,24.21,27.66,46.3,18.33,32.77,37.39,36.23,33.58,34.59,34.27,12.91,19.39,26.36,13.11,28.53,24.85,46.26,30.19,33.73,23.28,23.77,22.5,18.05,35.36,37.37,14.94,27.46,25.64,42.37,31.44,38.17,39.23,25.89,40.92,40.37,30.69,24.47,28.26,24.68,31.09,28.19,30.77,39.2,25.11,26.96,2.61,27.71,14.98,21.79,0.5,35.31,24.26,19.85,43.21,79.45,21.92,22.78,46.84,68.47,0.5,62.58,1.63,0.5,8.75,25.38,49.76,99.5,42.73,0.5,21.03,48.41,0.5,35.07,32.23,19.8,58.19,16.41,34.51,53.32,8.82,11.67,13.81,5.42,0.5,46.86,8.69,27.28,48.83,21.71,0.5,27.03,68.82,4.96,57.41,10.79,46.12,2.75,34.97,35.39,45.56,27.35,0.5,13.94,71.42,23.39,5.19,38.29,3.5,21.87,52.04,25.7,2.88,7.14,58.4,0.5,6.26,9.61,0.5,40.06], y=[17.58,15.41,0.5,5.29,0.5,7.92,9.87,1.44,10.87,8.3,2.23,5.31,2.73,9.29,0.78,0.5,5.3,7.17,5.81,10.35,9.74,3.6,6.77,0.5,3.86,0.8,4.65,3.49,9.02,3.88,9.35,5.22,4.66,8.34,0.86,14.5,17.18,10.46,4.26,17.05,6.26,11.58,12.68,5.96,13.18,23.13,17.32,10.55,10.77,13.41,16.14,16.73,1.6,16.61,10.33,5.78,13.06,17.34,5.71,13.59,14.78,16.1,19.2,16.55,10.2,16.35,1.68,16.65,13.23,18.03,11.03,0.5,18.28,0.5,2.77,18.89,11.31,9.69,9.75,4.17,18.18,23.42,15.44,29.14,15.11,10.3,6.46,18.0,22.47,10.69,22.96,3.74,17.23,12.94,14.45,21.08,28.74,17.75,21.28,24.21,29.75,18.66,22.68,20.33,31.01,28.66,23.9,11.69,27.84,1.4,16.56,13.38,18.85,28.14,20.01,19.21,26.34,24.38,15.67,20.88,26.17,23.53,16.32,18.59,18.24,34.52,18.4,24.76,19.6,28.23,21.25,27.25,21.15,21.78,18.18,24.51,28.01,33.95,38.41,37.2,22.08,24.0,26.32,28.9,22.52,32.44,25.71,11.93,17.32,19.95,32.87,25.88,32.4,24.48,20.69,21.75,25.84,12.28,37.47,9.64,26.98,19.49,19.42,38.11,31.77,25.54,40.5,32.55,7.04,27.07,33.99,24.63,38.99,22.11,27.59,20.61,16.25,32.26,29.54,30.73,25.4,36.82,36.6,27.66,12.28,18.5,25.73,25.78,35.0,20.5,18.04,35.04,13.44,15.67,23.2,34.23,23.16,33.78,44.46,22.91,18.22,34.45,24.49,27.95,35.03,41.98,22.82,34.16,33.25,22.77,30.24,37.52,15.99,24.84,23.66,29.42,35.47,29.35,29.46,15.91,35.03,19.66,40.2,16.27,26.88,19.21,21.39,31.72,30.87,38.18,28.1,33.21,40.06,32.1,38.55,31.42,33.21,32.55,36.4,40.6,49.91,37.26,29.62,32.84,26.5,37.05,43.34,44.01,30.08,39.33,43.31,37.73,27.18,35.11,41.34,22.67,29.41,50.87,15.78,42.47,34.23,26.92,41.52,64.96,37.46,38.31,48.43,38.37,47.02,34.37,23.81,39.88,23.78,43.5,32.37,34.85,15.75,41.28,38.02,35.08,45.06,32.59,37.16,39.33,28.77,46.07,34.28,56.87,38.17,26.6,20.8,46.81,30.36,27.84,21.26,49.46,34.72,28.04,41.14,35.69,48.14,44.9,51.6,46.1,38.82,55.62,50.05,41.35,53.73,40.63,46.31,44.01,41.28,48.07,53.88,38.75,45.67,44.71,38.52,45.34,43.49,43.06,46.66,58.06,56.07,49.69,30.91,67.18,43.58,39.89,40.86,42.76,39.69,38.04,44.42,46.62,44.01,50.24,59.12,39.88,49.06,44.82,44.17,58.76,51.58,40.21,47.58,55.73,45.13,34.24,41.2,55.8,49.24,60.12,46.94,39.91,51.38,60.2,25.29,35.25,39.63,46.87,47.15,49.37,51.89,47.06,37.43,48.58,48.7,53.11,46.89,41.72,51.64,42.09,31.44,61.12,41.7,58.35,45.44,60.44,42.63,52.68,58.87,63.01,52.03,57.81,32.67,52.93,67.07,60.23,50.12,57.98,49.64,66.24,57.93,54.03,63.07,60.7,47.62,54.11,56.75,55.37,47.92,42.13,56.38,50.42,55.37,47.48,53.97,55.57,55.3,46.68,49.8,54.9,54.97,48.28,66.33,54.53,56.02,59.07,53.83,42.68,42.03,54.13,58.53,72.85,55.33,36.9,54.22,55.54,60.48,55.89,48.21,54.33,56.66,51.55,55.92,67.16,51.68,49.71,54.38,71.23,61.83,54.82,63.05,44.67,54.69,57.97,70.03,58.76,62.92,45.15,60.19,64.48,56.59,59.16,60.46,43.35,51.04,71.57,58.35,56.26,45.08,61.13,58.63,60.2,69.12,67.5,61.74,59.57,59.82,61.86,53.12,60.71,53.85,45.19,51.15,57.67,60.28,74.85,43.96,47.42,59.78,65.23,55.6,57.55,63.69,68.21,64.16,69.21,52.8,62.4,68.93,71.57,57.2,62.54,63.03,58.11,55.27,60.95,72.08,54.94,67.1,64.96,66.74,59.24,52.2,66.21,67.7,47.04,68.02,68.94,71.13,57.28,68.64,84.27,67.63,62.88,63.19,65.55,65.05,82.45,58.85,56.31,57.26,65.2,79.28,56.56,76.89,53.71,62.95,63.96,48.93,69.12,76.43,77.34,76.89,69.41,79.31,49.79,82.42,58.64,65.61,68.57,75.69,75.6,56.33,61.39,64.3,61.71,66.42,75.94,72.05,74.43,73.13,63.32,82.66,66.94,63.35,77.59,67.97,57.71,69.2,65.74,57.88,65.99,75.22,70.32,64.82,71.11,83.38,67.93,78.42,78.13,80.09,60.11,71.86,74.59,60.62,80.86,79.2,71.62,72.2,85.28,68.82,79.26,72.6,81.3,70.83,79.53,55.56,67.12,60.87,85.84,69.42,73.38,79.6,70.1,79.39,70.96,60.35,60.32,64.15,75.0,70.61,76.46,72.07,72.36,71.44,66.21,68.81,78.31,82.68,90.38,66.4,62.43,83.7,75.27,68.9,74.91,70.81,77.86,90.96,83.07,67.69,79.54,89.02,92.17,67.47,66.37,73.91,83.1,83.31,62.34,89.05,67.95,64.28,66.06,78.23,71.73,59.57,76.45,88.36,72.46,78.17,95.22,76.27,79.72,92.29,82.28,74.46,77.61,64.6,83.74,80.35,79.09,83.93,77.86,81.95,85.56,85.37,84.12,80.42,91.28,79.32,81.55,88.34,86.72,84.15,91.94,68.71,99.5,87.93,79.2,87.1,90.7,79.61,90.6,86.24,84.03,87.92,87.14,72.19,85.05,96.08,79.92,71.8,91.85,97.41,94.5,77.42,93.97,92.79,81.58,85.62,94.35,94.22,72.29,78.28,98.1,83.94,87.91,99.5,91.38,88.83,97.87,79.6,84.48,97.2,88.84,86.26,86.9,80.38,89.31,91.13,88.19,80.47,80.78,99.5,99.5,94.01,94.71,94.54,86.72,99.5,87.32,97.24,99.5,94.88,94.78,76.04,87.67,91.63,91.69,87.85,89.39,93.45,99.5,82.94,99.5,92.88,91.87,99.5,99.5,98.78,97.83,98.34,90.92,99.5,98.62,98.75,93.63,91.53,96.66,96.07,75.65,0.5,56.45,36.06,52.55,60.35,60.58,81.53,85.02,99.5,0.5,90.67,71.53,81.18,0.5,3.43,79.59,87.25,81.47,23.36,12.76,98.4,28.63,41.33,96.07,16.16,61.81,55.04,0.5,52.06,41.14,31.09,29.71,78.02,21.93,99.5,35.38,68.71,76.18,43.89,4.07,38.72,29.93,99.5,73.94,31.94,53.22,84.31,0.5,71.2,0.5,39.72,24.28,74.63,94.05,17.73,73.24,84.69,44.73,44.57,0.5,88.87,53.69,11.79,60.94,47.09,97.85,22.01,58.72,28.78,66.24,68.84,27.09,39.84,85.17], width=760, height=1200, variant=\"neural\""
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
        b"<defs>\
        <filter id=\"nglowSoft\" x=\"-120%\" y=\"-120%\" width=\"340%\" height=\"340%\" color-interpolation-filters=\"sRGB\"><feGaussianBlur stdDeviation=\"4.4\"/></filter>\
        <filter id=\"nglow\" x=\"-80%\" y=\"-80%\" width=\"260%\" height=\"260%\" color-interpolation-filters=\"sRGB\"><feGaussianBlur stdDeviation=\"1.9\" result=\"b\"/><feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter>\
        </defs>",
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

    push_b(&mut f.buf, b"<g fill=\"none\" filter=\"url(#nglowSoft)\">");
    for fk in 0..FRAMES {
        for &i in order.iter().rev() {
            if rank_t[i] < 0.68 {
                break;
            }
            if frame_cells[fk][i].len() < 3 {
                continue;
            }
            let col = color_at(rank_t[i]);
            draw_glow_cell(&mut f.buf, &frame_cells[fk][i], col, 2.6, 0.16);
        }
    }
    push_b(&mut f.buf, b"</g>");

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
            draw_glow_cell(&mut f.buf, &frame_cells[fk][i], col, sw, 0.16);
        }
    }
    push_b(&mut f.buf, b"</g>");

    push_b(&mut f.buf, b"<g fill=\"none\">");
    for &i in order.iter().rev() {
        let col = color_at(rank_t[i]);
        let op = 0.32 + rank_t[i] * 0.5;
        let sw = 0.35 + rank_t[i] * 0.45;
        if frame_cells[0][i].len() >= 3 {
            draw_mesh_cell(&mut f.buf, i, &frame_cells[0][i], col, raw[i], op, sw);
        }
        if rank_t[i] < 0.35 {
            continue;
        }
        for fk in 1..FRAMES {
            if frame_cells[fk][i].len() < 3 {
                continue;
            }
            draw_mesh_cell_plain(&mut f.buf, &frame_cells[fk][i], col, op * 0.55, sw * 0.9);
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

fn draw_glow_cell(buf: &mut Vec<u8>, poly: &[(f64, f64)], col: u32, sw: f64, op: f64) {
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

fn legend(f: &mut Frame, vmin: f64, vmax: f64, ink: u32, sub: u32, color_at: &dyn Fn(f64) -> u32) {
    let bar_x = f.pl + f.pw + 24;
    let cap_y = f.pt + 6;
    let bar_y = cap_y + 54;
    let bar_w = 14;
    let bar_h = (f.ph - 130).max(60);

    for (li, line) in ["Voronoi cells,", "sized by inverse", "local point density"].iter().enumerate() {
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, bar_x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, cap_y + li as i32 * 12);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" font-style=\"italic\" fill=\"#");
        f.buf.extend_from_slice(&hex6(sub));
        push_b(&mut f.buf, b"\">");
        push_b(&mut f.buf, line.as_bytes());
        push_b(&mut f.buf, b"</text>");
    }

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
            width: 760,
            height: 1200,
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
