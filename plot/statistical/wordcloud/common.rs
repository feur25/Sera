use super::config::WordCloudConfig;
use super::shape::WordCloudShape;
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

pub fn shape_inside(shape: WordCloudShape, nx: f64, ny: f64) -> bool {
    use WordCloudShape::*;
    match shape {
        Rect => nx.abs() <= 1.0 && ny.abs() <= 1.0,
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
        Glasses => {
            let cx_l = -0.50_f64;
            let cx_r =  0.50_f64;
            let rx = 0.42_f64;
            let ry = 0.34_f64;
            let in_l = ((nx - cx_l) / rx).powi(2) + (ny / ry).powi(2) <= 1.0;
            let in_r = ((nx - cx_r) / rx).powi(2) + (ny / ry).powi(2) <= 1.0;
            let in_bridge = nx.abs() <= 0.55 && ny.abs() <= 0.05;
            in_l || in_r || in_bridge
        }
    }
}

pub fn mask_inside(mask: &[i32], mw: i32, mh: i32, nx: f64, ny: f64) -> bool {
    if mw <= 0 || mh <= 0 || mask.is_empty() { return true; }
    let u = (nx + 1.0) * 0.5;
    let v = (ny + 1.0) * 0.5;
    if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 { return false; }
    let mx = ((u * (mw as f64 - 1.0)).round() as i32).clamp(0, mw - 1);
    let my = ((v * (mh as f64 - 1.0)).round() as i32).clamp(0, mh - 1);
    let i = (my * mw + mx) as usize;
    if i >= mask.len() { return false; }
    mask[i] != 0
}

pub fn shape_box_inside(shape: WordCloudShape, x: f64, y: f64, w: f64, h: f64, cx: f64, cy: f64, rx: f64, ry: f64) -> bool {
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

pub fn mask_box_inside(mask: &[i32], mw: i32, mh: i32, x: f64, y: f64, w: f64, h: f64, cx: f64, cy: f64, rx: f64, ry: f64) -> bool {
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
        if !mask_inside(mask, mw, mh, nx, ny) { return false; }
    }
    true
}

pub fn place_words(words: &[String], font_sizes: &[f64], width: f64, height: f64, pad_t: f64, test: &dyn Fn(f64, f64, f64, f64, f64, f64, f64, f64) -> bool) -> Vec<PlacedWord> {
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
                && test(px, py, tw, th, cx, cy, rx, ry) {
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

fn svg_open(buf: &mut Vec<u8>, w: i32, h: i32, bg: &str) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, w);
    push_b(buf, b"\" height=\"");
    push_i(buf, h);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, w);
    push_b(buf, b" ");
    push_i(buf, h);
    push_b(buf, b"\">");
    push_b(buf, b"<rect width=\"100%\" height=\"100%\" fill=\"");
    push_b(buf, bg.as_bytes());
    push_b(buf, b"\"/>");
}

fn svg_title(buf: &mut Vec<u8>, w: i32, title: &str, fill: &str) {
    if title.is_empty() { return; }
    push_b(buf, b"<text x=\"");
    push_i(buf, w / 2);
    push_b(buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"");
    push_b(buf, fill.as_bytes());
    push_b(buf, b"\">");
    escape_xml(buf, title);
    push_b(buf, b"</text>");
}

fn finalize(cfg: &WordCloudConfig, buf: Vec<u8>) -> String {
    let auto_hover = cfg.hover.is_empty();
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}

pub fn render_basic(cfg: &WordCloudConfig) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };
    let shape = cfg.shape;
    let test = move |x: f64, y: f64, w: f64, h: f64, cx: f64, cy: f64, rx: f64, ry: f64| -> bool {
        shape_box_inside(shape, x, y, w, h, cx, cy, rx, ry)
    };
    let placed = place_words(&p.words, &p.sizes, cfg.width as f64, cfg.height as f64, p.pad_t, &test);
    let bg = cfg.bg_color.unwrap_or("#1a1a2e");
    let mut buf = Vec::<u8>::with_capacity(p.words.len() * 320 + 2048);
    svg_open(&mut buf, cfg.width, cfg.height, bg);
    svg_title(&mut buf, cfg.width, cfg.title, "#fff");
    write_words(&mut buf, &placed, &p, cfg);
    push_b(&mut buf, b"</svg>");
    finalize(cfg, buf)
}

