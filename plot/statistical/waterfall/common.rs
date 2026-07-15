use super::config::WaterfallConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{
    escape_xml, push_b, push_f2, push_i, sort_indices, sorted, truncate,
};

pub const COLOR_POS: u32 = 0x10B981;
pub const COLOR_NEG: u32 = 0xF43F5E;
pub const COLOR_TOTAL: u32 = 0x636EFA;

pub struct Layout {
    pub pad_l: i32,
    pub pad_t: i32,
    pub pad_b: i32,
    pub plot_w: i32,
    pub plot_h: i32,
    pub bar_w: i32,
    pub bar_step: i32,
    pub min_val: f64,
    pub max_val: f64,
    pub range: f64,
    pub zero_y: i32,
}

pub struct Prepared {
    pub n: usize,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub starts: Vec<f64>,
    pub ends: Vec<f64>,
    pub is_total: Vec<bool>,
    pub layout: Layout,
}

pub fn prepare(cfg: &WaterfallConfig) -> Option<Prepared> {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return None;
    }
    let idx = sort_indices(n, cfg.values, cfg.labels, cfg.sort_order);
    let labels = sorted(&idx, cfg.labels);
    let values = sorted(&idx, cfg.values);

    let mut running = 0.0_f64;
    let mut starts: Vec<f64> = Vec::with_capacity(n);
    let mut ends: Vec<f64> = Vec::with_capacity(n);
    let is_total: Vec<bool> = labels
        .iter()
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
            running += values[i];
            ends.push(running);
        }
    }

    let all_vals: Vec<f64> = starts.iter().chain(ends.iter()).copied().collect();
    let min_val = all_vals
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .min(0.0);
    let max_val = all_vals
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0);
    let range = (max_val - min_val).max(1e-12);

    let pad_l: i32 = 60;
    let pad_t: i32 = 46;
    let pad_b: i32 = 52;
    let plot_w = cfg.width - pad_l - 20;
    let plot_h = cfg.height - pad_t - pad_b;
    let bar_step = plot_w / n as i32;
    let bar_w = (bar_step - 6).max(4);
    let zero_y = pad_t + ((1.0 - (0.0 - min_val) / range) * plot_h as f64) as i32;

    Some(Prepared {
        n,
        labels,
        values,
        starts,
        ends,
        is_total,
        layout: Layout {
            pad_l,
            pad_t,
            pad_b,
            plot_w,
            plot_h,
            bar_w,
            bar_step,
            min_val,
            max_val,
            range,
            zero_y,
        },
    })
}

pub fn val_to_y(l: &Layout, v: f64) -> i32 {
    l.pad_t + ((1.0 - (v - l.min_val) / l.range) * l.plot_h as f64) as i32
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &WaterfallConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\">");
    push_b(
        buf,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\"");
        push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn axes(buf: &mut Vec<u8>, cfg: &WaterfallConfig, p: &Prepared) {
    let l = &p.layout;
    let n_yticks: i32 = 6;
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let v = l.min_val + frac * l.range;
        let y = l.pad_t + ((1.0 - frac) * l.plot_h as f64) as i32;
        if cfg.gridlines && ti > 0 {
            push_b(buf, b"<line x1=\"");
            push_i(buf, l.pad_l);
            push_b(buf, b"\" y1=\"");
            push_i(buf, y);
            push_b(buf, b"\" x2=\"");
            push_i(buf, l.pad_l + l.plot_w);
            push_b(buf, b"\" y2=\"");
            push_i(buf, y);
            push_b(
                buf,
                b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>",
            );
        }
        push_b(buf, b"<text x=\"");
        push_i(buf, l.pad_l - 4);
        push_b(buf, b"\" y=\"");
        push_i(buf, y + 4);
        push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        if v.abs() >= 1_000_000.0 {
            push_f2(buf, v / 1_000_000.0);
            push_b(buf, b"M");
        } else if v.abs() >= 1_000.0 {
            push_i(buf, v as i32);
        } else {
            push_f2(buf, v);
        }
        push_b(buf, b"</text>");
    }
    push_b(buf, b"<line x1=\"");
    push_i(buf, l.pad_l);
    push_b(buf, b"\" y1=\"");
    push_i(buf, l.zero_y);
    push_b(buf, b"\" x2=\"");
    push_i(buf, l.pad_l + l.plot_w);
    push_b(buf, b"\" y2=\"");
    push_i(buf, l.zero_y);
    push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\"/>");
    push_b(buf, b"<line x1=\"");
    push_i(buf, l.pad_l);
    push_b(buf, b"\" y1=\"");
    push_i(buf, l.pad_t);
    push_b(buf, b"\" x2=\"");
    push_i(buf, l.pad_l);
    push_b(buf, b"\" y2=\"");
    push_i(buf, l.pad_t + l.plot_h);
    push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
}

pub fn xlabel(buf: &mut Vec<u8>, cx: i32, y: i32, lbl: &str) {
    push_b(buf, b"<text x=\"");
    push_i(buf, cx);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
    escape_xml(buf, truncate(lbl, 10));
    push_b(buf, b"</text>");
}

pub fn value_text(buf: &mut Vec<u8>, cx: i32, y: i32, v: f64) {
    push_b(buf, b"<text x=\"");
    push_i(buf, cx);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#374151\" pointer-events=\"none\">");
    if v.abs() >= 1_000_000.0 {
        push_f2(buf, v / 1_000_000.0);
        push_b(buf, b"M");
    } else if v.abs() >= 1_000.0 {
        push_f2(buf, v / 1_000.0);
        push_b(buf, b"k");
    } else {
        push_f2(buf, v);
    }
    push_b(buf, b"</text>");
}

pub fn bar_color(p: &Prepared, i: usize) -> u32 {
    if p.is_total[i] {
        COLOR_TOTAL
    } else if p.values[i] >= 0.0 {
        COLOR_POS
    } else {
        COLOR_NEG
    }
}

pub fn bar_x(l: &Layout, pad_l: i32, i: usize) -> i32 {
    pad_l + i as i32 * l.bar_step + l.bar_step / 2 - l.bar_w / 2
}

pub fn data_attrs(buf: &mut Vec<u8>, p: &Prepared, i: usize) {
    push_b(buf, b" data-idx=\"");
    push_i(buf, i as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, p.values[i]);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, &p.labels[i]);
    push_b(buf, b"\"");
}

pub fn finalize(mut buf: Vec<u8>, cfg: &WaterfallConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
