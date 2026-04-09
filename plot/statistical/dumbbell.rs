use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_open_rescalable, svg_title, svg_hgrid, svg_vgrid, svg_tick_y, svg_tick_x, svg_axis_lines, svg_x_label, svg_y_label, svg_legend_item};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct DumbbellConfig<'a> {
    pub title: &'a str,
    pub labels: &'a [String],
    pub values_start: &'a [f64],
    pub values_end: &'a [f64],
    pub series_labels: (&'a str, &'a str),
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub gridlines: bool,
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for DumbbellConfig<'a> {
    fn default() -> Self {
        Self {
            title: "", labels: &[], values_start: &[], values_end: &[],
            series_labels: ("Start", "End"),
            palette: &[], width: 1000, height: 500,
            x_label: "", y_label: "",
            gridlines: true, hover: &[],
        }
    }
}

pub fn render_dumbbell_html(cfg: &DumbbellConfig) -> String {
    let n = cfg.labels.len().min(cfg.values_start.len()).min(cfg.values_end.len());
    if n == 0 { return String::new(); }

    let mut global_min = f64::INFINITY;
    let mut global_max = f64::NEG_INFINITY;
    for i in 0..n {
        let lo = cfg.values_start[i].min(cfg.values_end[i]);
        let hi = cfg.values_start[i].max(cfg.values_end[i]);
        if lo < global_min { global_min = lo; }
        if hi > global_max { global_max = hi; }
    }
    if global_min > 0.0 { global_min = 0.0; }
    let val_range = (global_max - global_min).max(1.0);

    let c1 = if !cfg.palette.is_empty() { cfg.palette[0] } else { 0x6366F1 };
    let c2 = if cfg.palette.len() >= 2 { cfg.palette[1] } else { 0x22D3EE };

    let pad_l: i32 = 132;
    let pad_t: i32 = 38;
    let pad_b: i32 = 52;
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;

    let mut buf = Vec::<u8>::with_capacity(n * 400 + 2048);
    svg_open_rescalable(&mut buf, cfg.width, cfg.height, pad_l, pad_t, plot_w, plot_h);
    svg_title(&mut buf, cfg.title, pad_l + plot_w / 2, 26);

    let n_xticks = 6i32;
    for i in 0..=n_xticks {
        let frac = i as f64 / n_xticks as f64;
        let x = pad_l + (frac * plot_w as f64) as i32;
        let val = global_min + frac * val_range;
        if cfg.gridlines && i > 0 { svg_vgrid(&mut buf, x, pad_t, pad_t + plot_h); }
        svg_tick_x(&mut buf, x, pad_t + plot_h + 14, val);
    }

    svg_axis_lines(&mut buf, pad_l, pad_t, plot_w, plot_h);
    svg_x_label(&mut buf, cfg.x_label, pad_l + plot_w / 2, pad_t + plot_h + 42);
    svg_y_label(&mut buf, cfg.y_label, 14, pad_t, plot_h);

    let pitch = plot_h as f64 / n as f64;
    let hx1 = hex6(c1);
    let hx2 = hex6(c2);

    for i in 0..n {
        let cy = pad_t + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = pad_l + (((cfg.values_start[i] - global_min) / val_range) * plot_w as f64) as i32;
        let x2 = pad_l + (((cfg.values_end[i] - global_min) / val_range) * plot_w as f64) as i32;

        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l - 6);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, cy + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut buf, truncate(&cfg.labels[i], 18));
        push_b(&mut buf, b"</text>");

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, x1);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, x2);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" stroke=\"#9ca3af\" stroke-width=\"2\" stroke-linecap=\"round\"/>");

        push_b(&mut buf, b"<circle data-idx=\""); push_i(&mut buf, (i * 2) as i32);
        push_b(&mut buf, b"\" data-series=\"0\" data-y=\""); push_f2(&mut buf, cfg.values_start[i]);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b" ("); escape_xml(&mut buf, cfg.series_labels.0);
        push_b(&mut buf, b")\" cx=\""); push_i(&mut buf, x1);
        push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"6\" fill=\"#"); buf.extend_from_slice(&hx1);
        push_b(&mut buf, b"\"/>");

        push_b(&mut buf, b"<circle data-idx=\""); push_i(&mut buf, (i * 2 + 1) as i32);
        push_b(&mut buf, b"\" data-series=\"1\" data-y=\""); push_f2(&mut buf, cfg.values_end[i]);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b" ("); escape_xml(&mut buf, cfg.series_labels.1);
        push_b(&mut buf, b")\" cx=\""); push_i(&mut buf, x2);
        push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"6\" fill=\"#"); buf.extend_from_slice(&hx2);
        push_b(&mut buf, b"\"/>");
    }

    svg_legend_item(&mut buf, 0, cfg.series_labels.0, c1, cfg.width - 140, pad_t + 4, 20);
    svg_legend_item(&mut buf, 1, cfg.series_labels.1, c2, cfg.width - 140, pad_t + 22, 20);

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}
