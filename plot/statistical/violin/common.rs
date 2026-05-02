use super::config::ViolinConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i,
    sort_indices, svg_axis_lines, svg_legend_item, svg_x_label, svg_y_label, Frame,
};

pub struct GroupStat {
    pub label: String,
    pub values: Vec<f64>,
    pub sorted: Vec<f64>,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub n: usize,
}

pub fn percentile(s: &[f64], p: f64) -> f64 {
    if s.is_empty() { return 0.0; }
    let idx = p * (s.len() - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    let frac = idx - lo as f64;
    s[lo] * (1.0 - frac) + s[hi.min(s.len() - 1)] * frac
}

pub fn density_at(points: &[f64], x: f64, bw: f64) -> f64 {
    if bw <= 0.0 { return 0.0; }
    let inv_bw = 1.0 / bw;
    let c = inv_bw * 0.3989422804014327;
    let mut sum = 0.0f64;
    for &p in points {
        let z = (x - p) * inv_bw;
        let zz = z * z;
        if zz < 16.0 { sum += c * (-0.5 * zz).exp(); }
    }
    sum
}

pub fn estimate_bw(sorted_vals: &[f64], scale: f64) -> f64 {
    let n = sorted_vals.len() as f64;
    if n < 2.0 { return 1.0; }
    let mean = sorted_vals.iter().sum::<f64>() / n;
    let var: f64 = sorted_vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let sd = var.sqrt().max(1e-9);
    let q1 = percentile(sorted_vals, 0.25);
    let q3 = percentile(sorted_vals, 0.75);
    let iqr = (q3 - q1).max(1e-9);
    let h = 0.9 * sd.min(iqr / 1.34) * n.powf(-0.2);
    (h * scale.max(0.05)).max(1e-6)
}

pub fn kde_curve(sorted_vals: &[f64], v_min: f64, range: f64, steps: usize, bw: f64) -> Vec<f64> {
    (0..=steps).map(|si| {
        let v = v_min + si as f64 / steps as f64 * range;
        density_at(sorted_vals, v, bw)
    }).collect()
}

pub fn group_data(cats: &[String], vals: &[f64]) -> Vec<GroupStat> {
    let n = cats.len().min(vals.len());
    let mut keys: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<f64>> = Vec::new();
    for i in 0..n {
        let v = vals[i];
        if !v.is_finite() { continue; }
        match keys.iter().position(|c| c == &cats[i]) {
            Some(p) => groups[p].push(v),
            None => { keys.push(cats[i].clone()); groups.push(vec![v]); }
        }
    }
    keys.into_iter().zip(groups.into_iter()).map(|(label, values)| {
        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = sorted.len();
        let q1 = percentile(&sorted, 0.25);
        let median = percentile(&sorted, 0.5);
        let q3 = percentile(&sorted, 0.75);
        let mean = if n > 0 { sorted.iter().sum::<f64>() / n as f64 } else { 0.0 };
        let min = sorted.first().copied().unwrap_or(0.0);
        let max = sorted.last().copied().unwrap_or(1.0);
        GroupStat { label, values, sorted, q1, median, q3, mean, min, max, n }
    }).collect()
}

pub fn sort_groups(g: Vec<GroupStat>, order: &str) -> Vec<GroupStat> {
    let labels: Vec<String> = g.iter().map(|s| s.label.clone()).collect();
    let medians: Vec<f64> = g.iter().map(|s| s.median).collect();
    let idx = sort_indices(g.len(), &medians, &labels, order);
    let mut wrapped: Vec<Option<GroupStat>> = g.into_iter().map(Some).collect();
    idx.into_iter().map(|i| wrapped[i].take().unwrap()).collect()
}

pub struct VRange { pub min: f64, pub max: f64, pub range: f64 }

pub fn value_range(groups: &[GroupStat]) -> VRange {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for g in groups {
        lo = lo.min(g.min);
        hi = hi.max(g.max);
    }
    if !lo.is_finite() || !hi.is_finite() { lo = 0.0; hi = 1.0; }
    let pad = (hi - lo).abs().max(1.0) * 0.08;
    let vmin = lo - pad;
    let vmax = hi + pad;
    VRange { min: vmin, max: vmax, range: (vmax - vmin).max(1e-9) }
}

pub fn make_frame(cfg: &ViolinConfig, n_cats: usize, legend_w: i32) -> Frame {
    Frame::new_html(cfg.title, cfg.width, cfg.height, 62, 46, 56, legend_w, n_cats * 700 + 4096)
}

pub fn open_axes_y(f: &mut Frame, title: &str, gridlines: bool, y_min: f64, y_max: f64) {
    f.open(title, false);
    f.y_grid(5, y_min, y_max, gridlines);
    svg_axis_lines(&mut f.buf, f.pl, f.pt, f.pw, f.ph);
}

pub fn open_axes_x(f: &mut Frame, title: &str, gridlines: bool, x_min: f64, x_max: f64) {
    f.open(title, false);
    f.x_grid(5, x_min, x_max, gridlines);
    svg_axis_lines(&mut f.buf, f.pl, f.pt, f.pw, f.ph);
}

pub fn finish(f: &mut Frame, labels: &[&str], palette: &[u32], x_label: &str, y_label: &str, legend_w: i32) {
    if labels.len() > 1 && legend_w > 30 {
        let lx = f.w - legend_w + 12;
        for (ci, name) in labels.iter().enumerate() {
            svg_legend_item(&mut f.buf, ci as i32, name, palette_color(palette, ci), lx, f.pt + ci as i32 * 22, 14);
        }
    }
    svg_x_label(&mut f.buf, x_label, f.pl + f.pw / 2, f.h - 4);
    svg_y_label(&mut f.buf, y_label, 14, f.pt, f.ph);
}

pub fn rng_next(state: &mut u64) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    ((*state >> 11) & ((1u64 << 53) - 1)) as f64 / ((1u64 << 53) as f64)
}

