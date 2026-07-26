use super::common::{bin_color_log, data_bounds, draw_hex_cell, finalize, legend_bar_log, make_frame, prepare};
use super::config::HexbinConfig;

#[crate::chart_demo(
    "gridsize=18, x=[1,1,1,1,1,2,2,2,3,3,4,5,6,7,7,7,7,7,8,8,8,8,8,8,8,8,9,9,9,9,9,9,9,9,9,9,10,10,10,10,10,10,10,10,10,10,10,10,2,3,4,5,6,7,8,9,10,11,12,13], y=[1,2,3,4,5,1,2,3,4,5,4,3,2,1,2,3,4,5,1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,6,7,8,9,10,11,12,8,8,8,8,8,8,8,8,8,8,8,8], variant=\"log_counts\""
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
    let p = match prepare(cfg, &f, &bounds) {
        Some(v) => v,
        None => return String::new(),
    };
    for (i, bin) in p.bins.iter().enumerate() {
        let col = bin_color_log(cfg.colorscale, p.min_count, p.max_count, bin.count);
        draw_hex_cell(&mut f.buf, i, bin, p.r * 0.98, col, None, false, false);
    }
    legend_bar_log(&mut f, cfg, &p);
    finalize(f, cfg)
}
