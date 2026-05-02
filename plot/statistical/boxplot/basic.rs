use super::common::{
    compute_box, draw_box_vertical, draw_cat_label, draw_outliers_vertical, finish_frame,
    global_range, group_values, make_frame, open_axes, rng_next, sorted_groups,
};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_i};

pub fn render(cfg: &BoxplotConfig) -> String {
    render_with(cfg, cfg.notch, cfg.show_points, false)
}

pub fn render_with(cfg: &BoxplotConfig, notch: bool, show_points: bool, big_outliers: bool) -> String {
    let n = cfg.category_labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }
    let (cats, groups) = group_values(cfg.category_labels, cfg.values);
    let stats: Vec<_> = groups.iter().map(|g| compute_box(g)).collect();
    let (cats, groups, stats) = sorted_groups(cats, groups, stats, cfg.sort_order);
    let n_cats = cats.len();
    let gr = global_range(&stats);

    let legend_w: i32 = if n_cats > 1 { 130 } else { 20 };
    let mut f = make_frame(cfg, n_cats, legend_w);
    let plot_w = f.pw;
    let slot_w = plot_w as f64 / n_cats as f64;
    let box_hw = (slot_w * 0.28) as i32;

    open_axes(&mut f, cfg.title, cfg.gridlines, gr.y_min, gr.y_max);

    let mut rng: u64 = 0x9E3779B97F4A7C15;
    for (ci, ((cat, st), grp)) in cats.iter().zip(stats.iter()).zip(groups.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");
        draw_box_vertical(&mut f, cx, box_hw, st, color, cfg.fill_opacity, cfg.stroke_width, notch, cat, ci as i32, gr.y_min, gr.range_y);
        let outlier_r = if big_outliers { 4.5 } else { 3.0 };
        draw_outliers_vertical(&mut f, cx, st, color, outlier_r, gr.y_min, gr.range_y);
        if show_points {
            let hx = hex6(color);
            for &v in grp {
                if !v.is_finite() {
                    continue;
                }
                let dx = (rng_next(&mut rng) - 0.5) * (box_hw as f64 * 1.1) * cfg.jitter * 2.0;
                let py = f.pt + f.ph - ((v - gr.y_min) / gr.range_y * f.ph as f64) as i32;
                push_b(&mut f.buf, b"<circle cx=\"");
                push_i(&mut f.buf, cx + dx as i32);
                push_b(&mut f.buf, b"\" cy=\"");
                push_i(&mut f.buf, py);
                push_b(&mut f.buf, b"\" r=\"2\" fill=\"#");
                f.buf.extend_from_slice(&hx);
                push_b(&mut f.buf, b"\" fill-opacity=\"0.55\"/>");
            }
        }
        if big_outliers {
            for (oi, &ov) in st.outliers.iter().enumerate() {
                let oy = f.pt + f.ph - ((ov - gr.y_min) / gr.range_y * f.ph as f64) as i32;
                push_b(&mut f.buf, b"<text x=\"");
                push_i(&mut f.buf, cx + 8);
                push_b(&mut f.buf, b"\" y=\"");
                push_i(&mut f.buf, oy + 3);
                push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">#");
                push_i(&mut f.buf, oi as i32 + 1);
                push_b(&mut f.buf, b"</text>");
            }
        }
        draw_cat_label(&mut f, cx, cat);
        push_b(&mut f.buf, b"</g>");
    }

    finish_frame(&mut f, &cats, cfg.palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}
