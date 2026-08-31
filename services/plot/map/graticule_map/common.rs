use super::config::GraticuleMapConfig;
use crate::plot::map::world_data;

pub fn svg_open(width: i32, height: i32) -> String {
    let mut svg = String::with_capacity(8192);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" class=\"sp-bg\"/>");
    svg
}

pub fn push_base_outlines(svg: &mut String, width: i32, height: i32) {
    for shape in world_data::all_countries() {
        for poly in world_data::normalized_polygons(shape) {
            if poly.len() < 3 {
                continue;
            }
            svg.push_str("<path d=\"M");
            for (j, pt) in poly.iter().enumerate() {
                let px = pt[0] * width as f32;
                let py = pt[1] * height as f32;
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(" Z\" fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"0.5\" opacity=\"0.55\"/>");
        }
    }
}

pub fn lat_steps(step: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut lat = -90.0 + step;
    while lat < 90.0 {
        out.push(lat);
        lat += step;
    }
    out
}

pub fn lon_steps(step: f64) -> Vec<f64> {
    let mut out = Vec::new();
    let mut lon = -180.0;
    while lon <= 180.0 {
        out.push(lon);
        lon += step;
    }
    out
}

pub fn lerp_rgb(low: u32, high: u32, t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    let lr = ((low >> 16) & 0xFF) as f64;
    let lg = ((low >> 8) & 0xFF) as f64;
    let lb = (low & 0xFF) as f64;
    let hr = ((high >> 16) & 0xFF) as f64;
    let hg = ((high >> 8) & 0xFF) as f64;
    let hb = (high & 0xFF) as f64;
    (
        (lr + (hr - lr) * t).round() as u8,
        (lg + (hg - lg) * t).round() as u8,
        (lb + (hb - lb) * t).round() as u8,
    )
}

pub fn destination_point(lat0_deg: f64, lon0_deg: f64, dist_deg: f64, bearing_deg: f64) -> (f64, f64) {
    let lat0 = lat0_deg.to_radians();
    let lon0 = lon0_deg.to_radians();
    let delta = dist_deg.to_radians();
    let theta = bearing_deg.to_radians();
    let lat = (lat0.sin() * delta.cos() + lat0.cos() * delta.sin() * theta.cos()).asin();
    let lon = lon0 + (theta.sin() * delta.sin() * lat0.cos()).atan2(delta.cos() - lat0.sin() * lat.sin());
    (lat.to_degrees(), lon.to_degrees())
}

pub fn to_html(cfg: &GraticuleMapConfig, mut svg: String, hover_json: &str) -> String {
    use crate::html::hover::build_chart_html;
    if !svg.ends_with("</svg>") {
        svg.push_str("</svg>");
    }
    build_chart_html(cfg.title, &svg, hover_json)
}
