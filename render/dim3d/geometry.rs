use super::types::*;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<usize>,
    pub name: String,
    pub visible: bool,
}

impl Mesh {
    pub fn new(name: String) -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            name,
            visible: true,
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex3D) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    pub fn add_triangle(&mut self, i1: usize, i2: usize, i3: usize) {
        self.indices.extend_from_slice(&[i1, i2, i3]);
    }

    pub fn add_line(&mut self, i1: usize, i2: usize) {
        self.indices.extend_from_slice(&[i1, i2]);
    }

    pub fn bounds(&self) -> Option<Bounds3D> {
        if self.vertices.is_empty() {
            return None;
        }

        let mut min = self.vertices[0].point;
        let mut max = self.vertices[0].point;

        for v in &self.vertices {
            if v.point.x < min.x {
                min.x = v.point.x;
            }
            if v.point.y < min.y {
                min.y = v.point.y;
            }
            if v.point.z < min.z {
                min.z = v.point.z;
            }

            if v.point.x > max.x {
                max.x = v.point.x;
            }
            if v.point.y > max.y {
                max.y = v.point.y;
            }
            if v.point.z > max.z {
                max.z = v.point.z;
            }
        }

        Some(Bounds3D::new(min, max))
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

#[derive(Clone, Debug)]
pub struct Plane {
    pub origin: Point3D,
    pub normal: Vector3D,
}

impl Plane {
    pub fn new(origin: Point3D, normal: Vector3D) -> Self {
        Plane {
            origin,
            normal: normal.normalize(),
        }
    }

    pub fn xy() -> Self {
        Plane::new(Point3D::origin(), Vector3D::new(0.0, 0.0, 1.0))
    }

    pub fn xz() -> Self {
        Plane::new(Point3D::origin(), Vector3D::new(0.0, 1.0, 0.0))
    }

    pub fn yz() -> Self {
        Plane::new(Point3D::origin(), Vector3D::new(1.0, 0.0, 0.0))
    }

    pub fn distance_to_point(&self, point: &Point3D) -> f64 {
        let v = Vector3D::new(
            point.x - self.origin.x,
            point.y - self.origin.y,
            point.z - self.origin.z,
        );
        (v.dot(&self.normal)).abs()
    }

    pub fn project_point(&self, point: &Point3D) -> Point3D {
        let v = Vector3D::new(
            point.x - self.origin.x,
            point.y - self.origin.y,
            point.z - self.origin.z,
        );
        let dist = v.dot(&self.normal);
        let scaled = self.normal.scale(dist);
        Point3D::new(
            point.x - scaled.x,
            point.y - scaled.y,
            point.z - scaled.z,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Box3D {
    pub min: Point3D,
    pub max: Point3D,
    pub color: Color,
}

impl Box3D {
    pub fn new(min: Point3D, max: Point3D, color: Color) -> Self {
        Box3D { min, max, color }
    }

    pub fn size(&self) -> Vector3D {
        Vector3D::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn center(&self) -> Point3D {
        Point3D::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, color: Color) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }

    pub fn contains_point(&self, point: &Point3D) -> bool {
        self.center.distance(point) <= self.radius
    }
}

#[derive(Clone, Debug)]
pub struct Cylinder {
    pub center: Point3D,
    pub radius: f64,
    pub height: f64,
    pub color: Color,
}

impl Cylinder {
    pub fn new(center: Point3D, radius: f64, height: f64, color: Color) -> Self {
        Cylinder {
            center,
            radius,
            height,
            color,
        }
    }
}
