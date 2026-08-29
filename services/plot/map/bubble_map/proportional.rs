use super::common::{push_outlines, svg_open, to_html, visible_shapes, PALETTE};
use super::config::BubbleMapConfig;

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"IL\",\"PA\",\"OH\",\"WA\",\"GA\",\"NC\"], values=[38.9,30.5,19.6,22.6,12.6,12.9,11.8,7.8,11.0,10.8], title=\"Population by State (millions)\", map=\"usa_states\", variant=\"proportional\""
)]

pub fn render(cfg: &BubbleMapConfig) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }
    let shapes = visible_shapes(cfg);
    let visible_ids: std::collections::HashSet<&str> = shapes.iter().map(|s| s.id.as_str()).collect();

    let mut matched: Vec<(usize, f64, &crate::plot::map::svg_parser::CountryShape)> = Vec::new();
    for i in 0..n {
        if let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) {
            if visible_ids.contains(shape.id.as_str()) {
                matched.push((i, cfg.values[i], shape));
            }
        }
    }
    let max_val = matched.iter().map(|(_, v, _)| v.abs()).fold(0.0_f64, f64::max).max(1e-9);

    let mut svg = svg_open(cfg.width, cfg.height);
    push_outlines(&mut svg, cfg, &shapes);

    for &(i, value, shape) in &matched {
        let centroid = crate::plot::map::world_data::shape_centroid(shape);
        let cx = centroid[0] / cfg.region.svg_width * cfg.width as f32;
        let cy = centroid[1] / cfg.region.svg_height * cfg.height as f32;
        let t = (value.abs() / max_val).sqrt();
        let radius = cfg.min_bubble_size + t * (cfg.max_bubble_size - cfg.min_bubble_size);
        let (r, g, b) = PALETTE[i % PALETTE.len()];
        svg.push_str(&format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{radius:.1}\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.55\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"1.4\" data-index=\"{i}\"/>"
        ));
        svg.push_str(&format!(
            "<text x=\"{cx:.0}\" y=\"{cy:.0}\" fill=\"white\" font-size=\"8\" font-weight=\"700\" text-anchor=\"middle\" dominant-baseline=\"middle\">{}</text>",
            shape.id
        ));
    }

    to_html(cfg, svg)
}
