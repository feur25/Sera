use super::common::{
    angle_at, draw_grid, draw_legend, finalize, open_svg, polygon_pts, prepare, project,
};
use super::config::RadarConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("axes=[\"Speed\",\"Power\",\"Range\",\"Cost\",\"Style\"], series=[[80,65,70,40,75],[60,80,55,60,70]], series_names=[\"A\",\"B\"]")]

pub fn render(cfg: &RadarConfig) -> String {
    let mut p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };

    let mut cum = vec![0.0_f64; p.n_axes];
    for (_, vals) in cfg.series.iter() {
        for ai in 0..p.n_axes {
            cum[ai] += vals.get(ai).copied().unwrap_or(0.0).max(0.0);
        }
    }
    let stacked_max = cum.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
    p.global_max = stacked_max;

    let mut b = Vec::<u8>::with_capacity(4096 + p.n_ser * p.n_axes * 80);
    open_svg(&mut b, cfg, &p);
    draw_grid(&mut b, cfg, &p);

    let mut totals = vec![0.0_f64; p.n_axes];
    let mut layers: Vec<Vec<(f64, f64)>> = Vec::with_capacity(p.n_ser);
    for (_, vals) in cfg.series.iter() {
        for ai in 0..p.n_axes {
            totals[ai] += vals.get(ai).copied().unwrap_or(0.0).max(0.0);
        }
        let pts: Vec<(f64, f64)> = (0..p.n_axes)
            .map(|ai| {
                let frac = (totals[ai] / p.global_max).min(1.0);
                let a = angle_at(ai, p.n_axes);
                project(p.layout.plot_cx, p.layout.plot_cy, p.layout.r, frac, a)
            })
            .collect();
        layers.push(pts);
    }

    for si in (0..p.n_ser).rev() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut b, b"<g data-series=\"");
        push_i(&mut b, si as i32);
        push_b(&mut b, b"\" data-idx=\"");
        push_i(&mut b, si as i32);
        push_b(&mut b, b"\">");
        polygon_pts(&mut b, &layers[si]);
        push_b(&mut b, b" fill=\"#");
        b.extend_from_slice(&hx);
        let op = ((cfg.fill_opacity as f64 / 255.0) * 1.7).clamp(0.25, 0.95);
        push_b(&mut b, b"\" fill-opacity=\"");
        push_f2(&mut b, op);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(
            &mut b,
            b"\" stroke-width=\"1.5\" stroke-linejoin=\"round\"/>",
        );
        push_b(&mut b, b"</g>");
    }

    draw_legend(&mut b, cfg, &p);
    finalize(b, cfg)
}
