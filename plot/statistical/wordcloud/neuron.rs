use super::config::WordCloudConfig;
use super::common::{prepare, place_words};
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::{slots_to_json, build_chart_html};

pub fn render(cfg: &WordCloudConfig) -> String {
    let p = match prepare(cfg) { Some(p) => p, None => return String::new() };

    let test = |x: f64, y: f64, w: f64, h: f64, _cx: f64, _cy: f64, _rx: f64, _ry: f64| -> bool {
        let _ = (x, y, w, h);
        true
    };
    let placed = place_words(&p.words, &p.sizes, cfg.width as f64, cfg.height as f64, p.pad_t, &test);

    let n_placed = placed.len();
    let nodes: Vec<(f64, f64, usize)> = placed.iter().map(|pw| {
        let cx = pw.x + pw.w / 2.0;
        let cy = pw.y + pw.h / 2.0;
        (cx, cy, pw.idx)
    }).collect();

    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let mut buf = Vec::<u8>::with_capacity(n_placed * 800 + 4096);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#080d1a\"/>");

    push_b(&mut buf, b"<defs>");
    push_b(&mut buf, b"<filter id=\"nrngf\" x=\"-60%\" y=\"-60%\" width=\"320%\" height=\"320%\">");
    push_b(&mut buf, b"<feGaussianBlur stdDeviation=\"4\" result=\"b\"/>");
    push_b(&mut buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut buf, b"</filter>");
    push_b(&mut buf, b"<filter id=\"nrnwf\" x=\"-80%\" y=\"-80%\" width=\"360%\" height=\"360%\">");
    push_b(&mut buf, b"<feGaussianBlur stdDeviation=\"2.5\" result=\"b\"/>");
    push_b(&mut buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut buf, b"</filter>");
    push_b(&mut buf, b"</defs>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#e2e8f0\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<g id=\"nrn-edges\">");
    for i in 0..n_placed {
        let (cx_i, cy_i, idx_i) = nodes[i];
        let col_i = palette_color(cfg.palette, idx_i);
        let hx = hex6(col_i);

        let mut dists: Vec<(f64, usize)> = (0..n_placed)
            .filter(|&j| j != i)
            .map(|j| {
                let dx = nodes[j].0 - cx_i;
                let dy = nodes[j].1 - cy_i;
                (dx * dx + dy * dy, j)
            })
            .collect();
        dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let n_edges = 2.min(dists.len());
        for k in 0..n_edges {
            let j = dists[k].1;
            let (cx_j, cy_j, _) = nodes[j];
            let dist = dists[k].0.sqrt().max(1.0);
            let opacity = (1.0 - (dist / (w.max(h) * 0.45))).clamp(0.04, 0.22);

            push_b(&mut buf, b"<line x1=\"");
            push_f2(&mut buf, cx_i);
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, cy_i);
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, cx_j);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, cy_j);
            push_b(&mut buf, b"\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"0.6\" stroke-opacity=\"");
            push_f2(&mut buf, opacity);
            push_b(&mut buf, b"\"/>");
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g id=\"nrn-nodes\">");
    for pw in &placed {
        let word = &p.words[pw.idx];
        let freq = p.freqs[pw.idx];
        let color = palette_color(cfg.palette, p.orig_idx[pw.idx]);
        let pct = if p.total > 0.0 { freq / p.total * 100.0 } else { 0.0 };
        let cx = pw.x + pw.w / 2.0;
        let cy = pw.y + pw.h / 2.0;
        let node_r = (pw.font_size * 0.55).max(4.0);
        let hx = hex6(color);

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, node_r * 1.6);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.07\" filter=\"url(#nrngf)\"/>");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, node_r * 0.35);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.9\" filter=\"url(#nrnwf)\"/>");

        push_b(&mut buf, b"<text data-idx=\"");
        push_i(&mut buf, pw.idx as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, word);
        push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, freq);
        push_b(&mut buf, b"\" data-kv-Pct=\""); push_f2(&mut buf, pct); buf.push(b'%');
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + pw.font_size * 0.36);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"'Segoe UI',Arial,sans-serif\" font-size=\"");
        push_f2(&mut buf, pw.font_size);
        push_b(&mut buf, b"\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" filter=\"url(#nrnwf)\" style=\"cursor:pointer\">");
        escape_xml(&mut buf, word);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"</svg>");

    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}


