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

pub fn edge_crossing(v0: f64, v1: f64, p0: (f32, f32), p1: (f32, f32), t: f64) -> Option<(f32, f32)> {
    if (v0 - t) * (v1 - t) > 0.0 {
        return None;
    }
    if (v1 - v0).abs() < 1e-12 {
        return None;
    }
    let frac = ((t - v0) / (v1 - v0)) as f32;
    Some((p0.0 + (p1.0 - p0.0) * frac, p0.1 + (p1.1 - p0.1) * frac))
}

pub fn draw_isoline_paths(
    svg: &mut String,
    grid: &[Vec<f64>],
    pixels: &[Vec<(f32, f32)>],
    bounds: &FieldBounds,
    levels: usize,
    color_low: u32,
    color_high: u32,
    stroke_width: f64,
) {
    let span = (bounds.max_val - bounds.min_val).max(1e-9);
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let levels = levels.max(2);
    for level_idx in 1..levels {
        let t = bounds.min_val + span * (level_idx as f64 / levels as f64);
        let band_t = (t - bounds.min_val) / span;
        let (r, g, b) = lerp_rgb(color_low, color_high, band_t);
        let mut path = String::new();

        for row in 0..rows.saturating_sub(1) {
            for col in 0..cols.saturating_sub(1) {
                let tl = grid[row][col];
                let tr = grid[row][col + 1];
                let br = grid[row + 1][col + 1];
                let bl = grid[row + 1][col];
                let p_tl = pixels[row][col];
                let p_tr = pixels[row][col + 1];
                let p_br = pixels[row + 1][col + 1];
                let p_bl = pixels[row + 1][col];

                let top = edge_crossing(tl, tr, p_tl, p_tr, t);
                let right = edge_crossing(tr, br, p_tr, p_br, t);
                let bottom = edge_crossing(bl, br, p_bl, p_br, t);
                let left = edge_crossing(tl, bl, p_tl, p_bl, t);
                let found: Vec<(f32, f32)> = [top, right, bottom, left].into_iter().flatten().collect();

                let mut push_seg = |a: (f32, f32), c: (f32, f32)| {
                    path.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", a.0, a.1, c.0, c.1));
                };
                match found.len() {
                    2 => push_seg(found[0], found[1]),
                    4 => {
                        let center = (tl + tr + br + bl) / 4.0;
                        if center >= t {
                            push_seg(top.unwrap(), right.unwrap());
                            push_seg(bottom.unwrap(), left.unwrap());
                        } else {
                            push_seg(top.unwrap(), left.unwrap());
                            push_seg(right.unwrap(), bottom.unwrap());
                        }
                    }
                    _ => {}
                }
            }
        }

        if !path.is_empty() {
            svg.push_str(&format!(
                "<path d=\"{path}\" fill=\"none\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"{stroke_width}\" opacity=\"0.9\"/>"
            ));
        }
    }
}

pub fn find_extrema(grid: &[Vec<f64>], window: usize) -> Vec<(usize, usize, bool)> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let half = window / 2;
    let mut candidates: Vec<(usize, usize, bool, f64)> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let v = grid[r][c];
            let r0 = r.saturating_sub(half);
            let r1 = (r + half).min(rows.saturating_sub(1));
            let c0 = c.saturating_sub(half);
            let c1 = (c + half).min(cols.saturating_sub(1));
            let mut is_max = true;
            let mut is_min = true;
            for rr in r0..=r1 {
                for cc in c0..=c1 {
                    if rr == r && cc == c {
                        continue;
                    }
                    if grid[rr][cc] > v {
                        is_max = false;
                    }
                    if grid[rr][cc] < v {
                        is_min = false;
                    }
                }
            }
            if is_max {
                candidates.push((r, c, true, v));
            } else if is_min {
                candidates.push((r, c, false, v));
            }
        }
    }

    let mut out: Vec<(usize, usize, bool)> = Vec::new();
    for &(r, c, is_high, _) in &candidates {
        let too_close = out.iter().any(|&(or, oc, oh)| {
            oh == is_high && (or as isize - r as isize).abs() < window as isize && (oc as isize - c as isize).abs() < window as isize
        });
        if !too_close {
            out.push((r, c, is_high));
        }
    }
    out
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
