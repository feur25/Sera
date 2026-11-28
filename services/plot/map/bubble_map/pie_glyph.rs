use crate::plot::statistical::pie::common::{render_pie_svg, PiePiece};
use crate::plot::statistical::pie::config::PieConfig;
use std::f64::consts::PI;

fn base_piece(cx: f64, cy: f64, radius: f64) -> PiePiece {
    PiePiece {
        area_x: cx - radius,
        area_y: cy - radius,
        area_w: radius * 2.0,
        area_h: radius * 2.0,
        draw_legend: false,
        title_top: false,
        ..PiePiece::default()
    }
}

fn base_config<'a>(labels: &'a [String], values: &'a [f64]) -> PieConfig<'a> {
    PieConfig {
        labels,
        values,
        show_pct: false,
        ..Default::default()
    }
}

pub fn push_pie_svg_glyph(
    svg: &mut String,
    cx: f64,
    cy: f64,
    radius: f64,
    labels: &[String],
    values: &[f64],
    pull: &[f64],
    customize: impl FnOnce(&mut PiePiece),
) {
    let cfg = base_config(labels, values);
    let mut piece = base_piece(cx, cy, radius);
    customize(&mut piece);
    let mut buf = Vec::<u8>::with_capacity(labels.len() * 220 + 512);
    render_pie_svg(&mut buf, &cfg, labels, values, pull, &piece);
    svg.push_str(&unsafe { String::from_utf8_unchecked(buf) });
}

pub fn largest_index(values: &[f64]) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (i, &v) in values.iter().enumerate() {
        match best {
            None => best = Some((i, v)),
            Some((_, bv)) if v > bv => best = Some((i, v)),
            _ => {}
        }
    }
    best.map(|(i, _)| i)
}

pub fn push_donut(svg: &mut String, cx: f64, cy: f64, radius: f64, labels: &[String], values: &[f64]) {
    push_pie_svg_glyph(svg, cx, cy, radius, labels, values, &[], |p| p.donut = 0.55);
}

pub fn push_semi(svg: &mut String, cx: f64, cy: f64, radius: f64, labels: &[String], values: &[f64]) {
    push_pie_svg_glyph(svg, cx, cy, radius, labels, values, &[], |p| {
        p.arc_start = PI;
        p.arc_span = PI;
        p.donut = 0.45;
    });
}

pub fn push_exploded(svg: &mut String, cx: f64, cy: f64, radius: f64, labels: &[String], values: &[f64]) {
    let mut pull = vec![0.0_f64; values.len()];
    if let Some(idx) = largest_index(values) {
        pull[idx] = 0.18;
    }
    push_pie_svg_glyph(svg, cx, cy, radius, labels, values, &pull, |_| {});
}

pub fn push_nightingale(svg: &mut String, cx: f64, cy: f64, radius: f64, palette: &[(u8, u8, u8)], values: &[f64]) {
    let n = values.len();
    if n == 0 {
        return;
    }
    let vmax = values.iter().cloned().fold(0.0_f64, f64::max).max(1e-9);
    let slice = 2.0 * PI / n as f64;
    for i in 0..n {
        if values[i] <= 0.0 {
            continue;
        }
        let a0 = -PI / 2.0 + slice * i as f64;
        let a1 = a0 + slice;
        let r = radius * (values[i] / vmax).clamp(0.0, 1.0);
        let x0 = cx + r * a0.cos();
        let y0 = cy + r * a0.sin();
        let x1 = cx + r * a1.cos();
        let y1 = cy + r * a1.sin();
        let large: u8 = if slice > PI { 1 } else { 0 };
        let (red, green, blue) = palette[i % palette.len()];
        svg.push_str(&format!(
            "<path d=\"M{cx:.1},{cy:.1} L{x0:.1},{y0:.1} A{r:.1},{r:.1} 0 {large},1 {x1:.1},{y1:.1} Z\" fill=\"rgb({red},{green},{blue})\" fill-opacity=\"0.88\" stroke=\"#0d1117\" stroke-width=\"0.8\" data-cat=\"{i}\"/>"
        ));
    }
    svg.push_str(&format!(
        "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{radius:.1}\" fill=\"none\" stroke=\"#0d1117\" stroke-width=\"1\"/>"
    ));
}

pub fn push_waffle(svg: &mut String, cx: f64, cy: f64, radius: f64, palette: &[(u8, u8, u8)], values: &[f64]) {
    const SIDE: usize = 3;
    let n = values.len();
    if n == 0 {
        return;
    }
    let total: f64 = values.iter().sum();
    if total <= 0.0 {
        return;
    }
    let n_cells = SIDE * SIDE;
    let raw: Vec<f64> = values.iter().map(|&v| v.max(0.0) / total * n_cells as f64).collect();
    let mut counts: Vec<usize> = raw.iter().map(|&r| r.floor() as usize).collect();
    let assigned: usize = counts.iter().sum();
    let mut remainders: Vec<(usize, f64)> = raw.iter().enumerate().map(|(i, &r)| (i, r - r.floor())).collect();
    remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut leftover = n_cells.saturating_sub(assigned);
    let mut ri = 0;
    while leftover > 0 && ri < remainders.len() {
        counts[remainders[ri].0] += 1;
        leftover -= 1;
        ri += 1;
    }
    let mut cell_cat: Vec<usize> = Vec::with_capacity(n_cells);
    for (i, &c) in counts.iter().enumerate() {
        for _ in 0..c {
            cell_cat.push(i);
        }
    }
    while cell_cat.len() < n_cells {
        cell_cat.push(n - 1);
    }

    let side_len = radius * 2.0;
    let cell = side_len / SIDE as f64;
    let gap = (cell * 0.12).max(0.4);
    let origin_x = cx - radius;
    let origin_y = cy - radius;
    for r in 0..SIDE {
        for c in 0..SIDE {
            let idx = r * SIDE + c;
            let cat = cell_cat[idx];
            let (red, green, blue) = palette[cat % palette.len()];
            let x = origin_x + c as f64 * cell;
            let y = origin_y + r as f64 * cell;
            svg.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"0.6\" fill=\"rgb({red},{green},{blue})\" data-cat=\"{cat}\"/>",
                cell - gap, cell - gap,
            ));
        }
    }
}
