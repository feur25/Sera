use super::common::{data_bounds, draw_main_axes, draw_marginals, finalize, layout, open};
use super::config::JointConfig;
use crate::plot::statistical::hexbin::common::{bin_color_raw, draw_hex_cell, highlight_cutoff, prepare_raw};

#[crate::chart_demo(
    "x=[1,2,2,3,3,3,4,4,4,4,5,5,5,2,3,3,4,1,2,3,4,5,3,3], y=[1,2,3,2,3,4,3,4,5,3,4,5,6,1,2,4,4,2,3,3,5,4,3,5]"
)]
pub fn render(cfg: &JointConfig) -> String {
    let bounds = match data_bounds(cfg.x_values, cfg.y_values) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open(cfg);
    let l = layout(cfg);
    draw_main_axes(&mut f, cfg, &l);

    let hb_bounds = crate::plot::statistical::hexbin::common::Bounds {
        xmin: bounds.xmin,
        xmax: bounds.xmax,
        ymin: bounds.ymin,
        ymax: bounds.ymax,
    };
    if let Some(p) = prepare_raw(cfg.x_values, cfg.y_values, cfg.bins, l.pl, l.pt, l.pw, l.ph, &hb_bounds) {
        let cutoff = highlight_cutoff(&p.bins);
        let show_labels = p.r > 13.0;
        for (i, bin) in p.bins.iter().enumerate() {
            let col = bin_color_raw(cfg.colorscale, p.min_count, p.max_count, bin.count);
            match cfg.panel_variant {
                "outlined" | "outline" | "stroke" | "labeled" => {
                    draw_hex_cell(&mut f.buf, i, bin, p.r * 0.94, col, None, true, show_labels);
                }
                "spaced" | "gapped" | "confetti" | "dotted" => {
                    draw_hex_cell(&mut f.buf, i, bin, p.r * 0.72, col, None, false, false);
                }
                "highlight" | "top" | "hotspot" | "peak" => {
                    let is_top = bin.count >= cutoff.max(1) && bin.count > 0;
                    if is_top {
                        draw_hex_cell(&mut f.buf, i, bin, p.r * 0.98, col, Some(1.0), true, show_labels);
                    } else {
                        draw_hex_cell(&mut f.buf, i, bin, p.r * 0.98, col, Some(0.16), false, false);
                    }
                }
                _ => draw_hex_cell(&mut f.buf, i, bin, p.r * 0.98, col, None, false, false),
            }
        }
    }

    draw_marginals(&mut f, &l, &bounds, cfg.x_values, cfg.y_values, cfg.marginal, cfg.bins, cfg.point_hex);
    finalize(f, cfg)
}
