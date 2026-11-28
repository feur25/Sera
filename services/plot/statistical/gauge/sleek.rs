use super::common::{arc_path, color_for, finalize, label_text, open_svg, prepare};
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
    let hx_track = hex6(0xE5E7EB);
    let a0 = start_a;
    let a1 = start_a - total;
    let x1 = p.cx + p.radius * a0.cos();
    let y1 = p.cy - p.radius * a0.sin();
    let x2 = p.cx + p.radius * a1.cos();
    let y2 = p.cy - p.radius * a1.sin();
    push_b(b, b"<path d=\"M ");
    push_f2(b, x1);
    push_b(b, b" ");
    push_f2(b, y1);
    push_b(b, b" A ");
    push_f2(b, p.radius);
    push_b(b, b" ");
    push_f2(b, p.radius);
    push_b(b, b" 0 0 1 ");
    push_f2(b, x2);
    push_b(b, b" ");
    push_f2(b, y2);
    push_b(b, b"\" fill=\"none\" stroke=\"#");
    b.extend_from_slice(&hx_track);
    push_b(b, b"\" stroke-width=\"");
    push_f2(b, p.arc_w + 4.0);
    push_b(b, b"\" stroke-linecap=\"round\"/>");
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
            p.arc_w + 4.0,
            1.0,
        );
    }
    push_b(b, b"<text class=\"sp-val\" x=\"");
    push_f2(b, p.cx);
    push_b(b, b"\" y=\"");
    push_f2(b, p.cy + 10.0);
    push_b(b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"800\" font-size=\"40\" fill=\"#0f172a\">");
    let s = format!("{:.0}", cfg.value);
    b.extend_from_slice(s.as_bytes());
    push_b(b, b"</text>");
    label_text(b, cfg, p.cx, p.cy + 36.0);
}
