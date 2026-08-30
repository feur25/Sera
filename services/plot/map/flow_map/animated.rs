use super::common::resolve_edges;
use super::config::FlowMapConfig;
use crate::plot::map::regions;

#[crate::chart_demo(
    "labels=[\"US\",\"CN\",\"DE\",\"GB\",\"JP\",\"IN\",\"BR\",\"FR\",\"CA\",\"MX\",\"KR\",\"IT\",\"RU\",\"AU\",\"ES\",\"NL\",\"CH\",\"SA\",\"SG\",\"ZA\"], edges_i=[0,1,0,0,0,0,0,2,2,2,1,1,4,3,3,7,7,11,5,5,6,6,8,9,12,12,13,13,14,16,17,17,18,18,19,10,10], edges_j=[1,0,8,9,2,4,3,1,7,15,4,10,10,2,0,2,11,2,1,0,1,0,1,1,1,2,1,4,7,2,1,0,1,0,1,0,1], edges_w=[420,380,580,490,190,210,140,200,170,200,300,280,90,130,120,160,90,140,95,110,100,65,70,85,190,45,150,60,55,100,65,40,80,55,30,95,170], title=\"Trade Flow, animated direction\", variant=\"animated\""
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

    let mut svg = String::with_capacity(visible.len() * 400 + edges.len() * 400 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str(
        "\"><defs><style>.sp-flow-dash{stroke-dasharray:2 7;animation:sp-flow-run 1.4s linear infinite}\
@keyframes sp-flow-run{to{stroke-dashoffset:-18}}\
@media (prefers-reduced-motion: reduce){.sp-flow-dash{animation:none}}</style></defs>\
<rect width=\"100%\" height=\"100%\" fill=\"#0b0e18\"/>",
    );

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
        let mx = (e.x1 + e.x2) / 2.0;
        let my = (e.y1 + e.y2) / 2.0;
        let dx = e.x2 - e.x1;
        let dy = e.y2 - e.y1;
        let bow = 0.22;
        let cx = mx - dy * bow;
        let cy = my + dx * bow;
        svg.push_str(&format!(
            "<path d=\"M{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1}\" fill=\"none\" stroke=\"rgb({r},{g},{b})\" stroke-opacity=\"0.55\" stroke-width=\"{sw:.1}\" stroke-linecap=\"round\"/>",
            e.x1, e.y1, cx, cy, e.x2, e.y2
        ));
        svg.push_str(&format!(
            "<path class=\"sp-flow-dash\" d=\"M{:.1},{:.1} Q{:.1},{:.1} {:.1},{:.1}\" fill=\"none\" stroke=\"white\" stroke-opacity=\"0.9\" stroke-width=\"{:.1}\" stroke-linecap=\"round\" data-index=\"{i}\"/>",
            e.x1, e.y1, cx, cy, e.x2, e.y2, (sw * 0.4).max(1.0)
        ));
        svg.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"rgb({r},{g},{b})\"/>", e.x1, e.y1));
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
