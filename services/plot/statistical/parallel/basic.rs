use super::common::{axes_grid, finalize, legend, open, prepare, write_dots, write_polyline};
use super::config::ParallelConfig;
use crate::plot::statistical::common::palette_color;

#[crate::chart_demo("axes=[\"Speed\",\"Power\",\"Range\",\"Cost\"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=[\"A\",\"B\",\"C\"]")]

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
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
