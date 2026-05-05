use super::config::GaugeConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, escape_xml};
use crate::html::hover::{build_chart_html, slots_to_json};

pub struct Prepared {
    pub range: f64,
    pub frac: f64,
    pub frac_cmp: f64,
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
    pub arc_w: f64,
    pub thresholds: Vec<(f64, u32)>,
}

pub fn prepare_with(cfg: &GaugeConfig, cy_factor: f64, r_factor: f64) -> Prepared {
    let range = (cfg.max_val - cfg.min_val).max(1e-9);
    let frac = ((cfg.value - cfg.min_val) / range).clamp(0.0, 1.0);
    let frac_cmp = ((cfg.comparison - cfg.min_val) / range).clamp(0.0, 1.0);
    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 * cy_factor;
    let radius = (cfg.width.min(cfg.height) as f64) * r_factor;
    let arc_w = radius * 0.18;
    let thresholds: Vec<(f64, u32)> = if cfg.thresholds.is_empty() {
        vec![(0.0, 0x10B981), (0.6, 0xF59E0B), (0.8, 0xEF4444)]
    } else {
        cfg.thresholds.iter().map(|&(v, c)| (((v - cfg.min_val) / range).clamp(0.0, 1.0), c)).collect()
    };
    Prepared { range, frac, frac_cmp, cx, cy, radius, arc_w, thresholds }
}

pub fn prepare(cfg: &GaugeConfig) -> Prepared { prepare_with(cfg, 0.65, 0.38) }

pub fn open_svg(buf: &mut Vec<u8>, cfg: &GaugeConfig) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, cfg.width);
    push_b(buf, b"\" height=\"");
    push_i(buf, cfg.height);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, cfg.width);
    push_b(buf, b" ");
    push_i(buf, cfg.height);
    push_b(buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(buf, b"<text x=\""); push_i(buf, cfg.width / 2);
        push_b(buf, b"\" y=\"28\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"15\" fill=\"#1e293b\">");
        escape_xml(buf, cfg.title);
        push_b(buf, b"</text>");
    }
}

pub fn arc_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64, color: u32, w: f64, opacity: f64) {
    let x1 = cx + r * a0.cos();
    let y1 = cy - r * a0.sin();
    let x2 = cx + r * a1.cos();
    let y2 = cy - r * a1.sin();
    let sweep_diff = (a0 - a1).abs();
    let large = if sweep_diff > std::f64::consts::PI { 1 } else { 0 };
    let sweep_dir = if a1 < a0 { 0 } else { 1 };
    let hx = hex6(color);
    push_b(buf, b"<path d=\"M ");
    push_f2(buf, x1); push_b(buf, b" "); push_f2(buf, y1);
    push_b(buf, b" A "); push_f2(buf, r); push_b(buf, b" "); push_f2(buf, r);
    push_b(buf, b" 0 "); push_i(buf, large); push_b(buf, b" "); push_i(buf, sweep_dir); push_b(buf, b" ");
    push_f2(buf, x2); push_b(buf, b" "); push_f2(buf, y2);
    push_b(buf, b"\" fill=\"none\" stroke=\"#"); buf.extend_from_slice(&hx);
    push_b(buf, b"\" stroke-width=\""); push_f2(buf, w);
    push_b(buf, b"\" stroke-linecap=\"round\" opacity=\""); push_f2(buf, opacity);
    push_b(buf, b"\"/>");
}

pub fn color_for(p: &Prepared, frac: f64) -> u32 {
    let mut c = p.thresholds[0].1;
    for &(t, col) in &p.thresholds { if frac >= t { c = col; } }
    c
}

pub fn value_text(buf: &mut Vec<u8>, cfg: &GaugeConfig, x: f64, y: f64, size: i32) {
    push_b(buf, b"<text x=\""); push_f2(buf, x);
    push_b(buf, b"\" y=\""); push_f2(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"800\" font-size=\"");
    push_i(buf, size);
    push_b(buf, b"\" fill=\"#1e293b\">");
    let s = format!("{:.1}", cfg.value);
    buf.extend_from_slice(s.as_bytes());
    push_b(buf, b"</text>");
}

pub fn label_text(buf: &mut Vec<u8>, cfg: &GaugeConfig, x: f64, y: f64) {
    if cfg.label.is_empty() { return; }
    push_b(buf, b"<text x=\""); push_f2(buf, x);
    push_b(buf, b"\" y=\""); push_f2(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#64748b\">");
    escape_xml(buf, cfg.label);
    push_b(buf, b"</text>");
}

pub fn min_max_labels(buf: &mut Vec<u8>, cfg: &GaugeConfig, p: &Prepared) {
    push_b(buf, b"<text x=\""); push_f2(buf, p.cx - p.radius - 10.0);
    push_b(buf, b"\" y=\""); push_f2(buf, p.cy + 16.0);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let s = format!("{:.0}", cfg.min_val);
    buf.extend_from_slice(s.as_bytes());
    push_b(buf, b"</text>");
    push_b(buf, b"<text x=\""); push_f2(buf, p.cx + p.radius + 10.0);
    push_b(buf, b"\" y=\""); push_f2(buf, p.cy + 16.0);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let s = format!("{:.0}", cfg.max_val);
    buf.extend_from_slice(s.as_bytes());
    push_b(buf, b"</text>");
}

pub fn finalize(buf: Vec<u8>, cfg: &GaugeConfig) -> String {
    let mut buf = buf;
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
