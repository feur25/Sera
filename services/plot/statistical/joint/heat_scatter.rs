use super::common::{data_bounds, draw_main_axes, finalize, layout, open, right_histogram, top_histogram};
use super::config::JointConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2};
use crate::plot::statistical::heatmap::common::colorscale_color;

#[crate::chart_demo(
    "x=[1,2,2,3,3,3,4,4,4,4,5,5,5,2,3,3,4,1,2,3,4,5,3,3], y=[1,2,3,2,3,4,3,4,5,3,4,5,6,1,2,4,4,2,3,3,5,4,3,5]"
)]
pub fn render(cfg: &JointConfig) -> String {
    let bounds = match data_bounds(cfg.x_values, cfg.y_values) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open(cfg);
    let l = layout(cfg);
    draw_main_axes(&mut f, cfg, &l);

    let grid_n = cfg.bins.max(4);
    let n = cfg.x_values.len().min(cfg.y_values.len());
    let mut counts = vec![0u32; grid_n * grid_n];
    let xr = (bounds.xmax - bounds.xmin).max(1e-12);
    let yr = (bounds.ymax - bounds.ymin).max(1e-12);
    for i in 0..n {
        let gx = (((cfg.x_values[i] - bounds.xmin) / xr) * grid_n as f64) as usize;
        let gy = (((cfg.y_values[i] - bounds.ymin) / yr) * grid_n as f64) as usize;
        let gx = gx.min(grid_n - 1);
        let gy = gy.min(grid_n - 1);
        counts[gy * grid_n + gx] += 1;
    }
    let max_c = counts.iter().copied().max().unwrap_or(1).max(1) as f64;
    let cell_w = l.pw as f64 / grid_n as f64;
    let cell_h = l.ph as f64 / grid_n as f64;
    let scale = if cfg.colorscale.is_empty() { "magma" } else { cfg.colorscale };
    for gy in 0..grid_n {
        for gx in 0..grid_n {
            let c = counts[gy * grid_n + gx];
            if c == 0 {
                continue;
            }
            let t = c as f64 / max_c;
            let col = colorscale_color(scale, t);
            let x0 = l.pl as f64 + gx as f64 * cell_w;
            let y0 = l.pt as f64 + l.ph as f64 - (gy as f64 + 1.0) * cell_h;
            push_b(&mut f.buf, b"<rect x=\"");
            push_f2(&mut f.buf, x0);
            push_b(&mut f.buf, b"\" y=\"");
            push_f2(&mut f.buf, y0);
            push_b(&mut f.buf, b"\" width=\"");
            push_f2(&mut f.buf, cell_w + 0.6);
            push_b(&mut f.buf, b"\" height=\"");
            push_f2(&mut f.buf, cell_h + 0.6);
            push_b(&mut f.buf, b"\" fill=\"#");
            f.buf.extend_from_slice(&hex6(col));
            push_b(&mut f.buf, b"\"/>");
        }
    }

    top_histogram(&mut f, &l, &bounds, cfg.x_values, cfg.bins, cfg.point_hex);
    right_histogram(&mut f, &l, &bounds, cfg.y_values, cfg.bins, cfg.point_hex);
    finalize(f, cfg)
}
