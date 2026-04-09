use super::common::{push_b, push_i, push_f2, escape_xml, hex6, svg_open, svg_hgrid, svg_axis_lines, svg_y_label, svg_x_label};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct Histogram;

pub struct HistogramConfig<'a> {
    pub title: &'a str,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub values: &'a [f64],
    pub bins: usize,
    pub color: u32,
    pub opacity: u8,
    pub width: i32,
    pub height: i32,
    pub overlay_values: Option<&'a [f64]>,
    pub overlay_color: u32,
    pub gridlines: bool,
    pub show_counts: bool,
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for HistogramConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            x_label: "",
            y_label: "Count",
            values: &[],
            bins: 0,
            color: 0x6366F1,
            opacity: 204,
            width: 860,
            height: 380,
            overlay_values: None,
            overlay_color: 0xF43F5E,
            gridlines: true,
            show_counts: false,
            hover: &[],
        }
    }
}

pub fn render_histogram_html(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (bin_counts, edges) = compute_bins(cfg.values, cfg.bins);
    let overlay_counts = cfg.overlay_values.map(|v| {
        let (oc, _) = bin_to_edges(v, &edges);
        oc
    });
    let n_bins = bin_counts.len();
    let max_count = {
        let mut m = *bin_counts.iter().max().unwrap_or(&1) as f64;
        if let Some(ref oc) = overlay_counts { m = m.max(*oc.iter().max().unwrap_or(&0) as f64); }
        m.max(1.0)
    };
    let pad_l: i32 = 52;
    let pad_t: i32 = 36;
    let pad_b: i32 = 46;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let bw = plot_w as f64 / n_bins as f64;
    let auto_hover = cfg.hover.is_empty();
    let mut buf = Vec::<u8>::with_capacity(n_bins * 240 + 2048);
    svg_open(&mut buf, cfg.width, cfg.height);
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, cfg.width / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    svg_y_label(&mut buf, cfg.y_label, 12, pad_t, plot_h);
    let n_yticks = 5;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let count_val = (frac * max_count).round() as i32;
        if cfg.gridlines && i > 0 {
            svg_hgrid(&mut buf, pad_l, pad_l + plot_w, y);
        }
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l - 4);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        push_i(&mut buf, count_val);
        push_b(&mut buf, b"</text>");
    }
    svg_axis_lines(&mut buf, pad_l, pad_t, plot_w, plot_h);
    let op_i = (cfg.opacity as f64 / 255.0 * 100.0 + 0.5) as i32;
    let hx_main = hex6(cfg.color);
    for (i, &cnt) in bin_counts.iter().enumerate() {
        let bh = (cnt as f64 / max_count * plot_h as f64) as i32;
        let x = pad_l + (i as f64 * bw) as i32;
        let y = pad_t + plot_h - bh;
        let w_px = (bw as i32).max(1) - 1;
        push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\""); push_f2(&mut buf, edges[i]); buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut buf, b"\" data-kv-Count=\""); push_i(&mut buf, cnt as i32);
        push_b(&mut buf, b"\" data-kv-Min=\""); push_f2(&mut buf, edges[i]);
        push_b(&mut buf, b"\" data-kv-Max=\""); push_f2(&mut buf, edges.get(i+1).copied().unwrap_or(edges[i]));
        push_b(&mut buf, b"\" x=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y);
        push_b(&mut buf, b"\" width=\""); push_i(&mut buf, w_px);
        push_b(&mut buf, b"\" height=\""); push_i(&mut buf, bh);
        push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx_main);
        push_b(&mut buf, b"\" fill-opacity=\"0."); push_i(&mut buf, op_i);
        push_b(&mut buf, b"\" rx=\"2\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
        if cfg.show_counts && bh > 14 {
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, x + w_px / 2);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y + 11);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\">");
            push_i(&mut buf, cnt as i32);
            push_b(&mut buf, b"</text>");
        }
    }
    if let Some(ref oc) = overlay_counts {
        let hx2 = hex6(cfg.overlay_color);
        let base_idx = n_bins;
        for (i, &cnt) in oc.iter().enumerate() {
            if cnt == 0 { continue; }
            let bh = (cnt as f64 / max_count * plot_h as f64) as i32;
            let x = pad_l + (i as f64 * bw) as i32;
            let y = pad_t + plot_h - bh;
            let w_px = (bw as i32).max(1) - 1;
            push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, (base_idx + i) as i32);
            push_b(&mut buf, b"\" x=\""); push_i(&mut buf, x);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" width=\""); push_i(&mut buf, w_px);
            push_b(&mut buf, b"\" height=\""); push_i(&mut buf, bh);
            push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx2);
            push_b(&mut buf, b"\" fill-opacity=\"0.60\" rx=\"2\" stroke=\"#fff\" stroke-width=\"0.4\"/>");
        }
    }
    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = pad_l + (i as f64 * bw) as i32;
        let val = if i < edges.len() { edges[i] } else { *edges.last().unwrap_or(&0.0) };
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 14);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        push_f2(&mut buf, val);
        push_b(&mut buf, b"</text>");
    }
    svg_x_label(&mut buf, cfg.x_label, pad_l + plot_w / 2, cfg.height - 4);
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if auto_hover { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}

pub fn compute_bins(values: &[f64], n_bins: usize) -> (Vec<u64>, Vec<f64>) {
    let valid: Vec<f64> = values.iter().cloned().filter(|v| v.is_finite()).collect();
    if valid.is_empty() { return (vec![], vec![]); }
    let min = valid.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = valid.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let n = if n_bins == 0 {
        ((valid.len() as f64).log2().ceil() as usize + 1).clamp(5, 128)
    } else {
        n_bins.clamp(2, 512)
    };
    let range = (max - min).max(1e-12);
    let step = range / n as f64;
    let mut edges = Vec::with_capacity(n + 1);
    for i in 0..=n { edges.push(min + i as f64 * step); }
    bin_to_edges(&valid, &edges)
}

pub fn bin_to_edges(values: &[f64], edges: &[f64]) -> (Vec<u64>, Vec<f64>) {
    let n = edges.len().saturating_sub(1);
    let mut counts = vec![0u64; n];
    if n == 0 { return (counts, edges.to_vec()); }
    let min = edges[0];
    let max = *edges.last().unwrap();
    let range = (max - min).max(1e-12);
    let step = range / n as f64;
    for &v in values.iter() {
        if !v.is_finite() { continue; }
        let idx = ((v - min) / step) as usize;
        counts[idx.min(n - 1)] += 1;
    }
    (counts, edges.to_vec())
}
