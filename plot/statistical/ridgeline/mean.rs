use super::common::{prepare, open_svg, ridge_label, project_pts, area_path, polyline, close_svg, finalize, x_to_px, y_at_x};
use super::config::RidgelineConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, hex6, push_f2};

pub const DEMO_KWARGS: &str = "categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"D\",\"D\",\"D\",\"D\",\"D\"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]";
pub fn render(cfg: &RidgelineConfig) -> String {
    let p = match prepare(cfg, None) { Some(v) => v, None => return String::new() };
    let n_groups = p.group_order.len();
    let mut b = Vec::<u8>::with_capacity(n_groups * p.xs.len() * 24 + 2048);
    open_svg(&mut b, cfg, &p.layout, p.x0, p.xr);

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

        let vals = &p.group_vals[gi];
        if !vals.is_empty() {
            let mean: f64 = vals.iter().filter(|v| v.is_finite()).copied().sum::<f64>() / vals.len() as f64;
            let mx = x_to_px(&p, mean);
            let my = y_at_x(&p, &p.curves[gi], mean, base_y);
            push_b(&mut b, b"<line x1=\""); push_f2(&mut b, mx);
            push_b(&mut b, b"\" y1=\""); push_f2(&mut b, my);
            push_b(&mut b, b"\" x2=\""); push_f2(&mut b, mx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, base_y);
            push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1.6\" stroke-dasharray=\"4,3\"/>");
            push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, mx);
            push_b(&mut b, b"\" cy=\""); push_i(&mut b, base_y);
            push_b(&mut b, b"\" r=\"4.5\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke=\"#ffffff\" stroke-width=\"1.5\"/>");
        }

        polyline(&mut b, &pts, &hx, 2.0);
        ridge_label(&mut b, &p.layout, base_y, &p.group_order[gi]);
        push_b(&mut b, b"</g>");
    }

    close_svg(&mut b, cfg, &p, true);
    finalize(b, cfg)
}


