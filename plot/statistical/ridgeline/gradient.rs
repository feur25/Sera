use super::common::{
    area_path, close_svg, finalize, open_svg, polyline, prepare, project_pts, ridge_label,
};
use super::config::RidgelineConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_i};

#[crate::chart_demo("categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"D\",\"D\",\"D\",\"D\",\"D\"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]")]

pub fn render(cfg: &RidgelineConfig) -> String {
    let p = match prepare(cfg, None) {
        Some(v) => v,
        None => return String::new(),
    };
    let n_groups = p.group_order.len();
    let mut b = Vec::<u8>::with_capacity(n_groups * p.xs.len() * 28 + 4096);
    open_svg(&mut b, cfg, &p.layout, p.x0, p.xr);

    push_b(&mut b, b"<defs>");
    for gi in 0..n_groups {
        let color = palette_color(cfg.palette, gi);
        let hx = hex6(color);
        push_b(&mut b, b"<linearGradient id=\"sp-rdg-");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">");
        push_b(&mut b, b"<stop offset=\"0%\" stop-color=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.85\"/>");
        push_b(&mut b, b"<stop offset=\"60%\" stop-color=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.30\"/>");
        push_b(&mut b, b"<stop offset=\"100%\" stop-color=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0\"/>");
        push_b(&mut b, b"</linearGradient>");
    }
    push_b(&mut b, b"</defs>");

    for gi in (0..n_groups).rev() {
        let color = palette_color(cfg.palette, gi);
        let hx = hex6(color);
        let base_y = p.layout.title_h + (gi + 1) as i32 * p.layout.row_h;
        let pts = project_pts(&p, &p.curves[gi], base_y);

        push_b(&mut b, b"<g data-series=\"");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b"\" data-idx=\"");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b"\">");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"url(#sp-rdg-");
        push_i(&mut b, gi as i32);
        push_b(&mut b, b")\"/>");
        polyline(&mut b, &pts, &hx, 1.8);
        ridge_label(&mut b, &p.layout, base_y, &p.group_order[gi]);
        push_b(&mut b, b"</g>");
    }

    close_svg(&mut b, cfg, &p, true);
    finalize(b, cfg)
}
