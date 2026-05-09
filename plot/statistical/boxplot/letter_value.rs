use super::common::{
    compute_box, draw_cat_label, finish_frame, global_range, group_values, make_frame,
    open_axes, quartile, sorted_groups,
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
    let plot_w = f.pw;
    let slot_w = plot_w as f64 / n_cats as f64;
    let max_hw = (slot_w * 0.36) as i32;
    let depth = cfg.boxen_depth.max(2).min(7);

    open_axes(&mut f, cfg.title, cfg.gridlines, gr.y_min, gr.y_max);

    for (ci, ((cat, grp), st)) in cats.iter().zip(groups.iter()).zip(stats.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32); push_b(&mut f.buf, b"\">");

        let mut sorted_vals: Vec<f64> = grp.iter().copied().filter(|v| v.is_finite()).collect();
        sorted_vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let yv = |v: f64| f.pt + f.ph - ((v - gr.y_min) / gr.range_y * f.ph as f64) as i32;

        for k in 0..depth {
            let frac = 0.5_f64.powi(k as i32 + 1);
            let q_lo = quartile(&sorted_vals, frac);
            let q_hi = quartile(&sorted_vals, 1.0 - frac);
            let band_top = yv(q_hi).min(yv(q_lo));
            let band_bot = yv(q_hi).max(yv(q_lo));
            let band_h = (band_bot - band_top).max(2);
            let scale = 1.0 - (k as f64) / depth as f64;
            let hw = ((max_hw as f64) * scale).max(4.0) as i32;
            let opacity = 0.18 + 0.55 * scale;

            push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, ci as i32);
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, cat);
            push_b(&mut f.buf, b"\" data-kv-Quantile=\""); push_f2(&mut f.buf, frac * 100.0);
            push_b(&mut f.buf, b"\" data-kv-LowerVal=\""); push_f2(&mut f.buf, q_lo);
            push_b(&mut f.buf, b"\" data-kv-UpperVal=\""); push_f2(&mut f.buf, q_hi);
            push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, cx - hw);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, band_top);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, hw * 2);
            push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, band_h);
            push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, opacity);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"0.6\" rx=\"2\"/>");
        }

        let y_med = yv(st.median);
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - max_hw);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + max_hw);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2.2\"/>");

        for &ov in &st.outliers {
            let oy = yv(ov);
            push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, oy);
            push_b(&mut f.buf, b"\" r=\"2.5\" fill=\"#9ca3af\" fill-opacity=\"0.7\"/>");
        }

        draw_cat_label(&mut f, cx, cat);
        push_b(&mut f.buf, b"</g>");
    }

    finish_frame(&mut f, &cats, cfg.palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}


