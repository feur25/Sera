use super::common::compute_bins;
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (bin_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = bin_counts.len();
    if n_bins == 0 { return String::new(); }
    let total = bin_counts.iter().sum::<u64>().max(1) as f64;
    let bw_data = if edges.len() >= 2 { (edges[1] - edges[0]).max(1e-12) } else { 1.0 };
    let densities: Vec<f64> = bin_counts.iter().map(|&c| c as f64 / total / bw_data).collect();
    let max_d = densities.iter().cloned().fold(0.0_f64, f64::max).max(1e-9);

    let pad_l: i32 = 56;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, 36, 46, pad_r, n_bins * 240 + 2048);
    f.open(cfg.title, false);
    f.y_grid(5, 0.0, max_d, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let op_i = (cfg.opacity as f64 / 255.0 * 100.0 + 0.5) as i32;
    let hx = hex6(cfg.color);

    for (i, &d) in densities.iter().enumerate() {
        let bh = (d / max_d * f.ph as f64) as i32;
        let x = f.pl + (i as f64 * bw_px) as i32;
        let y = f.pt + f.ph - bh;
        let w_px = (bw_px as i32).max(1) - 1;
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Density=\""); push_f2(&mut f.buf, d);
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, bin_counts[i] as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bh);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0."); push_i(&mut f.buf, op_i);
        push_b(&mut f.buf, b"\" rx=\"2\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
    }

    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = f.pl + (i as f64 * bw_px) as i32;
        let val = if i < edges.len() { edges[i] } else { *edges.last().unwrap_or(&0.0) };
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
