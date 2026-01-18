use super::super::super::containers_3d::{CameraController, Cube3DContainer};
use super::super::super::camera::Point3D;

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
    let container_size = 10.0;
    
    let cube = Cube3DContainer::new(Point3D::new(0.0, 0.0, 0.0), container_size);
    
    render_container_frame(ctx.painter, ctx.plot_rect, ctx.camera_controller);
    
    let mut bars_to_render: Vec<(egui::Pos2, egui::Pos2, egui::Color32, usize, f32, f32)> = Vec::new();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = ((value / max_val).min(1.0).max(0.0)) as f32;
        
        let x_idx = (vis_idx % grid_size) as f32;
        let y_idx = (vis_idx / grid_size) as f32;
        
        let u = (x_idx + 0.5) / grid_size as f32;
        let v = (y_idx + 0.5) / grid_size as f32;
        
        let base_3d = cube.point_normalized(u, v, 0.3);
        let top_3d = cube.point_normalized(u, v, 0.3 + norm_val * 0.4);
        
        if let (Some(proj_base), Some(proj_top)) = (
            ctx.camera_controller.camera.project(base_3d),
            ctx.camera_controller.camera.project(top_3d),
        ) {
            let scale_base = 0.5 / (base_3d.x.abs() + base_3d.y.abs() + base_3d.z.abs() + 1.0).max(1.0);
            let scale_top = 0.5 / (top_3d.x.abs() + top_3d.y.abs() + top_3d.z.abs() + 1.0).max(1.0);
            
            let screen_base = egui::pos2(
                center.x + proj_base.x * ctx.plot_rect.width() * 0.2 * scale_base,
                center.y + proj_base.y * ctx.plot_rect.height() * 0.2 * scale_base,
            );
            let screen_top = egui::pos2(
                center.x + proj_top.x * ctx.plot_rect.width() * 0.2 * scale_top,
                center.y + proj_top.y * ctx.plot_rect.height() * 0.2 * scale_top,
            );
            
            let color = ctx.colors[actual_idx % ctx.colors.len()];
            let depth = top_3d.x + top_3d.y + top_3d.z;
            
            bars_to_render.push((screen_base, screen_top, color, actual_idx, norm_val, depth));
        }
    }
    
    bars_to_render.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap_or(std::cmp::Ordering::Equal));
    
    for (screen_base, screen_top, color, actual_idx, norm_val, _) in bars_to_render {
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        let stroke_width = 3.0 + norm_val * 4.0;
        ctx.painter.line_segment([screen_base, screen_top], egui::Stroke::new(stroke_width, display_color));
        
        let base_radius = 3.0 + norm_val * 2.0;
        let top_radius = 4.0 + norm_val * 3.0;
        
        ctx.painter.circle_filled(screen_base, base_radius, display_color);
        ctx.painter.circle_filled(screen_top, top_radius, display_color);
        
        if is_hovered {
            ctx.painter.circle_stroke(
                screen_top,
                top_radius + 2.0,
                egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 200, 0)),
            );
        }
    }
}

fn render_container_frame(
    painter: &egui::Painter,
    plot_rect: egui::Rect,
    camera_controller: &CameraController,
) {
    let center = plot_rect.center();
    let container_size = 10.0;
    let container_center = Point3D::new(0.0, 0.0, 0.0);
    
    let cube = Cube3DContainer::new(container_center, container_size);
    let vertices = cube.vertices();
    
    let mut projected = Vec::new();
    for v in &vertices {
        if let Some(p) = camera_controller.camera.project(*v) {
            let scale = 0.5 / (v.x.abs() + v.y.abs() + v.z.abs() + 1.0).max(1.0);
            let screen_x = center.x + p.x * plot_rect.width() * 0.2 * scale;
            let screen_y = center.y + p.y * plot_rect.height() * 0.2 * scale;
            projected.push(egui::pos2(screen_x, screen_y));
        } else {
            projected.push(egui::Pos2::ZERO);
        }
    }
    
    let color = egui::Color32::from_rgb(180, 180, 220);
    
    for edge in cube.edges() {
        let start = projected[edge[0]];
        let end = projected[edge[1]];
        if start != egui::Pos2::ZERO && end != egui::Pos2::ZERO {
            painter.line_segment([start, end], egui::Stroke::new(2.0, color));
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
