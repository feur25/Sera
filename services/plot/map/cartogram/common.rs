use super::config::CartogramConfig;
use crate::plot::map::{regions, world_data};

#[derive(Clone, Copy)]
pub enum Glyph {
    Circle,
    Square,
}

pub fn lerp_rgb(low: u32, high: u32, t: f64) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    let lr = ((low >> 16) & 0xFF) as f64;
    let lg = ((low >> 8) & 0xFF) as f64;
    let lb = (low & 0xFF) as f64;
    let hr = ((high >> 16) & 0xFF) as f64;
    let hg = ((high >> 8) & 0xFF) as f64;
    let hb = (high & 0xFF) as f64;
    (
        (lr + (hr - lr) * t).round() as u8,
        (lg + (hg - lg) * t).round() as u8,
        (lb + (hb - lb) * t).round() as u8,
    )
}

struct Relaxed {
    x: Vec<f64>,
    y: Vec<f64>,
}

fn relax(true_x: &[f64], true_y: &[f64], radius: &[f64], iterations: u32) -> Relaxed {
    let n = true_x.len();
    let mut x = true_x.to_vec();
    let mut y = true_y.to_vec();
    let gravity = 0.02;
    let padding = 1.5;

    for _ in 0..iterations {
        let mut push_x = vec![0.0_f64; n];
        let mut push_y = vec![0.0_f64; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let ddx = x[j] - x[i];
                let ddy = y[j] - y[i];
                let dist = (ddx * ddx + ddy * ddy).sqrt().max(1e-6);
                let min_dist = radius[i] + radius[j] + padding;
                if dist < min_dist {
                    let overlap = (min_dist - dist) * 0.5;
                    let ux = ddx / dist;
                    let uy = ddy / dist;
                    push_x[i] -= ux * overlap;
                    push_y[i] -= uy * overlap;
                    push_x[j] += ux * overlap;
                    push_y[j] += uy * overlap;
                }
            }
        }
        for i in 0..n {
            x[i] += push_x[i];
            y[i] += push_y[i];
            x[i] += (true_x[i] - x[i]) * gravity;
            y[i] += (true_y[i] - y[i]) * gravity;
        }
    }
    Relaxed { x, y }
}

fn glyph_path(glyph: Glyph, cx: f64, cy: f64, r: f64) -> String {
    match glyph {
        Glyph::Circle => format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\""
        ),
        Glyph::Square => {
            let side = r * 1.772_453_85;
            format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{side:.1}\" height=\"{side:.1}\" rx=\"2\"",
                cx - side / 2.0,
                cy - side / 2.0,
            )
        }
    }
}

pub fn render(cfg: &CartogramConfig, glyph: Glyph) -> String {
    let n = cfg.labels.len().min(cfg.values.len()).min(cfg.lats.len()).min(cfg.lons.len());
    if n == 0 {
        return String::new();
    }

    let true_pts: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let (nx, ny) = world_data::latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
            (nx as f64 * cfg.width as f64, ny as f64 * cfg.height as f64)
        })
        .collect();
    let true_x: Vec<f64> = true_pts.iter().map(|p| p.0).collect();
    let true_y: Vec<f64> = true_pts.iter().map(|p| p.1).collect();

    let max_val = cfg.values[..n].iter().cloned().fold(1e-9_f64, f64::max);
    let min_val = cfg.values[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let span = (max_val - min_val).max(1e-9);
    let radius: Vec<f64> = (0..n)
        .map(|i| {
            let t = (cfg.values[i].max(0.0) / max_val).sqrt();
            cfg.min_radius + t * (cfg.max_radius - cfg.min_radius)
        })
        .collect();

    let relaxed = relax(&true_x, &true_y, &radius, cfg.iterations);

    let mut svg = String::with_capacity(n * 220 + 4096);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&cfg.width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&cfg.height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&cfg.width.to_string());
    svg.push(' ');
    svg.push_str(&cfg.height.to_string());
    svg.push_str("\"><rect width=\"100%\" height=\"100%\" class=\"sp-bg\"/>");

    for shape in regions::shapes_in_group(cfg.region, cfg.group) {
        for poly in (cfg.region.normalize)(shape) {
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
            svg.push_str(" Z\" fill=\"#141a24\" stroke=\"#242c3d\" stroke-width=\"0.3\"/>");
        }
    }

    for i in 0..n {
        let (tx, ty) = (true_x[i], true_y[i]);
        let (rx, ry) = (relaxed.x[i], relaxed.y[i]);
        let moved = ((rx - tx).powi(2) + (ry - ty).powi(2)).sqrt();
        if moved > radius[i] * 0.5 {
            svg.push_str(&format!(
                "<line x1=\"{tx:.1}\" y1=\"{ty:.1}\" x2=\"{rx:.1}\" y2=\"{ry:.1}\" stroke=\"#475569\" stroke-width=\"0.8\" stroke-dasharray=\"2,2\" opacity=\"0.55\"/>"
            ));
        }
    }

    for i in 0..n {
        let t = (cfg.values[i] - min_val) / span;
        let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, t);
        svg.push_str(&glyph_path(glyph, relaxed.x[i], relaxed.y[i], radius[i]));
        svg.push_str(&format!(
            " fill=\"rgb({r},{g},{b})\" fill-opacity=\"0.88\" stroke=\"#0d1117\" stroke-width=\"1.2\" data-index=\"{i}\"/>"
        ));
        if radius[i] > 15.0 {
            let font = (radius[i] * 0.34).clamp(8.0, 14.0);
            svg.push_str(&format!(
                "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#f8fafc\" font-size=\"{font:.1}\" font-weight=\"700\" text-anchor=\"middle\" dominant-baseline=\"middle\" pointer-events=\"none\">{}</text>",
                relaxed.x[i], relaxed.y[i], cfg.labels[i],
            ));
        }
    }

    to_html(cfg, svg, n)
}

fn to_html(cfg: &CartogramConfig, mut svg: String, n: usize) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let auto = cfg.hover.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if auto { Vec::with_capacity(n) } else { Vec::new() };
    if auto {
        for i in 0..n {
            auto_slots.push(
                HoverSlot::new(cfg.labels[i].clone())
                    .kv("Valeur", format!("{:.2}", cfg.values[i]))
                    .kv("Lat / Lon", format!("{:.2} / {:.2}", cfg.lats[i], cfg.lons[i])),
            );
        }
    }
    if !svg.ends_with("</svg>") {
        svg.push_str("</svg>");
    }
    let slots = if auto { &auto_slots } else { cfg.hover };
    build_chart_html(cfg.title, &svg, &slots_to_json(slots))
}
