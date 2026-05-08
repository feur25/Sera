use super::common::{compute_layout, make_frame, point_px, radius};
use super::config::BubbleConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn iridescent_colors(i: usize) -> (u32, u32, u32, u32) {
    const SETS: [(u32, u32, u32, u32); 7] = [
        (0x0D2A4A, 0x3B82F6, 0x93C5FD, 0xDBEAFE),
        (0x1A0A2E, 0x7C3AED, 0xA78BFA, 0xEDE9FE),
        (0x052E1C, 0x059669, 0x34D399, 0xD1FAE5),
        (0x2D1B00, 0xD97706, 0xFBBF24, 0xFEF3C7),
        (0x2D0A1A, 0xDB2777, 0xF472B6, 0xFCE7F3),
        (0x001A2D, 0x0284C7, 0x38BDF8, 0xE0F2FE),
        (0x1A0A2E, 0x9333EA, 0xC084FC, 0xF3E8FF),
    ];
    SETS[i % SETS.len()]
}

pub fn render(cfg: &BubbleConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let n = layout.n;
    let mut f = make_frame(cfg, n, 20);
    f.open(cfg.title, true);

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"irdsf\" x=\"-50%\" y=\"-50%\" width=\"200%\" height=\"200%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"6\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    for i in 0..n {
        let (deep, mid, bright, pale) = iridescent_colors(i);
        push_b(&mut f.buf, b"<radialGradient id=\"irdrg");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" cx=\"32%\" cy=\"28%\" r=\"72%\" fx=\"32%\" fy=\"28%\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(pale)); push_b(&mut f.buf, b"\" stop-opacity=\"0.98\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.3\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(bright)); push_b(&mut f.buf, b"\" stop-opacity=\"0.82\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.65\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(mid)); push_b(&mut f.buf, b"\" stop-opacity=\"0.55\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#"); f.buf.extend_from_slice(&hex6(deep)); push_b(&mut f.buf, b"\" stop-opacity=\"0.22\"/>");
        push_b(&mut f.buf, b"</radialGradient>");
        push_b(&mut f.buf, b"<radialGradient id=\"irdsh"); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" cx=\"38%\" cy=\"30%\" r=\"35%\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#ffffff\" stop-opacity=\"0.7\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#ffffff\" stop-opacity=\"0\"/>");
        push_b(&mut f.buf, b"</radialGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for &i in &layout.indices {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let r = radius(cfg, &layout, i);
        let (_, mid, _, _) = iridescent_colors(i);
        let hx = hex6(mid);

        push_b(&mut f.buf, b"<g data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r * 1.2);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.12\" filter=\"url(#irdsf)\"/>");
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"url(#irdrg"); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"0.8\" stroke-opacity=\"0.45\"/>");
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"url(#irdsh"); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\"/>");
        push_b(&mut f.buf, b"</g>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
