pub struct Scatter;

pub fn render_scatter_fast(
    values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
) -> String {
    let n = values.len().min(labels.len());
    if n == 0 { return String::new(); }
    
    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let max_val = max_val.max(1.0);
    let radius = 4;
    let scale_x = width as f64 / n as f64;
    let scale_y = height as f64 / max_val;
    
    let mut svg = String::with_capacity(n * 100 + 256);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\"><defs><style>.s{fill-opacity:0.8}</style></defs>");
    
    let colors = [0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6, 0x06B6D4, 0xEC4899, 0x84CC16];
    
    for i in 0..n {
        let x = (i as f64 * scale_x + scale_x / 2.0) as i32;
        let y = height - (values[i] * scale_y) as i32;
        let hex = format!("{:06x}", colors[i % colors.len()]);
        
        svg.push_str("<circle cx=\"");
        svg.push_str(&x.to_string());
        svg.push_str("\" cy=\"");
        svg.push_str(&y.to_string());
        svg.push_str("\" r=\"");
        svg.push_str(&radius.to_string());
        svg.push_str("\" fill=\"#");
        svg.push_str(&hex);
        svg.push_str("\" data-index=\"");
        svg.push_str(&i.to_string());
        svg.push_str("\"/>");
    }
    
    svg.push_str("</svg>");
    svg
}

pub fn render_points(ctx: super::PlotRenderContext) {
    let visible_count = ctx.visible_indices.len();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        
        let pos = if ctx.vertical {
            let x = ctx.plot_rect.left() + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            let y = ctx.plot_rect.bottom() - norm_val as f32 * ctx.plot_rect.height();
            egui::pos2(x, y)
        } else {
            let x = ctx.plot_rect.left() + norm_val as f32 * ctx.plot_rect.width();
            let y = ctx.plot_rect.top() + (ctx.plot_rect.height() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
            egui::pos2(x, y)
        };
        
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let (radius, display_color) = if is_hovered { 
            (6.0, egui::Color32::from_rgb(255, 200, 0)) 
        } else { 
            (4.0, color) 
        };
        ctx.painter.circle_filled(pos, radius, display_color);
    }
}

pub fn render_svg_scatter(
    svg: &mut String,
    values: &[f64],
    colors: &[&'static str],
    pad: i32,
    plot_width: i32,
    plot_height: i32,
    max_val: f64,
    vertical: bool,
) {
    let visible_count = values.len();
    let step = ((visible_count as f64) / (plot_width as f64)).max(1.0) as usize;

    for (vis_idx, &val) in values.iter().enumerate().step_by(step.max(1)) {
        let norm_val = val / max_val.max(1.0);
        let (x, y) = if vertical {
            let x_pos = pad as f64 + (plot_width as f64 / (visible_count as f64 - 1.0).max(1.0)) * vis_idx as f64;
            let y_pos = pad + plot_height - (norm_val * plot_height as f64) as i32;
            (x_pos as i32, y_pos)
        } else {
            let x_pos = pad as i32 + (norm_val * plot_width as f64) as i32;
            let y_pos = pad as f64 + (plot_height as f64 / (visible_count as f64 - 1.0).max(1.0)) * vis_idx as f64;
            (x_pos, y_pos as i32)
        };
        let color = colors[vis_idx % colors.len()];

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\" class=\"interactive-point\" data-index=\"{}\"/>",
            x, y, color, vis_idx
        ));
    }
}

