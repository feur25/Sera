use super::common::{prepare, open_svg, arc_path, color_for, value_text, label_text, min_max_labels, finalize};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{push_b, push_f2, hex6};

pub const DEMO_KWARGS: &str = "value=72, min_val=0, max_val=100, label=\"Score\"";
pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare(cfg);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    let start_a: f64 = std::f64::consts::PI;
    let total = std::f64::consts::PI;
    for i in 0..p.thresholds.len() {
        let f0 = p.thresholds[i].0;
        let f1 = if i + 1 < p.thresholds.len() { p.thresholds[i + 1].0 } else { 1.0 };
        let a0 = start_a - f0 * total;
        let a1 = start_a - f1 * total;
        arc_path(&mut b, p.cx, p.cy, p.radius, a0, a1, p.thresholds[i].1, p.arc_w, 0.25);
    }
    if p.frac > 0.001 {
        let a_end = start_a - p.frac * total;
        let col = color_for(&p, p.frac);
        arc_path(&mut b, p.cx, p.cy, p.radius, start_a, a_end, col, p.arc_w + 2.0, 1.0);
        let ax = p.cx + p.radius * a_end.cos();
        let ay = p.cy - p.radius * a_end.sin();
        let hx = hex6(col);
        push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, ax);
        push_b(&mut b, b"\" cy=\""); push_f2(&mut b, ay);
        push_b(&mut b, b"\" r=\""); push_f2(&mut b, p.arc_w * 0.4);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\"/>");
    }
    let needle_a = start_a - p.frac * total;
    let nl = p.radius * 0.72;
    let nx = p.cx + nl * needle_a.cos();
    let ny = p.cy - nl * needle_a.sin();
    push_b(&mut b, b"<line x1=\""); push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" y1=\""); push_f2(&mut b, p.cy);
    push_b(&mut b, b"\" x2=\""); push_f2(&mut b, nx);
    push_b(&mut b, b"\" y2=\""); push_f2(&mut b, ny);
    push_b(&mut b, b"\" stroke=\"#1e293b\" stroke-width=\"2.5\" stroke-linecap=\"round\"/>");
    push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" cy=\""); push_f2(&mut b, p.cy);
    push_b(&mut b, b"\" r=\"8\" fill=\"#1e293b\"/>");
    push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" cy=\""); push_f2(&mut b, p.cy);
    push_b(&mut b, b"\" r=\"4\" fill=\"#fff\"/>");
    value_text(&mut b, cfg, p.cx, p.cy + 30.0, 28);
    label_text(&mut b, cfg, p.cx, p.cy + 50.0);
    min_max_labels(&mut b, cfg, &p);
    finalize(b, cfg)
}


