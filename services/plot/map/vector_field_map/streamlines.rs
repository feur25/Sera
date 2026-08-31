use super::common::{field_bounds, idw_uv, lerp_rgb, project, push_base_outlines, svg_open, to_html, FieldBounds};
use super::config::VectorFieldMapConfig;

const SEED_COLS: usize = 12;
const SEED_ROWS: usize = 8;
const MAX_STEPS: usize = 60;

fn trace(cfg: &VectorFieldMapConfig, bounds: &FieldBounds, start_lat: f64, start_lon: f64) -> Vec<(f64, f64)> {
    let mut lat = start_lat;
    let mut lon = start_lon;
    let step = (bounds.max_lat - bounds.min_lat).max(bounds.max_lon - bounds.min_lon) / 240.0;
    let mut pts = vec![(lat, lon)];
    for _ in 0..MAX_STEPS {
        let (u, v) = idw_uv(cfg.lats, cfg.lons, cfg.u, cfg.v, lat, lon);
        let mag = (u * u + v * v).sqrt();
        if mag < 1e-6 {
            break;
        }
        lat += (v / mag) * step;
        lon += (u / mag) * step;
        if lat < bounds.min_lat || lat > bounds.max_lat || lon < bounds.min_lon || lon > bounds.max_lon {
            break;
        }
        pts.push((lat, lon));
    }
    pts
}

#[crate::chart_demo(
    "lats=[50,45,40,35,50,45,40,35,50,45,40,35], lons=[-10,-5,0,5,-10,-5,0,5,-10,-5,0,5], u=[8,5,-3,-9,4,7,2,-6,-2,-8,6,3], v=[3,-6,8,-2,-9,1,-7,5,6,-3,-4,9], title=\"Flow Streamlines Over Sampled Wind\", variant=\"streamlines\""
)]
pub fn render(cfg: &VectorFieldMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.u.len()).min(cfg.v.len());
    if n == 0 {
        return String::new();
    }
    let bounds = field_bounds(cfg);
    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    for row in 0..SEED_ROWS {
        for col in 0..SEED_COLS {
            let lat = bounds.min_lat + (row as f64 + 0.5) / SEED_ROWS as f64 * (bounds.max_lat - bounds.min_lat);
            let lon = bounds.min_lon + (col as f64 + 0.5) / SEED_COLS as f64 * (bounds.max_lon - bounds.min_lon);
            let path = trace(cfg, &bounds, lat, lon);
            if path.len() < 4 {
                continue;
            }
            let (u0, v0) = idw_uv(cfg.lats, cfg.lons, cfg.u, cfg.v, lat, lon);
            let t = (u0 * u0 + v0 * v0).sqrt() / bounds.max_mag;
            let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, t);

            let mut d = String::new();
            for (i, &(plat, plon)) in path.iter().enumerate() {
                let (px, py) = project(plat, plon, cfg.width, cfg.height);
                d.push_str(if i == 0 { "M" } else { " L" });
                d.push_str(&format!("{:.1},{:.1}", px, py));
            }
            svg.push_str(&format!(
                "<path d=\"{d}\" fill=\"none\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"1.3\" opacity=\"0.78\" stroke-linecap=\"round\"/>"
            ));

            let last = path[path.len() - 1];
            let prev = path[path.len() - 2];
            let (lx, ly) = project(last.0, last.1, cfg.width, cfg.height);
            let (pxp, pyp) = project(prev.0, prev.1, cfg.width, cfg.height);
            let ang = (ly - pyp).atan2(lx - pxp);
            let hx1 = lx - 6.0 * (ang - 0.4).cos();
            let hy1 = ly - 6.0 * (ang - 0.4).sin();
            let hx2 = lx - 6.0 * (ang + 0.4).cos();
            let hy2 = ly - 6.0 * (ang + 0.4).sin();
            svg.push_str(&format!(
                "<polygon points=\"{lx:.1},{ly:.1} {hx1:.1},{hy1:.1} {hx2:.1},{hy2:.1}\" fill=\"rgb({r},{g},{b})\" opacity=\"0.9\"/>"
            ));
        }
    }

    for i in 0..n {
        let (px, py) = project(cfg.lats[i], cfg.lons[i], cfg.width, cfg.height);
        svg.push_str(&format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"2.6\" fill=\"#ffffff\" stroke=\"#0d1117\" stroke-width=\"0.8\" data-index=\"{i}\"/>"
        ));
    }

    to_html(cfg, svg)
}
