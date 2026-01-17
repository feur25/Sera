use super::camera::*;

#[derive(Clone, Copy, Debug)]
pub struct InputState {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_dx: f64,
    pub mouse_dy: f64,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub scroll_delta: f64,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
            left_button: false,
            right_button: false,
            middle_button: false,
            scroll_delta: 0.0,
            shift_pressed: false,
            ctrl_pressed: false,
        }
    }

    pub fn update_mouse(&mut self, x: f64, y: f64) {
        self.mouse_dx = x - self.mouse_x;
        self.mouse_dy = y - self.mouse_y;
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn set_buttons(&mut self, left: bool, right: bool, middle: bool) {
        self.left_button = left;
        self.right_button = right;
        self.middle_button = middle;
    }

    pub fn reset_deltas(&mut self) {
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
        self.scroll_delta = 0.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ControlMode {
    Orbit,
    Pan,
    Zoom,
    Free,
}

#[derive(Clone, Copy, Debug)]
pub struct CameraControls {
    pub mode: ControlMode,
    pub sensitivity: f64,
    pub zoom_speed: f64,
    pub pan_speed: f64,
    pub auto_rotate: bool,
    pub auto_rotate_speed: f64,
}

impl CameraControls {
    pub fn new() -> Self {
        CameraControls {
            mode: ControlMode::Orbit,
            sensitivity: 1.0,
            zoom_speed: 1.0,
            pan_speed: 1.0,
            auto_rotate: false,
            auto_rotate_speed: 0.5,
        }
    }

    pub fn orbit_mode(mut self) -> Self {
        self.mode = ControlMode::Orbit;
        self
    }

    pub fn pan_mode(mut self) -> Self {
        self.mode = ControlMode::Pan;
        self
    }

    pub fn zoom_mode(mut self) -> Self {
        self.mode = ControlMode::Zoom;
        self
    }

    pub fn set_sensitivity(mut self, sensitivity: f64) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn apply_input(&self, camera: &mut Camera, input: &InputState) {
        if input.scroll_delta != 0.0 {
            let factor = 1.0 - input.scroll_delta * 0.1 * self.zoom_speed;
            camera.zoom(factor);
        }

        match self.mode {
            ControlMode::Orbit => {
                if input.left_button {
                    let radius = camera.orbit_radius();
                    camera.orbit(
                        input.mouse_dx * self.sensitivity,
                        input.mouse_dy * self.sensitivity,
                        radius,
                    );
                }
            }
            ControlMode::Pan => {
                if input.left_button {
                    camera.pan(
                        input.mouse_dx * self.pan_speed * 0.01,
                        -input.mouse_dy * self.pan_speed * 0.01,
                        0.0,
                    );
                }
            }
            ControlMode::Zoom => {
                if input.left_button {
                    camera.zoom(1.0 + input.mouse_dy * 0.01 * self.zoom_speed);
                }
            }
            ControlMode::Free => {
                if input.left_button {
                    let radius = camera.orbit_radius();
                    camera.orbit(
                        input.mouse_dx * self.sensitivity * 0.5,
                        input.mouse_dy * self.sensitivity * 0.5,
                        radius,
                    );
                }
                if input.right_button {
                    camera.pan(
                        input.mouse_dx * self.pan_speed * 0.01,
                        -input.mouse_dy * self.pan_speed * 0.01,
                        0.0,
                    );
                }
            }
        }

        if self.auto_rotate {
            let radius = camera.orbit_radius();
            camera.orbit(self.auto_rotate_speed, 0.0, radius);
        }
    }
}

impl Default for CameraControls {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct MeshToggle {
    pub mesh_id: usize,
    pub enabled: bool,
}

impl MeshToggle {
    pub fn new(mesh_id: usize) -> Self {
        MeshToggle {
            mesh_id,
            enabled: true,
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

#[derive(Clone, Debug)]
pub struct InteractiveContext {
    pub input: InputState,
    pub controls: CameraControls,
    pub mesh_toggles: Vec<MeshToggle>,
}

impl InteractiveContext {
    pub fn new() -> Self {
        InteractiveContext {
            input: InputState::new(),
            controls: CameraControls::new(),
            mesh_toggles: Vec::new(),
        }
    }

    pub fn add_mesh_toggle(&mut self, mesh_id: usize) {
        self.mesh_toggles.push(MeshToggle::new(mesh_id));
    }

    pub fn toggle_mesh(&mut self, idx: usize) {
        if let Some(toggle) = self.mesh_toggles.get_mut(idx) {
            toggle.toggle();
        }
    }

    pub fn update(&mut self, mouse_x: f64, mouse_y: f64) {
        self.input.update_mouse(mouse_x, mouse_y);
    }

    pub fn reset_frame(&mut self) {
        self.input.reset_deltas();
    }
}

impl Default for InteractiveContext {
    fn default() -> Self {
        Self::new()
    }
}
