use super::camera::{Point3D, Point2D};

#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4 {
    data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let range = far - near;
        
        Self {
            data: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, -(far + near) / range, -1.0],
                [0.0, 0.0, -(2.0 * far * near) / range, 0.0],
            ],
        }
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        
        Self {
            data: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / height, 0.0, 0.0],
                [0.0, 0.0, -2.0 / depth, 0.0],
                [-(right + left) / width, -(top + bottom) / height, -(far + near) / depth, 1.0],
            ],
        }
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }

    pub fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            data: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, s, 0.0],
                [0.0, -s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                [c, 0.0, -s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            data: [
                [c, s, 0.0, 0.0],
                [-s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn mul(&self, other: &Matrix4x4) -> Self {
        let mut result = [[0.0; 4]; 4];
        
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result[i][j] = sum;
            }
        }
        
        Self { data: result }
    }

    pub fn mul_vec(&self, p: Point3D) -> Point3D {
        let v = [p.x, p.y, p.z, 1.0];
        let mut result = [0.0; 4];
        
        for i in 0..4 {
            for j in 0..4 {
                result[i] += self.data[i][j] * v[j];
            }
        }
        
        if result[3] != 0.0 {
            let w = result[3];
            Point3D::new(result[0] / w, result[1] / w, result[2] / w)
        } else {
            Point3D::new(result[0], result[1], result[2])
        }
    }
}

pub struct PerspectiveCamera {
    pub eye: Point3D,
    pub target: Point3D,
    pub up: Point3D,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl PerspectiveCamera {
    pub fn new(eye: Point3D, target: Point3D, fov: f32, aspect: f32) -> Self {
        Self {
            eye,
            target,
            up: Point3D::new(0.0, 0.0, 1.0),
            fov,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> Matrix4x4 {
        let f = Point3D::new(
            self.target.x - self.eye.x,
            self.target.y - self.eye.y,
            self.target.z - self.eye.z,
        );
        
        let f_len = (f.x * f.x + f.y * f.y + f.z * f.z).sqrt().max(0.001);
        let f = Point3D::new(f.x / f_len, f.y / f_len, f.z / f_len);
        
        let s = Point3D::new(
            f.y * self.up.z - f.z * self.up.y,
            f.z * self.up.x - f.x * self.up.z,
            f.x * self.up.y - f.y * self.up.x,
        );
        
        let s_len = (s.x * s.x + s.y * s.y + s.z * s.z).sqrt().max(0.001);
        let s = Point3D::new(s.x / s_len, s.y / s_len, s.z / s_len);
        
        let u = Point3D::new(
            s.y * f.z - s.z * f.y,
            s.z * f.x - s.x * f.z,
            s.x * f.y - s.y * f.x,
        );
        
        Matrix4x4 {
            data: [
                [s.x, u.x, -f.x, 0.0],
                [s.y, u.y, -f.y, 0.0],
                [s.z, u.z, -f.z, 0.0],
                [
                    -(s.x * self.eye.x + s.y * self.eye.y + s.z * self.eye.z),
                    -(u.x * self.eye.x + u.y * self.eye.y + u.z * self.eye.z),
                    f.x * self.eye.x + f.y * self.eye.y + f.z * self.eye.z,
                    1.0,
                ],
            ],
        }
    }

    pub fn projection_matrix(&self) -> Matrix4x4 {
        Matrix4x4::perspective(self.fov, self.aspect, self.near, self.far)
    }

    pub fn project(&self, p: Point3D) -> Option<Point2D> {
        let view = self.view_matrix();
        let proj = self.projection_matrix();
        let mvp = proj.mul(&view);
        
        let p_proj = mvp.mul_vec(p);
        
        let denom = p_proj.z.abs();
        if denom < 0.01 {
            return None;
        }
        
        let x = p_proj.x / denom;
        let y = p_proj.y / denom;
        
        if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
            return None;
        }
        
        Some(Point2D::new(x, y))
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self::new(
            Point3D::new(12.0, 12.0, 12.0),
            Point3D::new(0.0, 0.0, 0.0),
            45.0_f32.to_radians(),
            1.0,
        )
    }
}
