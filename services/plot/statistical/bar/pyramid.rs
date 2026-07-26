use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title,
    truncate,
};

#[crate::chart_demo(
    "labels=[\"0-9\",\"10-19\",\"20-29\",\"30-39\",\"40-49\",\"50-59\",\"60-69\",\"70+\"], series=[[12,18,24,22,17,13,9,5],[11,17,25,23,18,14,10,6]], series_names=[\"Male\",\"Female\"], variant=\"population_pyramid\""
)]

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser < 2 {
        return String::new();
    }
    let left = &cfg.series[0];
    let right = &cfg.series[1];

    let max_val = (0..n_cats)
        .map(|ci| {
            left.1.get(ci).copied().unwrap_or(0.0).abs().max(right.1.get(ci).copied().unwrap_or(0.0).abs())
        })
        .fold(0.0_f64, f64::max)
        .max(1e-9);

    let pad_t = 42;
    let pad_b = 44;
    let center_gap = 96;
    let pad_side = 70;
    let w = cfg.width;
    let h = cfg.height;
    let half_w = (w / 2 - pad_side - center_gap / 2).max(20);
    let cx = w / 2;
    let row_h = (h - pad_t - pad_b) as f64 / n_cats as f64;
    let bar_h = (row_h * (1.0 - cfg.bar_gap)).max(2.0);

    let mut buf = Vec::<u8>::with_capacity(n_cats * 260 + 4096);
    svg_open_rescalable(&mut buf, w, h, pad_side, pad_t, w - 2 * pad_side, h - pad_t - pad_b);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, cx, 26);

    let color_l = palette_color(cfg.palette, 0);
    let color_r = palette_color(cfg.palette, 1);
    let hx_l = hex6(color_l);
    let hx_r = hex6(color_r);

    push_b(&mut buf, b"<line x1=\"");
    push_i(&mut buf, cx);
    push_b(&mut buf, b"\" y1=\"");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, cx);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, h - pad_b);
    push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" class=\"sp-ax-y\"/>");

    let n_xticks = 4;
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let val = max_val * frac;
        let dx = (half_w as f64 * frac) as i32;
        for (sign, x) in [(-1i32, cx - center_gap / 2 - dx), (1i32, cx + center_gap / 2 + dx)] {
            let _ = sign;
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, x);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, h - pad_b + 16);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
            push_f2(&mut buf, val);
            push_b(&mut buf, b"</text>");
        }
    }

    for ci in 0..n_cats {
        let cy_top = pad_t as f64 + row_h * ci as f64;
        let cy = cy_top + (row_h - bar_h) / 2.0;

        let vl = left.1.get(ci).copied().unwrap_or(0.0).abs();
        let vr = right.1.get(ci).copied().unwrap_or(0.0).abs();
        let wl = (vl / max_val * half_w as f64).max(0.0);
        let wr = (vr / max_val * half_w as f64).max(0.0);

        push_b(&mut buf, b"<rect data-idx=\"");
        push_i(&mut buf, (ci * 2) as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &left.0);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, vl);
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, (cx - center_gap / 2) as f64 - wl);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, wl);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, bar_h);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx_l);
        push_b(&mut buf, b"\"/>");

        push_b(&mut buf, b"<rect data-idx=\"");
        push_i(&mut buf, (ci * 2 + 1) as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &right.0);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, vr);
        push_b(&mut buf, b"\" x=\"");
        push_f2(&mut buf, (cx + center_gap / 2) as f64);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, wr);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, bar_h);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx_r);
        push_b(&mut buf, b"\"/>");

        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + bar_h / 2.0 + 3.5);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut buf, truncate(&cfg.category_labels[ci], 12));
        push_b(&mut buf, b"</text>");
    }

    let names = [left.0.as_str(), right.0.as_str()];
    if !cfg.legend_position.is_empty() && cfg.legend_position != "none" {
        let ly = pad_t - 18;
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, cx - 60);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"5\" fill=\"#");
        buf.extend_from_slice(&hx_l);
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cx - 50);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ly + 4);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, names[0]);
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, cx + 10);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"5\" fill=\"#");
        buf.extend_from_slice(&hx_r);
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cx + 20);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ly + 4);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, names[1]);
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
