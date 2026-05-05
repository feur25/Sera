use super::config::SunburstConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::{build_chart_html, slots_to_json};
use std::collections::HashMap;

pub const SB_PALETTE: &[u32] = &[
    0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6,
    0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6,
    0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
    0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF,
];

pub const FLAME_PALETTE: &[u32] = &[
    0xB91C1C, 0xEA580C, 0xF59E0B, 0xFBBF24, 0xFDE68A, 0xFEF3C7,
];

pub struct Layout {
    pub cx: i32,
    pub cy: i32,
    pub r_hole: i32,
    pub r_max: i32,
    pub ring_w: i32,
    pub max_depth: usize,
}

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values_eff: Vec<f64>,
    pub bfs_order: Vec<usize>,
    pub depth: Vec<usize>,
    pub ang: Vec<(f64, f64)>,
    pub cidx: Vec<usize>,
    pub roots: Vec<usize>,
    pub layout: Layout,
    pub palette: Vec<u32>,
    pub grand_total: f64,
}

pub fn prepare_with_hole(cfg: &SunburstConfig, hole_factor: f64) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values.len()).min(cfg.parents.len());
    if n == 0 { return None; }
    let labels: Vec<String> = cfg.labels[..n].to_vec();

    let mut label_idx: HashMap<&str, usize> = HashMap::with_capacity(n);
    for i in 0..n { label_idx.insert(labels[i].as_str(), i); }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut roots: Vec<usize> = Vec::new();
    for i in 0..n {
        let par = cfg.parents[i].as_str();
        if par.is_empty() { roots.push(i); }
        else if let Some(&pi) = label_idx.get(par) { children[pi].push(i); }
    }
    if roots.is_empty() { return None; }

    let mut sv: Vec<f64> = cfg.values[..n].to_vec();
    let mut stack: Vec<(usize, bool)> = roots.iter().map(|&r| (r, false)).collect();
    while let Some((i, visited)) = stack.pop() {
        if visited {
            let cs: f64 = children[i].iter().map(|&c| sv[c]).sum();
            if cs > sv[i] { sv[i] = cs; }
        } else {
            stack.push((i, true));
            for &c in children[i].iter().rev() { stack.push((c, false)); }
        }
    }

    let mut depth = vec![0usize; n];
    let mut bfs_order: Vec<usize> = Vec::with_capacity(n);
    bfs_order.extend_from_slice(&roots);
    let mut head = 0usize;
    while head < bfs_order.len() {
        let i = bfs_order[head]; head += 1;
        for &c in &children[i] { depth[c] = depth[i] + 1; bfs_order.push(c); }
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
        let ct: f64 = ch.iter().map(|&c| sv[c]).sum::<f64>().max(1e-12);
        let span = ang[i].1 - ang[i].0;
        let mut c_cur = ang[i].0;
        for &j in ch {
            let sweep = span * sv[j] / ct;
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
    let r_hole = (available * hole_factor) as i32;
    let r_max  = (available * 0.46) as i32;
    let ring_levels = (max_depth as i32 + 1).max(1);
    let ring_w = ((r_max - r_hole) / ring_levels).max(1);

    let palette: Vec<u32> = if cfg.palette.is_empty() { SB_PALETTE.to_vec() } else { cfg.palette.to_vec() };
    let grand_total: f64 = roots.iter().map(|&r| sv[r]).sum::<f64>().max(1e-12);

    Some(Prepared {
        n, labels, values_eff: sv, bfs_order, depth, ang, cidx, roots, palette, grand_total,
        layout: Layout { cx, cy, r_hole, r_max, ring_w, max_depth },
    })
}

pub fn prepare(cfg: &SunburstConfig) -> Option<Prepared> { prepare_with_hole(cfg, 0.09) }

pub fn open_svg(buf: &mut Vec<u8>, cfg: &SunburstConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, cfg.width); push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height); push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width); push_b(buf, b" ");
    push_i(buf, cfg.height); push_b(buf, b"\">");
    push_b(buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\""); push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn arc_path(buf: &mut Vec<u8>, cx: i32, cy: i32, r1: i32, r2: i32, a1: f64, a2: f64) {
    let pi = std::f64::consts::PI;
    let large = if a2 - a1 > pi { 1 } else { 0 };
    let (x1o, y1o) = (cx as f64 + r2 as f64 * a1.cos(), cy as f64 + r2 as f64 * a1.sin());
    let (x2o, y2o) = (cx as f64 + r2 as f64 * a2.cos(), cy as f64 + r2 as f64 * a2.sin());
    let (x1i, y1i) = (cx as f64 + r1 as f64 * a2.cos(), cy as f64 + r1 as f64 * a2.sin());
    let (x2i, y2i) = (cx as f64 + r1 as f64 * a1.cos(), cy as f64 + r1 as f64 * a1.sin());
    push_b(buf, b"M "); push_f2(buf, x1o); buf.push(b' '); push_f2(buf, y1o);
    push_b(buf, b" A "); push_i(buf, r2); buf.push(b' '); push_i(buf, r2);
    push_b(buf, b" 0 "); push_i(buf, large); push_b(buf, b" 1 ");
    push_f2(buf, x2o); buf.push(b' '); push_f2(buf, y2o);
    push_b(buf, b" L "); push_f2(buf, x1i); buf.push(b' '); push_f2(buf, y1i);
    push_b(buf, b" A "); push_i(buf, r1); buf.push(b' '); push_i(buf, r1);
    push_b(buf, b" 0 "); push_i(buf, large); push_b(buf, b" 0 ");
    push_f2(buf, x2i); buf.push(b' '); push_f2(buf, y2i);
    push_b(buf, b" Z");
}

