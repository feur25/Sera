use super::common::{prepare, open_svg, ridge_label, project_pts, area_path, polyline, close_svg, finalize, x_to_px};
use super::config::RidgelineConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, hex6, push_f2};

pub fn render(cfg: &RidgelineConfig) -> String {
    let p = match prepare(cfg, None) { Some(v) => v, None => return String::new() };
    let n_groups = p.group_order.len();
    let mut b = Vec::<u8>::with_capacity(n_groups * (p.xs.len() * 24 + 200 * 14) + 2048);
    open_svg(&mut b, cfg, &p.layout, p.x0, p.xr);

    let tick_h: f64 = 7.0;

    for gi in (0..n_groups).rev() {
        let color = palette_color(cfg.palette, gi);
        let hx = hex6(color);
        let base_y = p.layout.title_h + (gi + 1) as i32 * p.layout.row_h;
        let pts = project_pts(&p, &p.curves[gi], base_y);

        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, gi as i32); push_b(&mut b, b"\">");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"#ffffff\"/>");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"#"); b.extend_from_slice(&hx);
        let op = (cfg.fill_opacity as f64 / 255.0).clamp(0.05, 1.0);
        push_b(&mut b, b"\" fill-opacity=\""); push_f2(&mut b, op); push_b(&mut b, b"\"/>");
        polyline(&mut b, &pts, &hx, 2.0);

        let vals = &p.group_vals[gi];
        let cap = 200usize;
        let step = if vals.len() > cap { (vals.len() + cap - 1) / cap } else { 1 };
        for v in vals.iter().step_by(step) {
            if !v.is_finite() { continue; }
            let xpx = x_to_px(&p, *v);
            push_b(&mut b, b"<line x1=\""); push_f2(&mut b, xpx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, base_y);
            push_b(&mut b, b"\" x2=\""); push_f2(&mut b, xpx);
            push_b(&mut b, b"\" y2=\""); push_f2(&mut b, base_y as f64 + tick_h);
            push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"0.8\" stroke-opacity=\"0.55\"/>");
        }

        ridge_label(&mut b, &p.layout, base_y, &p.group_order[gi]);
        push_b(&mut b, b"</g>");
    }

    close_svg(&mut b, cfg, &p, true);
    finalize(b, cfg)
}


