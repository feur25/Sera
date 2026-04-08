a lib!use super::common::{push, push_b, escape_xml, hex6};
use super::hover::{HoverSlot, slots_to_json, build_html};

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
            color: 0x4C72B0,
            opacity: 204,
            width: 860,
            height: 380,
            overlay_values: None,
            overlay_color: 0xC44E52,
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
    let mut auto_slots: Vec<HoverSlot> = if auto_hover { Vec::with_capacity(n_bins) } else { Vec::new() };
    let mut buf = Vec::<u8>::with_capacity(n_bins * 240 + 2048);
    push(&mut buf, &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">",
        w=cfg.width, h=cfg.height,
    ));
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"22\" text-anchor=\"middle\" \
             font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" \
             font-weight=\"700\" fill=\"#1a202c\">",
            tx=cfg.width / 2,
        ));
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    if !cfg.y_label.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"12\" y=\"{y}\" text-anchor=\"middle\" \
             font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" \
             transform=\"rotate(-90,12,{y})\">",
            y = pad_t + plot_h / 2,
        ));
        escape_xml(&mut buf, cfg.y_label);
        push_b(&mut buf, b"</text>");
    }
    let n_yticks = 5;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let count_val = (frac * max_count).round() as u64;
        if cfg.gridlines && i > 0 {
            push(&mut buf, &format!(
                "<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" \
                 stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"3,3\"/>",
                x1=pad_l, x2=pad_l+plot_w, y=y,
            ));
        }
        push(&mut buf, &format!(
            "<text x=\"{x}\" y=\"{ty}\" text-anchor=\"end\" \
             font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">{v}</text>",
            x=pad_l-4, ty=y+3, v=count_val,
        ));
    }
    push(&mut buf, &format!(
        "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>",
        x1=pad_l, y1=pad_t, x2=pad_l, y2=pad_t+plot_h,
    ));
    push(&mut buf, &format!(
        "<line x1=\"{x1}\" y1=\"{y}\" x2=\"{x2}\" y2=\"{y}\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>",
        x1=pad_l, x2=pad_l+plot_w, y=pad_t+plot_h,
    ));
    let op = cfg.opacity as f64 / 255.0;
    let hx_main = {
        let h = hex6(cfg.color);
        unsafe { std::str::from_utf8_unchecked(&h).to_string() }
    };
    for (i, &cnt) in bin_counts.iter().enumerate() {
        let bh = (cnt as f64 / max_count * plot_h as f64) as i32;
        let x = pad_l + (i as f64 * bw) as i32;
        let y = pad_t + plot_h - bh;
        let w_px = (bw as i32).max(1) - 1;
        push(&mut buf, &format!(
            "<rect data-idx=\"{i}\" x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{bh}\" \
             fill=\"#{hx}\" fill-opacity=\"{op:.2}\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
            i=i, x=x, y=y, w=w_px, bh=bh, hx=hx_main, op=op,
        ));
        if cfg.show_counts && bh > 14 {
            push(&mut buf, &format!(
                "<text x=\"{tx}\" y=\"{ty}\" text-anchor=\"middle\" \
                 font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\">{cnt}</text>",
                tx = x + w_px / 2, ty = y + 11, cnt = cnt,
            ));
        }
        if auto_hover {
            auto_slots.push(
                HoverSlot::new(format!("{:.1}\u{2013}{:.1}", edges[i], edges.get(i+1).copied().unwrap_or(edges[i])))
                    .kv("Count", cnt.to_string())
                    .kv("Min", format!("{:.3}", edges[i]))
                    .kv("Max", format!("{:.3}", edges.get(i+1).copied().unwrap_or(edges[i])))
            );
        }
    }
    if let Some(ref oc) = overlay_counts {
        let hx2 = {
            let h = hex6(cfg.overlay_color);
            unsafe { std::str::from_utf8_unchecked(&h).to_string() }
        };
        let base_idx = n_bins;
        for (i, &cnt) in oc.iter().enumerate() {
            if cnt == 0 { continue; }
            let bh = (cnt as f64 / max_count * plot_h as f64) as i32;
            let x = pad_l + (i as f64 * bw) as i32;
            let y = pad_t + plot_h - bh;
            let w_px = (bw as i32).max(1) - 1;
            push(&mut buf, &format!(
                "<rect data-idx=\"{idx}\" x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{bh}\" \
                 fill=\"#{hx}\" fill-opacity=\"0.60\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
                idx=base_idx+i, x=x, y=y, w=w_px, bh=bh, hx=hx2,
            ));
        }
    }
    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = pad_l + (i as f64 * bw) as i32;
        let val = if i < edges.len() { edges[i] } else { *edges.last().unwrap_or(&0.0) };
        push(&mut buf, &format!(
            "<text x=\"{x}\" y=\"{y}\" text-anchor=\"middle\" \
             font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">{v:.1}</text>",
            x=x, y=pad_t+plot_h+14, v=val,
        ));
    }
    if !cfg.x_label.is_empty() {
        push(&mut buf, &format!(
            "<text x=\"{tx}\" y=\"{ty}\" text-anchor=\"middle\" \
             font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
            tx = pad_l + plot_w / 2, ty = cfg.height - 4,
        ));
        escape_xml(&mut buf, cfg.x_label);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots = if auto_hover { &auto_slots } else { cfg.hover };
    build_html(cfg.title, &svg, &slots_to_json(slots))
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
