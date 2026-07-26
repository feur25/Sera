use super::common::{compute_layout, cycle_symbol, draw_marker, make_frame, point_px};
use super::config::ScatterConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_legend_item,
};

#[crate::chart_demo(
    "x=[1,2,3,4,5,6,7,8,9,10,2,4,6,8,3,5,7,9], y=[2,5,3,8,7,9,6,11,9,13,6,9,7,11,4,8,6,12], categories=[\"Lunch\",\"Lunch\",\"Dinner\",\"Dinner\",\"Lunch\",\"Dinner\",\"Lunch\",\"Dinner\",\"Lunch\",\"Dinner\",\"Lunch\",\"Dinner\",\"Lunch\",\"Dinner\",\"Lunch\",\"Dinner\",\"Lunch\",\"Dinner\"], categories2=[\"Thur\",\"Fri\",\"Thur\",\"Sat\",\"Sun\",\"Thur\",\"Fri\",\"Sat\",\"Sun\",\"Thur\",\"Fri\",\"Sat\",\"Sun\",\"Thur\",\"Fri\",\"Sat\",\"Sun\",\"Thur\"], variant=\"dual_style\""
)]

pub fn render(cfg: &ScatterConfig) -> String {
    let layout = match compute_layout(cfg) {
        Some(l) => l,
        None => return String::new(),
    };

    let mut hue_order: Vec<String> = Vec::new();
    let mut style_order: Vec<String> = Vec::new();
    for i in 0..layout.n {
        if let Some(c) = cfg.categories.get(i) {
            if !c.is_empty() && !hue_order.iter().any(|x| x == c) {
                hue_order.push(c.clone());
            }
        }
        if let Some(c) = cfg.categories2.get(i) {
            if !c.is_empty() && !style_order.iter().any(|x| x == c) {
                style_order.push(c.clone());
            }
        }
    }
    let legend_w: i32 = 170;
    let mut f = make_frame(cfg, layout.n, legend_w);
    f.open(cfg.title, true);
    f.x_grid(6, layout.xmin2, layout.xmax2, cfg.gridlines);
    f.y_grid(5, layout.ymin2, layout.ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for i in 0..layout.n {
        let (cx, cy) = point_px(&layout, &f, cfg.x_values[i], cfg.y_values[i]);
        let hi = cfg
            .categories
            .get(i)
            .and_then(|c| hue_order.iter().position(|x| x == c))
            .unwrap_or(0);
        let si = cfg
            .categories2
            .get(i)
            .and_then(|c| style_order.iter().position(|x| x == c))
            .unwrap_or(0);
        let col = palette_color(cfg.palette, hi);
        let hx = hex6(col);
        let sym = cycle_symbol(si);

        push_b(&mut f.buf, b"<g data-idx=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-x=\"");
        push_f2(&mut f.buf, cfg.x_values[i]);
        push_b(&mut f.buf, b"\" data-y=\"");
        push_f2(&mut f.buf, cfg.y_values[i]);
        push_b(&mut f.buf, b"\">");
        draw_marker(
            &mut f.buf,
            sym,
            cx,
            cy,
            cfg.point_size,
            &hx,
            &hx,
            cfg.stroke_width.max(1.2),
            0.78,
        );
        push_b(&mut f.buf, b"</g>");
    }

    let lx = cfg.width - legend_w + 10;
    let mut ly = f.pt + 4;
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, lx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, ly);
    push_b(
        &mut f.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#94a3b8\">HUE</text>",
    );
    ly += 18;
    for (li, name) in hue_order.iter().enumerate() {
        let col = palette_color(cfg.palette, li);
        svg_legend_item(&mut f.buf, li as i32, name, col, lx, ly, 16);
        ly += 20;
    }
    ly += 10;
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, lx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, ly);
    push_b(
        &mut f.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#94a3b8\">STYLE</text>",
    );
    ly += 18;
    for (si, name) in style_order.iter().enumerate() {
        let sym = cycle_symbol(si);
        draw_marker(&mut f.buf, sym, lx + 8, ly - 4, 6.0, b"9ca3af", b"9ca3af", 1.2, 0.85);
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, lx + 22);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ly);
        push_b(
            &mut f.buf,
            b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#cbd5e1\">",
        );
        escape_xml(&mut f.buf, name);
        push_b(&mut f.buf, b"</text>");
        ly += 20;
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
