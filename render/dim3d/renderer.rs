use super::camera::*;
use super::scene::*;
use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CanvasVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub color: String,
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CanvasTriangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CanvasLine {
    pub v1: usize,
    pub v2: usize,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CanvasScene {
    pub vertices: Vec<CanvasVertex>,
    pub triangles: Vec<CanvasTriangle>,
    pub lines: Vec<CanvasLine>,
    pub background: String,
    pub camera: CameraData,
    pub viewport: ViewportData,
    pub mesh_visibility: Vec<MeshVisibility>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CameraData {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub fov: f64,
    pub aspect_ratio: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ViewportData {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeshVisibility {
    pub name: String,
    pub visible: bool,
    pub index: usize,
}

pub struct Renderer3D;

impl Renderer3D {
    pub fn render_scene(
        scene: &Scene,
        camera: &Camera,
        viewport: &Viewport,
    ) -> CanvasScene {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut lines = Vec::new();
        let mut mesh_visibility = Vec::new();

        let mut vertex_offset = 0;

        for (mesh_idx, mesh) in scene.meshes.iter().enumerate() {
            if !mesh.visible || !scene.is_mesh_active(mesh_idx) {
                continue;
            }

            let mesh_vis = MeshVisibility {
                name: mesh.name.clone(),
                visible: mesh.visible,
                index: mesh_idx,
            };
            mesh_visibility.push(mesh_vis);

            for vertex in &mesh.vertices {
                let projected = Self::project_point(&vertex.point, camera, viewport);
                let canvas_vertex = CanvasVertex {
                    x: projected.0,
                    y: projected.1,
                    z: vertex.point.z,
                    color: vertex.color.to_rgba(),
                    id: vertex
                        .label
                        .clone()
                        .unwrap_or_else(|| format!("v_{}", vertices.len())),
                };
                vertices.push(canvas_vertex);
            }

            for i in (0..mesh.indices.len()).step_by(3) {
                if i + 2 < mesh.indices.len() {
                    triangles.push(CanvasTriangle {
                        v1: mesh.indices[i] + vertex_offset,
                        v2: mesh.indices[i + 1] + vertex_offset,
                        v3: mesh.indices[i + 2] + vertex_offset,
                        color: mesh.vertices[mesh.indices[i]].color.to_rgba(),
                    });
                } else if i + 1 < mesh.indices.len() {
                    lines.push(CanvasLine {
                        v1: mesh.indices[i] + vertex_offset,
                        v2: mesh.indices[i + 1] + vertex_offset,
                        color: mesh.vertices[mesh.indices[i]].color.to_rgba(),
                    });
                }
            }

            vertex_offset += mesh.vertices.len();
        }

        CanvasScene {
            vertices,
            triangles,
            lines,
            background: scene.background.to_rgba(),
            camera: CameraData {
                position: [camera.position.x, camera.position.y, camera.position.z],
                target: [camera.target.x, camera.target.y, camera.target.z],
                fov: camera.fov,
                aspect_ratio: camera.aspect_ratio,
            },
            viewport: ViewportData {
                width: viewport.width,
                height: viewport.height,
            },
            mesh_visibility,
        }
    }

    fn project_point(point: &Point3D, camera: &Camera, viewport: &Viewport) -> (f64, f64) {
        let dx = point.x - camera.position.x;
        let dy = point.y - camera.position.y;
        let dz = point.z - camera.position.z;

        let depth = (dx * dx + dy * dy + dz * dz).sqrt();

        if depth <= 0.0 {
            return (viewport.width / 2.0, viewport.height / 2.0);
        }

        let fov_rad = camera.fov.to_radians();
        let scale = (fov_rad / 2.0).tan();

        let projected_x = (dx / (dz * scale + 0.1)) * viewport.width / 2.0 + viewport.width / 2.0;
        let projected_y =
            (dy / (dz * scale + 0.1)) * viewport.height / 2.0 + viewport.height / 2.0;

        (projected_x, projected_y)
    }

    pub fn to_json(canvas_scene: &CanvasScene) -> Result<String, serde_json::Error> {
        serde_json::to_string(canvas_scene)
    }

    pub fn to_svg(canvas_scene: &CanvasScene) -> String {
        let width = 800.0;
        let height = 600.0;

        let mut svg = String::with_capacity(16384);
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            width as i32, height as i32, width as i32, height as i32
        ));

        svg.push_str(r#"<defs><style>text{font-family:Arial;font-size:12px;fill:#333}.mesh{stroke:#2196F3;stroke-width:1.5;fill:rgba(33,150,243,0.2)}</style></defs>"#);
        svg.push_str(&format!(r#"<rect width="{}" height="{}" fill="white"/>"#, width as i32, height as i32));
        svg.push_str(r#"<text x="10" y="25" font-weight="bold">SeraPlot 3D Visualization</text>"#);

        let cx = width / 2.0;
        let cy = height / 2.0;
        let scale = 100.0;

        for vertex in &canvas_scene.vertices {
            let x = cx + vertex.x * scale;
            let y = cy + vertex.y * scale;
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="2" fill="{}"/>"#,
                x as i32, y as i32, vertex.color
            ));
        }

        for line in &canvas_scene.lines {
            if line.v1 < canvas_scene.vertices.len() && line.v2 < canvas_scene.vertices.len() {
                let v1 = &canvas_scene.vertices[line.v1];
                let v2 = &canvas_scene.vertices[line.v2];
                let x1 = cx + v1.x * scale;
                let y1 = cy + v1.y * scale;
                let x2 = cx + v2.x * scale;
                let y2 = cy + v2.y * scale;
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1.5"/>"#,
                    x1 as i32, y1 as i32, x2 as i32, y2 as i32, line.color
                ));
            }
        }

        for triangle in &canvas_scene.triangles {
            if triangle.v1 < canvas_scene.vertices.len() 
                && triangle.v2 < canvas_scene.vertices.len()
                && triangle.v3 < canvas_scene.vertices.len() {
                let v1 = &canvas_scene.vertices[triangle.v1];
                let v2 = &canvas_scene.vertices[triangle.v2];
                let v3 = &canvas_scene.vertices[triangle.v3];
                let x1 = cx + v1.x * scale;
                let y1 = cy + v1.y * scale;
                let x2 = cx + v2.x * scale;
                let y2 = cy + v2.y * scale;
                let x3 = cx + v3.x * scale;
                let y3 = cy + v3.y * scale;
                svg.push_str(&format!(
                    r#"<polygon points="{},{} {},{} {},{}" class="mesh" fill="{}"/>"#,
                    x1 as i32, y1 as i32, x2 as i32, y2 as i32, x3 as i32, y3 as i32, triangle.color
                ));
            }
        }
        svg.push_str("</svg>");
        svg
    }

    pub fn to_html(canvas_scene: &CanvasScene) -> String {
        format!(
            "<html><head><title>3D Scene</title></head><body>{}</body></html>",
            Self::to_svg(canvas_scene)
        )
    }
}