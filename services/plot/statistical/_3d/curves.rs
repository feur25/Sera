use super::super::bar::Bar3DBlock;

pub type Point = (f64, f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Riser {
    After,
    Before,
    Middle,
}

impl Riser {
    pub fn parse(shape: &str) -> Self {
        match shape {
            "vh" => Riser::Before,
            "hvh" | "vhv" => Riser::Middle,
            _ => Riser::After,
        }
    }
}

pub fn points_of(values: &[f64]) -> Vec<Point> {
    values.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect()
}

pub fn thickness_of(points: &[Point]) -> f64 {
    let (lo, hi) = points
        .iter()
        .map(|p| p.1)
        .filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    if lo.is_finite() { ((hi - lo).max(1e-9)) * 0.012 } else { 1e-9 }
}

pub fn joined_by_jump(points: &[Point], threshold: f64) -> Vec<bool> {
    points
        .windows(2)
        .map(|w| w[0].1.is_finite() && w[1].1.is_finite() && !(threshold.is_finite() && (w[1].1 - w[0].1).abs() > threshold))
        .collect()
}

pub fn ribbon_blocks(points: &[Point], joined: &[bool], row: f64, depth: f64, class: usize, tone: Option<f64>) -> Vec<Bar3DBlock> {
    let half = thickness_of(points);
    points
        .windows(2)
        .enumerate()
        .filter(|(i, w)| joined.get(*i).copied().unwrap_or(true) && w[0].1.is_finite() && w[1].1.is_finite())
        .map(|(_, w)| {
            let block = Bar3DBlock::sloped(
                (w[0].0 + w[1].0) / 2.0,
                row,
                (w[0].1 - half, w[0].1 + half),
                (w[1].1 - half, w[1].1 + half),
                (w[1].0 - w[0].0) / 2.0,
                depth,
                class,
            );
            match tone {
                Some(t) => block.with_tone(t),
                None => block,
            }
        })
        .collect()
}

pub fn toned_ribbon(points: &[Point], tones: &[f64], row: f64, depth: f64, class: usize) -> Vec<Bar3DBlock> {
    let joined = vec![true; points.len().saturating_sub(1)];
    let mut out = ribbon_blocks(points, &joined, row, depth, class, None);
    for (block, tone) in out.iter_mut().zip(tones) {
        block.tone = Some(tone.clamp(0.0, 1.0));
    }
    out
}

pub fn area_blocks(points: &[Point], joined: &[bool], floor: f64, row: f64, depth: f64, class: usize, graded: bool) -> Vec<Bar3DBlock> {
    let (lo, hi) = points
        .iter()
        .map(|p| p.1)
        .filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    let range = (hi - floor.min(lo)).max(1e-9);
    points
        .windows(2)
        .enumerate()
        .filter(|(i, w)| joined.get(*i).copied().unwrap_or(true) && w[0].1.is_finite() && w[1].1.is_finite())
        .map(|(_, w)| {
            let block = Bar3DBlock::sloped(
                (w[0].0 + w[1].0) / 2.0,
                row,
                (floor, w[0].1),
                (floor, w[1].1),
                (w[1].0 - w[0].0) / 2.0,
                depth,
                class,
            );
            if graded {
                block.with_tone((((w[0].1 + w[1].1) / 2.0 - floor) / range).clamp(0.0, 1.0))
            } else {
                block
            }
        })
        .collect()
}

pub fn band_blocks(low: &[Point], high: &[Point], row: f64, depth: f64, class: usize) -> Vec<Bar3DBlock> {
    low.windows(2)
        .zip(high.windows(2))
        .filter(|(l, h)| [l[0].1, l[1].1, h[0].1, h[1].1].iter().all(|v| v.is_finite()))
        .map(|(l, h)| {
            Bar3DBlock::sloped(
                (l[0].0 + l[1].0) / 2.0,
                row,
                (l[0].1, h[0].1),
                (l[1].1, h[1].1),
                (l[1].0 - l[0].0) / 2.0,
                depth,
                class,
            )
        })
        .collect()
}

pub fn step_blocks(points: &[Point], riser: Riser, row: f64, depth: f64, class: usize, tone: Option<f64>) -> Vec<Bar3DBlock> {
    let half = thickness_of(points);
    let paint = |block: Bar3DBlock| match tone {
        Some(t) => block.with_tone(t),
        None => block,
    };
    let tread = |from: f64, to: f64, v: f64| {
        paint(Bar3DBlock::new((from + to) / 2.0, row, v - half, v + half, (to - from) / 2.0, depth, class))
    };
    let rise = |x: f64, a: f64, b: f64| {
        paint(Bar3DBlock::new(x, row, a.min(b) - half, a.max(b) + half, half.max(1e-6), depth, class))
    };
    points
        .windows(2)
        .filter(|w| w[0].1.is_finite() && w[1].1.is_finite())
        .flat_map(|w| {
            let (p, q) = (w[0], w[1]);
            match riser {
                Riser::After => vec![tread(p.0, q.0, p.1), rise(q.0, p.1, q.1)],
                Riser::Before => vec![rise(p.0, p.1, q.1), tread(p.0, q.0, q.1)],
                Riser::Middle => {
                    let mid = (p.0 + q.0) / 2.0;
                    vec![tread(p.0, mid, p.1), rise(mid, p.1, q.1), tread(mid, q.0, q.1)]
                }
            }
        })
        .collect()
}

pub fn dashed_blocks(
    points: &[Point],
    pattern: (usize, usize),
    resolution: usize,
    row: f64,
    depth: f64,
    class: usize,
    tone: Option<f64>,
) -> Vec<Bar3DBlock> {
    let dense = densified(points, resolution.max(1));
    let cycle = (pattern.0 + pattern.1).max(1);
    dense
        .windows(2)
        .enumerate()
        .filter(|(i, _)| i % cycle < pattern.0.max(1))
        .flat_map(|(_, w)| ribbon_blocks(w, &[true], row, depth, class, tone))
        .collect()
}

pub fn densified(points: &[Point], per_segment: usize) -> Vec<Point> {
    if per_segment <= 1 || points.len() < 2 {
        return points.to_vec();
    }
    let mut out = Vec::with_capacity(points.len() * per_segment);
    for w in points.windows(2) {
        for k in 0..per_segment {
            let t = k as f64 / per_segment as f64;
            out.push((w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t));
        }
    }
    out.push(points[points.len() - 1]);
    out
}

pub fn catmull_rom(points: &[Point], per_segment: usize, tension: f64) -> Vec<Point> {
    if points.len() < 3 || per_segment <= 1 || points.iter().any(|p| !p.1.is_finite()) {
        return points.to_vec();
    }
    let k = tension.clamp(0.0, 1.0);
    let at = |i: isize| points[i.clamp(0, points.len() as isize - 1) as usize];
    let mut out = Vec::with_capacity(points.len() * per_segment);
    for i in 0..points.len() - 1 {
        let (p0, p1, p2, p3) = (at(i as isize - 1), at(i as isize), at(i as isize + 1), at(i as isize + 2));
        for s in 0..per_segment {
            let t = s as f64 / per_segment as f64;
            let (t2, t3) = (t * t, t * t * t);
            let h = |a: f64, b: f64, c: f64, d: f64| {
                let (m1, m2) = ((c - a) * k, (d - b) * k);
                (2.0 * t3 - 3.0 * t2 + 1.0) * b + (t3 - 2.0 * t2 + t) * m1 + (-2.0 * t3 + 3.0 * t2) * c + (t3 - t2) * m2
            };
            out.push((h(p0.0, p1.0, p2.0, p3.0), h(p0.1, p1.1, p2.1, p3.1)));
        }
    }
    out.push(points[points.len() - 1]);
    out
}

pub fn marker_blocks(points: &[Point], size_xy: f64, size_z: f64, row: f64, class: usize, tone: Option<f64>) -> Vec<Bar3DBlock> {
    points
        .iter()
        .filter(|p| p.1.is_finite())
        .map(|p| {
            let block = Bar3DBlock::new(p.0, row, p.1 - size_z / 2.0, p.1 + size_z / 2.0, size_xy / 2.0, size_xy / 2.0, class);
            match tone {
                Some(t) => block.with_tone(t),
                None => block,
            }
        })
        .collect()
}

pub fn slope_tones(points: &[Point]) -> Vec<f64> {
    let slopes: Vec<f64> = points
        .windows(2)
        .map(|w| if w[0].1.is_finite() && w[1].1.is_finite() { (w[1].1 - w[0].1) / (w[1].0 - w[0].0).abs().max(1e-12) } else { 0.0 })
        .collect();
    let peak = slopes.iter().fold(1e-12_f64, |m, v| m.max(v.abs()));
    slopes.iter().map(|s| 0.5 + 0.5 * s / peak).collect()
}

pub fn regimes(points: &[Point], tolerance: f64) -> Vec<f64> {
    let (lo, hi) = points
        .iter()
        .map(|p| p.1)
        .filter(|v| v.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
    let eps = (hi - lo).max(1e-12) * tolerance;
    points
        .windows(2)
        .map(|w| {
            let delta = w[1].1 - w[0].1;
            if !delta.is_finite() || delta.abs() <= eps { 0.5 } else if delta > 0.0 { 1.0 } else { 0.0 }
        })
        .collect()
}

pub fn regime_plates(points: &[Point], tones: &[f64], floor: f64, thickness: f64, row: f64, depth: f64, class: usize) -> Vec<Bar3DBlock> {
    let mut out = Vec::new();
    let mut start = 0usize;
    while start < tones.len() {
        let mut end = start;
        while end + 1 < tones.len() && tones[end + 1] == tones[start] {
            end += 1;
        }
        let (x0, x1) = (points[start].0, points[end + 1].0);
        out.push(Bar3DBlock::new((x0 + x1) / 2.0, row, floor - thickness, floor, (x1 - x0) / 2.0, depth, class).with_tone(tones[start]));
        start = end + 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line() -> Vec<Point> {
        points_of(&[12.0, 19.0, 15.0, 22.0, 28.0, 24.0])
    }

    #[test]
    fn a_ribbon_slopes_from_each_point_to_the_next() {
        let joined = vec![true; 5];
        let ribbon = ribbon_blocks(&line(), &joined, 0.0, 0.1, 0, None);
        assert_eq!(ribbon.len(), 5);
        let first = ribbon[0];
        let end = first.end.expect("sloped");
        assert!(first.z1 < end.1);
        assert_eq!(first.cx, 0.5);
        assert_eq!(first.hw, 0.5);
    }

    #[test]
    fn gaps_and_jumps_break_the_ribbon() {
        let mut points = line();
        points[2].1 = f64::NAN;
        let joined = joined_by_jump(&points, f64::NAN);
        assert_eq!(ribbon_blocks(&points, &joined, 0.0, 0.1, 0, None).len(), 3);
        let jumps = joined_by_jump(&line(), 5.0);
        assert_eq!(jumps, vec![false, true, false, false, true]);
    }

    #[test]
    fn areas_fill_down_to_the_floor_with_a_tone_by_height() {
        let joined = vec![true; 5];
        let area = area_blocks(&line(), &joined, 0.0, 0.0, 0.4, 1, true);
        assert!(area.iter().all(|b| b.z0 == 0.0 && b.end.map(|e| e.0 == 0.0).unwrap_or(false)));
        assert!(area[3].tone.unwrap() > area[0].tone.unwrap());
        assert!(area_blocks(&line(), &joined, 0.0, 0.0, 0.4, 1, false).iter().all(|b| b.tone.is_none()));
    }

    #[test]
    fn bands_span_between_the_two_boundaries_at_both_ends() {
        let low = points_of(&[1.0, 2.0, 3.0]);
        let high = points_of(&[3.0, 4.0, 6.0]);
        let band = band_blocks(&low, &high, 0.0, 0.4, 0);
        assert_eq!(band.len(), 2);
        assert_eq!((band[1].z0, band[1].z1), (2.0, 4.0));
        assert_eq!(band[1].end, Some((3.0, 6.0)));
    }

    #[test]
    fn steps_alternate_treads_and_risers_by_shape() {
        let after = step_blocks(&line(), Riser::After, 0.0, 0.1, 0, None);
        assert_eq!(after.len(), 5 + 5);
        let middle = step_blocks(&line(), Riser::Middle, 0.0, 0.1, 0, None);
        assert!(middle.len() > after.len() - 1);
        assert_eq!(Riser::parse("vh"), Riser::Before);
        assert_eq!(Riser::parse("hvh"), Riser::Middle);
        assert_eq!(Riser::parse("hv"), Riser::After);
    }

    #[test]
    fn splines_densify_and_pass_through_the_data_points() {
        let smooth = catmull_rom(&line(), 8, 0.5);
        assert_eq!(smooth.len(), 5 * 8 + 1);
        assert_eq!(smooth[0], (0.0, 12.0));
        assert_eq!(smooth[8], (1.0, 19.0));
        assert_eq!(catmull_rom(&points_of(&[1.0, 2.0]), 8, 0.5).len(), 2);
    }

    #[test]
    fn dashes_keep_only_the_on_segments_and_markers_skip_gaps() {
        let dashes = dashed_blocks(&line(), (2, 1), 4, 0.0, 0.1, 0, None);
        assert!(!dashes.is_empty() && dashes.len() < 20);
        let mut points = line();
        points[1].1 = f64::NAN;
        assert_eq!(marker_blocks(&points, 0.5, 2.0, 0.0, 0, None).len(), 5);
    }

    #[test]
    fn slopes_and_regimes_become_tones_and_floor_plates() {
        let tones = slope_tones(&line());
        assert_eq!(tones.len(), 5);
        assert!(tones[0] > 0.5 && tones[1] < 0.5);
        let steady = points_of(&[10.0, 10.0, 20.0, 30.0, 5.0]);
        let regime = regimes(&steady, 0.05);
        assert_eq!(regime, vec![0.5, 1.0, 1.0, 0.0]);
        let plates = regime_plates(&steady, &regime, 0.0, 1.0, 0.0, 0.4, 0);
        assert_eq!(plates.len(), 3);
        assert_eq!((plates[1].cx, plates[1].hw, plates[1].tone), (2.0, 1.0, Some(1.0)));
    }

    #[test]
    fn toned_ribbons_carry_one_tone_per_segment() {
        let ribbon = toned_ribbon(&line(), &[0.1, 0.9, 0.5, 0.5, 0.5], 0.0, 0.1, 0);
        assert_eq!(ribbon[1].tone, Some(0.9));
    }
}
