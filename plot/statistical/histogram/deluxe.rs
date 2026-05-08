use super::common::{bin_to_edges, compute_bins};
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, Frame};

fn spectrum_color(i: usize, n: usize) -> (u32, u32) {
    let t = if n <= 1 { 0.5 } else { i as f64 / (n - 1) as f64 };
    let stops = [
        (0x06B6D4_u32, 0x22D3EE_u32),
        (0x6366F1, 0x818CF8),
        (0x8B5CF6, 0xA78BFA),
        (0xF43F5E, 0xFB7185),
        (0xF59E0B, 0xFBBF24),
        (0x10B981, 0x34D399),
    ];
    let idx = (t * (stops.len() - 1) as f64) as usize;
    let idx = idx.min(stops.len() - 1);
    stops[idx]
}

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (bin_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = bin_counts.len();
    if n_bins == 0 { return String::new(); }
    let max_count = (*bin_counts.iter().max().unwrap_or(&1) as f64).max(1.0);

    let pad_l: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, 36, 46, pad_r, n_bins * 300 + 4096);
    f.open(cfg.title, false);

    push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt);
    push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, f.pw);
    push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, f.ph);
    push_b(&mut f.buf, b"\" fill=\"#0f172a\" rx=\"3\"/>");

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"dlxhf\" x=\"-30%\" y=\"-30%\" width=\"160%\" height=\"160%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"3\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    for i in 0..n_bins {
        let (base, bright) = spectrum_color(i, n_bins);
        push_b(&mut f.buf, b"<linearGradient id=\"dlxhg"); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(base)); push_b(&mut f.buf, b"\" stop-opacity=\"0.9\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(bright)); push_b(&mut f.buf, b"\" stop-opacity=\"1\"/>");
        push_b(&mut f.buf, b"</linearGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    f.y_grid_int(5, max_count, cfg.gridlines);

    for i in 0..n_bins {
        let cnt = bin_counts[i];
        let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
        let x = f.pl + (i as f64 * bw) as i32;
        let y = f.pt + f.ph - bh;
        let w_px = ((bw as i32) - cfg.gap.max(0)).max(1);
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
        push_b(&mut f.buf, b"\" data-kv-Min=\""); push_f2(&mut f.buf, edges[i]);
        push_b(&mut f.buf, b"\" data-kv-Max=\""); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bh.max(1));
        push_b(&mut f.buf, b"\" fill=\"url(#dlxhg"); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\" rx=\"3\" filter=\"url(#dlxhf)\"/>");
        push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\"3\" fill=\"#ffffff\" fill-opacity=\"0.22\" rx=\"2\"/>");
        if cfg.show_counts && bh > 16 {
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x + w_px / 2);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y + 12);
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
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\" class=\"sp-xt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }

    let _ = (escape_xml, bin_to_edges);
    f.html(&slots_to_json(cfg.hover))
}
