use super::config::TreemapConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, apply_sort};
use crate::html::hover::{slots_to_json, build_chart_html};

pub const TM_PALETTE: &[u32] = &[
    0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6,
    0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6,
    0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA,
];

#[derive(Clone, Copy)]
pub struct Rect { pub x: f64, pub y: f64, pub w: f64, pub h: f64 }

pub struct Prepared {
    pub n_total: usize,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub parents: Vec<String>,
    pub has_parents: bool,
    pub leaf_indices: Vec<usize>,
    pub leaf_values: Vec<f64>,
    pub rects: Vec<Rect>,
    pub root: Rect,
    pub total: f64,
    pub palette: Vec<u32>,
    pub parent_color: std::collections::HashMap<String, u32>,
    pub parent_groups: Vec<(String, Rect)>,
}

pub fn prepare_with_pad(cfg: &TreemapConfig, inner_pad: f64) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return None; }
    let (sorted_labels, sorted_values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let has_parents = cfg.parents.len() >= n && cfg.parents.iter().any(|p| !p.is_empty());
    let sorted_parents: Vec<String> = if has_parents && !cfg.sort_order.is_empty() && cfg.sort_order != "none" {
        let n2 = cfg.labels.len().min(cfg.values.len());
        let mut idx: Vec<usize> = (0..n2).collect();
        match cfg.sort_order {
            "asc" | "ascending" => idx.sort_by(|&a, &b| cfg.values[a].partial_cmp(&cfg.values[b]).unwrap_or(std::cmp::Ordering::Equal)),
            "desc" | "descending" => idx.sort_by(|&a, &b| cfg.values[b].partial_cmp(&cfg.values[a]).unwrap_or(std::cmp::Ordering::Equal)),
            "alpha" | "alphabetical" => idx.sort_by(|&a, &b| cfg.labels[a].cmp(&cfg.labels[b])),
            "alpha_desc" => idx.sort_by(|&a, &b| cfg.labels[b].cmp(&cfg.labels[a])),
            _ => {}
        }
        idx.iter().map(|&i| if i < cfg.parents.len() { cfg.parents[i].clone() } else { String::new() }).collect()
    } else if cfg.parents.len() >= n {
        cfg.parents[..n].to_vec()
    } else {
        vec![String::new(); n]
    };

    let (leaf_indices, leaf_values): (Vec<usize>, Vec<f64>) = if has_parents {
        (0..n).filter(|&i| sorted_values[i] > 0.0 && !sorted_parents[i].is_empty())
            .map(|i| (i, sorted_values[i])).unzip()
    } else {
        (0..n).filter(|&i| sorted_values[i] > 0.0).map(|i| (i, sorted_values[i])).unzip()
    };
    if leaf_indices.is_empty() { return None; }

    let pad_t: i32 = if cfg.title.is_empty() { 8 } else { 38 };
    let pad = 4.0;
    let plot_w = cfg.width as f64 - pad * 2.0;
    let plot_h = cfg.height as f64 - pad_t as f64 - pad;
    let root = Rect { x: pad, y: pad_t as f64, w: plot_w, h: plot_h };

    let total: f64 = leaf_values.iter().sum();
    let palette: Vec<u32> = if cfg.palette.is_empty() { TM_PALETTE.to_vec() } else { cfg.palette.to_vec() };

    let mut parent_color: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let mut parent_groups: Vec<(String, Rect)> = Vec::new();

    let rects: Vec<Rect> = if has_parents {
        let mut groups: Vec<(String, Vec<usize>, Vec<f64>)> = Vec::new();
        for (li, &oi) in leaf_indices.iter().enumerate() {
            let p = sorted_parents[oi].clone();
            if let Some(g) = groups.iter_mut().find(|g| g.0 == p) {
                g.1.push(li); g.2.push(leaf_values[li]);
            } else {
                groups.push((p, vec![li], vec![leaf_values[li]]));
            }
        }
        let group_totals: Vec<f64> = groups.iter().map(|g| g.2.iter().sum()).collect();
        let group_rects = squarify_into(&group_totals, root);
        let mut rects: Vec<Rect> = vec![Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }; leaf_indices.len()];
        for (gi, g) in groups.iter().enumerate() {
            let mut gr = group_rects[gi];
            if inner_pad > 0.0 && gr.w > inner_pad * 2.0 + 2.0 && gr.h > inner_pad * 2.0 + 2.0 {
                gr.x += inner_pad; gr.y += inner_pad;
                gr.w -= inner_pad * 2.0; gr.h -= inner_pad * 2.0;
            }
            parent_groups.push((g.0.clone(), gr));
            let local = squarify_into(&g.2, gr);
            for (k, &li) in g.1.iter().enumerate() { rects[li] = local[k]; }
            let pi = parent_color.len();
            parent_color.entry(g.0.clone()).or_insert_with(|| palette_color(&palette, pi));
        }
        rects
    } else {
        squarify_into(&leaf_values, root)
    };

    let parents_out = sorted_parents;
    Some(Prepared {
        n_total: n,
        labels: sorted_labels,
        values: sorted_values,
        parents: parents_out,
        has_parents,
        leaf_indices,
        leaf_values,
        rects,
        root,
        total,
        palette,
        parent_color,
        parent_groups,
    })
}

