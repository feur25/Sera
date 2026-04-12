use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, apply_sort};
use crate::html::hover::{slots_to_json, build_chart_html};

pub struct Treemap;

crate::chart_config!(TreemapConfig, 1100, 520;
    struct {
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub parents: &'a [String],
        pub palette: &'a [u32],
    }
    defaults {
        labels: &[],
        values: &[],
        parents: &[],
        palette: &[],
    }
);

#[derive(Clone)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn squarify(values: &[f64], rect: &Rect) -> Vec<Rect> {
    let n = values.len();
    if n == 0 { return vec![]; }
    if n == 1 {
        return vec![Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h }];
    }
    let total: f64 = values.iter().sum();
    if total <= 0.0 {
        return values.iter().map(|_| Rect { x: rect.x, y: rect.y, w: 0.0, h: 0.0 }).collect();
    }
    let areas: Vec<f64> = values.iter().map(|v| v / total * rect.w * rect.h).collect();
    let mut rects = vec![Rect { x: 0.0, y: 0.0, w: 0.0, h: 0.0 }; n];
    layout_squarify(&areas, rect, &mut rects, 0, n);
    rects
}

fn layout_squarify(areas: &[f64], rect: &Rect, out: &mut [Rect], start: usize, end: usize) {
    let count = end - start;
    if count == 0 { return; }
    if count == 1 {
        out[start] = Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h };
        return;
    }
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
        let cnt = (i - start + 1) as f64;
        let max_in_row = areas[start..=i].iter().copied().fold(0.0_f64, f64::max);
        let min_in_row = areas[start..=i].iter().copied().fold(f64::INFINITY, f64::min);
        let r1 = (other_side * other_side * cnt) / row_area;
        let r2 = row_area / (other_side * other_side);
        let ratio = if max_in_row > 0.0 {
            let a = (full_side * full_side * max_in_row) / (row_area * row_area);
            let b = (row_area * row_area) / (full_side * full_side * min_in_row.max(1e-12));
            a.max(b)
        } else {
            r1.max(r2)
        };
        if ratio <= best_ratio {
            best_ratio = ratio;
            split = i + 1;
        } else {
            break;
        }
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
        layout_squarify(areas, &rest, out, split, end);
    } else {
        let row_h = rect.h * frac;
        let mut cx = rect.x;
        for i in start..split {
            let w = if row_total > 0.0 { areas[i] / row_total * rect.w } else { 0.0 };
            out[i] = Rect { x: cx, y: rect.y, w, h: row_h };
            cx += w;
        }
        let rest = Rect { x: rect.x, y: rect.y + row_h, w: rect.w, h: rect.h - row_h };
        layout_squarify(areas, &rest, out, split, end);
    }
}

pub fn render_treemap_html(cfg: &TreemapConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }
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
    } else {
        cfg.parents.to_vec()
    };
    let parents = &sorted_parents;
    let labels = &sorted_labels;
    let values = &sorted_values;
    let (leaf_indices, leaf_values): (Vec<usize>, Vec<f64>) = if has_parents {
        (0..n).filter(|&i| values[i] > 0.0 && !parents[i].is_empty())
            .map(|i| (i, values[i]))
            .unzip()
    } else {
        (0..n).filter(|&i| values[i] > 0.0)
            .map(|i| (i, values[i]))
            .unzip()
    };
    let leaf_n = leaf_indices.len();
    if leaf_n == 0 { return String::new(); }
    let pad_t: i32 = if cfg.title.is_empty() { 8 } else { 38 };
    let pad = 4;
    let plot_w = (cfg.width - pad * 2) as f64;
    let plot_h = (cfg.height - pad_t - pad) as f64;
    let root = Rect { x: pad as f64, y: pad_t as f64, w: plot_w, h: plot_h };
    let rects = squarify(&leaf_values, &root);
    let auto_hover = cfg.hover.is_empty();
    let total: f64 = leaf_values.iter().sum();
    let mut buf = Vec::<u8>::with_capacity(leaf_n * 200 + 2048);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    let mut parent_color_map: Vec<(String, u32)> = Vec::new();
    for (ri, &oi) in leaf_indices.iter().enumerate() {
        let r = &rects[ri];
        if r.w < 0.5 || r.h < 0.5 { continue; }
        let color = if has_parents {
            let parent = &parents[oi];
            if let Some(pos) = parent_color_map.iter().position(|(p, _)| p == parent) {
                parent_color_map[pos].1
            } else {
                let c = palette_color(cfg.palette, parent_color_map.len());
                parent_color_map.push((parent.clone(), c));
                c
            }
        } else {
            palette_color(cfg.palette, ri)
        };
        let label = &labels[oi];
        let pct = if total > 0.0 { values[oi] / total * 100.0 } else { 0.0 };
        push_b(&mut buf, b"<rect data-idx=\"");
        push_i(&mut buf, ri as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &labels[oi]);
        push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, values[oi]);
        push_b(&mut buf, b"\" data-kv-Pct=\""); push_f2(&mut buf, pct); buf.push(b'%');
        if has_parents {
            push_b(&mut buf, b"\" data-kv-Groupe=\""); escape_xml(&mut buf, &parents[oi]);
        }
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, r.x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, r.y);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, r.w);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, r.h);
        push_b(&mut buf, b"\" rx=\"4\" fill=\"#");
        let hx = hex6(color);
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"2\"/>");
        let max_chars = ((r.w / 7.0) as usize).max(1);
        if r.w > 30.0 && r.h > 18.0 {
            let font_size: i32 = if r.w > 100.0 && r.h > 40.0 { 12 }
                else if r.w > 60.0 && r.h > 28.0 { 10 }
                else { 8 };
            let cx = r.x + r.w / 2.0;
            let cy = r.y + r.h / 2.0;
            if r.h > 36.0 {
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, cx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
                push_i(&mut buf, font_size);
                push_b(&mut buf, b"\" font-weight=\"600\" fill=\"#fff\" dy=\"-0.3em\">");
                escape_xml(&mut buf, truncate(label, max_chars));
                push_b(&mut buf, b"</text>");
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, cx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
                push_i(&mut buf, font_size - 1);
                push_b(&mut buf, b"\" fill=\"rgba(255,255,255,0.8)\" dy=\"1.0em\">");
                push_f2(&mut buf, pct);
                push_b(&mut buf, b"%</text>");
            } else {
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, cx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
                push_i(&mut buf, font_size);
                push_b(&mut buf, b"\" font-weight=\"600\" fill=\"#fff\" dy=\"0.35em\">");
                escape_xml(&mut buf, truncate(label, max_chars));
                push_b(&mut buf, b"</text>");
            }
        }
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}
