use super::super::super::containers_3d::{CameraController, Cube3DContainer};
use super::super::super::camera::Point3D;
use super::super::super::scale_renderer::render_scale_labels;
use super::super::super::containers_3d::render_3d_grid;

pub struct Bar3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub camera_controller: &'a CameraController,
}

pub fn render_bars_3d(ctx: Bar3DRenderContext) {
    let visible_count = ctx.visible_indices.len();
    let max_val = ctx.max_val.max(1.0);
    let center = ctx.plot_rect.center();
    
    let grid_size = (visible_count as f32).sqrt().ceil() as usize;
    
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), 0.5);
    
    render_3d_grid(ctx.painter, &cube, ctx.camera_controller, ctx.plot_rect);
    render_scale_labels(ctx.painter, ctx.plot_rect, max_val);
    
    let mut bars_to_render: Vec<(egui::Pos2, egui::Pos2, egui::Color32, usize, f32)> = Vec::new();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let x_idx = (vis_idx % grid_size) as f32;
        let y_idx = (vis_idx / grid_size) as f32;
        
        let u = x_idx / ((grid_size - 1).max(1) as f32);
        let v = y_idx / ((grid_size - 1).max(1) as f32);
        
        let base_3d = cube.point_normalized(u, v, 0.0);
        let top_3d = cube.point_normalized(u, v, norm_val);
        
        let half_width = ctx.plot_rect.width() / 2.0;
        let half_height = ctx.plot_rect.height() / 2.0;
        
        if let (Some(proj_base), Some(proj_top)) = (
            ctx.camera_controller.camera.project(base_3d),
            ctx.camera_controller.camera.project(top_3d),
        ) {
            let screen_base = egui::pos2(
                center.x + proj_base.x * half_width,
                center.y - proj_base.y * half_height,
            );
            let screen_top = egui::pos2(
                center.x + proj_top.x * half_width,
                center.y - proj_top.y * half_height,
            );
            
            if screen_base.x.is_finite() && screen_base.y.is_finite() && screen_top.x.is_finite() && screen_top.y.is_finite() {
                let color = ctx.colors[actual_idx % ctx.colors.len()];
                let depth = top_3d.z;
                bars_to_render.push((screen_base, screen_top, color, actual_idx, depth));
            }
        }
    }
    
    bars_to_render.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap_or(std::cmp::Ordering::Equal));
    
    for (screen_base, screen_top, color, actual_idx, _) in bars_to_render {
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 220, 0)
        } else {
            color
        };
        
        let stroke_width = if is_hovered { 8.0 } else { 5.5 };
        ctx.painter.line_segment([screen_base, screen_top], egui::Stroke::new(stroke_width, display_color));
        
        let base_radius = if is_hovered { 6.0 } else { 4.5 };
        let top_radius = if is_hovered { 7.5 } else { 5.5 };
        
        ctx.painter.circle_filled(screen_base, base_radius, display_color);
        ctx.painter.circle_filled(screen_top, top_radius, display_color);
        
        if is_hovered {
            ctx.painter.circle_stroke(
                screen_top,
                top_radius + 2.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 0)),
            );
            ctx.painter.circle_stroke(
                screen_base,
                base_radius + 2.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 0)),
            );
        }
    }
}

pub fn get_3d_positions(
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    let visible_count = visible_indices.len();
    if visible_count == 0 {
        return Vec::new();
    }
    
    let max_val = max_val.max(1.0);
    let grid_size = (visible_count as f32).sqrt().ceil() as usize;
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), 0.5);
    
    let center = plot_rect.center();
    let half_width = plot_rect.width() / 2.0;
    let half_height = plot_rect.height() / 2.0;
    
    let mut positions = Vec::new();
    
    for (vis_idx, &actual_idx) in visible_indices.iter().enumerate() {
        let value = values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let x_idx = (vis_idx % grid_size) as f32;
        let y_idx = (vis_idx / grid_size) as f32;
        
        let u = x_idx / ((grid_size - 1).max(1) as f32);
        let v = y_idx / ((grid_size - 1).max(1) as f32);
        
        let top_3d = cube.point_normalized(u, v, norm_val);
        
        if let Some(proj_top) = camera_controller.camera.project(top_3d) {
            let screen_x = center.x + proj_top.x * half_width;
            let screen_y = center.y - proj_top.y * half_height;
            positions.push((egui::pos2(screen_x, screen_y), actual_idx));
        }
    }
    
    positions
}