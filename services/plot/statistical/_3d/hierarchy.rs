use super::super::bar::Bar3DBlock;
use std::f64::consts::{PI, TAU};

pub const HOLE: f64 = 0.7;
pub const RING_STEP: f64 = 0.95;
pub const RING_THICK: f64 = 0.42;
pub const ARC_STEP: f64 = 0.16;
pub const ROW_STEP: f64 = 1.1;
pub const ROW_THICK: f64 = 0.44;
pub const WIDTH: f64 = 12.0;
pub const BASE_HEIGHT: f64 = 1.4;
pub const FADE: f64 = 0.16;
pub const FADE_FLOOR: f64 = 0.3;
pub const MIN_SPAN: f64 = 1e-4;

pub fn depth_cap(depth: &[usize], cap: usize) -> usize {
    let max_d = depth.iter().copied().max().unwrap_or(0);
    let mut chosen = 0;
    for candidate in 0..=max_d {
        let count = depth.iter().filter(|&&d| d <= candidate).count();
        if count > cap {
            break;
        }
        chosen = candidate;
    }
    chosen
}

pub fn faded_height(depth: usize, base: f64) -> f64 {
    base * (1.0 - depth as f64 * FADE).max(FADE_FLOOR)
}

pub fn rings(
    order: &[usize],
    depth: &[usize],
    span_radians: &[(f64, f64)],
    cap_depth: usize,
    hole: f64,
    height_of: impl Fn(usize) -> f64,
    tone_of: impl Fn(usize) -> f64,
) -> Vec<Bar3DBlock> {
    order
        .iter()
        .copied()
        .filter(|&i| depth[i] <= cap_depth && span_radians[i].1 - span_radians[i].0 > MIN_SPAN)
        .flat_map(|i| {
            let (a0, a1) = span_radians[i];
            let r = hole + RING_STEP * (depth[i] as f64 + 0.5);
            let steps = (((a1 - a0).abs() / ARC_STEP).ceil() as usize).max(1);
            let height = height_of(i);
            let tone = tone_of(i);
            (0..steps).map(move |k| {
                let t = a0 + (a1 - a0) * (k as f64 + 0.5) / steps as f64;
                Bar3DBlock::new(r * t.cos(), r * t.sin(), 0.0, height, RING_THICK, RING_THICK, i).with_tone(tone)
            })
        })
        .collect()
}

pub fn tiers(
    order: &[usize],
    depth: &[usize],
    xspan: &[(f64, f64)],
    cap_depth: usize,
    height_of: impl Fn(usize) -> f64,
    tone_of: impl Fn(usize) -> f64,
) -> Vec<Bar3DBlock> {
    order
        .iter()
        .copied()
        .filter(|&i| depth[i] <= cap_depth && xspan[i].1 - xspan[i].0 > MIN_SPAN)
        .map(|i| {
            let (x0, x1) = xspan[i];
            let (lo, hi) = (x0.min(x1) * WIDTH, x0.max(x1) * WIDTH);
            let row = depth[i] as f64 * ROW_STEP;
            Bar3DBlock::new((lo + hi) / 2.0, row, 0.0, height_of(i), (hi - lo) / 2.0, ROW_THICK, i).with_tone(tone_of(i))
        })
        .collect()
}

pub fn radians_from_unit(span: (f64, f64)) -> (f64, f64) {
    let base = -PI / 2.0;
    (base + span.0 * TAU, base + span.1 * TAU)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(n: usize) -> (Vec<usize>, Vec<usize>) {
        ((0..n).collect(), (0..n).collect())
    }

    #[test]
    fn depth_cap_grows_while_the_running_count_still_fits_the_budget() {
        let depth = vec![0, 1, 1, 2, 2, 2, 2];
        assert_eq!(depth_cap(&depth, 100), 2);
        assert_eq!(depth_cap(&depth, 3), 1);
        assert_eq!(depth_cap(&depth, 1), 0);
        assert_eq!(depth_cap(&depth, 0), 0);
    }

    #[test]
    fn faded_height_shrinks_with_depth_but_never_below_the_floor() {
        assert_eq!(faded_height(0, 10.0), 10.0);
        assert!(faded_height(1, 10.0) < 10.0);
        assert!(faded_height(50, 10.0) >= 10.0 * FADE_FLOOR);
    }

    #[test]
    fn rings_place_one_arc_of_blocks_per_node_at_a_radius_that_grows_with_depth() {
        let (order, depth) = chain(2);
        let span = vec![(0.0, std::f64::consts::PI), (0.0, std::f64::consts::PI / 2.0)];
        let out = rings(&order, &vec![0, 1], &span, 5, HOLE, |_| BASE_HEIGHT, |_| 0.5);
        assert!(out.iter().filter(|b| b.z1 == BASE_HEIGHT).count() == out.len());
        let (near, far): (Vec<&Bar3DBlock>, Vec<&Bar3DBlock>) = out.iter().partition(|b| b.ci == 0);
        let _ = order;
        assert!(!near.is_empty() && !far.is_empty());
        let r_near = near[0].cx.hypot(near[0].cy);
        let r_far = far[0].cx.hypot(far[0].cy);
        assert!(r_far > r_near);
    }

    #[test]
    fn depth_beyond_the_cap_is_skipped_and_zero_span_nodes_draw_nothing() {
        let order = vec![0, 1];
        let depth = vec![0, 1];
        let span = vec![(0.0, 1.0), (0.0, 1.0)];
        assert!(rings(&order, &depth, &span, 0, HOLE, |_| 1.0, |_| 0.0).iter().all(|b| b.ci == 0));
        let zero = vec![(0.0, 0.0)];
        assert!(rings(&[0], &[0], &zero, 5, HOLE, |_| 1.0, |_| 0.0).is_empty());
    }

    #[test]
    fn tiers_stack_one_flat_rectangle_per_node_with_row_by_depth_and_width_by_span() {
        let order = vec![0, 1];
        let depth = vec![0, 1];
        let xspan = vec![(0.0, 1.0), (0.25, 0.75)];
        let out = tiers(&order, &depth, &xspan, 5, |_| BASE_HEIGHT, |_| 0.2);
        assert_eq!(out.len(), 2);
        assert!(out[0].cy < out[1].cy);
        assert_eq!(out[1].hw, 0.25 * WIDTH);
    }

    #[test]
    fn radians_from_unit_starts_at_the_top_and_sweeps_a_full_turn() {
        let (a0, a1) = radians_from_unit((0.0, 1.0));
        assert!((a1 - a0 - TAU).abs() < 1e-9);
        assert_eq!(a0, -PI / 2.0);
    }
}
