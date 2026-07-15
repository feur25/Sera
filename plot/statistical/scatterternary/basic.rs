use super::common::{axis_labels, finalize, gridlines, open_svg, prepare, tri_point, triangle_outline};
use super::config::ScatterTernaryConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

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
    let col = hex6(palette_color(cfg.palette, 0));
    for i in 0..p.n {
        let (px, py) = tri_point(&p.layout, cfg.a_values[i], cfg.b_values[i], cfg.c_values[i]);
        push_b(&mut b, b"<circle data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\" cx=\"");
        push_f2(&mut b, px);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, py);
        push_b(&mut b, b"\" r=\"");
        push_f2(&mut b, cfg.point_size);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&col);
        push_b(&mut b, b"\" fill-opacity=\"0.75\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
    }
    axis_labels(&mut b, &p.layout, cfg);
    finalize(b, cfg)
}
