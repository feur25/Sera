use super::common::{cell_color, clone_cfg, finite_minmax, map_value_to_t};
use super::config::HeatmapConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, truncate};

pub const DEMO_KWARGS: &str = "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], col_labels=[\"8h\",\"12h\",\"16h\",\"20h\"], values=[5,9,7,3,6,12,10,4,8,15,13,7,4,8,11,5,3,7,9,2]";
pub fn render(cfg: &HeatmapConfig) -> String {
    let cfg = HeatmapConfig {
        marginal_mode: true,
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

    let pad_left: i32 = 100;
    let pad_top: i32 = 110;
    let pad_right: i32 = 150;
    let pad_bottom: i32 = 52;
    let marg_top = 60;
    let marg_right = 50;

    let svg_w = cfg.width;
    let plot_w = (svg_w - pad_left - pad_right).max(40);
    let cell_w = (plot_w / n_cols as i32).max(6);
    let svg_h = if cfg.height > 0 { cfg.height } else { pad_top + cell_w * n_rows as i32 + pad_bottom };
    let plot_h = (svg_h - pad_top - pad_bottom).max(40);
    let cell_h = (plot_h / n_rows as i32).max(6);

    let col_sums: Vec<f64> = (0..n_cols).map(|c| (0..n_rows).map(|r| data[r * n_cols + c]).filter(|v| v.is_finite()).sum()).collect();
    let row_sums: Vec<f64> = (0..n_rows).map(|r| (0..n_cols).map(|c| data[r * n_cols + c]).filter(|v| v.is_finite()).sum()).collect();
    let max_col = col_sums.iter().cloned().fold(0.0f64, f64::max).max(1e-12);
    let max_row = row_sums.iter().cloned().fold(0.0f64, f64::max).max(1e-12);

    let mut buf = Vec::<u8>::with_capacity(n_rows * n_cols * 220 + 4096);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b" ");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\"><rect width=\"100%\" height=\"100%\" fill=\"#f8f9fa\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, svg_w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let marg_y_base = pad_top - 8;
    for col in 0..n_cols {
        let h = (col_sums[col].max(0.0) / max_col * marg_top as f64) as i32;
        let cx = pad_left + col as i32 * cell_w;
        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, cx + 1);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, marg_y_base - h);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, cell_w - 2);
        push_b(&mut buf, b"\" height=\"");
        push_i(&mut buf, h.max(1));
        push_b(&mut buf, b"\" fill=\"#6366f1\" fill-opacity=\"0.75\"/>");
    }

    let marg_x_base = svg_w - pad_right + 6;
    for row in 0..n_rows {
        let w = (row_sums[row].max(0.0) / max_row * marg_right as f64) as i32;
        let ry = pad_top + row as i32 * cell_h;
        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, marg_x_base);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ry + 1);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, w.max(1));
        push_b(&mut buf, b"\" height=\"");
        push_i(&mut buf, cell_h - 2);
        push_b(&mut buf, b"\" fill=\"#f43f5e\" fill-opacity=\"0.75\"/>");
    }

    for (col, lbl) in col_lbls.iter().enumerate() {
        let cx = pad_left + col as i32 * cell_w + cell_w / 2;
        let cy = pad_top - 10;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, cy);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#4b5563\" transform=\"rotate(-40,");
        push_i(&mut buf, cx);
        push_b(&mut buf, b",");
        push_i(&mut buf, cy);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(lbl, 12));
        push_b(&mut buf, b"</text>");
    }
    let mut idx = 0i32;
    for row in 0..n_rows {
        let ry = pad_top + row as i32 * cell_h;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_left - 6);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ry + cell_h / 2 + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#4b5563\">");
        escape_xml(&mut buf, truncate(&cfg.row_labels[row], 14));
        push_b(&mut buf, b"</text>");
        for col in 0..n_cols {
            let v = data[row * n_cols + col];
            let t = map_value_to_t(v, vmin, vmax, false, cfg.diverging);
            let color = cell_color(t, &cfg);
            let hx = hex6(color);
            let cx = pad_left + col as i32 * cell_w;
            push_b(&mut buf, b"<rect data-idx=\"");
            push_i(&mut buf, idx);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"\" x=\"");
            push_i(&mut buf, cx);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, ry);
            push_b(&mut buf, b"\" width=\"");
            push_i(&mut buf, cell_w);
            push_b(&mut buf, b"\" height=\"");
            push_i(&mut buf, cell_h);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");
            idx += 1;
        }
    }

    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, pad_left + plot_w / 2);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, pad_top - marg_top - 14);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6366f1\" font-weight=\"700\">column totals</text>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, marg_x_base + marg_right / 2);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, pad_top - 8);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#f43f5e\" font-weight=\"700\">row totals</text>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let hover_json = if cfg.hover.is_empty() { "[]".to_string() } else { slots_to_json(cfg.hover) };
    build_chart_html(cfg.title, &svg, &hover_json)
}


