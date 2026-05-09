use super::common::{prepare, open_svg, draw_grid, draw_legend, finalize, angle_at, project};
use super::config::RadarConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, hex6};
use std::f64::consts::PI;

pub fn render(cfg: &RadarConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(4096 + p.n_ser * p.n_axes * 120);
    open_svg(&mut b, cfg, &p);
    draw_grid(&mut b, cfg, &p);

    let cx = p.layout.plot_cx;
    let cy = p.layout.plot_cy;
    let r = p.layout.r;
    let sector = 2.0 * PI / p.n_axes as f64;
    let group_span = sector * 0.78;
    let bar_span = group_span / p.n_ser as f64;

    for si in 0..p.n_ser {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, si as i32); push_b(&mut b, b"\">");
        for ai in 0..p.n_axes {
            let v = cfg.series[si].1.get(ai).copied().unwrap_or(0.0).max(0.0);
            let frac = (v / p.global_max).min(1.0);
            if frac <= 0.0 { continue; }
            let center = angle_at(ai, p.n_axes);
            let half = bar_span / 2.0;
            let offset = -group_span / 2.0 + bar_span / 2.0 + si as f64 * bar_span;
            let a_mid = center - offset;
            let a0 = a_mid - half;
            let a1 = a_mid + half;
            let outer_r = r * frac;
            let (x0o, y0o) = project(cx, cy, outer_r, 1.0, a0);
            let (x1o, y1o) = project(cx, cy, outer_r, 1.0, a1);
            let (x0i, y0i) = (cx + 0.0_f64, cy - 0.0_f64);
            let _ = (x0i, y0i);
            push_b(&mut b, b"<path d=\"M ");
            push_f2(&mut b, cx); b.push(b' '); push_f2(&mut b, cy);
            push_b(&mut b, b" L "); push_f2(&mut b, x0o); b.push(b' '); push_f2(&mut b, y0o);
            push_b(&mut b, b" A "); push_f2(&mut b, outer_r); b.push(b' '); push_f2(&mut b, outer_r);
            push_b(&mut b, b" 0 0 0 "); push_f2(&mut b, x1o); b.push(b' '); push_f2(&mut b, y1o);
            push_b(&mut b, b" Z\" fill=\"#"); b.extend_from_slice(&hx);
            let op = ((cfg.fill_opacity as f64 / 255.0) * 1.6).clamp(0.35, 0.95);
            push_b(&mut b, b"\" fill-opacity=\""); push_f2(&mut b, op);
            push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1\"/>");
        }
        push_b(&mut b, b"</g>");
    }

    draw_legend(&mut b, cfg, &p);
    finalize(b, cfg)
}


