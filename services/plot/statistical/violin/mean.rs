use super::common::{
    draw_cat_label_v, draw_mean_v, estimate_bw, finish, group_data, kde_curve, make_frame,
    open_axes_y, sort_groups, value_range, write_violin_v, Side,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{palette_color, push_b, push_i};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"]")]

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

    for (ci, g) in groups.iter().enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");
        let bw = estimate_bw(&g.sorted, cfg.bandwidth);
        let dens = kde_curve(&g.sorted, vr.min, vr.range, cfg.kde_steps, bw);
        let max_d = dens.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        let kv = [
            ("Mean", g.mean),
            ("Median", g.median),
            ("Q1", g.q1),
            ("Q3", g.q3),
            ("N", g.n as f64),
        ];
        write_violin_v(
            &mut f,
            cx,
            half_w,
            Side::Both,
            &dens,
            max_d,
            vr.min,
            vr.range,
            color,
            cfg.fill_opacity,
            cfg.stroke_width,
            ci as i32,
            &g.label,
            &kv,
        );
        draw_mean_v(&mut f, cx, half_w, g, vr.min, vr.range);
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
