use super::super::super::containers_3d::{CameraController, Cube3DContainer};
use super::super::super::camera::Point3D;
use super::super::super::scale_renderer::render_scale_labels;
use super::super::super::containers_3d::render_3d_grid;

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

    render_3d_grid(&ctx.painter, &cube, ctx.camera_controller, ctx.plot_rect);
    render_scale_labels(&ctx.painter, ctx.plot_rect, max_val);

    let center = ctx.plot_rect.center();
    let half_width = ctx.plot_rect.width() / 2.0;
    let half_height = ctx.plot_rect.height() / 2.0;

    let mut to_render: Vec<(egui::Pos2, egui::Color32, usize, f32)> = Vec::new();

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

pub fn render_scatter3d_html(
    title: &str,
    x_values: &[f64],
    y_values: &[f64],
    z_values: &[f64],
    axis_labels: (&str, &str, &str),
    color_values: &[f64],
    color_labels: &[String],
    width: i32,
    height: i32,
) -> String {
    crate::html::js_3d::render_3d_html(
        0, title, x_values, y_values, z_values,
        axis_labels, color_values, color_labels, width, height,
    )
}