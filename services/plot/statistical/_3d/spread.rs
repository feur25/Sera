use super::super::bar::Bar3DBlock;
use crate::plot::statistical::boxplot::common::{compute_box, quartile};

pub const MEDIAN_TONE: f64 = 0.5;

#[derive(Clone, Copy)]
pub struct Group<'a> {
    pub samples: &'a [f64],
    pub cx: f64,
    pub cy: f64,
    pub class: usize,
}

#[derive(Clone, Copy)]
pub struct BoxStyle {
    pub hw: f64,
    pub depth: f64,
    pub whisker_hw: f64,
    pub notch: bool,
    pub median: bool,
}

#[derive(Clone, Copy)]
pub enum Scatter {
    Aligned,
    Jitter,
    Swarm,
}

pub fn overall_span(groups: &[Group]) -> f64 {
    let all = groups.iter().flat_map(|g| g.samples.iter()).filter(|v| v.is_finite());
    let (lo, hi) = all.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if lo.is_finite() { (hi - lo).max(1e-9) } else { 1.0 }
}

fn sorted(samples: &[f64]) -> Vec<f64> {
    let mut s: Vec<f64> = samples.iter().copied().filter(|v| v.is_finite()).collect();
    s.sort_by(|a, b| a.total_cmp(b));
    s
}

pub fn box_blocks(groups: &[Group], style: &BoxStyle) -> Vec<Bar3DBlock> {
    let thin = overall_span(groups) * 0.006;
    groups
        .iter()
        .flat_map(|g| {
            let s = compute_box(g.samples);
            let mut blocks = vec![Bar3DBlock::new(
                g.cx,
                g.cy,
                s.whisker_lo,
                s.whisker_hi,
                style.whisker_hw,
                style.depth * 0.27,
                g.class,
            )];
            if style.notch {
                let reach = (1.57 * (s.q3 - s.q1) / (s.n.max(1) as f64).sqrt()).min((s.q3 - s.q1) / 2.0);
                let (lo, hi) = (s.median - reach, s.median + reach);
                blocks.push(Bar3DBlock::new(g.cx, g.cy, s.q1, lo.max(s.q1), style.hw, style.depth, g.class));
                blocks.push(Bar3DBlock::new(g.cx, g.cy, lo.max(s.q1), hi.min(s.q3), style.hw * 0.55, style.depth, g.class));
                blocks.push(Bar3DBlock::new(g.cx, g.cy, hi.min(s.q3), s.q3, style.hw, style.depth, g.class));
            } else {
                blocks.push(Bar3DBlock::new(g.cx, g.cy, s.q1, s.q3, style.hw, style.depth, g.class));
            }
            if style.median {
                blocks.push(
                    Bar3DBlock::new(g.cx, g.cy, s.median - thin, s.median + thin, style.hw * 1.08, style.depth * 1.08, g.class)
                        .with_tone(MEDIAN_TONE),
                );
            }
            blocks
        })
        .collect()
}

pub fn outlier_blocks(groups: &[Group], size: f64) -> Vec<Bar3DBlock> {
    groups
        .iter()
        .flat_map(|g| {
            compute_box(g.samples)
                .outliers
                .into_iter()
                .map(move |v| Bar3DBlock::new(g.cx, g.cy, v - size / 2.0, v + size / 2.0, size / 2.0, size / 2.0, g.class))
        })
        .collect()
}

fn swarm_offsets(values: &[f64], step: f64, reach: f64, size: f64) -> Vec<f64> {
    let mut placed: Vec<(f64, f64)> = Vec::with_capacity(values.len());
    let mut offsets = Vec::with_capacity(values.len());
    for &v in values {
        let clear = |off: f64| {
            !placed
                .iter()
                .any(|&(pv, po)| (pv - v).abs() < size && (po - off).abs() < step)
        };
        let choice = (0..64)
            .map(|k| {
                let magnitude = ((k + 1) / 2) as f64 * step;
                if k % 2 == 0 { magnitude } else { -magnitude }
            })
            .find(|&off| off.abs() <= reach && clear(off))
            .unwrap_or(0.0);
        placed.push((v, choice));
        offsets.push(choice);
    }
    offsets
}

