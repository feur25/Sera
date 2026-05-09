use super::common::{
    draw_cat_label_h, estimate_bw, finish, group_data, kde_curve, make_frame,
    open_axes_x, sort_groups, value_range, write_violin_h,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

pub fn render(cfg: &ViolinConfig) -> String {
    let groups = group_data(cfg.categories, cfg.values);
    if groups.is_empty() { return String::new(); }
    let groups = sort_groups(groups, cfg.sort_order);
    let n_cats = groups.len();
    let vr = value_range(&groups);

    let legend_w: i32 = if n_cats > 1 { 130 } else { 20 };
    let mut f = make_frame(cfg, n_cats, legend_w);
    let slot_h = f.ph as f64 / n_cats as f64;
    let half_h = (slot_h * 0.42) as i32;
    open_axes_x(&mut f, cfg.title, cfg.gridlines, vr.min, vr.max);

    for (ci, g) in groups.iter().enumerate() {
        let cy = f.pt + (ci as f64 * slot_h + slot_h / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");
        let bw = estimate_bw(&g.sorted, cfg.bandwidth);
        let dens = kde_curve(&g.sorted, vr.min, vr.range, cfg.kde_steps, bw);
        let max_d = dens.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        write_violin_h(&mut f, cy, half_h, &dens, max_d, vr.min, vr.range,
            color, cfg.fill_opacity, cfg.stroke_width, ci as i32, &g.label);
        let xv = |v: f64| f.pl + ((v - vr.min) / vr.range * f.pw as f64) as i32;
        let hx = hex6(color);
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, xv(g.q1));
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy - (half_h as f64 * 0.25) as i32);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, xv(g.q3));
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy - (half_h as f64 * 0.25) as i32);
        push_b(&mut f.buf, b"\" stroke=\"#1a202c\" stroke-width=\"6\" stroke-linecap=\"round\" stroke-opacity=\"0.85\"/>");
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, xv(g.median));
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy - (half_h as f64 * 0.25) as i32);
        push_b(&mut f.buf, b"\" r=\"2.4\" fill=\"#fff\" data-kv-Median=\""); push_f2(&mut f.buf, g.median);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &g.label);
        push_b(&mut f.buf, b"\"/>");
        let _ = hx;
        draw_cat_label_h(&mut f, cy, &g.label);
        push_b(&mut f.buf, b"</g>");
    }

    let names: Vec<&str> = groups.iter().map(|g| g.label.as_str()).collect();
    finish(&mut f, &names, cfg.palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}


