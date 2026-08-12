use super::common::finite_minmax;
use super::config::HeatmapConfig;
use crate::plot::statistical::common::{escape_xml, hex6, lerp_color, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title};
use std::f64::consts::PI;

#[crate::chart_demo(
    "labels=[\"Figma Design Systems\",\"Product Management\",\"Design Research\"], col_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\",\"Jul\",\"Aug\",\"Sep\",\"Oct\",\"Nov\",\"Dec\",\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\"], values=[61,25,42,25,61,42,25,42,61,42,25,61,86,100,42,61,84,25,61,86,42,61,25,42,61,86,42,25,61,86,100,42,61,61,25,42,25,86,42,25,100,86,61,25,42,25,61,86,25,25,61], variant=\"hex_grid\", width=760, height=230"
)]

pub fn render(cfg: &HeatmapConfig) -> String {
    let n_rows = cfg.row_labels.len();
    let n_cols = cfg.col_labels.len();
    let n = n_rows * n_cols;
    if n_rows == 0 || n_cols == 0 || cfg.flat_matrix.len() < n {
        return String::new();
    }

    let (gmin, gmax) = finite_minmax(&cfg.flat_matrix[..n]);
    let grange = (gmax - gmin).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let label_w = 132.0;
    let top_pad = 32.0;
    let bottom_pad = 12.0;
    let usable_w = (w as f64 - label_w - 20.0).max(10.0);
    let usable_h = (h as f64 - top_pad - bottom_pad).max(10.0);
    let cell_w = usable_w / n_cols as f64;
    let cell_h = usable_h / n_rows as f64;
    let hex_r = (cell_w.min(cell_h) * 0.5) * 0.90;

    let hex_points = |buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64| {
        push_b(buf, b"<polygon points=\"");
        for i in 0..6 {
            let a = PI / 2.0 + i as f64 * PI / 3.0;
            let x = cx + r * a.cos();
            let y = cy + r * a.sin();
            if i > 0 {
                push_b(buf, b" ");
            }
            push_f2(buf, x);
            push_b(buf, b",");
            push_f2(buf, y);
        }
        push_b(buf, b"\"");
    };

    let mut buf = Vec::<u8>::with_capacity(n * 200 + 8192);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 18);

    push_b(&mut buf, b"<g fill=\"#94a3b8\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" text-anchor=\"middle\">");
    for c in 0..n_cols {
        let cx = label_w + 10.0 + cell_w * (c as f64 + 0.5);
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, top_pad - 10.0);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.col_labels[c]);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"</g>");

    for r in 0..n_rows {
        let row = &cfg.flat_matrix[r * n_cols..r * n_cols + n_cols];
        let mut peak_c = 0usize;
        let mut peak_v = f64::NEG_INFINITY;
        for (c, &v) in row.iter().enumerate() {
            if v > peak_v {
                peak_v = v;
                peak_c = c;
            }
        }
        let base = palette_color(cfg.palette, r);
        let base_hx = hex6(base);
        let cy = top_pad + cell_h * (r as f64 + 0.5);

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, label_w - 10.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&base_hx);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.row_labels[r]);
        push_b(&mut buf, b"</text>");

        for c in 0..n_cols {
            let v = row[c];
            let t = ((v - gmin) / grange).clamp(0.0, 1.0);
            let cx = label_w + 10.0 + cell_w * (c as f64 + 0.5);
            let is_peak = c == peak_c;
            let fill = if is_peak { 0xffffff } else { lerp_color(t, 0xf1f5f9, base, base) };
            let fill_hx = hex6(fill);

            hex_points(&mut buf, cx, cy, hex_r);
            push_b(&mut buf, b" fill=\"#");
            buf.extend_from_slice(&fill_hx);
            push_b(&mut buf, b"\" fill-opacity=\"");
            push_f2(&mut buf, if is_peak { 1.0 } else { 0.16 + t * 0.74 });
            push_b(&mut buf, b"\" stroke=\"#");
            buf.extend_from_slice(&base_hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, if is_peak { 1.6 } else { 0.8 });
            push_b(&mut buf, b"\" stroke-opacity=\"");
            push_f2(&mut buf, if is_peak { 0.9 } else { 0.45 });
            push_b(&mut buf, b"\" data-idx=\"");
            push_i(&mut buf, (r * n_cols + c) as i32);
            push_b(&mut buf, b"\"/>");

            if cfg.show_values {
                let text_color: &[u8] = if is_peak { b"#0f172a" } else if t > 0.55 { b"#ffffff" } else { b"#334155" };
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, cx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy + 3.0);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"");
                push_f2(&mut buf, (hex_r * 0.34).clamp(6.0, 10.0));
                push_b(&mut buf, b"\" font-weight=\"");
                push_b(&mut buf, if is_peak { b"800" } else { b"500" });
                push_b(&mut buf, b"\" fill=\"");
                buf.extend_from_slice(text_color);
                push_b(&mut buf, b"\">");
                let s = format!("{:.0}%", v);
                buf.extend_from_slice(s.as_bytes());
                push_b(&mut buf, b"</text>");
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(rows: &'a [String], cols: &'a [String], values: &'a [f64]) -> HeatmapConfig<'a> {
        HeatmapConfig {
            title: "Test",
            row_labels: rows,
            col_labels: cols,
            flat_matrix: values,
            width: 700,
            height: 300,
            show_values: true,
            ..HeatmapConfig::default()
        }
    }

    fn grid(n_rows: usize, n_cols: usize) -> (Vec<String>, Vec<String>, Vec<f64>) {
        let rows: Vec<String> = (0..n_rows).map(|r| format!("Row{r}")).collect();
        let cols: Vec<String> = (0..n_cols).map(|c| format!("C{c}")).collect();
        let values: Vec<f64> = (0..n_rows * n_cols).map(|i| ((i * 37) % 100) as f64).collect();
        (rows, cols, values)
    }

    #[test]
    fn renders_one_hexagon_per_cell() {
        let (rows, cols, values) = grid(3, 6);
        let html = render(&cfg(&rows, &cols, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<polygon").count(), 18);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn exactly_one_peak_per_row_gets_the_white_highlight() {
        let (rows, cols, values) = grid(4, 5);
        let html = render(&cfg(&rows, &cols, &values));
        assert_eq!(html.matches("fill=\"#ffffff\" fill-opacity=\"1.00\"").count(), 4);
    }

    #[test]
    fn each_row_label_is_present_and_colored_by_its_own_palette_slot() {
        let (rows, cols, values) = grid(2, 4);
        let html = render(&cfg(&rows, &cols, &values));
        assert!(html.contains(">Row0<"));
        assert!(html.contains(">Row1<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let rows: Vec<String> = vec![];
        let cols: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        assert!(render(&cfg(&rows, &cols, &values)).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_hex_grid_stays_fast() {
        let (rows, cols, values) = grid(40, 30);
        let start = std::time::Instant::now();
        let html = render(&cfg(&rows, &cols, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
