use super::config::ChoroplethConfig;
use crate::plot::map::world_data;

fn terminator_lat_deg(lon_deg: f64, sub_lat_deg: f64, sub_lon_deg: f64) -> f64 {
    let sub_lat = sub_lat_deg.to_radians();
    let h = (lon_deg - sub_lon_deg).to_radians();
    (-sub_lat.cos() * h.cos()).atan2(sub_lat.sin()).to_degrees()
}

#[crate::chart_demo(
    "title=\"Day / Night Terminator\", variant=\"daynight\", center_lat=15.0, center_lon=-40.0"
)]
pub fn render(cfg: &ChoroplethConfig) -> String {
    let width = cfg.width;
    let height = cfg.height;
    let sub_lat = cfg.center_lat.unwrap_or(0.0);
    let sub_lon = cfg.center_lon.unwrap_or(0.0);

    let mut svg = String::with_capacity(16_384);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0a1a2e\"/>");

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
            svg.push_str(" Z\" fill=\"#e2c88a\" stroke=\"#7a6a4a\" stroke-width=\"0.4\"/>");
        }
    }

    const LON_MIN: f64 = -169.110266;
    const LON_MAX: f64 = 190.486279;
    const STEPS: usize = 240;
    let north_is_night = sub_lat.to_radians().sin() < 0.0;

    let mut path = String::from("M");
    for i in 0..=STEPS {
        let lon = LON_MIN + (i as f64 / STEPS as f64) * (LON_MAX - LON_MIN);
        let lat = terminator_lat_deg(lon, sub_lat, sub_lon).clamp(-85.0, 85.0);
        let (nx, ny) = world_data::latlon_to_normalized(lat, lon);
        let px = nx * width as f32;
        let py = ny * height as f32;
        path.push_str(&format!("{:.1},{:.1} L", px, py));
    }
    let pole_lat = if north_is_night { 85.0 } else { -85.0 };
    for lon in [LON_MAX, LON_MIN] {
        let (nx, ny) = world_data::latlon_to_normalized(pole_lat, lon);
        path.push_str(&format!("{:.1},{:.1} L", nx * width as f32, ny * height as f32));
    }
    path.push_str("Z");

    svg.push_str(&format!(
        "<path d=\"{path}\" fill=\"#0a1128\" fill-opacity=\"0.62\" stroke=\"none\"/>"
    ));

    let (snx, sny) = world_data::latlon_to_normalized(sub_lat, sub_lon);
    let spx = snx * width as f32;
    let spy = sny * height as f32;
    svg.push_str(&format!(
        "<circle cx=\"{spx:.1}\" cy=\"{spy:.1}\" r=\"16\" fill=\"#facc15\" opacity=\"0.25\"/>\
         <circle cx=\"{spx:.1}\" cy=\"{spy:.1}\" r=\"7\" fill=\"#facc15\" stroke=\"#fff7cc\" stroke-width=\"1.5\"/>"
    ));

    svg.push_str("</svg>");
    crate::html::hover::build_chart_html(cfg.title, &svg, "[]")
}
