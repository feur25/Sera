use super::config::ChoroplethConfig;
use crate::plot::map::regions;
use crate::plot::map::world_data::point_in_polygon;

const PALETTE: &[(u8, u8, u8)] = &[
    (99, 102, 241), (244, 63, 94), (16, 185, 129), (245, 158, 11),
    (139, 92, 246), (6, 182, 212), (236, 72, 153), (132, 204, 22),
];

fn bounds_of(polys: &[Vec<[f32; 2]>]) -> Option<(f32, f32, f32, f32)> {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for poly in polys {
        for p in poly {
            min_x = min_x.min(p[0]);
            max_x = max_x.max(p[0]);
            min_y = min_y.min(p[1]);
            max_y = max_y.max(p[1]);
        }
    }
    (max_x > min_x && max_y > min_y).then_some((min_x, min_y, max_x, max_y))
}

fn scatter(polys: &[Vec<[f32; 2]>], count: usize, seed: u32) -> Vec<[f32; 2]> {
    let Some((min_x, min_y, max_x, max_y)) = bounds_of(polys) else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(count);
    let mut state = seed.wrapping_mul(2654435761).wrapping_add(1);
    let max_tries = count.saturating_mul(40).saturating_add(200);
    let mut tries = 0;
    while out.len() < count && tries < max_tries {
        tries += 1;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let rx = ((state >> 8) & 0x00FF_FFFF) as f32 / 16_777_216.0;
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let ry = ((state >> 8) & 0x00FF_FFFF) as f32 / 16_777_216.0;
        let px = min_x + rx * (max_x - min_x);
        let py = min_y + ry * (max_y - min_y);
        if polys.iter().any(|poly| point_in_polygon(px, py, poly)) {
            out.push([px, py]);
        }
    }
    out
}

pub fn render_svg(cfg: &ChoroplethConfig, dot_value: f64) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }

    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let visible_ids: std::collections::HashSet<&str> = visible.iter().map(|s| s.id.as_str()).collect();

    let mut matched: Vec<(usize, f64)> = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) {
            if visible_ids.contains(shape.id.as_str()) {
                matched.push((i, cfg.values[i]));
            }
        }
    }

    let width = cfg.width;
    let height = cfg.height;
    let mut svg = String::with_capacity(n * 4000 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" fill=\"#0f0f1e\"/>");

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
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }

    let dot_value = dot_value.max(1e-6);
    for &(i, value) in &matched {
        let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) else {
            continue;
        };
        let polys = (cfg.region.normalize)(shape);
        let scaled: Vec<Vec<[f32; 2]>> = polys
            .iter()
            .map(|poly| poly.iter().map(|[x, y]| [x * width as f32, y * height as f32]).collect())
            .collect();
        let count = ((value / dot_value).round() as usize).clamp(1, 400);
        let dots = scatter(&scaled, count, (i as u32).wrapping_mul(747796405).wrapping_add(2891336453));
        let (r, g, b) = PALETTE[i % PALETTE.len()];
        for d in &dots {
            svg.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"1.4\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.85\" data-idx=\"{i}\"/>",
                d[0], d[1]
            ));
        }
    }

    svg.push_str("</svg>");
    svg
}

pub fn render_html(cfg: &ChoroplethConfig, dot_value: f64) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.values.len().min(cfg.labels.len());
    if n == 0 {
        return String::new();
    }
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(HoverSlot::new(cfg.labels[i].clone()).kv("Valeur", format!("{:.2}", cfg.values[i])));
        }
    }
    let svg = render_svg(cfg, dot_value);
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"WA\",\"CO\",\"IL\",\"OH\",\"GA\",\"AZ\",\"NV\",\"UT\",\"OR\",\"NC\",\"MA\",\"VA\",\"MN\",\"MI\",\"PA\",\"TN\"], values=[38900,30500,19600,22600,7800,5900,12600,11800,11000,7400,3200,3400,4200,10800,7000,8700,5700,10000,12900,7100], title=\"Population, one dot per 400 people\", map=\"usa_states\", variant=\"dot_density\""
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    render_html(cfg, 400.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scatter_keeps_every_dot_inside_the_source_polygon() {
        let square = vec![vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]];
        let dots = scatter(&square, 50, 7);
        assert_eq!(dots.len(), 50);
        for d in &dots {
            assert!(point_in_polygon(d[0], d[1], &square[0]));
        }
    }

    #[test]
    fn scatter_on_a_degenerate_polygon_returns_no_dots_instead_of_looping_forever() {
        let point = vec![vec![[5.0, 5.0], [5.0, 5.0], [5.0, 5.0]]];
        let dots = scatter(&point, 20, 1);
        assert!(dots.is_empty());
    }
}
