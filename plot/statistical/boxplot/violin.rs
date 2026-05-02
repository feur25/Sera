use super::common::{
    compute_box, draw_box_vertical, draw_cat_label, draw_outliers_vertical, finish_frame,
    global_range, group_values, make_frame, open_axes, sorted_groups,
};
use super::config::BoxplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

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
    let box_hw = (slot_w * 0.18) as i32;
    let half_w = (slot_w * 0.42) as i32;

    open_axes(&mut f, cfg.title, cfg.gridlines, gr.y_min, gr.y_max);

    for (ci, ((cat, grp), st)) in cats.iter().zip(groups.iter()).zip(stats.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32); push_b(&mut f.buf, b"\">");

        let density = kde_profile(grp, gr.y_min, gr.y_max, 64);
        let max_d = density.iter().map(|d| d.1).fold(0f64, f64::max).max(1e-9);
        let n_pts = density.len();
        push_b(&mut f.buf, b"<path d=\"");
        for i in 0..n_pts {
            let (yv, dv) = density[i];
            let py = f.pt + f.ph - ((yv - gr.y_min) / gr.range_y * f.ph as f64) as i32;
            let dx = (dv / max_d * half_w as f64) as i32;
            if i == 0 {
                push_b(&mut f.buf, b"M ");
            } else {
                push_b(&mut f.buf, b" L ");
            }
            push_i(&mut f.buf, cx - dx);
            push_b(&mut f.buf, b" ");
            push_i(&mut f.buf, py);
        }
        for i in (0..n_pts).rev() {
            let (yv, dv) = density[i];
            let py = f.pt + f.ph - ((yv - gr.y_min) / gr.range_y * f.ph as f64) as i32;
            let dx = (dv / max_d * half_w as f64) as i32;
            push_b(&mut f.buf, b" L ");
            push_i(&mut f.buf, cx + dx);
            push_b(&mut f.buf, b" ");
            push_i(&mut f.buf, py);
        }
        push_b(&mut f.buf, b" Z\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.22\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.4\" stroke-opacity=\"0.85\"/>");
        let _ = push_f2;

        draw_box_vertical(&mut f, cx, box_hw, st, color, cfg.fill_opacity + 0.18, cfg.stroke_width, cfg.notch, cat, ci as i32, gr.y_min, gr.range_y);
        draw_outliers_vertical(&mut f, cx, st, color, 3.0, gr.y_min, gr.range_y);
        draw_cat_label(&mut f, cx, cat);
        push_b(&mut f.buf, b"</g>");
    }

    finish_frame(&mut f, &cats, cfg.palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}

fn kde_profile(vals: &[f64], y_min: f64, y_max: f64, n_pts: usize) -> Vec<(f64, f64)> {
    let clean: Vec<f64> = vals.iter().copied().filter(|v| v.is_finite()).collect();
    if clean.is_empty() || y_max <= y_min {
        return Vec::new();
    }
    let n = clean.len() as f64;
    let mean = clean.iter().sum::<f64>() / n;
    let var = clean.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n.max(1.0);
    let sd = var.sqrt().max(1e-9);
    let bw = 1.06 * sd * n.powf(-1.0 / 5.0);
    let bw = bw.max((y_max - y_min) * 0.01);
    let step = (y_max - y_min) / (n_pts - 1).max(1) as f64;
    (0..n_pts)
        .map(|i| {
            let yv = y_min + step * i as f64;
            let mut s = 0.0;
            for &x in &clean {
                let u = (yv - x) / bw;
                s += (-0.5 * u * u).exp();
            }
            (yv, s / (n * bw * (2.0 * std::f64::consts::PI).sqrt()))
        })
        .collect()
}
