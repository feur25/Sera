use super::super::super::containers_3d::{CameraController, Cube3DContainer};
use super::super::super::camera::Point3D;
use super::scale_renderer::render_scale_labels;

pub struct Scatter3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub camera_controller: &'a CameraController,
}

pub fn render_points_3d(ctx: Scatter3DRenderContext) {
    let visible_count = ctx.visible_indices.len();
    if visible_count == 0 {
        return;
    }
    
    let max_val = ctx.max_val.max(1.0);
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), 0.5);
    
    render_grid_3d(&ctx.painter, &cube, ctx.camera_controller, ctx.plot_rect);
    render_scale_labels(&ctx.painter, ctx.plot_rect, max_val);
    
    let center = ctx.plot_rect.center();
    let half_width = ctx.plot_rect.width() / 2.0;
    let half_height = ctx.plot_rect.height() / 2.0;
    
    let mut to_render: Vec<(egui::Pos2, egui::Color32, usize, f32)> = Vec::new();
    let mut point_positions: Vec<(egui::Pos2, usize)> = Vec::new();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let cube_size = (visible_count as f32).cbrt().ceil() as usize;
        let x_idx = (vis_idx % (cube_size * cube_size)) % cube_size;
        let y_idx = (vis_idx % (cube_size * cube_size)) / cube_size;
        let z_idx = vis_idx / (cube_size * cube_size);
        
        let u = if cube_size > 1 { x_idx as f32 / (cube_size - 1) as f32 } else { 0.5 };
        let v = if cube_size > 1 { y_idx as f32 / (cube_size - 1) as f32 } else { 0.5 };
        let w = if cube_size > 1 { z_idx as f32 / (cube_size - 1) as f32 } else { norm_val };
        
        let point_3d = cube.point_normalized(u, v, w);
        
        if let Some(proj) = ctx.camera_controller.camera.project(point_3d) {
            let screen_x = center.x + proj.x * half_width;
            let screen_y = center.y - proj.y * half_height;
            let screen_pos = egui::pos2(screen_x, screen_y);
            
            point_positions.push((screen_pos, actual_idx));
            
            let color = ctx.colors[actual_idx % ctx.colors.len()];
            let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
            let display_color = if is_hovered {
                egui::Color32::from_rgb(255, 220, 0)
            } else {
                color
            };
            
            to_render.push((screen_pos, display_color, actual_idx, point_3d.z));
        }
    }
    
    to_render.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
    
    for (pos, color, _, _) in to_render {
        ctx.painter.circle_filled(pos, 9.0, color);
    }
}

pub fn render_3d_grid(
    painter: &egui::Painter,
    cube: &Cube3DContainer,
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) {
    render_grid_3d(painter, cube, camera_controller, plot_rect);
}

pub fn get_3d_point_positions(
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
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), 0.5);
    
    let center = plot_rect.center();
    let half_width = plot_rect.width() / 2.0;
    let half_height = plot_rect.height() / 2.0;
    
    let mut positions = Vec::new();
    
    for (vis_idx, &actual_idx) in visible_indices.iter().enumerate() {
        let value = values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let cube_size = (visible_count as f32).cbrt().ceil() as usize;
        let x_idx = (vis_idx % (cube_size * cube_size)) % cube_size;
        let y_idx = (vis_idx % (cube_size * cube_size)) / cube_size;
        let z_idx = vis_idx / (cube_size * cube_size);
        
        let u = if cube_size > 1 { x_idx as f32 / (cube_size - 1) as f32 } else { 0.5 };
        let v = if cube_size > 1 { y_idx as f32 / (cube_size - 1) as f32 } else { 0.5 };
        let w = if cube_size > 1 { z_idx as f32 / (cube_size - 1) as f32 } else { norm_val };
        
        let point_3d = cube.point_normalized(u, v, w);
        
        if let Some(proj) = camera_controller.camera.project(point_3d) {
            let screen_x = center.x + proj.x * half_width;
            let screen_y = center.y - proj.y * half_height;
            positions.push((egui::pos2(screen_x, screen_y), actual_idx));
        }
    }
    
    positions
}

