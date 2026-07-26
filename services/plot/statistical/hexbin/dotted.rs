use super::common::{bin_color, data_bounds, draw_hex_cell_dashed, finalize, legend_bar, make_frame, prepare};
use super::config::HexbinConfig;

#[crate::chart_demo(
    "gridsize=15, colorscale=\"magma\", x=[11.03,12.37,13.2,14.37,13.24,14.2,14.39,14.06,14.83,13.86,14.1,14.12,13.75,14.75,14.38,13.63,14.3,13.83,14.19,13.64,14.06,12.93,13.71,12.85,13.5,13.05,13.39,13.3,13.87,14.02,13.73,13.58,13.68,13.76,13.51,13.48,13.28,13.05,13.07,14.22,13.56,13.41,13.88,13.24,12.37,13.34,12.21,12.29,13.86,13.05], y=[2.16,1.63,1.78,1.95,2.59,1.76,1.87,2.15,1.64,1.35,2.16,1.48,1.73,1.68,1.87,1.81,1.92,1.57,2.14,3.1,1.63,3.8,1.79,3.27,1.77,2.05,1.68,1.72,1.9,3.99,4.36,2.58,1.83,1.53,1.8,1.61,1.64,1.65,1.5,3.99,1.71,3.43,1.87,3.98,1.13,1.94,1.19,1.61,1.51,2.05], variant=\"dotted\""
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
        let col = bin_color(cfg, &p, bin.count);
        draw_hex_cell_dashed(&mut f.buf, i, bin, p.r * 0.98, col, "2,2");
    }
    legend_bar(&mut f, cfg, &p);
    finalize(f, cfg)
}
