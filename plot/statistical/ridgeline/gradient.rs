use super::common::{prepare, open_svg, ridge_label, project_pts, area_path, polyline, close_svg, finalize};
use super::config::RidgelineConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, hex6};

pub fn render(cfg: &RidgelineConfig) -> String {
    let p = match prepare(cfg, None) { Some(v) => v, None => return String::new() };
    let n_groups = p.group_order.len();
    let mut b = Vec::<u8>::with_capacity(n_groups * p.xs.len() * 28 + 4096);
    open_svg(&mut b, cfg, &p.layout, p.x0, p.xr);

    push_b(&mut b, b"<defs>");
    for gi in 0..n_groups {
        let color = palette_color(cfg.palette, gi);
        let hx = hex6(color);
        push_b(&mut b, b"<linearGradient id=\"sp-rdg-"); push_i(&mut b, gi as i32);
        push_b(&mut b, b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">");
        push_b(&mut b, b"<stop offset=\"0%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.85\"/>");
        push_b(&mut b, b"<stop offset=\"60%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.30\"/>");
        push_b(&mut b, b"<stop offset=\"100%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0\"/>");
        push_b(&mut b, b"</linearGradient>");
    }
    push_b(&mut b, b"</defs>");

    for gi in (0..n_groups).rev() {
        let color = palette_color(cfg.palette, gi);
        let hx = hex6(color);
        let base_y = p.layout.title_h + (gi + 1) as i32 * p.layout.row_h;
        let pts = project_pts(&p, &p.curves[gi], base_y);

        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, gi as i32); push_b(&mut b, b"\">");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"url(#sp-rdg-"); push_i(&mut b, gi as i32);
        push_b(&mut b, b")\"/>");
        polyline(&mut b, &pts, &hx, 1.8);
        ridge_label(&mut b, &p.layout, base_y, &p.group_order[gi]);
        push_b(&mut b, b"</g>");
    }

    close_svg(&mut b, cfg, &p, true);
    finalize(b, cfg)
}


