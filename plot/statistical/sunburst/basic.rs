use super::common::{prepare, open_svg, finalize, arc_path, slice_data_attrs, label_arc, ring_radii, color_hex};
use super::config::SunburstConfig;
use crate::plot::statistical::common::{push_b, push_i};

pub fn render(cfg: &SunburstConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 320 + 1536);
    open_svg(&mut b, cfg);
    let order = p.bfs_order.clone();
    for i in order {
        let (a1, a2) = p.ang[i];
        if a2 - a1 < 1e-4 { continue; }
        let (r1, r2) = ring_radii(&p, i);
        let hx = color_hex(&p, i);
        let opacity: &[u8] = match p.depth[i] { 0 => b"0.92", 1 => b"0.76", 2 => b"0.66", _ => b"0.56" };
        push_b(&mut b, b"<path");
        slice_data_attrs(&mut b, &p, i);
        push_b(&mut b, b" d=\"");
        arc_path(&mut b, p.layout.cx, p.layout.cy, r1, r2, a1, a2);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\""); b.extend_from_slice(opacity);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
        label_arc(&mut b, &p, i, true);
    }
    push_b(&mut b, b"<circle cx=\""); push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" cy=\""); push_i(&mut b, p.layout.cy);
    push_b(&mut b, b"\" r=\""); push_i(&mut b, p.layout.r_hole - 1);
    push_b(&mut b, b"\" fill=\"#fff\"/>");
    finalize(b, cfg)
}


