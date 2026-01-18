use super::super::super::containers_3d::{CameraController, Cube3DContainer};
use super::super::super::camera::Point3D;
use super::scale_renderer::render_scale_labels;

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
    let half_width = ctx.plot_rect.width() / 2.0;
    let half_height = ctx.plot_rect.height() / 2.0;
    
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), 0.5);
    
    render_scale_labels(ctx.painter, ctx.plot_rect, max_val);
    
    let mut line_points: Vec<(egui::Pos2, egui::Color32, usize, f32)> = Vec::new();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let u = vis_idx as f32 / ((visible_count - 1).max(1) as f32);
        let v = 0.5;
        let w = norm_val;
        
        let point_3d = cube.point_normalized(u, v, w);
        
        if let Some(proj) = ctx.camera_controller.camera.project(point_3d) {
            let screen_x = center.x + proj.x * half_width;
            let screen_y = center.y - proj.y * half_height;
            let screen = egui::pos2(screen_x, screen_y);
            
            if screen_x.is_finite() && screen_y.is_finite() {
                let color = ctx.colors[actual_idx % ctx.colors.len()];
                let depth = point_3d.z;
                line_points.push((screen, color, actual_idx, depth));
            }
        }
    }
    
    line_points.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
    
    for i in 0..line_points.len() {
        let (screen, color, actual_idx, _) = line_points[i];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 220, 0)
        } else {
            color
        };
        
        if i > 0 {
            let (prev_screen, _, _, _) = line_points[i - 1];
            let line_width = if is_hovered { 4.5 } else { 3.5 };
            ctx.painter.line_segment([prev_screen, screen], egui::Stroke::new(line_width, display_color));
        }
        
        let radius = if is_hovered { 8.0 } else { 6.0 };
        ctx.painter.circle_filled(screen, radius, display_color);
        
        if is_hovered {
            ctx.painter.circle_stroke(screen, radius + 2.0, egui::Stroke::new(2.0, egui::Color32::WHITE));
        }
    }
}
