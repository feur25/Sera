use super::config::BubbleMapConfig;
use crate::plot::map::world_data;
use std::collections::HashMap;

const HEX_SIZE: f64 = 13.0;
const SQRT3: f64 = 1.7320508075688772;

fn pixel_to_axial(px: f64, py: f64) -> (f64, f64) {
    let q = (SQRT3 / 3.0 * px - 1.0 / 3.0 * py) / HEX_SIZE;
    let r = (2.0 / 3.0 * py) / HEX_SIZE;
    (q, r)
}

fn round_axial(q: f64, r: f64) -> (i32, i32) {
    let x = q;
    let z = r;
    let y = -x - z;
    let mut rx = x.round();
    let mut ry = y.round();
    let rz = z.round();
    let dx = (rx - x).abs();
    let dy = (ry - y).abs();
    let dz = (rz - z).abs();
    if dx > dy && dx > dz {
        rx = -ry - rz;
    } else if dy > dz {
        ry = -rx - rz;
    }
    (rx as i32, rz as i32)
}

fn axial_to_pixel(q: i32, r: i32) -> (f64, f64) {
    let x = HEX_SIZE * (SQRT3 * q as f64 + SQRT3 / 2.0 * r as f64);
    let y = HEX_SIZE * (1.5 * r as f64);
    (x, y)
}

fn hexagon_points(cx: f64, cy: f64, r: f64) -> String {
    let mut pts = String::new();
    for i in 0..6 {
        let angle = std::f64::consts::PI / 180.0 * (60.0 * i as f64 - 30.0);
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        if i > 0 {
            pts.push(' ');
        }
        pts.push_str(&format!("{:.1},{:.1}", x, y));
    }
    pts
}

#[crate::chart_demo(
    "lats=[40.7,40.75,40.72,40.68,41.0,34.0,34.05,34.1,33.9,37.7,37.75,37.8,29.7,29.75], lons=[-74.0,-73.95,-74.05,-73.9,-73.8,-118.2,-118.25,-118.15,-118.3,-122.4,-122.45,-122.35,-95.3,-95.35], title=\"Sampled Event Density\", variant=\"hexbin\""
)]
pub fn render(cfg: &BubbleMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len());
    if n == 0 {
        return String::new();
    }

    let mut counts: HashMap<(i32, i32), u32> = HashMap::new();
    for i in 0..n {
        let (nx, ny) = world_data::latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
        let px = nx as f64 * cfg.width as f64;
        let py = ny as f64 * cfg.height as f64;
        let (q, r) = pixel_to_axial(px, py);
        let cell = round_axial(q, r);
        *counts.entry(cell).or_insert(0) += 1;
    }
    let max_count = counts.values().cloned().max().unwrap_or(1).max(1);

    let mut svg = String::with_capacity(8192);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&cfg.width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&cfg.height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&cfg.width.to_string());
    svg.push(' ');
    svg.push_str(&cfg.height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0d1117\"/>");

    for shape in world_data::all_countries() {
        for poly in world_data::normalized_polygons(shape) {
            if poly.len() < 3 {
                continue;
            }
            svg.push_str("<path d=\"M");
            for (j, pt) in poly.iter().enumerate() {
                let px = pt[0] * cfg.width as f32;
                let py = pt[1] * cfg.height as f32;
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }

    let mut cells: Vec<(&(i32, i32), &u32)> = counts.iter().collect();
    cells.sort_by_key(|(k, _)| (k.1, k.0));
    for (idx, ((q, r), count)) in cells.into_iter().enumerate() {
        let (cx, cy) = axial_to_pixel(*q, *r);
        let t = (*count as f64 / max_count as f64).sqrt();
        let (red, green, blue) = super::common::PALETTE[0];
        let alpha = 0.25 + t * 0.65;
        let radius = HEX_SIZE * (0.55 + t * 0.42);
        svg.push_str(&format!(
            "<polygon points=\"{}\" fill=\"rgb({red},{green},{blue})\" fill-opacity=\"{alpha:.2}\" stroke=\"rgb({red},{green},{blue})\" stroke-width=\"0.8\" data-index=\"{idx}\"/>",
            hexagon_points(cx, cy, radius)
        ));
        if *count > 1 {
            svg.push_str(&format!(
                "<text x=\"{cx:.0}\" y=\"{cy:.0}\" fill=\"white\" font-size=\"9\" font-weight=\"700\" text-anchor=\"middle\" dominant-baseline=\"middle\">{count}</text>"
            ));
        }
    }

    svg.push_str("</svg>");
    crate::html::hover::build_chart_html(cfg.title, &svg, "[]")
}
