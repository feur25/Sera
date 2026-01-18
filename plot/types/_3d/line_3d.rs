use super::super::super::generic::*;
use super::super::super::camera::{Camera3D, Point3D};

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
    
    let norm_height = ctx.plot_rect.height();
    let center_x = ctx.plot_rect.center().x;
    let center_y = ctx.plot_rect.center().y;
    
    let points: Vec<_> = ctx.visible_indices.iter().enumerate()
        .map(|(vis_idx, &actual_idx)| {
            let value = ctx.values[actual_idx];
            let norm_val = value / ctx.max_val.max(1.0);
            let height = norm_val * norm_height as f64;
            
            let p_front = Point3D::new(vis_idx as f32, 0.5, height as f32);
            let p_back = Point3D::new(vis_idx as f32, 0.8, height as f32);
            
            let proj_front = ctx.camera.project(p_front);
            let proj_back = ctx.camera.project(p_back);
            
            let screen_front = egui::pos2(center_x + proj_front.x, center_y - proj_front.y);
            let screen_back = egui::pos2(center_x + proj_back.x, center_y - proj_back.y);
            
            let color = ctx.colors[actual_idx % ctx.colors.len()];
            let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
            let display_color = if is_hovered {
                egui::Color32::from_rgb(255, 200, 0)
            } else {
                color
            };
            
            (screen_front, screen_back, display_color, actual_idx)
        })
        .collect();
    
    for i in 0..points.len() {
        let (front, back, color, _) = points[i];
        
        ctx.painter.circle_filled(front, 4.0, color);
        ctx.painter.circle_filled(back, 3.5, shade_color(color, 0.8));
        ctx.painter.line_segment([front, back], egui::Stroke::new(1.5, shade_color(color, 0.6)));
        
        if i > 0 {
            let (prev_front, _, _, _) = points[i - 1];
            ctx.painter.line_segment([prev_front, front], egui::Stroke::new(2.0, color));
        }
    }
}

fn shade_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    egui::Color32::from_rgb(
        (color.r() as f32 * factor) as u8,
        (color.g() as f32 * factor) as u8,
        (color.b() as f32 * factor) as u8,
    )
}
