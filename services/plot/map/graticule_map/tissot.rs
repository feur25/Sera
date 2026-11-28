use super::common::{destination_point, lerp_rgb, push_base_outlines, svg_open, to_html};
use super::config::GraticuleMapConfig;
use crate::plot::map::world_data;

const RADIUS_DEG: f64 = 5.0;
const RING_STEPS: usize = 24;

fn indicatrix_path(lat: f64, lon: f64, width: i32, height: i32) -> String {
    let mut d = String::new();
    for step in 0..=RING_STEPS {
        let bearing = (step as f64 / RING_STEPS as f64) * 360.0;
        let (plat, plon) = destination_point(lat, lon, RADIUS_DEG, bearing);
        let (nx, ny) = world_data::latlon_to_normalized(plat, plon);
        let px = nx as f64 * width as f64;
        let py = ny as f64 * height as f64;
        d.push_str(if step == 0 { "M" } else { " L" });
        d.push_str(&format!("{:.1},{:.1}", px, py));
    }
    d.push_str(" Z");
    d
}

#[crate::chart_demo("title=\"Tissot's Indicatrix -- This Map's Own Distortion\", variant=\"tissot\", step=30")]
pub fn render(cfg: &GraticuleMapConfig) -> String {
    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    let step = cfg.step.max(10.0);
    let mut idx = 0;
    let mut lat = -60.0;
    while lat <= 60.0 {
        let mut lon = -150.0;
        while lon <= 150.0 {
            let path = indicatrix_path(lat, lon, cfg.width, cfg.height);
            let t = (lat.abs() / 90.0).clamp(0.0, 1.0);
            let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, t);
            svg.push_str(&format!(
                "<path d=\"{path}\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.35\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"1\" data-index=\"{idx}\"/>"
            ));
            idx += 1;
            lon += step;
        }
        lat += step;
    }

    to_html(cfg, svg, "[]")
}
