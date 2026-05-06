use super::common::{prepare, open, axes_grid, legend, finalize, write_polyline, write_dots};
use super::config::ParallelConfig;
use crate::plot::statistical::common::palette_color;

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = open(cfg, &p);
    axes_grid(&mut b, cfg, &p);
    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let col = palette_color(cfg.palette, si);
        write_polyline(&mut b, &p, v, col, 4.0, 0.12, si, &cfg.series_names[si]);
        write_polyline(&mut b, &p, v, col, 1.8, 0.7, si, &cfg.series_names[si]);
        write_dots(&mut b, &p, v, col, 3.0, 0.85, si);
    }
    legend(&mut b, cfg, &p);
    finalize(b, cfg)
}
