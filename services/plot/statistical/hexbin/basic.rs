use super::common::{bin_color, data_bounds, draw_hex_cell, finalize, legend_bar, make_frame, prepare};
use super::config::HexbinConfig;

#[crate::chart_demo("x=[1,2,2,3,3,3,4,4,5,1,2,3], y=[1,2,3,2,3,4,3,5,4,2,1,1]")]

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
        let col = bin_color(cfg, &p, bin.count);
        draw_hex_cell(&mut f.buf, i, bin, p.r * 0.98, col, None, false, false);
    }
    legend_bar(&mut f, cfg, &p);
    finalize(f, cfg)
}
