#[derive(Clone, Copy, Debug)]
pub struct Bar3DBlock {
    pub cx: f64,
    pub cy: f64,
    pub z0: f64,
    pub z1: f64,
    pub hw: f64,
    pub hd: f64,
    pub ci: usize,
    pub tone: Option<f64>,
    pub end: Option<(f64, f64)>,
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
            end: None,
        }
    }

    pub fn sloped(cx: f64, cy: f64, start: (f64, f64), end: (f64, f64), hw: f64, hd: f64, ci: usize) -> Self {
        Self::new(cx, cy, start.0, start.1, hw, hd, ci).with_end(end)
    }

    pub fn with_tone(mut self, tone: f64) -> Self {
        self.tone = Some(tone.clamp(0.0, 1.0));
        self
    }

    pub fn with_end(mut self, end: (f64, f64)) -> Self {
        self.end = Some(end);
        self
    }

    pub fn z_range(&self) -> (f64, f64) {
        let (e0, e1) = self.end.unwrap_or((self.z0, self.z1));
        (self.z0.min(self.z1).min(e0).min(e1), self.z0.max(self.z1).max(e0).max(e1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sloped_block_keeps_both_end_profiles_and_reports_the_full_height_range() {
        let block = Bar3DBlock::sloped(0.5, 0.0, (1.0, 2.0), (3.0, 5.0), 0.5, 0.2, 0);
        assert_eq!(block.end, Some((3.0, 5.0)));
        assert_eq!(block.z_range(), (1.0, 5.0));
        assert_eq!(Bar3DBlock::new(0.0, 0.0, 4.0, 2.0, 0.1, 0.1, 0).z_range(), (2.0, 4.0));
    }
}
