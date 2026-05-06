use super::common::{prepare, finalize, color_for};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_open_rescalable, svg_title, palette_color};

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(4096);
    svg_open_rescalable(&mut b, cfg.width, cfg.height, 0, 0, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, cfg.width / 2, 26);
    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0 + 10.0;
    let r_outer = (cfg.width.min(cfg.height) as f64) * 0.42;
    let r_inner = r_outer * 0.32;
    let max_val = p.vmax.max(1.0);
    let min_val = p.vmin.min(0.0);
    let range = (max_val - min_val).max(1.0);
    for ring in 1..=4 {
        let rr = r_inner + (r_outer - r_inner) * ring as f64 / 4.0;
        push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, cx);
        push_b(&mut b, b"\" cy=\""); push_f2(&mut b, cy);
        push_b(&mut b, b"\" r=\""); push_f2(&mut b, rr);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"2,3\"/>");
    }
    let two_pi = std::f64::consts::TAU;
    for i in 0..p.n {
        let theta = -std::f64::consts::FRAC_PI_2 + two_pi * i as f64 / p.n as f64;
        let frac = ((p.values[i] - min_val) / range).clamp(0.0, 1.0);
        let r_end = r_inner + (r_outer - r_inner) * frac;
        let x0 = cx + r_inner * theta.cos();
        let y0 = cy + r_inner * theta.sin();
        let x1 = cx + r_end * theta.cos();
        let y1 = cy + r_end * theta.sin();
        let col = color_for(cfg, &p, i);
        let hx = hex6(col);
        push_b(&mut b, b"<line data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, p.values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" x1=\""); push_f2(&mut b, x0);
        push_b(&mut b, b"\" y1=\""); push_f2(&mut b, y0);
        push_b(&mut b, b"\" x2=\""); push_f2(&mut b, x1);
        push_b(&mut b, b"\" y2=\""); push_f2(&mut b, y1);
        push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2.4\" stroke-linecap=\"round\"/>");
        push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, x1);
        push_b(&mut b, b"\" cy=\""); push_f2(&mut b, y1);
        push_b(&mut b, b"\" r=\"5\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        let r_lab = r_outer + 16.0;
        let xl = cx + r_lab * theta.cos();
        let yl = cy + r_lab * theta.sin();
        push_b(&mut b, b"<text x=\""); push_f2(&mut b, xl);
        push_b(&mut b, b"\" y=\""); push_f2(&mut b, yl + 3.0);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut b, truncate(&p.labels[i], 12));
        push_b(&mut b, b"</text>");
    }
    let _ = palette_color;
    finalize(b, cfg)
}
