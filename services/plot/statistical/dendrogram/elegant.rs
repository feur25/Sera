use super::common::{assign_positions_vertical, node_color, svg_header, tree_for};
use super::config::DendrogramConfig;
use crate::html::hover::{html_id, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"A1\",\"A2\",\"A3\",\"B1\",\"B2\",\"B3\",\"C1\",\"C2\",\"C3\"], matrix=[[1,1],[1.2,0.9],[0.9,1.1],[5,5],[5.2,4.8],[4.9,5.1],[1,5],[1.1,4.9],[0.9,5.2]], variant=\"elegant\"")]
pub fn render(cfg: &DendrogramConfig) -> String {
    let pad_l = 20.0f64; let pad_r = 40.0; let pad_t = 32.0; let pad_b = 48.0;
    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let Some((mut nodes, roots)) = tree_for(cfg) else { return String::new(); };
    assign_positions_vertical(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 200 + 4096);
    svg_header(&mut buf, cfg, hid, w / 2.0);

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let hx = hex6(node_color(cfg, &nodes[i]));
            push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, cfg.line_width);
            push_b(&mut buf, b"\" stroke-opacity=\"0.8\" d=\"M");
            push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[pi].y);
            push_b(&mut buf, b"C");
            push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b",");
            push_f2(&mut buf, (nodes[pi].y + nodes[i].y) / 2.0);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, nodes[i].x); push_b(&mut buf, b",");
            push_f2(&mut buf, (nodes[pi].y + nodes[i].y) / 2.0);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, nodes[i].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[i].y);
            push_b(&mut buf, b"\"/>");
        }
    }

    for i in 0..nodes.len() {
        let hx = hex6(node_color(cfg, &nodes[i]));
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 3.0 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, nodes[i].x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, nodes[i].y + 14.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
