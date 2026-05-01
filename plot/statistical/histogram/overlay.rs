use super::common::{bin_to_edges, compute_bins};
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, svg_legend_item, Frame};

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (a_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = a_counts.len();
    if n_bins == 0 { return String::new(); }
    let b_counts = match cfg.overlay_values {
        Some(v) => bin_to_edges(v, &edges).0,
        None => vec![0u64; n_bins],
    };
    let max_count = a_counts.iter().chain(b_counts.iter()).cloned().max().unwrap_or(1).max(1) as f64;

    let has_legend = cfg.series_names.is_some();
    let legend_w = if has_legend { 130 } else { 20 };
    let pad_l: i32 = 52;
    let pad_r: i32 = legend_w;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, 36, 46, pad_r, n_bins * 320 + 2048);
    f.open(cfg.title, false);
    f.y_grid_int(5, max_count, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let hx_a = hex6(cfg.color);
    let hx_b = hex6(cfg.overlay_color);

    for (i, &cnt) in a_counts.iter().enumerate() {
        let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
        let x = f.pl + (i as f64 * bw_px) as i32;
        let y = f.pt + f.ph - bh;
        let w_px = (bw_px as i32).max(1) - 1;
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-lbl=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bh);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx_a);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.55\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
    }
    for (i, &cnt) in b_counts.iter().enumerate() {
        if cnt == 0 { continue; }
        let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
        let x = f.pl + (i as f64 * bw_px) as i32;
        let y = f.pt + f.ph - bh;
        let w_px = (bw_px as i32).max(1) - 1;
        let base_idx = n_bins;
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, (base_idx + i) as i32);
        push_b(&mut f.buf, b"\" data-series=\"1\" data-lbl=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bh);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx_b);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.55\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
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

    if let Some((n0, n1)) = cfg.series_names {
        let lx = cfg.width - legend_w + 10;
        svg_legend_item(&mut f.buf, 0, n0, cfg.color, lx, f.pt + 6, 18);
        svg_legend_item(&mut f.buf, 1, n1, cfg.overlay_color, lx, f.pt + 28, 18);
    }
    let _ = escape_xml;

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
