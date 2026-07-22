use super::common::{data_bounds, draw_main_axes, draw_marginals, finalize, kde_heat_grid, layout, open};
use super::config::JointConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};

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

    let grid_n = cfg.bins.clamp(12, 60);
    kde_heat_grid(&mut f, &l, &bounds, cfg.x_values, cfg.y_values, cfg.colorscale, grid_n);
    draw_main_axes(&mut f, cfg, &l);

    if cfg.show_points {
        let n = cfg.x_values.len().min(cfg.y_values.len());
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
    }

    draw_marginals(&mut f, &l, &bounds, cfg.x_values, cfg.y_values, cfg.marginal, cfg.bins, cfg.point_hex);
    finalize(f, cfg)
}
