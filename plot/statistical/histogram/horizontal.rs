use super::common::compute_bins;
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (bin_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = bin_counts.len();
    if n_bins == 0 { return String::new(); }
    let max_count = (*bin_counts.iter().max().unwrap_or(&1)) as f64;
    let pad_l: i32 = 78;
    let pad_r: i32 = 24;
    let pad_t: i32 = 36;
    let pad_b: i32 = 46;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, pad_t, pad_b, pad_r, n_bins * 240 + 2048);
    f.open(cfg.title, false);
    f.x_grid(5, 0.0, max_count, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let bh = f.ph as f64 / n_bins as f64;
    let op_i = (cfg.opacity as f64 / 255.0 * 100.0 + 0.5) as i32;
    let hx = hex6(cfg.color);

    for (i, &cnt) in bin_counts.iter().enumerate() {
        let bw = (cnt as f64 / max_count * f.pw as f64) as i32;
        let y = f.pt + (i as f64 * bh) as i32;
        let h_px = (bh as i32 - cfg.gap.max(0)).max(1);
        let x = f.pl;
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bw);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, h_px);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0."); push_i(&mut f.buf, op_i);
        push_b(&mut f.buf, b"\" rx=\"2\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
        if cfg.show_counts && bw > 22 {
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x + bw - 4);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y + h_px / 2 + 3);
            push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\">");
            push_i(&mut f.buf, cnt as i32);
            push_b(&mut f.buf, b"</text>");
        }
    }

    let tick_step = ((n_bins as f64 / 10.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let y = f.pt + (i as f64 * bh) as i32;
        let val = if i < edges.len() { edges[i] } else { *edges.last().unwrap_or(&0.0) };
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 6);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y + 4);
        push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-yt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