pub fn render_image(cfg: &WordCloudConfig) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };
    let mask = cfg.mask;
    let mw = cfg.mask_width;
    let mh = cfg.mask_height;
    let test = move |x: f64, y: f64, w: f64, h: f64, cx: f64, cy: f64, rx: f64, ry: f64| -> bool {
        mask_box_inside(mask, mw, mh, x, y, w, h, cx, cy, rx, ry)
    };
    let placed = place_words(&p.words, &p.sizes, cfg.width as f64, cfg.height as f64, p.pad_t, &test);
    let bg = cfg.bg_color.unwrap_or("#ffffff");
    let mut buf = Vec::<u8>::with_capacity(p.words.len() * 320 + 2048);
    svg_open(&mut buf, cfg.width, cfg.height, bg);
    svg_title(&mut buf, cfg.width, cfg.title, "#0f172a");
    write_words(&mut buf, &placed, &p, cfg);
    push_b(&mut buf, b"</svg>");
    finalize(cfg, buf)
}

fn write_words(buf: &mut Vec<u8>, placed: &[PlacedWord], p: &Prepared, cfg: &WordCloudConfig) {
    for pw in placed {
        let word = &p.words[pw.idx];
        let freq = p.freqs[pw.idx];
        let color_idx = p.orig_idx[pw.idx];
        let color = palette_color(cfg.palette, color_idx);
        let pct = if p.total > 0.0 { freq / p.total * 100.0 } else { 0.0 };
        let text_x = pw.x + pw.w / 2.0;
        let text_y = pw.y + pw.h * 0.72;
        push_b(buf, b"<text data-idx=\"");
        push_i(buf, pw.idx as i32);
        push_b(buf, b"\" data-lbl=\""); escape_xml(buf, word);
        push_b(buf, b"\" data-v=\""); push_f2(buf, freq);
        push_b(buf, b"\" data-kv-Pct=\""); push_f2(buf, pct); buf.push(b'%');
        push_b(buf, b"\" x=\"");
        push_f2(buf, text_x);
        push_b(buf, b"\" y=\"");
        push_f2(buf, text_y);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-size=\"");
        push_f2(buf, pw.font_size);
        push_b(buf, b"\" font-weight=\"700\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(buf, b"\" style=\"cursor:pointer\">");
        escape_xml(buf, word);
        push_b(buf, b"</text>");
    }
}

pub fn render_labelmap(cfg: &WordCloudConfig) -> String {
    let bg = cfg.bg_color.unwrap_or("#fafafa");
    let mut buf = Vec::<u8>::with_capacity(8192);
    svg_open(&mut buf, cfg.width, cfg.height, bg);
    svg_title(&mut buf, cfg.width, cfg.title, "#0f172a");

    let n = cfg.points_x.len().min(cfg.points_y.len());
    if n == 0 { push_b(&mut buf, b"</svg>"); return finalize(cfg, buf); }

    let pad_t = if cfg.title.is_empty() { 12.0 } else { 42.0 };
    let pad = 16.0_f64;
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let plot_w = w - pad * 2.0;
    let plot_h = h - pad_t - pad;

    let mut x_min = f64::INFINITY; let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY; let mut y_max = f64::NEG_INFINITY;
    for i in 0..n {
        let xv = cfg.points_x[i]; let yv = cfg.points_y[i];
        if xv < x_min { x_min = xv; } if xv > x_max { x_max = xv; }
        if yv < y_min { y_min = yv; } if yv > y_max { y_max = yv; }
    }
    let xr = (x_max - x_min).max(1e-6);
    let yr = (y_max - y_min).max(1e-6);

    let project = |xv: f64, yv: f64| -> (f64, f64) {
        let px = pad + (xv - x_min) / xr * plot_w;
        let py = pad_t + (1.0 - (yv - y_min) / yr) * plot_h;
        (px, py)
    };

    let cluster_max = cfg.point_clusters.iter().copied().max().unwrap_or(0).max(0);
    let n_clusters = (cluster_max + 1) as usize;

    push_b(&mut buf, b"<g class=\"sp-points\" pointer-events=\"none\">");
    for i in 0..n {
        let cid = if i < cfg.point_clusters.len() { cfg.point_clusters[i].max(0) as usize } else { 0 };
        let color = palette_color(cfg.palette, cid);
        let (px, py) = project(cfg.points_x[i], cfg.points_y[i]);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"1.6\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.55\"/>");
    }
    push_b(&mut buf, b"</g>");

    let mut sums_x: Vec<f64> = vec![0.0; n_clusters];
    let mut sums_y: Vec<f64> = vec![0.0; n_clusters];
    let mut counts: Vec<usize> = vec![0; n_clusters];
    for i in 0..n {
        let cid = if i < cfg.point_clusters.len() { cfg.point_clusters[i].max(0) as usize } else { 0 };
        if cid < n_clusters {
            sums_x[cid] += cfg.points_x[i];
            sums_y[cid] += cfg.points_y[i];
            counts[cid] += 1;
        }
    }

    let n_lab = cfg.cluster_labels.len().min(n_clusters);
    push_b(&mut buf, b"<g class=\"sp-labels\">");
    for cid in 0..n_lab {
        if counts[cid] == 0 { continue; }
        let cx_data = sums_x[cid] / counts[cid] as f64;
        let cy_data = sums_y[cid] / counts[cid] as f64;
        let (cx, cy) = project(cx_data, cy_data);
        let label = &cfg.cluster_labels[cid];
        let fs = cfg.min_font.max(11.0).min(cfg.max_font);
        let color = palette_color(cfg.palette, cid);
        let lw = estimate_text_width(label, fs) + 14.0;

        let mut lx = cx + 30.0;
        let mut ly = cy - 6.0;
        if lx + lw > w - 4.0 { lx = cx - 30.0 - lw; }
        if ly < pad_t + 8.0 { ly = pad_t + 8.0; }
        if ly > h - 18.0 { ly = h - 18.0; }
        let anchor_x = if lx > cx { lx } else { lx + lw };

        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, cx); push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, cy); push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, anchor_x); push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ly + 7.0); push_b(&mut buf, b"\" stroke=\"#475569\" stroke-width=\"0.6\" stroke-opacity=\"0.65\"/>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx + lw / 2.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 11.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-weight=\"700\" font-size=\"");
        push_f2(&mut buf, fs);
        push_b(&mut buf, b"\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"</svg>");
    finalize(cfg, buf)
}

