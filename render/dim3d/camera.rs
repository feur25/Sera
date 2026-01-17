use super::types::*;

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Point3D,
    pub target: Point3D,
    pub up: Vector3D,
    pub fov: f64,
    pub near: f64,
    pub far: f64,
    pub projection: ProjectionMode,
    pub aspect_ratio: f64,
    pub rotation: Rotation,
}

impl Camera {
    pub fn new(position: Point3D, target: Point3D) -> Self {
        Camera {
            position,
            target,
            up: Vector3D::new(0.0, 1.0, 0.0),
            fov: 45.0,
            near: 0.1,
            far: 1000.0,
            projection: ProjectionMode::Perspective,
            aspect_ratio: 16.0 / 9.0,
            rotation: Rotation::zero(),
        }
    }

    pub fn standard() -> Self {
        Camera::new(Point3D::new(5.0, 5.0, 5.0), Point3D::origin())
    }

    pub fn orbit_radius(&self) -> f64 {
        self.position.distance(&self.target)
    }

    pub fn orbit(&mut self, yaw_delta: f64, pitch_delta: f64, radius: f64) {
        let yaw = self.rotation.yaw + yaw_delta;
        let mut pitch = self.rotation.pitch + pitch_delta;

        pitch = pitch.max(-89.9).min(89.9);

        self.rotation = Rotation::new(self.rotation.roll, pitch, yaw);

        let yaw_rad = yaw.to_radians();
        let pitch_rad = pitch.to_radians();

        self.position = Point3D::new(
            self.target.x + radius * pitch_rad.cos() * yaw_rad.sin(),
            self.target.y + radius * pitch_rad.sin(),
            self.target.z + radius * pitch_rad.cos() * yaw_rad.cos(),
        );
    }

    pub fn pan(&mut self, dx: f64, dy: f64, dz: f64) {
        self.position = self.position.translate(dx, dy, dz);
        self.target = self.target.translate(dx, dy, dz);
    }

    pub fn zoom(&mut self, factor: f64) {
        let direction = Vector3D::new(
            self.target.x - self.position.x,
            self.target.y - self.position.y,
            self.target.z - self.position.z,
        );
        let scaled = direction.scale(factor);
        self.position = Point3D::new(
            self.position.x + scaled.x,
            self.position.y + scaled.y,
            self.position.z + scaled.z,
        );
    }

    pub fn look_at(&mut self, target: Point3D) {
        self.target = target;
    }

    pub fn reset(&mut self) {
        self.rotation = Rotation::zero();
        self.position = Point3D::new(5.0, 5.0, 5.0);
        self.target = Point3D::origin();
    }

    pub fn front_view(&mut self) {
        self.rotation = Rotation::zero();
        self.position = Point3D::new(0.0, 0.0, 5.0);
    }

    pub fn top_view(&mut self) {
        self.rotation = Rotation::new(0.0, -90.0, 0.0);
        self.position = Point3D::new(0.0, 5.0, 0.0);
    }

    pub fn side_view(&mut self) {
        self.rotation = Rotation::new(0.0, 0.0, 90.0);
        self.position = Point3D::new(5.0, 0.0, 0.0);
    }

    pub fn isometric_view(&mut self) {
        self.rotation = Rotation::new(0.0, 45.0, 45.0);
        self.position = Point3D::new(5.0, 5.0, 5.0);
    }

    pub fn set_projection(&mut self, mode: ProjectionMode) {
        self.projection = mode;
    }

    pub fn set_aspect_ratio(&mut self, width: f64, height: f64) {
        self.aspect_ratio = width / height;
    }
}

#[derive(Clone, Debug)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Viewport {
            width,
            height,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn with_offset(width: f64, height: f64, x: f64, y: f64) -> Self {
        Viewport { width, height, x, y }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }
}
