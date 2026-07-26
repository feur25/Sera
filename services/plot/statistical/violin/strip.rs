use super::common::{
    draw_cat_label_v, draw_points_v_r, finish, group_data, make_frame, open_axes_y, sort_groups,
    value_range,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{palette_color, push_b, push_i};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], values=[1.2,1.6,1.9,2.1,2.4,2.4,2.6,2.7,2.9,3.1,3.2,3.4,3.5,3.6,3.8,4.0,4.2,4.5,1.5,1.8,2.0,2.3,2.5,2.8,2.8,3.0,3.2,3.3,3.6,3.7,3.9,4.1,4.3,4.5,4.8,5.0,1.0,1.4,1.7,1.9,2.1,2.3,2.5,2.6,2.8,3.0,3.1,3.3,3.5,3.7,3.9,4.1,4.4,4.7], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"]")]

pub fn render(cfg: &ViolinConfig) -> String {
    let groups = group_data(cfg.categories, cfg.values);
    if groups.is_empty() {
        return String::new();
    }
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
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");
        draw_points_v_r(
            &mut f,
            cx,
            half_w,
            g,
            vr.min,
            vr.range,
            color,
            cfg.jitter.max(0.5),
            &mut rng,
            4.0,
            0.8,
        );
        draw_cat_label_v(&mut f, cx, &g.label);
        push_b(&mut f.buf, b"</g>");
    }

    let names: Vec<&str> = groups.iter().map(|g| g.label.as_str()).collect();
    finish(
        &mut f,
        &names,
        cfg.palette,
        cfg.x_label,
        cfg.y_label,
        legend_w,
    );
    f.html(&slots_to_json(cfg.hover))
}