pub fn slice_data_attrs(buf: &mut Vec<u8>, p: &Prepared, i: usize) {
    push_b(buf, b" data-idx=\""); push_i(buf, i as i32);
    push_b(buf, b"\" data-y=\""); push_f2(buf, p.values_eff[i]);
    push_b(buf, b"\" data-kv-pct=\""); push_f2(buf, p.values_eff[i] / p.grand_total * 100.0); push_b(buf, b"%");
    push_b(buf, b"\" data-lbl=\""); escape_xml(buf, &p.labels[i]); push_b(buf, b"\"");
}

pub fn label_arc(buf: &mut Vec<u8>, p: &Prepared, i: usize, white: bool) {
    let (a1, a2) = p.ang[i];
    let sweep = a2 - a1;
    let d = p.depth[i] as i32;
    let l = &p.layout;
    let r1 = l.r_hole + d * l.ring_w;
    let r_mid = r1 + l.ring_w / 2;
    let arc_len = sweep * r_mid as f64;
    if arc_len < 24.0 { return; }
    let mid = a1 + sweep / 2.0;
    let tx = l.cx as f64 + r_mid as f64 * mid.cos();
    let ty = l.cy as f64 + r_mid as f64 * mid.sin();
    let lbl = &p.labels[i];
    let max_ch = ((arc_len / 6.5) as usize).min(16).max(2);
    let n_chars = lbl.chars().count();
    let short: &str = if n_chars <= max_ch { lbl.as_str() } else {
        let end = lbl.char_indices().nth(max_ch).map(|(idx, _)| idx).unwrap_or(lbl.len());
        &lbl[..end]
    };
    let font_size: &[u8] = if l.ring_w > 28 { b"10" } else { b"8" };
    let fill: &[u8] = if white { b"#fff" } else { b"#1f2937" };
    push_b(buf, b"<text x=\""); push_f2(buf, tx);
    push_b(buf, b"\" y=\""); push_f2(buf, ty + 4.0);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
    buf.extend_from_slice(font_size);
    push_b(buf, b"\" fill=\""); buf.extend_from_slice(fill);
    push_b(buf, b"\" pointer-events=\"none\">");
    escape_xml(buf, short);
    push_b(buf, b"</text>");
}

pub fn ring_radii(p: &Prepared, i: usize) -> (i32, i32) {
    let l = &p.layout;
    let d = p.depth[i] as i32;
    let r1 = l.r_hole + d * l.ring_w;
    let r2 = (r1 + l.ring_w - 1).min(l.r_max);
    (r1, r2)
}

pub fn finalize(mut buf: Vec<u8>, cfg: &SunburstConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

pub fn color_hex(p: &Prepared, i: usize) -> [u8; 6] {
    hex6(p.palette[p.cidx[i] % p.palette.len()])
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> u32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = (h.rem_euclid(360.0)) / 60.0;
    let x = c * (1.0 - (hp.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match hp as i32 {
        0 => (c, x, 0.0), 1 => (x, c, 0.0), 2 => (0.0, c, x),
        3 => (0.0, x, c), 4 => (x, 0.0, c), _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    let bl = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u32;
    (r << 16) | (g << 8) | bl
}
