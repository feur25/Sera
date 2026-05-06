use super::config::WordCloudConfig;
use super::variant::WordCloudVariant;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, apply_sort};
use crate::html::hover::{slots_to_json, build_chart_html};

pub struct PlacedWord {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub font_size: f64,
    pub idx: usize,
}

pub fn estimate_text_width(text: &str, font_size: f64) -> f64 {
    let mut w = 0.0;
    for ch in text.chars() {
        let ratio = if ch.is_uppercase() { 0.72 } else if ch == ' ' { 0.3 } else { 0.55 };
        w += font_size * ratio;
    }
    w
}

pub fn intersects(a: &PlacedWord, b: &PlacedWord) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

pub fn shape_inside(shape: WordCloudVariant, nx: f64, ny: f64) -> bool {
    use WordCloudVariant::*;
    match shape {
        Basic => nx.abs() <= 1.0 && ny.abs() <= 1.0,
        Circle => nx * nx + ny * ny <= 1.0,
        Heart => {
            let yp = -ny;
            let v = (nx * nx + yp * yp - 1.0).powi(3) - nx * nx * yp.powi(3);
            v <= 0.0
        }
        Diamond => nx.abs() + ny.abs() <= 1.0,
        Star => {
            let r2 = nx * nx + ny * ny;
            if r2 <= 0.0001 { return true; }
            let theta = ny.atan2(nx);
            let n_pts = 5.0_f64;
            let inner = 0.45_f64;
            let phase = std::f64::consts::PI / 2.0;
            let a = ((theta + phase) * n_pts / 2.0).cos().abs();
            let lim = inner + (1.0 - inner) * a;
            r2.sqrt() <= lim
        }
        Bird => {
            let disks: [(f64, f64, f64); 7] = [
                (0.00, 0.00, 0.55),
                (0.40, -0.20, 0.42),
                (-0.35, 0.15, 0.42),
                (0.25, 0.45, 0.36),
                (-0.15, -0.40, 0.32),
                (0.55, 0.25, 0.30),
                (0.10, -0.55, 0.28),
            ];
            for (dx, dy, r) in disks.iter() {
                let d = (nx - dx).powi(2) + (ny - dy).powi(2);
                if d <= r * r { return true; }
            }
            false
        }
    }
}

pub fn shape_box_inside(shape: WordCloudVariant, x: f64, y: f64, w: f64, h: f64, cx: f64, cy: f64, rx: f64, ry: f64) -> bool {
    let pts = [
        (x + w * 0.5, y + h * 0.5),
        (x, y),
        (x + w, y),
        (x, y + h),
        (x + w, y + h),
    ];
    for (px, py) in pts.iter() {
        let nx = (px - cx) / rx;
        let ny = (py - cy) / ry;
        if !shape_inside(shape, nx, ny) { return false; }
    }
    true
}

pub fn place_words(words: &[String], font_sizes: &[f64], width: f64, height: f64, pad_t: f64, shape: WordCloudVariant) -> Vec<PlacedWord> {
    let n = words.len();
    let mut placed: Vec<PlacedWord> = Vec::with_capacity(n);
    let cx = width / 2.0;
    let cy = pad_t + (height - pad_t) / 2.0;
    let rx = (width / 2.0) - 6.0;
    let ry = ((height - pad_t) / 2.0) - 6.0;
    let pad = 4.0;

    for i in 0..n {
        let fs = font_sizes[i];
        let tw = estimate_text_width(&words[i], fs) + pad * 2.0;
        let th = fs * 1.3 + pad;

        let mut best_x = cx - tw / 2.0;
        let mut best_y = cy - th / 2.0;
        let mut found = false;

        let max_r = (width.max(height)) / 1.6;
        let step_a = 0.32;
        let step_r = 1.0;
        let mut angle: f64 = 0.0;
        let mut r: f64 = 0.0;

        'spiral: loop {
            let px = cx + r * angle.cos() - tw / 2.0;
            let py = cy + r * angle.sin() * 0.65 - th / 2.0;

            if px >= 2.0 && px + tw <= width - 2.0 && py >= pad_t && py + th <= height - 2.0
                && shape_box_inside(shape, px, py, tw, th, cx, cy, rx, ry) {
                let candidate = PlacedWord { x: px, y: py, w: tw, h: th, font_size: fs, idx: i };
                let mut ok = true;
                for p in &placed {
                    if intersects(&candidate, p) { ok = false; break; }
                }
                if ok {
                    best_x = px;
                    best_y = py;
                    found = true;
                    break 'spiral;
                }
            }

            angle += step_a;
            r += step_r * step_a / (2.0 * std::f64::consts::PI);

            if r > max_r { break; }
        }

        if found || placed.is_empty() {
            placed.push(PlacedWord { x: best_x, y: best_y, w: tw, h: th, font_size: fs, idx: i });
        }
    }
    placed
}

