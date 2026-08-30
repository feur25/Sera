use super::common::{push_outlines, svg_open, to_html, visible_shapes, PALETTE};
use super::config::BubbleMapConfig;

#[crate::chart_demo(
    "labels=[\"AL\",\"AK\",\"AZ\",\"AR\",\"CA\",\"CO\",\"CT\",\"DE\",\"FL\",\"GA\",\"HI\",\"ID\",\"IL\",\"IN\",\"IA\",\"KS\",\"KY\",\"LA\",\"ME\",\"MD\",\"MA\",\"MI\",\"MN\",\"MS\",\"MO\",\"MT\",\"NE\",\"NV\",\"NH\",\"NJ\",\"NM\",\"NY\",\"NC\",\"ND\",\"OH\",\"OK\",\"OR\",\"PA\",\"RI\",\"SC\",\"SD\",\"TN\",\"TX\",\"UT\",\"VT\",\"VA\",\"WA\",\"WV\",\"WI\",\"WY\",\"DC\"], values=[5.1,0.73,7.4,3.0,38.9,5.9,3.6,1.0,22.6,11.0,1.4,2.0,12.6,6.8,3.2,2.9,4.5,4.6,1.4,6.2,7.0,10.0,5.7,2.9,6.2,1.1,2.0,3.2,1.4,9.3,2.1,19.6,10.8,0.78,11.8,4.0,4.2,12.9,1.1,5.4,0.92,7.1,30.5,3.4,0.65,8.7,7.8,1.8,5.9,0.58,0.68], title=\"Population by State (millions)\", map=\"usa_states\""
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
