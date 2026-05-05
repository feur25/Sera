use super::common::{prepare_with, open_svg, arc_path, color_for, value_text, label_text, finalize};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{push_b, push_f2};

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare_with(cfg, 0.55, 0.36);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    let start_a: f64 = std::f64::consts::PI + std::f64::consts::PI / 4.0;
    let total = std::f64::consts::PI * 1.5;
    for i in 0..p.thresholds.len() {
        let f0 = p.thresholds[i].0;
        let f1 = if i + 1 < p.thresholds.len() { p.thresholds[i + 1].0 } else { 1.0 };
        let a0 = start_a - f0 * total;
        let a1 = start_a - f1 * total;
        arc_path(&mut b, p.cx, p.cy, p.radius, a0, a1, p.thresholds[i].1, p.arc_w, 0.22);
    }
    if p.frac > 0.001 {
        let a_end = start_a - p.frac * total;
        let col = color_for(&p, p.frac);
        arc_path(&mut b, p.cx, p.cy, p.radius, start_a, a_end, col, p.arc_w + 2.0, 1.0);
    }
    value_text(&mut b, cfg, p.cx, p.cy + 8.0, 30);
    label_text(&mut b, cfg, p.cx, p.cy + 30.0);
    push_b(&mut b, b"<text x=\""); push_f2(&mut b, p.cx - p.radius * 0.78);
    push_b(&mut b, b"\" y=\""); push_f2(&mut b, p.cy + p.radius * 0.85);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let s = format!("{:.0}", cfg.min_val); b.extend_from_slice(s.as_bytes());
    push_b(&mut b, b"</text>");
    push_b(&mut b, b"<text x=\""); push_f2(&mut b, p.cx + p.radius * 0.78);
    push_b(&mut b, b"\" y=\""); push_f2(&mut b, p.cy + p.radius * 0.85);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let s = format!("{:.0}", cfg.max_val); b.extend_from_slice(s.as_bytes());
    push_b(&mut b, b"</text>");
    finalize(b, cfg)
}
