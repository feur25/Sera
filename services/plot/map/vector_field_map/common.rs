use super::config::VectorFieldMapConfig;
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
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0d1117\"/>");
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
            svg.push_str(" Z\" fill=\"none\" stroke=\"#3a4a5a\" stroke-width=\"0.6\" opacity=\"0.7\"/>");
        }
    }
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

pub struct FieldBounds {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub max_mag: f64,
}

pub fn field_bounds(cfg: &VectorFieldMapConfig) -> FieldBounds {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.u.len()).min(cfg.v.len());
    let pad = 6.0;
    let min_lat = cfg.lats[..n].iter().cloned().fold(f64::INFINITY, f64::min) - pad;
    let max_lat = cfg.lats[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) + pad;
    let min_lon = cfg.lons[..n].iter().cloned().fold(f64::INFINITY, f64::min) - pad;
    let max_lon = cfg.lons[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) + pad;
    let max_mag = (0..n)
        .map(|i| (cfg.u[i] * cfg.u[i] + cfg.v[i] * cfg.v[i]).sqrt())
        .fold(1e-9_f64, f64::max);
    FieldBounds { min_lat, max_lat, min_lon, max_lon, max_mag }
}

pub fn idw_uv(lats: &[f64], lons: &[f64], u: &[f64], v: &[f64], at_lat: f64, at_lon: f64) -> (f64, f64) {
    let n = lats.len().min(lons.len()).min(u.len()).min(v.len());
    let mut num_u = 0.0;
    let mut num_v = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        let dlat = lats[i] - at_lat;
        let dlon = lons[i] - at_lon;
        let d2 = dlat * dlat + dlon * dlon;
        if d2 < 1e-9 {
            return (u[i], v[i]);
        }
        let w = 1.0 / (d2 * d2);
        num_u += w * u[i];
        num_v += w * v[i];
        den += w;
    }
    if den <= 0.0 {
        (0.0, 0.0)
    } else {
        (num_u / den, num_v / den)
    }
}

pub fn project(lat: f64, lon: f64, width: i32, height: i32) -> (f32, f32) {
    let (nx, ny) = world_data::latlon_to_normalized(lat, lon);
    (nx * width as f32, ny * height as f32)
}

pub fn to_html(cfg: &VectorFieldMapConfig, mut svg: String) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.u.len()).min(cfg.v.len());
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        let mag = (cfg.u[i] * cfg.u[i] + cfg.v[i] * cfg.v[i]).sqrt();
        slots.push(
            HoverSlot::new(format!("p{i}"))
                .kv("Lat", format!("{:.2}", cfg.lats[i]))
                .kv("Lon", format!("{:.2}", cfg.lons[i]))
                .kv("Magnitude", format!("{mag:.2}"))
                .kv("U / V", format!("{:.2} / {:.2}", cfg.u[i], cfg.v[i])),
        );
    }
    if !svg.ends_with("</svg>") {
        svg.push_str("</svg>");
    }
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}
