use super::common::{compute_layout, draw_marker, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, lerp_rgb, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x=[1,2,3,4,5,6,7,8,9,10], y=[2,5,3,8,7,9,6,11,9,13], variant=\"rug\""
)]

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) {
        Some(l) => l,
        None => return String::new(),
    };
    let mut f = make_frame(cfg, layout.n, 20);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let color = if cfg.color_hex != 0 {
        cfg.color_hex
    } else {
        0x636EFA
    };
    let hx = hex6(color);
    let band_hx = hex6(lerp_rgb(color, 0xFFFFFF, 0.88));

    let band_w = 16.0;
    push_b(&mut f.buf, b"<rect x=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y=\"");
    push_f2(&mut f.buf, (f.pt + f.ph) as f64 - band_w);
    push_b(&mut f.buf, b"\" width=\"");
    push_i(&mut f.buf, f.pw);
    push_b(&mut f.buf, b"\" height=\"");
    push_f2(&mut f.buf, band_w);
    push_b(&mut f.buf, b"\" fill=\"#");
    f.buf.extend_from_slice(&band_hx);
    push_b(&mut f.buf, b"\"/>");
    push_b(&mut f.buf, b"<rect x=\"");
    push_i(&mut f.buf, f.pl);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, f.pt);
    push_b(&mut f.buf, b"\" width=\"");
    push_f2(&mut f.buf, band_w);
    push_b(&mut f.buf, b"\" height=\"");
    push_i(&mut f.buf, f.ph);
    push_b(&mut f.buf, b"\" fill=\"#");
    f.buf.extend_from_slice(&band_hx);
    push_b(&mut f.buf, b"\"/>");

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\"");
        push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.labels.len() {
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &cfg.labels[i]);
        }
        push_b(&mut f.buf, b"\">");
        draw_marker(
            &mut f.buf,
            cfg.symbol,
            cx,
            cy,
            cfg.point_size,
            &hx,
            &hx,
            cfg.stroke_width,
            0.82,
        );
        push_b(&mut f.buf, b"</g>");

        let base_y = f.pt + f.ph;
        push_b(&mut f.buf, b"<line x1=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\"");
        push_i(&mut f.buf, base_y);
        push_b(&mut f.buf, b"\" x2=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, base_y as f64 - band_w);
        push_b(&mut f.buf, b"\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-opacity=\"0.8\"/>");

        let base_x = f.pl;
        push_b(&mut f.buf, b"<line x1=\"");
        push_i(&mut f.buf, base_x);
        push_b(&mut f.buf, b"\" y1=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, base_x as f64 + band_w);
        push_b(&mut f.buf, b"\" y2=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-opacity=\"0.8\"/>");
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