pub fn prepare(cfg: &TreemapConfig) -> Option<Prepared> { prepare_with_pad(cfg, 0.0) }

pub fn leaf_color(p: &Prepared, ri: usize) -> u32 {
    if p.has_parents {
        let oi = p.leaf_indices[ri];
        let par = &p.parents[oi];
        *p.parent_color.get(par).unwrap_or(&p.palette[0])
    } else {
        palette_color(&p.palette, ri)
    }
}

pub fn squarify_into(values: &[f64], rect: Rect) -> Vec<Rect> {
    let n = values.len();
    if n == 0 { return vec![]; }
    if n == 1 { return vec![rect]; }
    let total: f64 = values.iter().sum();
    if total <= 0.0 { return values.iter().map(|_| Rect { x: rect.x, y: rect.y, w: 0.0, h: 0.0 }).collect(); }
    let areas: Vec<f64> = values.iter().map(|v| v / total * rect.w * rect.h).collect();
    let mut out = vec![Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }; n];
    layout(&areas, &rect, &mut out, 0, n);
    out
}

fn layout(areas: &[f64], rect: &Rect, out: &mut [Rect], start: usize, end: usize) {
    let count = end - start;
    if count == 0 { return; }
    if count == 1 { out[start] = *rect; return; }
    let total_area: f64 = areas[start..end].iter().sum();
    if total_area <= 0.0 { return; }
    let vertical = rect.w >= rect.h;
    let full_side = if vertical { rect.h } else { rect.w };
    let mut row_area = 0.0;
    let mut best_ratio = f64::INFINITY;
    let mut split = start + 1;
    for i in start..end {
        row_area += areas[i];
        let other_side = row_area / full_side;
        if other_side <= 0.0 { continue; }
        let max_in_row = areas[start..=i].iter().copied().fold(0.0_f64, f64::max);
        let min_in_row = areas[start..=i].iter().copied().fold(f64::INFINITY, f64::min);
        let ratio = if max_in_row > 0.0 {
            let a = (full_side * full_side * max_in_row) / (row_area * row_area);
            let b = (row_area * row_area) / (full_side * full_side * min_in_row.max(1e-12));
            a.max(b)
        } else { f64::INFINITY };
        if ratio <= best_ratio { best_ratio = ratio; split = i + 1; } else { break; }
    }
    if split == start { split = start + 1; }
    let row_total: f64 = areas[start..split].iter().sum();
    let frac = row_total / total_area;
    if vertical {
        let row_w = rect.w * frac;
        let mut cy = rect.y;
        for i in start..split {
            let h = if row_total > 0.0 { areas[i] / row_total * rect.h } else { 0.0 };
            out[i] = Rect { x: rect.x, y: cy, w: row_w, h };
            cy += h;
        }
        let rest = Rect { x: rect.x + row_w, y: rect.y, w: rect.w - row_w, h: rect.h };
        layout(areas, &rest, out, split, end);
    } else {
        let row_h = rect.h * frac;
        let mut cx = rect.x;
        for i in start..split {
            let w = if row_total > 0.0 { areas[i] / row_total * rect.w } else { 0.0 };
            out[i] = Rect { x: cx, y: rect.y, w, h: row_h };
            cx += w;
        }
        let rest = Rect { x: rect.x, y: rect.y + row_h, w: rect.w, h: rect.h - row_h };
        layout(areas, &rest, out, split, end);
    }
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &TreemapConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, cfg.width); push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height); push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width); push_b(buf, b" ");
    push_i(buf, cfg.height); push_b(buf, b"\">");
    push_b(buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\""); push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn tile_data_attrs(buf: &mut Vec<u8>, p: &Prepared, ri: usize) {
    let oi = p.leaf_indices[ri];
    let pct = if p.total > 0.0 { p.leaf_values[ri] / p.total * 100.0 } else { 0.0 };
    push_b(buf, b" data-idx=\""); push_i(buf, ri as i32);
    push_b(buf, b"\" data-lbl=\""); escape_xml(buf, &p.labels[oi]);
    push_b(buf, b"\" data-v=\""); push_f2(buf, p.leaf_values[ri]);
    push_b(buf, b"\" data-kv-Pct=\""); push_f2(buf, pct); buf.push(b'%');
    if p.has_parents {
        push_b(buf, b"\" data-kv-Group=\""); escape_xml(buf, &p.parents[oi]);
    }
    push_b(buf, b"\"");
}

