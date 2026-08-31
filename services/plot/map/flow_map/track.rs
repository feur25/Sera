use super::config::FlowMapConfig;
use crate::plot::map::world_data;

#[crate::chart_demo(
    "lats=[12.1,13.4,15.2,17.8,20.5,23.1,26.0,29.4], lons=[-45.2,-48.6,-52.1,-56.0,-60.4,-65.2,-70.8,-76.5], field=[35,45,60,75,95,110,90,60], title=\"Sampled Storm Track\", variant=\"track\""
)]
pub fn render(cfg: &FlowMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len());
    if n < 2 {
        return String::new();
    }
    let values = if cfg.track_values.len() >= n { cfg.track_values } else { &[] };
    let max_val = values.iter().cloned().fold(1e-9_f64, f64::max);

    let mut svg = String::with_capacity(16_384);
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

    let points: Vec<(f32, f32)> = (0..n)
        .map(|i| {
            let (nx, ny) = world_data::latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
            (nx * cfg.width as f32, ny * cfg.height as f32)
        })
        .collect();

    let mut d = String::new();
    for (i, &(px, py)) in points.iter().enumerate() {
        d.push_str(if i == 0 { "M" } else { " L" });
        d.push_str(&format!("{:.1},{:.1}", px, py));
    }
    svg.push_str(&format!(
        "<path d=\"{d}\" fill=\"none\" stroke=\"#f59e0b\" stroke-width=\"2\" stroke-opacity=\"0.85\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>"
    ));

    for (i, &(px, py)) in points.iter().enumerate() {
        let t = if values.is_empty() { 0.5 } else { values[i] / max_val };
        let radius = 4.0 + t * 10.0;
        let (r, g, b) = super::common::PALETTE[0];
        let color = if t > 0.66 {
            "#ef4444".to_string()
        } else if t > 0.33 {
            "#f59e0b".to_string()
        } else {
            format!("rgb({r},{g},{b})")
        };
        svg.push_str(&format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{radius:.1}\" fill=\"{color}\" fill-opacity=\"0.85\" stroke=\"#0b0e18\" stroke-width=\"1.4\" data-index=\"{i}\"/>"
        ));
    }

    if points.len() >= 2 {
        let last = points[points.len() - 1];
        let prev = points[points.len() - 2];
        let ang = (last.1 - prev.1).atan2(last.0 - prev.0);
        let hx1 = last.0 - 10.0 * (ang - 0.4).cos();
        let hy1 = last.1 - 10.0 * (ang - 0.4).sin();
        let hx2 = last.0 - 10.0 * (ang + 0.4).cos();
        let hy2 = last.1 - 10.0 * (ang + 0.4).sin();
        svg.push_str(&format!(
            "<polygon points=\"{:.1},{:.1} {hx1:.1},{hy1:.1} {hx2:.1},{hy2:.1}\" fill=\"#f59e0b\"/>",
            last.0, last.1,
        ));
    }

    svg.push_str("</svg>");
    crate::html::hover::build_chart_html(cfg.title, &svg, "[]")
}
