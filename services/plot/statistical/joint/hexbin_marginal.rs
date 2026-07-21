use super::common::{data_bounds, draw_main_axes, finalize, layout, open, right_histogram, top_histogram};
use super::config::JointConfig;
use crate::plot::statistical::common::{hex6, push_b, push_i};
use crate::plot::statistical::hexbin::common::{bin_color_raw, hex_path, prepare_raw};

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
        for (i, bin) in p.bins.iter().enumerate() {
            let col = bin_color_raw(cfg.colorscale, p.min_count, p.max_count, bin.count);
            push_b(&mut f.buf, b"<path data-idx=\"");
            push_i(&mut f.buf, i as i32);
            push_b(&mut f.buf, b"\" d=\"");
            hex_path(&mut f.buf, bin.cx, bin.cy, p.r * 0.98);
            push_b(&mut f.buf, b"\" fill=\"#");
            f.buf.extend_from_slice(&hex6(col));
            push_b(&mut f.buf, b"\"/>");
        }
    }

    top_histogram(&mut f, &l, &bounds, cfg.x_values, cfg.bins, cfg.point_hex);
    right_histogram(&mut f, &l, &bounds, cfg.y_values, cfg.bins, cfg.point_hex);
    finalize(f, cfg)
}
