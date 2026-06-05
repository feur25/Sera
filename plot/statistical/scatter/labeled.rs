use super::common::{compute_layout, draw_marker, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};

#[crate::chart_demo(
    "x=[1,2,3,4,5,6], y=[2,5,3,8,7,9], labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\"]"
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

    let use_groups = !cfg.categories.is_empty();
    let mut cat_order: Vec<String> = Vec::new();
    if use_groups {
        for i in 0..layout.n {
            let c = if i < cfg.categories.len() {
                &cfg.categories[i]
            } else {
                ""
            };
            if !c.is_empty() && !cat_order.iter().any(|x| x == c) {
                cat_order.push(c.to_string());
            }
        }
    }
    let fixed_hex = if cfg.color_hex != 0 {
        cfg.color_hex
    } else {
        0x6366F1
    };

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let col = if use_groups {
            let ci = cat_order
                .iter()
                .position(|c| i < cfg.categories.len() && c == &cfg.categories[i])
                .unwrap_or(0);
            palette_color(cfg.palette, ci)
        } else {
            fixed_hex
        };
        let hx = hex6(col);

        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
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
            0.7,
        );
        push_b(&mut f.buf, b"</g>");
    }

    for i in 0..layout.n {
        let label = if i < cfg.labels.len() {
            cfg.labels[i].as_str()
        } else {
            continue;
        };
        if label.is_empty() {
            continue;
        }
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let ty = cy - (cfg.point_size as i32) - 5;
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ty);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"#e2e8f0\" stroke=\"#0f172a\" stroke-width=\"3\" paint-order=\"stroke\">");
        escape_xml(&mut f.buf, truncate(label, 20));
        push_b(&mut f.buf, b"</text>");
    }

    let _ = push_f2;
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
