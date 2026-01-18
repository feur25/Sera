use super::super::super::generic::*;
use super::super::super::camera::Camera3D;

pub struct Line3DRenderContext<'a> {
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

pub fn render_lines_3d(ctx: Line3DRenderContext) {
    if ctx.visible_indices.len() < 2 {
        return;
    }
    
    let visible_count = ctx.visible_indices.len();
    let depth = ctx.camera.depth_scale;
    
    let mut prev_pos: Option<egui::Pos2> = None;
    let mut prev_back_pos: Option<egui::Pos2> = None;
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        
        let x = ctx.plot_rect.left() + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
        let y = ctx.plot_rect.bottom() - norm_val as f32 * ctx.plot_rect.height();
        
        let pos = egui::pos2(x, y);
        let (offset_x, offset_y) = ctx.camera.back_offset(depth * ctx.plot_rect.width());
        let back_x = x + offset_x;
        let back_y = y + offset_y;
        let back_pos = egui::pos2(back_x, back_y);
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        let darker_color = egui::Color32::from_rgb(
            (color.r() as f32 * 0.6) as u8,
            (color.g() as f32 * 0.6) as u8,
            (color.b() as f32 * 0.6) as u8,
        );
        
        if let Some(p) = prev_pos {
            let prev_back = prev_back_pos.unwrap();
            
            ctx.painter.line_segment([p, pos], egui::Stroke::new(2.5, display_color));
            ctx.painter.line_segment([prev_back, back_pos], egui::Stroke::new(2.5, darker_color));
            
            ctx.painter.line_segment([p, prev_back], egui::Stroke::new(1.5, darker_color));
        }
        
        ctx.painter.circle_filled(pos, 5.0, display_color);
        ctx.painter.circle_filled(back_pos, 4.0, darker_color);
        ctx.painter.line_segment([pos, back_pos], egui::Stroke::new(1.0, darker_color));
        
        prev_pos = Some(pos);
        prev_back_pos = Some(back_pos);
    }
}
