use super::common::{prepare, series_points, polygon_pts, draw_legend, finalize, angle_at, project};
use super::config::RadarConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open, svg_title};
use std::f64::consts::PI;

fn neon_color(si: usize) -> u32 {
    const COLS: [u32; 8] = [
        0x22D3EE, 0xA78BFA, 0x34D399, 0xF472B6, 0xFBBF24, 0x60A5FA, 0xFB7185, 0x4ADE80,
    ];
    COLS[si % COLS.len()]
}

pub fn render(cfg: &RadarConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let cx = p.layout.plot_cx;
    let cy = p.layout.plot_cy;
    let r = p.layout.r;
    let mut b = Vec::<u8>::with_capacity(8192 + p.n_ser * p.n_axes * 80);

    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, cx as i32, if p.layout.title_h > 0 { 24 } else { 0 });

    push_b(&mut b, b"<defs>");
    push_b(&mut b, b"<filter id=\"dlxradf\" x=\"-50%\" y=\"-50%\" width=\"200%\" height=\"200%\">");
    push_b(&mut b, b"<feGaussianBlur stdDeviation=\"4\" result=\"b\"/>");
    push_b(&mut b, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut b, b"</filter>");
    push_b(&mut b, b"<filter id=\"dlxradg\" x=\"-60%\" y=\"-60%\" width=\"220%\" height=\"220%\">");
    push_b(&mut b, b"<feGaussianBlur stdDeviation=\"3\" result=\"b\"/>");
    push_b(&mut b, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut b, b"</filter>");
    for si in 0..p.n_ser {
        let col = if !cfg.palette.is_empty() { palette_color(cfg.palette, si) } else { neon_color(si) };
        push_b(&mut b, b"<radialGradient id=\"dlxradrg"); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" cx=\"50%\" cy=\"50%\" r=\"50%\">");
        push_b(&mut b, b"<stop offset=\"0\" stop-color=\"#"); b.extend_from_slice(&hex6(col)); push_b(&mut b, b"\" stop-opacity=\"0.5\"/>");
        push_b(&mut b, b"<stop offset=\"1\" stop-color=\"#"); b.extend_from_slice(&hex6(col)); push_b(&mut b, b"\" stop-opacity=\"0.06\"/>");
        push_b(&mut b, b"</radialGradient>");
    }
    push_b(&mut b, b"</defs>");

    let n_rings = 5;
    let label_pad = 24.0;
    for ring in 1..=n_rings {
        let frac = ring as f64 / n_rings as f64;
        let rr = r * frac;
        push_b(&mut b, b"<polygon points=\"");
        for ai in 0..p.n_axes {
            let a = angle_at(ai, p.n_axes);
            if ai > 0 { b.push(b' '); }
            push_f2(&mut b, cx + rr * a.cos()); b.push(b','); push_f2(&mut b, cy - rr * a.sin());
        }
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"0.6\"/>");
        let lx = cx + 5.0;
        let ly = cy - rr - 2.0;
        push_b(&mut b, b"<text x=\""); push_f2(&mut b, lx);
        push_b(&mut b, b"\" y=\""); push_f2(&mut b, ly);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#334155\">");
        let val = frac * p.global_max;
        if val.fract() == 0.0 { push_i(&mut b, val as i32); } else { push_f2(&mut b, val); }
        push_b(&mut b, b"</text>");
    }

    for ai in 0..p.n_axes {
        let a = angle_at(ai, p.n_axes);
        let epx = cx + r * a.cos();
        let epy = cy - r * a.sin();
        push_b(&mut b, b"<line x1=\""); push_f2(&mut b, cx);
        push_b(&mut b, b"\" y1=\""); push_f2(&mut b, cy);
        push_b(&mut b, b"\" x2=\""); push_f2(&mut b, epx);
        push_b(&mut b, b"\" y2=\""); push_f2(&mut b, epy);
        push_b(&mut b, b"\" stroke=\"#1e3a5f\" stroke-width=\"1\"/>");
        let anchor = if a.cos().abs() < 0.12 { "middle" } else if a.cos() > 0.0 { "start" } else { "end" };
        let lx = cx + (r + label_pad) * a.cos();
        let ly = cy - (r + label_pad) * a.sin();
        let dy = if a.sin() > 0.3 { -4.0 } else if a.sin() < -0.3 { 12.0 } else { 4.0 };
        push_b(&mut b, b"<text x=\""); push_f2(&mut b, lx);
        push_b(&mut b, b"\" y=\""); push_f2(&mut b, ly + dy);
        push_b(&mut b, b"\" text-anchor=\""); b.extend_from_slice(anchor.as_bytes());
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#94a3b8\">");
        escape_xml(&mut b, &cfg.axes[ai]);
        push_b(&mut b, b"</text>");
    }

    for si in 0..p.n_ser {
        let col = if !cfg.palette.is_empty() { palette_color(cfg.palette, si) } else { neon_color(si) };
        let hx = hex6(col);
        let pts = series_points(cfg, &p, si);
        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, si as i32); push_b(&mut b, b"\">");
        polygon_pts(&mut b, &pts);
        push_b(&mut b, b" fill=\"url(#dlxradrg"); push_i(&mut b, si as i32);
        push_b(&mut b, b")\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\" filter=\"url(#dlxradg)\"/>");
        for &(px, py) in &pts {
            push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, px);
            push_b(&mut b, b"\" cy=\""); push_f2(&mut b, py);
            push_b(&mut b, b"\" r=\"4\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" filter=\"url(#dlxradf)\"/>");
        }
        push_b(&mut b, b"</g>");
    }

    let _ = (project, PI);
    draw_legend(&mut b, cfg, &p);
    finalize(b, cfg)
}