fn render_grid_3d(
    painter: &egui::Painter,
    cube: &Cube3DContainer,
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) {
    let center = plot_rect.center();
    let half_width = plot_rect.width() / 2.0;
    let half_height = plot_rect.height() / 2.0;
    let grid_divisions = 5;
    let grid_color = egui::Color32::from_rgba_unmultiplied(120, 120, 140, 100);
    let grid_stroke = egui::Stroke::new(1.0, grid_color);
    
    for i in 0..=grid_divisions {
        let t = i as f32 / grid_divisions as f32;
        for j in 0..=grid_divisions {
            let s = j as f32 / grid_divisions as f32;
            let p1 = cube.point_normalized(t, s, 0.0);
            let p2 = cube.point_normalized(t, s, 1.0);
            if let (Some(proj1), Some(proj2)) = (camera_controller.camera.project(p1), camera_controller.camera.project(p2)) {
                let screen1 = egui::pos2(center.x + proj1.x * half_width, center.y - proj1.y * half_height);
                let screen2 = egui::pos2(center.x + proj2.x * half_width, center.y - proj2.y * half_height);
                painter.line_segment([screen1, screen2], grid_stroke);
            }
        }
    }
    
    for i in 0..=grid_divisions {
        let t = i as f32 / grid_divisions as f32;
        for j in 0..=grid_divisions {
            let s = j as f32 / grid_divisions as f32;
            let p1 = cube.point_normalized(t, 0.0, s);
            let p2 = cube.point_normalized(t, 1.0, s);
            if let (Some(proj1), Some(proj2)) = (camera_controller.camera.project(p1), camera_controller.camera.project(p2)) {
                let screen1 = egui::pos2(center.x + proj1.x * half_width, center.y - proj1.y * half_height);
                let screen2 = egui::pos2(center.x + proj2.x * half_width, center.y - proj2.y * half_height);
                painter.line_segment([screen1, screen2], grid_stroke);
            }
        }
    }
    
    for i in 0..=grid_divisions {
        let t = i as f32 / grid_divisions as f32;
        for j in 0..=grid_divisions {
            let s = j as f32 / grid_divisions as f32;
            let p1 = cube.point_normalized(0.0, t, s);
            let p2 = cube.point_normalized(1.0, t, s);
            if let (Some(proj1), Some(proj2)) = (camera_controller.camera.project(p1), camera_controller.camera.project(p2)) {
                let screen1 = egui::pos2(center.x + proj1.x * half_width, center.y - proj1.y * half_height);
                let screen2 = egui::pos2(center.x + proj2.x * half_width, center.y - proj2.y * half_height);
                painter.line_segment([screen1, screen2], grid_stroke);
            }
        }
    }
    
    let axis_length = 0.15;
    let axis_stroke_x = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 0, 0));
    let axis_stroke_y = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 0));
    let axis_stroke_z = egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 0, 255));
    
    let origin = cube.center;
    if let Some(proj_origin) = camera_controller.camera.project(origin) {
        let screen_origin = egui::pos2(center.x + proj_origin.x * half_width, center.y - proj_origin.y * half_height);
        
        let x_end = origin.offset(axis_length, 0.0, 0.0);
        if let Some(proj_x) = camera_controller.camera.project(x_end) {
            let screen_x = egui::pos2(center.x + proj_x.x * half_width, center.y - proj_x.y * half_height);
            painter.line_segment([screen_origin, screen_x], axis_stroke_x);
        }
        
        let y_end = origin.offset(0.0, axis_length, 0.0);
        if let Some(proj_y) = camera_controller.camera.project(y_end) {
            let screen_y = egui::pos2(center.x + proj_y.x * half_width, center.y - proj_y.y * half_height);
            painter.line_segment([screen_origin, screen_y], axis_stroke_y);
        }
        
        let z_end = origin.offset(0.0, 0.0, axis_length);
        if let Some(proj_z) = camera_controller.camera.project(z_end) {
            let screen_z = egui::pos2(center.x + proj_z.x * half_width, center.y - proj_z.y * half_height);
            painter.line_segment([screen_origin, screen_z], axis_stroke_z);
        }
    }
    
    let label_font = egui::FontId::monospace(12.0);
    if let Some(proj_origin) = camera_controller.camera.project(origin) {
        let screen_origin = egui::pos2(center.x + proj_origin.x * half_width, center.y - proj_origin.y * half_height);
        
        let x_end = origin.offset(axis_length, 0.0, 0.0);
        if let Some(proj_x) = camera_controller.camera.project(x_end) {
            let screen_x = egui::pos2(center.x + proj_x.x * half_width, center.y - proj_x.y * half_height);
            painter.text(screen_x + egui::vec2(5.0, -10.0), egui::Align2::LEFT_CENTER, "X", label_font.clone(), egui::Color32::from_rgb(255, 100, 100));
        }
        
        let y_end = origin.offset(0.0, axis_length, 0.0);
        if let Some(proj_y) = camera_controller.camera.project(y_end) {
            let screen_y = egui::pos2(center.x + proj_y.x * half_width, center.y - proj_y.y * half_height);
            painter.text(screen_y + egui::vec2(5.0, -10.0), egui::Align2::LEFT_CENTER, "Y", label_font.clone(), egui::Color32::from_rgb(100, 255, 100));
        }
        
        let z_end = origin.offset(0.0, 0.0, axis_length);
        if let Some(proj_z) = camera_controller.camera.project(z_end) {
            let screen_z = egui::pos2(center.x + proj_z.x * half_width, center.y - proj_z.y * half_height);
            painter.text(screen_z + egui::vec2(5.0, -10.0), egui::Align2::LEFT_CENTER, "Z", label_font.clone(), egui::Color32::from_rgb(100, 100, 255));
        }
    }
}
