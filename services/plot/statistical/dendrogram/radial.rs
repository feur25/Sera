use super::common::{assign_positions_radial, node_color, svg_header, tree_for, write_radial_link};
use super::config::DendrogramConfig;
use crate::html::hover::{html_id, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn render_impl(cfg: &DendrogramConfig, smooth: bool) -> String {
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0 + 6.0;
    let r_max = (w.min(h) / 2.0 - 58.0).max(30.0);

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    assign_positions_radial(&mut nodes, &roots, cx, cy, r_max);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 200 + 4096);
    svg_header(&mut buf, cfg, hid, cx);

    for ring in 1..=4 {
        let rr = r_max * ring as f64 / 4.0;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, rr);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"2,3\"/>");
    }

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(node_color(cfg, &nodes[i]));
            write_radial_link(
                &mut buf, cx, cy,
                nodes[pi].x, nodes[pi].y, nodes[i].x, nodes[i].y,
                &hx, cfg.line_width, 0.85, smooth,
            );
        }
    }

    for i in 0..nodes.len() {
        let hx = hex6(node_color(cfg, &nodes[i]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.2 } else { 2.4 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            let dx = nodes[i].x - cx;
            let dy = nodes[i].y - cy;
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            let tx = nodes[i].x + dx / len * 10.0;
            let ty = nodes[i].y + dy / len * 10.0;
            let anchor: &[u8] = if dx > 1.0 { b"start" } else if dx < -1.0 { b"end" } else { b"middle" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty + 3.5);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

#[crate::chart_demo("labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\",\"D1\",\"D2\",\"D3\"], matrix=[[1,1],[1.2,0.9],[0.9,1.1],[5,5],[5.2,4.8],[4.9,5.1],[1,5],[1.1,4.9],[0.9,5.2],[5,1],[5.2,0.9],[4.9,1.1]], variant=\"radial\"")]
pub fn render(cfg: &DendrogramConfig) -> String {
    render_impl(cfg, true)
}
