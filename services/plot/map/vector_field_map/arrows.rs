use super::common::{field_bounds, lerp_rgb, project, push_base_outlines, svg_open, to_html};
use super::config::VectorFieldMapConfig;

const MAX_ARROW_LEN: f64 = 42.0;
const MIN_ARROW_LEN: f64 = 8.0;

fn push_arrow(svg: &mut String, x: f32, y: f32, dx: f32, dy: f32, r: u8, g: u8, b: u8) {
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let ux = dx / len;
    let uy = dy / len;
    let tip_x = x + dx;
    let tip_y = y + dy;
    let head_len = (len * 0.32).clamp(2.5, 9.0);
    let head_w = head_len * 0.55;
    let bx = tip_x - ux * head_len;
    let by = tip_y - uy * head_len;
    let px = -uy;
    let py = ux;
    svg.push_str(&format!(
        "<line x1=\"{x:.1}\" y1=\"{y:.1}\" x2=\"{bx:.1}\" y2=\"{by:.1}\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"1.6\" stroke-linecap=\"round\"/>"
    ));
    svg.push_str(&format!(
        "<polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"rgb({r},{g},{b})\"/>",
        tip_x,
        tip_y,
        bx + px * head_w,
        by + py * head_w,
        bx - px * head_w,
        by - py * head_w,
    ));
}

#[crate::chart_demo(
    "lats=[50,45,40,35,50,45,40,35,50,45,40,35], lons=[-10,-5,0,5,-10,-5,0,5,-10,-5,0,5], u=[8,5,-3,-9,4,7,2,-6,-2,-8,6,3], v=[3,-6,8,-2,-9,1,-7,5,6,-3,-4,9], title=\"Sampled Wind Vectors\""
)]
pub fn render(cfg: &VectorFieldMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.u.len()).min(cfg.v.len());
    if n == 0 {
        return String::new();
    }
    let bounds = field_bounds(cfg);
    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    for i in 0..n {
        let (px, py) = project(cfg.lats[i], cfg.lons[i], cfg.width, cfg.height);
        let mag = (cfg.u[i] * cfg.u[i] + cfg.v[i] * cfg.v[i]).sqrt();
        let t = mag / bounds.max_mag;
        let len = MIN_ARROW_LEN + t * (MAX_ARROW_LEN - MIN_ARROW_LEN);
        let ux = cfg.u[i] / mag.max(1e-9);
        let uy = -cfg.v[i] / mag.max(1e-9);
        let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, t);
        push_arrow(&mut svg, px, py, (ux * len) as f32, (uy * len) as f32, r, g, b);
        svg.push_str(&format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"2.2\" fill=\"rgb({r},{g},{b})\" data-index=\"{i}\"/>"
        ));
    }

    to_html(cfg, svg)
}
