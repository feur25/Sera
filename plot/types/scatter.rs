pub struct Scatter;

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
) {
    let visible_count = values.len();
    let step = ((visible_count as f64) / (plot_width as f64)).max(1.0) as usize;

    for (vis_idx, &val) in values.iter().enumerate().step_by(step.max(1)) {
        let norm_val = val / max_val.max(1.0);
        let x_pos = pad as f64 + (plot_width as f64 / (visible_count as f64 - 1.0).max(1.0)) * vis_idx as f64;
        let y = pad + plot_height - (norm_val * plot_height as f64) as i32;
        let color = colors[vis_idx % colors.len()];

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\" class=\"interactive-point\" data-index=\"{}\"/>",
            x_pos as i32, y, color, vis_idx
        ));
    }
}
