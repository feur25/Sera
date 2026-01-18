use crate::plot::{Surface3D, Heatmap3D, Camera3D};

pub struct Surface3DViewer {
    pub surface: Option<Surface3D>,
    pub camera: Camera3D,
    pub auto_rotate: bool,
    pub rotation_speed: f32,
}

impl Surface3DViewer {
    pub fn new() -> Self {
        Self {
            surface: None,
            camera: Camera3D::new(),
            auto_rotate: false,
            rotation_speed: 0.01,
        }
    }

    pub fn with_surface(mut self, surface: Surface3D) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn set_auto_rotate(mut self, enabled: bool) -> Self {
        self.auto_rotate = enabled;
        self
    }

    pub fn update(&mut self) {
        if self.auto_rotate {
            self.camera = self.camera.with_rotation(
                self.rotation_speed,
                self.rotation_speed * 1.5,
                0.0,
            );
        }
    }

    pub fn render(&self, painter: &egui::Painter, rect: egui::Rect) {
        if let Some(surface) = &self.surface {
            surface.render(painter, &self.camera, rect.center());
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.camera = self.camera.with_rotation(angle, 0.0, 0.0);
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.camera = self.camera.with_rotation(0.0, angle, 0.0);
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.camera = self.camera.with_rotation(0.0, 0.0, angle);
    }

    pub fn reset_view(&mut self) {
        self.camera = Camera3D::new();
    }
}

pub struct Heatmap3DViewer {
    pub heatmap: Option<Heatmap3D>,
    pub camera: Camera3D,
    pub auto_rotate: bool,
}

impl Heatmap3DViewer {
    pub fn new() -> Self {
        Self {
            heatmap: None,
            camera: Camera3D::new(),
            auto_rotate: false,
        }
    }

    pub fn with_heatmap(mut self, heatmap: Heatmap3D) -> Self {
        self.heatmap = Some(heatmap);
        self
    }

    pub fn render(&self, painter: &egui::Painter, rect: egui::Rect) {
        if let Some(heatmap) = &self.heatmap {
            heatmap.render(painter, &self.camera, rect.center());
        }
    }

    pub fn rotate_view(&mut self, angle: f32) {
        self.camera = self.camera.with_rotation(angle, angle * 0.7, 0.0);
    }
}
