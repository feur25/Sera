use super::common::resolve_edges;
use super::config::FlowMapConfig;
use crate::plot::map::regions;

const SAMPLES: usize = 20;

fn bezier(p0: (f32, f32), c: (f32, f32), p1: (f32, f32), t: f32) -> (f32, f32) {
    let mt = 1.0 - t;
    (
        mt * mt * p0.0 + 2.0 * mt * t * c.0 + t * t * p1.0,
        mt * mt * p0.1 + 2.0 * mt * t * c.1 + t * t * p1.1,
    )
}

fn ribbon_path(p0: (f32, f32), c: (f32, f32), p1: (f32, f32), w_start: f32, w_end: f32) -> String {
    let mut top = Vec::with_capacity(SAMPLES + 1);
    let mut bottom = Vec::with_capacity(SAMPLES + 1);
    for i in 0..=SAMPLES {
        let t = i as f32 / SAMPLES as f32;
        let (x, y) = bezier(p0, c, p1, t);
        let eps = 0.001;
        let (xa, ya) = bezier(p0, c, p1, (t - eps).max(0.0));
        let (xb, yb) = bezier(p0, c, p1, (t + eps).min(1.0));
        let (dx, dy) = (xb - xa, yb - ya);
        let len = (dx * dx + dy * dy).sqrt().max(1e-6);
        let (nx, ny) = (-dy / len, dx / len);
        let half = (w_start + (w_end - w_start) * t) * 0.5;
        top.push((x + nx * half, y + ny * half));
        bottom.push((x - nx * half, y - ny * half));
    }
    let mut d = String::with_capacity(SAMPLES * 24);
    d.push('M');
    for (i, p) in top.iter().enumerate() {
        if i > 0 {
            d.push('L');
        }
        d.push_str(&format!("{:.1},{:.1} ", p.0, p.1));
    }
    for p in bottom.iter().rev() {
        d.push('L');
        d.push_str(&format!("{:.1},{:.1} ", p.0, p.1));
    }
    d.push('Z');
    d
}

#[crate::chart_demo(
    "labels=[\"US\",\"CN\",\"DE\",\"GB\",\"JP\",\"IN\",\"BR\",\"FR\",\"CA\",\"MX\",\"KR\",\"IT\",\"RU\",\"AU\",\"ES\",\"NL\",\"CH\",\"SA\",\"SG\",\"ZA\"], edges_i=[0,1,0,0,0,0,0,2,2,2,1,1,4,3,3,7,7,11,5,5,6,6,8,9,12,12,13,13,14,16,17,17,18,18,19,10,10], edges_j=[1,0,8,9,2,4,3,1,7,15,4,10,10,2,0,2,11,2,1,0,1,0,1,1,1,2,1,4,7,2,1,0,1,0,1,0,1], edges_w=[420,380,580,490,190,210,140,200,170,200,300,280,90,130,120,160,90,140,95,110,100,65,70,85,190,45,150,60,55,100,65,40,80,55,30,95,170], title=\"Trade Flow, tapered ribbons\", variant=\"ribbon\""
)]

pub fn render(cfg: &FlowMapConfig) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let edges = resolve_edges(cfg);
    if edges.is_empty() {
        return String::new();
    }

    let width = cfg.width;
    let height = cfg.height;
    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let max_w = edges.iter().fold(0.0_f64, |m, e| m.max(e.weight.abs())).max(1e-9);

    let mut svg = String::with_capacity(visible.len() * 400 + edges.len() * 800 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0b0e18\"/>");

    for shape in &visible {
        let polys = (cfg.region.normalize)(shape);
        for poly in &polys {
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
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#242c3d\" stroke-width=\"0.3\"/>");
        }
    }

    for (i, e) in edges.iter().enumerate() {
        let t = (e.weight.abs() / max_w).sqrt();
        let w_end = (cfg.min_width + t * (cfg.max_width - cfg.min_width)) as f32;
        let w_start = w_end * 2.4;
        let (r, g, b) = e.color;
        let mx = (e.x1 + e.x2) / 2.0;
        let my = (e.y1 + e.y2) / 2.0;
        let dx = e.x2 - e.x1;
        let dy = e.y2 - e.y1;
        let bow = 0.22;
        let cx = mx - dy * bow;
        let cy = my + dx * bow;
        let d = ribbon_path((e.x1, e.y1), (cx, cy), (e.x2, e.y2), w_start, w_end);
        svg.push_str(&format!(
            "<path d=\"{d}\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.55\" stroke=\"rgb({r},{g},{b})\" stroke-opacity=\"0.8\" stroke-width=\"0.6\" data-index=\"{i}\"/>"
        ));
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"rgb({r},{g},{b})\" stroke=\"white\" stroke-width=\"1\"/>",
            e.x2, e.y2
        ));
    }

    svg.push_str("</svg>");

    let auto_slots: Vec<HoverSlot> = if cfg.hover.is_empty() {
        let n = cfg.sources.len().min(cfg.targets.len()).min(cfg.weights.len());
        (0..n)
            .filter_map(|i| {
                let si = usize::try_from(*cfg.sources.get(i)?).ok()?;
                let ti = usize::try_from(*cfg.targets.get(i)?).ok()?;
                let src = cfg.labels.get(si)?;
                let dst = cfg.labels.get(ti)?;
                Some(HoverSlot::new(format!("{src} \u{2192} {dst}")).kv("Flux", format!("{:.2}", cfg.weights[i])))
            })
            .collect()
    } else {
        Vec::new()
    };
    let slots = if cfg.hover.is_empty() { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ribbon_path_starts_and_ends_with_a_move_and_close() {
        let d = ribbon_path((0.0, 0.0), (5.0, 5.0), (10.0, 0.0), 6.0, 2.0);
        assert!(d.starts_with('M'));
        assert!(d.trim_end().ends_with('Z'));
    }
}
