use super::config::ScatterConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title,
};

#[crate::chart_demo(
    "x=[1,2,3,4,5,6,7,8,2,4,6,8,9,3,5,7,9,1], y=[2,5,3,8,7,9,6,11,6,9,7,11,9,4,8,6,12,3], categories=[\"Lunch\",\"Lunch\",\"Lunch\",\"Lunch\",\"Lunch\",\"Lunch\",\"Lunch\",\"Lunch\",\"Lunch\",\"Dinner\",\"Dinner\",\"Dinner\",\"Dinner\",\"Dinner\",\"Dinner\",\"Dinner\",\"Dinner\",\"Dinner\"], variant=\"facet\""
)]

pub fn render(cfg: &ScatterConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return String::new();
    }

    let mut order: Vec<String> = Vec::new();
    for i in 0..n {
        if let Some(c) = cfg.categories.get(i) {
            if !c.is_empty() && !order.iter().any(|x| x == c) {
                order.push(c.clone());
            }
        }
    }
    if order.is_empty() {
        order.push(String::new());
    }
    let nf = order.len();
    let cols = (nf as f64).sqrt().ceil().max(1.0) as usize;
    let rows = (nf + cols - 1) / cols;

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for i in 0..n {
        xmin = xmin.min(cfg.x_values[i]);
        xmax = xmax.max(cfg.x_values[i]);
        ymin = ymin.min(cfg.y_values[i]);
        ymax = ymax.max(cfg.y_values[i]);
    }
    let xr = (xmax - xmin).max(1e-9) * 1.12;
    let yr = (ymax - ymin).max(1e-9) * 1.16;
    let xmid = (xmax + xmin) / 2.0;
    let ymid = (ymax + ymin) / 2.0;
    let xmin2 = xmid - xr / 2.0;
    let xmax2 = xmid + xr / 2.0;
    let ymin2 = ymid - yr / 2.0;
    let ymax2 = ymid + yr / 2.0;

    let title_h = if cfg.title.is_empty() { 0 } else { 30 };
    let pad_out = 16;
    let gap = 14;
    let grid_w = cfg.width - pad_out * 2;
    let grid_h = cfg.height - title_h - pad_out * 2;
    let panel_w = (grid_w - gap * (cols as i32 - 1)) / cols as i32;
    let panel_h = (grid_h - gap * (rows as i32 - 1)) / rows as i32;

    let mut buf = Vec::<u8>::with_capacity(n * 60 + nf * 400 + 4096);
    svg_open_rescalable(&mut buf, cfg.width, cfg.height, pad_out, title_h + pad_out, grid_w, grid_h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        svg_title(&mut buf, cfg.title, cfg.width / 2, 20);
    }

    for (fi, cat) in order.iter().enumerate() {
        let col_i = (fi % cols) as i32;
        let row_i = (fi / cols) as i32;
        let px = pad_out + col_i * (panel_w + gap);
        let py = title_h + pad_out + row_i * (panel_h + gap);
        let color = palette_color(cfg.palette, fi);
        let hx = hex6(color);

        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, px);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, py);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, panel_w);
        push_b(&mut buf, b"\" height=\"");
        push_i(&mut buf, panel_h);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

        if !cat.is_empty() {
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, px + panel_w / 2);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, py - 6);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#374151\">");
            escape_xml(&mut buf, cat);
            push_b(&mut buf, b"</text>");
        }

        for i in 0..n {
            let same = cfg.categories.get(i).map(|c| c == cat).unwrap_or(order.len() == 1);
            if !same {
                continue;
            }
            let cx = px as f64 + ((cfg.x_values[i] - xmin2) / (xmax2 - xmin2)) * panel_w as f64;
            let cy = py as f64 + panel_h as f64
                - ((cfg.y_values[i] - ymin2) / (ymax2 - ymin2)) * panel_h as f64;
            push_b(&mut buf, b"<circle data-idx=\"");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" data-x=\"");
            push_f2(&mut buf, cfg.x_values[i]);
            push_b(&mut buf, b"\" data-y=\"");
            push_f2(&mut buf, cfg.y_values[i]);
            push_b(&mut buf, b"\" cx=\"");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, cy);
            push_b(&mut buf, b"\" r=\"");
            push_f2(&mut buf, cfg.point_size.max(3.0));
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.8\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, cfg.stroke_width);
            push_b(&mut buf, b"\"/>");
        }
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = crate::html::hover::slots_to_json(cfg.hover);
        &slots_json
    };
    crate::html::hover::build_chart_html(cfg.title, &svg, json)
}
