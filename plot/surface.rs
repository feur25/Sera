use super::camera::{Camera3D, Point3D, Point2D};

pub struct Surface3D {
    vertices: Vec<Point3D>,
    faces: Vec<[usize; 3]>,
    colors: Vec<egui::Color32>,
}

impl Surface3D {
    pub fn from_grid(x_data: &[f32], y_data: &[f32], z_data: &[Vec<f32>]) -> Self {
        let cols = x_data.len();
        let rows = y_data.len();
        let mut vertices = Vec::new();
        let mut colors = Vec::new();
        let mut z_min = f32::INFINITY;
        let mut z_max = f32::NEG_INFINITY;

        for (j, &y) in y_data.iter().enumerate() {
            for (i, &x) in x_data.iter().enumerate() {
                let z = z_data[j][i];
                z_min = z_min.min(z);
                z_max = z_max.max(z);
                vertices.push(Point3D::new(x, y, z));
            }
        }

        let z_range = if (z_max - z_min).abs() < 0.001 { 1.0 } else { z_max - z_min };

        for vertex in &vertices {
            let normalized = (vertex.z - z_min) / z_range;
            colors.push(Self::colorscale_viridis(normalized));
        }

        let mut faces = Vec::new();
        for j in 0..rows - 1 {
            for i in 0..cols - 1 {
                let v0 = j * cols + i;
                let v1 = j * cols + (i + 1);
                let v2 = (j + 1) * cols + i;
                let v3 = (j + 1) * cols + (i + 1);

                faces.push([v0, v1, v2]);
                faces.push([v1, v3, v2]);
            }
        }

        Self { vertices, faces, colors }
    }

    fn colorscale_viridis(t: f32) -> egui::Color32 {
        let t = t.clamp(0.0, 1.0);
        let r = (0.993728 * (1.0 - t) * (1.0 - t) * (1.0 - t) + 0.906157 * (1.0 - t) * (1.0 - t) * t + 0.143280 * (1.0 - t) * t * t + 0.00332947 * t * t * t) * 255.0;
        let g = (0.0063770 * (1.0 - t) * (1.0 - t) * (1.0 - t) + 0.769443 * (1.0 - t) * (1.0 - t) * t + 0.331746 * (1.0 - t) * t * t + 0.805091 * t * t * t) * 255.0;
        let b = (0.567649 * (1.0 - t) * (1.0 - t) * (1.0 - t) + 0.158671 * (1.0 - t) * (1.0 - t) * t + 0.889177 * (1.0 - t) * t * t + 0.999558 * t * t * t) * 255.0;
        egui::Color32::from_rgb(r as u8, g as u8, b as u8)
    }

    pub fn render(&self, painter: &egui::Painter, camera: &Camera3D, center: egui::Pos2) {
        let mut projected_faces: Vec<_> = self.faces.iter()
            .map(|&[v0, v1, v2]| {
                let p0 = camera.project(self.vertices[v0]);
                let p1 = camera.project(self.vertices[v1]);
                let p2 = camera.project(self.vertices[v2]);
                let depth = (self.vertices[v0].z + self.vertices[v1].z + self.vertices[v2].z) / 3.0;
                let color = (self.colors[v0].r() as u32 + self.colors[v1].r() as u32 + self.colors[v2].r() as u32) / 3;
                ((p0, p1, p2), depth, egui::Color32::from_rgb(color as u8, 100, 150))
            })
            .collect();

        projected_faces.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        for ((p0, p1, p2), _, color) in projected_faces {
            let screen0 = egui::pos2(center.x + p0.x, center.y - p0.y);
            let screen1 = egui::pos2(center.x + p1.x, center.y - p1.y);
            let screen2 = egui::pos2(center.x + p2.x, center.y - p2.y);

            painter.line_segment([screen0, screen1], egui::Stroke::new(0.5, color));
            painter.line_segment([screen1, screen2], egui::Stroke::new(0.5, color));
            painter.line_segment([screen2, screen0], egui::Stroke::new(0.5, color));
        }
    }
}

pub struct Heatmap3D {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<Vec<f32>>,
    colorscale: fn(f32) -> egui::Color32,
}

impl Heatmap3D {
    pub fn new(x: Vec<f32>, y: Vec<f32>, z: Vec<Vec<f32>>) -> Self {
        Self {
            x,
            y,
            z,
            colorscale: Self::colorscale_rdylbu,
        }
    }

    pub fn with_colorscale(mut self, scale: fn(f32) -> egui::Color32) -> Self {
        self.colorscale = scale;
        self
    }

