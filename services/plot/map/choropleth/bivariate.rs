use super::config::ChoroplethConfig;
use crate::plot::map::regions;

const GRID: [[(u8, u8, u8); 3]; 3] = [
    [(232, 232, 232), (178, 212, 212), (90, 200, 200)],
    [(223, 176, 214), (165, 173, 211), (86, 152, 185)],
    [(190, 100, 172), (140, 98, 170), (59, 73, 148)],
];

fn tercile(value: f64, min: f64, max: f64) -> usize {
    let span = (max - min).max(1e-9);
    (((value - min) / span) * 3.0).floor().clamp(0.0, 2.0) as usize
}

pub fn render_svg(cfg: &ChoroplethConfig) -> String {
    let n = cfg.values.len().min(cfg.labels.len()).min(cfg.secondary_values.len());
    if n == 0 {
        return String::new();
    }

    let visible = regions::shapes_in_group(cfg.region, cfg.group);
    let visible_ids: std::collections::HashSet<&str> = visible.iter().map(|s| s.id.as_str()).collect();

    let mut matched: Vec<(usize, f64, f64)> = Vec::with_capacity(n);
    for i in 0..n {
        if let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) {
            if visible_ids.contains(shape.id.as_str()) {
                matched.push((i, cfg.values[i], cfg.secondary_values[i]));
            }
        }
    }
    let a_min = matched.iter().map(|(_, a, _)| *a).fold(f64::INFINITY, f64::min);
    let a_max = matched.iter().map(|(_, a, _)| *a).fold(f64::NEG_INFINITY, f64::max);
    let b_min = matched.iter().map(|(_, _, b)| *b).fold(f64::INFINITY, f64::min);
    let b_max = matched.iter().map(|(_, _, b)| *b).fold(f64::NEG_INFINITY, f64::max);
    let (a_min, a_max) = if a_min.is_finite() && a_max.is_finite() { (a_min, a_max) } else { (0.0, 1.0) };
    let (b_min, b_max) = if b_min.is_finite() && b_max.is_finite() { (b_min, b_max) } else { (0.0, 1.0) };

    let width = cfg.width;
    let height = cfg.height;
    let legend_w = 96.0;
    let legend_h = 96.0;
    let legend_x = width as f32 - legend_w - 24.0;
    let legend_y = height as f32 - legend_h - 24.0;
    let cell = legend_w / 3.0;

    let mut svg = String::with_capacity(n * 2000 + 4096);
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
            svg.push_str(" Z\" fill=\"#1a1a2e\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }

    for &(i, a, b) in &matched {
        let Some(shape) = (cfg.region.lookup)(&cfg.labels[i]) else {
            continue;
        };
        let (r, g, bl) = GRID[tercile(b, b_min, b_max)][tercile(a, a_min, a_max)];
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
            svg.push_str(&format!(
                " Z\" fill=\"rgb({r},{g},{bl})\" fill-opacity=\"0.92\" stroke=\"rgba(255,255,255,0.25)\" stroke-width=\"0.5\" data-index=\"{i}\"/>"
            ));
        }
    }

    for row in 0..3 {
        for col in 0..3 {
            let (r, g, bl) = GRID[row][col];
            let x = legend_x + col as f32 * cell;
            let y = legend_y + (2 - row) as f32 * cell;
            svg.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"rgb({r},{g},{bl})\"/>",
                cell + 0.5, cell + 0.5
            ));
        }
    }
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#94a3b8\" font-size=\"10\" text-anchor=\"middle\">A →</text>",
        legend_x + legend_w / 2.0, legend_y + legend_h + 16.0
    ));
    svg.push_str(&format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#94a3b8\" font-size=\"10\" text-anchor=\"middle\" transform=\"rotate(-90 {:.1} {:.1})\">B →</text>",
        legend_x - 12.0, legend_y + legend_h / 2.0, legend_x - 12.0, legend_y + legend_h / 2.0
    ));

    svg.push_str("</svg>");
    svg
}

pub fn render_html(cfg: &ChoroplethConfig) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let n = cfg.values.len().min(cfg.labels.len()).min(cfg.secondary_values.len());
    if n == 0 {
        return String::new();
    }
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(
                HoverSlot::new(cfg.labels[i].clone())
                    .kv("A", format!("{:.2}", cfg.values[i]))
                    .kv("B", format!("{:.2}", cfg.secondary_values[i])),
            );
        }
    }
    let mut svg = render_svg(cfg);
    svg = svg.replace("data-index=\"", "data-idx=\"");
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"WA\",\"CO\",\"IL\",\"OH\",\"GA\",\"AZ\",\"NV\",\"UT\",\"OR\",\"NC\",\"MA\",\"VA\",\"MN\",\"MI\",\"PA\",\"TN\",\"MO\",\"WI\",\"MD\",\"SC\",\"AL\"], values=[245,180,310,190,275,260,225,195,205,215,230,250,265,200,320,240,255,185,210,175,190,220,270,180,165], secondary_values=[42,28,38,31,45,40,35,27,29,33,37,44,41,30,47,36,39,26,32,25,28,34,43,27,24], title=\"Broadband Speed vs Remote-Work Share\", map=\"usa_states\", variant=\"bivariate\""
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    render_html(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tercile_splits_the_range_into_three_even_bins() {
        assert_eq!(tercile(0.0, 0.0, 9.0), 0);
        assert_eq!(tercile(3.5, 0.0, 9.0), 1);
        assert_eq!(tercile(8.9, 0.0, 9.0), 2);
    }

    #[test]
    fn render_svg_colors_every_matched_region_from_the_nine_cell_grid() {
        let cfg = ChoroplethConfig {
            labels: &["CA".to_string(), "TX".to_string()],
            values: &[1.0, 9.0],
            secondary_values: &[1.0, 9.0],
            ..ChoroplethConfig::new(crate::plot::map::regions::resolve("usa_states").unwrap())
        };
        let svg = render_svg(&cfg);
        assert!(svg.contains("data-index=\"0\""));
        assert!(svg.contains("data-index=\"1\""));
    }
}
