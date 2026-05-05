use super::common::prepare;
use super::config::DumbbellConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_legend_item, Frame};
use crate::html::hover::slots_to_json;

pub fn render(cfg: &DumbbellConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 50, 56, 84, 30, p.n * 400 + 2048);
    f.open(cfg.title, true);
    f.y_grid(6, p.vmin, p.vmin + p.vrange, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    let pitch = f.pw as f64 / p.n as f64;
    let hx1 = hex6(p.c1);
    let hx2 = hex6(p.c2);
    for i in 0..p.n {
        let cx = f.pl + (i as f64 * pitch + pitch / 2.0) as i32;
        let y_at = |v: f64| f.pt + ((1.0 - (v - p.vmin) / p.vrange) * f.ph as f64) as i32;
        let y1 = y_at(p.start[i]);
        let y2 = y_at(p.end[i]);
        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y1);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y2);
        push_b(&mut f.buf, b"\" stroke=\"#9ca3af\" stroke-width=\"2\" stroke-linecap=\"round\"/>");
        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, (i as i32) * 2);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-y=\""); push_f2(&mut f.buf, p.start[i]);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &p.labels[i]);
        push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, y1);
        push_b(&mut f.buf, b"\" r=\"6\" fill=\"#"); f.buf.extend_from_slice(&hx1);
        push_b(&mut f.buf, b"\"/>");
        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, (i as i32) * 2 + 1);
        push_b(&mut f.buf, b"\" data-series=\"1\" data-y=\""); push_f2(&mut f.buf, p.end[i]);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &p.labels[i]);
        push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, y2);
        push_b(&mut f.buf, b"\" r=\"6\" fill=\"#"); f.buf.extend_from_slice(&hx2);
        push_b(&mut f.buf, b"\"/>");
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut f.buf, truncate(&p.labels[i], 12));
        push_b(&mut f.buf, b"</text>");
    }
    svg_legend_item(&mut f.buf, 0, cfg.series_names.0, p.c1, cfg.width - 140, f.pt + 4, 20);
    svg_legend_item(&mut f.buf, 1, cfg.series_names.1, p.c2, cfg.width - 140, f.pt + 22, 20);
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
