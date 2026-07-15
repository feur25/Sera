use super::common::{assign_positions_horizontal, assign_positions_radial, assign_positions_vertical, build_tree};
use super::config::DendrogramConfig;
use super::variant::DendrogramVariant;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"C\",\"A1\",\"A2\",\"B1\",\"B2\",\"C1\"], parents=[\"\",\"Root\",\"Root\",\"Root\",\"A\",\"A\",\"B\",\"B\",\"C\"]")]
pub fn render(cfg: &DendrogramConfig) -> String {
    render_impl(cfg, false, false)
}

pub fn render_horizontal(cfg: &DendrogramConfig) -> String { render_impl(cfg, true, false) }
pub fn render_radial(cfg: &DendrogramConfig)    -> String { render_radial_impl(cfg) }
pub fn render_compact(cfg: &DendrogramConfig)   -> String { render_impl(cfg, false, true) }
pub fn render_elegant(cfg: &DendrogramConfig)   -> String { render_impl_elegant(cfg) }

fn render_impl(cfg: &DendrogramConfig, horizontal: bool, compact: bool) -> String {
    let pad = if compact { (16.0f64, 16.0, 28.0, 36.0) } else { (20.0f64, 40.0, 32.0, 48.0) };
    let (pad_l, pad_r, pad_t, pad_b) = pad;
    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let (mut nodes, roots) = build_tree(cfg.labels, cfg.parents);
    if nodes.is_empty() { return String::new(); }

    if horizontal {
        assign_positions_horizontal(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);
    } else {
        assign_positions_vertical(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);
    }

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 180 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, w / 2.0);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let color = palette_color(cfg.palette, nodes[i].color_idx);
            let hx = hex6(color);

            if horizontal {
                let mid_x = (nodes[pi].x + nodes[i].x) / 2.0;
                push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke-width=\"");
                push_f2(&mut buf, cfg.line_width);
                push_b(&mut buf, b"\" stroke-opacity=\"0.7\" d=\"M");
                push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[pi].y);
                push_b(&mut buf, b"H"); push_f2(&mut buf, mid_x);
                push_b(&mut buf, b"V"); push_f2(&mut buf, nodes[i].y);
                push_b(&mut buf, b"H"); push_f2(&mut buf, nodes[i].x);
                push_b(&mut buf, b"\"/>");
            } else {
                let mid_y = (nodes[pi].y + nodes[i].y) / 2.0;
                push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke-width=\"");
                push_f2(&mut buf, cfg.line_width);
                push_b(&mut buf, b"\" stroke-opacity=\"0.7\" d=\"M");
                push_f2(&mut buf, nodes[pi].x); push_b(&mut buf, b","); push_f2(&mut buf, nodes[pi].y);
                push_b(&mut buf, b"V"); push_f2(&mut buf, mid_y);
                push_b(&mut buf, b"H"); push_f2(&mut buf, nodes[i].x);
                push_b(&mut buf, b"V"); push_f2(&mut buf, nodes[i].y);
                push_b(&mut buf, b"\"/>");
            }
        }
    }

    let font_size = if compact { 8.5f64 } else { 10.0 };
    for i in 0..nodes.len() {
        let color = palette_color(cfg.palette, nodes[i].color_idx);
        let hx = hex6(color);

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 4.5 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels {
            let (tx, ty, anchor) = if horizontal {
                if nodes[i].children.is_empty() {
                    (nodes[i].x + 7.0, nodes[i].y + 3.5, b"start" as &[u8])
                } else {
                    (nodes[i].x - 6.0, nodes[i].y + 3.5, b"end" as &[u8])
                }
            } else {
                if nodes[i].children.is_empty() {
                    (nodes[i].x, nodes[i].y + 14.0, b"middle" as &[u8])
                } else {
                    (nodes[i].x + 6.0, nodes[i].y - 6.0, b"start" as &[u8])
                }
            };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"");
            push_f2(&mut buf, font_size);
            push_b(&mut buf, b"\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn render_radial_impl(cfg: &DendrogramConfig) -> String {
    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let r_max = (w.min(h) / 2.0 - 52.0).max(40.0);

    let (mut nodes, roots) = build_tree(cfg.labels, cfg.parents);
    if nodes.is_empty() { return String::new(); }

    assign_positions_radial(&mut nodes, &roots, cx, cy, r_max);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 180 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let color = palette_color(cfg.palette, nodes[i].color_idx);
            let hx = hex6(color);
            push_b(&mut buf, b"<line x1=\"");
            push_f2(&mut buf, nodes[pi].x);
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, nodes[pi].y);
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, nodes[i].x);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, nodes[i].y);
            push_b(&mut buf, b"\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, cfg.line_width);
            push_b(&mut buf, b"\" stroke-opacity=\"0.7\"/>");
        }
    }

    for i in 0..nodes.len() {
        let color = palette_color(cfg.palette, nodes[i].color_idx);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 4.5 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels && nodes[i].children.is_empty() {
            let dx = nodes[i].x - cx;
            let dy = nodes[i].y - cy;
            let len = (dx * dx + dy * dy).sqrt().max(1.0);
            let tx = nodes[i].x + dx / len * 10.0;
            let ty = nodes[i].y + dy / len * 10.0;
            let anchor: &[u8] = if dx > 0.0 { b"start" } else { b"end" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty + 3.5);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn render_impl_elegant(cfg: &DendrogramConfig) -> String {
    let pad_l = 20.0f64; let pad_r = 40.0; let pad_t = 32.0; let pad_b = 48.0;
    let w = cfg.width as f64;
    let h = cfg.height as f64;

    let (mut nodes, roots) = build_tree(cfg.labels, cfg.parents);
    if nodes.is_empty() { return String::new(); }

    assign_positions_vertical(&mut nodes, &roots, w, h, pad_l, pad_r, pad_t, pad_b);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(nodes.len() * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, w / 2.0);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for i in 0..nodes.len() {
        if let Some(pi) = nodes[i].parent {
            let color = palette_color(cfg.palette, nodes[i].color_idx);
            let hx = hex6(color);
            push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, cfg.line_width);
            push_b(&mut buf, b"\" stroke-opacity=\"0.75\" d=\"M");
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
        let color = palette_color(cfg.palette, nodes[i].color_idx);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, nodes[i].x);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, nodes[i].y);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, if nodes[i].children.is_empty() { 3.5 } else { 4.5 });
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels {
            let (tx, ty, anchor): (f64, f64, &[u8]) = if nodes[i].children.is_empty() {
                (nodes[i].x, nodes[i].y + 14.0, b"middle")
            } else {
                (nodes[i].x + 6.0, nodes[i].y - 6.0, b"start")
            };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            escape_xml(&mut buf, &nodes[i].label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
