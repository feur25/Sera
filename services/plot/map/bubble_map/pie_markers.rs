use super::common::{svg_open, PALETTE};
use super::config::BubbleMapConfig;
use crate::plot::map::world_data;
use std::f64::consts::PI;

fn push_pie(svg: &mut String, cx: f64, cy: f64, r: f64, series: &[f64], point_idx: usize) {
    let total: f64 = series.iter().sum();
    if total <= 0.0 {
        return;
    }
    let mut angle = -PI / 2.0;
    for (c, &val) in series.iter().enumerate() {
        if val <= 0.0 {
            continue;
        }
        let sweep = (val / total) * 2.0 * PI;
        let end = angle + sweep;
        let large_arc = if sweep > PI { 1 } else { 0 };
        let x1 = cx + r * angle.cos();
        let y1 = cy + r * angle.sin();
        let x2 = cx + r * end.cos();
        let y2 = cy + r * end.sin();
        let (red, green, blue) = PALETTE[c % PALETTE.len()];
        if sweep >= 2.0 * PI - 1e-6 {
            svg.push_str(&format!(
                "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"rgb({red},{green},{blue})\" stroke=\"#0d1117\" stroke-width=\"1\" data-index=\"{point_idx}\" data-cat=\"{c}\"/>"
            ));
        } else {
            svg.push_str(&format!(
                "<path d=\"M{cx:.1},{cy:.1} L{x1:.1},{y1:.1} A{r:.1},{r:.1} 0 {large_arc},1 {x2:.1},{y2:.1} Z\" fill=\"rgb({red},{green},{blue})\" stroke=\"#0d1117\" stroke-width=\"1\" data-index=\"{point_idx}\" data-cat=\"{c}\"/>"
            ));
        }
        angle = end;
    }
    svg.push_str(&format!(
        "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"none\" stroke=\"#0d1117\" stroke-width=\"1.4\"/>"
    ));
}

fn push_legend(svg: &mut String, categories: &[String], width: i32) {
    if categories.is_empty() {
        return;
    }
    let x = width as f64 - 168.0;
    for (c, name) in categories.iter().enumerate() {
        let y = 24.0 + c as f64 * 20.0;
        let (red, green, blue) = PALETTE[c % PALETTE.len()];
        svg.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{:.1}\" width=\"12\" height=\"12\" rx=\"2\" fill=\"rgb({red},{green},{blue})\"/><text x=\"{:.1}\" y=\"{:.1}\" fill=\"#cbd5e1\" font-size=\"11\" font-family=\"Arial,sans-serif\">{}</text>",
            y - 10.0,
            x + 17.0,
            y,
            name,
        ));
    }
}

#[crate::chart_demo(
    "lats=[51.5,48.85,52.5,41.9,40.4], lons=[-0.12,2.35,13.4,12.5,-3.7], series=[[40,25,35],[55,15,30],[30,45,25],[60,10,30],[35,35,30]], categories=[\"Wind\",\"Solar\",\"Hydro\"], title=\"Energy Mix by Capital\", variant=\"pie_markers\""
)]
pub fn render(cfg: &BubbleMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.series.len());
    if n == 0 {
        return String::new();
    }

    let totals: Vec<f64> = cfg.series[..n].iter().map(|s| s.iter().sum()).collect();
    let max_total = totals.iter().cloned().fold(1e-9_f64, f64::max);

    let mut svg = svg_open(cfg.width, cfg.height);
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
            svg.push_str(" Z\" fill=\"#151b23\" stroke=\"#2a2a4a\" stroke-width=\"0.3\"/>");
        }
    }

    for i in 0..n {
        if totals[i] <= 0.0 {
            continue;
        }
        let (nx, ny) = world_data::latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
        let cx = nx as f64 * cfg.width as f64;
        let cy = ny as f64 * cfg.height as f64;
        let t = (totals[i] / max_total).sqrt();
        let radius = cfg.min_bubble_size + t * (cfg.max_bubble_size - cfg.min_bubble_size);
        push_pie(&mut svg, cx, cy, radius, &cfg.series[i], i);
    }

    push_legend(&mut svg, cfg.categories, cfg.width);
    to_html(cfg, svg, n)
}

fn to_html(cfg: &BubbleMapConfig, mut svg: String, n: usize) -> String {
    use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        let name = cfg.labels.get(i).cloned().unwrap_or_else(|| format!("p{i}"));
        let mut slot = HoverSlot::new(name);
        let total: f64 = cfg.series[i].iter().sum();
        for (c, &val) in cfg.series[i].iter().enumerate() {
            let cat = cfg.categories.get(c).cloned().unwrap_or_else(|| format!("cat{c}"));
            let pct = if total > 0.0 { val / total * 100.0 } else { 0.0 };
            slot = slot.kv(cat, format!("{val:.1} ({pct:.0}%)"));
        }
        slots.push(slot);
    }
    if !svg.ends_with("</svg>") {
        svg.push_str("</svg>");
    }
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}
