use super::super::super::generic::*;
use super::super::super::camera::{Camera3D, Point3D};

pub struct Scatter3DRenderContext<'a> {
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

pub fn render_points_3d(ctx: Scatter3DRenderContext) {
    let norm_height = ctx.plot_rect.height();
    let center_x = ctx.plot_rect.center().x;
    let center_y = ctx.plot_rect.center().y;
    
    let mut sorted: Vec<_> = ctx.visible_indices.iter().enumerate()
        .map(|(vis_idx, &actual_idx)| {
            let value = ctx.values[actual_idx];
            let norm_val = value / ctx.max_val.max(1.0);
            let height = norm_val as f32 * norm_height;
            
            let p = Point3D::new(
                vis_idx as f32,
                0.5 + (actual_idx as f32 % 5.0) * 0.1,
                height,
            );
            
            (vis_idx, actual_idx, height, p)
        })
        .collect();
    
    sorted.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    
    for (_, actual_idx, _, p) in sorted {
        let proj = ctx.camera.project(p);
        let screen = egui::pos2(center_x + proj.x, center_y - proj.y);
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        let radius = if is_hovered { 6.5 } else { 5.0 };
        let shadow_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 40);
        
        ctx.painter.circle_filled(
            egui::pos2(screen.x + 2.5, screen.y + 2.5),
            radius * 0.9,
            shadow_color,
        );
        ctx.painter.circle_filled(screen, radius, display_color);
        
        let highlight = egui::Color32::from_rgb(255, 255, 255);
        let offset = radius * 0.3;
        ctx.painter.circle_filled(
            egui::pos2(screen.x - offset, screen.y - offset),
            radius * 0.25,
            highlight,
        );
    }
}
