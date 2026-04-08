use super::common::{push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::build_chart_html;
use std::collections::HashMap;

pub struct SunburstConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub parents: &'a [String],
    pub values: &'a [f64],
    pub width: i32,
    pub height: i32,
}

impl<'a> Default for SunburstConfig<'a> {
    fn default() -> Self {
        Self { title: "", labels: &[], parents: &[], values: &[], width: 700, height: 700 }
    }
}

const SB_PALETTE: &[u32] = &[
    0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6,
    0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6,
    0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
    0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF,
];

pub fn render_sunburst_html(cfg: &SunburstConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len()).min(cfg.parents.len());
    if n == 0 { return String::new(); }

    let mut label_idx: HashMap<&str, usize> = HashMap::with_capacity(n);
    for i in 0..n {
        label_idx.insert(cfg.labels[i].as_str(), i);
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut roots: Vec<usize> = Vec::new();
    for i in 0..n {
        let par = cfg.parents[i].as_str();
        if par.is_empty() {
            roots.push(i);
        } else if let Some(&pi) = label_idx.get(par) {
            children[pi].push(i);
        }
    }
    if roots.is_empty() { return String::new(); }

    let mut sv: Vec<f64> = cfg.values[..n].to_vec();
    let mut stack: Vec<(usize, bool)> = roots.iter().map(|&r| (r, false)).collect();
    
    while let Some((i, visited)) = stack.pop() {
        if visited {
            let child_sum: f64 = children[i].iter().map(|&c| sv[c]).sum();
            if child_sum > sv[i] { sv[i] = child_sum; }
        } else {
            stack.push((i, true));
            for &c in children[i].iter().rev() {
                stack.push((c, false));
            }
        }
    }

    let mut depth = vec![0usize; n];
    let mut bfs_order: Vec<usize> = Vec::with_capacity(n);
    let mut head = 0usize;
    bfs_order.extend_from_slice(&roots);
    while head < bfs_order.len() {
        let i = bfs_order[head]; head += 1;
        for &c in &children[i] {
            depth[c] = depth[i] + 1;
            bfs_order.push(c);
        }
    }
    let max_depth = bfs_order.iter().map(|&i| depth[i]).max().unwrap_or(0);

    let pi2 = 2.0 * std::f64::consts::PI;
    let start0 = -std::f64::consts::PI / 2.0;
    let mut ang: Vec<(f64, f64)> = vec![(0.0, 0.0); n];

    let root_total: f64 = roots.iter().map(|&r| sv[r]).sum::<f64>().max(1e-12);
    let mut cursor = start0;
    for &r in &roots {
        let sweep = pi2 * sv[r] / root_total;
        ang[r] = (cursor, cursor + sweep);
        cursor += sweep;
    }

    for &i in &bfs_order {
        let ch = &children[i];
        if ch.is_empty() { continue; }
        let child_total: f64 = ch.iter().map(|&c| sv[c]).sum::<f64>().max(1e-12);
        let parent_span = ang[i].1 - ang[i].0;
        let mut c_cur = ang[i].0;
        for &j in ch {
            let sweep = parent_span * sv[j] / child_total;
            ang[j] = (c_cur, c_cur + sweep);
            c_cur += sweep;
        }
    }

    let mut cidx: Vec<usize> = vec![0usize; n];
    for (ri, &r) in roots.iter().enumerate() {
        let mut stk: Vec<usize> = vec![r];
        while let Some(i) = stk.pop() {
            cidx[i] = ri;
            for &c in &children[i] { stk.push(c); }
        }
    }

    let cx = cfg.width / 2;
    let title_h = if cfg.title.is_empty() { 0i32 } else { 28 };
    let cy = (cfg.height + title_h) / 2;
    let available = cfg.width.min(cfg.height - title_h) as f64;
    let r_hole = (available * 0.09) as i32;
    let r_max  = (available * 0.46) as i32;
    let ring_levels = (max_depth as i32 + 1).max(1);
    let ring_w = ((r_max - r_hole) / ring_levels).max(1);

    let pi = std::f64::consts::PI;
    let arc = |r1: i32, r2: i32, a1: f64, a2: f64, buf: &mut Vec<u8>| {
        let large = if a2 - a1 > pi { 1 } else { 0 };
        let (x1o, y1o) = (cx as f64 + r2 as f64 * a1.cos(), cy as f64 + r2 as f64 * a1.sin());
        let (x2o, y2o) = (cx as f64 + r2 as f64 * a2.cos(), cy as f64 + r2 as f64 * a2.sin());
        let (x1i, y1i) = (cx as f64 + r1 as f64 * a2.cos(), cy as f64 + r1 as f64 * a2.sin());
        let (x2i, y2i) = (cx as f64 + r1 as f64 * a1.cos(), cy as f64 + r1 as f64 * a1.sin());
        buf.extend_from_slice(b"M "); push_f2(buf, x1o); buf.push(b' '); push_f2(buf, y1o);
        buf.extend_from_slice(b" A "); push_i(buf, r2); buf.push(b' '); push_i(buf, r2);
        buf.extend_from_slice(b" 0 "); push_i(buf, large); buf.extend_from_slice(b" 1 ");
        push_f2(buf, x2o); buf.push(b' '); push_f2(buf, y2o);
        buf.extend_from_slice(b" L ");
        push_f2(buf, x1i); buf.push(b' '); push_f2(buf, y1i);
        buf.extend_from_slice(b" A "); push_i(buf, r1); buf.push(b' '); push_i(buf, r1);
        buf.extend_from_slice(b" 0 "); push_i(buf, large); buf.extend_from_slice(b" 0 ");
        push_f2(buf, x2i); buf.push(b' '); push_f2(buf, y2i);
        buf.extend_from_slice(b" Z");
    };

    let grand_total: f64 = roots.iter().map(|&r| sv[r]).sum::<f64>().max(1e-12);

    let mut b = Vec::<u8>::with_capacity(n * 320 + 1536);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title); push_b(&mut b, b"</text>");
    }

    for &i in &bfs_order {
        let (a1, a2) = ang[i];
        let sweep = a2 - a1;
        if sweep < 1e-4 { continue; }
        let d = depth[i] as i32;
        let r1 = r_hole + d * ring_w;
        let r2 = (r1 + ring_w - 1).min(r_max);
        let hx = hex6(SB_PALETTE[cidx[i] % SB_PALETTE.len()]);

        let opacity: &[u8] = match d { 0 => b"0.92", 1 => b"0.76", 2 => b"0.66", _ => b"0.56" };

        let pct = sv[i] / grand_total * 100.0;

        push_b(&mut b, b"<path data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, sv[i]);
        push_b(&mut b, b"\" data-kv-pct=\""); push_f2(&mut b, pct); push_b(&mut b, b"%");
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &cfg.labels[i]);
        push_b(&mut b, b"\" d=\"");
        arc(r1, r2, a1, a2, &mut b);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\""); b.extend_from_slice(opacity);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"0.8\"/>");

        let r_mid = r1 + ring_w / 2;
        let arc_len = sweep * r_mid as f64;
        if arc_len > 24.0 {
            let mid = a1 + sweep / 2.0;
            let tr = r_mid;
            let (tx, ty) = (cx as f64 + tr as f64 * mid.cos(), cy as f64 + tr as f64 * mid.sin());
            let lbl = &cfg.labels[i];

            let max_ch = ((arc_len / 6.5) as usize).min(16).max(2);
            let short: &str = {
                let n_chars = lbl.chars().count();
                if n_chars <= max_ch {
                    lbl.as_str()
                } else {
                    let end = lbl.char_indices()
                        .nth(max_ch)
                        .map(|(idx, _)| idx)
                        .unwrap_or(lbl.len());
                    &lbl[..end]
                }
            };
            let font_size: &[u8] = if ring_w > 28 { b"10" } else { b"8" };
            push_b(&mut b, b"<text x=\""); push_f2(&mut b, tx);
            push_b(&mut b, b"\" y=\""); push_f2(&mut b, ty + 4.0);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
            b.extend_from_slice(font_size);
            push_b(&mut b, b"\" fill=\"#fff\" pointer-events=\"none\">");
            escape_xml(&mut b, short);
            push_b(&mut b, b"</text>");
        }
    }

    push_b(&mut b, b"<circle cx=\""); push_i(&mut b, cx); push_b(&mut b, b"\" cy=\"");
    push_i(&mut b, cy); push_b(&mut b, b"\" r=\""); push_i(&mut b, r_hole - 1);
    push_b(&mut b, b"\" fill=\"#fff\"/>");
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
