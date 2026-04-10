use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open_rescalable, svg_title, svg_hgrid, svg_vgrid, svg_tick_y, svg_tick_x, svg_axis_lines, svg_x_label, svg_y_label, svg_legend_item};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

pub struct BubbleConfig<'a> {
    pub title: &'a str,
    pub x_values: &'a [f64],
    pub y_values: &'a [f64],
    pub sizes: &'a [f64],
    pub categories: &'a [String],
    pub palette: &'a [u32],
    pub width: i32,
    pub height: i32,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub gridlines: bool,
    pub hover: &'a [HoverSlot],
}

impl<'a> Default for BubbleConfig<'a> {
    fn default() -> Self {
        Self {
            title: "", x_values: &[], y_values: &[], sizes: &[],
            categories: &[], palette: &[], width: 900, height: 500,
            x_label: "", y_label: "", gridlines: true, hover: &[],
        }
    }
}

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
    let pad_l: i32 = 56;
    let pad_t: i32 = 38;
    let pad_b: i32 = 52;
    let plot_w = cfg.width - pad_l - legend_w;
    let plot_h = cfg.height - pad_t - pad_b;
    let min_r = 4.0f64;
    let max_r = 40.0f64;

    let mut buf = Vec::<u8>::with_capacity(n * 300 + 4096);
    svg_open_rescalable(&mut buf, cfg.width, cfg.height, pad_l, pad_t, plot_w, plot_h);
    svg_title(&mut buf, cfg.title, pad_l + plot_w / 2, 26);

    // Grids and axes
    let n_xticks = 6i32;
    for i in 0..=n_xticks {
        let frac = i as f64 / n_xticks as f64;
        let x = pad_l + (frac * plot_w as f64) as i32;
        let val = xmin2 + frac * xr2;
        if cfg.gridlines && i > 0 { svg_vgrid(&mut buf, x, pad_t, pad_t + plot_h); }
        svg_tick_x(&mut buf, x, pad_t + plot_h + 14, val);
    }
    let n_yticks = 5i32;
    for i in 0..=n_yticks {
        let frac = i as f64 / n_yticks as f64;
        let y = pad_t + plot_h - (frac * plot_h as f64) as i32;
        let val = ymin2 + frac * yr2;
        if cfg.gridlines && i > 0 { svg_hgrid(&mut buf, pad_l, y, pad_l + plot_w); }
        svg_tick_y(&mut buf, pad_l - 6, y + 3, val);
    }
    svg_axis_lines(&mut buf, pad_l, pad_t, plot_w, plot_h);
    svg_x_label(&mut buf, cfg.x_label, pad_l + plot_w / 2, pad_t + plot_h + 42);
    svg_y_label(&mut buf, cfg.y_label, 14, pad_t, plot_h);

    // Bubbles (sorted by size desc so smaller ones are on top)
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| cfg.sizes[b].partial_cmp(&cfg.sizes[a]).unwrap_or(std::cmp::Ordering::Equal));

    for &i in &indices {
        let cx = pad_l + (((cfg.x_values[i] - xmin2) / xr2) * plot_w as f64) as i32;
        let cy = pad_t + plot_h - (((cfg.y_values[i] - ymin2) / yr2) * plot_h as f64) as i32;
        let sn = (cfg.sizes[i] - smin) / sr;
        let r = min_r + sn * (max_r - min_r);

        let ci = if i < cfg.categories.len() {
            cat_order.iter().position(|c| c == &cfg.categories[i]).unwrap_or(0)
        } else { i };
        let col = palette_color(cfg.palette, ci);
        let hx = hex6(col);

        push_b(&mut buf, b"<circle data-idx=\""); push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, ci as i32);
        push_b(&mut buf, b"\" data-y=\""); push_f2(&mut buf, cfg.y_values[i]);
        if i < cfg.categories.len() {
            push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &cfg.categories[i]);
        }
        push_b(&mut buf, b"\" cx=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.6\" stroke=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.5\"/>");
    }

    // Legend
    if has_legend {
        for (li, name) in cat_order.iter().enumerate() {
            let col = palette_color(cfg.palette, li);
            svg_legend_item(&mut buf, li as i32, name, col, cfg.width - legend_w + 10, pad_t + 4 + (li as i32) * 20, 20);
        }
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    build_chart_html(cfg.title, &svg, json)
}
