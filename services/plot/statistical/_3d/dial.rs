use super::super::bar::Bar3DBlock;
use std::f64::consts::{PI, TAU};

pub const RADIUS: f64 = 3.2;
pub const TRACK_H: f64 = 0.5;
pub const FILL_H: f64 = 1.2;
pub const THIN_H: f64 = 0.7;
pub const HALO_H: f64 = 0.18;
pub const DEPTH: f64 = 0.5;
pub const COUNT: usize = 40;
pub const SEG_COUNT: usize = 14;
pub const CONTINUOUS: f64 = 0.62;
pub const CHUNKY: f64 = 0.34;
pub const HALO_SPREAD: f64 = 1.6;
pub const NEEDLE_STEPS: usize = 9;
pub const NEEDLE_REACH: f64 = 0.72;
pub const NEEDLE_SIZE: f64 = 0.14;
pub const HUB_HW: f64 = 0.3;
pub const TICK_COUNT: usize = 20;
pub const TICK_SHORT: f64 = 0.18;
pub const TICK_LONG: f64 = 0.34;
pub const TICK_SIZE: f64 = 0.055;
pub const INNER_STEP: f64 = 0.62;

pub mod tone {
    pub const SAFE: f64 = 0.0;
    pub const WARN: f64 = 0.1;
    pub const DANGER: f64 = 0.2;
    pub const TRACK: f64 = 0.3;
    pub const NEEDLE: f64 = 0.4;
    pub const COMPARE: f64 = 0.5;
}

#[derive(Clone, Copy, Debug)]
pub struct Sweep {
    pub start: f64,
    pub span: f64,
}

impl Sweep {
    pub fn half() -> Self {
        Self { start: PI, span: PI }
    }

    pub fn arc270() -> Self {
        Self { start: PI + PI / 4.0, span: PI * 1.5 }
    }

    pub fn full() -> Self {
        Self { start: PI, span: TAU }
    }

    fn angle(&self, frac: f64) -> f64 {
        self.start - self.span * frac.clamp(0.0, 1.0)
    }
}

pub fn band_index(thresholds: &[(f64, u32)], frac: f64) -> usize {
    let mut idx = 0;
    for (i, &(t, _)) in thresholds.iter().enumerate() {
        if frac >= t {
            idx = i;
        }
    }
    idx
}

pub fn band_tone(count: usize, index: usize) -> f64 {
    let last = count.max(2) - 1;
    tone::SAFE + (tone::DANGER - tone::SAFE) * index.min(last) as f64 / last as f64
}

pub fn fill_tone(thresholds: &[(f64, u32)], frac: f64) -> f64 {
    band_tone(thresholds.len().max(1), band_index(thresholds, frac))
}

fn positions(count: usize) -> impl Iterator<Item = f64> {
    (0..count).map(move |k| (k as f64 + 0.5) / count as f64)
}

pub fn wedges(
    sweep: Sweep,
    count: usize,
    radius: f64,
    height: f64,
    fill_factor: f64,
    depth: f64,
    class: usize,
    tone_of: impl Fn(f64) -> Option<f64>,
) -> Vec<Bar3DBlock> {
    let step = radius * sweep.span.abs() / count.max(1) as f64;
    let thick = (step * fill_factor).max(1e-4);
    positions(count)
        .filter_map(|pos| {
            let tone = tone_of(pos)?;
            let a = sweep.angle(pos);
            Some(Bar3DBlock::new(radius * a.cos(), radius * a.sin(), 0.0, height, thick, depth, class).with_tone(tone))
        })
        .collect()
}

pub fn track(sweep: Sweep, thresholds: &[(f64, u32)], banded: bool, class: usize) -> Vec<Bar3DBlock> {
    wedges(sweep, COUNT, RADIUS, TRACK_H, CONTINUOUS, DEPTH, class, |pos| {
        Some(if banded { band_tone(thresholds.len().max(1), band_index(thresholds, pos)) } else { tone::TRACK })
    })
}

pub fn fill(sweep: Sweep, frac: f64, thresholds: &[(f64, u32)], height: f64, class: usize) -> Vec<Bar3DBlock> {
    let tone = fill_tone(thresholds, frac);
    wedges(sweep, COUNT, RADIUS, height, CONTINUOUS, DEPTH, class, move |pos| (pos <= frac).then_some(tone))
}

pub fn segmented(sweep: Sweep, frac: f64, thresholds: &[(f64, u32)], class: usize) -> Vec<Bar3DBlock> {
    let tone = fill_tone(thresholds, frac);
    let mut blocks = wedges(sweep, SEG_COUNT, RADIUS, TRACK_H, CHUNKY, DEPTH, class, |_| Some(tone::TRACK));
    blocks.extend(wedges(sweep, SEG_COUNT, RADIUS, FILL_H, CHUNKY, DEPTH, class, move |pos| (pos <= frac).then_some(tone)));
    blocks
}

pub fn halo(sweep: Sweep, frac: f64, thresholds: &[(f64, u32)], class: usize) -> Vec<Bar3DBlock> {
    let tone = fill_tone(thresholds, frac);
    wedges(sweep, COUNT, RADIUS, HALO_H, HALO_SPREAD, DEPTH * HALO_SPREAD, class, move |pos| (pos <= frac).then_some(tone))
}

pub fn inner_ring(sweep: Sweep, frac: f64, class: usize) -> Vec<Bar3DBlock> {
    wedges(sweep, COUNT, RADIUS * INNER_STEP, THIN_H, CONTINUOUS, DEPTH * INNER_STEP, class, move |pos| {
        (pos <= frac).then_some(tone::COMPARE)
    })
}

