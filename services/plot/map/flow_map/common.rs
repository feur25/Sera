use super::config::FlowMapConfig;
use crate::plot::map::regions;

pub const PALETTE: &[(u8, u8, u8)] = &[
    (99, 102, 241),
    (244, 63, 94),
    (16, 185, 129),
    (245, 158, 11),
    (139, 92, 246),
    (6, 182, 212),
];

struct Edge {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    weight: f64,
    color: (u8, u8, u8),
}

fn resolve_edges(cfg: &FlowMapConfig) -> Vec<Edge> {
    let n = cfg.sources.len().min(cfg.targets.len());
    let n = n.min(cfg.weights.len());
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let (Some(src_i), Some(dst_i)) = (usize::try_from(cfg.sources[i]).ok(), usize::try_from(cfg.targets[i]).ok()) else {
            continue;
        };
        let (Some(src_label), Some(dst_label)) = (cfg.labels.get(src_i), cfg.labels.get(dst_i)) else {
            continue;
        };
        let (Some(src_shape), Some(dst_shape)) = ((cfg.region.lookup)(src_label), (cfg.region.lookup)(dst_label)) else {
            continue;
        };
        let p1 = regions::centroid_of(cfg.region, src_shape);
        let p2 = regions::centroid_of(cfg.region, dst_shape);
        out.push(Edge {
            x1: p1[0] * cfg.width as f32,
            y1: p1[1] * cfg.height as f32,
            x2: p2[0] * cfg.width as f32,
            y2: p2[1] * cfg.height as f32,
            weight: cfg.weights[i],
            color: PALETTE[i % PALETTE.len()],
        });
    }
    out
}

pub fn render_svg(cfg: &FlowMapConfig, curved: bool) -> String {
    let width = cfg.width;
    let height = cfg.height;
    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let edges = resolve_edges(cfg);
    let max_w = edges.iter().fold(0.0_f64, |m, e| m.max(e.weight.abs())).max(1e-9);

    let mut svg = String::with_capacity(visible.len() * 400 + edges.len() * 300 + 4096);
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
        let sw = cfg.min_width + t * (cfg.max_width - cfg.min_width);
        let (r, g, b) = e.color;
        let d = if curved {
            let mx = (e.x1 + e.x2) / 2.0;
            let my = (e.y1 + e.y2) / 2.0;
            let dx = e.x2 - e.x1;
            let dy = e.y2 - e.y1;
            let bow = 0.22;
            let cx = mx - dy * bow;
            let cy = my + dx * bow;
            format!("M{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1}", e.x1, e.y1, cx, cy, e.x2, e.y2)
        } else {
            format!("M{:.1},{:.1} L{:.1},{:.1}", e.x1, e.y1, e.x2, e.y2)
        };
        svg.push_str(&format!(
            "<path d=\"{d}\" fill=\"none\" stroke=\"rgb({r},{g},{b})\" stroke-opacity=\"0.75\" stroke-width=\"{sw:.1}\" stroke-linecap=\"round\" data-index=\"{i}\"/>"
        ));
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"rgb({r},{g},{b})\"/>",
            e.x1, e.y1
        ));
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"rgb({r},{g},{b})\" stroke=\"white\" stroke-width=\"1\"/>",
            e.x2, e.y2
        ));
    }

    svg.push_str("</svg>");
    svg
}

pub fn render_html(cfg: &FlowMapConfig, curved: bool) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let edges = resolve_edges(cfg);
    if edges.is_empty() {
        return String::new();
    }
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(edges.len()) } else { Vec::new() };
    if auto {
        let n = cfg.sources.len().min(cfg.targets.len()).min(cfg.weights.len());
        for i in 0..n {
            let (Some(&si), Some(&ti)) = (cfg.sources.get(i), cfg.targets.get(i)) else {
                continue;
            };
            let (Some(src), Some(dst)) = (usize::try_from(si).ok().and_then(|x| cfg.labels.get(x)), usize::try_from(ti).ok().and_then(|x| cfg.labels.get(x))) else {
                continue;
            };
            auto_slots.push(HoverSlot::new(format!("{src} \u{2192} {dst}")).kv("Flux", format!("{:.2}", cfg.weights[i])));
        }
    }
    let mut svg = render_svg(cfg, curved);
    svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
