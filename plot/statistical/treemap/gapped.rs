use super::common::{
    fill_hex, finalize, label_inside, leaf_color, open_svg, prepare_with_pad, rect_attrs,
    tile_data_attrs, Rect,
};
use super::config::TreemapConfig;
use crate::plot::statistical::common::push_b;

#[crate::chart_demo(
    "labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\",\"G\"], values=[40,25,20,10,5,8,12]"
)]

pub fn render(cfg: &TreemapConfig) -> String {
    let mut p = match prepare_with_pad(cfg, 6.0) {
        Some(v) => v,
        None => return String::new(),
    };
    let inset = 4.0_f64;
    for ri in 0..p.rects.len() {
        let r = p.rects[ri];
        if r.w > inset * 2.0 + 2.0 && r.h > inset * 2.0 + 2.0 {
            p.rects[ri] = Rect {
                x: r.x + inset,
                y: r.y + inset,
                w: r.w - inset * 2.0,
                h: r.h - inset * 2.0,
            };
        }
    }
    let p = p;
    let mut b = Vec::<u8>::with_capacity(p.leaf_indices.len() * 200 + 2048);
    open_svg(&mut b, cfg);
    for ri in 0..p.leaf_indices.len() {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 {
            continue;
        }
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"6");
        fill_hex(&mut b, leaf_color(&p, ri));
        push_b(&mut b, b"\"/>");
        label_inside(&mut b, &p, ri, b"#fff");
    }
    finalize(b, cfg)
}
