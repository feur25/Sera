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
        
        Point3D::new(result[0], result[1], result[2])
    }

    pub fn mul_vec_homo(&self, p: Point3D) -> [f32; 4] {
        let v = [p.x, p.y, p.z, 1.0];
        let mut result = [0.0; 4];
        
        for i in 0..4 {
            for j in 0..4 {
                result[i] += self.data[i][j] * v[j];
            }
        }
        
        result
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
            near: 0.001,
            far: 1000.0,
        }
    }

    pub fn project(&self, p: Point3D) -> Option<Point2D> {
        let f_x = self.target.x - self.eye.x;
        let f_y = self.target.y - self.eye.y;
        let f_z = self.target.z - self.eye.z;
        let f_len = (f_x * f_x + f_y * f_y + f_z * f_z).sqrt();
        
        if f_len < 0.0001 {
            return None;
        }
        
        let f_x_n = f_x / f_len;
        let f_y_n = f_y / f_len;
        let f_z_n = f_z / f_len;
        
        let r_x = self.up.y * f_z_n - self.up.z * f_y_n;
        let r_y = self.up.z * f_x_n - self.up.x * f_z_n;
        let r_z = self.up.x * f_y_n - self.up.y * f_x_n;
        let r_len = (r_x * r_x + r_y * r_y + r_z * r_z).sqrt();
        
        let r_x = r_x / r_len.max(0.0001);
        let r_y = r_y / r_len.max(0.0001);
        let r_z = r_z / r_len.max(0.0001);
        
        let u_x = f_y_n * r_z - f_z_n * r_y;
        let u_y = f_z_n * r_x - f_x_n * r_z;
        let u_z = f_x_n * r_y - f_y_n * r_x;
        
        let p_x = p.x - self.eye.x;
        let p_y = p.y - self.eye.y;
        let p_z = p.z - self.eye.z;
        
        let depth = p_x * f_x_n + p_y * f_y_n + p_z * f_z_n;
        
        if depth < self.near || depth > self.far {
            return None;
        }
        
        let x_cam = p_x * r_x + p_y * r_y + p_z * r_z;
        let y_cam = p_x * u_x + p_y * u_y + p_z * u_z;
        
        let tan_half = (self.fov / 2.0).tan();
        let scale = tan_half * self.aspect;
        let x_screen = (x_cam / depth) / scale;
        let y_screen = (y_cam / depth) / tan_half;
        
        if !x_screen.is_finite() || !y_screen.is_finite() {
            return None;
        }
        
        if x_screen.abs() > 2.0 || y_screen.abs() > 2.0 {
            return None;
        }
        
        Some(Point2D::new(x_screen, y_screen))
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


