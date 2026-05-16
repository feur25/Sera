use super::common::{prepare, open_svg, ridge_label, project_pts, area_path, polyline, close_svg, finalize};
use super::config::RidgelineConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub const DEMO_KWARGS: &str = "categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"D\",\"D\",\"D\",\"D\",\"D\"], values=[1.2,2.4,2.7,3.1,3.5,2.0,2.8,3.2,3.6,4.1,1.8,2.2,2.6,3.0,3.4,2.3,2.9,3.5,3.9,4.4]";
fn lerp(a: u32, b: u32, t: f64) -> u32 {
    let ar = ((a >> 16) & 0xFF) as f64;
    let ag = ((a >> 8) & 0xFF) as f64;
    let ab = (a & 0xFF) as f64;
    let br = ((b >> 16) & 0xFF) as f64;
    let bg = ((b >> 8) & 0xFF) as f64;
    let bb = (b & 0xFF) as f64;
    let r = (ar + (br - ar) * t).round() as u32;
    let g = (ag + (bg - ag) * t).round() as u32;
    let bl = (ab + (bb - ab) * t).round() as u32;
    (r << 16) | (g << 8) | bl
}

fn viridis(t: f64) -> u32 {
    let stops: [u32; 5] = [0x440154, 0x3B528B, 0x21918C, 0x5EC962, 0xFDE725];
    let t = t.clamp(0.0, 1.0);
    let n = stops.len() - 1;
    let pos = t * n as f64;
    let lo = (pos.floor() as usize).min(n);
    let hi = (lo + 1).min(n);
    lerp(stops[lo], stops[hi], pos - lo as f64)
}

pub fn render(cfg: &RidgelineConfig) -> String {
    let p = match prepare(cfg, None) { Some(v) => v, None => return String::new() };
    let n_groups = p.group_order.len();
    let mut b = Vec::<u8>::with_capacity(n_groups * p.xs.len() * 22 + 2048);
    open_svg(&mut b, cfg, &p.layout, p.x0, p.xr);

    let denom = if n_groups > 1 { (n_groups - 1) as f64 } else { 1.0 };
    for gi in (0..n_groups).rev() {
        let t = gi as f64 / denom;
        let color = if cfg.palette.is_empty() { viridis(t) } else { crate::plot::statistical::common::palette_color(cfg.palette, gi) };
        let hx = hex6(color);
        let base_y = p.layout.title_h + (gi + 1) as i32 * p.layout.row_h;
        let pts = project_pts(&p, &p.curves[gi], base_y);

        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, gi as i32); push_b(&mut b, b"\">");
        area_path(&mut b, &pts, base_y as f64);
        push_b(&mut b, b" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\"0.85\"/>");
        polyline(&mut b, &pts, &hx, 1.4);
        ridge_label(&mut b, &p.layout, base_y, &p.group_order[gi]);
        push_b(&mut b, b"</g>");
    }

    close_svg(&mut b, cfg, &p, false);
    finalize(b, cfg)
}


