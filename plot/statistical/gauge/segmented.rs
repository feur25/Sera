use super::common::{prepare, open_svg, color_for, value_text, label_text, min_max_labels, finalize};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{push_b, push_f2, hex6};

#[crate::chart_demo("value=72, min_val=0, max_val=100, label=\"Score\"")]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare(cfg);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    let start_a: f64 = std::f64::consts::PI;
    let total = std::f64::consts::PI;
    let n_seg = 14;
    let gap = 0.012_f64;
    let active_col = color_for(&p, p.frac);
    let hx_active = hex6(active_col);
    let hx_off = hex6(0xE5E7EB);
    for i in 0..n_seg {
        let f0 = i as f64 / n_seg as f64 + gap;
        let f1 = (i as f64 + 1.0) / n_seg as f64 - gap;
        let a0 = start_a - f0 * total;
        let a1 = start_a - f1 * total;
        let x1 = p.cx + p.radius * a0.cos();
        let y1 = p.cy - p.radius * a0.sin();
        let x2 = p.cx + p.radius * a1.cos();
        let y2 = p.cy - p.radius * a1.sin();
        let mid = (i as f64 + 0.5) / n_seg as f64;
        let on = mid <= p.frac;
        push_b(&mut b, b"<path d=\"M ");
        push_f2(&mut b, x1); push_b(&mut b, b" "); push_f2(&mut b, y1);
        push_b(&mut b, b" A "); push_f2(&mut b, p.radius); push_b(&mut b, b" "); push_f2(&mut b, p.radius);
        push_b(&mut b, b" 0 0 1 "); push_f2(&mut b, x2); push_b(&mut b, b" "); push_f2(&mut b, y2);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
        if on { b.extend_from_slice(&hx_active); } else { b.extend_from_slice(&hx_off); }
        push_b(&mut b, b"\" stroke-width=\""); push_f2(&mut b, p.arc_w + 2.0);
        push_b(&mut b, b"\" stroke-linecap=\"round\"/>");
    }
    value_text(&mut b, cfg, p.cx, p.cy + 30.0, 28);
    label_text(&mut b, cfg, p.cx, p.cy + 50.0);
    min_max_labels(&mut b, cfg, &p);
    finalize(b, cfg)
}

