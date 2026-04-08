use super::common::{push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::build_chart_html;

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
        Self {
            title: "",
            labels: &[],
            parents: &[],
            values: &[],
            width: 700,
            height: 700,
        }
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
    let cx = cfg.width / 2;
    let cy = (cfg.height + if cfg.title.is_empty() { 0 } else { 28 }) / 2;
    let r_inner = (cfg.width.min(cfg.height) as f64 * 0.18) as i32;
    let r_mid   = (cfg.width.min(cfg.height) as f64 * 0.34) as i32;
    let r_outer = (cfg.width.min(cfg.height) as f64 * 0.47) as i32;

    let root_labels: Vec<&str> = cfg.labels[..n].iter()
        .zip(cfg.parents[..n].iter())
        .filter_map(|(l, p)| if p.is_empty() { Some(l.as_str()) } else { None })
        .collect();
    let root_total: f64 = root_labels.iter()
        .map(|rl| {
            cfg.labels[..n].iter().zip(cfg.values[..n].iter())
                .filter(|(l, _)| l.as_str() == *rl)
                .map(|(_, v)| *v)
                .sum::<f64>()
            + cfg.labels[..n].iter().enumerate()
                .filter(|(_, l)| l.as_str() == *rl)
                .map(|(i, _)| {
                    let parent_label = cfg.labels[i].as_str();
                    cfg.labels[..n].iter().zip(cfg.parents[..n].iter()).zip(cfg.values[..n].iter())
                        .filter(|((_, p), _)| p.as_str() == parent_label)
                        .map(|(_, v)| *v)
                        .sum::<f64>()
                })
                .sum::<f64>()
        })
        .sum::<f64>()
        .max(1.0);

    let leaf_total: f64 = cfg.labels[..n].iter()
        .zip(cfg.parents[..n].iter())
        .zip(cfg.values[..n].iter())
        .filter(|((_, p), _)| !p.is_empty())
        .filter(|((l, _), _)| {
            !cfg.parents[..n].iter().any(|pp| pp == *l)
        })
        .map(|(_, v)| *v)
        .sum::<f64>()
        .max(1.0);

    let _ = root_total;

    let mut b = Vec::<u8>::with_capacity(n * 300 + 2048);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    struct Node {
        label: String,
        value: f64,
        parent: String,
        color_idx: usize,
        start_angle: f64,
        sweep_angle: f64,
    }

    let mut all_nodes: Vec<Node> = Vec::with_capacity(n);
    let mut root_nodes: Vec<(String, f64)> = Vec::new();
    for i in 0..n {
        if cfg.parents[i].is_empty() {
            let leaf_sum: f64 = cfg.labels[..n].iter()
                .zip(cfg.parents[..n].iter())
                .zip(cfg.values[..n].iter())
                .filter(|((_, p), _)| p.as_str() == cfg.labels[i].as_str())
                .map(|(_, v)| *v)
                .sum();
            let own_val = cfg.values[i];
            root_nodes.push((cfg.labels[i].clone(), leaf_sum.max(own_val)));
        }
    }
    let total_root: f64 = root_nodes.iter().map(|(_, v)| *v).sum::<f64>().max(1.0);
    let mut angle_cursor = -std::f64::consts::PI / 2.0;
    for (ri, (root_lbl, root_val)) in root_nodes.iter().enumerate() {
        let root_sweep = 2.0 * std::f64::consts::PI * root_val / total_root;
        all_nodes.push(Node {
            label: root_lbl.clone(),
            value: *root_val,
            parent: String::new(),
            color_idx: ri,
            start_angle: angle_cursor,
            sweep_angle: root_sweep,
        });
        let mut child_cursor = angle_cursor;
        let children: Vec<(usize, &String, &f64)> = cfg.labels[..n].iter()
            .zip(cfg.parents[..n].iter())
            .zip(cfg.values[..n].iter())
            .enumerate()
            .filter(|(_, ((_, p), _))| p.as_str() == root_lbl.as_str())
            .map(|(i, ((l, _), v))| (i, l, v))
            .collect();
        let child_total: f64 = children.iter().map(|(_, _, v)| **v).sum::<f64>().max(1.0);
        for (ci, (_, child_lbl, child_val)) in children.iter().enumerate() {
            let child_sweep = root_sweep * *child_val / child_total;
            let _ = ci;
            all_nodes.push(Node {
                label: (*child_lbl).clone(),
                value: **child_val,
                parent: root_lbl.clone(),
                color_idx: ri,
                start_angle: child_cursor,
                sweep_angle: child_sweep,
            });
            child_cursor += child_sweep;
        }
        angle_cursor += root_sweep;
    }

    let pi = std::f64::consts::PI;
    let deg_to_f = |a: f64| (a * 180.0 / pi);
    let _ = deg_to_f;

    let arc_path = |cx: i32, cy: i32, r1: i32, r2: i32, a1: f64, a2: f64| -> Vec<u8> {
        let mut p = Vec::with_capacity(120);
        let sweep = a2 - a1;
        let large = if sweep > pi { 1 } else { 0 };
        let x1o = cx as f64 + r2 as f64 * a1.cos();
        let y1o = cy as f64 + r2 as f64 * a1.sin();
        let x2o = cx as f64 + r2 as f64 * a2.cos();
        let y2o = cy as f64 + r2 as f64 * a2.sin();
        let x1i = cx as f64 + r1 as f64 * a2.cos();
        let y1i = cy as f64 + r1 as f64 * a2.sin();
        let x2i = cx as f64 + r1 as f64 * a1.cos();
        let y2i = cy as f64 + r1 as f64 * a1.sin();
        p.extend_from_slice(b"M ");
        push_f2(&mut p, x1o); p.push(b' '); push_f2(&mut p, y1o);
        p.extend_from_slice(b" A "); push_i(&mut p, r2); p.push(b' '); push_i(&mut p, r2);
        p.extend_from_slice(b" 0 "); push_i(&mut p, large); p.extend_from_slice(b" 1 ");
        push_f2(&mut p, x2o); p.push(b' '); push_f2(&mut p, y2o);
        p.extend_from_slice(b" L ");
        push_f2(&mut p, x1i); p.push(b' '); push_f2(&mut p, y1i);
        p.extend_from_slice(b" A "); push_i(&mut p, r1); p.push(b' '); push_i(&mut p, r1);
        p.extend_from_slice(b" 0 "); push_i(&mut p, large); p.extend_from_slice(b" 0 ");
        push_f2(&mut p, x2i); p.push(b' '); push_f2(&mut p, y2i);
        p.extend_from_slice(b" Z");
        p
    };

    let _ = leaf_total;

    for (ni, node) in all_nodes.iter().enumerate() {
        if node.sweep_angle < 1e-6 { continue; }
        let (r1, r2) = if node.parent.is_empty() {
            (r_inner, r_mid)
        } else {
            (r_mid, r_outer)
        };
        let a1 = node.start_angle;
        let a2 = node.start_angle + node.sweep_angle;
        let path = arc_path(cx, cy, r1, r2, a1, a2);
        let color = SB_PALETTE[node.color_idx % SB_PALETTE.len()];
        let hx = hex6(color);
        let opacity = if node.parent.is_empty() { "0.95" } else { "0.75" };
        push_b(&mut b, b"<path data-idx=\"");
        push_i(&mut b, ni as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, node.value);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &node.label);
        push_b(&mut b, b"\" d=\""); b.extend_from_slice(&path);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\""); push_b(&mut b, opacity.as_bytes());
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
        let mid_angle = a1 + node.sweep_angle / 2.0;
        let text_r = if node.parent.is_empty() {
            (r_inner + r_mid) / 2
        } else {
            (r_mid + r_outer) / 2
        };
        if node.sweep_angle > 0.15 {
            let tx = cx as f64 + text_r as f64 * mid_angle.cos();
            let ty = cy as f64 + text_r as f64 * mid_angle.sin();
            let max_len = if node.parent.is_empty() { 12 } else { 8 };
            let label = if node.label.len() > max_len { &node.label[..max_len] } else { &node.label };
            push_b(&mut b, b"<text x=\""); push_f2(&mut b, tx);
            push_b(&mut b, b"\" y=\""); push_f2(&mut b, ty + 4.0);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
            if node.parent.is_empty() { push_b(&mut b, b"10"); } else { push_b(&mut b, b"8"); }
            push_b(&mut b, b"\" fill=\"#fff\" pointer-events=\"none\">");
            escape_xml(&mut b, label);
            push_b(&mut b, b"</text>");
        }
    }
    push_b(&mut b, b"<circle cx=\""); push_i(&mut b, cx); push_b(&mut b, b"\" cy=\"");
    push_i(&mut b, cy); push_b(&mut b, b"\" r=\""); push_i(&mut b, r_inner - 2);
    push_b(&mut b, b"\" fill=\"#fff\"/>");
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
