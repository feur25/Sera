use super::super::bar::Bar3DBlock;
use super::ohlc::{track_blocks, DOWN, UP};

pub const TOTAL_TONE: f64 = 0.5;

#[derive(Clone, Copy)]
pub struct Step {
    pub start: f64,
    pub end: f64,
    pub total: bool,
}

impl Step {
    pub fn tone(&self) -> f64 {
        match (self.total, self.end >= self.start) {
            (true, _) => TOTAL_TONE,
            (false, true) => UP,
            (false, false) => DOWN,
        }
    }

    pub fn low(&self) -> f64 {
        self.start.min(self.end)
    }

    pub fn high(&self) -> f64 {
        self.start.max(self.end)
    }
}

pub fn span_of(steps: &[Step]) -> f64 {
    let lo = steps.iter().map(Step::low).fold(f64::INFINITY, f64::min).min(0.0);
    let hi = steps.iter().map(Step::high).fold(f64::NEG_INFINITY, f64::max).max(0.0);
    (hi - lo).max(1e-9)
}

pub fn floating_bars(steps: &[Step], hw: f64, depth: f64) -> Vec<Bar3DBlock> {
    let floor = span_of(steps) * 0.004;
    steps
        .iter()
        .enumerate()
        .map(|(i, s)| {
            Bar3DBlock::new(i as f64, 0.0, s.low(), s.high().max(s.low() + floor), hw, depth, i).with_tone(s.tone())
        })
        .collect()
}

pub fn stem_heads(steps: &[Step], stem_hw: f64, head: f64, depth: f64) -> Vec<Bar3DBlock> {
    steps
        .iter()
        .enumerate()
        .flat_map(|(i, s)| {
            [
                Bar3DBlock::new(i as f64, 0.0, s.low(), s.high(), stem_hw, depth * 0.5, i).with_tone(s.tone()),
                Bar3DBlock::new(i as f64, 0.0, s.end - head / 2.0, s.end + head / 2.0, head, head, i).with_tone(s.tone()),
            ]
        })
        .collect()
}

pub fn arrow_tips(steps: &[Step], size: f64) -> Vec<Bar3DBlock> {
    steps
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.total)
        .map(|(i, s)| {
            let (from, to) = if s.end >= s.start { (s.end, s.end + size) } else { (s.end - size, s.end) };
            Bar3DBlock::new(i as f64, 0.0, from, to, size * 0.5, size * 0.5, i).with_tone(s.tone())
        })
        .collect()
}

pub fn running_track(steps: &[Step], row: f64, depth: f64, tone: f64) -> Vec<Bar3DBlock> {
    let ends: Vec<f64> = steps.iter().map(|s| s.end).collect();
    track_blocks(&ends, span_of(steps), depth, row, tone)
}

pub fn from_deltas(values: &[f64], is_total: &[bool]) -> Vec<Step> {
    let mut running = 0.0;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if is_total.get(i).copied().unwrap_or(false) {
                Step { start: 0.0, end: running, total: true }
            } else {
                let start = running;
                running += v;
                Step { start, end: running, total: false }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steps() -> Vec<Step> {
        from_deltas(&[100.0, 30.0, -15.0, 0.0], &[false, false, false, true])
    }

    #[test]
    fn deltas_accumulate_and_totals_reset_to_the_floor() {
        let s = steps();
        assert_eq!((s[1].start, s[1].end), (100.0, 130.0));
        assert_eq!((s[2].start, s[2].end), (130.0, 115.0));
        assert_eq!((s[3].start, s[3].end, s[3].total), (0.0, 115.0, true));
    }

    #[test]
    fn floating_bars_span_each_step_with_a_tone_per_direction() {
        let bars = floating_bars(&steps(), 0.4, 0.4);
        assert_eq!((bars[2].z0, bars[2].z1), (115.0, 130.0));
        assert_eq!(bars[1].tone, Some(UP));
        assert_eq!(bars[2].tone, Some(DOWN));
        assert_eq!(bars[3].tone, Some(TOTAL_TONE));
    }

    #[test]
    fn stems_come_with_a_head_and_tips_skip_totals() {
        assert_eq!(stem_heads(&steps(), 0.05, 0.3, 0.3).len(), 8);
        let tips = arrow_tips(&steps(), 0.3);
        assert_eq!(tips.len(), 3);
        assert!(tips[2].z1 <= 115.0);
    }

    #[test]
    fn the_running_track_follows_every_step_end() {
        assert_eq!(running_track(&steps(), 1.0, 0.1, 0.5).len(), 4);
    }
}
