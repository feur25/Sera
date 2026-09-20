#[derive(Clone, Copy)]
pub struct Bar3DBlock {
    pub cx: f64,
    pub cy: f64,
    pub z0: f64,
    pub z1: f64,
    pub hw: f64,
    pub hd: f64,
    pub ci: usize,
    pub tone: Option<f64>,
}

impl Bar3DBlock {
    pub fn new(cx: f64, cy: f64, z0: f64, z1: f64, hw: f64, hd: f64, ci: usize) -> Self {
        Self {
            cx,
            cy,
            z0,
            z1,
            hw,
            hd,
            ci,
            tone: None,
        }
    }

    pub fn with_tone(mut self, tone: f64) -> Self {
        self.tone = Some(tone.clamp(0.0, 1.0));
        self
    }
}
