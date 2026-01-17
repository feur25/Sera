use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn origin() -> Self {
        Point3D::new(0.0, 0.0, 0.0)
    }

    pub fn distance(&self, other: &Point3D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn translate(&self, dx: f64, dy: f64, dz: f64) -> Self {
        Point3D::new(self.x + dx, self.y + dy, self.z + dz)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn zero() -> Self {
        Vector3D::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector3D::zero()
        } else {
            Vector3D::new(self.x / mag, self.y / mag, self.z / mag)
        }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn scale(&self, factor: f64) -> Self {
        Vector3D::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

#[derive(Clone, Debug)]
pub struct Vertex3D {
    pub point: Point3D,
    pub color: Color,
    pub label: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Vertex3D {
    pub fn new(point: Point3D, color: Color) -> Self {
        Vertex3D {
            point,
            color,
            label: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Color { r, g, b, a: 255 }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_rgba(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a as f64 / 255.0)
    }

    pub fn red() -> Self {
        Color::new(255, 68, 68)
    }

    pub fn green() -> Self {
        Color::new(102, 204, 102)
    }

    pub fn blue() -> Self {
        Color::new(102, 153, 255)
    }

    pub fn yellow() -> Self {
        Color::new(255, 204, 0)
    }

    pub fn purple() -> Self {
        Color::new(153, 102, 204)
    }

    pub fn cyan() -> Self {
        Color::new(102, 204, 204)
    }

    pub fn magenta() -> Self {
        Color::new(255, 0, 255)
    }

    pub fn white() -> Self {
        Color::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProjectionMode {
    Orthographic,
    Perspective,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChartMode3D {
    Scatter,
    Surface,
    Line,
    Bar,
    Bubble,
    Wireframe,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Rotation {
    pub fn new(roll: f64, pitch: f64, yaw: f64) -> Self {
        Rotation { roll, pitch, yaw }
    }

    pub fn zero() -> Self {
        Rotation::new(0.0, 0.0, 0.0)
    }
}

#[derive(Clone, Debug)]
pub struct Bounds3D {
    pub min: Point3D,
    pub max: Point3D,
}

impl Bounds3D {
    pub fn new(min: Point3D, max: Point3D) -> Self {
        Bounds3D { min, max }
    }

    pub fn center(&self) -> Point3D {
        Point3D::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn size(&self) -> Vector3D {
        Vector3D::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }
}
