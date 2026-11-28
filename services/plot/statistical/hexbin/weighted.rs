use super::common::{
    bin_color_value, data_bounds, draw_weighted_cell, finalize, legend_bar_value, make_frame, prepare_weighted,
};
use super::config::HexbinConfig;

#[crate::chart_demo(
    "gridsize=12, x=[1,2,2,3,3,3,4,4,5,5,6,6,7,7,8,8,9,9,1,2,3,4,5,6,7,8,9,2,3,4,5,6,7,8], y=[1,2,3,2,3,4,3,5,4,5,3,4,2,3,1,2,4,5,3,4,2,3,4,2,3,1,2,5,4,3,2,1,4,3], values=[10,20,15,30,45,25,60,55,70,65,80,75,20,25,15,10,90,85,40,35,30,25,50,45,60,15,20,95,88,72,66,30,58,42], variant=\"weighted\""
)]

pub fn render(cfg: &HexbinConfig) -> String {
    let bounds = match data_bounds(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = make_frame(cfg);
    f.open(cfg.title, true);
    f.x_grid(6, bounds.xmin, bounds.xmax, cfg.gridlines);
    f.y_grid(5, bounds.ymin, bounds.ymax, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let p = match prepare_weighted(cfg, &f, &bounds) {
        Some(v) => v,
        None => return String::new(),
    };
    for (i, bin) in p.bins.iter().enumerate() {
        let col = bin_color_value(cfg.colorscale, p.min_avg, p.max_avg, bin.avg);
        draw_weighted_cell(&mut f.buf, i, bin, p.r * 0.98, col);
    }
    legend_bar_value(&mut f, cfg, &p);
    finalize(f, cfg)
}
