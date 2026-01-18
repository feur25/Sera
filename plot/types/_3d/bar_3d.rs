use super::super::super::generic::*;
use super::super::super::camera::{Camera3D, Point3D, Cube3D};

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
    let bar_width = ctx.plot_rect.width() / (visible_count as f32).max(1.0) * 0.6;
    let norm_height = ctx.plot_rect.height();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        let height = norm_val as f32 * norm_height;
        
        let cube = Cube3D::new(
            Point3D::new(vis_idx as f32, 0.25, height / 2.0),
            bar_width,
            0.5,
            height,
        );
        
        let projected = cube.project_all(ctx.camera);
        let center_x = ctx.plot_rect.center().x;
        let center_y = ctx.plot_rect.center().y;
        
        let screen_pts: [egui::Pos2; 8] = projected.map(|p| {
            egui::pos2(center_x + p.x, center_y - p.y)
        });
        
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        render_cube_faces(ctx.painter, &screen_pts, display_color);
    }
}

fn render_cube_faces(painter: &egui::Painter, pts: &[egui::Pos2; 8], color: egui::Color32) {
    let darker = shade_color(color, 0.7);
    let darkest = shade_color(color, 0.5);
    
    render_quad(painter, [pts[0], pts[1], pts[5], pts[4]], color);
    render_quad(painter, [pts[1], pts[2], pts[6], pts[5]], darker);
    render_quad(painter, [pts[3], pts[7], pts[6], pts[2]], darker);
    render_quad(painter, [pts[0], pts[4], pts[7], pts[3]], darkest);
    
    for edge in &[[pts[4], pts[5]], [pts[5], pts[6]], [pts[6], pts[7]], [pts[7], pts[4]]] {
        painter.line_segment(*edge, egui::Stroke::new(1.0, darkest));
    }
}

fn render_quad(painter: &egui::Painter, points: [egui::Pos2; 4], color: egui::Color32) {
    for i in 0..4 {
        painter.line_segment(
            [points[i], points[(i + 1) % 4]],
            egui::Stroke::new(1.0, color),
        );
    }
}

fn shade_color(color: egui::Color32, factor: f32) -> egui::Color32 {
    egui::Color32::from_rgb(
        (color.r() as f32 * factor) as u8,
        (color.g() as f32 * factor) as u8,
        (color.b() as f32 * factor) as u8,
    )
}
