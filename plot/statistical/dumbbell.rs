use super::common::{sorted, sort_indices, push_b, push_i, push_f2, escape_xml, hex6, truncate, svg_legend_item, Frame};
use crate::html::hover::slots_to_json;

crate::chart_config!(DumbbellConfig, 1000, 500;
    struct {
        pub labels: &'a [String],
        pub values_start: &'a [f64],
        pub values_end: &'a [f64],
        pub series_names: (&'a str, &'a str),
        pub palette: &'a [u32],
    }
    defaults {
        labels: &[],
        values_start: &[],
        values_end: &[],
        series_names: ("Start", "End"),
        palette: &[],
    }
);

pub fn render_dumbbell_html(cfg: &DumbbellConfig) -> String {
    let n = cfg.labels.len().min(cfg.values_start.len()).min(cfg.values_end.len());
    if n == 0 { return String::new(); }
    let sort_idx = sort_indices(n, cfg.values_start, cfg.labels, cfg.sort_order);
    let s_labels = sorted(&sort_idx, cfg.labels);
    let s_start  = sorted(&sort_idx, cfg.values_start);
    let s_end    = sorted(&sort_idx, cfg.values_end);

    let mut global_min = f64::INFINITY;
    let mut global_max = f64::NEG_INFINITY;
    for i in 0..n {
        let lo = s_start[i].min(s_end[i]);
        let hi = s_start[i].max(s_end[i]);
        if lo < global_min { global_min = lo; }
        if hi > global_max { global_max = hi; }
    }
    if global_min > 0.0 { global_min = 0.0; }
    let val_range = (global_max - global_min).max(1.0);

    let c1 = if !cfg.palette.is_empty() { cfg.palette[0] } else { 0x6366F1 };
    let c2 = if cfg.palette.len() >= 2 { cfg.palette[1] } else { 0x22D3EE };

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 132, 38, 52, 20, n * 400 + 2048);
    f.open(cfg.title, true);
    f.x_grid(6, global_min, global_min + val_range, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let pitch = f.ph as f64 / n as f64;
    let hx1 = hex6(c1);
    let hx2 = hex6(c2);

    for i in 0..n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = f.pl + (((s_start[i] - global_min) / val_range) * f.pw as f64) as i32;
        let x2 = f.pl + (((s_end[i] - global_min) / val_range) * f.pw as f64) as i32;

        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, f.pl - 6);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, cy + 3);
        push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
        escape_xml(&mut f.buf, truncate(&s_labels[i], 18));
        push_b(&mut f.buf, b"</text>");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" stroke=\"#9ca3af\" stroke-width=\"2\" stroke-linecap=\"round\"/>");

        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, (i * 2) as i32);
        push_b(&mut f.buf, b"\" data-series=\"0\" data-y=\""); push_f2(&mut f.buf, s_start[i]);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &s_labels[i]);
        push_b(&mut f.buf, b" ("); escape_xml(&mut f.buf, cfg.series_names.0);
        push_b(&mut f.buf, b")\" cx=\""); push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"6\" fill=\"#"); f.buf.extend_from_slice(&hx1);
        push_b(&mut f.buf, b"\"/>");

        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, (i * 2 + 1) as i32);
        push_b(&mut f.buf, b"\" data-series=\"1\" data-y=\""); push_f2(&mut f.buf, s_end[i]);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &s_labels[i]);
        push_b(&mut f.buf, b" ("); escape_xml(&mut f.buf, cfg.series_names.1);
        push_b(&mut f.buf, b")\" cx=\""); push_i(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"6\" fill=\"#"); f.buf.extend_from_slice(&hx2);
        push_b(&mut f.buf, b"\"/>");
    }

    svg_legend_item(&mut f.buf, 0, cfg.series_names.0, c1, cfg.width - 140, f.pt + 4, 20);
    svg_legend_item(&mut f.buf, 1, cfg.series_names.1, c2, cfg.width - 140, f.pt + 22, 20);

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
