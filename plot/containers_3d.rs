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
            [0, 1], [1, 2], [2, 3], [3, 0],
            [4, 5], [5, 6], [6, 7], [7, 4],
            [0, 4], [1, 5], [2, 6], [3, 7],
        ]
    }
}
