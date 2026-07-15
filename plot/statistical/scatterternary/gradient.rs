use super::common::{axis_labels, finalize, gridlines, open_svg, prepare, tri_point, triangle_outline};
use super::config::ScatterTernaryConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
use crate::plot::statistical::heatmap::common::colorscale_color;

#[crate::chart_demo(
    "x=[0.7,0.2,0.1,0.4,0.33], y=[0.2,0.6,0.1,0.3,0.33], z=[0.1,0.2,0.8,0.3,0.34]"
)]

pub fn render(cfg: &ScatterTernaryConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 96 + 4096);
    open_svg(&mut b, cfg);
    gridlines(&mut b, &p.layout);
    triangle_outline(&mut b, &p.layout);

    let cv: Vec<f64> = if cfg.color_values.len() >= p.n {
        cfg.color_values[..p.n].to_vec()
    } else {
        (0..p.n).map(|i| cfg.a_values[i]).collect()
    };
    let cmin = cv.iter().cloned().fold(f64::INFINITY, f64::min);
    let cmax = cv.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let cr = (cmax - cmin).max(1e-9);
    let scale = if cfg.colorscale.is_empty() { "viridis" } else { cfg.colorscale };

    for i in 0..p.n {
        let (px, py) = tri_point(&p.layout, cfg.a_values[i], cfg.b_values[i], cfg.c_values[i]);
        let t = (cv[i] - cmin) / cr;
        let col = hex6(colorscale_color(scale, t));
        push_b(&mut b, b"<circle data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\"");
        push_f2(&mut b, cv[i]);
        push_b(&mut b, b"\" cx=\"");
        push_f2(&mut b, px);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, py);
        push_b(&mut b, b"\" r=\"");
        push_f2(&mut b, cfg.point_size);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&col);
        push_b(&mut b, b"\" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
    }
    axis_labels(&mut b, &p.layout, cfg);
    finalize(b, cfg)
}
