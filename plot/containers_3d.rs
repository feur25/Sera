use super::camera::Point3D;
use super::projection::PerspectiveCamera;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Container3DKind {
    Cube,
}

pub struct CameraController {
    pub camera: PerspectiveCamera,
    pub target: Point3D,
    pub orbit_radius: f32,
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}

impl CameraController {
    pub fn new(target: Point3D) -> Self {
        let mut cam = Self {
            camera: PerspectiveCamera::new(
                Point3D::new(1.0, 1.0, 1.0),
                target,
                45.0_f32.to_radians(),
                1.0,
            ),
            target,
            orbit_radius: 1.0,
            orbit_yaw: 0.785,
            orbit_pitch: 0.785,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
        };
        cam.update_eye();
        cam
    }

    pub fn set_viewport(&mut self, width: f32, height: f32) {
        if height > 0.0 {
            self.camera.aspect = width / height;
        }
        self.update_eye();
    }

    pub fn rotate_orbit(&mut self, dyaw: f32, dpitch: f32) {
        self.orbit_yaw += dyaw;
        self.orbit_pitch = (self.orbit_pitch + dpitch)
            .max(-std::f32::consts::PI / 2.0 + 0.1)
            .min(std::f32::consts::PI / 2.0 - 0.1);
        self.update_eye();
    }

    pub fn zoom(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).max(0.1).min(10.0);
        self.update_eye();
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.pan_x += dx * 0.1 / self.zoom;
        self.pan_y += dy * 0.1 / self.zoom;
        self.update_eye();
    }

    pub fn reset(&mut self) {
        self.orbit_yaw = 0.0;
        self.orbit_pitch = 0.0;
        self.zoom = 1.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
        self.update_eye();
    }

    fn update_eye(&mut self) {
        let r = self.orbit_radius * self.zoom;
        let eye_x = self.target.x + r * self.orbit_yaw.cos() * self.orbit_pitch.cos();
        let eye_y = self.target.y + r * self.orbit_yaw.sin() * self.orbit_pitch.cos();
        let eye_z = self.target.z + r * self.orbit_pitch.sin();

        self.camera.eye = Point3D::new(eye_x, eye_y, eye_z);
        self.camera.target = self.target;
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new(Point3D::new(0.0, 0.0, 0.0))
    }
}

pub struct Cube3DContainer {
    pub center: Point3D,
    pub size: f32,
}

impl Cube3DContainer {
    pub fn new(center: Point3D, size: f32) -> Self {
        Self { center, size }
    }

    pub fn vertices(&self) -> [Point3D; 8] {
        let h = self.size / 2.0;
        [
            Point3D::new(self.center.x - h, self.center.y - h, self.center.z - h),
            Point3D::new(self.center.x + h, self.center.y - h, self.center.z - h),
            Point3D::new(self.center.x + h, self.center.y + h, self.center.z - h),
            Point3D::new(self.center.x - h, self.center.y + h, self.center.z - h),
            Point3D::new(self.center.x - h, self.center.y - h, self.center.z + h),
            Point3D::new(self.center.x + h, self.center.y - h, self.center.z + h),
            Point3D::new(self.center.x + h, self.center.y + h, self.center.z + h),
            Point3D::new(self.center.x - h, self.center.y + h, self.center.z + h),
        ]
    }

    pub fn point_normalized(&self, u: f32, v: f32, w: f32) -> Point3D {
        let h = self.size / 2.0;
        Point3D::new(
            self.center.x + (u - 0.5) * 2.0 * h,
            self.center.y + (v - 0.5) * 2.0 * h,
            self.center.z + (w - 0.5) * 2.0 * h,
        )
    }

