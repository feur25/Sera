use super::common::{arc_path, color_for, finalize, label_text, open_svg, prepare, value_text};
use super::config::GaugeConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2};

#[crate::chart_demo("value=72, min_val=0, max_val=100, label=\"Score\"")]

pub fn render(cfg: &GaugeConfig) -> String {
    let p = prepare(cfg);
    let mut b = Vec::<u8>::with_capacity(4096);
    open_svg(&mut b, cfg);
    let start_a: f64 = std::f64::consts::PI;
    let total = std::f64::consts::PI;
    let r_outer = p.radius;
    let r_inner = p.radius * 0.62;
    let aw = p.arc_w * 0.7;
    let hx_track = hex6(0xE5E7EB);
    arc_path(
        &mut b,
        p.cx,
        p.cy,
        r_outer,
        start_a,
        start_a - total,
        0xE5E7EB,
        aw,
        1.0,
    );
    arc_path(
        &mut b,
        p.cx,
        p.cy,
        r_inner,
        start_a,
        start_a - total,
        0xE5E7EB,
        aw,
        1.0,
    );
    let _ = hx_track;
    if p.frac > 0.001 {
        let a_end = start_a - p.frac * total;
        let col = color_for(&p, p.frac);
        arc_path(
            &mut b,
            p.cx,
            p.cy,
            r_outer,
            start_a,
            a_end,
            col,
            aw + 1.5,
            1.0,
        );
    }
    if p.frac_cmp > 0.001 {
        let a_end = start_a - p.frac_cmp * total;
        arc_path(
            &mut b,
            p.cx,
            p.cy,
            r_inner,
            start_a,
            a_end,
            0x94A3B8,
            aw + 1.0,
            0.95,
        );
    } else {
        let a_end = start_a - p.frac * 0.7 * total;
        arc_path(
            &mut b,
            p.cx,
            p.cy,
            r_inner,
            start_a,
            a_end,
            0x94A3B8,
            aw + 1.0,
            0.6,
        );
    }
    value_text(&mut b, cfg, p.cx, p.cy + 18.0, 28);
    label_text(&mut b, cfg, p.cx, p.cy + 40.0);
    push_b(&mut b, b"<text x=\"");
    push_f2(&mut b, p.cx);
    push_b(&mut b, b"\" y=\"");
    push_f2(&mut b, p.cy - 8.0);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
    let s = if cfg.comparison > 0.0 {
        format!("vs {:.0}", cfg.comparison)
    } else {
        format!("{:.0}/{:.0}", cfg.value, cfg.max_val)
    };
    b.extend_from_slice(s.as_bytes());
    push_b(&mut b, b"</text>");
    finalize(b, cfg)
}
