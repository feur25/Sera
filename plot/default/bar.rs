pub struct Bar;

pub fn render_bars_fast(
    values: &[f64],
    labels: &[String],
    width: i32,
    height: i32,
) -> String {
    let n = values.len().min(labels.len());
    if n == 0 { return String::new(); }
    
    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
    let max_val = max_val.max(1.0);
    let bar_width = width as f64 / n as f64;
    let scale = height as f64 / max_val;
    
    let mut svg = String::with_capacity(n * 120 + 256);
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    svg.push_str(&width.to_string());
    svg.push_str("\" height=\"");
    svg.push_str(&height.to_string());
    svg.push_str("\" viewBox=\"0 0 ");
    svg.push_str(&width.to_string());
    svg.push(' ');
    svg.push_str(&height.to_string());
    svg.push_str("\"><defs><style>.b{font:10px sans-serif}.v{fill:#1f77b4}.l{fill:#666;font-size:9px}</style></defs>");
    
    let colors = [0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6, 0x06B6D4, 0xEC4899, 0x84CC16];
    
    for i in 0..n {
        let bar_height = (values[i] * scale) as i32;
        let x = (i as f64 * bar_width) as i32;
        let y = height - bar_height;
        let w = bar_width.max(1.0) as i32;
        let color = colors[i % colors.len()];
        
        svg.push_str("<rect x=\"");
        svg.push_str(&x.to_string());
        svg.push_str("\" y=\"");
        svg.push_str(&y.to_string());
        svg.push_str("\" width=\"");
        svg.push_str(&w.to_string());
        svg.push_str("\" height=\"");
        svg.push_str(&bar_height.to_string());
        svg.push_str("\" fill=\"#");
        svg.push_str(&format!("{:06x}", color));
        svg.push_str("\" data-index=\"");
        svg.push_str(&i.to_string());
        svg.push_str("\"/>");
    }
    
    svg.push_str("</svg>");
    svg
}

pub fn render_bars(ctx: super::PlotRenderContext) {
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
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        
        let (bar_length, bar_thickness) = if ctx.vertical {
            let width = (ctx.plot_rect.width() / visible_count as f32) * 0.6;
            let height = ctx.plot_rect.bottom() - pos.y;
            (height, width)
        } else {
            let length = norm_val as f32 * ctx.plot_rect.width();
            let thickness = (ctx.plot_rect.height() / visible_count as f32) * 0.6;
            (length, thickness)
        };
        
        let rect = if ctx.vertical {
            egui::Rect::from_min_size(
                egui::pos2(pos.x - bar_thickness / 2.0, pos.y),
                egui::vec2(bar_thickness, bar_length),
            )
        } else {
            egui::Rect::from_min_size(
                egui::pos2(ctx.plot_rect.left(), pos.y - bar_thickness / 2.0),
                egui::vec2(bar_length, bar_thickness),
            )
        };
        
        let display_color = if is_hovered { 
            egui::Color32::from_rgb(255, 200, 0) 
        } else { 
            color 
        };
        ctx.painter.rect_filled(rect, 0.0, display_color);
        
        let label_text = if actual_idx < ctx.labels.len() {
            &ctx.labels[actual_idx]
        } else {
            ""
        };
        
        if ctx.vertical {
            ctx.painter.text(
                egui::pos2(pos.x, ctx.plot_rect.bottom() + 8.0),
                egui::Align2::CENTER_TOP,
                label_text,
                egui::FontId::proportional(10.0),
                egui::Color32::from_gray(100),
            );
        } else {
            ctx.painter.text(
                egui::pos2(ctx.plot_rect.left() - 8.0, pos.y),
                egui::Align2::RIGHT_CENTER,
                label_text,
                egui::FontId::proportional(10.0),
                egui::Color32::from_gray(100),
            );
        }
    }
}

pub fn render_svg_bars(
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

    for (vis_idx, &val) in values.iter().enumerate() {
        let norm_val = val / max_val.max(1.0);
        let color = colors[vis_idx % colors.len()];

        if vertical {
            let bar_spacing = plot_width as f64 / visible_count as f64;
            let x = pad as f64 + bar_spacing * vis_idx as f64;
            let bar_width = bar_spacing * 0.6;
            let bar_height = (norm_val * plot_height as f64) as i32;

            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"#ccc\" stroke-width=\"0.5\" class=\"interactive-bar\" data-index=\"{}\"/>",
                (x - bar_width / 2.0) as i32, pad + plot_height - bar_height, bar_width as i32, bar_height, color, vis_idx
            ));
        } else {
            let bar_spacing = plot_height as f64 / visible_count as f64;
            let y = pad as f64 + bar_spacing * vis_idx as f64;
            let bar_length = (norm_val * plot_width as f64) as i32;
            let bar_thickness = bar_spacing * 0.6;

            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"#ccc\" stroke-width=\"0.5\" class=\"interactive-bar\" data-index=\"{}\"/>",
                pad, (y - bar_thickness / 2.0) as i32, bar_length, bar_thickness as i32, color, vis_idx
            ));
        }
    }
}

