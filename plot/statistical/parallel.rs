use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open, svg_title, svg_legend_item};
use crate::html::hover::build_chart_html;

pub struct ParallelConfig<'a> {
    pub title: &'a str,
    pub axes: &'a [String],
    pub series_names: &'a [String],
    pub series_values: &'a [Vec<f64>],
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
}

impl<'a> Default for ParallelConfig<'a> {
    fn default() -> Self {
        Self {
            title: "", axes: &[], series_names: &[], series_values: &[],
            palette: &[], width: 1000, height: 500,
        }
    }
}

pub fn render_parallel_html(cfg: &ParallelConfig) -> String {
    let n_axes = cfg.axes.len();
    let n_series = cfg.series_names.len().min(cfg.series_values.len());
    if n_axes < 2 || n_series == 0 { return String::new(); }

    // Compute min/max per axis
    let mut mins: Vec<f64> = vec![f64::INFINITY; n_axes];
    let mut maxs: Vec<f64> = vec![f64::NEG_INFINITY; n_axes];
    for si in 0..n_series {
        let vals = &cfg.series_values[si];
        for ai in 0..n_axes.min(vals.len()) {
            if vals[ai] < mins[ai] { mins[ai] = vals[ai]; }
            if vals[ai] > maxs[ai] { maxs[ai] = vals[ai]; }
        }
    }
    for ai in 0..n_axes {
        if (maxs[ai] - mins[ai]).abs() < 1e-9 { maxs[ai] = mins[ai] + 1.0; }
    }

    let pad_l: i32 = 50;
    let pad_r: i32 = 150;
    let pad_t: i32 = 44;
    let pad_b: i32 = 44;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;

    let mut buf = Vec::<u8>::with_capacity(n_series * n_axes * 200 + 4096);
    svg_open(&mut buf, cfg.width, cfg.height);
    svg_title(&mut buf, cfg.title, cfg.width / 2, 26);

    // Draw axis lines
    for ai in 0..n_axes {
        let x = pad_l + (ai as f64 / (n_axes - 1) as f64 * plot_w as f64) as i32;
        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
        push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

        // Axis label
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 18);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut buf, &cfg.axes[ai]);
        push_b(&mut buf, b"</text>");

        // Min label
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 32);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#94a3b8\">");
        let min_s = format!("{:.1}", mins[ai]);
        buf.extend_from_slice(min_s.as_bytes());
        push_b(&mut buf, b"</text>");

        // Max label
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t - 6);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#94a3b8\">");
        let max_s = format!("{:.1}", maxs[ai]);
        buf.extend_from_slice(max_s.as_bytes());
        push_b(&mut buf, b"</text>");

        // Tick marks
        for t in 0..=4 {
            let frac = t as f64 / 4.0;
            let y = pad_t + plot_h - (frac * plot_h as f64) as i32;
            push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, x - 3);
            push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, x + 3);
            push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"0.5\"/>");
        }
    }

    // Draw polylines for each series
    for si in 0..n_series {
        let vals = &cfg.series_values[si];
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);

        // Shadow
        push_b(&mut buf, b"<polyline data-idx=\""); push_i(&mut buf, si as i32);
        push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, si as i32);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"4\" stroke-opacity=\"0.12\" points=\"");
        for ai in 0..n_axes {
            if ai >= vals.len() { break; }
            let x = pad_l as f64 + (ai as f64 / (n_axes - 1) as f64 * plot_w as f64);
            let f = (vals[ai] - mins[ai]) / (maxs[ai] - mins[ai]);
            let y = (pad_t + plot_h) as f64 - f * plot_h as f64;
            if ai > 0 { push_b(&mut buf, b" "); }
            push_f2(&mut buf, x); push_b(&mut buf, b","); push_f2(&mut buf, y);
        }
        push_b(&mut buf, b"\"/>");

        // Main line
        push_b(&mut buf, b"<polyline data-idx=\""); push_i(&mut buf, si as i32);
        push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, si as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &cfg.series_names[si]);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.8\" stroke-opacity=\"0.7\" stroke-linejoin=\"round\" points=\"");
        for ai in 0..n_axes {
            if ai >= vals.len() { break; }
            let x = pad_l as f64 + (ai as f64 / (n_axes - 1) as f64 * plot_w as f64);
            let f = (vals[ai] - mins[ai]) / (maxs[ai] - mins[ai]);
            let y = (pad_t + plot_h) as f64 - f * plot_h as f64;
            if ai > 0 { push_b(&mut buf, b" "); }
            push_f2(&mut buf, x); push_b(&mut buf, b","); push_f2(&mut buf, y);
        }
        push_b(&mut buf, b"\"/>");

        // Dots at each axis
        for ai in 0..n_axes.min(vals.len()) {
            let x = pad_l as f64 + (ai as f64 / (n_axes - 1) as f64 * plot_w as f64);
            let f = (vals[ai] - mins[ai]) / (maxs[ai] - mins[ai]);
            let y = (pad_t + plot_h) as f64 - f * plot_h as f64;
            push_b(&mut buf, b"<circle data-idx=\""); push_i(&mut buf, (si * n_axes + ai) as i32);
            push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, si as i32);
            push_b(&mut buf, b"\" cx=\""); push_f2(&mut buf, x);
            push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, y);
            push_b(&mut buf, b"\" r=\"3\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.85\"/>");
        }
    }

    // Legend
    for (li, name) in cfg.series_names.iter().enumerate() {
        let col = palette_color(cfg.palette, li);
        svg_legend_item(&mut buf, li as i32, name, col, cfg.width - pad_r + 14, pad_t + 4 + (li as i32) * 20, 20);
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, "[]")
}