pub fn label_inside(buf: &mut Vec<u8>, p: &Prepared, ri: usize, fill: &[u8]) {
    let r = p.rects[ri];
    if r.w < 30.0 || r.h < 18.0 { return; }
    let oi = p.leaf_indices[ri];
    let label = &p.labels[oi];
    let pct = if p.total > 0.0 { p.leaf_values[ri] / p.total * 100.0 } else { 0.0 };
    let max_chars = ((r.w / 7.0) as usize).max(1);
    let font_size: i32 = if r.w > 100.0 && r.h > 40.0 { 12 }
        else if r.w > 60.0 && r.h > 28.0 { 10 } else { 8 };
    let cx = r.x + r.w / 2.0;
    let cy = r.y + r.h / 2.0;
    if r.h > 36.0 {
        push_b(buf, b"<text x=\""); push_f2(buf, cx);
        push_b(buf, b"\" y=\""); push_f2(buf, cy);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_i(buf, font_size);
        push_b(buf, b"\" font-weight=\"600\" fill=\""); buf.extend_from_slice(fill);
        push_b(buf, b"\" dy=\"-0.3em\" pointer-events=\"none\">");
        escape_xml(buf, truncate(label, max_chars));
        push_b(buf, b"</text>");
        push_b(buf, b"<text x=\""); push_f2(buf, cx);
        push_b(buf, b"\" y=\""); push_f2(buf, cy);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_i(buf, font_size - 1);
        push_b(buf, b"\" fill=\""); buf.extend_from_slice(fill);
        push_b(buf, b"\" opacity=\"0.78\" dy=\"1.0em\" pointer-events=\"none\">");
        push_f2(buf, pct);
        push_b(buf, b"%</text>");
    } else {
        push_b(buf, b"<text x=\""); push_f2(buf, cx);
        push_b(buf, b"\" y=\""); push_f2(buf, cy);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
        push_i(buf, font_size);
        push_b(buf, b"\" font-weight=\"600\" fill=\""); buf.extend_from_slice(fill);
        push_b(buf, b"\" dy=\"0.35em\" pointer-events=\"none\">");
        escape_xml(buf, truncate(label, max_chars));
        push_b(buf, b"</text>");
    }
}

pub fn finalize(buf: Vec<u8>, cfg: &TreemapConfig) -> String {
    let mut b = buf;
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}

pub fn rect_attrs(buf: &mut Vec<u8>, r: Rect) {
    push_b(buf, b" x=\""); push_f2(buf, r.x);
    push_b(buf, b"\" y=\""); push_f2(buf, r.y);
    push_b(buf, b"\" width=\""); push_f2(buf, r.w);
    push_b(buf, b"\" height=\""); push_f2(buf, r.h);
}

pub fn fill_hex(buf: &mut Vec<u8>, color: u32) {
    let hx = hex6(color);
    push_b(buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
}

pub fn lerp_color(a: u32, b: u32, t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let ar = ((a >> 16) & 0xFF) as f64; let ag = ((a >> 8) & 0xFF) as f64; let ab = (a & 0xFF) as f64;
    let br = ((b >> 16) & 0xFF) as f64; let bg = ((b >> 8) & 0xFF) as f64; let bb = (b & 0xFF) as f64;
    let r = (ar + (br - ar) * t) as u32;
    let g = (ag + (bg - ag) * t) as u32;
    let bl = (ab + (bb - ab) * t) as u32;
    (r << 16) | (g << 8) | bl
}
