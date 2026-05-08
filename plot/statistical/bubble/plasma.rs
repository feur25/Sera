use super::common::{compute_layout, make_frame, point_px, radius};
use super::config::BubbleConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};

fn plasma_color(i: usize) -> (u32, u32, u32) {
    const PALETTE: [(u32, u32, u32); 6] = [
        (0x1E40AF, 0x3B82F6, 0x93C5FD),
        (0x7C2D12, 0xF97316, 0xFED7AA),
        (0x4C1D95, 0xA855F7, 0xE9D5FF),
        (0x064E3B, 0x10B981, 0xA7F3D0),
        (0x9F1239, 0xF43F5E, 0xFDA4AF),
        (0x0C4A6E, 0x06B6D4, 0xA5F3FC),
    ];
    PALETTE[i % PALETTE.len()]
}

pub fn render(cfg: &BubbleConfig) -> String {
    let layout = match compute_layout(cfg) { Some(l) => l, None => return String::new() };
    let n = layout.n;
    let mut f = make_frame(cfg, n, 20);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    push_b(&mut f.buf, b"<defs>");
    push_b(&mut f.buf, b"<filter id=\"plsf\" x=\"-50%\" y=\"-50%\" width=\"200%\" height=\"200%\">");
    push_b(&mut f.buf, b"<feGaussianBlur stdDeviation=\"5\" result=\"b\"/>");
    push_b(&mut f.buf, b"<feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut f.buf, b"</filter>");
    for i in 0..n {
        let (deep, mid, bright) = plasma_color(i);
        push_b(&mut f.buf, b"<radialGradient id=\"plrg");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" cx=\"35%\" cy=\"30%\" r=\"70%\" fx=\"35%\" fy=\"30%\">");
        push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(bright));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.95\"/>");
        push_b(&mut f.buf, b"<stop offset=\"0.45\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(mid));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.75\"/>");
        push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(deep));
        push_b(&mut f.buf, b"\" stop-opacity=\"0.35\"/>");
        push_b(&mut f.buf, b"</radialGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    for &i in &layout.indices {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let r = radius(cfg, &layout, i);
        let (_, mid, _) = plasma_color(i);
        let hx = hex6(mid);

        push_b(&mut f.buf, b"<g data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");

        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r * 1.15);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.15\" filter=\"url(#plsf)\"/>");

        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"url(#plrg"); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, cfg.stroke_width * 0.5);
        push_b(&mut f.buf, b"\" stroke-opacity=\"0.4\"/>");

        push_b(&mut f.buf, b"</g>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
