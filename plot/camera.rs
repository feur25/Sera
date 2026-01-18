#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PerspectiveAxis {
    XY,
    XZ,
    YZ,
}

pub struct Camera3D {
    pub perspective: PerspectiveAxis,
    pub depth_scale: f32,
    pub angle_offset: f32,
}

impl Camera3D {
    pub fn new() -> Self {
        Self {
            perspective: PerspectiveAxis::XY,
            depth_scale: 0.15,
            angle_offset: 30.0,
        }
    }

    pub fn xy() -> Self {
        Self {
            perspective: PerspectiveAxis::XY,
            depth_scale: 0.15,
            angle_offset: 30.0,
        }
    }

    pub fn xz() -> Self {
        Self {
            perspective: PerspectiveAxis::XZ,
            depth_scale: 0.12,
            angle_offset: 25.0,
        }
    }

    pub fn yz() -> Self {
        Self {
            perspective: PerspectiveAxis::YZ,
            depth_scale: 0.18,
            angle_offset: 35.0,
        }
    }

    pub fn next_perspective(&mut self) {
        self.perspective = match self.perspective {
            PerspectiveAxis::XY => PerspectiveAxis::XZ,
            PerspectiveAxis::XZ => PerspectiveAxis::YZ,
            PerspectiveAxis::YZ => PerspectiveAxis::XY,
        };
    }

    pub fn back_offset(&self, depth: f32) -> (f32, f32) {
        match self.perspective {
            PerspectiveAxis::XY => (depth * 0.6, depth * 0.3),
            PerspectiveAxis::XZ => (depth * 0.5, -depth * 0.4),
            PerspectiveAxis::YZ => (depth * 0.7, depth * 0.25),
        }
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::new()
    }
}
