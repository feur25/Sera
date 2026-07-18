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
    let hi = if cfg.highlight_index >= 0 {
        cfg.highlight_index as usize
    } else {
        0
    };
    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let is_hi = si == hi;
        let col = if is_hi {
            palette_color(cfg.palette, si)
        } else {
            0xCBD5E1
        };
        let sw_back = if is_hi { 6.0 } else { 1.0 };
        let op_back = if is_hi { 0.18 } else { 0.0 };
        let sw_main = if is_hi { 2.4 } else { 1.0 };
        let op_main = if is_hi { 0.95 } else { 0.35 };
        if op_back > 0.0 {
            write_polyline(
                &mut b,
                &p,
                v,
                col,
                sw_back,
                op_back,
                si,
                &cfg.series_names[si],
            );
        }
        write_polyline(
            &mut b,
            &p,
            v,
            col,
            sw_main,
            op_main,
            si,
            &cfg.series_names[si],
        );
        if is_hi {
            write_dots(&mut b, &p, v, col, 4.0, 1.0, si);
        }
    }
    legend(&mut b, cfg, &p);
    finalize(b, cfg)
}