pub fn render_bars_html(
    title: &str,
    labels: &[String],
    values: &[f64],
    width: i32,
    height: i32,
    hover: &[crate::html::hover::HoverSlot],
    orientation: u8,
    color_groups: &[String],
    show_text: bool,
    x_label: &str,
    y_label: &str,
    palette: &[u32],
    color_hex: u32,
    gridlines: bool,
    sort_order: &str,
) -> String {
    use crate::html::hover::{slots_to_json, html_id, html_prefix, html_suffix};
    use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color, truncate, PALETTE as DEFAULT_PAL, apply_sort, apply_sort_groups, svg_open_rescalable, svg_title, svg_axis_lines, svg_x_label, svg_y_label, svg_hgrid, svg_vgrid, svg_tick_y, svg_tick_x, svg_legend_item};
    let n = values.len().min(labels.len());
    if n == 0 { return String::new(); }
    let has_groups = !color_groups.is_empty() && color_groups.len() >= n;
    let (labels, values, color_groups_sorted) = if has_groups {
        let (sl, sv, sg) = apply_sort_groups(labels, values, color_groups, sort_order);
        (sl, sv, sg)
    } else {
        let (sl, sv) = apply_sort(labels, values, sort_order);
        (sl, sv, Vec::new())
    };
    let cg_ref: &[String] = if has_groups { &color_groups_sorted } else { &[] };
    let (_, max_val) = crate::bindings::utils::simd_ops::find_minmax(&values);
    let max_val = max_val.max(1.0);
    let horizontal = orientation == b'h';
    let pal = if !palette.is_empty() { palette } else { DEFAULT_PAL };
    let single_color = color_hex != 0;

    let mut group_names: Vec<String> = Vec::new();
    if has_groups {
        for g in &cg_ref[..n] {
            if !group_names.contains(g) { group_names.push(g.clone()); }
        }
    }
    let legend_w: i32 = if has_groups { 148 } else { 20 };

    let pad_l: i32 = if horizontal { 132 } else if !y_label.is_empty() { 68 } else { 52 };
    let pad_t: i32 = 36;
    let pad_b: i32 = if !x_label.is_empty() { 58 } else { 48 };
    let pad_r: i32 = legend_w;
    let plot_w = width - pad_l - pad_r;
    let plot_h = height - pad_t - pad_b;

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 280 + 16_000);
    html_prefix(&mut buf, title, hid);
    svg_open_rescalable(&mut buf, width, height, pad_l, pad_t, plot_w, plot_h);
    svg_title(&mut buf, title, pad_l + plot_w / 2, 24);

    if horizontal {
        let n_xticks = 5i32;
        for ti in 0..=n_xticks {
            let frac = ti as f64 / n_xticks as f64;
            let tx = pad_l + (frac * plot_w as f64) as i32;
            let val = frac * max_val;
            if gridlines { svg_vgrid(&mut buf, tx, pad_t, pad_t + plot_h); }
            svg_tick_x(&mut buf, tx, pad_t + plot_h + 14, val);
        }
    } else {
        for i in 0..=5 {
            let frac = i as f64 / 5.0;
            let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
            let val = frac * max_val;
            if gridlines && i > 0 { svg_hgrid(&mut buf, pad_l, pad_l + plot_w, y); }
            svg_tick_y(&mut buf, pad_l - 4, y + 3, val);
        }
    }

    svg_axis_lines(&mut buf, pad_l, pad_t, plot_w, plot_h);
    svg_x_label(&mut buf, x_label, pad_l + plot_w / 2, pad_t + plot_h + if horizontal { 38 } else { 42 });
    svg_y_label(&mut buf, y_label, 14, pad_t, plot_h);

    if horizontal {
        let pitch = plot_h as f64 / n as f64;
        let bar_h = (pitch * 0.62) as i32;

        for i in 0..n {
            let bar_y = pad_t + (i as f64 * pitch + (pitch - bar_h as f64) / 2.0) as i32;
            let bar_w = ((values[i] / max_val) * plot_w as f64) as i32;

            let color = if single_color { color_hex }
                else if has_groups { let gi = group_names.iter().position(|g| g == &cg_ref[i]).unwrap_or(i % pal.len()); palette_color(pal, gi) }
                else { palette_color(pal, i) };
            let hx = hex6(color);

            let series_idx = if has_groups { group_names.iter().position(|g| g == &cg_ref[i]).unwrap_or(i) } else { i };

            push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, series_idx as i32);
            push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, values[i]);
            push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &labels[i]);
            push_b(&mut buf, b"\" x=\""); push_i(&mut buf, pad_l);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, bar_y);
            push_b(&mut buf, b"\" width=\""); push_i(&mut buf, bar_w.max(1));
            push_b(&mut buf, b"\" height=\""); push_i(&mut buf, bar_h);
            push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" rx=\"2\"/>");

            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l - 5);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, bar_y + bar_h / 2 + 4);
            push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            escape_xml(&mut buf, truncate(&labels[i], 18));
            push_b(&mut buf, b"</text>");

            if show_text && bar_w > 8 {
                push_b(&mut buf, b"<text data-series=\""); push_i(&mut buf, series_idx as i32);
                push_b(&mut buf, b"\" x=\""); push_i(&mut buf, pad_l + bar_w + 4);
                push_b(&mut buf, b"\" y=\""); push_i(&mut buf, bar_y + bar_h / 2 + 4);
                push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
                if values[i] >= 1000.0 {
                    let k = values[i] / 1000.0;
                    if k >= 10.0 { push_i(&mut buf, k as i32); push_b(&mut buf, b"k"); }
                    else { push_f2(&mut buf, k); push_b(&mut buf, b"k"); }
                } else {
                    push_f2(&mut buf, values[i]);
                }
                push_b(&mut buf, b"</text>");
            }
        }
    } else {
        let bar_w_f = plot_w as f64 / n as f64;

        for i in 0..n {
            let bh = ((values[i] / max_val) * plot_h as f64) as i32;
            let x = pad_l + (i as f64 * bar_w_f) as i32;
            let y = pad_t + plot_h - bh;
            let w = (bar_w_f as i32).max(2) - 1;

            let color = if single_color { color_hex }
                else if has_groups { let gi = group_names.iter().position(|g| g == &cg_ref[i]).unwrap_or(i % pal.len()); palette_color(pal, gi) }
                else { palette_color(pal, i) };
            let hx = hex6(color);
            let series_idx = if has_groups { group_names.iter().position(|g| g == &cg_ref[i]).unwrap_or(i) } else { i };

            push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" data-series=\""); push_i(&mut buf, series_idx as i32);
            push_b(&mut buf, b"\" data-v=\""); push_f2(&mut buf, values[i]);
            push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, &labels[i]);
            push_b(&mut buf, b"\" x=\""); push_i(&mut buf, x);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y);
            push_b(&mut buf, b"\" width=\""); push_i(&mut buf, w);
            push_b(&mut buf, b"\" height=\""); push_i(&mut buf, bh);
            push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" rx=\"2\"/>");

            let trunc_lbl = truncate(&labels[i], 12);
            push_b(&mut buf, b"<text x=\""); push_i(&mut buf, x + w / 2);
            push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 14);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
            escape_xml(&mut buf, trunc_lbl);
            push_b(&mut buf, b"</text>");

            if show_text && bh > 14 {
                push_b(&mut buf, b"<text data-series=\""); push_i(&mut buf, series_idx as i32);
                push_b(&mut buf, b"\" x=\""); push_i(&mut buf, x + w / 2);
                push_b(&mut buf, b"\" y=\""); push_i(&mut buf, y - 4);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
                if values[i] >= 1000.0 {
                    let k = values[i] / 1000.0;
                    if k >= 10.0 { push_i(&mut buf, k as i32); push_b(&mut buf, b"k"); }
                    else { push_f2(&mut buf, k); push_b(&mut buf, b"k"); }
                } else {
                    push_f2(&mut buf, values[i]);
                }
                push_b(&mut buf, b"</text>");
            }
        }
    }

    if has_groups {
        let leg_x = width - legend_w + 12;
        for (gi, gname) in group_names.iter().enumerate() {
            let ly = pad_t + 8 + gi as i32 * 18;
            svg_legend_item(&mut buf, gi as i32, gname, palette_color(pal, gi), leg_x, ly, 14);
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
