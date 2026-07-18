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
    push_b(&mut b, b"<defs><filter id=\"sp-g-glow\" x=\"-50%\" y=\"-50%\" width=\"200%\" height=\"200%\"><feGaussianBlur stdDeviation=\"6\" result=\"blur\"/><feMerge><feMergeNode in=\"blur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter></defs>");
    let start_a: f64 = std::f64::consts::PI;
    let total = std::f64::consts::PI;
    arc_path(
        &mut b,
        p.cx,
        p.cy,
        p.radius,
        start_a,
        start_a - total,
        0x1E293B,
        p.arc_w + 2.0,
        0.4,
    );
    if p.frac > 0.001 {
        let a_end = start_a - p.frac * total;
        let col = color_for(&p, p.frac);
        let hx = hex6(col);
        let x1 = p.cx + p.radius * start_a.cos();
        let y1 = p.cy - p.radius * start_a.sin();
        let x2 = p.cx + p.radius * a_end.cos();
        let y2 = p.cy - p.radius * a_end.sin();
        push_b(&mut b, b"<path d=\"M ");
        push_f2(&mut b, x1);
        push_b(&mut b, b" ");
        push_f2(&mut b, y1);
        push_b(&mut b, b" A ");
        push_f2(&mut b, p.radius);
        push_b(&mut b, b" ");
        push_f2(&mut b, p.radius);
        push_b(&mut b, b" 0 0 1 ");
        push_f2(&mut b, x2);
        push_b(&mut b, b" ");
        push_f2(&mut b, y2);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"");
        push_f2(&mut b, p.arc_w + 4.0);
        push_b(
            &mut b,
            b"\" stroke-linecap=\"round\" filter=\"url(#sp-g-glow)\"/>",
        );
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, x2);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, y2);
        push_b(&mut b, b"\" r=\"");
        push_f2(&mut b, p.arc_w * 0.6);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" filter=\"url(#sp-g-glow)\"/>");
    }
    value_text(&mut b, cfg, p.cx, p.cy + 30.0, 28);
    label_text(&mut b, cfg, p.cx, p.cy + 50.0);
    min_max_labels(&mut b, cfg, &p);
    finalize(b, cfg)
}
