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
    
    let colors = [0x1f77b4, 0xff7f0e, 0x2ca02c, 0xd62728, 0x9467bd, 0x8c564b, 0xe377c2, 0x7f7f7f];
    
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
