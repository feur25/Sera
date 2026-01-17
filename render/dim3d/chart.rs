use super::geometry::*;
use super::types::*;

#[derive(Clone, Debug)]
pub struct Chart3D {
    pub mesh: Mesh,
    pub chart_type: ChartMode3D,
    pub title: String,
}

impl Chart3D {
    pub fn new(chart_type: ChartMode3D, title: String) -> Self {
        Chart3D {
            mesh: Mesh::new(title.clone()),
            chart_type,
            title,
        }
    }

    pub fn scatter(title: String) -> Self {
        Chart3D::new(ChartMode3D::Scatter, title)
    }

    pub fn surface(title: String) -> Self {
        Chart3D::new(ChartMode3D::Surface, title)
    }

    pub fn line_chart(title: String) -> Self {
        Chart3D::new(ChartMode3D::Line, title)
    }

    pub fn bar_chart(title: String) -> Self {
        Chart3D::new(ChartMode3D::Bar, title)
    }

    pub fn bubble_chart(title: String) -> Self {
        Chart3D::new(ChartMode3D::Bubble, title)
    }

    pub fn wireframe(title: String) -> Self {
        Chart3D::new(ChartMode3D::Wireframe, title)
    }

    pub fn add_point(&mut self, point: Point3D, color: Color) -> usize {
        self.mesh.add_vertex(Vertex3D::new(point, color))
    }

    pub fn add_point_with_label(
        &mut self,
        point: Point3D,
        color: Color,
        label: String,
    ) -> usize {
        self.mesh
            .add_vertex(Vertex3D::new(point, color).with_label(label))
    }

    pub fn add_line(&mut self, i1: usize, i2: usize) {
        self.mesh.add_line(i1, i2);
    }

    pub fn add_triangle(&mut self, i1: usize, i2: usize, i3: usize) {
        self.mesh.add_triangle(i1, i2, i3);
    }

    pub fn vertex_count(&self) -> usize {
        self.mesh.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.mesh.indices.len() / 2
    }

    pub fn face_count(&self) -> usize {
        self.mesh.indices.len() / 3
    }

    pub fn bounds(&self) -> Option<Bounds3D> {
        self.mesh.bounds()
    }
}

pub struct ScatterChart3D;

impl ScatterChart3D {
    pub fn from_series(
        series_name: String,
        points: Vec<(f64, f64, f64)>,
        colors: Option<Vec<Color>>,
    ) -> Chart3D {
        let mut chart = Chart3D::scatter(series_name);

        let colors = colors.unwrap_or_else(|| {
            vec![
                Color::red(),
                Color::blue(),
                Color::green(),
                Color::yellow(),
                Color::cyan(),
                Color::magenta(),
            ]
        });

        for (idx, (x, y, z)) in points.iter().enumerate() {
            let color = colors[idx % colors.len()];
            let point = Point3D::new(*x, *y, *z);
            chart.add_point(point, color);
        }

        chart
    }
}

pub struct SurfaceChart3D;

impl SurfaceChart3D {
    pub fn grid(
        title: String,
        width: usize,
        height: usize,
        data: Vec<Vec<f64>>,
        color: Color,
    ) -> Chart3D {
        let mut chart = Chart3D::surface(title);

        for i in 0..height {
            for j in 0..width {
                if i < data.len() && j < data[i].len() {
                    let x = j as f64;
                    let y = i as f64;
                    let z = data[i][j];
                    chart.add_point(Point3D::new(x, y, z), color);
                }
            }
        }

        for i in 0..height - 1 {
            for j in 0..width - 1 {
                let idx = i * width + j;
                let idx_right = i * width + (j + 1);
                let idx_down = (i + 1) * width + j;
                let idx_diag = (i + 1) * width + (j + 1);

                if idx < chart.mesh.vertices.len()
                    && idx_right < chart.mesh.vertices.len()
                    && idx_down < chart.mesh.vertices.len()
                    && idx_diag < chart.mesh.vertices.len()
                {
                    chart.add_triangle(idx, idx_right, idx_down);
                    chart.add_triangle(idx_right, idx_diag, idx_down);
                }
            }
        }

        chart
    }
}

pub struct LineChart3D;

impl LineChart3D {
    pub fn from_points(
        title: String,
        points: Vec<Point3D>,
        color: Color,
    ) -> Chart3D {
        let mut chart = Chart3D::line_chart(title);

        for point in &points {
            chart.add_point(*point, color);
        }

        for i in 0..points.len() - 1 {
            chart.add_line(i, i + 1);
        }

        chart
    }
}

pub struct BarChart3D;

impl BarChart3D {
    pub fn from_categories(
        title: String,
        categories: Vec<String>,
        values: Vec<f64>,
        colors: Option<Vec<Color>>,
    ) -> Chart3D {
        let mut chart = Chart3D::bar_chart(title);

        let colors = colors.unwrap_or_else(|| {
            vec![Color::red(), Color::blue(), Color::green()]
        });

        for (idx, (_cat, val)) in categories.iter().zip(values.iter()).enumerate() {
            let x = idx as f64;
            let color = colors[idx % colors.len()];

            let base = chart.add_point(Point3D::new(x, 0.0, 0.0), color);
            let top = chart.add_point(Point3D::new(x, *val, 0.0), color);

            chart.add_line(base, top);
        }

        chart
    }
}
