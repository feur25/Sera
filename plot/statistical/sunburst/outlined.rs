use super::common::{prepare, open_svg, finalize, arc_path, slice_data_attrs, label_arc, ring_radii, color_hex};
use super::config::SunburstConfig;
use crate::plot::statistical::common::{push_b, push_i};

pub const DEMO_KWARGS: &str = "labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\",\"B2\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\"], values=[0,40,30,20,20,15,15]";
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
        let sw: &[u8] = match p.depth[i] { 0 => b"3", 1 => b"2.4", 2 => b"1.8", _ => b"1.4" };
        push_b(&mut b, b"<path");
        slice_data_attrs(&mut b, &p, i);
        push_b(&mut b, b" d=\"");
        arc_path(&mut b, p.layout.cx, p.layout.cy, r1, r2, a1, a2);
        push_b(&mut b, b"\" fill=\"#ffffff\" fill-opacity=\"0.6\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\""); b.extend_from_slice(sw);
        push_b(&mut b, b"\" stroke-linejoin=\"round\"/>");
        label_arc(&mut b, &p, i, false);
    }
    push_b(&mut b, b"<circle cx=\""); push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" cy=\""); push_i(&mut b, p.layout.cy);
    push_b(&mut b, b"\" r=\""); push_i(&mut b, p.layout.r_hole - 1);
    push_b(&mut b, b"\" fill=\"#fff\"/>");
    finalize(b, cfg)
}