pub fn cat_label_short(cat: &str) -> &str {
    if cat.len() <= 14 { cat } else { &cat[..14] }
}

pub enum Side { Both, Left, Right }

pub fn write_violin_v(
    f: &mut Frame,
    cx: i32,
    half_w: i32,
    side: Side,
    dens: &[f64],
    max_dens: f64,
    v_min: f64,
    range: f64,
    color: u32,
    fill_opacity: f64,
    stroke_width: f64,
    idx: i32,
    label: &str,
    extra_kv: &[(&str, f64)],
) {
    if dens.is_empty() || max_dens <= 0.0 { return; }
    let _ = (v_min, range);
    let hx = hex6(color);
    let steps = dens.len() - 1;
    let yv = |si: usize| -> i32 {
        let frac = si as f64 / steps as f64;
        f.pt + f.ph - (frac * f.ph as f64) as i32
    };
    let w_at = |si: usize| -> i32 { (dens[si] / max_dens * half_w as f64) as i32 };
    push_b(&mut f.buf, b"<path data-idx=\""); push_i(&mut f.buf, idx);
    push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, label);
    for (k, v) in extra_kv {
        push_b(&mut f.buf, b"\" data-kv-"); push_b(&mut f.buf, k.as_bytes());
        push_b(&mut f.buf, b"=\""); push_f2(&mut f.buf, *v);
    }
    push_b(&mut f.buf, b"\" d=\"M ");
    let (start_x, start_y, build_right, build_left) = match side {
        Side::Both => (cx + w_at(0), yv(0), true, true),
        Side::Right => (cx + w_at(0), yv(0), true, false),
        Side::Left => (cx, yv(0), false, true),
    };
    push_i(&mut f.buf, start_x); f.buf.push(b' '); push_i(&mut f.buf, start_y);
    if build_right {
        for si in 1..=steps {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx + w_at(si));
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
    } else {
        for si in 1..=steps {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx);
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
    }
    if build_left {
        for si in (0..=steps).rev() {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx - w_at(si));
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
    } else {
        for si in (0..=steps).rev() {
            push_b(&mut f.buf, b" L "); push_i(&mut f.buf, cx);
            f.buf.push(b' '); push_i(&mut f.buf, yv(si));
        }
    }
    push_b(&mut f.buf, b" Z\" fill=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, fill_opacity);
    push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, stroke_width);
    push_b(&mut f.buf, b"\"/>");
}

pub fn write_violin_h(
    f: &mut Frame,
    cy: i32,
    half_h: i32,
    dens: &[f64],
    max_dens: f64,
    x_min: f64,
    range: f64,
    color: u32,
    fill_opacity: f64,
    stroke_width: f64,
    idx: i32,
    label: &str,
) {
    if dens.is_empty() || max_dens <= 0.0 { return; }
    let _ = (x_min, range);
    let hx = hex6(color);
    let steps = dens.len() - 1;
    let xv = |si: usize| -> i32 {
        let frac = si as f64 / steps as f64;
        f.pl + (frac * f.pw as f64) as i32
    };
    let h_at = |si: usize| -> i32 { (dens[si] / max_dens * half_h as f64) as i32 };
    push_b(&mut f.buf, b"<path data-idx=\""); push_i(&mut f.buf, idx);
    push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, label);
    push_b(&mut f.buf, b"\" d=\"M ");
    push_i(&mut f.buf, xv(0)); f.buf.push(b' '); push_i(&mut f.buf, cy + h_at(0));
    for si in 1..=steps {
        push_b(&mut f.buf, b" L "); push_i(&mut f.buf, xv(si));
        f.buf.push(b' '); push_i(&mut f.buf, cy + h_at(si));
    }
    for si in (0..=steps).rev() {
        push_b(&mut f.buf, b" L "); push_i(&mut f.buf, xv(si));
        f.buf.push(b' '); push_i(&mut f.buf, cy - h_at(si));
    }
    push_b(&mut f.buf, b" Z\" fill=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, fill_opacity);
    push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, stroke_width);
    push_b(&mut f.buf, b"\"/>");
}