pub fn needle(sweep: Sweep, frac: f64, class: usize) -> Vec<Bar3DBlock> {
    let a = sweep.angle(frac);
    (0..NEEDLE_STEPS)
        .map(|k| {
            let r = RADIUS * NEEDLE_REACH * (k as f64 + 1.0) / NEEDLE_STEPS as f64;
            Bar3DBlock::new(r * a.cos(), r * a.sin(), 0.0, FILL_H * 1.15, NEEDLE_SIZE, NEEDLE_SIZE, class).with_tone(tone::NEEDLE)
        })
        .collect()
}

pub fn hub(class: usize) -> Bar3DBlock {
    Bar3DBlock::new(0.0, 0.0, 0.0, TRACK_H, HUB_HW, HUB_HW, class).with_tone(tone::NEEDLE)
}

pub fn ticks(sweep: Sweep, class: usize) -> Vec<Bar3DBlock> {
    (0..=TICK_COUNT)
        .map(|k| {
            let pos = k as f64 / TICK_COUNT as f64;
            let a = sweep.angle(pos);
            let len = if k % 5 == 0 { TICK_LONG } else { TICK_SHORT };
            let r = RADIUS + len;
            Bar3DBlock::new(r * a.cos(), r * a.sin(), 0.0, TRACK_H, TICK_SIZE, TICK_SIZE, class).with_tone(tone::TRACK)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BANDS: [(f64, u32); 3] = [(0.0, 0), (0.6, 0), (0.8, 0)];

    #[test]
    fn band_index_matches_the_2d_color_for_lookup() {
        assert_eq!(band_index(&BANDS, 0.0), 0);
        assert_eq!(band_index(&BANDS, 0.59), 0);
        assert_eq!(band_index(&BANDS, 0.6), 1);
        assert_eq!(band_index(&BANDS, 0.95), 2);
    }

    #[test]
    fn band_tone_spans_from_safe_to_danger_across_the_band_count() {
        assert_eq!(band_tone(3, 0), tone::SAFE);
        assert_eq!(band_tone(3, 1), tone::WARN);
        assert_eq!(band_tone(3, 2), tone::DANGER);
        assert_eq!(band_tone(1, 0), tone::SAFE);
    }

    #[test]
    fn the_track_ring_covers_every_position_and_the_fill_ring_stops_at_frac() {
        let sweep = Sweep::half();
        assert_eq!(track(sweep, &BANDS, true, 0).len(), COUNT);
        let filled = fill(sweep, 0.3, &BANDS, FILL_H, 0);
        assert!(filled.len() < COUNT && !filled.is_empty());
        assert!(filled.iter().all(|b| b.tone == Some(tone::SAFE)));
    }

    #[test]
    fn a_half_sweep_stays_in_the_upper_half_plane_and_a_full_sweep_wraps_the_circle() {
        let half = track(Sweep::half(), &BANDS, false, 0);
        assert!(half.iter().all(|b| b.cy >= -1e-9));
        let full = track(Sweep::full(), &BANDS, false, 0);
        assert!(full.iter().any(|b| b.cy < 0.0));
    }

    #[test]
    fn segmented_rings_combine_a_full_track_with_a_partial_gappier_fill() {
        let chunky = segmented(Sweep::half(), 0.5, &BANDS, 0);
        assert_eq!(chunky.len(), SEG_COUNT + 7);
        assert!(chunky.iter().filter(|b| b.z1 == TRACK_H).count() == SEG_COUNT);
        assert!(chunky.iter().filter(|b| b.z1 == FILL_H).count() == 7);
        let chunk_slot = RADIUS * Sweep::half().span / SEG_COUNT as f64;
        let smooth_slot = RADIUS * Sweep::half().span / COUNT as f64;
        assert!(chunky[0].hw / chunk_slot < fill(Sweep::half(), 1.0, &BANDS, FILL_H, 0)[0].hw / smooth_slot);
    }

    #[test]
    fn the_needle_climbs_in_radius_towards_the_value_angle_and_the_hub_sits_at_the_centre() {
        let spoke = needle(Sweep::half(), 0.5, 2);
        assert_eq!(spoke.len(), NEEDLE_STEPS);
        assert!(spoke[0].cx.hypot(spoke[0].cy) < spoke[NEEDLE_STEPS - 1].cx.hypot(spoke[NEEDLE_STEPS - 1].cy));
        assert!(spoke.iter().all(|b| b.tone == Some(tone::NEEDLE)));
        let centre = hub(3);
        assert_eq!((centre.cx, centre.cy, centre.ci), (0.0, 0.0, 3));
    }

    #[test]
    fn ticks_alternate_short_and_long_every_fifth_mark() {
        let marks = ticks(Sweep::half(), 0);
        assert_eq!(marks.len(), TICK_COUNT + 1);
        assert!(marks[0].cx.hypot(marks[0].cy) > marks[1].cx.hypot(marks[1].cy));
    }

    #[test]
    fn the_inner_ring_follows_its_own_fraction_independently_of_the_outer_one() {
        let inner = inner_ring(Sweep::half(), 0.4, 1);
        assert!(!inner.is_empty());
        assert!(inner.iter().all(|b| b.cx.hypot(b.cy) < RADIUS));
    }

    #[test]
    fn empty_sweeps_and_zero_fraction_draw_nothing_in_the_fill_ring() {
        assert!(fill(Sweep::half(), 0.0, &BANDS, FILL_H, 0).is_empty());
    }
}
