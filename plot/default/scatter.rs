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
