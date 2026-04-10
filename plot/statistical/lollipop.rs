use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open_rescalable, svg_title, svg_axis_lines, svg_y_label, svg_x_label, svg_hgrid, svg_vgrid, svg_tick_y, truncate, apply_sort};
use crate::html::hover::build_chart_html;

pub struct LollipopConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub values: &'a [f64],
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub palette: &'a [u32],
    pub color_hex: u32,
    pub gridlines: bool,
    pub show_values: bool,
    pub orientation: u8,
    pub sort_order: &'a str,
    pub width: i32,
    pub height: i32,
}

impl<'a> Default for LollipopConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            labels: &[],
            values: &[],
            x_label: "",
            y_label: "",
            palette: &[],
            color_hex: 0x6366F1,
            gridlines: true,
            show_values: false,
            orientation: b'v',
            sort_order: "none",
            width: 900,
            height: 480,
        }
    }
}

pub fn render_lollipop_html(cfg: &LollipopConfig) -> String {
    if cfg.labels.is_empty() || cfg.values.is_empty() { return String::new(); }
    let (sorted_labels, sorted_values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = sorted_labels.len().min(sorted_values.len());
    let max_val = sorted_values[..n].iter().copied().fold(0.0_f64, f64::max).max(1.0);
    let horiz = cfg.orientation == b'h';
    let single_color = cfg.color_hex != 0;

    let pad_l: i32 = if horiz { 120 } else { 56 };
    let pad_t: i32 = 42;
    let pad_b: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;

    let mut b = Vec::<u8>::with_capacity(n * 180 + 2048);
    svg_open_rescalable(&mut b, cfg.width, cfg.height, pad_l, pad_t, plot_w, plot_h);
    svg_title(&mut b, cfg.title, cfg.width / 2, 26);

    if !horiz {
        let n_yticks: i32 = 5;
        if cfg.gridlines {
            for ti in 0..=n_yticks {
                let y = pad_t + plot_h - (plot_h as f64 * ti as f64 / n_yticks as f64) as i32;
                svg_hgrid(&mut b, pad_l, pad_l + plot_w, y);
            }
        }
        svg_y_label(&mut b, cfg.y_label, 12, pad_t, plot_h);
        for ti in 0..=n_yticks {
            let frac = ti as f64 / n_yticks as f64;
            let y = pad_t + plot_h - (plot_h as f64 * frac) as i32;
            svg_tick_y(&mut b, pad_l - 4, y + 4, frac * max_val);
        }
        svg_axis_lines(&mut b, pad_l, pad_t, plot_w, plot_h);
        let step = plot_w as f64 / n as f64;
        for i in 0..n {
            let cx = pad_l + (step * 0.5 + step * i as f64) as i32;
            let frac = (sorted_values[i] / max_val).clamp(0.0, 1.0);
            let top_y = pad_t + plot_h - (plot_h as f64 * frac) as i32;
            let base_y = pad_t + plot_h;
            let color = if single_color { cfg.color_hex } else { palette_color(cfg.palette, i) };
            let hx = hex6(color);
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, cx);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, base_y);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, cx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, top_y);
            push_b(&mut b, b"\" data-idx=\""); push_i(&mut b, i as i32);
            push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1.8\"/>");
            push_b(&mut b, b"<circle data-idx=\""); push_i(&mut b, i as i32);
            push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, sorted_values[i]);
            push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &sorted_labels[i]);
            push_b(&mut b, b"\" cx=\""); push_i(&mut b, cx);
            push_b(&mut b, b"\" cy=\""); push_i(&mut b, top_y);
            push_b(&mut b, b"\" r=\"5\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\"/>");
            if cfg.show_values {
                push_b(&mut b, b"<text x=\""); push_i(&mut b, cx);
                push_b(&mut b, b"\" y=\""); push_i(&mut b, top_y - 9);
                push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
                push_f2(&mut b, sorted_values[i]);
                push_b(&mut b, b"</text>");
            }
            let tick_step = ((n as f64 / 12.0).ceil() as usize).max(1);
            if i % tick_step == 0 {
                push_b(&mut b, b"<text x=\""); push_i(&mut b, cx);
                push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t + plot_h + 14);
                push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
                escape_xml(&mut b, truncate(&sorted_labels[i], 12));
                push_b(&mut b, b"</text>");
            }
        }
        svg_x_label(&mut b, cfg.x_label, pad_l + plot_w / 2, cfg.height - 4);
    } else {
        let n_xticks: i32 = 5;
        if cfg.gridlines {
            for ti in 0..=n_xticks {
                let x = pad_l + (plot_w as f64 * ti as f64 / n_xticks as f64) as i32;
                svg_vgrid(&mut b, x, pad_t, pad_t + plot_h);
            }
        }
        svg_axis_lines(&mut b, pad_l, pad_t, plot_w, plot_h);
        let step = plot_h as f64 / n as f64;
        for i in 0..n {
            let cy = pad_t + (step * 0.5 + step * i as f64) as i32;
            let frac = (sorted_values[i] / max_val).clamp(0.0, 1.0);
            let rx = pad_l + (plot_w as f64 * frac) as i32;
            let color = if single_color { cfg.color_hex } else { palette_color(cfg.palette, i) };
            let hx = hex6(color);
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, cy);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, rx);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, cy);
            push_b(&mut b, b"\" data-idx=\""); push_i(&mut b, i as i32);
            push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1.8\"/>");
            push_b(&mut b, b"<circle data-idx=\""); push_i(&mut b, i as i32);
            push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, sorted_values[i]);
            push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &sorted_labels[i]);
            push_b(&mut b, b"\" cx=\""); push_i(&mut b, rx);
            push_b(&mut b, b"\" cy=\""); push_i(&mut b, cy);
            push_b(&mut b, b"\" r=\"5\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\"/>");
            push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 6);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, cy + 4);
            push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
            escape_xml(&mut b, truncate(&sorted_labels[i], 16));
            push_b(&mut b, b"</text>");
            if cfg.show_values {
                push_b(&mut b, b"<text x=\""); push_i(&mut b, rx + 9);
                push_b(&mut b, b"\" y=\""); push_i(&mut b, cy + 4);
                push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
                push_f2(&mut b, sorted_values[i]);
                push_b(&mut b, b"</text>");
            }
        }
        for ti in 0..=n_xticks {
            let frac = ti as f64 / n_xticks as f64;
            let x = pad_l + (plot_w as f64 * frac) as i32;
            push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t + plot_h + 14);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#9ca3af\">");
            push_f2(&mut b, frac * max_val);
            push_b(&mut b, b"</text>");
        }
        svg_x_label(&mut b, cfg.x_label, pad_l + plot_w / 2, cfg.height - 4);
        svg_y_label(&mut b, cfg.y_label, 12, pad_t, plot_h);
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
