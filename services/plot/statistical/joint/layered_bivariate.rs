use super::common::{data_bounds, draw_main_axes, finalize, layout, open, right_kde, top_kde};
use super::config::JointConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
use crate::plot::statistical::heatmap::common::colorscale_color;
use crate::plot::statistical::kde::common::scott_bw;

#[crate::chart_demo(
    "x=[1.2,2.4,2.1,3.6,3.1,3.9,4.2,4.6,4.4,4.9,5.5,5.1,5.8,2.2,3.3,3.7,4.1,1.8,2.6,3.4,4.3,5.2,3.2,3.8], \
y=[1.1,2.3,3.2,2.4,3.6,4.1,3.3,4.7,5.2,3.9,4.4,5.6,6.1,1.4,2.5,4.2,4.6,2.1,3.1,3.3,5.1,4.5,3.4,5.3]"
)]
pub fn render(cfg: &JointConfig) -> String {
    let bounds = match data_bounds(cfg.x_values, cfg.y_values) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open(cfg);
    let l = layout(cfg);

    let n = cfg.x_values.len().min(cfg.y_values.len());
    let bw_x = scott_bw(cfg.x_values).max(1e-9);
    let bw_y = scott_bw(cfg.y_values).max(1e-9);
    let grid_n = 34usize;
    let mut dens = vec![0f64; grid_n * grid_n];
    let mut max_d = 1e-12f64;
    for gy in 0..grid_n {
        let gyv = bounds.ymin + (bounds.ymax - bounds.ymin) * gy as f64 / (grid_n - 1) as f64;
        for gx in 0..grid_n {
            let gxv = bounds.xmin + (bounds.xmax - bounds.xmin) * gx as f64 / (grid_n - 1) as f64;
            let mut acc = 0f64;
            for i in 0..n {
                let ux = (gxv - cfg.x_values[i]) / bw_x;
                let uy = (gyv - cfg.y_values[i]) / bw_y;
                let e = ux * ux + uy * uy;
                if e < 24.0 {
                    acc += (-0.5 * e).exp();
                }
            }
            dens[gy * grid_n + gx] = acc;
            if acc > max_d {
                max_d = acc;
            }
        }
    }

    let cell_w = l.pw as f64 / (grid_n - 1) as f64;
    let cell_h = l.ph as f64 / (grid_n - 1) as f64;
    let scale = if cfg.colorscale.is_empty() { "cividis" } else { cfg.colorscale };
    for gy in 0..grid_n {
        for gx in 0..grid_n {
            let t = dens[gy * grid_n + gx] / max_d;
            if t < 0.04 {
                continue;
            }
            let col = colorscale_color(scale, t);
            let x0 = l.pl as f64 + gx as f64 * cell_w - cell_w / 2.0;
            let y0 = l.pt as f64 + l.ph as f64 - (gy as f64 * cell_h + cell_h / 2.0);
            push_b(&mut f.buf, b"<rect x=\"");
            push_f2(&mut f.buf, x0);
            push_b(&mut f.buf, b"\" y=\"");
            push_f2(&mut f.buf, y0);
            push_b(&mut f.buf, b"\" width=\"");
            push_f2(&mut f.buf, cell_w + 0.8);
            push_b(&mut f.buf, b"\" height=\"");
            push_f2(&mut f.buf, cell_h + 0.8);
            push_b(&mut f.buf, b"\" fill=\"#");
            f.buf.extend_from_slice(&hex6(col));
            push_b(&mut f.buf, b"\" fill-opacity=\"");
            push_f2(&mut f.buf, (0.25 + 0.55 * t).min(0.85));
            push_b(&mut f.buf, b"\"/>");
        }
    }

    draw_main_axes(&mut f, cfg, &l);

    let point_hx = hex6(cfg.point_hex);
    for i in 0..n {
        let px = super::common::px(&l, &bounds, cfg.x_values[i]);
        let py = super::common::py(&l, &bounds, cfg.y_values[i]);
        push_b(&mut f.buf, b"<circle data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" cx=\"");
        push_f2(&mut f.buf, px);
        push_b(&mut f.buf, b"\" cy=\"");
        push_f2(&mut f.buf, py);
        push_b(&mut f.buf, b"\" r=\"3\" fill=\"#");
        f.buf.extend_from_slice(&point_hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.85\" stroke=\"#ffffff\" stroke-width=\"0.6\"/>");
    }

    top_kde(&mut f, &l, &bounds, cfg.x_values, cfg.point_hex);
    right_kde(&mut f, &l, &bounds, cfg.y_values, cfg.point_hex);
    finalize(f, cfg)
}
