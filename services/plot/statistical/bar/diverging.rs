use super::config::BarConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, push_b, push_f2, push_i, svg_open_rescalable, svg_title,
    svg_vgrid_vis, truncate,
};

#[crate::chart_demo(
    "title=\"Extremes regionaux\", y_label=\"Robustesse Score median\", labels=[\"Consecrated Snowfield\",\"Gravesite Plain\",\"Scadu Altus\",\"Jagged Peak\",\"Liurnia of the Lakes\",\"Limgrave\",\"Charo's Hidden Grave\",\"Weeping Peninsula\"], values=[0.55,0.54,0.53,0.41,-0.35,-0.38,-0.41,-0.47], sort_order=\"desc\", variant=\"diverging\""
)]

pub fn render(cfg: &BarConfig) -> String {
    let (labels, values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = labels.len().min(values.len());
    if n == 0 {
        return String::new();
    }

    let pad_l = 168;
    let pad_r = 28;
    let pad_t = if cfg.title.is_empty() { 12 } else { 42 };
    let pad_b = 40;
    let w = cfg.width;
    let h = cfg.height;
    let pw = (w - pad_l - pad_r).max(10);
    let ph = (h - pad_t - pad_b).max(10);

    let vmax = values[..n].iter().cloned().fold(0.0_f64, f64::max).max(0.0);
    let vmin = values[..n].iter().cloned().fold(0.0_f64, f64::min).min(0.0);
    let pad_v = (vmax - vmin).abs().max(1e-9) * 0.1;
    let x_max = vmax + pad_v;
    let x_min = vmin - pad_v;
    let range = (x_max - x_min).max(1e-9);

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 4096);
    svg_open_rescalable(&mut buf, w, h, pad_l, pad_t, pw, ph);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 26);

    let n_xticks = 5;
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = pad_l + (pw as f64 * frac) as i32;
        svg_vgrid_vis(&mut buf, x, pad_t, pad_t + ph, cfg.gridlines);
    }
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let val = x_min + range * frac;
        let x = pad_l + (pw as f64 * frac) as i32;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, pad_t + ph + 16);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        push_f2(&mut buf, val);
        push_b(&mut buf, b"</text>");
    }

    let zero_x = pad_l + ((0.0 - x_min) / range * pw as f64) as i32;
    push_b(&mut buf, b"<line x1=\"");
    push_i(&mut buf, zero_x);
    push_b(&mut buf, b"\" y1=\"");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, zero_x);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, pad_t + ph);
    push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" class=\"sp-ax-y\"/>");

    let color_pos = 0x3b82f6u32;
    let color_neg = 0x3b82f6u32;
    let row_h = ph as f64 / n as f64;
    let bar_h = (row_h * (1.0 - cfg.bar_gap.max(0.1))).max(4.0);

    for i in 0..n {
        let v = values[i];
        let cy_top = pad_t as f64 + row_h * i as f64;
        let cy = cy_top + (row_h - bar_h) / 2.0;
        let x_v = pad_l + ((v - x_min) / range * pw as f64) as i32;
        let color = if v >= 0.0 { color_pos } else { color_neg };
        let hx = hex6(color);
        let bx0 = zero_x.min(x_v);
        let bw = (zero_x - x_v).unsigned_abs() as f64;

        push_b(&mut buf, b"<rect data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &labels[i]);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, v);
        push_b(&mut buf, b"\" x=\"");
        push_i(&mut buf, bx0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" width=\"");
        push_f2(&mut buf, bw);
        push_b(&mut buf, b"\" height=\"");
        push_f2(&mut buf, bar_h);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\"/>");

        if cfg.show_text {
            let inside = bw > 32.0;
            let (tx, anchor, fill): (i32, &[u8], &[u8]) = if inside {
                let ex = if v >= 0.0 { x_v - 6 } else { x_v + 6 };
                let anc: &[u8] = if v >= 0.0 { b"end" } else { b"start" };
                (ex, anc, b"#ffffff")
            } else {
                let ex = if v >= 0.0 { x_v + 6 } else { x_v - 6 };
                let anc: &[u8] = if v >= 0.0 { b"start" } else { b"end" };
                (ex, anc, b"#111827")
            };
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy + bar_h / 2.0 + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"");
            buf.extend_from_slice(fill);
            push_b(&mut buf, b"\">");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"</text>");
        }

        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l - 8);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + bar_h / 2.0 + 4.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, truncate(&labels[i], 22));
        push_b(&mut buf, b"</text>");
    }

    if !cfg.y_label.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l + pw / 2);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, h - 6);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut buf, cfg.y_label);
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
