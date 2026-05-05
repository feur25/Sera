use super::common::{prepare, open_svg, finalize, arc_path, slice_data_attrs, label_arc, ring_radii, hsl_to_rgb};
use super::config::SunburstConfig;
use crate::plot::statistical::common::{push_b, push_i, hex6};

pub fn render(cfg: &SunburstConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 320 + 1536);
    open_svg(&mut b, cfg);
    let two_pi = 2.0 * std::f64::consts::PI;
    let order = p.bfs_order.clone();
    for i in order {
        let (a1, a2) = p.ang[i];
        if a2 - a1 < 1e-4 { continue; }
        let (r1, r2) = ring_radii(&p, i);
        let mid = a1 + (a2 - a1) / 2.0;
        let h = ((mid + std::f64::consts::PI / 2.0).rem_euclid(two_pi) / two_pi) * 360.0;
        let l = 0.55 - (p.depth[i] as f64) * 0.06;
        let s = 0.75 - (p.depth[i] as f64) * 0.05;
        let c = hsl_to_rgb(h, s.max(0.35), l.max(0.30));
        let hx = hex6(c);
        push_b(&mut b, b"<path");
        slice_data_attrs(&mut b, &p, i);
        push_b(&mut b, b" d=\"");
        arc_path(&mut b, p.layout.cx, p.layout.cy, r1, r2, a1, a2);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"0.95\" stroke=\"#fff\" stroke-width=\"0.6\"/>");
        label_arc(&mut b, &p, i, true);
    }
    push_b(&mut b, b"<circle cx=\""); push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" cy=\""); push_i(&mut b, p.layout.cy);
    push_b(&mut b, b"\" r=\""); push_i(&mut b, p.layout.r_hole - 1);
    push_b(&mut b, b"\" fill=\"#fff\"/>");
    finalize(b, cfg)
}
