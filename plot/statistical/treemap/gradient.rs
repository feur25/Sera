use super::common::{prepare, open_svg, finalize, leaf_color, label_inside, tile_data_attrs, rect_attrs};
use super::config::TreemapConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub fn render(cfg: &TreemapConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.leaf_indices.len() * 320 + 2048);
    open_svg(&mut b, cfg);
    push_b(&mut b, b"<defs>");
    for ri in 0..p.leaf_indices.len() {
        let hx = hex6(leaf_color(&p, ri));
        push_b(&mut b, b"<linearGradient id=\"tmg-"); push_i(&mut b, ri as i32);
        push_b(&mut b, b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">");
        push_b(&mut b, b"<stop offset=\"0%\" stop-color=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\" stop-opacity=\"1\"/>");
        push_b(&mut b, b"<stop offset=\"100%\" stop-color=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\" stop-opacity=\"0.55\"/>");
        push_b(&mut b, b"</linearGradient>");
    }
    push_b(&mut b, b"</defs>");
    for ri in 0..p.leaf_indices.len() {
        let r = p.rects[ri];
        if r.w < 0.5 || r.h < 0.5 { continue; }
        push_b(&mut b, b"<rect");
        tile_data_attrs(&mut b, &p, ri);
        rect_attrs(&mut b, r);
        push_b(&mut b, b" rx=\"3\" fill=\"url(#tmg-"); push_i(&mut b, ri as i32);
        push_b(&mut b, b")\" stroke=\"rgba(255,255,255,0.5)\" stroke-width=\"1\"/>");
        label_inside(&mut b, &p, ri, b"#fff");
    }
    finalize(b, cfg)
}