pub fn render_network(cfg: &WordCloudConfig) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };
    let bg = cfg.bg_color.unwrap_or("#ffffff");
    let mut buf = Vec::<u8>::with_capacity(p.words.len() * 360 + 4096);
    svg_open(&mut buf, cfg.width, cfg.height, bg);
    svg_title(&mut buf, cfg.width, cfg.title, "#0f172a");

    let n = p.words.len();
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let pad_t = p.pad_t;
    let cx = w / 2.0;
    let cy = pad_t + (h - pad_t) / 2.0;
    let r_max = (w.min(h - pad_t)) * 0.40;

    let mut pos: Vec<(f64, f64)> = Vec::with_capacity(n);
    let max_f = p.freqs.iter().copied().fold(0.0_f64, f64::max).max(1.0);
    for i in 0..n {
        let intensity = p.freqs[i] / max_f;
        let ring = 1.0 - intensity;
        let r = 0.12 * r_max + ring * (r_max * 0.95);
        let golden = std::f64::consts::PI * (3.0 - (5.0_f64).sqrt());
        let theta = (i as f64) * golden;
        let px = cx + r * theta.cos();
        let py = cy + r * theta.sin();
        pos.push((px, py));
    }

    let m = cfg.edges_i.len().min(cfg.edges_j.len());
    push_b(&mut buf, b"<g class=\"sp-edges\" pointer-events=\"none\">");
    for k in 0..m {
        let i = cfg.edges_i[k] as usize;
        let j = cfg.edges_j[k] as usize;
        if i >= n || j >= n { continue; }
        let weight = if k < cfg.edges_w.len() { cfg.edges_w[k] } else { 1.0 };
        let (x1, y1) = pos[i];
        let (x2, y2) = pos[j];
        let cmx = (x1 + x2) / 2.0 + (y2 - y1) * 0.12;
        let cmy = (y1 + y2) / 2.0 - (x2 - x1) * 0.12;
        let color_a = palette_color(cfg.palette, i);
        let stroke_w = (0.3 + weight.abs().sqrt() * 0.9).clamp(0.4, 3.0);
        push_b(&mut buf, b"<path d=\"M");
        push_f2(&mut buf, x1); buf.push(b','); push_f2(&mut buf, y1);
        push_b(&mut buf, b" Q"); push_f2(&mut buf, cmx); buf.push(b','); push_f2(&mut buf, cmy);
        buf.push(b' '); push_f2(&mut buf, x2); buf.push(b','); push_f2(&mut buf, y2);
        push_b(&mut buf, b"\" stroke=\"#");
        let hx = hex6(color_a);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-opacity=\"0.30\" stroke-width=\"");
        push_f2(&mut buf, stroke_w);
        push_b(&mut buf, b"\" fill=\"none\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g class=\"sp-nodes\">");
    for i in 0..n {
        let word = &p.words[i];
        let freq = p.freqs[i];
        let pct = if p.total > 0.0 { freq / p.total * 100.0 } else { 0.0 };
        let color = palette_color(cfg.palette, p.orig_idx[i]);
        let (x, y) = pos[i];
        let r = (4.0 + (freq / max_f) * 14.0).clamp(3.0, 22.0);
        let fs = (p.sizes[i] * 0.55).clamp(cfg.min_font, cfg.max_font * 0.6);

        push_b(&mut buf, b"<g data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, word);
        push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, freq);
        push_b(&mut buf, b"\" data-kv-Pct=\""); push_f2(&mut buf, pct); buf.push(b'%');
        push_b(&mut buf, b"\" style=\"cursor:pointer\">");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, x); push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, y); push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.85\" stroke=\"#0f172a\" stroke-opacity=\"0.25\" stroke-width=\"0.8\"/>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, x); push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, y - r - 3.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-weight=\"700\" font-size=\"");
        push_f2(&mut buf, fs);
        push_b(&mut buf, b"\" fill=\"#0f172a\">");
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"</text>");

        push_b(&mut buf, b"</g>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"</svg>");
    finalize(cfg, buf)
}

