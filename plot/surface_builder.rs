use crate::plot::surface::{Surface3D, Heatmap3D, Mesh3D};
use crate::plot::camera::Point3D;

pub trait SurfaceGenerator {
    fn generate(&self, x_range: (f32, f32), y_range: (f32, f32), samples: usize) -> (Vec<f32>, Vec<f32>, Vec<Vec<f32>>);
}

pub struct GaussianGenerator {
    amplitude: f32,
}

impl GaussianGenerator {
    pub fn new() -> Self {
        Self { amplitude: 1.0 }
    }

    pub fn with_amplitude(mut self, amp: f32) -> Self {
        self.amplitude = amp;
        self
    }
}

impl SurfaceGenerator for GaussianGenerator {
    fn generate(&self, x_range: (f32, f32), y_range: (f32, f32), samples: usize) -> (Vec<f32>, Vec<f32>, Vec<Vec<f32>>) {
        let x: Vec<f32> = (0..samples).map(|i| x_range.0 + (x_range.1 - x_range.0) * i as f32 / (samples - 1) as f32).collect();
        let y: Vec<f32> = (0..samples).map(|i| y_range.0 + (y_range.1 - y_range.0) * i as f32 / (samples - 1) as f32).collect();
        let z: Vec<Vec<f32>> = y.iter()
            .map(|&yv| {
                x.iter()
                    .map(|&xv| {
                        let r2 = xv * xv + yv * yv;
                        self.amplitude * (-r2 / 4.0).exp() * xv.cos() * yv.sin()
                    })
                    .collect()
            })
            .collect();
        (x, y, z)
    }
}

pub struct RippleGenerator {
    amplitude: f32,
    frequency: f32,
}

impl RippleGenerator {
    pub fn new() -> Self {
        Self { amplitude: 1.0, frequency: 1.0 }
    }

    pub fn with_amplitude(mut self, amp: f32) -> Self {
        self.amplitude = amp;
        self
    }

    pub fn with_frequency(mut self, freq: f32) -> Self {
        self.frequency = freq;
        self
    }
}

impl SurfaceGenerator for RippleGenerator {
    fn generate(&self, x_range: (f32, f32), y_range: (f32, f32), samples: usize) -> (Vec<f32>, Vec<f32>, Vec<Vec<f32>>) {
        let x: Vec<f32> = (0..samples).map(|i| x_range.0 + (x_range.1 - x_range.0) * i as f32 / (samples - 1) as f32).collect();
        let y: Vec<f32> = (0..samples).map(|i| y_range.0 + (y_range.1 - y_range.0) * i as f32 / (samples - 1) as f32).collect();
        let z: Vec<Vec<f32>> = y.iter()
            .map(|&yv| {
                x.iter()
                    .map(|&xv| {
                        let r = (xv * xv + yv * yv).sqrt();
                        self.amplitude * (self.frequency * r).sin() * (-r / 3.0).exp()
                    })
                    .collect()
            })
            .collect();
        (x, y, z)
    }
}

pub struct TorusGenerator;

impl TorusGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl SurfaceGenerator for TorusGenerator {
    fn generate(&self, _x_range: (f32, f32), _y_range: (f32, f32), samples: usize) -> (Vec<f32>, Vec<f32>, Vec<Vec<f32>>) {
        let u: Vec<f32> = (0..samples).map(|i| 2.0 * std::f32::consts::PI * i as f32 / samples as f32).collect();
        let v: Vec<f32> = (0..samples).map(|i| 2.0 * std::f32::consts::PI * i as f32 / samples as f32).collect();

        let mut x = vec![vec![0.0; samples]; samples];
        let mut y = vec![vec![0.0; samples]; samples];
        let mut z = vec![vec![0.0; samples]; samples];

        for (j, &vj) in v.iter().enumerate() {
            for (i, &ui) in u.iter().enumerate() {
                let r = 2.0 + 1.0 * vj.cos();
                x[j][i] = r * ui.cos();
                y[j][i] = r * ui.sin();
                z[j][i] = 1.0 * vj.sin();
            }
        }

        (u, v, z)
    }
}

pub struct Surface3DBuilder {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<Vec<f32>>,
}

impl Surface3DBuilder {
    pub fn from_generator<G: SurfaceGenerator>(gen: &G, x_range: (f32, f32), y_range: (f32, f32), samples: usize) -> Self {
        let (x, y, z) = gen.generate(x_range, y_range, samples);
        Self { x, y, z }
    }

    pub fn with_data(x: Vec<f32>, y: Vec<f32>, z: Vec<Vec<f32>>) -> Self {
        Self { x, y, z }
    }

    pub fn build(self) -> Surface3D {
        Surface3D::from_grid(&self.x, &self.y, &self.z)
    }
}

pub struct Heatmap3DBuilder {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<Vec<f32>>,
    colorscale: fn(f32) -> egui::Color32,
}

impl Heatmap3DBuilder {
    pub fn from_generator<G: SurfaceGenerator>(gen: &G, x_range: (f32, f32), y_range: (f32, f32), samples: usize) -> Self {
        let (x, y, z) = gen.generate(x_range, y_range, samples);
        Self {
            x,
            y,
            z,
            colorscale: Heatmap3D::colorscale_rdylbu,
        }
    }

    pub fn with_colorscale(mut self, scale: fn(f32) -> egui::Color32) -> Self {
        self.colorscale = scale;
        self
    }

    pub fn build(self) -> Heatmap3D {
        Heatmap3D::new(self.x, self.y, self.z).with_colorscale(self.colorscale)
    }
}

pub struct Mesh3DBuilder {
    vertices: Vec<Point3D>,
    vertex_colors: Vec<egui::Color32>,
    faces: Vec<[usize; 3]>,
}

impl Mesh3DBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            vertex_colors: Vec::new(),
            faces: Vec::new(),
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

    pub fn build(self) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        for (v, c) in self.vertices.iter().zip(self.vertex_colors.iter()) {
            mesh = mesh.add_vertex(*v, *c);
        }
        for &face in &self.faces {
            mesh = mesh.add_face(face[0], face[1], face[2]);
        }
        mesh
    }
}
