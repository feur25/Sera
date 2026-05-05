use super::common::{prepare, open_svg, draw_grid, series_points, polygon_pts, dots, draw_legend, finalize};
use super::config::RadarConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, hex6};

pub fn render(cfg: &RadarConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(4096 + p.n_ser * p.n_axes * 90);
    open_svg(&mut b, cfg, &p);

    push_b(&mut b, b"<defs>");
    for si in 0..p.n_ser {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut b, b"<radialGradient id=\"sp-rad-"); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" cx=\"50%\" cy=\"50%\" r=\"50%\">");
        push_b(&mut b, b"<stop offset=\"0%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.85\"/>");
        push_b(&mut b, b"<stop offset=\"100%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.05\"/>");
        push_b(&mut b, b"</radialGradient>");
    }
    push_b(&mut b, b"</defs>");

    draw_grid(&mut b, cfg, &p);

    for si in 0..p.n_ser {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let pts = series_points(cfg, &p, si);
        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, si as i32); push_b(&mut b, b"\">");
        polygon_pts(&mut b, &pts);
        push_b(&mut b, b" fill=\"url(#sp-rad-"); push_i(&mut b, si as i32);
        push_b(&mut b, b")\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.8\" stroke-linejoin=\"round\"/>");
        dots(&mut b, &pts, &hx, 3.0);
        push_b(&mut b, b"</g>");
        let _ = push_f2;
    }

    draw_legend(&mut b, cfg, &p);
    finalize(b, cfg)
}
