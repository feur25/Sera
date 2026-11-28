use super::config::BubbleConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, push_b, push_f2, push_i, svg_open_rescalable, svg_title,
};

#[crate::chart_demo(
    "title=\"Survival by Embarkation Town and Class\", x_categories=[\"Cherbourg\",\"Cherbourg\",\"Cherbourg\",\"Cherbourg\",\"Cherbourg\",\"Cherbourg\",\"Queenstown\",\"Queenstown\",\"Queenstown\",\"Queenstown\",\"Queenstown\",\"Queenstown\",\"Southampton\",\"Southampton\",\"Southampton\",\"Southampton\",\"Southampton\",\"Southampton\"], y_categories=[\"First\",\"First\",\"Second\",\"Second\",\"Third\",\"Third\",\"First\",\"First\",\"Second\",\"Second\",\"Third\",\"Third\",\"First\",\"First\",\"Second\",\"Second\",\"Third\",\"Third\"], categories=[\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\",\"yes\",\"no\"], sizes=[42,26,7,8,15,41,1,1,2,2,24,55,74,53,76,88,67,286], variant=\"split\""
)]

pub fn render(cfg: &BubbleConfig) -> String {
    let n = cfg
        .x_categories
        .len()
        .min(cfg.y_categories.len())
        .min(cfg.categories.len())
        .min(cfg.sizes.len());
    if n == 0 {
        return String::new();
    }

    let mut x_order: Vec<String> = Vec::new();
    let mut y_order: Vec<String> = Vec::new();
    let mut split_order: Vec<String> = Vec::new();
    for i in 0..n {
        if !x_order.iter().any(|s| s == &cfg.x_categories[i]) {
            x_order.push(cfg.x_categories[i].clone());
        }
        if !y_order.iter().any(|s| s == &cfg.y_categories[i]) {
            y_order.push(cfg.y_categories[i].clone());
        }
        if !split_order.iter().any(|s| s == &cfg.categories[i]) {
            split_order.push(cfg.categories[i].clone());
        }
    }
    let nx = x_order.len().max(1);
    let ny = y_order.len().max(1);

    let title_h = if cfg.title.is_empty() { 0 } else { 30 };
    let pad_l = 90;
    let pad_t = title_h + 20;
    let pad_b = 44;
    let legend_w = 130;
    let pad_r = legend_w + 20;
    let pw = cfg.width - pad_l - pad_r;
    let ph = cfg.height - pad_t - pad_b;

    let max_size = cfg.sizes[..n].iter().cloned().fold(0.0_f64, f64::max).max(1e-9);
    let cell_r = (pw as f64 / nx as f64).min(ph as f64 / ny as f64) * 0.42;
    let scale = |v: f64| -> f64 { (v / max_size).sqrt() * cell_r };

    let mut buf = Vec::<u8>::with_capacity(n * 300 + 4096);
    svg_open_rescalable(&mut buf, cfg.width, cfg.height, pad_l, pad_t, pw, ph);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        svg_title(&mut buf, cfg.title, cfg.width / 2, 20);
    }

    let col_x = |ci: usize| -> f64 { pad_l as f64 + (ci as f64 + 0.5) / nx as f64 * pw as f64 };
    let row_y = |ri: usize| -> f64 { pad_t as f64 + (ri as f64 + 0.5) / ny as f64 * ph as f64 };

    for ci in 0..nx {
        let x = col_x(ci);
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y1=\"");
        push_i(&mut buf, pad_t);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y2=\"");
        push_i(&mut buf, pad_t + ph);
        push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" stroke-dasharray=\"4,3\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, pad_t + ph + 20);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, &x_order[ci]);
        push_b(&mut buf, b"</text>");
    }
    for ri in 0..ny {
        let y = row_y(ri);
        push_b(&mut buf, b"<line x1=\"");
        push_i(&mut buf, pad_l);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y);
        push_b(&mut buf, b"\" x2=\"");
        push_i(&mut buf, pad_l + pw);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y);
        push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" stroke-dasharray=\"4,3\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l - 10);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, y + 4.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, &y_order[ri]);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"<rect x=\"");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, pw);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, ph);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#1f2937\" stroke-width=\"1\"/>");

    let split_color = |si: usize| -> u32 {
        if si == 0 { 0x3b5bfd } else { 0xfd5c4b }
    };

    for i in 0..n {
        let ci = x_order.iter().position(|s| s == &cfg.x_categories[i]).unwrap_or(0);
        let ri = y_order.iter().position(|s| s == &cfg.y_categories[i]).unwrap_or(0);
        let si = split_order.iter().position(|s| s == &cfg.categories[i]).unwrap_or(0);
        let cx = col_x(ci);
        let cy = row_y(ri);
        let r = scale(cfg.sizes[i]).max(1.5);
        let color = split_color(si);
        let hx = hex6(color);

        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &cfg.categories[i]);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, cfg.sizes[i]);
        push_b(&mut buf, b"\" d=\"M");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b",");
        push_f2(&mut buf, cy - r);
        push_b(&mut buf, b" A");
        push_f2(&mut buf, r);
        push_b(&mut buf, b",");
        push_f2(&mut buf, r);
        if si == 0 {
            push_b(&mut buf, b" 0 0,0 ");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b",");
            push_f2(&mut buf, cy + r);
        } else {
            push_b(&mut buf, b" 0 0,1 ");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b",");
            push_f2(&mut buf, cy + r);
        }
        push_b(&mut buf, b" Z\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.72\" stroke=\"#111827\" stroke-width=\"");
        push_f2(&mut buf, cfg.stroke_width);
        push_b(&mut buf, b"\"/>");
    }

    let lx = cfg.width - legend_w + 6;
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, lx);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, pad_t - 4);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#374151\">Legend</text>");
    for (si, name) in split_order.iter().enumerate() {
        let ly = pad_t + 14 + si as i32 * 20;
        let color = split_color(si);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, lx + 8);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"7\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.72\" stroke=\"#111827\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx + 22);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ly + 4);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
    }

    let scale_n = 5;
    let scale_max = max_size;
    let scale_min = (scale_max / scale_n as f64).max(1.0);
    let sy_top = pad_t + 90;
    let sy_bot = pad_t + ph - 10;
    for k in 0..scale_n {
        let frac = k as f64 / (scale_n - 1).max(1) as f64;
        let v = scale_min + frac * (scale_max - scale_min);
        let sy = sy_bot as f64 - frac * (sy_bot - sy_top) as f64;
        let r = scale(v).max(2.0);
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, lx + 40);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#e040fb\" fill-opacity=\"0.75\" stroke=\"#111827\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx + 40);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, sy + 4.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#ffffff\">");
        push_i(&mut buf, v.round() as i32);
        push_b(&mut buf, b"</text>");
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
