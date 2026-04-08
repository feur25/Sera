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
    
    let colors = [0x1f77b4, 0xff7f0e, 0x2ca02c, 0xd62728, 0x9467bd, 0x8c564b, 0xe377c2, 0x7f7f7f];
    
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
) -> String {
    use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};
    use std::fmt::Write as FmtWrite;
    let n = x_values.len().min(y_values.len());
    if n == 0 { return String::new(); }
    let (_, max_x) = crate::bindings::utils::simd_ops::find_minmax(x_values);
    let (_, max_y) = crate::bindings::utils::simd_ops::find_minmax(y_values);
    let min_x = x_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let min_y = y_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);
    let pad_l = 56i32; let pad_t = 36i32; let pad_b = 48i32; let pad_r = 20i32;
    let plot_w = width - pad_l - pad_r;
    let plot_h = height - pad_t - pad_b;
    const PALETTE: &[u32] = &[0x4C72B0, 0xDD8452, 0x55A868, 0xC44E52, 0x8172B3, 0x64B5CD, 0xDA8BC3, 0xCCB974, 0x937860, 0x8C8C8C];
    let use_lazy = hover.is_empty() && labels.is_empty();
    let mut auto_slots: Vec<HoverSlot> = if !use_lazy && hover.is_empty() { Vec::with_capacity(n) } else { Vec::new() };
    let mut buf = String::with_capacity(n * 160 + 2048);
    let _ = write!(buf, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">",
        width, height, width, height);
    buf.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
    if !title.is_empty() {
        let _ = write!(buf, "<text x=\"{}\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\">{}</text>",
            width / 2, scat_xml_esc(title));
    }
    for i in 0..=5 {
        let frac = i as f64 / 5.0;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let val = min_y + frac * range_y;
        let x2 = pad_l + plot_w;
        if i > 0 {
            let _ = write!(buf, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"3,3\"/>",
                pad_l, y, x2, y);
        }
        let lbl = if val.abs() >= 1000.0 { format!("{:.0}", val) } else if val.abs() >= 1.0 { format!("{:.1}", val) } else { format!("{:.2}", val) };
        let _ = write!(buf, "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">{}</text>",
            pad_l - 4, y + 3, lbl);
        let xval = min_x + frac * range_x;
        let xi = pad_l + (frac * plot_w as f64) as i32;
        let xlbl = if xval.abs() >= 1000.0 { format!("{:.0}", xval) } else if xval.abs() >= 1.0 { format!("{:.1}", xval) } else { format!("{:.2}", xval) };
        let _ = write!(buf, "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">{}</text>",
            xi, pad_t + plot_h + 14, xlbl);
    }
    let _ = write!(buf, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>",
        pad_l, pad_t, pad_l, pad_t + plot_h);
    let _ = write!(buf, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>",
        pad_l, pad_t + plot_h, pad_l + plot_w, pad_t + plot_h);
    for i in 0..n {
        let cx = pad_l + ((x_values[i] - min_x) / range_x * plot_w as f64) as i32;
        let cy = pad_t + plot_h - ((y_values[i] - min_y) / range_y * plot_h as f64) as i32;
        let color = PALETTE[i % PALETTE.len()];
        let lbl = if i < labels.len() { labels[i].as_str() } else { "" };
        let _ = write!(buf,
            "<circle data-idx=\"{}\" data-x=\"{:.2}\" data-y=\"{:.2}\"",
            i, x_values[i], y_values[i]);
        if !lbl.is_empty() {
            buf.push_str(" data-lbl=\"");
            buf.push_str(&scat_xml_esc(lbl));
            buf.push('"');
        }
        let h = b"0123456789abcdef";
        let hex = [
            h[((color >> 20) & 0xf) as usize], h[((color >> 16) & 0xf) as usize],
            h[((color >> 12) & 0xf) as usize], h[((color >> 8)  & 0xf) as usize],
            h[((color >> 4)  & 0xf) as usize], h[(color & 0xf) as usize],
        ];
        let _ = write!(buf,
            " cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"#{}{}{}{}{}{}\" fill-opacity=\"0.8\" stroke=\"#fff\" stroke-width=\"1\"/>",
            cx, cy, hex[0] as char, hex[1] as char, hex[2] as char,
            hex[3] as char, hex[4] as char, hex[5] as char);
        if !use_lazy && hover.is_empty() {
            auto_slots.push(HoverSlot::new(if lbl.is_empty() { format!("Point {}", i + 1) } else { lbl.to_string() })
                .kv("X", format!("{:.2}", x_values[i])).kv("Y", format!("{:.2}", y_values[i])));
        }
    }
    buf.push_str("</svg>");
    let hover_json = if use_lazy {
        "[]".to_string()
    } else if hover.is_empty() {
        slots_to_json(&auto_slots)
    } else {
        slots_to_json(hover)
    };
    build_chart_html(title, &buf, &hover_json)
}

#[inline] fn scat_xml_esc(s: &str) -> String { s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;") }