pub fn draw_inner_box_v(
    f: &mut Frame, cx: i32, half_w: i32, g: &GroupStat,
    v_min: f64, range: f64,
) {
    let yv = |v: f64| f.pt + f.ph - ((v - v_min) / range * f.ph as f64) as i32;
    let bw = (half_w as f64 * 0.18).max(3.0) as i32;
    let y_q1 = yv(g.q1);
    let y_q3 = yv(g.q3);
    let y_med = yv(g.median);
    let top = y_q3.min(y_q1);
    let bot = y_q3.max(y_q1);
    push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, cx - bw);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, top);
    push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bw * 2);
    push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, (bot - top).max(2));
    push_b(&mut f.buf, b"\" fill=\"#1a202c\" fill-opacity=\"0.85\" rx=\"2\"/>");
    push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, yv(g.min));
    push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, yv(g.max));
    push_b(&mut f.buf, b"\" stroke=\"#1a202c\" stroke-width=\"1\" stroke-opacity=\"0.55\"/>");
    push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, y_med);
    push_b(&mut f.buf, b"\" r=\"2.6\" fill=\"#fff\"/>");
}

pub fn draw_quartile_lines_v(
    f: &mut Frame, cx: i32, half_w: i32, g: &GroupStat,
    v_min: f64, range: f64, color: u32,
) {
    let yv = |v: f64| f.pt + f.ph - ((v - v_min) / range * f.ph as f64) as i32;
    let hx = hex6(color);
    for (val, sw, op) in [(g.q1, 1.2, 0.7), (g.median, 2.4, 1.0), (g.q3, 1.2, 0.7)] {
        let y = yv(val);
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - half_w);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + half_w);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, sw);
        push_b(&mut f.buf, b"\" stroke-opacity=\""); push_f2(&mut f.buf, op);
        push_b(&mut f.buf, b"\" stroke-dasharray=\"4 3\"/>");
    }
}

pub fn draw_mean_v(
    f: &mut Frame, cx: i32, half_w: i32, g: &GroupStat,
    v_min: f64, range: f64,
) {
    let yv = |v: f64| f.pt + f.ph - ((v - v_min) / range * f.ph as f64) as i32;
    let y_mean = yv(g.mean);
    push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - half_w);
    push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_mean);
    push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + half_w);
    push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_mean);
    push_b(&mut f.buf, b"\" stroke=\"#facc15\" stroke-width=\"2\" stroke-dasharray=\"6 3\"/>");
    push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, yv(g.median));
    push_b(&mut f.buf, b"\" r=\"3\" fill=\"#fff\" stroke=\"#1a202c\" stroke-width=\"1\"/>");
}

pub fn draw_points_v(
    f: &mut Frame, cx: i32, half_w: i32, g: &GroupStat,
    v_min: f64, range: f64, color: u32, jitter: f64, rng: &mut u64,
) {
    let yv = |v: f64| f.pt + f.ph - ((v - v_min) / range * f.ph as f64) as i32;
    let hx = hex6(color);
    let max_off = (half_w as f64 * 0.55).max(2.0) * jitter.clamp(0.0, 1.0);
    for &v in &g.values {
        if !v.is_finite() { continue; }
        let dx = (rng_next(rng) - 0.5) * 2.0 * max_off;
        let py = yv(v);
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx + dx as i32);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, py);
        push_b(&mut f.buf, b"\" r=\"1.8\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.6\"/>");
    }
}

pub fn draw_cat_label_v(f: &mut Frame, cx: i32, label: &str) {
    push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
    push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
    escape_xml(&mut f.buf, cat_label_short(label));
    push_b(&mut f.buf, b"</text>");
}

pub fn draw_cat_label_h(f: &mut Frame, cy: i32, label: &str) {
    push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 6);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy + 4);
    push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
    escape_xml(&mut f.buf, cat_label_short(label));
    push_b(&mut f.buf, b"</text>");
}

pub fn rainbow_color(i: usize, n: usize) -> u32 {
    let h = if n <= 1 { 0.0 } else { i as f64 / n as f64 * 320.0 };
    hsv_to_rgb_u32(h, 0.72, 0.92)
}

fn hsv_to_rgb_u32(h: f64, s: f64, v: f64) -> u32 {
    let c = v * s;
    let hp = (h / 60.0) % 6.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r1, g1, b1) = match hp as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    let r = ((r1 + m) * 255.0) as u32;
    let g = ((g1 + m) * 255.0) as u32;
    let b = ((b1 + m) * 255.0) as u32;
    (r << 16) | (g << 8) | b
}
