use super::common::compute_bins;
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};

#[crate::chart_demo("values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]")]

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (bin_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = bin_counts.len();
    if n_bins == 0 { return String::new(); }
    let max_count = (*bin_counts.iter().max().unwrap_or(&1)) as f64;
    let pad_l: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, 36, 46, pad_r, n_bins * 320 + 2048);
    f.open(cfg.title, false);
    f.y_grid_int(5, max_count, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let hx = hex6(cfg.color);

    push_b(&mut f.buf, b"<path fill=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" fill-opacity=\"0.18\" stroke=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\"");
    let sw = cfg.stroke_width.max(1.5);
    push_f2(&mut f.buf, sw);
    push_b(&mut f.buf, b"\" stroke-linejoin=\"miter\" d=\"");
    let baseline_y = f.pt + f.ph;
    let first_x = f.pl;
    f.buf.push(b'M'); push_i(&mut f.buf, first_x); f.buf.push(b' '); push_i(&mut f.buf, baseline_y);
    for (i, &cnt) in bin_counts.iter().enumerate() {
        let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
        let x0 = f.pl + (i as f64 * bw_px) as i32;
        let x1 = f.pl + ((i + 1) as f64 * bw_px) as i32;
        let y = f.pt + f.ph - bh;
        push_b(&mut f.buf, b" L"); push_i(&mut f.buf, x0); f.buf.push(b' '); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b" L"); push_i(&mut f.buf, x1); f.buf.push(b' '); push_i(&mut f.buf, y);
    }
    let last_x = f.pl + plot_w;
    push_b(&mut f.buf, b" L"); push_i(&mut f.buf, last_x); f.buf.push(b' '); push_i(&mut f.buf, baseline_y);
    push_b(&mut f.buf, b" Z\"/>");

    for (i, &cnt) in bin_counts.iter().enumerate() {
        let x = f.pl + (i as f64 * bw_px) as i32;
        let y_top = f.pt + f.ph - (cnt as f64 / max_count * f.ph as f64) as i32;
        let w_px = (bw_px as i32).max(1);
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y_top);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, f.pt + f.ph - y_top);
        push_b(&mut f.buf, b"\" fill=\"transparent\"/>");
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

