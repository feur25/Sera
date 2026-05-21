use super::common::{

    draw_cat_label_v, draw_inner_box_v, estimate_bw, finish, group_data, kde_curve,
    make_frame, open_axes_y, sort_groups, value_range,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn crystal_color(ci: usize) -> u32 {
    const COLS: [u32; 6] = [
        0x7DD3FC, 0xC4B5FD, 0x6EE7B7, 0xFDA4AF,
        0xFDE68A, 0xA5B4FC,
    ];
    COLS[ci % COLS.len()]
}

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"]")]

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

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"crygf\" x=\"-30%\" y=\"-10%\" width=\"160%\" height=\"120%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"2.5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    for ci in 0..n_cats {
        let col = crystal_color(ci);
        push_b(&mut f.buf, b"<linearGradient id=\"cryg");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.08\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.3\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.55\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.55\" stop-color=\"#ffffff\" stop-opacity=\"0.72\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.75\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.38\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.1\"/>");
        push_b(&mut f.buf, b"</linearGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    let palette: Vec<u32> = (0..n_cats).map(crystal_color).collect();

    for (ci, g) in groups.iter().enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");

        let col = crystal_color(ci);
        let hx = hex6(col);
        let bw = estimate_bw(&g.sorted, cfg.bandwidth);
        let dens = kde_curve(&g.sorted, vr.min, vr.range, cfg.kde_steps, bw);
        let max_d = dens.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        let kv: [(&str, f64); 5] = [
            ("Median", g.median),
            ("Q1", g.q1),
            ("Q3", g.q3),
            ("Mean", g.mean),
            ("N", g.n as f64),
        ];
        let steps = dens.len() - 1;
        let yv = |si: usize| -> i32 { f.pt + f.ph - (si as f64 / steps as f64 * f.ph as f64) as i32 };
        let w_at = |si: usize| -> i32 { (dens[si] / max_d * half_w as f64) as i32 };

        push_b(&mut f.buf, b"<path data-idx=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &g.label);
        for (k, v) in &kv {
            push_b(&mut f.buf, b"\" data-kv-"); push_b(&mut f.buf, k.as_bytes());
            push_b(&mut f.buf, b"=\""); push_f2(&mut f.buf, *v);
        }
        push_b(&mut f.buf, b"\" d=\"M ");
        push_i(&mut f.buf, cx + w_at(0)); f.buf.push(b' '); push_i(&mut f.buf, yv(0));
        for si in 1..=steps {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx + w_at(si));
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
        for si in (0..=steps).rev() {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx - w_at(si));
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
        push_b(&mut f.buf, b" Z\" fill=\"url(#cryg"); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b")\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.2\" stroke-opacity=\"0.65\" filter=\"url(#crygf)\"/>");

        let stripe_step = 8;
        let top_y = yv(steps);
        let bot_y = yv(0);
        let mut sy = top_y;
        while sy < bot_y {
            let si_frac = (bot_y - sy).max(1);
            let ki = (steps as f64 * (sy - top_y) as f64 / (bot_y - top_y).max(1) as f64) as usize;
            let ki = ki.min(steps);
            let hw = w_at(ki);
            if hw > 2 {
                push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - hw);
                push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, sy);
                push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + hw);
                push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, sy);
                push_b(&mut f.buf, b"\" stroke=\"#ffffff\" stroke-width=\"0.5\" stroke-opacity=\"0.22\"/>");
                let _ = si_frac;
            }
            sy += stripe_step;
        }

        push_b(&mut f.buf, b"<path d=\"M ");
        push_i(&mut f.buf, cx + w_at(0)); f.buf.push(b' '); push_i(&mut f.buf, yv(0));
        for si in 1..=steps {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx + w_at(si));
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
        for si in (0..=steps).rev() {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx - w_at(si));
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
        push_b(&mut f.buf, b" Z\" fill=\"none\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.8\" stroke-opacity=\"0.9\"/>");

        draw_inner_box_v(&mut f, cx, half_w, g, vr.min, vr.range);
        draw_cat_label_v(&mut f, cx, &g.label);
        push_b(&mut f.buf, b"</g>");
    }

    let names: Vec<&str> = groups.iter().map(|g| g.label.as_str()).collect();
    finish(&mut f, &names, &palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}

