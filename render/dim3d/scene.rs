use super::geometry::*;
use super::types::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub boxes: Vec<Box3D>,
    pub spheres: Vec<Sphere>,
    pub cylinders: Vec<Cylinder>,
    pub background: Color,
    pub ambient_light: f64,
    pub mesh_names: HashMap<String, usize>,
    pub active_meshes: std::collections::HashSet<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            meshes: Vec::new(),
            boxes: Vec::new(),
            spheres: Vec::new(),
            cylinders: Vec::new(),
            background: Color::new(15, 15, 25),
            ambient_light: 0.5,
            mesh_names: HashMap::new(),
            active_meshes: std::collections::HashSet::new(),
        }
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> usize {
        let idx = self.meshes.len();
        self.mesh_names.insert(mesh.name.clone(), idx);
        self.active_meshes.insert(idx);
        self.meshes.push(mesh);
        idx
    }

    pub fn get_mesh(&self, idx: usize) -> Option<&Mesh> {
        self.meshes.get(idx)
    }

    pub fn get_mesh_mut(&mut self, idx: usize) -> Option<&mut Mesh> {
        self.meshes.get_mut(idx)
    }

    pub fn get_mesh_by_name(&self, name: &str) -> Option<&Mesh> {
        self.mesh_names
            .get(name)
            .and_then(|idx| self.meshes.get(*idx))
    }

    pub fn get_mesh_by_name_mut(&mut self, name: &str) -> Option<&mut Mesh> {
        let idx = *self.mesh_names.get(name)?;
        self.meshes.get_mut(idx)
    }

    pub fn toggle_mesh(&mut self, idx: usize) {
        if let Some(mesh) = self.get_mesh_mut(idx) {
            mesh.toggle_visibility();
        }
    }

    pub fn toggle_mesh_by_name(&mut self, name: &str) {
        if let Some(idx) = self.mesh_names.get(name).copied() {
            self.toggle_mesh(idx);
        }
    }

    pub fn set_mesh_visible(&mut self, idx: usize, visible: bool) {
        if let Some(mesh) = self.get_mesh_mut(idx) {
            mesh.set_visible(visible);
        }
    }

    pub fn activate_mesh(&mut self, idx: usize) {
        self.active_meshes.insert(idx);
    }

    pub fn deactivate_mesh(&mut self, idx: usize) {
        self.active_meshes.remove(&idx);
    }

    pub fn is_mesh_active(&self, idx: usize) -> bool {
        self.active_meshes.contains(&idx)
    }

    pub fn add_box(&mut self, bbox: Box3D) {
        self.boxes.push(bbox);
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_cylinder(&mut self, cylinder: Cylinder) {
        self.cylinders.push(cylinder);
    }

    pub fn bounds(&self) -> Option<Bounds3D> {
        let mut bounds: Option<Bounds3D> = None;

        for mesh in &self.meshes {
            if let Some(b) = mesh.bounds() {
                bounds = match bounds {
                    None => Some(b),
                    Some(existing) => Some(Bounds3D {
                        min: Point3D {
                            x: existing.min.x.min(b.min.x),
                            y: existing.min.y.min(b.min.y),
                            z: existing.min.z.min(b.min.z),
                        },
                        max: Point3D {
                            x: existing.max.x.max(b.max.x),
                            y: existing.max.y.max(b.max.y),
                            z: existing.max.z.max(b.max.z),
                        },
                    }),
                };
            }
        }

        bounds
    }

    pub fn clear(&mut self) {
        self.meshes.clear();
        self.boxes.clear();
        self.spheres.clear();
        self.cylinders.clear();
        self.mesh_names.clear();
        self.active_meshes.clear();
    }

    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    pub fn visible_mesh_count(&self) -> usize {
        self.meshes.iter().filter(|m| m.visible).count()
    }

    pub fn active_mesh_count(&self) -> usize {
        self.active_meshes.len()
    }

    pub fn list_meshes(&self) -> Vec<(usize, String, bool, bool)> {
        self.meshes
            .iter()
            .enumerate()
            .map(|(i, m)| {
                (
                    i,
                    m.name.clone(),
                    m.visible,
                    self.is_mesh_active(i),
                )
            })
            .collect()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene::new()
    }
}
