pub mod bar_3d;
pub mod line_3d;
pub mod scatter_3d;
pub mod scale_renderer;
pub mod sphere_renderer;

use super::super::containers_3d::CameraController;
use std::collections::HashMap;
use std::sync::LazyLock;

pub use bar_3d::{render_bars_3d, Bar3DRenderContext};
pub use line_3d::{render_lines_3d, Line3DRenderContext};
pub use scatter_3d::{render_points_3d, Scatter3DRenderContext};

pub struct Plot3DRegistry {
    renderers: HashMap<u8, (&'static str, u8)>,
}

impl Plot3DRegistry {
    fn new() -> Self {
        let mut renderers = HashMap::new();
        renderers.insert(3, ("line_3d", 0));
        renderers.insert(4, ("scatter_3d", 1));
        renderers.insert(5, ("bar_3d", 2));
        
        Self { renderers }
    }
    
    pub fn get(&self, kind: u8) -> Option<(&'static str, u8)> {
        self.renderers.get(&kind).copied()
    }
    
    pub fn list(&self) -> Vec<(u8, &'static str, u8)> {
        let mut items: Vec<_> = self.renderers.iter()
            .map(|(&k, &(n, d))| (k, n, d))
            .collect();
        items.sort_by_key(|&(k, _, _)| k);
        items
    }
}

pub static PLOT_3D_REGISTRY: LazyLock<Plot3DRegistry> = LazyLock::new(Plot3DRegistry::new);

pub fn render_plot_3d_by_type(
    chart_type: u8,
    painter: &egui::Painter,
    plot_rect: egui::Rect,
    colors: &[egui::Color32],
    hovered_idx: Option<usize>,
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &CameraController,
) {
    match chart_type {
        3 if visible_indices.len() > 1 => {
            render_lines_3d(Line3DRenderContext {
                painter,
                plot_rect,
                colors,
                hovered_idx,
                values,
                max_val,
                visible_indices,
                camera_controller,
            });
        }
        4 => {
            render_points_3d(Scatter3DRenderContext {
                painter,
                plot_rect,
                colors,
                hovered_idx,
                values,
                max_val,
                visible_indices,
                camera_controller,
            });
        }
        5 => {
            render_bars_3d(Bar3DRenderContext {
                painter,
                plot_rect,
                colors,
                hovered_idx,
                values,
                max_val,
                visible_indices,
                camera_controller,
            });
        }
        _ => {}
    }
}
