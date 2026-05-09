use super::common::{
    compute_box, draw_cat_label, finish_frame, global_range, group_values, make_frame,
    open_axes, rng_next, sorted_groups,
};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

pub fn render(cfg: &BoxplotConfig) -> String {
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
    open_axes(&mut f, cfg.title, cfg.gridlines, gr.y_min, gr.y_max);

    let plot_w = f.pw;
    let slot_w = plot_w as f64 / n_cats as f64;
    let half = (slot_w * 0.34) as i32;

    let mut rng: u64 = 0xDEADBEEFCAFEBABE;
    for (ci, ((cat, grp), st)) in cats.iter().zip(groups.iter()).zip(stats.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");

        let y_med = f.pt + f.ph - ((st.median - gr.y_min) / gr.range_y * f.ph as f64) as i32;
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - half);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + half);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-opacity=\"0.65\" stroke-width=\"1.6\" stroke-dasharray=\"4,3\"/>");

        for &v in grp {
            if !v.is_finite() {
                continue;
            }
            let dx = (rng_next(&mut rng) - 0.5) * (half as f64 * 1.6) * cfg.jitter * 1.5;
            let py = f.pt + f.ph - ((v - gr.y_min) / gr.range_y * f.ph as f64) as i32;
            push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, ci as i32);
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, cat);
            push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, v);
            push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx + dx as i32);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, py);
            push_b(&mut f.buf, b"\" r=\"3\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\"0.6\" stroke=\"#fff\" stroke-width=\"0.6\"/>");
        }

        draw_cat_label(&mut f, cx, cat);
        push_b(&mut f.buf, b"</g>");
    }

    finish_frame(&mut f, &cats, cfg.palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}


