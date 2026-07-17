use super::common::{axis_labels, finalize, gridlines, open_svg, prepare, tri_point, triangle_outline};
use super::config::ScatterTernaryConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x=[0.7,0.2,0.1,0.4,0.33], y=[0.2,0.6,0.1,0.3,0.33], z=[0.1,0.2,0.8,0.3,0.34], color_values=[5,20,45,12,30]"
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

    let has_sizes = cfg.color_values.len() >= p.n;
    let smin = if has_sizes {
        cfg.color_values[..p.n].iter().cloned().fold(f64::INFINITY, f64::min)
    } else {
        0.0
    };
    let smax = if has_sizes {
        cfg.color_values[..p.n].iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    } else {
        1.0
    };
    let srange = (smax - smin).max(1e-9);
    let min_r = cfg.point_size;
    let max_r = cfg.point_size * 4.5;

    for i in 0..p.n {
        let (px, py) = tri_point(&p.layout, cfg.a_values[i], cfg.b_values[i], cfg.c_values[i]);
        let r = if has_sizes {
            let t = (cfg.color_values[i] - smin) / srange;
            min_r + t * (max_r - min_r)
        } else {
            cfg.point_size
        };
        let col = hex6(palette_color(cfg.palette, i));
        push_b(&mut b, b"<circle data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" cx=\"");
        push_f2(&mut b, px);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, py);
        push_b(&mut b, b"\" r=\"");
        push_f2(&mut b, r);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&col);
        push_b(&mut b, b"\" fill-opacity=\"0.68\" stroke=\"#");
        b.extend_from_slice(&col);
        push_b(&mut b, b"\" stroke-width=\"1\"/>");
    }
    axis_labels(&mut b, &p.layout, cfg);
    finalize(b, cfg)
}
