use super::common::{push_outlines, svg_open, to_html, visible_shapes, PALETTE};
use super::config::BubbleMapConfig;

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"IL\",\"PA\",\"OH\",\"WA\"], values=[38.9,30.5,19.6,22.6,12.6,12.9,11.8,7.8], title=\"Largest Metro Populations (millions)\", map=\"usa_states\""
)]

pub fn render(cfg: &BubbleMapConfig) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }
    let shapes = visible_shapes(cfg);
    let mut svg = svg_open(cfg.width, cfg.height);
    push_outlines(&mut svg, cfg, &shapes);

    for i in 0..n {
        let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) else {
            continue;
        };
        if !shapes.iter().any(|s| s.id == shape.id) {
            continue;
        }
        let (r, g, b) = PALETTE[i % PALETTE.len()];
        let polys = (cfg.region.normalize)(shape);
        for poly in &polys {
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
            svg.push_str(&format!(
                " Z\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.7\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"0.8\" data-index=\"{i}\"/>"
            ));
        }
        let centroid = crate::plot::map::world_data::shape_centroid(shape);
        let cx = centroid[0] / cfg.region.svg_width * cfg.width as f32;
        let cy = centroid[1] / cfg.region.svg_height * cfg.height as f32;
        svg.push_str(&format!(
            "<text x=\"{cx:.0}\" y=\"{cy:.0}\" fill=\"white\" font-size=\"8\" text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>",
            shape.id
        ));
    }

    to_html(cfg, svg)
}
