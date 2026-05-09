use super::common::{
    draw_cat_label_v, draw_points_v, finish, group_data, make_frame, open_axes_y,
    sort_groups, value_range,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{palette_color, push_b, push_i};

pub fn render(cfg: &ViolinConfig) -> String {
    let groups = group_data(cfg.categories, cfg.values);
    if groups.is_empty() { return String::new(); }
    let groups = sort_groups(groups, cfg.sort_order);
    let n_cats = groups.len();
    let vr = value_range(&groups);

    let legend_w: i32 = if n_cats > 1 { 130 } else { 20 };
    let mut f = make_frame(cfg, n_cats, legend_w);
    let slot_w = f.pw as f64 / n_cats as f64;
    let half_w = (slot_w * 0.42) as i32;
    open_axes_y(&mut f, cfg.title, cfg.gridlines, vr.min, vr.max);

    let mut rng: u64 = 0xD1B54A32D192ED03;
    for (ci, g) in groups.iter().enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");
        draw_points_v(&mut f, cx, half_w, g, vr.min, vr.range, color, cfg.jitter.max(0.5), &mut rng);
        draw_cat_label_v(&mut f, cx, &g.label);
        push_b(&mut f.buf, b"</g>");
    }

    let names: Vec<&str> = groups.iter().map(|g| g.label.as_str()).collect();
    finish(&mut f, &names, cfg.palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}


