use super::common::{
    dots, draw_grid, draw_legend, finalize, open_svg, polygon_pts, prepare, series_points,
};
use super::config::RadarConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("axes=[\"Speed\",\"Power\",\"Range\",\"Cost\",\"Style\"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=[\"A\",\"B\"]")]

pub fn render(cfg: &RadarConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(4096 + p.n_ser * p.n_axes * 80);
    open_svg(&mut b, cfg, &p);
    draw_grid(&mut b, cfg, &p);

    for si in 0..p.n_ser {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let pts = series_points(cfg, &p, si);
        push_b(&mut b, b"<g data-series=\"");
        push_i(&mut b, si as i32);
        push_b(&mut b, b"\">");
        polygon_pts(&mut b, &pts);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        let op = cfg.fill_opacity as f64 / 255.0;
        push_b(&mut b, b"\" fill-opacity=\"");
        push_f2(&mut b, op);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\"/>");
        dots(&mut b, &pts, &hx, 3.5);
        push_b(&mut b, b"</g>");
    }

    draw_legend(&mut b, cfg, &p);
    finalize(b, cfg)
}
