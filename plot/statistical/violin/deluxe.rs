use super::common::{
    draw_cat_label_v, draw_inner_box_v, estimate_bw, finish, group_data, kde_curve,
    make_frame, open_axes_y, sort_groups, value_range,
};
use super::config::ViolinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn deluxe_palette(ci: usize) -> (u32, u32) {
    const STOPS: [(u32, u32); 6] = [
        (0x06B6D4, 0x6366F1),
        (0x10B981, 0x0EA5E9),
        (0xA855F7, 0x06B6D4),
        (0xF43F5E, 0x8B5CF6),
        (0x0EA5E9, 0x10B981),
        (0xF59E0B, 0xEF4444),
    ];
    STOPS[ci % STOPS.len()]
}

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

    push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt);
    push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, f.pw);
    push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, f.ph);
    push_b(&mut f.buf, b"\" fill=\"#0f172a\" rx=\"3\"/>");

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"dlxvf\" x=\"-80%\" y=\"-10%\" width=\"260%\" height=\"120%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    for ci in 0..n_cats {
        let (bot, top) = deluxe_palette(ci);
        push_b(&mut f.buf, b"<linearGradient id=\"dlxvg");
        push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(bot));
        push_b(&mut f.buf, b"\"/><stop offset=\"1\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(top));
        push_b(&mut f.buf, b"\"/></linearGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    let palette: Vec<u32> = (0..n_cats).map(|ci| deluxe_palette(ci).0).collect();

    for (ci, g) in groups.iter().enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\">");

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
        push_b(&mut f.buf, b" Z\" fill=\"url(#dlxvg"); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b")\" fill-opacity=\""); push_f2(&mut f.buf, (cfg.fill_opacity + 0.2).min(0.92));
        push_b(&mut f.buf, b"\" stroke=\"url(#dlxvg"); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b")\" stroke-width=\""); push_f2(&mut f.buf, (cfg.stroke_width + 0.5).min(2.5));
        push_b(&mut f.buf, b"\" filter=\"url(#dlxvf)\"/>");

        draw_inner_box_v(&mut f, cx, half_w, g, vr.min, vr.range);
        draw_cat_label_v(&mut f, cx, &g.label);
        push_b(&mut f.buf, b"</g>");
    }

    let names: Vec<&str> = groups.iter().map(|g| g.label.as_str()).collect();
    finish(&mut f, &names, &palette, cfg.x_label, cfg.y_label, legend_w);
    f.html(&slots_to_json(cfg.hover))
}
