use crate::plot::containers_3d::CameraController;

pub struct AdvancedViewer3D {
    pub camera_controller: CameraController,
    pub last_mouse_pos: Option<egui::Pos2>,
    pub is_dragging: bool,
    pub show_info: bool,
}

impl AdvancedViewer3D {
    pub fn new() -> Self {
        Self {
            camera_controller: CameraController::default(),
            last_mouse_pos: None,
            is_dragging: false,
            show_info: false,
        }
    }

    pub fn handle_input(&mut self, ui: &egui::Ui) {
        let response = ui.interact(ui.available_rect_before_wrap(), ui.id(), egui::Sense::click_and_drag());
        
        if response.drag_started() {
            self.is_dragging = true;
            self.last_mouse_pos = response.interact_pointer_pos();
        }
        
        if self.is_dragging {
            if let Some(curr_pos) = response.interact_pointer_pos() {
                if let Some(last_pos) = self.last_mouse_pos {
                    let delta_x = (curr_pos.x - last_pos.x) * 0.015;
                    let delta_y = (curr_pos.y - last_pos.y) * 0.015;
                    self.camera_controller.rotate_orbit(delta_x, -delta_y);
                }
                self.last_mouse_pos = Some(curr_pos);
            }
        }
        
        if response.drag_stopped() {
            self.is_dragging = false;
            self.last_mouse_pos = None;
        }
        
        let scroll = ui.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            let zoom_factor = if scroll > 0.0 { 0.85 } else { 1.15 };
            self.camera_controller.zoom(zoom_factor);
        }
        
        let mut pan_dx = 0.0;
        let mut pan_dy = 0.0;
        
        ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) {
                pan_dx -= 0.3;
            }
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) {
                pan_dx += 0.3;
            }
            if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W) {
                pan_dy += 0.3;
            }
            if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S) {
                pan_dy -= 0.3;
            }
        });
        
        if pan_dx != 0.0 || pan_dy != 0.0 {
            self.camera_controller.pan(pan_dx, pan_dy);
        }
    }

    pub fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Rotation X").clicked() {
                self.camera_controller.rotate_orbit(0.2, 0.0);
            }
            if ui.button("Rotation Y").clicked() {
                self.camera_controller.rotate_orbit(0.0, 0.2);
            }
            if ui.button("Rotation Z").clicked() {
                self.camera_controller.rotate_orbit(0.2, 0.2);
            }
            
            ui.separator();
            
            if ui.button("Zoom In").clicked() {
                self.camera_controller.zoom(0.8);
            }
            if ui.button("Zoom Out").clicked() {
                self.camera_controller.zoom(1.2);
            }
            if ui.button("↺ Reset").clicked() {
                self.camera_controller.reset();
            }
            
            if ui.button("ℹInfo").clicked() {
                self.show_info = !self.show_info;
            }
        });

        if self.show_info {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Drag: Rotation | Scroll: Zoom | WASD/Arrows: Pan | Orbite: {:.2}° Élévation: {:.2}° | Zoom: {:.2}x | Pan: ({:.2}, {:.2})",
                    self.camera_controller.orbit_yaw.to_degrees(),
                    self.camera_controller.orbit_pitch.to_degrees(),
                    self.camera_controller.zoom,
                    self.camera_controller.pan_x,
                    self.camera_controller.pan_y,
                ));
            });
        }
    }
}

impl Default for AdvancedViewer3D {
    fn default() -> Self {
        Self::new()
    }
}
