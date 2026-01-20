pub struct Line;

pub fn render_lines(ctx: super::PlotRenderContext) {
    if ctx.visible_indices.len() < 2 {
        return;
    }
    
    let visible_count = ctx.visible_indices.len();
    let mut prev_pos: Option<egui::Pos2> = None;
    
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
        if let Some(p) = prev_pos {
            ctx.painter.line_segment([p, pos], egui::Stroke::new(2.0, color));
        }
        
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let (radius, display_color) = if is_hovered { 
            (6.0, egui::Color32::from_rgb(255, 200, 0)) 
        } else { 
            (4.0, color) 
        };
        ctx.painter.circle_filled(pos, radius, display_color);
        prev_pos = Some(pos);
    }
}

pub fn render_svg_lines(
    svg: &mut String,
    values: &[f64],
    colors: &[&'static str],
    pad: i32,
    plot_width: i32,
    plot_height: i32,
    max_val: f64,
) {
    if values.len() < 2 {
        return;
    }

    let visible_count = values.len();
    let mut prev_pos: Option<(i32, i32)> = None;

    for (vis_idx, &val) in values.iter().enumerate() {
        let norm_val = val / max_val.max(1.0);
        let x_pos = pad as f64 + (plot_width as f64 / (visible_count - 1) as f64) * vis_idx as f64;
        let y = pad + plot_height - (norm_val * plot_height as f64) as i32;
        let color = colors[vis_idx % colors.len()];

        if let Some((px, py)) = prev_pos {
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" class=\"interactive-line\"/>",
                px, py, x_pos as i32, y, color
            ));
        }

        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\" class=\"interactive-point\" data-index=\"{}\"/>",
            x_pos as i32, y, color, vis_idx
        ));

        prev_pos = Some((x_pos as i32, y));
    }
}
