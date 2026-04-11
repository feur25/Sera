use super::common::{sorted, push_b, push_i, push_f2, escape_xml, hex6, sort_indices};
use crate::html::hover::{build_chart_html, slots_to_json};

crate::chart_config!(BulletConfig, 800, 300;
    struct {
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub targets: &'a [f64],
        pub max_vals: &'a [f64],
        pub ranges: &'a [f64],
    }
    defaults {
        labels: &[],
        values: &[],
        targets: &[],
        max_vals: &[],
        ranges: &[],
    }
);

pub fn render_bullet_html(cfg: &BulletConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }
    let idx = sort_indices(n, cfg.values, cfg.labels, cfg.sort_order);
    let labels = sorted(&idx, cfg.labels);
    let values = sorted(&idx, cfg.values);
    let targets: Vec<f64> = idx.iter().map(|&i| cfg.targets.get(i).copied().unwrap_or(0.0)).collect();
    let max_vals: Vec<f64> = idx.iter().map(|&i| cfg.max_vals.get(i).copied().unwrap_or(0.0)).collect();
    let ranges: Vec<f64> = idx.iter().map(|&i| cfg.ranges.get(i).copied().unwrap_or(0.0)).collect();
    let auto_h = if cfg.height < n as i32 * 50 + 60 { n as i32 * 50 + 60 } else { cfg.height };
    let pad_l: i32 = 130;
    let pad_r: i32 = 30;
    let pad_t: i32 = if cfg.title.is_empty() { 20 } else { 46 };
    let row_h: i32 = (auto_h - pad_t - 20) / n as i32;
    let bar_h: i32 = (row_h as f64 * 0.38) as i32;
    let range_h: i32 = (row_h as f64 * 0.6) as i32;
    let plot_w = cfg.width - pad_l - pad_r;
    let track_color: u32 = 0xE5E7EB;
    let range_color: u32 = 0xC7D2FE;
    let value_color: u32 = 0x6366F1;
    let hx_track = hex6(track_color);
    let hx_range = hex6(range_color);
    let hx_value = hex6(value_color);
    let mut b = Vec::<u8>::with_capacity(n * 300 + 1024);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, auto_h); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, auto_h); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }
    for i in 0..n {
        let max_v = if max_vals[i] > 0.0 {
            max_vals[i]
        } else {
            let v = values[i];
            let t = targets[i];
            (v.max(t) * 1.2).max(1.0)
        };
        let ry = pad_t + i as i32 * row_h + (row_h - range_h) / 2;
        let by = pad_t + i as i32 * row_h + (row_h - bar_h) / 2;
        let range_pct = if ranges[i] > 0.0 {
            (ranges[i] / max_v).min(1.0)
        } else { 0.75 };
        let range_w = (plot_w as f64 * range_pct) as i32;
        let value_w = ((values[i] / max_v).min(1.0) * plot_w as f64) as i32;
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ry);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, plot_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, range_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_track);
        push_b(&mut b, b"\" rx=\"3\"/>");
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ry);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, range_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, range_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_range);
        push_b(&mut b, b"\" rx=\"3\"/>");
        push_b(&mut b, b"<rect data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &labels[i]);
        push_b(&mut b, b"\" x=\""); push_i(&mut b, pad_l);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, by);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, value_w.max(2));
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bar_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx_value);
        push_b(&mut b, b"\" rx=\"2\" opacity=\"0.9\"/>");
        if targets[i] > 0.0 {
            let tx = pad_l + ((targets[i] / max_v).min(1.0) * plot_w as f64) as i32;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, ry);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, tx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, ry + range_h);
            push_b(&mut b, b"\" stroke=\"#1a202c\" stroke-width=\"2.5\"/>");
        }
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 6);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, by + bar_h / 2 + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        let lbl = &labels[i];
        let short = if lbl.len() > 16 { &lbl[..16] } else { lbl };
        escape_xml(&mut b, short);
        push_b(&mut b, b"</text>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l + value_w + 5);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, by + bar_h / 2 + 4);
        push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
        if values[i] >= 1_000_000.0 { push_f2(&mut b, values[i] / 1_000_000.0); push_b(&mut b, b"M"); }
        else if values[i] >= 1_000.0 { push_f2(&mut b, values[i] / 1_000.0); push_b(&mut b, b"k"); }
        else { push_f2(&mut b, values[i]); }
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
