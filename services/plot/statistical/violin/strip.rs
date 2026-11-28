use super::common::{
    draw_cat_label_v, estimate_bw, finish, group_data, kde_curve, make_frame, open_axes_y,
    rng_next, sort_groups, value_range, write_violin_v, Side,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\"], values=[1.2,1.6,1.9,2.1,2.4,2.4,2.6,2.7,2.9,3.1,3.2,3.4,3.5,3.6,3.8,4.0,4.2,4.5,1.5,1.8,2.0,2.3,2.5,2.8,2.8,3.0,3.2,3.3,3.6,3.7,3.9,4.1,4.3,4.5,4.8,5.0,1.0,1.4,1.7,1.9,2.1,2.3,2.5,2.6,2.8,3.0,3.1,3.3,3.5,3.7,3.9,4.1,4.4,4.7], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"]"
)]

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
    let cloud_w = (slot_w * 0.30) as i32;
    let strip_w = (slot_w * 0.34) as i32;
    open_axes_y(&mut f, cfg.title, cfg.gridlines, vr.min, vr.max);

    let mut rng: u64 = 0xD1B54A32D192ED03;
    for (ci, g) in groups.iter().enumerate() {
        let slot_l = f.pl + (ci as f64 * slot_w) as i32;
        let cx = slot_l + (slot_w * 0.42) as i32;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");

        let bw = estimate_bw(&g.sorted, cfg.bandwidth);
        let dens = kde_curve(&g.sorted, vr.min, vr.range, cfg.kde_steps, bw);
        let max_d = dens.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        let kv = [
            ("Median", g.median),
            ("Q1", g.q1),
            ("Q3", g.q3),
            ("Mean", g.mean),
            ("N", g.n as f64),
        ];
        write_violin_v(
            &mut f,
            cx,
            cloud_w,
            Side::Left,
            &dens,
            max_d,
            color,
            cfg.fill_opacity * 0.7,
            cfg.stroke_width,
            ci as i32,
            &g.label,
            &kv,
        );

        let yv = |v: f64| f.pt + f.ph - ((v - vr.min) / vr.range * f.ph as f64) as i32;
        let strip_x0 = cx + (cloud_w as f64 * 0.28) as i32;
        let point_step = ((g.sorted.len() as f64 / 6_000.0).ceil() as usize).max(1);
        for &v in g.sorted.iter().step_by(point_step) {
            if !v.is_finite() {
                continue;
            }
            let dx = rng_next(&mut rng) * strip_w as f64 * cfg.jitter.max(0.5).min(1.0);
            let py = yv(v);
            push_b(&mut f.buf, b"<circle cx=\"");
            push_f2(&mut f.buf, strip_x0 as f64 + dx);
            push_b(&mut f.buf, b"\" cy=\"");
            push_i(&mut f.buf, py);
            push_b(&mut f.buf, b"\" r=\"3.4\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\"0.75\" stroke=\"#fff\" stroke-width=\"0.7\"/>");
        }

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
