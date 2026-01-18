use super::super::super::containers_3d::CameraController;
use super::super::super::camera::Point3D;

pub fn render_sphere_3d(
    painter: &egui::Painter,
    center_3d: Point3D,
    radius: f32,
    color: egui::Color32,
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) {
    if let Some(proj) = camera_controller.camera.project(center_3d) {
        let center_screen = plot_rect.center();
        let screen_x = center_screen.x + proj.x * plot_rect.width() / 2.0;
        let screen_y = center_screen.y - proj.y * plot_rect.height() / 2.0;
        let screen_pos = egui::pos2(screen_x, screen_y);
        
        painter.circle_filled(screen_pos, radius, color);
    }
}