    pub fn colorscale_rdylbu(t: f32) -> egui::Color32 {
        let t = t.clamp(0.0, 1.0);
        if t < 0.5 {
            let s = t * 2.0;
            let r = 215;
            let g = ((48.0 + (221.0 - 48.0) * s) as u8);
            let b = ((48.0 + (55.0 - 48.0) * s) as u8);
            egui::Color32::from_rgb(r, g, b)
        } else {
            let s = (t - 0.5) * 2.0;
            let r = ((215.0 * (1.0 - s) + 33.0 * s) as u8);
            let g = ((48.0 + (166.0 - 48.0) * (1.0 - s)) as u8);
            let b = ((48.0 + (181.0 - 48.0) * (1.0 - s)) as u8);
            egui::Color32::from_rgb(r, g, b)
        }
    }

    pub fn render(&self, painter: &egui::Painter, camera: &Camera3D, center: egui::Pos2) {
        let mut z_min = f32::INFINITY;
        let mut z_max = f32::NEG_INFINITY;

        for row in &self.z {
            for &val in row {
                z_min = z_min.min(val);
                z_max = z_max.max(val);
            }
        }

        let z_range = if (z_max - z_min).abs() < 0.001 { 1.0 } else { z_max - z_min };

        let cols = self.x.len();
        let rows = self.y.len();

        for j in 0..rows - 1 {
            for i in 0..cols - 1 {
                let x0 = self.x[i];
                let x1 = self.x[i + 1];
                let y0 = self.y[j];
                let y1 = self.y[j + 1];
                let z0 = self.z[j][i];
                let z1 = self.z[j][i + 1];
                let z2 = self.z[j + 1][i];
                let z3 = self.z[j + 1][i + 1];

                let p0 = camera.project(Point3D::new(x0, y0, z0));
                let p1 = camera.project(Point3D::new(x1, y0, z1));
                let p2 = camera.project(Point3D::new(x0, y1, z2));
                let p3 = camera.project(Point3D::new(x1, y1, z3));

                let avg_z = (z0 + z1 + z2 + z3) / 4.0;
                let normalized = (avg_z - z_min) / z_range;
                let color = (self.colorscale)(normalized);

                let s0 = egui::pos2(center.x + p0.x, center.y - p0.y);
                let s1 = egui::pos2(center.x + p1.x, center.y - p1.y);
                let s2 = egui::pos2(center.x + p2.x, center.y - p2.y);
                let s3 = egui::pos2(center.x + p3.x, center.y - p3.y);

                painter.line_segment([s0, s1], egui::Stroke::new(0.5, color));
                painter.line_segment([s1, s3], egui::Stroke::new(0.5, color));
                painter.line_segment([s3, s2], egui::Stroke::new(0.5, color));
                painter.line_segment([s2, s0], egui::Stroke::new(0.5, color));
            }
        }
    }
}

pub struct Mesh3D {
    vertices: Vec<Point3D>,
    faces: Vec<[usize; 3]>,
    vertex_colors: Vec<egui::Color32>,
}

impl Mesh3D {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            faces: Vec::new(),
            vertex_colors: Vec::new(),
        }
    }

    pub fn add_vertex(mut self, v: Point3D, color: egui::Color32) -> Self {
        self.vertices.push(v);
        self.vertex_colors.push(color);
        self
    }

    pub fn add_face(mut self, v0: usize, v1: usize, v2: usize) -> Self {
        self.faces.push([v0, v1, v2]);
        self
    }

    pub fn render(&self, painter: &egui::Painter, camera: &Camera3D, center: egui::Pos2) {
        let mut projected: Vec<_> = self.faces.iter()
            .map(|&[v0, v1, v2]| {
                let p0 = camera.project(self.vertices[v0]);
                let p1 = camera.project(self.vertices[v1]);
                let p2 = camera.project(self.vertices[v2]);
                let depth = (self.vertices[v0].z + self.vertices[v1].z + self.vertices[v2].z) / 3.0;
                let c0 = self.vertex_colors[v0];
                ((p0, p1, p2), depth, c0)
            })
            .collect();

        projected.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        for ((p0, p1, p2), _, color) in projected {
            let s0 = egui::pos2(center.x + p0.x, center.y - p0.y);
            let s1 = egui::pos2(center.x + p1.x, center.y - p1.y);
            let s2 = egui::pos2(center.x + p2.x, center.y - p2.y);

            painter.line_segment([s0, s1], egui::Stroke::new(1.0, color));
            painter.line_segment([s1, s2], egui::Stroke::new(1.0, color));
            painter.line_segment([s2, s0], egui::Stroke::new(1.0, color));
        }
    }
}
