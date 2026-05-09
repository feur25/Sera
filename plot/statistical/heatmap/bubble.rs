use super::common::{cell_color, clone_cfg, finite_minmax, map_value_to_t};
use super::config::HeatmapConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, truncate};

pub fn render(cfg: &HeatmapConfig) -> String {
    let cfg = HeatmapConfig {
        bubble_mode: true,
        smooth: true,
        ..clone_cfg(cfg)
    };
    let n_rows = cfg.row_labels.len();
    let col_lbls: &[String] = if cfg.col_labels.is_empty() { cfg.row_labels } else { cfg.col_labels };
    let n_cols = col_lbls.len();
    if n_rows == 0 || n_cols == 0 || cfg.flat_matrix.len() < n_rows * n_cols {
        return String::new();
    }
    let data = &cfg.flat_matrix[..n_rows * n_cols];
    let (vmin, vmax) = finite_minmax(data);
    let max_abs = vmin.abs().max(vmax.abs()).max(1e-12);

    let pad_left: i32 = 100;
    let pad_top: i32 = 88;
    let pad_right: i32 = 90;
    let pad_bottom: i32 = 52;

    let svg_w = cfg.width;
    let plot_w = (svg_w - pad_left - pad_right).max(40);
    let cell_w = (plot_w / n_cols as i32).max(8);
    let svg_h = if cfg.height > 0 { cfg.height } else { pad_top + cell_w * n_rows as i32 + pad_bottom };
    let plot_h = (svg_h - pad_top - pad_bottom).max(40);
    let cell_h = (plot_h / n_rows as i32).max(8);
    let max_r = (cell_w.min(cell_h) as f64) * 0.42;

    let mut buf = Vec::<u8>::with_capacity(n_rows * n_cols * 200 + 4096);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b" ");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\"><rect width=\"100%\" height=\"100%\" fill=\"#0f172a\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, svg_w / 2);
        push_b(&mut buf, b"\" y=\"24\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#f1f5f9\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    for (col, lbl) in col_lbls.iter().enumerate() {
        let cx = pad_left + col as i32 * cell_w + cell_w / 2;
        let cy = pad_top - 10;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, cy);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#cbd5e1\" transform=\"rotate(-40,");
        push_i(&mut buf, cx);
        push_b(&mut buf, b",");
        push_i(&mut buf, cy);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(lbl, 14));
        push_b(&mut buf, b"</text>");
    }
    let mut idx = 0i32;
    for row in 0..n_rows {
        let cy = pad_top + row as i32 * cell_h + cell_h / 2;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_left - 8);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, cy + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#cbd5e1\">");
        escape_xml(&mut buf, truncate(&cfg.row_labels[row], 14));
        push_b(&mut buf, b"</text>");
        for col in 0..n_cols {
            let v = data[row * n_cols + col];
            let cx = pad_left + col as i32 * cell_w + cell_w / 2;
            let mag = (v.abs() / max_abs).clamp(0.0, 1.0).sqrt();
            let r = (mag * max_r).max(1.5);
            let t = map_value_to_t(v, vmin, vmax, false, cfg.diverging);
            let color = cell_color(t, &cfg);
            let hx = hex6(color);
            push_b(&mut buf, b"<circle data-idx=\"");
            push_i(&mut buf, idx);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"\" data-r=\"");
            escape_xml(&mut buf, truncate(&cfg.row_labels[row], 16));
            push_b(&mut buf, b"\" data-c=\"");
            escape_xml(&mut buf, truncate(&col_lbls[col], 16));
            push_b(&mut buf, b"\" cx=\"");
            push_i(&mut buf, cx);
            push_b(&mut buf, b"\" cy=\"");
            push_i(&mut buf, cy);
            push_b(&mut buf, b"\" r=\"");
            push_f2(&mut buf, r);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.92\" stroke=\"#0a0f1c\" stroke-width=\"0.7\"/>");
            idx += 1;
        }
    }

    let bar_x = svg_w - pad_right + 14;
    let bar_w = 16;
    let bar_y0 = pad_top;
    let bar_y1 = svg_h - pad_bottom;
    let bar_h = (bar_y1 - bar_y0).max(20);
    let n_steps = 64;
    let step_h = bar_h as f64 / n_steps as f64;
    for si in 0..n_steps {
        let t_top = 1.0 - si as f64 / (n_steps - 1) as f64;
        let color = cell_color(t_top, &cfg);
        let hx = hex6(color);
        let sy = bar_y0 + (si as f64 * step_h) as i32;
        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, bar_x);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, sy);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, bar_w);
        push_b(&mut buf, b"\" height=\"");
        push_i(&mut buf, (step_h as i32 + 1).max(1));
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\"/>");
    }
    for ti in 0..5 {
        let frac = ti as f64 / 4.0;
        let v = vmax - frac * (vmax - vmin);
        let ty = bar_y0 + (frac * bar_h as f64) as i32;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, bar_x + bar_w + 6);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ty + 3);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#cbd5e1\">");
        push_f2(&mut buf, v);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let hover_json = if cfg.hover.is_empty() { "[]".to_string() } else { slots_to_json(cfg.hover) };
    build_chart_html(cfg.title, &svg, &hover_json)
}