pub fn render_bubble(cfg: &WordCloudConfig) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };
    let bg = cfg.bg_color.unwrap_or("#1a1a2e");
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let pad_t = p.pad_t;
    let cx = w / 2.0;
    let cy = pad_t + (h - pad_t) / 2.0;
    let max_f = p.freqs.iter().copied().fold(0.0_f64, f64::max).max(1.0);
    let min_r = 12.0_f64;
    let max_r = (w.min(h - pad_t)) * 0.16;
    let radii: Vec<f64> = p.freqs.iter().map(|&f| {
        min_r + (f / max_f).powf(0.5) * (max_r - min_r)
    }).collect();

    let mut circles: Vec<(f64, f64, f64, usize)> = Vec::with_capacity(p.words.len());
    let max_search = w.max(h) * 1.5;

    for i in 0..p.words.len() {
        let r = radii[i];
        let step_a = 0.32_f64;
        let step_r = 2.2_f64;
        let mut angle = 0.0_f64;
        let mut radius = 0.0_f64;
        let mut best = (cx, cy);

        loop {
            let px = cx + radius * angle.cos();
            let py = cy + radius * angle.sin() * 0.82;
            if px - r >= 2.0 && px + r <= w - 2.0 && py - r >= pad_t + 2.0 && py + r <= h - 2.0 {
                let mut ok = true;
                for &(ox, oy, or2, _) in &circles {
                    if ((px - ox).powi(2) + (py - oy).powi(2)).sqrt() < r + or2 + 2.5 {
                        ok = false; break;
                    }
                }
                if ok { best = (px, py); break; }
            }
            angle += step_a;
            radius += step_r * step_a / (2.0 * std::f64::consts::PI);
            if radius > max_search { break; }
        }
        circles.push((best.0, best.1, r, i));
    }

    let mut buf = Vec::<u8>::with_capacity(p.words.len() * 420 + 2048);
    svg_open(&mut buf, cfg.width, cfg.height, bg);
    svg_title(&mut buf, cfg.width, cfg.title, "#e2e8f0");

    for &(bx, by, br, idx) in &circles {
        let word = &p.words[idx];
        let freq = p.freqs[idx];
        let pct = if p.total > 0.0 { freq / p.total * 100.0 } else { 0.0 };
        let color = palette_color(cfg.palette, p.orig_idx[idx]);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, bx); push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, by); push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, br);
        push_b(&mut buf, b"\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.82\" stroke=\"rgba(255,255,255,0.18)\" stroke-width=\"1\"/>");

        let fs = (br * 0.40).clamp(cfg.min_font, cfg.max_font * 0.65);
        push_b(&mut buf, b"<text data-idx=\"");
        push_i(&mut buf, idx as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, word);
        push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, freq);
        push_b(&mut buf, b"\" data-kv-Pct=\""); push_f2(&mut buf, pct); buf.push(b'%');
        push_b(&mut buf, b"\" x=\""); push_f2(&mut buf, bx);
        push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, by + fs * 0.38);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-weight=\"700\" font-size=\"");
        push_f2(&mut buf, fs);
        push_b(&mut buf, b"\" fill=\"rgba(255,255,255,0.92)\" style=\"cursor:pointer\">");
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    finalize(cfg, buf)
}

