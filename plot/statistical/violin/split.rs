use super::common::{
    draw_cat_label_v, estimate_bw, finish, group_data, kde_curve, make_frame, open_axes_y,
    sort_groups, value_range, write_violin_v, Side,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, palette_color, push_b, push_i, truncate};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"], color_groups=[\"L\",\"L\",\"L\",\"R\",\"R\",\"R\",\"L\",\"L\",\"L\",\"R\",\"R\",\"R\",\"L\",\"L\",\"L\",\"R\",\"R\",\"R\"]")]

pub fn render(cfg: &ViolinConfig) -> String {
    let groups = group_data(cfg.categories, cfg.values);
    if groups.is_empty() {
        return String::new();
    }
    let groups = sort_groups(groups, cfg.sort_order);
    let n_cats = groups.len();
    let n_pairs = (n_cats + 1) / 2;
    let vr = value_range(&groups);

    let legend_w: i32 = if n_cats > 1 { 130 } else { 20 };
    let mut f = make_frame(cfg, n_pairs.max(1), legend_w);
    let slot_w = f.pw as f64 / n_pairs as f64;
    let half_w = (slot_w * 0.45) as i32;
    open_axes_y(&mut f, cfg.title, cfg.gridlines, vr.min, vr.max);

    for (ci, g) in groups.iter().enumerate() {
        let pair_idx = ci / 2;
        let in_pair = ci % 2;
        let cx = f.pl + (pair_idx as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(cfg.palette, ci);
        let side = if in_pair == 0 {
            Side::Left
        } else {
            Side::Right
        };
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
            half_w,
            side,
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
        push_b(&mut f.buf, b"</g>");
    }

    for pair_idx in 0..n_pairs {
        let cx = f.pl + (pair_idx as f64 * slot_w + slot_w / 2.0) as i32;
        let a = &groups[pair_idx * 2];
        let b_label = if pair_idx * 2 + 1 < n_cats {
            Some(&groups[pair_idx * 2 + 1].label)
        } else {
            None
        };
        let combined = match b_label {
            Some(lb) => format!("{} | {}", a.label, lb),
            None => a.label.clone(),
        };
        push_b(&mut f.buf, b"<line x1=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\"");
        push_i(&mut f.buf, f.pt);
        push_b(&mut f.buf, b"\" x2=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\"");
        push_i(&mut f.buf, f.pt + f.ph);
        push_b(
            &mut f.buf,
            b"\" stroke=\"#cbd5e1\" stroke-width=\"0.6\" stroke-dasharray=\"3 3\"/>",
        );
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut f.buf, truncate(&combined, 22));
        push_b(&mut f.buf, b"</text>");
    }
    let _ = draw_cat_label_v;

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
