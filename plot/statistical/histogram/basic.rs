use super::common::{bin_to_edges, compute_bins};
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, sorted, svg_legend_item, Frame};

pub const DEMO_KWARGS: &str = "values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]";
pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (mut bin_counts, mut edges) = compute_bins(cfg.values, cfg.bins);
    if cfg.count_scale > 1 { for c in bin_counts.iter_mut() { *c *= cfg.count_scale as u64; } }
    let mut overlay_counts = cfg.overlay_values.map(|v| {
        let (oc, _) = bin_to_edges(v, &edges);
        oc
    });
    let n_bins = bin_counts.len();
    if n_bins > 0 && cfg.sort_order != "none" && !cfg.sort_order.is_empty() {
        let mut idx: Vec<usize> = (0..n_bins).collect();
        match cfg.sort_order {
            "asc" | "ascending" => idx.sort_by(|&a, &b| bin_counts[a].cmp(&bin_counts[b])),
            "desc" | "descending" => idx.sort_by(|&a, &b| bin_counts[b].cmp(&bin_counts[a])),
            _ => {}
        }
        if let Some(ref oc) = overlay_counts { overlay_counts = Some(sorted(&idx, oc)); }
        let mut se = sorted(&idx, &edges[..n_bins]);
        se.push(*edges.last().unwrap_or(&0.0));
        bin_counts = sorted(&idx, &bin_counts);
        edges = se;
    }
    let max_count = {
        let mut m = *bin_counts.iter().max().unwrap_or(&1) as f64;
        if let Some(ref oc) = overlay_counts { m = m.max(*oc.iter().max().unwrap_or(&0) as f64); }
        m.max(1.0)
    };
    let pad_l: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw = plot_w as f64 / n_bins as f64;
    let auto_hover = cfg.hover.is_empty();
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 52, 36, 46, 20, n_bins * 240 + 2048);
    f.open(cfg.title, false);
    f.y_grid_int(5, max_count, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let op_i = (cfg.opacity as f64 / 255.0 * 100.0 + 0.5) as i32;
    let hx_main = hex6(cfg.color);
    for (i, &cnt) in bin_counts.iter().enumerate() {
        let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
        let x = f.pl + (i as f64 * bw) as i32;
        let y = f.pt + f.ph - bh;
        let w_px = (bw as i32 - cfg.gap.max(0)).max(1);
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
        push_b(&mut f.buf, b"\" data-kv-Min=\""); push_f2(&mut f.buf, edges[i]);
        push_b(&mut f.buf, b"\" data-kv-Max=\""); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bh);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx_main);
        push_b(&mut f.buf, b"\" fill-opacity=\"0."); push_i(&mut f.buf, op_i);
        push_b(&mut f.buf, b"\" rx=\"2\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
        if cfg.show_counts && bh > 14 {
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x + w_px / 2);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y + 11);
            push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\">");
            push_i(&mut f.buf, cnt as i32);
            push_b(&mut f.buf, b"</text>");
        }
    }
    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = f.pl + (i as f64 * bw) as i32;
        let val = if i < edges.len() { edges[i] } else { *edges.last().unwrap_or(&0.0) };
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }
    let _ = escape_xml;
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}

pub fn render_with_overlay_legend(cfg: &HistogramConfig) -> String {
    if let Some((n0, n1)) = cfg.series_names {
        let mut s = render(cfg);
        if cfg.overlay_values.is_some() {
            let lx = cfg.width - 130;
            let mut tmp: Vec<u8> = Vec::new();
            svg_legend_item(&mut tmp, 0, n0, cfg.color, lx, 36 + 5, 16);
            svg_legend_item(&mut tmp, 1, n1, cfg.overlay_color, lx, 36 + 22, 16);
            if let Some(idx) = s.rfind("</svg>") {
                let extra = String::from_utf8_lossy(&tmp).into_owned();
                s.insert_str(idx, &extra);
            }
        }
        return s;
    }
    render(cfg)
}


