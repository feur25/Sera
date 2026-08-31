use super::config::FlowMapConfig;
use crate::plot::map::world_data;

const EARTH_RADIUS_KM: f64 = 6371.0;
const RING_STEPS: usize = 72;
const DEFAULT_RINGS: &[f64] = &[500.0, 1500.0, 3000.0];

fn ring_polygon(lat0_deg: f64, lon0_deg: f64, dist_km: f64, width: i32, height: i32) -> String {
    let lat0 = lat0_deg.to_radians();
    let lon0 = lon0_deg.to_radians();
    let delta = (dist_km / EARTH_RADIUS_KM).min(std::f64::consts::PI - 1e-6);
    let mut d = String::new();
    for step in 0..=RING_STEPS {
        let theta = (step as f64 / RING_STEPS as f64) * 2.0 * std::f64::consts::PI;
        let lat = (lat0.sin() * delta.cos() + lat0.cos() * delta.sin() * theta.cos()).asin();
        let lon = lon0 + (theta.sin() * delta.sin() * lat0.cos()).atan2(delta.cos() - lat0.sin() * lat.sin());
        let (nx, ny) = world_data::latlon_to_normalized(lat.to_degrees(), lon.to_degrees());
        let px = nx as f64 * width as f64;
        let py = ny as f64 * height as f64;
        d.push_str(if step == 0 { "M" } else { " L" });
        d.push_str(&format!("{:.1},{:.1}", px, py));
    }
    d.push_str(" Z");
    d
}

#[crate::chart_demo(
    "lats=[40.7,51.5], lons=[-74.0,-0.12], field=[500,1500,3000], title=\"Coverage Range Rings\", variant=\"range_rings\""
)]
pub fn render(cfg: &FlowMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len());
    if n == 0 {
        return String::new();
    }
    let rings: &[f64] = if cfg.track_values.is_empty() { DEFAULT_RINGS } else { cfg.track_values };
    let mut sorted_rings = rings.to_vec();
    sorted_rings.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let max_ring = sorted_rings.last().cloned().unwrap_or(1.0).max(1.0);

    let mut svg = String::with_capacity(n * sorted_rings.len() * 900 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&cfg.width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&cfg.height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&cfg.width.to_string());
    svg.push(' ');
    svg.push_str(&cfg.height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0b0e18\"/>");

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
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#242c3d\" stroke-width=\"0.3\"/>");
        }
    }

    let (red, green, blue) = super::common::PALETTE[0];
    for hub in 0..n {
        for (ring_idx, &dist) in sorted_rings.iter().enumerate() {
            if dist <= 0.0 {
                continue;
            }
            let d = ring_polygon(cfg.lats[hub], cfg.lons[hub], dist, cfg.width, cfg.height);
            let t = dist / max_ring;
            let opacity = 0.85 - t * 0.55;
            svg.push_str(&format!(
                "<path d=\"{d}\" fill=\"rgb({red},{green},{blue})\" fill-opacity=\"{:.3}\" stroke=\"rgb({red},{green},{blue})\" stroke-width=\"1.1\" stroke-opacity=\"0.9\" data-index=\"{hub}\" data-ring=\"{ring_idx}\"/>",
                (opacity * 0.14).max(0.02),
            ));
        }
    }

    for hub in 0..n {
        let (nx, ny) = world_data::latlon_to_normalized(cfg.lats[hub], cfg.lons[hub]);
        let px = nx as f64 * cfg.width as f64;
        let py = ny as f64 * cfg.height as f64;
        svg.push_str(&format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"4\" fill=\"rgb({red},{green},{blue})\" stroke=\"#0b0e18\" stroke-width=\"1.4\"/>"
        ));
        if let Some(label) = cfg.labels.get(hub) {
            svg.push_str(&format!(
                "<text x=\"{px:.1}\" y=\"{:.1}\" fill=\"#e2e8f0\" font-size=\"11\" font-weight=\"700\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\">{label}</text>",
                py - 10.0,
            ));
        }
    }

    svg.push_str("</svg>");
    crate::html::hover::build_chart_html(cfg.title, &svg, "[]")
}
