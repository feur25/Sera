use super::config::ContourMapConfig;
use crate::plot::map::world_data;

pub const GRID_COLS: usize = 64;
pub const GRID_ROWS: usize = 40;

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
    pub min_val: f64,
    pub max_val: f64,
}

pub fn field_bounds(cfg: &ContourMapConfig) -> FieldBounds {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    let pad = 6.0;
    let min_lat = cfg.lats[..n].iter().cloned().fold(f64::INFINITY, f64::min) - pad;
    let max_lat = cfg.lats[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) + pad;
    let min_lon = cfg.lons[..n].iter().cloned().fold(f64::INFINITY, f64::min) - pad;
    let max_lon = cfg.lons[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) + pad;
    let min_val = cfg.field[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = cfg.field[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    FieldBounds { min_lat, max_lat, min_lon, max_lon, min_val, max_val }
}

pub fn idw_interpolate(lats: &[f64], lons: &[f64], field: &[f64], at_lat: f64, at_lon: f64) -> f64 {
    let n = lats.len().min(lons.len()).min(field.len());
    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..n {
        let dlat = lats[i] - at_lat;
        let dlon = lons[i] - at_lon;
        let d2 = dlat * dlat + dlon * dlon;
        if d2 < 1e-9 {
            return field[i];
        }
        let w = 1.0 / (d2 * d2);
        num += w * field[i];
        den += w;
    }
    if den <= 0.0 {
        0.0
    } else {
        num / den
    }
}

pub fn build_grid(cfg: &ContourMapConfig, bounds: &FieldBounds, cols: usize, rows: usize) -> Vec<Vec<f64>> {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    let mut grid = vec![vec![0.0; cols]; rows];
    for row in 0..rows {
        let lat = bounds.max_lat - (row as f64 / (rows - 1).max(1) as f64) * (bounds.max_lat - bounds.min_lat);
        for col in 0..cols {
            let lon = bounds.min_lon + (col as f64 / (cols - 1).max(1) as f64) * (bounds.max_lon - bounds.min_lon);
            grid[row][col] = idw_interpolate(&cfg.lats[..n], &cfg.lons[..n], &cfg.field[..n], lat, lon);
        }
    }
    grid
}

pub fn grid_to_pixel(cfg: &ContourMapConfig, bounds: &FieldBounds, col: usize, cols: usize, row: usize, rows: usize) -> (f32, f32) {
    let lat = bounds.max_lat - (row as f64 / (rows - 1).max(1) as f64) * (bounds.max_lat - bounds.min_lat);
    let lon = bounds.min_lon + (col as f64 / (cols - 1).max(1) as f64) * (bounds.max_lon - bounds.min_lon);
    let (nx, ny) = world_data::latlon_to_normalized(lat, lon);
    (nx * cfg.width as f32, ny * cfg.height as f32)
}

pub fn to_html(cfg: &ContourMapConfig, mut svg: String) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        slots.push(
            HoverSlot::new(format!("p{i}"))
                .kv("Lat", format!("{:.2}", cfg.lats[i]))
                .kv("Lon", format!("{:.2}", cfg.lons[i]))
                .kv("Value", format!("{:.2}", cfg.field[i])),
        );
    }
    if !svg.ends_with("</svg>") {
        svg.push_str("</svg>");
    }
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}
