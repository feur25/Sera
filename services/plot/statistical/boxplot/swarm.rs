use super::common::{
    beeswarm_offsets, compute_box, draw_cat_label, finish_frame, global_range, group_values,
    make_frame, open_axes, sorted_groups,
};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]")]

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
    let half = slot_w * 0.42;
    let radius = 3.2f64;

    for (ci, ((cat, grp), st)) in cats.iter().zip(groups.iter()).zip(stats.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");

        let y_med = f.pt + f.ph - ((st.median - gr.y_min) / gr.range_y * f.ph as f64) as i32;
        push_b(&mut f.buf, b"<line x1=\"");
        push_i(&mut f.buf, cx - half as i32);
        push_b(&mut f.buf, b"\" y1=\"");
        push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" x2=\"");
        push_i(&mut f.buf, cx + half as i32);
        push_b(&mut f.buf, b"\" y2=\"");
        push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(
            &mut f.buf,
            b"\" stroke-opacity=\"0.65\" stroke-width=\"1.6\" stroke-dasharray=\"4,3\"/>",
        );

        let py: Vec<i32> = grp
            .iter()
            .map(|&v| f.pt + f.ph - ((v - gr.y_min) / gr.range_y * f.ph as f64) as i32)
            .collect();
        let offsets = beeswarm_offsets(&py, radius, half);

        for (pi, &v) in grp.iter().enumerate() {
            if !v.is_finite() {
                continue;
            }
            push_b(&mut f.buf, b"<circle data-idx=\"");
            push_i(&mut f.buf, ci as i32);
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, cat);
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, v);
            push_b(&mut f.buf, b"\" cx=\"");
            push_i(&mut f.buf, cx + offsets[pi] as i32);
            push_b(&mut f.buf, b"\" cy=\"");
            push_i(&mut f.buf, py[pi]);
            push_b(&mut f.buf, b"\" r=\"");
            push_f2(&mut f.buf, radius);
            push_b(&mut f.buf, b"\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(
                &mut f.buf,
                b"\" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"0.6\"/>",
            );
        }

        draw_cat_label(&mut f, cx, cat);
        push_b(&mut f.buf, b"</g>");
    }

    finish_frame(
        &mut f,
        &cats,
        cfg.palette,
        cfg.x_label,
        cfg.y_label,
        legend_w,
    );
    f.html(&slots_to_json(cfg.hover))
}
