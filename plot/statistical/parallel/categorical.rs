use super::common::{prepare, open, axes_grid, finalize, write_polyline, write_dots};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{palette_color, svg_legend_item};

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = open(cfg, &p);
    axes_grid(&mut b, cfg, &p);
    let n_cat = if cfg.categories.len() == p.n_series {
        let mut m = 0i32;
        for &c in cfg.categories.iter() { if c > m { m = c; } }
        (m + 1) as usize
    } else { 0 };
    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let col = if n_cat > 0 {
            let cidx = cfg.categories[si].max(0) as usize;
            palette_color(cfg.palette, cidx)
        } else { palette_color(cfg.palette, si) };
        write_polyline(&mut b, &p, v, col, 4.0, 0.10, si, &cfg.series_names[si]);
        write_polyline(&mut b, &p, v, col, 1.6, 0.55, si, &cfg.series_names[si]);
        write_dots(&mut b, &p, v, col, 2.6, 0.85, si);
    }
    if n_cat > 0 {
        for c in 0..n_cat {
            let col = palette_color(cfg.palette, c);
            let nm = format!("group {}", c);
            svg_legend_item(&mut b, c as i32, &nm, col, cfg.width - p.pad_r + 14, p.pad_t + 4 + (c as i32) * 20, 20);
        }
    } else {
        for (li, name) in cfg.series_names.iter().enumerate() {
            let col = palette_color(cfg.palette, li);
            svg_legend_item(&mut b, li as i32, name, col, cfg.width - p.pad_r + 14, p.pad_t + 4 + (li as i32) * 20, 20);
        }
    }
    finalize(b, cfg)
}


