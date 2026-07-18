use super::common::{cell_color, finite_minmax};
use super::config::HeatmapConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i};
use std::f64::consts::PI;

pub fn render(cfg: &HeatmapConfig) -> String {
    let nr = cfg.row_labels.len();
    let nc = if cfg.col_labels.is_empty() { 24 } else { cfg.col_labels.len() };
    if nr == 0 || nc == 0 || cfg.flat_matrix.len() < nr * nc {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let side = w.min(h) as f64;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0 + 10.0;
    let inner_r = side * 0.072;
    let outer_r = side * 0.385;
    let ring_h = (outer_r - inner_r) / nr as f64;
    let gap_rad = PI / 180.0 * 0.35;

    let (v_min, v_max) = finite_minmax(cfg.flat_matrix);
    let norm_v = |v: f64| -> f64 {
        if v_max > v_min { ((v - v_min) / (v_max - v_min)).clamp(0.0, 1.0) } else { 0.5 }
    };

    let n_total = nr * nc;
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n_total);
    let mut buf = Vec::<u8>::with_capacity(n_total * 220 + 16384);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    let col_lo = hex6(cfg.color_low);
    let col_hi = hex6(cfg.color_high);
    push_b(&mut buf, b"<defs><linearGradient id=\"ph-cscale\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
    push_b(&mut buf, b"<stop offset=\"0\" stop-color=\"#");
    buf.extend_from_slice(&col_lo);
    push_b(&mut buf, b"\"/><stop offset=\"1\" stop-color=\"#");
    buf.extend_from_slice(&col_hi);
    push_b(&mut buf, b"\"/></linearGradient></defs>");

    for ri in 1..nr {
        let r = inner_r + ri as f64 * ring_h;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#060a14\" stroke-width=\"1.5\"/>");
    }

    for ri in 0..nr {
        let r0 = inner_r + ri as f64 * ring_h + 0.8;
        let r1 = inner_r + (ri + 1) as f64 * ring_h - 0.8;
        for ci in 0..nc {
            let val = cfg.flat_matrix[ri * nc + ci];
            let t = norm_v(val);
            let col = cell_color(t, cfg);
            let hx = hex6(col);

            let a0 = -PI * 0.5 + ci as f64 * 2.0 * PI / nc as f64 + gap_rad;
            let a1 = -PI * 0.5 + (ci + 1) as f64 * 2.0 * PI / nc as f64 - gap_rad;

            let x00 = cx + r0 * a0.cos();
            let y00 = cy + r0 * a0.sin();
            let x01 = cx + r0 * a1.cos();
            let y01 = cy + r0 * a1.sin();
            let x10 = cx + r1 * a1.cos();
            let y10 = cy + r1 * a1.sin();
            let x11 = cx + r1 * a0.cos();
            let y11 = cy + r1 * a0.sin();

            push_b(&mut buf, b"<path data-idx=\"");
            push_i(&mut buf, (ri * nc + ci) as i32);
            push_b(&mut buf, b"\" d=\"M ");
            push_f2(&mut buf, x00);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y00);
            push_b(&mut buf, b" A ");
            push_f2(&mut buf, r0);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, r0);
            push_b(&mut buf, b" 0 0 1 ");
            push_f2(&mut buf, x01);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y01);
            push_b(&mut buf, b" L ");
            push_f2(&mut buf, x10);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y10);
            push_b(&mut buf, b" A ");
            push_f2(&mut buf, r1);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, r1);
            push_b(&mut buf, b" 0 0 0 ");
            push_f2(&mut buf, x11);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, y11);
            push_b(&mut buf, b" Z\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");

            let region = cfg.row_labels.get(ri).map(|s| s.as_str()).unwrap_or("?");
            let hour = cfg.col_labels.get(ci).map(|s| s.as_str()).unwrap_or("");
            slots.push(
                HoverSlot::new(format!("{} · {}", region, hour))
                    .kv("Intensite", format!("{:.0} gCO2e/kWh", val))
                    .kv("Region", region.to_string())
                    .kv("Heure locale", hour.to_string()),
            );
        }
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, outer_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#1e293b\" stroke-width=\"1\"/>");
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, inner_r);
    push_b(&mut buf, b"\" fill=\"#060a14\" stroke=\"#1e293b\" stroke-width=\"1\"/>");
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"3\" fill=\"#6366f133\"/>");

    for ci in 0..nc {
        let a_mid = -PI * 0.5 + (ci as f64 + 0.5) * 2.0 * PI / nc as f64;
        let tx0 = cx + outer_r * a_mid.cos();
        let ty0 = cy + outer_r * a_mid.sin();
        let tx1 = cx + (outer_r + 7.0) * a_mid.cos();
        let ty1 = cy + (outer_r + 7.0) * a_mid.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, tx0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, ty0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, tx1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ty1);
        push_b(&mut buf, b"\" stroke=\"#1e293b\" stroke-width=\"0.6\"/>");

        if ci % 3 == 0 {
            let lr = outer_r + 19.0;
            let lx = cx + lr * a_mid.cos();
            let ly = cy + lr * a_mid.sin() + 3.5;
            let hour_label = cfg.col_labels.get(ci).map(|s| s.as_str()).unwrap_or("");
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" fill=\"#334155\">");
            escape_xml(&mut buf, hour_label);
            push_b(&mut buf, b"</text>");
        }
    }

    let leg_y = (cy + outer_r + 34.0) as i32;
    let entry_w = 120i32;
    let total_leg_w = nr as i32 * entry_w;
    let leg_x0 = (w - total_leg_w) / 2;

    for ri in 0..nr {
        let lx = leg_x0 + ri as i32 * entry_w;
        let avg_val = (0..nc).map(|ci| cfg.flat_matrix[ri * nc + ci]).sum::<f64>() / nc as f64;
        let avg_t = norm_v(avg_val);
        let swatch_col = cell_color(avg_t, cfg);
        let sw_hx = hex6(swatch_col);
        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, leg_y - 5);
        push_b(&mut buf, b"\" width=\"8\" height=\"8\" rx=\"2\" fill=\"#");
        buf.extend_from_slice(&sw_hx);
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx + 12);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, leg_y + 2);
        push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" fill=\"#475569\">");
        let label: String = cfg.row_labels.get(ri).map(|s| s.chars().take(13).collect()).unwrap_or_default();
        escape_xml(&mut buf, &label);
        push_b(&mut buf, b"</text>");
    }

    let bar_y = leg_y + 22;
    let bar_w = 200i32;
    let bar_x = (w - bar_w) / 2;
    push_b(&mut buf, b"<rect x=\"");
    push_i(&mut buf, bar_x);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, bar_y);
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, bar_w);
    push_b(&mut buf, b"\" height=\"5\" rx=\"2\" fill=\"url(#ph-cscale)\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, bar_x);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, bar_y + 14);
    push_b(&mut buf, b"\" text-anchor=\"start\" font-family=\"system-ui,sans-serif\" font-size=\"6.5\" fill=\"#334155\">");
    escape_xml(&mut buf, &format!("{:.0}", v_min));
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, bar_x + bar_w);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, bar_y + 14);
    push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"system-ui,sans-serif\" font-size=\"6.5\" fill=\"#334155\">");
    escape_xml(&mut buf, &format!("{:.0} gCO2e/kWh", v_max));
    push_b(&mut buf, b"</text>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"20\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
          font-size=\"9.5\" font-weight=\"700\" fill=\"#1a2744\" letter-spacing=\"3\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");

    let svg_str = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg_str, &slots_to_json(&slots))
}
