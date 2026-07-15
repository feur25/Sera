use super::config::WordCloudConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("words=[\"transformer\",\"attention\",\"gpu\",\"carbon\",\"water\",\"data\",\"training\",\"inference\",\"cooling\",\"energy\",\"compute\",\"model\",\"layer\",\"batch\",\"gradient\"], frequencies=[95,88,82,76,70,64,58,52,46,40,34,28,22,16,10]")]

pub fn render(cfg: &WordCloudConfig) -> String {
    let n = cfg.words.len().min(cfg.frequencies.len());
    if n == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let pad_t = if cfg.title.is_empty() { 16.0 } else { 46.0 };
    let orbit_cx = w / 2.0;
    let orbit_cy = (h + pad_t) / 2.0;
    let max_r = (w.min(h - pad_t) * 0.43).max(70.0);

    let total: f64 = cfg.frequencies.iter().copied().sum::<f64>().max(1.0);
    let freq_max = cfg.frequencies.iter().cloned().fold(0.0_f64, f64::max).max(1.0);

    let min_node_r = 3.5_f64;
    let max_node_r = (cfg.max_font * 0.48).clamp(10.0, 26.0);

    let golden = std::f64::consts::PI * (3.0 - 5.0_f64.sqrt());

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        cfg.frequencies[b]
            .partial_cmp(&cfg.frequencies[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut pos_by_orig = vec![(orbit_cx, orbit_cy, 6.0_f64); n];
    for (rank, &orig) in order.iter().enumerate() {
        let t = if n > 1 { rank as f64 / (n - 1) as f64 } else { 0.0 };
        let r = max_r * t.sqrt();
        let theta = rank as f64 * golden;
        let px = orbit_cx + r * theta.cos();
        let py = orbit_cy + r * theta.sin();
        let freq = cfg.frequencies[orig];
        let node_r = min_node_r + (freq / freq_max) * (max_node_r - min_node_r);
        pos_by_orig[orig] = (px, py, node_r);
    }

    let mut buf = Vec::<u8>::with_capacity(n * 700 + 10240);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\">");

    push_b(
        &mut buf,
        b"<defs>\
        <radialGradient id=\"csbg\" cx=\"50%\" cy=\"50%\" r=\"72%\">\
        <stop offset=\"0%\" stop-color=\"#111827\"/>\
        <stop offset=\"100%\" stop-color=\"#030712\"/>\
        </radialGradient>\
        <filter id=\"cs-outer\" x=\"-150%\" y=\"-150%\" width=\"500%\" height=\"500%\" color-interpolation-filters=\"sRGB\">\
        <feGaussianBlur stdDeviation=\"6\" result=\"blur\"/>\
        <feMerge><feMergeNode in=\"blur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>\
        </filter>\
        <filter id=\"cs-mid\" x=\"-80%\" y=\"-80%\" width=\"360%\" height=\"360%\" color-interpolation-filters=\"sRGB\">\
        <feGaussianBlur stdDeviation=\"3\" result=\"blur\"/>\
        <feMerge><feMergeNode in=\"blur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>\
        </filter>\
        <filter id=\"cs-core\" x=\"-60%\" y=\"-60%\" width=\"320%\" height=\"320%\" color-interpolation-filters=\"sRGB\">\
        <feGaussianBlur stdDeviation=\"1.6\" result=\"blur\"/>\
        <feMerge><feMergeNode in=\"blur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>\
        </filter>\
        </defs>",
    );

    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"url(#csbg)\"/>");

    let n_stars = ((w * h / 2600.0) as usize).clamp(90, 450);
    push_b(&mut buf, b"<g id=\"cs-stars\" pointer-events=\"none\">");
    let mut seed: u32 = 0x9e3779b9_u32;
    let mut lcg = || -> f64 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        (seed >> 8) as f64 / 16777216.0
    };
    for _ in 0..n_stars {
        let sx = lcg() * w;
        let sy = lcg() * h;
        let sr = lcg() * 0.9 + 0.15;
        let so = lcg() * 0.38 + 0.06;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, sx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, sr);
        push_b(&mut buf, b"\" fill=\"#ffffff\" fill-opacity=\"");
        push_f2(&mut buf, so);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"30\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#c8d5f0\" letter-spacing=\"1.5\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<g id=\"cs-edges\">");
    let has_edges = !cfg.edges_i.is_empty() && cfg.edges_i.len() == cfg.edges_j.len();
    if has_edges {
        let w_max = cfg.edges_w.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
        for k in 0..cfg.edges_i.len() {
            let i = cfg.edges_i[k] as usize;
            let j = cfg.edges_j[k] as usize;
            if i >= n || j >= n {
                continue;
            }
            let (x1, y1, _) = pos_by_orig[i];
            let (x2, y2, _) = pos_by_orig[j];
            let ww = if cfg.edges_w.is_empty() { 1.0 } else { cfg.edges_w[k] };
            let op = (ww / w_max * 0.38).clamp(0.04, 0.42);
            let sw = (ww / w_max * 1.2 + 0.25).clamp(0.25, 1.8);
            let col = palette_color(cfg.palette, i);
            let hx = hex6(col);
            let mx = (x1 + x2) / 2.0 + (y2 - y1) * 0.14;
            let my = (y1 + y2) / 2.0 + (x1 - x2) * 0.14;
            push_b(&mut buf, b"<path d=\"M ");
            push_f2(&mut buf, x1);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y1);
            push_b(&mut buf, b" Q ");
            push_f2(&mut buf, mx);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, my);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, x2);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y2);
            push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, sw);
            push_b(&mut buf, b"\" stroke-opacity=\"");
            push_f2(&mut buf, op);
            push_b(&mut buf, b"\"/>");
        }
    } else {
        for i in 0..n {
            let (x1, y1, _) = pos_by_orig[i];
            let col = palette_color(cfg.palette, i);
            let hx = hex6(col);
            let mut dists: Vec<(f64, usize)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| {
                    let dx = pos_by_orig[j].0 - x1;
                    let dy = pos_by_orig[j].1 - y1;
                    (dx * dx + dy * dy, j)
                })
                .collect();
            dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            for k in 0..2.min(dists.len()) {
                let j = dists[k].1;
                if j < i {
                    continue;
                }
                let (x2, y2, _) = pos_by_orig[j];
                let dist = dists[k].0.sqrt().max(1.0);
                let op = (0.32 - dist / (w.max(h) * 2.0)).clamp(0.03, 0.28);
                let mx = (x1 + x2) / 2.0 + (y2 - y1) * 0.12;
                let my = (y1 + y2) / 2.0 + (x1 - x2) * 0.12;
                push_b(&mut buf, b"<path d=\"M ");
                push_f2(&mut buf, x1);
                push_b(&mut buf, b" ");
                push_f2(&mut buf, y1);
                push_b(&mut buf, b" Q ");
                push_f2(&mut buf, mx);
                push_b(&mut buf, b" ");
                push_f2(&mut buf, my);
                push_b(&mut buf, b" ");
                push_f2(&mut buf, x2);
                push_b(&mut buf, b" ");
                push_f2(&mut buf, y2);
                push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke-width=\"0.45\" stroke-opacity=\"");
                push_f2(&mut buf, op);
                push_b(&mut buf, b"\"/>");
            }
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g id=\"cs-nodes\">");
    for orig in 0..n {
        let word = &cfg.words[orig];
        let freq = cfg.frequencies[orig];
        let pct = freq / total * 100.0;
        let (px, py, node_r) = pos_by_orig[orig];
        let color = palette_color(cfg.palette, orig);
        let hx = hex6(color);

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, node_r * 2.8);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.04\" filter=\"url(#cs-outer)\"/>");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, node_r * 1.5);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.12\" filter=\"url(#cs-mid)\"/>");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, node_r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.88\" filter=\"url(#cs-core)\"/>");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, (node_r * 0.22).max(1.0));
        push_b(&mut buf, b"\" fill=\"#ffffff\" fill-opacity=\"0.94\"/>");

        let font_sz = (node_r * 1.05 + 5.5).clamp(8.5, cfg.max_font * 0.8);
        push_b(&mut buf, b"<text data-idx=\"");
        push_i(&mut buf, orig as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, freq);
        push_b(&mut buf, b"\" data-kv-Pct=\"");
        push_f2(&mut buf, pct);
        buf.push(b'%');
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, py + node_r + font_sz + 2.0);
        push_b(
            &mut buf,
            b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-size=\"",
        );
        push_f2(&mut buf, font_sz);
        push_b(&mut buf, b"\" font-weight=\"600\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(
            &mut buf,
            b"\" fill-opacity=\"0.88\" style=\"cursor:pointer\">",
        );
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");
    push_b(&mut buf, b"</svg>");

    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    build_chart_html(cfg.title, &svg, json)
}
