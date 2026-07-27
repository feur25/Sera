use super::common::{
    arc_path, color_for, finalize, label_text, min_max_labels, open_svg, prepare, value_text,
};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2};

#[crate::chart_demo("value=72, min_val=0, max_val=100, label=\"Score\"")]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare(cfg);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    draw(&mut b, cfg, &p);
    finalize(b, cfg)
}

pub fn draw(b: &mut Vec<u8>, cfg: &GaugeConfig, p: &super::common::Prepared) {
    let start_a: f64 = std::f64::consts::PI;
    let total = std::f64::consts::PI;
    for i in 0..p.thresholds.len() {
        let f0 = p.thresholds[i].0;
        let f1 = if i + 1 < p.thresholds.len() {
            p.thresholds[i + 1].0
        } else {
            1.0
        };
        let a0 = start_a - f0 * total;
        let a1 = start_a - f1 * total;
        arc_path(
            b,
            p.cx,
            p.cy,
            p.radius,
            a0,
            a1,
            p.thresholds[i].1,
            p.arc_w,
            0.25,
        );
    }
    if p.frac > 0.001 {
        let a_end = start_a - p.frac * total;
        let col = color_for(&p, p.frac);
        arc_path(
            b,
            p.cx,
            p.cy,
            p.radius,
            start_a,
            a_end,
            col,
            p.arc_w + 2.0,
            1.0,
        );
        let ax = p.cx + p.radius * a_end.cos();
        let ay = p.cy - p.radius * a_end.sin();
        let hx = hex6(col);
        push_b(b, b"<circle cx=\"");
        push_f2(b, ax);
        push_b(b, b"\" cy=\"");
        push_f2(b, ay);
        push_b(b, b"\" r=\"");
        push_f2(b, p.arc_w * 0.4);
        push_b(b, b"\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(b, b"\"/>");
    }
    let needle_a = start_a - p.frac * total;
    let nl = p.radius * 0.72;
    let nx = p.cx + nl * needle_a.cos();
    let ny = p.cy - nl * needle_a.sin();
    push_b(b, b"<line x1=\"");
    push_f2(b, p.cx);
    push_b(b, b"\" y1=\"");
    push_f2(b, p.cy);
    push_b(b, b"\" x2=\"");
    push_f2(b, nx);
    push_b(b, b"\" y2=\"");
    push_f2(b, ny);
    push_b(
        b,
        b"\" stroke=\"#1e293b\" stroke-width=\"2.5\" stroke-linecap=\"round\"/>",
    );
    push_b(b, b"<circle cx=\"");
    push_f2(b, p.cx);
    push_b(b, b"\" cy=\"");
    push_f2(b, p.cy);
    push_b(b, b"\" r=\"8\" fill=\"#1e293b\"/>");
    push_b(b, b"<circle cx=\"");
    push_f2(b, p.cx);
    push_b(b, b"\" cy=\"");
    push_f2(b, p.cy);
    push_b(b, b"\" r=\"4\" fill=\"#fff\"/>");
    value_text(b, cfg, p.cx, p.cy + 30.0, 28);
    label_text(b, cfg, p.cx, p.cy + 50.0);
    min_max_labels(b, cfg, &p);
}
