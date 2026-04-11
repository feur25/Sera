use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_legend_item, Frame};
use crate::html::hover::slots_to_json;

crate::chart_config!(BubbleConfig, 900, 500;
    struct {
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub sizes: &'a [f64],
        pub categories: &'a [String],
        pub palette: &'a [u32],
    }
    defaults {
        x_values: &[],
        y_values: &[],
        sizes: &[],
        categories: &[],
        palette: &[],
    }
);

pub fn render_bubble_html(cfg: &BubbleConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len()).min(cfg.sizes.len());
    if n == 0 { return String::new(); }

    let mut xmin = f64::INFINITY; let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY; let mut ymax = f64::NEG_INFINITY;
    let mut smin = f64::INFINITY; let mut smax = f64::NEG_INFINITY;
    for i in 0..n {
        if cfg.x_values[i] < xmin { xmin = cfg.x_values[i]; }
        if cfg.x_values[i] > xmax { xmax = cfg.x_values[i]; }
        if cfg.y_values[i] < ymin { ymin = cfg.y_values[i]; }
        if cfg.y_values[i] > ymax { ymax = cfg.y_values[i]; }
        if cfg.sizes[i] < smin { smin = cfg.sizes[i]; }
        if cfg.sizes[i] > smax { smax = cfg.sizes[i]; }
    }
    let xr = (xmax - xmin).max(1e-9);
    let yr = (ymax - ymin).max(1e-9);
    let sr = (smax - smin).max(1e-9);
    let xpad = xr * 0.08; let ypad = yr * 0.08;
    let xmin2 = xmin - xpad; let xmax2 = xmax + xpad;
    let ymin2 = ymin - ypad; let ymax2 = ymax + ypad;
    let xr2 = xmax2 - xmin2; let yr2 = ymax2 - ymin2;

    // Detect unique categories for legend
    let mut cat_order: Vec<String> = Vec::new();
    for i in 0..n {
        let c = if i < cfg.categories.len() { &cfg.categories[i] } else { "" };
        let cs = c.to_string();
        if !c.is_empty() && !cat_order.contains(&cs) { cat_order.push(cs); }
    }

    let has_legend = !cat_order.is_empty();
    let legend_w: i32 = if has_legend { 140 } else { 20 };
    let min_r = 4.0f64;
    let max_r = 40.0f64;

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 38, 52, legend_w, n * 300 + 4096);
    f.open(cfg.title, true);
    f.x_grid(6, xmin2, xmax2, cfg.gridlines);
    f.y_grid(5, ymin2, ymax2, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    // Bubbles (sorted by size: larger behind smaller; sort_order overrides direction)
    let mut indices: Vec<usize> = (0..n).collect();
    let sort_desc = cfg.sort_order == "asc" || cfg.sort_order == "ascending";
    if sort_desc {
        indices.sort_by(|&a, &b| cfg.sizes[a].partial_cmp(&cfg.sizes[b]).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        indices.sort_by(|&a, &b| cfg.sizes[b].partial_cmp(&cfg.sizes[a]).unwrap_or(std::cmp::Ordering::Equal));
    }

    for &i in &indices {
        let cx = f.pl + (((cfg.x_values[i] - xmin2) / xr2) * f.pw as f64) as i32;
        let cy = f.pt + f.ph - (((cfg.y_values[i] - ymin2) / yr2) * f.ph as f64) as i32;
        let sn = (cfg.sizes[i] - smin) / sr;
        let r = min_r + sn * (max_r - min_r);

        let ci = if i < cfg.categories.len() {
            cat_order.iter().position(|c| c == &cfg.categories[i]).unwrap_or(0)
        } else { i };
        let col = palette_color(cfg.palette, ci);
        let hx = hex6(col);

        push_b(&mut f.buf, b"<circle data-idx=\""); push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" data-series=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" data-y=\""); push_f2(&mut f.buf, cfg.y_values[i]);
        if i < cfg.categories.len() {
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, &cfg.categories[i]);
        }
        push_b(&mut f.buf, b"\" cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, r);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.6\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.5\"/>");
    }

    // Legend
    if has_legend {
        for (li, name) in cat_order.iter().enumerate() {
            let col = palette_color(cfg.palette, li);
            svg_legend_item(&mut f.buf, li as i32, name, col, cfg.width - legend_w + 10, f.pt + 4 + (li as i32) * 20, 20);
        }
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}