    pub fn edges(&self) -> [[usize; 2]; 12] {
        [
            [0, 1],
            [1, 2],
            [2, 3],
            [3, 0],
            [4, 5],
            [5, 6],
            [6, 7],
            [7, 4],
            [0, 4],
            [1, 5],
            [2, 6],
            [3, 7],
        ]
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
            if let (Some(proj1), Some(proj2)) = (
                camera_controller.camera.project(p1),
                camera_controller.camera.project(p2),
            ) {
                let screen1 = egui::pos2(
                    center.x + proj1.x * half_width,
                    center.y - proj1.y * half_height,
                );
                let screen2 = egui::pos2(
                    center.x + proj2.x * half_width,
                    center.y - proj2.y * half_height,
                );
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
            if let (Some(proj1), Some(proj2)) = (
                camera_controller.camera.project(p1),
                camera_controller.camera.project(p2),
            ) {
                let screen1 = egui::pos2(
                    center.x + proj1.x * half_width,
                    center.y - proj1.y * half_height,
                );
                let screen2 = egui::pos2(
                    center.x + proj2.x * half_width,
                    center.y - proj2.y * half_height,
                );
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
            if let (Some(proj1), Some(proj2)) = (
                camera_controller.camera.project(p1),
                camera_controller.camera.project(p2),
            ) {
                let screen1 = egui::pos2(
                    center.x + proj1.x * half_width,
                    center.y - proj1.y * half_height,
                );
                let screen2 = egui::pos2(
                    center.x + proj2.x * half_width,
                    center.y - proj2.y * half_height,
                );
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
        let screen_origin = egui::pos2(
            center.x + proj_origin.x * half_width,
            center.y - proj_origin.y * half_height,
        );

        let x_end = origin.offset(axis_length, 0.0, 0.0);
        if let Some(proj_x) = camera_controller.camera.project(x_end) {
            let screen_x = egui::pos2(
                center.x + proj_x.x * half_width,
                center.y - proj_x.y * half_height,
            );
            painter.line_segment([screen_origin, screen_x], axis_stroke_x);
        }

        let y_end = origin.offset(0.0, axis_length, 0.0);
        if let Some(proj_y) = camera_controller.camera.project(y_end) {
            let screen_y = egui::pos2(
                center.x + proj_y.x * half_width,
                center.y - proj_y.y * half_height,
            );
            painter.line_segment([screen_origin, screen_y], axis_stroke_y);
        }

        let z_end = origin.offset(0.0, 0.0, axis_length);
        if let Some(proj_z) = camera_controller.camera.project(z_end) {
            let screen_z = egui::pos2(
                center.x + proj_z.x * half_width,
                center.y - proj_z.y * half_height,
            );
            painter.line_segment([screen_origin, screen_z], axis_stroke_z);
        }
    }

    let label_font = egui::FontId::monospace(12.0);
    if let Some(_) = camera_controller.camera.project(origin) {
        let x_end = origin.offset(axis_length, 0.0, 0.0);
        if let Some(proj_x) = camera_controller.camera.project(x_end) {
            let screen_x = egui::pos2(
                center.x + proj_x.x * half_width,
                center.y - proj_x.y * half_height,
            );
            painter.text(
                screen_x + egui::vec2(5.0, -10.0),
                egui::Align2::LEFT_CENTER,
                "X",
                label_font.clone(),
                egui::Color32::from_rgb(255, 100, 100),
            );
        }

        let y_end = origin.offset(0.0, axis_length, 0.0);
        if let Some(proj_y) = camera_controller.camera.project(y_end) {
            let screen_y = egui::pos2(
                center.x + proj_y.x * half_width,
                center.y - proj_y.y * half_height,
            );
            painter.text(
                screen_y + egui::vec2(5.0, -10.0),
                egui::Align2::LEFT_CENTER,
                "Y",
                label_font.clone(),
                egui::Color32::from_rgb(100, 255, 100),
            );
        }

        let z_end = origin.offset(0.0, 0.0, axis_length);
        if let Some(proj_z) = camera_controller.camera.project(z_end) {
            let screen_z = egui::pos2(
                center.x + proj_z.x * half_width,
                center.y - proj_z.y * half_height,
            );
            painter.text(
                screen_z + egui::vec2(5.0, -10.0),
                egui::Align2::LEFT_CENTER,
                "Z",
                label_font.clone(),
                egui::Color32::from_rgb(100, 100, 255),
            );
        }
    }
}
