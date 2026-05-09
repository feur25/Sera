use super::common::{prepare, open, axes_grid, legend, finalize, write_polyline};
use super::config::ParallelConfig;
use crate::plot::statistical::common::palette_color;

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = open(cfg, &p);
    axes_grid(&mut b, cfg, &p);
    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let col = palette_color(cfg.palette, si);
        write_polyline(&mut b, &p, v, col, 5.0, 0.06, si, &cfg.series_names[si]);
        write_polyline(&mut b, &p, v, col, 2.0, 0.18, si, &cfg.series_names[si]);
    }
    legend(&mut b, cfg, &p);
    finalize(b, cfg)
}