pub fn render_context(cfg: &WordCloudConfig) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };
    let bg = cfg.bg_color.unwrap_or("#fafafa");
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let pad_t = p.pad_t;
    let n = p.words.len();
    let cx_init = w / 2.0;
    let cy_init = pad_t + (h - pad_t) / 2.0;
    let max_f = p.freqs.iter().copied().fold(0.0_f64, f64::max).max(1.0);
    let golden = std::f64::consts::PI * (3.0 - (5.0_f64).sqrt());
    let init_r = (w.min(h - pad_t)) * 0.36;

    let mut pos_x: Vec<f64> = Vec::with_capacity(n);
    let mut pos_y: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        let ring = 1.0 - (p.freqs[i] / max_f).powf(0.6);
        let r = ring * init_r;
        let theta = (i as f64) * golden;
        pos_x.push(cx_init + r * theta.cos());
        pos_y.push(cy_init + r * theta.sin());
    }

    let m = cfg.edges_i.len().min(cfg.edges_j.len());
    if m > 0 {
        let k_spring = 0.07_f64;
        let k_repel = 900.0_f64;
        let damping = 0.86_f64;
        let dt = 0.85_f64;
        let mut vx = vec![0.0_f64; n];
        let mut vy = vec![0.0_f64; n];

        for _iter in 0..220 {
            let mut fx = vec![0.0_f64; n];
            let mut fy = vec![0.0_f64; n];
            for i in 0..n {
                for j in (i + 1)..n {
                    let dx = pos_x[i] - pos_x[j];
                    let dy = pos_y[i] - pos_y[j];
                    let d2 = dx * dx + dy * dy + 1.0;
                    let d = d2.sqrt();
                    let f = k_repel / d2;
                    fx[i] += f * dx / d; fy[i] += f * dy / d;
                    fx[j] -= f * dx / d; fy[j] -= f * dy / d;
                }
            }
            for k in 0..m {
                let i = cfg.edges_i[k] as usize;
                let j = cfg.edges_j[k] as usize;
                if i >= n || j >= n { continue; }
                let we = if k < cfg.edges_w.len() { cfg.edges_w[k].abs().max(0.1) } else { 1.0 };
                let dx = pos_x[j] - pos_x[i];
                let dy = pos_y[j] - pos_y[i];
                let d = (dx * dx + dy * dy).sqrt().max(1.0);
                let f = k_spring * we * d;
                fx[i] += f * dx / d; fy[i] += f * dy / d;
                fx[j] -= f * dx / d; fy[j] -= f * dy / d;
            }
            for i in 0..n {
                fx[i] += (cx_init - pos_x[i]) * 0.007;
                fy[i] += (cy_init - pos_y[i]) * 0.007;
                vx[i] = (vx[i] + fx[i] * dt) * damping;
                vy[i] = (vy[i] + fy[i] * dt) * damping;
                pos_x[i] += vx[i] * dt;
                pos_y[i] += vy[i] * dt;
            }
        }
    }

    let pp = 54.0_f64;
    let x_min = pos_x.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = pos_x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let y_min = pos_y.iter().copied().fold(f64::INFINITY, f64::min);
    let y_max = pos_y.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let xr = (x_max - x_min).max(1.0);
    let yr = (y_max - y_min).max(1.0);
    let scale = ((w - pp * 2.0) / xr).min((h - pad_t - pp * 1.5) / yr);
    for i in 0..n {
        pos_x[i] = cx_init + (pos_x[i] - (x_min + x_max) / 2.0) * scale;
        pos_y[i] = cy_init + (pos_y[i] - (y_min + y_max) / 2.0) * scale;
    }

    let has_clusters = cfg.point_clusters.len() >= n;

    let mut buf = Vec::<u8>::with_capacity(n * 340 + 2048);
    svg_open(&mut buf, cfg.width, cfg.height, bg);
    svg_title(&mut buf, cfg.width, cfg.title, "#0f172a");

    for i in 0..n {
        let word = &p.words[i];
        let freq = p.freqs[i];
        let pct = if p.total > 0.0 { freq / p.total * 100.0 } else { 0.0 };
        let color_idx = if has_clusters {
            cfg.point_clusters[p.orig_idx[i]].max(0) as usize
        } else {
            p.orig_idx[i]
        };
        let color = palette_color(cfg.palette, color_idx);
        let fs = p.sizes[i];
        push_b(&mut buf, b"<text data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, word);
        push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, freq);
        push_b(&mut buf, b"\" data-kv-Pct=\""); push_f2(&mut buf, pct); buf.push(b'%');
        push_b(&mut buf, b"\" x=\""); push_f2(&mut buf, pos_x[i]);
        push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, pos_y[i]);
        push_b(&mut buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"'Segoe UI',Arial,sans-serif\" font-weight=\"700\" font-size=\"");
        push_f2(&mut buf, fs);
        push_b(&mut buf, b"\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" style=\"cursor:pointer\">");
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    finalize(cfg, buf)
}