pub struct Prepared {
    pub words: Vec<String>,
    pub freqs: Vec<f64>,
    pub sizes: Vec<f64>,
    pub orig_idx: Vec<usize>,
    pub total: f64,
    pub pad_t: f64,
}

pub fn prepare(cfg: &WordCloudConfig) -> Option<Prepared> {
    let n = cfg.words.len().min(cfg.frequencies.len());
    if n == 0 { return None; }
    let (sw, sf) = apply_sort(cfg.words, cfg.frequencies, cfg.sort_order);
    let max_f = sf.iter().copied().fold(0.0_f64, f64::max);
    let min_f = sf.iter().copied().fold(f64::INFINITY, f64::min);
    let range = (max_f - min_f).max(1.0);
    let font_sizes: Vec<f64> = sf.iter().map(|&f| {
        let t = (f - min_f) / range;
        cfg.min_font + t * (cfg.max_font - cfg.min_font)
    }).collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| font_sizes[b].partial_cmp(&font_sizes[a]).unwrap_or(std::cmp::Ordering::Equal));
    let words: Vec<String> = order.iter().map(|&i| sw[i].clone()).collect();
    let sizes: Vec<f64> = order.iter().map(|&i| font_sizes[i]).collect();
    let freqs: Vec<f64> = order.iter().map(|&i| sf[i]).collect();
    let total: f64 = sf.iter().sum();
    let pad_t: f64 = if cfg.title.is_empty() { 8.0 } else { 38.0 };
    Some(Prepared { words, freqs, sizes, orig_idx: order, total, pad_t })
}

pub fn render_with(cfg: &WordCloudConfig, shape: WordCloudVariant) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };
    let placed = place_words(&p.words, &p.sizes, cfg.width as f64, cfg.height as f64, p.pad_t, shape);
    let bg = cfg.bg_color.unwrap_or("#1a1a2e");
    let auto_hover = cfg.hover.is_empty();
    let mut buf = Vec::<u8>::with_capacity(p.words.len() * 320 + 2048);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\">");

    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"");
    push_b(&mut buf, bg.as_bytes());
    push_b(&mut buf, b"\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#fff\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for pw in &placed {
        let word = &p.words[pw.idx];
        let freq = p.freqs[pw.idx];
        let color_idx = p.orig_idx[pw.idx];
        let color = palette_color(cfg.palette, color_idx);
        let pct = if p.total > 0.0 { freq / p.total * 100.0 } else { 0.0 };

        let text_x = pw.x + pw.w / 2.0;
        let text_y = pw.y + pw.h * 0.72;

        push_b(&mut buf, b"<text data-idx=\"");
        push_i(&mut buf, pw.idx as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, word);
        push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, freq);
        push_b(&mut buf, b"\" data-kv-Pct=\""); push_f2(&mut buf, pct); buf.push(b'%');
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, text_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, text_y);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, pw.font_size);
        push_b(&mut buf, b"\" font-weight=\"700\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" style=\"cursor:pointer\">");
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");

    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}
