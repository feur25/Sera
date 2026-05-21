use super::common::{prepare, open_svg, arc_path, color_for, value_text, label_text, finalize};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{push_b, push_f2, hex6};

#[crate::chart_demo("value=72, min_val=0, max_val=100, label=\"Score\"")]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare(cfg);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    let start_a: f64 = std::f64::consts::PI;
    let total = std::f64::consts::PI;
    let hx_track = hex6(0xE5E7EB);
    arc_path(&mut b, p.cx, p.cy, p.radius, start_a, start_a - total, 0xE5E7EB, p.arc_w, 1.0);
    let _ = hx_track;
    if p.frac > 0.001 {
        let a_end = start_a - p.frac * total;
        let col = color_for(&p, p.frac);
        arc_path(&mut b, p.cx, p.cy, p.radius, start_a, a_end, col, p.arc_w + 2.0, 1.0);
    }
    let r_in = p.radius - p.arc_w * 0.65;
    let r_out_short = p.radius + p.arc_w * 0.55;
    let r_out_long = p.radius + p.arc_w * 0.85;
    for i in 0..=20 {
        let f = i as f64 / 20.0;
        let a = start_a - f * total;
        let r_out = if i % 2 == 0 { r_out_long } else { r_out_short };
        let xa = p.cx + r_in * a.cos();
        let ya = p.cy - r_in * a.sin();
        let xb = p.cx + r_out * a.cos();
        let yb = p.cy - r_out * a.sin();
        push_b(&mut b, b"<line x1=\""); push_f2(&mut b, xa);
        push_b(&mut b, b"\" y1=\""); push_f2(&mut b, ya);
        push_b(&mut b, b"\" x2=\""); push_f2(&mut b, xb);
        push_b(&mut b, b"\" y2=\""); push_f2(&mut b, yb);
        push_b(&mut b, b"\" stroke=\"#475569\" stroke-width=\"1\"/>");
        if i % 5 == 0 {
            let r_lab = r_out + 12.0;
            let xl = p.cx + r_lab * a.cos();
            let yl = p.cy - r_lab * a.sin();
            push_b(&mut b, b"<text x=\""); push_f2(&mut b, xl);
            push_b(&mut b, b"\" y=\""); push_f2(&mut b, yl + 3.0);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
            let v = cfg.min_val + f * (cfg.max_val - cfg.min_val);
            let s = format!("{:.0}", v); b.extend_from_slice(s.as_bytes());
            push_b(&mut b, b"</text>");
        }
    }
    let na = start_a - p.frac * total;
    let nl = p.radius * 0.7;
    let nx = p.cx + nl * na.cos();
    let ny = p.cy - nl * na.sin();
    push_b(&mut b, b"<line x1=\""); push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" y1=\""); push_f2(&mut b, p.cy);
    push_b(&mut b, b"\" x2=\""); push_f2(&mut b, nx);
    push_b(&mut b, b"\" y2=\""); push_f2(&mut b, ny);
    push_b(&mut b, b"\" stroke=\"#0f172a\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>");
    push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" cy=\""); push_f2(&mut b, p.cy);
    push_b(&mut b, b"\" r=\"6\" fill=\"#0f172a\"/>");
    value_text(&mut b, cfg, p.cx, p.cy + 50.0, 22);
    label_text(&mut b, cfg, p.cx, p.cy + 68.0);
    finalize(b, cfg)
}

