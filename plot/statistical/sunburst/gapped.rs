use super::common::{
    arc_path, color_hex, finalize, label_arc, open_svg, prepare, ring_radii, slice_data_attrs,
};
use super::config::SunburstConfig;
use crate::plot::statistical::common::{push_b, push_i};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"A1\",\"A2\",\"B1\",\"B2\"], parents=[\"\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\"], values=[0,40,30,20,20,15,15]")]

pub fn render(cfg: &SunburstConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 320 + 1536);
    open_svg(&mut b, cfg);
    let gap_rad = 0.018_f64;
    let order = p.bfs_order.clone();
    for i in order {
        let (a1, a2) = p.ang[i];
        let raw = a2 - a1;
        if raw < gap_rad * 2.5 {
            continue;
        }
        let aa1 = a1 + gap_rad;
        let aa2 = a2 - gap_rad;
        let (r1, r2) = ring_radii(&p, i);
        let r1g = r1 + 1;
        let r2g = (r2 - 1).max(r1g + 1);
        let hx = color_hex(&p, i);
        let opacity: &[u8] = match p.depth[i] {
            0 => b"0.95",
            1 => b"0.82",
            2 => b"0.72",
            _ => b"0.62",
        };
        push_b(&mut b, b"<path");
        slice_data_attrs(&mut b, &p, i);
        push_b(&mut b, b" d=\"");
        arc_path(&mut b, p.layout.cx, p.layout.cy, r1g, r2g, aa1, aa2);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"");
        b.extend_from_slice(opacity);
        push_b(&mut b, b"\"/>");
        label_arc(&mut b, &p, i, true);
    }
    push_b(&mut b, b"<circle cx=\"");
    push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" cy=\"");
    push_i(&mut b, p.layout.cy);
    push_b(&mut b, b"\" r=\"");
    push_i(&mut b, p.layout.r_hole - 1);
    push_b(&mut b, b"\" fill=\"none\"/>");
    finalize(b, cfg)
}
