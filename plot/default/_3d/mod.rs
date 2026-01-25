pub mod bar_3d;
pub mod line_3d;
pub mod scatter_3d;
pub mod plot_3d_types;

use super::super::containers_3d::CameraController;

pub use bar_3d::*;
pub use line_3d::*;
pub use scatter_3d::*;
pub use plot_3d_types::register_default_3d_types;

pub fn get_3d_positions(
    chart_type: u8,
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    crate::plot::controller::plot_3d_controller::get_3d_positions(
        chart_type, values, max_val, visible_indices, camera_controller, plot_rect
    )
}
