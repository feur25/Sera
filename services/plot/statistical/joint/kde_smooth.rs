use super::common::{data_bounds, draw_main_axes, finalize, kde_heat_grid, layout, open, right_histogram, top_histogram};
use super::config::JointConfig;

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

    kde_heat_grid(&mut f, &l, &bounds, cfg.x_values, cfg.y_values, cfg.colorscale, 44);
    draw_main_axes(&mut f, cfg, &l);

    top_histogram(&mut f, &l, &bounds, cfg.x_values, cfg.bins, cfg.point_hex);
    right_histogram(&mut f, &l, &bounds, cfg.y_values, cfg.bins, cfg.point_hex);
    finalize(f, cfg)
}
