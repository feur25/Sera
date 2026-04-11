use super::common::{push_b, hex6, escape_xml, truncate, lerp_color, push_i, push_f2};
use crate::html::hover::{slots_to_json, build_chart_html};

pub struct Heatmap;

crate::chart_config!(HeatmapConfig, 900, 0;
    struct {
        pub row_labels: &'a [String],
        pub col_labels: &'a [String],
        pub flat_matrix: &'a [f64],
        pub show_values: bool,
        pub value_min_cell: i32,
        pub color_low: u32,
        pub color_mid: u32,
        pub color_high: u32,
        pub value_decimals: usize,
        pub col_label_angle: i32,
    }
    defaults {
        row_labels: &[],
        col_labels: &[],
        flat_matrix: &[],
        show_values: true,
        value_min_cell: 26,
        color_low: 0x6366F1,
        color_mid: 0xfafbfc,
        color_high: 0xF43F5E,
        value_decimals: 2,
        col_label_angle: 40,
    }
);

pub fn render_heatmap_html(cfg: &HeatmapConfig) -> String {
    let n_rows = cfg.row_labels.len();
    let col_lbls: &[String] = if cfg.col_labels.is_empty() { cfg.row_labels } else { cfg.col_labels };
    let n_cols = col_lbls.len();
    if n_rows == 0 || n_cols == 0 || cfg.flat_matrix.len() < n_rows * n_cols {
        return String::new();
    }
    let data = &cfg.flat_matrix[..n_rows * n_cols];
    let min_val = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1e-12);
    let pad_left: i32 = 100;
    let pad_top: i32 = 88;
    let pad_right: i32 = 24;
    let pad_bottom: i32 = 52;
    let cell_w = ((cfg.width - pad_left - pad_right) / n_cols as i32).max(4);
    let cell_h = cell_w;
    let svg_h = if cfg.height > 0 { cfg.height } else { pad_top + cell_h * n_rows as i32 + pad_bottom };
    let svg_w = cfg.width;
    let auto_hover = cfg.hover.is_empty();
    let total_cells = n_rows * n_cols;
    let mut buf = Vec::<u8>::with_capacity(total_cells * 200 + 2048);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b" ");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#f8f9fa\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, svg_w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }
    for (col, lbl) in col_lbls.iter().enumerate() {
        let x = pad_left + col as i32 * cell_w + cell_w / 2;
        let y = pad_top - 8;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, y);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#4b5563\" transform=\"rotate(-");
        push_i(&mut buf, cfg.col_label_angle);
        push_b(&mut buf, b",");
        push_i(&mut buf, x);
        push_b(&mut buf, b",");
        push_i(&mut buf, y);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(lbl, 14));
        push_b(&mut buf, b"</text>");
    }
    let mut idx = 0usize;
    for row in 0..n_rows {
        let ry = pad_top + row as i32 * cell_h;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_left - 5);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ry + cell_h / 2 + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#4b5563\">");
        escape_xml(&mut buf, truncate(&cfg.row_labels[row], 14));
        push_b(&mut buf, b"</text>");
        for col in 0..n_cols {
            let val = data[row * n_cols + col];
            let t = ((val - min_val) / range).clamp(0.0, 1.0);
            let color = lerp_color(t, cfg.color_low, cfg.color_mid, cfg.color_high);
            let hx = hex6(color);
            let hx_str = unsafe { std::str::from_utf8_unchecked(&hx) };
            let cx = pad_left + col as i32 * cell_w;
            
            push_b(&mut buf, b"<rect data-idx=\"");
            push_i(&mut buf, idx as i32);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, val);
            push_b(&mut buf, b"\" data-r=\"");
            escape_xml(&mut buf, truncate(&cfg.row_labels[row], 16));
            push_b(&mut buf, b"\" data-c=\"");
            escape_xml(&mut buf, truncate(&col_lbls[col], 16));
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
            push_b(&mut buf, b"\" rx=\"2\" stroke=\"#e2e8f0\" stroke-width=\"0.4\"/>");
            if cfg.show_values && cell_w >= cfg.value_min_cell {
                let lum = 0.299 * ((color >> 16 & 0xFF) as f64)
                    + 0.587 * ((color >> 8 & 0xFF) as f64)
                    + 0.114 * ((color & 0xFF) as f64);
                let text_col = if lum > 140.0 { b"#1f2937".as_ref() } else { b"#f9fafb".as_ref() };
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, cx + cell_w / 2);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, ry + cell_h / 2);
                push_b(&mut buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"");
                buf.extend_from_slice(text_col);
                push_b(&mut buf, b"\">");
                push_f2(&mut buf, val);
                push_b(&mut buf, b"</text>");
            }
            if !auto_hover {
            }
            idx += 1;
        }
    }
    let scale_y: i32 = svg_h - pad_bottom + 8;
    let scale_x0: f64 = pad_left as f64;
    let scale_w: f64 = (svg_w - pad_left - pad_right) as f64;
    let n_steps = 32usize;
    let step_w = scale_w / n_steps as f64;
    for si in 0..n_steps {
        let t = si as f64 / (n_steps - 1) as f64;
        let color = lerp_color(t, cfg.color_low, cfg.color_mid, cfg.color_high);
        let hx = hex6(color);
        let sx = scale_x0 + si as f64 * step_w;

        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, sx as i32);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, scale_y);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, (step_w as i32 + 1).max(1));
        push_b(&mut buf, b"\" height=\"12\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"none\"/>");
    }
    let label_y = scale_y + 22;
    let scale_pw = svg_w - pad_left - pad_right;
    let scale_static = b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">";
    for (lx, lv) in [(pad_left, min_val), (pad_left + scale_pw / 2, (min_val + max_val) / 2.0), (pad_left + scale_pw, max_val)] {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, label_y);
        buf.extend_from_slice(scale_static);
        push_f2(&mut buf, lv);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let hover_json = if auto_hover { "[]".to_string() } else { slots_to_json(cfg.hover) };
    build_chart_html(cfg.title, &svg, &hover_json)
}
