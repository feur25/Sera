use super::common::{data_bounds, draw_main_axes, draw_marginals, finalize, layout, open, px, py};
use super::config::JointConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
use crate::plot::statistical::scatter::regression::fit_linear;

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
    draw_main_axes(&mut f, cfg, &l);

    let n = cfg.x_values.len().min(cfg.y_values.len());
    let point_hx = hex6(cfg.point_hex);
    for i in 0..n {
        let cx = px(&l, &bounds, cfg.x_values[i]);
        let cy = py(&l, &bounds, cfg.y_values[i]);
        push_b(&mut f.buf, b"<circle data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" cx=\"");
        push_f2(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_f2(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"3.2\" fill=\"#");
        f.buf.extend_from_slice(&point_hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.8\" stroke=\"#ffffff\" stroke-width=\"0.6\"/>");
    }

    if cfg.show_regression {
        if let Some((slope, intercept, _r2)) = fit_linear(&cfg.x_values[..n], &cfg.y_values[..n]) {
            let steps = 40usize;
            let dx = (bounds.xmax - bounds.xmin) / steps as f64;
            let line_hx = hex6(cfg.palette.get(1).copied().unwrap_or(0xef4444));
            push_b(&mut f.buf, b"<path fill=\"none\" stroke=\"#");
            f.buf.extend_from_slice(&line_hx);
            push_b(&mut f.buf, b"\" stroke-width=\"2.2\" d=\"");
            for k in 0..=steps {
                let xv = bounds.xmin + dx * k as f64;
                let yv = intercept + slope * xv;
                let sx = px(&l, &bounds, xv);
                let sy = py(&l, &bounds, yv);
                if k == 0 {
                    push_b(&mut f.buf, b"M");
                } else {
                    push_b(&mut f.buf, b"L");
                }
                push_f2(&mut f.buf, sx);
                push_b(&mut f.buf, b",");
                push_f2(&mut f.buf, sy);
            }
            push_b(&mut f.buf, b"\"/>");
        }
    }

    draw_marginals(&mut f, &l, &bounds, cfg.x_values, cfg.y_values, cfg.marginal, cfg.bins, cfg.point_hex);
    finalize(f, cfg)
}
