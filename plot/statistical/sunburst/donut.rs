use super::common::{prepare_with_hole, open_svg, finalize, arc_path, slice_data_attrs, label_arc, ring_radii, color_hex};
use super::config::SunburstConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml};

pub fn render(cfg: &SunburstConfig) -> String {
    let p = match prepare_with_hole(cfg, 0.20) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 320 + 2048);
    open_svg(&mut b, cfg);
    let order = p.bfs_order.clone();
    for i in order {
        let (a1, a2) = p.ang[i];
        if a2 - a1 < 1e-4 { continue; }
        let (r1, r2) = ring_radii(&p, i);
        let hx = color_hex(&p, i);
        let opacity: &[u8] = match p.depth[i] { 0 => b"0.92", 1 => b"0.78", 2 => b"0.68", _ => b"0.58" };
        push_b(&mut b, b"<path");
        slice_data_attrs(&mut b, &p, i);
        push_b(&mut b, b" d=\"");
        arc_path(&mut b, p.layout.cx, p.layout.cy, r1, r2, a1, a2);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\""); b.extend_from_slice(opacity);
        push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
        label_arc(&mut b, &p, i, true);
    }
    push_b(&mut b, b"<circle cx=\""); push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" cy=\""); push_i(&mut b, p.layout.cy);
    push_b(&mut b, b"\" r=\""); push_i(&mut b, p.layout.r_hole - 1);
    push_b(&mut b, b"\" fill=\"#ffffff\" stroke=\"#e5e7eb\" stroke-width=\"1\"/>");
    push_b(&mut b, b"<text x=\""); push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, p.layout.cy - 4);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">TOTAL</text>");
    push_b(&mut b, b"<text x=\""); push_i(&mut b, p.layout.cx);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, p.layout.cy + 12);
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1f2937\">");
    if p.grand_total >= 1_000_000.0 { push_f2(&mut b, p.grand_total / 1_000_000.0); push_b(&mut b, b"M"); }
    else if p.grand_total >= 1000.0 { push_f2(&mut b, p.grand_total / 1000.0); push_b(&mut b, b"k"); }
    else { push_f2(&mut b, p.grand_total); }
    push_b(&mut b, b"</text>");
    let _ = escape_xml;
    finalize(b, cfg)
}


