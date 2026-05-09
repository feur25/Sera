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
    let mut acc: Vec<f64> = Vec::with_capacity(n_bins);
    let mut s = 0.0_f64;
    for &c in &bin_counts { s += c as f64 / total; acc.push(s); }
    let max_y = 1.0_f64;

    let pad_l: i32 = 56;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, 36, 46, pad_r, n_bins * 240 + 2048);
    f.open(cfg.title, false);
    f.y_grid(5, 0.0, max_y, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let op_i = (cfg.opacity as f64 / 255.0 * 100.0 + 0.5) as i32;
    let hx = hex6(cfg.color);

    let baseline_y = f.pt + f.ph;
    push_b(&mut f.buf, b"<path d=\"M ");
    push_i(&mut f.buf, f.pl);
    f.buf.push(b' ');
    push_i(&mut f.buf, baseline_y);
    for (i, &p) in acc.iter().enumerate() {
        let x0 = f.pl + (i as f64 * bw_px) as i32;
        let x1 = f.pl + ((i + 1) as f64 * bw_px) as i32;
        let y = f.pt + f.ph - (p / max_y * f.ph as f64) as i32;
        push_b(&mut f.buf, b" L ");
        push_i(&mut f.buf, x0); f.buf.push(b' '); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b" L ");
        push_i(&mut f.buf, x1); f.buf.push(b' '); push_i(&mut f.buf, y);
    }
    push_b(&mut f.buf, b" L ");
    push_i(&mut f.buf, f.pl + (n_bins as f64 * bw_px) as i32);
    f.buf.push(b' ');
    push_i(&mut f.buf, baseline_y);
    push_b(&mut f.buf, b" Z\" fill=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" fill-opacity=\"0.");
    push_i(&mut f.buf, op_i);
    push_b(&mut f.buf, b"\" stroke=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\"1.6\" stroke-linejoin=\"miter\"/>");

    for (i, &p) in acc.iter().enumerate() {
        let x0 = f.pl + (i as f64 * bw_px) as i32;
        let x1 = f.pl + ((i + 1) as f64 * bw_px) as i32;
        let y = f.pt + f.ph - (p / max_y * f.ph as f64) as i32;
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\"");
        f.buf.extend_from_slice("\u{2264} ".as_bytes());
        push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Cumulative=\""); push_f2(&mut f.buf, p);
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, bin_counts[i] as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x0);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, (x1 - x0).max(1));
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, baseline_y - y);
        push_b(&mut f.buf, b"\" fill=\"transparent\" stroke=\"none\"/>");
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