pub fn render_scatter_html(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
    hover: &[crate::html::hover::HoverSlot],
    sizes: &[f64],
    color_groups: &[String],
    palette: &[u32],
    x_label: &str,
    y_label: &str,
    color_hex: u32,
    gridlines: bool,
    show_text: bool,
) -> String {
    use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};
    use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color};
    let n = x_values.len().min(y_values.len());
    if n == 0 { return String::new(); }
    let (_, max_x) = crate::bindings::utils::simd_ops::find_minmax(x_values);
    let (_, max_y) = crate::bindings::utils::simd_ops::find_minmax(y_values);
    let min_x = x_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let min_y = y_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);
    let pad_l = 56i32; let pad_t = 36i32; let pad_b = 48i32; let pad_r = 20i32;
    let has_groups = !color_groups.is_empty();
    let has_sizes = !sizes.is_empty();
    let (legend_w, group_names, group_map) = if has_groups {
        let mut names: Vec<String> = Vec::new();
        let mut map: Vec<usize> = Vec::with_capacity(n);
        for g in color_groups.iter().take(n) {
            let idx = names.iter().position(|x| x == g).unwrap_or_else(|| { names.push(g.clone()); names.len() - 1 });
            map.push(idx);
        }
        (160i32, names, map)
    } else {
        (0i32, Vec::new(), Vec::new())
    };
    let pad_r_actual = pad_r + legend_w;
    let plot_w = width - pad_l - pad_r_actual;
    let plot_h = height - pad_t - pad_b;
    let (size_min, size_max) = if has_sizes {
        let mn = sizes.iter().cloned().fold(f64::INFINITY, f64::min);
        let mx = sizes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (mn, (mx - mn).max(1.0))
    } else { (0.0, 1.0) };
    let mut buf = Vec::<u8>::with_capacity(n * 200 + 4096);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, width); push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, height); push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, width); push_b(&mut buf, b" ");
    push_i(&mut buf, height); push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, width / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, title);
        push_b(&mut buf, b"</text>");
    }
    for i in 0..=5 {
        let frac = i as f64 / 5.0;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let val = min_y + frac * range_y;
        if gridlines && i > 0 {
            push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
            push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
            push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\"/>");
        }
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l - 4);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        push_f2(&mut buf, val);
        push_b(&mut buf, b"</text>");
        let xval = min_x + frac * range_x;
        let xi = pad_l + (frac * plot_w as f64) as i32;
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, xi);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 14);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        push_f2(&mut buf, xval);
        push_b(&mut buf, b"</text>");
    }
    push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
    push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
    push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
    if !y_label.is_empty() {
        push_b(&mut buf, b"<text x=\"14\" y=\""); push_i(&mut buf, pad_t + plot_h / 2);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\" transform=\"rotate(-90,14,");
        push_i(&mut buf, pad_t + plot_h / 2);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, y_label);
        push_b(&mut buf, b"</text>");
    }
    if !x_label.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l + plot_w / 2);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, height - 4);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, x_label);
        push_b(&mut buf, b"</text>");
    }
    for i in 0..n {
        let cx = pad_l + ((x_values[i] - min_x) / range_x * plot_w as f64) as i32;
        let cy = pad_t + plot_h - ((y_values[i] - min_y) / range_y * plot_h as f64) as i32;
        let color = if has_groups && i < group_map.len() {
            palette_color(palette, group_map[i])
        } else if color_hex != 0 {
            color_hex
        } else {
            palette_color(palette, i)
        };
        let hx = hex6(color);
        let r = if has_sizes && i < sizes.len() {
            ((sizes[i] - size_min) / size_max * 18.0 + 3.0) as i32
        } else { 5 };
        push_b(&mut buf, b"<circle data-idx=\""); push_i(&mut buf, i as i32);
        if has_groups { push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, group_map[i] as i32); }
        push_b(&mut buf, b"\" data-kv-X=\""); push_f2(&mut buf, x_values[i]);
        push_b(&mut buf, b"\" data-kv-Y=\""); push_f2(&mut buf, y_values[i]);
        if i < labels.len() && !labels[i].is_empty() {
            push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &labels[i]);
        }
        push_b(&mut buf, b"\" cx=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, cy);
        push_b(&mut buf, b"\" r=\""); push_i(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.75\" stroke=\"#fff\" stroke-width=\"1\"/>");
        if show_text && i < labels.len() && !labels[i].is_empty() {
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, cx);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, cy - r - 3);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#374151\">");
            escape_xml(&mut buf, if labels[i].len() <= 14 { &labels[i] } else { &labels[i][..14] });
            push_b(&mut buf, b"</text>");
        }
    }
    if has_groups {
        let leg_x = pad_l + plot_w + 12;
        let leg_top = pad_t + 8;
        for (gi, name) in group_names.iter().enumerate() {
            let hx = hex6(palette_color(palette, gi));
            let ly = leg_top + gi as i32 * 22;
            push_b(&mut buf, b"<g data-legend=\"1\" data-series=\"");
            push_i(&mut buf, gi as i32);
            push_b(&mut buf, b"\">");
            push_b(&mut buf, b"<circle cx=\""); push_i(&mut buf, leg_x + 6);
            push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, ly + 6);
            push_b(&mut buf, b"\" r=\"6\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, leg_x + 17);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, ly + 11);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
            escape_xml(&mut buf, if name.len() <= 20 { name } else { &name[..20] });
            push_b(&mut buf, b"</text></g>");
        }
    }
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let hover_json = if hover.is_empty() {
        "[]".to_string()
    } else {
        slots_to_json(hover)
    };
    build_chart_html(title, &svg, &hover_json)
}

#[inline] fn scat_xml_esc(s: &str) -> String { s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;") }
