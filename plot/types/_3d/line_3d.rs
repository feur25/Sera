use super::super::super::containers_3d::{CameraController, Cube3DContainer};
use super::super::super::camera::Point3D;

pub struct Line3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub camera_controller: &'a CameraController,
}

pub fn render_lines_3d(ctx: Line3DRenderContext) {
    if ctx.visible_indices.len() < 2 {
        return;
    }
    
    let visible_count = ctx.visible_indices.len();
    let max_val = ctx.max_val.max(1.0);
    let center = ctx.plot_rect.center();
    
    let container_size = 10.0;
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), container_size);
    
    let mut line_points: Vec<(egui::Pos2, egui::Color32, usize, f32)> = Vec::new();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let u = vis_idx as f32 / (visible_count as f32).max(1.0);
        let v = 0.5;
        let w = 0.3 + norm_val * 0.4;
        
        let point_3d = cube.point_normalized(u, v, w);
        
        if let Some(proj) = ctx.camera_controller.camera.project(point_3d) {
            let scale = 0.5 / (point_3d.x.abs() + point_3d.y.abs() + point_3d.z.abs() + 1.0).max(1.0);
            let screen_x = center.x + proj.x * ctx.plot_rect.width() * 0.2 * scale;
            let screen_y = center.y + proj.y * ctx.plot_rect.height() * 0.2 * scale;
            let screen = egui::pos2(screen_x, screen_y);
            
            let color = ctx.colors[actual_idx % ctx.colors.len()];
            let depth = point_3d.x + point_3d.y + point_3d.z;
            line_points.push((screen, color, actual_idx, depth));
        }
    }
    
    for i in 0..line_points.len() {
        let (screen, color, actual_idx, _) = line_points[i];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        let radius = if is_hovered { 6.0 } else { 4.0 };
        ctx.painter.circle_filled(screen, radius, display_color);
        
        if i > 0 {
            let (prev_screen, _, _, _) = line_points[i - 1];
            let line_color = if is_hovered {
                egui::Color32::from_rgb(255, 200, 0)
            } else {
                color
            };
            ctx.painter.line_segment([prev_screen, screen], egui::Stroke::new(2.0, line_color));
        }
    }
}
