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

pub struct Camera3D {
    pub mode: ProjectionMode,
    pub scale: f32,
}

impl Camera3D {
    pub fn new() -> Self {
        Self {
            mode: ProjectionMode::XY,
            scale: 0.35,
        }
    }

    pub fn next_perspective(&mut self) {
        self.mode = match self.mode {
            ProjectionMode::XY => ProjectionMode::XZ,
            ProjectionMode::XZ => ProjectionMode::YZ,
            ProjectionMode::YZ => ProjectionMode::XY,
        };
    }

    pub fn project(&self, p: Point3D) -> Point2D {
        match self.mode {
            ProjectionMode::XY => {
                let proj = IsometricXY::new(self.scale);
                proj.project(p)
            }
            ProjectionMode::XZ => {
                let proj = IsometricXZ::new(self.scale);
                proj.project(p)
            }
            ProjectionMode::YZ => {
                let proj = IsometricYZ::new(self.scale);
                proj.project(p)
            }
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
