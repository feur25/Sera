/// 3D point with x, y, z coordinates
#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_2d(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }

    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    pub fn offset(mut self, dx: f32, dy: f32, dz: f32) -> Self {
        self.x += dx;
        self.y += dy;
        self.z += dz;
        self
    }
}

/// 2D projected point used for screen rendering
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_egui(&self) -> egui::Pos2 {
        egui::pos2(self.x, self.y)
    }
}
