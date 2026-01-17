use super::camera::*;
use super::chart::*;
use super::controls::*;
use super::renderer::*;
use super::scene::*;
use super::geometry::*;
use super::types::*;

pub struct Plot3DBuilder {
    scene: Scene,
    camera: Camera,
    viewport: Viewport,
    controls: CameraControls,
    charts: Vec<Chart3D>,
}

impl Plot3DBuilder {
    pub fn new() -> Self {
        Plot3DBuilder {
            scene: Scene::new(),
            camera: Camera::standard(),
            viewport: Viewport::new(1200.0, 800.0),
            controls: CameraControls::new(),
            charts: Vec::new(),
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.viewport = Viewport::new(width, height);
        self.camera.set_aspect_ratio(width, height);
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.scene = self.scene.with_background(color);
        self
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.camera = camera;
        self
    }

    pub fn with_orbit_controls(mut self) -> Self {
        self.controls = CameraControls::new().orbit_mode();
        self
    }

    pub fn with_pan_controls(mut self) -> Self {
        self.controls = CameraControls::new().pan_mode();
        self
    }

    pub fn add_chart(mut self, chart: Chart3D) -> Self {
        self.scene.add_mesh(chart.mesh.clone());
        self.charts.push(chart);
        self
    }

    pub fn add_scatter(
        mut self,
        title: String,
        points: Vec<(f64, f64, f64)>,
        colors: Option<Vec<Color>>,
    ) -> Self {
        let chart = ScatterChart3D::from_series(title, points, colors);
        self.scene.add_mesh(chart.mesh.clone());
        self.charts.push(chart);
        self
    }

    pub fn add_surface(
        mut self,
        title: String,
        width: usize,
        height: usize,
        data: Vec<Vec<f64>>,
        color: Color,
    ) -> Self {
        let chart = SurfaceChart3D::grid(title, width, height, data, color);
        self.scene.add_mesh(chart.mesh.clone());
        self.charts.push(chart);
        self
    }

    pub fn add_line(
        mut self,
        title: String,
        points: Vec<Point3D>,
        color: Color,
    ) -> Self {
        let chart = LineChart3D::from_points(title, points, color);
        self.scene.add_mesh(chart.mesh.clone());
        self.charts.push(chart);
        self
    }

    pub fn add_box(mut self, bbox: Box3D) -> Self {
        self.scene.add_box(bbox);
        self
    }

    pub fn add_sphere(mut self, sphere: Sphere) -> Self {
        self.scene.add_sphere(sphere);
        self
    }

    pub fn add_cylinder(mut self, cylinder: Cylinder) -> Self {
        self.scene.add_cylinder(cylinder);
        self
    }

    pub fn fit_camera(mut self) -> Self {
        if let Some(bounds) = self.scene.bounds() {
            let center = bounds.center();
            self.camera.look_at(center);
            let size = bounds.size();
            let _max_dim = size.x.max(size.y).max(size.z);
            self.camera.zoom(0.8);
        }
        self
    }

    pub fn set_control_sensitivity(mut self, sensitivity: f64) -> Self {
        self.controls.sensitivity = sensitivity;
        self
    }

    pub fn enable_auto_rotate(mut self, speed: f64) -> Self {
        self.controls.auto_rotate = true;
        self.controls.auto_rotate_speed = speed;
        self
    }

    pub fn build(self) -> Plot3D {
        Plot3D {
            scene: self.scene,
            camera: self.camera,
            viewport: self.viewport,
            controls: self.controls,
            charts: self.charts,
            interactive: InteractiveContext::new(),
        }
    }
}

impl Default for Plot3DBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Plot3D {
    pub scene: Scene,
    pub camera: Camera,
    pub viewport: Viewport,
    pub controls: CameraControls,
    pub charts: Vec<Chart3D>,
    pub interactive: InteractiveContext,
}

impl Plot3D {
    pub fn builder() -> Plot3DBuilder {
        Plot3DBuilder::new()
    }

    pub fn render_canvas(&self) -> CanvasScene {
        Renderer3D::render_scene(&self.scene, &self.camera, &self.viewport)
    }

    pub fn render_html(&self) -> String {
        let canvas_scene = self.render_canvas();
        Renderer3D::to_html(&canvas_scene)
    }

    pub fn render_json(&self) -> Result<String, serde_json::Error> {
        let canvas_scene = self.render_canvas();
        Renderer3D::to_json(&canvas_scene)
    }

    pub fn toggle_mesh(&mut self, idx: usize) {
        self.scene.toggle_mesh(idx);
    }

    pub fn toggle_mesh_by_name(&mut self, name: &str) {
        self.scene.toggle_mesh_by_name(name);
    }

    pub fn list_meshes(&self) -> Vec<(usize, String, bool, bool)> {
        self.scene.list_meshes()
    }

    pub fn update_camera(&mut self, input: &InputState) {
        self.controls.apply_input(&mut self.camera, input);
    }

    pub fn camera_front_view(&mut self) {
        self.camera.front_view();
    }

    pub fn camera_top_view(&mut self) {
        self.camera.top_view();
    }

    pub fn camera_side_view(&mut self) {
        self.camera.side_view();
    }

    pub fn camera_isometric_view(&mut self) {
        self.camera.isometric_view();
    }

    pub fn camera_reset(&mut self) {
        self.camera.reset();
    }

    pub fn get_mesh_count(&self) -> usize {
        self.scene.mesh_count()
    }

    pub fn get_visible_mesh_count(&self) -> usize {
        self.scene.visible_mesh_count()
    }
}
