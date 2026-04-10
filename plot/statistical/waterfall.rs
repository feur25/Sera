use super::common::{push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::build_chart_html;

pub struct WaterfallConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub values: &'a [f64],
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub show_text: bool,
    pub gridlines: bool,
    pub width: i32,
    pub height: i32,
}

impl<'a> Default for WaterfallConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            labels: &[],
            values: &[],
            x_label: "",
            y_label: "",
            show_text: true,
            gridlines: false,
            width: 900,
            height: 480,
        }
    }
}

const COLOR_POS: u32 = 0x10B981;
const COLOR_NEG: u32 = 0xF43F5E;
const COLOR_TOTAL: u32 = 0x6366F1;

pub fn render_waterfall_html(cfg: &WaterfallConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 { return String::new(); }

    let mut running = 0.0_f64;
    let mut starts: Vec<f64> = Vec::with_capacity(n);
    let mut ends: Vec<f64> = Vec::with_capacity(n);
    let is_total: Vec<bool> = cfg.labels.iter()
        .map(|l| {
            let lo = l.to_lowercase();
            lo.contains("total") || lo.contains("net") || lo.contains("final")
        })
        .collect();

    for i in 0..n {
        if is_total[i] {
            starts.push(0.0);
            ends.push(running);
        } else {
            starts.push(running);
            running += cfg.values[i];
            ends.push(running);
        }
    }

    let all_vals: Vec<f64> = starts.iter().chain(ends.iter()).copied().collect();
    let min_val = all_vals.iter().copied().fold(f64::INFINITY, f64::min).min(0.0);
    let max_val = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max).max(1.0);
    let range = (max_val - min_val).max(1e-12);

    let pad_l: i32 = 60;
    let pad_t: i32 = 46;
    let pad_b: i32 = 52;
    let plot_w = cfg.width - pad_l - 20;
    let plot_h = cfg.height - pad_t - pad_b;
    let bar_w = (plot_w / n as i32 - 6).max(4);
    let bar_step = plot_w / n as i32;

    let val_to_y = |v: f64| -> i32 {
        pad_t + ((1.0 - (v - min_val) / range) * plot_h as f64) as i32
    };

    let mut b = Vec::<u8>::with_capacity(n * 200 + 2048);
    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, (cfg.width) / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    let n_yticks: i32 = 6;
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let v = min_val + frac * range;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        if cfg.gridlines && ti > 0 {
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, y);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, y);
            push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>");
        }
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 4);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, y + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-yt\">");
        if v.abs() >= 1_000_000.0 { push_f2(&mut b, v / 1_000_000.0); push_b(&mut b, b"M"); }
        else if v.abs() >= 1_000.0 { push_i(&mut b, v as i32); }
        else { push_f2(&mut b, v); }
        push_b(&mut b, b"</text>");
    }

    let zero_y = val_to_y(0.0);
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, zero_y);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, zero_y);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\"/>");

    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\" class=\"sp-ax-y\"/>");

    for i in 0..n {
        let bx = pad_l + i as i32 * bar_step + bar_step / 2 - bar_w / 2;
        let y_start = val_to_y(starts[i]);
        let y_end   = val_to_y(ends[i]);
        let bar_top  = y_start.min(y_end);
        let bar_ht   = (y_start - y_end).abs().max(2);
        let color = if is_total[i] { COLOR_TOTAL }
                    else if cfg.values[i] >= 0.0 { COLOR_POS }
                    else { COLOR_NEG };
        let hx = hex6(color);
        push_b(&mut b, b"<rect data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, cfg.values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &cfg.labels[i]);
        push_b(&mut b, b"\" x=\""); push_i(&mut b, bx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, bar_top);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, bar_w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bar_ht);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" rx=\"2\" opacity=\"0.9\"/>");

        if i + 1 < n && !is_total[i + 1] {
            let connector_x = bx + bar_w;
            let next_bx = pad_l + (i + 1) as i32 * bar_step + bar_step / 2 - bar_w / 2;
            let connector_y = val_to_y(ends[i]);
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, connector_x);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, connector_y);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, next_bx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, connector_y);
            push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" stroke-dasharray=\"2,2\"/>");
        }

        let lbl_y = pad_t + plot_h + 14;
        push_b(&mut b, b"<text x=\""); push_i(&mut b, bx + bar_w / 2);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, lbl_y);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        let lbl = &cfg.labels[i];
        let short = if lbl.len() > 10 { &lbl[..10] } else { lbl };
        escape_xml(&mut b, short);
        push_b(&mut b, b"</text>");

        if cfg.show_text && bar_ht > 10 {
            let v = if is_total[i] { ends[i] } else { cfg.values[i] };
            let txt_y = bar_top - 3;
            push_b(&mut b, b"<text x=\""); push_i(&mut b, bx + bar_w / 2);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, txt_y);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#374151\" pointer-events=\"none\">");
            if v.abs() >= 1_000_000.0 { push_f2(&mut b, v / 1_000_000.0); push_b(&mut b, b"M"); }
            else if v.abs() >= 1_000.0 { push_f2(&mut b, v / 1_000.0); push_b(&mut b, b"k"); }
            else { push_f2(&mut b, v); }
            push_b(&mut b, b"</text>");
        }
    }

    if !cfg.y_label.is_empty() {
        let ym = pad_t + plot_h / 2;
        push_b(&mut b, b"<text x=\"14\" y=\""); push_i(&mut b, ym);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" transform=\"rotate(-90 14 ");
        push_i(&mut b, ym); push_b(&mut b, b")\">");
        escape_xml(&mut b, cfg.y_label);
        push_b(&mut b, b"</text>");
    }

    if !cfg.x_label.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l + plot_w / 2);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, cfg.height - 4);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#6b7280\">");
        escape_xml(&mut b, cfg.x_label);
        push_b(&mut b, b"</text>");
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
