use super::super::super::generic::*;

pub struct Scatter3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
}

pub fn render_points_3d(ctx: Scatter3DRenderContext) {
    let visible_count = ctx.visible_indices.len();
    let depth_scale = 0.15;
    
    let mut points: Vec<(egui::Pos2, usize, f64)> = Vec::new();
    
    for (vis_idx, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let value = ctx.values[actual_idx];
        let norm_val = value / ctx.max_val.max(1.0);
        
        let x = ctx.plot_rect.left() + (ctx.plot_rect.width() / (visible_count as f32 - 1.0).max(1.0)) * vis_idx as f32;
        let y = ctx.plot_rect.bottom() - norm_val as f32 * ctx.plot_rect.height();
        
        let depth = (vis_idx as f32 / visible_count as f32) * depth_scale;
        let pos = egui::pos2(x + depth * 30.0, y + depth * 20.0);
        
        points.push((pos, actual_idx, depth as f64));
    }
    
    points.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    
    for (pos, actual_idx, depth) in points {
        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        
        let radius = if is_hovered { 7.0 } else { 5.0 };
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };
        
        let shadow_offset = depth as f32 * 15.0;
        let shadow_color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 30);
        ctx.painter.circle_filled(
            egui::pos2(pos.x + shadow_offset * 0.3, pos.y + shadow_offset * 0.2),
            radius * 0.8,
            shadow_color,
        );
        
        ctx.painter.circle_filled(pos, radius, display_color);
        
        let highlight_color = egui::Color32::from_rgb(255, 255, 255);
        ctx.painter.circle_filled(
            egui::pos2(pos.x - radius * 0.3, pos.y - radius * 0.3),
            radius * 0.3,
            highlight_color,
        );
    }
}
