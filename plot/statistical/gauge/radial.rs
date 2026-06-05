use super::common::{color_for, finalize, label_text, open_svg, prepare_with, value_text};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};

#[crate::chart_demo("value=72, min_val=0, max_val=100, label=\"Score\"")]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare_with(cfg, 0.55, 0.36);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    let hx_track = hex6(0xE5E7EB);
    push_b(&mut b, b"<circle cx=\"");
    push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" cy=\"");
    push_f2(&mut b, p.cy);
    push_b(&mut b, b"\" r=\"");
    push_f2(&mut b, p.radius);
    push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
    b.extend_from_slice(&hx_track);
    push_b(&mut b, b"\" stroke-width=\"");
    push_f2(&mut b, p.arc_w);
    push_b(&mut b, b"\"/>");
    if p.frac > 0.001 {
        let col = color_for(&p, p.frac);
        let hx = hex6(col);
        let circ = 2.0 * std::f64::consts::PI * p.radius;
        let dash = circ * p.frac;
        let gap = circ - dash;
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, p.cx);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, p.cy);
        push_b(&mut b, b"\" r=\"");
        push_f2(&mut b, p.radius);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"");
        push_f2(&mut b, p.arc_w + 2.0);
        push_b(&mut b, b"\" stroke-linecap=\"round\" stroke-dasharray=\"");
        push_f2(&mut b, dash);
        push_b(&mut b, b" ");
        push_f2(&mut b, gap);
        push_b(&mut b, b"\" transform=\"rotate(-90 ");
        push_f2(&mut b, p.cx);
        push_b(&mut b, b" ");
        push_f2(&mut b, p.cy);
        push_b(&mut b, b")\"/>");
    }
    value_text(&mut b, cfg, p.cx, p.cy + 6.0, 28);
    label_text(&mut b, cfg, p.cx, p.cy + 28.0);
    push_b(&mut b, b"<text x=\"");
    push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" y=\"");
    push_f2(&mut b, p.cy - 6.0);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let s = format!("{:.0}%", p.frac * 100.0);
    b.extend_from_slice(s.as_bytes());
    push_b(&mut b, b"</text>");
    let _ = push_i;
    finalize(b, cfg)
}