pub fn point_blocks(groups: &[Group], scatter: Scatter, reach: f64, size: f64) -> Vec<Bar3DBlock> {
    groups
        .iter()
        .flat_map(|g| {
            let values = sorted(g.samples);
            let offsets: Vec<f64> = match scatter {
                Scatter::Aligned => vec![0.0; values.len()],
                Scatter::Jitter => (0..values.len())
                    .map(|k| ((k as f64 * 0.618_033_988_75) % 1.0 - 0.5) * 2.0 * reach)
                    .collect(),
                Scatter::Swarm => swarm_offsets(&values, size * 1.1, reach, size),
            };
            values
                .into_iter()
                .zip(offsets)
                .map(|(v, off)| Bar3DBlock::new(g.cx + off, g.cy, v - size / 2.0, v + size / 2.0, size / 2.0, size / 2.0, g.class))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn gaussian_density(values: &[f64], at: f64, bandwidth: f64) -> f64 {
    values
        .iter()
        .map(|&v| (-0.5 * ((at - v) / bandwidth).powi(2)).exp())
        .sum::<f64>()
        / (values.len().max(1) as f64 * bandwidth)
}

pub fn violin_blocks(groups: &[Group], slices: usize, max_hw: f64, depth: f64) -> Vec<Bar3DBlock> {
    groups
        .iter()
        .flat_map(|g| {
            let values = sorted(g.samples);
            let (Some(&lo), Some(&hi)) = (values.first(), values.last()) else {
                return Vec::new();
            };
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let spread = (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64).sqrt();
            let bandwidth = (1.06 * spread * (values.len() as f64).powf(-0.2)).max((hi - lo).max(1e-9) * 0.05);
            let step = (hi - lo).max(1e-9) / slices.max(1) as f64;
            let densities: Vec<f64> = (0..slices)
                .map(|s| gaussian_density(&values, lo + (s as f64 + 0.5) * step, bandwidth))
                .collect();
            let peak = densities.iter().cloned().fold(1e-12, f64::max);
            densities
                .iter()
                .enumerate()
                .map(|(s, d)| {
                    let width = (max_hw * d / peak).max(max_hw * 0.04);
                    Bar3DBlock::new(g.cx, g.cy, lo + s as f64 * step, lo + (s + 1) as f64 * step, width, depth, g.class)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn letter_value_blocks(groups: &[Group], levels: usize, hw: f64, depth: f64) -> Vec<Bar3DBlock> {
    groups
        .iter()
        .flat_map(|g| {
            let values = sorted(g.samples);
            if values.is_empty() {
                return Vec::new();
            }
            let bands = (0..levels.max(1)).map(|k| {
                let tail = 0.25 / 2.0_f64.powi(k as i32);
                let inner = if k == 0 { 0.25 } else { tail * 2.0 };
                let width = hw * (1.0 - 0.22 * k as f64).max(0.2);
                (tail, inner, width)
            });
            bands
                .flat_map(|(tail, inner, width)| {
                    let (lower_lo, lower_hi) = (quartile(&values, tail), quartile(&values, inner));
                    let (upper_lo, upper_hi) = (quartile(&values, 1.0 - inner), quartile(&values, 1.0 - tail));
                    if inner == 0.25 && tail == 0.25 {
                        return vec![Bar3DBlock::new(g.cx, g.cy, lower_lo, upper_hi, width, depth, g.class)];
                    }
                    vec![
                        Bar3DBlock::new(g.cx, g.cy, lower_lo, lower_hi, width, depth, g.class),
                        Bar3DBlock::new(g.cx, g.cy, upper_lo, upper_hi, width, depth, g.class),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn transposed(blocks: Vec<Bar3DBlock>) -> Vec<Bar3DBlock> {
    blocks
        .into_iter()
        .map(|b| Bar3DBlock { cx: b.cy, cy: b.cx, hw: b.hd, hd: b.hw, ..b })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: [f64; 9] = [1.2, 2.4, 2.7, 3.1, 3.5, 3.8, 4.2, 5.1, 6.0];
    const B: [f64; 11] = [2.0, 2.8, 3.2, 3.6, 4.1, 4.5, 5.0, 5.7, 6.5, 18.0, -6.0];

    fn groups() -> Vec<Group<'static>> {
        vec![
            Group { samples: &A, cx: 0.0, cy: 0.0, class: 0 },
            Group { samples: &B, cx: 1.0, cy: 0.0, class: 1 },
        ]
    }

    const STYLE: BoxStyle = BoxStyle { hw: 0.3, depth: 0.3, whisker_hw: 0.05, notch: false, median: true };

    #[test]
    fn boxes_stack_whisker_box_and_median_per_group() {
        let blocks = box_blocks(&groups(), &STYLE);
        assert_eq!(blocks.len(), 6);
        assert!(blocks[0].z0 <= blocks[1].z0 && blocks[0].z1 >= blocks[1].z1);
        assert_eq!(blocks[2].tone, Some(MEDIAN_TONE));
    }

    #[test]
    fn notched_boxes_split_around_a_narrower_waist() {
        let blocks = box_blocks(&groups(), &BoxStyle { notch: true, ..STYLE });
        let waist = &blocks[2];
        assert!(waist.hw < blocks[1].hw);
    }

    #[test]
    fn outliers_become_small_cubes_beyond_the_whiskers() {
        let cubes = outlier_blocks(&groups(), 0.3);
        assert_eq!(cubes.len(), 2);
        assert!(cubes.iter().all(|c| c.cx == 1.0));
    }

    #[test]
    fn scatter_modes_place_every_sample_and_swarm_avoids_overlap() {
        for mode in [Scatter::Aligned, Scatter::Jitter, Scatter::Swarm] {
            assert_eq!(point_blocks(&groups(), mode, 0.35, 0.2).len(), 20);
        }
        let swarm = point_blocks(&groups()[..1], Scatter::Swarm, 2.0, 0.6);
        let crowded = swarm
            .windows(2)
            .filter(|w| (w[0].z0 - w[1].z0).abs() < 0.6 && (w[0].cx - w[1].cx).abs() < 0.6 * 1.05)
            .count();
        assert_eq!(crowded, 0);
        let jitter = point_blocks(&groups()[..1], Scatter::Jitter, 0.35, 0.2);
        assert!(jitter.iter().any(|b| b.cx != 0.0));
    }

    #[test]
    fn violin_width_peaks_where_the_samples_are_dense() {
        let slices = violin_blocks(&groups()[..1], 12, 0.4, 0.3);
        assert_eq!(slices.len(), 12);
        let widest = slices.iter().map(|b| b.hw).fold(0.0, f64::max);
        assert!((widest - 0.4).abs() < 1e-9);
        assert!(slices[0].hw < widest);
    }

    #[test]
    fn letter_values_nest_narrower_bands_towards_the_tails() {
        let bands = letter_value_blocks(&groups()[..1], 3, 0.4, 0.3);
        assert_eq!(bands.len(), 5);
        assert!(bands[1].hw < bands[0].hw && bands[3].hw < bands[1].hw);
    }

    #[test]
    fn transposing_swaps_the_footprint_axes() {
        let flipped = transposed(vec![Bar3DBlock::new(1.0, 2.0, 0.0, 1.0, 0.3, 0.1, 0)]);
        assert_eq!((flipped[0].cx, flipped[0].cy, flipped[0].hw, flipped[0].hd), (2.0, 1.0, 0.1, 0.3));
    }
}
