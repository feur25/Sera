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
