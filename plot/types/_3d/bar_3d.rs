use super::super::super::generic::*;
use super::super::super::camera::Camera3D;

pub struct Bar3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
    pub camera: &'a Camera3D,
}

pub fn render_bars_3d(ctx: Bar3DRenderContext) {
    let visible_count = ctx.visible_indices.len();
    let depth_scale = ctx.camera.depth_scale;
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        
        let x = ctx.plot_rect.left() + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
        let bar_width = (ctx.plot_rect.width() / visible_count as f32) * 0.5;
        let bar_height = norm_val as f32 * ctx.plot_rect.height();
        let bar_depth = bar_width * depth_scale;
        
        let top_y = ctx.plot_rect.bottom() - bar_height;
        let base_y = ctx.plot_rect.bottom();
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        let front_left = egui::pos2(x - bar_width / 2.0, top_y);
        let front_right = egui::pos2(x + bar_width / 2.0, top_y);
        let front_bottom_left = egui::pos2(x - bar_width / 2.0, base_y);
        let front_bottom_right = egui::pos2(x + bar_width / 2.0, base_y);
        
        let (back_offset_x, back_offset_y) = ctx.camera.back_offset(bar_depth);
        
        let back_left = egui::pos2(front_left.x + back_offset_x, front_left.y + back_offset_y);
        let back_right = egui::pos2(front_right.x + back_offset_x, front_right.y + back_offset_y);
        let back_bottom_left = egui::pos2(front_bottom_left.x + back_offset_x, front_bottom_left.y + back_offset_y);
        let back_bottom_right = egui::pos2(front_bottom_right.x + back_offset_x, front_bottom_right.y + back_offset_y);
        
        let darker_color = egui::Color32::from_rgb(
            (display_color.r() as f32 * 0.7) as u8,
            (display_color.g() as f32 * 0.7) as u8,
            (display_color.b() as f32 * 0.7) as u8,
        );
        
        let darkest_color = egui::Color32::from_rgb(
            (display_color.r() as f32 * 0.5) as u8,
            (display_color.g() as f32 * 0.5) as u8,
            (display_color.b() as f32 * 0.5) as u8,
        );
        
        ctx.painter.rect_filled(
            egui::Rect::from_two_pos(front_left, front_bottom_right),
            0.0,
            display_color,
        );
        
        let top_face = [front_left, front_right, back_right, back_left];
        ctx.painter.rect_filled(
            egui::Rect::from_two_pos(top_face[0], back_right),
            0.0,
            darker_color,
        );
        
        ctx.painter.line_segment([front_left, back_left], egui::Stroke::new(1.0, darkest_color));
        ctx.painter.line_segment([front_right, back_right], egui::Stroke::new(1.0, darkest_color));
        ctx.painter.line_segment([front_bottom_left, back_bottom_left], egui::Stroke::new(1.0, darkest_color));
        ctx.painter.line_segment([front_bottom_right, back_bottom_right], egui::Stroke::new(1.0, darkest_color));
        
        ctx.painter.line_segment([back_left, back_right], egui::Stroke::new(1.0, darkest_color));
        ctx.painter.line_segment([back_left, back_bottom_left], egui::Stroke::new(1.0, darkest_color));
        ctx.painter.line_segment([back_right, back_bottom_right], egui::Stroke::new(1.0, darkest_color));
    }
}
