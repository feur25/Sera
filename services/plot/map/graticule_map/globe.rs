use super::common::{lat_steps, lon_steps, svg_open, to_html};
use super::config::GraticuleMapConfig;
use crate::plot::map::projections::{self, Projection};
use crate::plot::map::world_data;

fn sweep_lat(lon: f64, center_lat: f64, center_lon: f64) -> Vec<Vec<[f64; 2]>> {
    let mut segments: Vec<Vec<[f64; 2]>> = vec![Vec::new()];
    let mut lat = -90.0;
    while lat <= 90.0 {
        match Projection::Orthographic.project(lat, lon, center_lat, center_lon) {
            Some((x, y)) => segments.last_mut().unwrap().push([x, y]),
            None => {
                if !segments.last().unwrap().is_empty() {
                    segments.push(Vec::new());
                }
            }
        }
        lat += 2.0;
    }
    segments.into_iter().filter(|s| s.len() >= 2).collect()
}

fn sweep_lon(lat: f64, center_lat: f64, center_lon: f64) -> Vec<Vec<[f64; 2]>> {
    let mut segments: Vec<Vec<[f64; 2]>> = vec![Vec::new()];
    let mut lon = -180.0;
    while lon <= 180.0 {
        match Projection::Orthographic.project(lat, lon, center_lat, center_lon) {
            Some((x, y)) => segments.last_mut().unwrap().push([x, y]),
            None => {
                if !segments.last().unwrap().is_empty() {
                    segments.push(Vec::new());
                }
            }
        }
        lon += 2.0;
    }
    segments.into_iter().filter(|s| s.len() >= 2).collect()
}

fn build_raw_lines(step: f64, center_lat: f64, center_lon: f64) -> Vec<(usize, Vec<Vec<[f64; 2]>>)> {
    let mut out = Vec::new();
    let mut idx = 0usize;
    for lon in lon_steps(step) {
        let segs = sweep_lat(lon, center_lat, center_lon);
        if !segs.is_empty() {
            out.push((idx, segs));
            idx += 1;
        }
    }
    for lat in lat_steps(step) {
        let segs = sweep_lon(lat, center_lat, center_lon);
        if !segs.is_empty() {
            out.push((idx, segs));
            idx += 1;
        }
    }
    out
}

#[crate::chart_demo("title=\"Graticule on the Globe\", variant=\"globe\", center_lat=20, center_lon=10, step=15")]
pub fn render(cfg: &GraticuleMapConfig) -> String {
    let center_lat = cfg.center_lat.unwrap_or(20.0);
    let center_lon = cfg.center_lon.unwrap_or(10.0);
    let step = cfg.step.max(1.0);

    let shapes: Vec<&crate::plot::map::svg_parser::CountryShape> = world_data::all_countries().iter().collect();
    let raw_shapes = projections::project_shapes(&shapes, world_data::svg_to_latlon, Projection::Orthographic, center_lat, center_lon);
    let (transform, _) = projections::project_and_fit(&raw_shapes, cfg.width, cfg.height, 0.86);
    let disc = transform.disc();

    let mut svg = svg_open(cfg.width, cfg.height);
    svg.push_str(&format!(
        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#0d1117\" stroke=\"#334155\" stroke-width=\"1\"/>",
        disc.cx, disc.cy, disc.radius
    ));

    for (_, polys) in &raw_shapes {
        for poly in polys {
            if poly.len() < 3 {
                continue;
            }
            svg.push_str("<path d=\"M");
            for (j, &[x, y]) in poly.iter().enumerate() {
                let [px, py] = transform.apply(x, y);
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(" Z\" fill=\"#1e293b\" stroke=\"none\"/>");
        }
    }

    let raw_lines = build_raw_lines(step, center_lat, center_lon);
    for (idx, segs) in &raw_lines {
        let emphasize = *idx == 0;
        for seg in segs {
            svg.push_str("<path d=\"M");
            for (j, &[x, y]) in seg.iter().enumerate() {
                let [px, py] = transform.apply(x, y);
                if j > 0 {
                    svg.push_str(" L");
                }
                svg.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(&format!(
                "\" fill=\"none\" stroke=\"#f8fafc\" stroke-width=\"{}\" opacity=\"{}\"/>",
                if emphasize { 1.3 } else { 0.6 },
                if emphasize { 0.6 } else { 0.3 },
            ));
        }
    }

    to_html(cfg, svg, "[]")
}
