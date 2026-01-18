#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProjectionMode {
    XY,
    XZ,
    YZ,
}

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

pub trait Projector {
    fn project(&self, p: Point3D) -> Point2D;
}

pub struct IsometricXY {
    pub scale: f32,
    pub angle: f32,
}

impl IsometricXY {
    pub fn new(scale: f32) -> Self {
        Self { scale, angle: 30.0 }
    }
}

impl Projector for IsometricXY {
    fn project(&self, p: Point3D) -> Point2D {
        let rad = self.angle.to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();
        
        let screen_x = (p.x - p.y) * cos_a * self.scale;
        let screen_y = (p.x + p.y) * sin_a * self.scale - p.z * self.scale;
        
        Point2D { x: screen_x, y: screen_y }
    }
}

pub struct IsometricXZ {
    pub scale: f32,
    pub angle: f32,
}

impl IsometricXZ {
    pub fn new(scale: f32) -> Self {
        Self { scale, angle: 25.0 }
    }
}

impl Projector for IsometricXZ {
    fn project(&self, p: Point3D) -> Point2D {
        let rad = self.angle.to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();
        
        let screen_x = (p.x - p.z) * cos_a * self.scale;
        let screen_y = (p.x + p.z) * sin_a * self.scale - p.y * self.scale * 0.8;
        
        Point2D { x: screen_x, y: screen_y }
    }
}

pub struct IsometricYZ {
    pub scale: f32,
    pub angle: f32,
}

impl IsometricYZ {
    pub fn new(scale: f32) -> Self {
        Self { scale, angle: 35.0 }
    }
}

impl Projector for IsometricYZ {
    fn project(&self, p: Point3D) -> Point2D {
        let rad = self.angle.to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();
        
        let screen_x = (p.y - p.z) * cos_a * self.scale;
        let screen_y = (p.y + p.z) * sin_a * self.scale - p.x * self.scale;
        
        Point2D { x: screen_x, y: screen_y }
    }
}

pub struct Matrix3x3 {
    data: [[f32; 3]; 3],
}

impl Clone for Matrix3x3 {
    fn clone(&self) -> Self {
        Self { data: self.data }
    }
}

impl Copy for Matrix3x3 {}

impl Matrix3x3 {
    pub fn identity() -> Self {
        Self {
            data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn mul_vec(&self, v: Point3D) -> Point3D {
        let x = self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z;
        let y = self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z;
        let z = self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z;
        Point3D::new(x, y, z)
    }

    pub fn mul(&self, other: &Matrix3x3) -> Self {
        let mut result = Self::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = 0.0;
                for k in 0..3 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}

#[derive(Clone, Copy)]
pub struct Camera3D {
    pub mode: ProjectionMode,
    pub scale: f32,
    pub rotation: Matrix3x3,
    pub eye_distance: f32,
}

impl Camera3D {
    pub fn new() -> Self {
        Self {
            mode: ProjectionMode::XY,
            scale: 0.35,
            rotation: Matrix3x3::identity(),
            eye_distance: 5.0,
        }
    }

    pub fn with_rotation(mut self, rx: f32, ry: f32, rz: f32) -> Self {
        self.rotation = Matrix3x3::rotation_z(rz)
            .mul(&Matrix3x3::rotation_y(ry))
            .mul(&Matrix3x3::rotation_x(rx));
        self
    }

    pub fn next_perspective(&mut self) {
        self.mode = match self.mode {
            ProjectionMode::XY => {
                self.rotation = Matrix3x3::rotation_x(std::f32::consts::PI / 6.0);
                ProjectionMode::XZ
            }
            ProjectionMode::XZ => {
                self.rotation = Matrix3x3::rotation_y(std::f32::consts::PI / 4.0);
                ProjectionMode::YZ
            }
            ProjectionMode::YZ => {
                self.rotation = Matrix3x3::identity();
                ProjectionMode::XY
            }
        };
    }

    pub fn project(&self, p: Point3D) -> Point2D {
        let rotated = self.rotation.mul_vec(p);
        match self.mode {
            ProjectionMode::XY => IsometricXY::new(self.scale).project(rotated),
            ProjectionMode::XZ => IsometricXZ::new(self.scale).project(rotated),
            ProjectionMode::YZ => IsometricYZ::new(self.scale).project(rotated),
        }
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Cube3D([Point3D; 8]);

impl Cube3D {
    pub fn new(center: Point3D, width: f32, depth: f32, height: f32) -> Self {
        let hw = width / 2.0;
        let hd = depth / 2.0;
        
        let vertices = [
            Point3D::new(center.x - hw, center.y - hd, center.z),
            Point3D::new(center.x + hw, center.y - hd, center.z),
            Point3D::new(center.x + hw, center.y + hd, center.z),
            Point3D::new(center.x - hw, center.y + hd, center.z),
            Point3D::new(center.x - hw, center.y - hd, center.z + height),
            Point3D::new(center.x + hw, center.y - hd, center.z + height),
            Point3D::new(center.x + hw, center.y + hd, center.z + height),
            Point3D::new(center.x - hw, center.y + hd, center.z + height),
        ];
        
        Cube3D(vertices)
    }

    pub fn vertices(&self) -> &[Point3D; 8] {
        &self.0
    }

    pub fn project_all(&self, camera: &Camera3D) -> [egui::Pos2; 8] {
        [
            camera.project(self.0[0]).to_egui(),
            camera.project(self.0[1]).to_egui(),
            camera.project(self.0[2]).to_egui(),
            camera.project(self.0[3]).to_egui(),
            camera.project(self.0[4]).to_egui(),
            camera.project(self.0[5]).to_egui(),
            camera.project(self.0[6]).to_egui(),
            camera.project(self.0[7]).to_egui(),
        ]
    }
}
