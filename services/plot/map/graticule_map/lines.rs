use super::common::{lat_steps, lon_steps, push_base_outlines, svg_open, to_html};
use super::config::GraticuleMapConfig;
use crate::plot::map::world_data::{self, GEO_MAX_LAT, GEO_MAX_LON, GEO_MIN_LAT, GEO_MIN_LON};

fn push_line(svg: &mut String, a: (f32, f32), b: (f32, f32), width: f32, opacity: f64) {
    svg.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#3b82f6\" stroke-width=\"{width}\" opacity=\"{opacity:.2}\"/>",
        a.0, a.1, b.0, b.1
    ));
}

#[crate::chart_demo("title=\"World Graticule\", step=15")]
pub fn render(cfg: &GraticuleMapConfig) -> String {
    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    let step = cfg.step.max(1.0);
    let w = cfg.width as f32;
    let h = cfg.height as f32;

    for lon in lon_steps(step) {
        let top = world_data::latlon_to_normalized(GEO_MAX_LAT, lon);
        let bottom = world_data::latlon_to_normalized(GEO_MIN_LAT, lon);
        let emphasize = lon.abs() < 1e-6;
        push_line(&mut svg, (top.0 * w, top.1 * h), (bottom.0 * w, bottom.1 * h), if emphasize { 1.4 } else { 0.5 }, if emphasize { 0.55 } else { 0.22 });
        let label_y = top.1 * h + 12.0;
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{label_y:.1}\" fill=\"#64748b\" font-size=\"9\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\">{:.0}°</text>",
            top.0 * w, lon,
        ));
    }

    for lat in lat_steps(step) {
        let left = world_data::latlon_to_normalized(lat, GEO_MIN_LON);
        let right = world_data::latlon_to_normalized(lat, GEO_MAX_LON);
        let emphasize = lat.abs() < 1e-6;
        push_line(&mut svg, (left.0 * w, left.1 * h), (right.0 * w, right.1 * h), if emphasize { 1.4 } else { 0.5 }, if emphasize { 0.55 } else { 0.22 });
        svg.push_str(&format!(
            "<text x=\"4\" y=\"{:.1}\" fill=\"#64748b\" font-size=\"9\" font-family=\"Arial,sans-serif\">{:.0}°</text>",
            left.1 * h - 3.0, lat,
        ));
    }

    to_html(cfg, svg, "[]")
}
