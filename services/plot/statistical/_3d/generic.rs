use super::super::bar::Bar3DBlock;
use crate::plot::statistical::boxplot::common::compute_box;
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn linear_columns(values: &[f64], hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| Bar3DBlock::new(i as f64, 0.0, 0.0, v, hw, hd, i))
        .collect()
}

pub fn linear_columns_horizontal(values: &[f64], hd: f64, depth: f64) -> Vec<Bar3DBlock> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| Bar3DBlock::new(v / 2.0, i as f64, 0.0, depth, v.abs() / 2.0, hd, i))
        .collect()
}

pub fn diverging_columns(values: &[f64], hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let (z0, z1) = if v >= 0.0 { (0.0, v) } else { (v, 0.0) };
            Bar3DBlock::new(i as f64, 0.0, z0, z1, hw, hd, i)
        })
        .collect()
}

pub fn grouped_columns(
    series: &[(String, Vec<f64>)],
    n_cats: usize,
    stacked: bool,
    hw: f64,
    hd: f64,
) -> Vec<Bar3DBlock> {
    let n_ser = series.len();
    if n_cats == 0 || n_ser == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n_cats * n_ser);
    for ci in 0..n_cats {
        if stacked {
            let mut acc = 0.0_f64;
            for (si, (_, vals)) in series.iter().enumerate() {
                let v = vals.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() {
                    continue;
                }
                out.push(Bar3DBlock::new(ci as f64, 0.0, acc, acc + v, hw, hd, si));
                acc += v;
            }
        } else {
            for (si, (_, vals)) in series.iter().enumerate() {
                let v = vals.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() {
                    continue;
                }
                let cy = si as f64 - (n_ser as f64 - 1.0) / 2.0;
                out.push(Bar3DBlock::new(ci as f64, cy, 0.0, v, hw, hd, si));
            }
        }
    }
    out
}

pub fn diverging_stacked_columns(
    series: &[(String, Vec<f64>)],
    n_cats: usize,
    hw: f64,
    hd: f64,
) -> Vec<Bar3DBlock> {
    let mut out = Vec::new();
    for ci in 0..n_cats {
        let mut pos_acc = 0.0_f64;
        let mut neg_acc = 0.0_f64;
        for (si, (_, vals)) in series.iter().enumerate() {
            let v = vals.get(ci).copied().unwrap_or(0.0);
            if !v.is_finite() {
                continue;
            }
            if v >= 0.0 {
                out.push(Bar3DBlock::new(ci as f64, 0.0, pos_acc, pos_acc + v, hw, hd, si));
                pos_acc += v;
            } else {
                out.push(Bar3DBlock::new(ci as f64, 0.0, neg_acc + v, neg_acc, hw, hd, si));
                neg_acc += v;
            }
        }
    }
    out
}

pub fn two_row_columns(top: &[f64], bottom: &[f64], row_offset: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let n = top.len().min(bottom.len());
    let mut out = Vec::with_capacity(n * 2);
    for (ci_side, (row, offset)) in [(top, -row_offset), (bottom, row_offset)].into_iter().enumerate() {
        for ci in 0..n {
            let v = row.get(ci).copied().unwrap_or(0.0).abs();
            out.push(Bar3DBlock::new(ci as f64, offset, 0.0, v, hw, hd, ci_side));
        }
    }
    out
}

pub fn box_whisker_columns(samples: &[&[f64]], hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let mut out = Vec::with_capacity(samples.len() * 2);
    for (ci, dist) in samples.iter().enumerate() {
        let stats = compute_box(dist);
        out.push(Bar3DBlock::new(ci as f64, 0.0, stats.whisker_lo, stats.whisker_hi, hw * 0.27, hd * 0.27, ci));
        out.push(Bar3DBlock::new(ci as f64, 0.0, stats.q1, stats.q3, hw, hd, ci));
    }
    out
}

fn arc_positions(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| -FRAC_PI_2 + TAU * i as f64 / n.max(1) as f64)
        .collect()
}

pub fn radial_columns(values: &[f64], radius: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    let slot = std::f64::consts::TAU * radius / n as f64;
    let half_w = (slot * 0.36).clamp(hw, hw.max(0.5));
    let half_d = (slot * 0.36).clamp(hd, hd.max(0.5));
    arc_positions(n)
        .into_iter()
        .zip(values.iter())
        .enumerate()
        .map(|(i, (theta, &v))| Bar3DBlock::new(radius * theta.cos(), radius * theta.sin(), 0.0, v, half_w, half_d, i))
        .collect()
}

pub fn radial_diverging_columns(values: &[f64], radius: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    arc_positions(n)
        .into_iter()
        .zip(values.iter())
        .enumerate()
        .map(|(i, (theta, &v))| {
            let (z0, z1) = if v >= 0.0 { (0.0, v) } else { (v, 0.0) };
            Bar3DBlock::new(radius * theta.cos(), radius * theta.sin(), z0, z1, hw, hd, i)
        })
        .collect()
}

fn arc_groups<'a>(n: usize, group_of: impl Fn(usize) -> &'a str) -> (Vec<&'a str>, Vec<Vec<usize>>) {
    let mut groups: Vec<&str> = Vec::new();
    for i in 0..n {
        let g = group_of(i);
        if !groups.contains(&g) {
            groups.push(g);
        }
    }
    let mut idxs: Vec<Vec<usize>> = vec![Vec::new(); groups.len()];
    for i in 0..n {
        let g = group_of(i);
        if let Some(gi) = groups.iter().position(|x| *x == g) {
            idxs[gi].push(i);
        }
    }
    (groups, idxs)
}

pub fn radial_grouped_columns(values: &[f64], groups_of: &[String], radius: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    let use_groups = groups_of.len() == n;
    let (groups, group_idxs) = if use_groups {
        arc_groups(n, |i| groups_of[i].as_str())
    } else {
        (vec![""], vec![(0..n).collect()])
    };
    let n_groups = groups.len().max(1);
    let gap = if n_groups > 1 { TAU * 0.03 } else { 0.0 };
    let usable = TAU - gap * n_groups as f64;
    let mut out = Vec::with_capacity(n);
    let mut cursor = -FRAC_PI_2;
    for (gi, idxs) in group_idxs.iter().enumerate() {
        let count = idxs.len().max(1);
        let group_angle = usable / n_groups as f64;
        let slot = group_angle / count as f64;
        for (k, &i) in idxs.iter().enumerate() {
            let theta = cursor + slot * (k as f64 + 0.5);
            out.push(Bar3DBlock::new(radius * theta.cos(), radius * theta.sin(), 0.0, values[i], hw, hd, gi));
        }
        cursor += group_angle + gap;
    }
    out
}

pub fn radial_hierarchical_columns(
    values: &[f64],
    super_group_of: &[String],
    radius: f64,
    hw: f64,
    hd: f64,
) -> Vec<Bar3DBlock> {
    radial_grouped_columns(values, super_group_of, radius, hw, hd)
}

pub fn spiral_turn_length(n: usize) -> usize {
    ((n as f64 / 3.2).ceil() as usize).clamp(12, 48)
}

pub fn spiral_columns(values: &[f64], hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    let per_turn = spiral_turn_length(n);
    let angle_step = std::f64::consts::TAU / per_turn as f64;
    let footprint = hw.max(hd);
    let r_hub = 2.6 * footprint / angle_step;
    let r_pitch = 3.0 * footprint;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let theta = -std::f64::consts::FRAC_PI_2 + angle_step * i as f64;
            let r = r_hub + r_pitch * i as f64 / per_turn as f64;
            Bar3DBlock::new(r * theta.cos(), r * theta.sin(), 0.0, v, hw, hd, i)
        })
        .collect()
}

pub fn radial_band_columns(mins: &[f64], maxs: &[f64], radius: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let n = mins.len().min(maxs.len());
    if n == 0 {
        return Vec::new();
    }
    arc_positions(n)
        .into_iter()
        .enumerate()
        .map(|(i, theta)| Bar3DBlock::new(radius * theta.cos(), radius * theta.sin(), mins[i], maxs[i], hw, hd, i))
        .collect()
}

pub fn radial_stacked_columns(series: &[(String, Vec<f64>)], n_axes: usize, radius: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    if n_axes == 0 || series.is_empty() {
        return Vec::new();
    }
    let angles = arc_positions(n_axes);
    let mut out = Vec::with_capacity(n_axes * series.len());
    for ai in 0..n_axes {
        let theta = angles[ai];
        let (cx, cy) = (radius * theta.cos(), radius * theta.sin());
        let mut acc = 0.0_f64;
        for (si, (_, vals)) in series.iter().enumerate() {
            let v = vals.get(ai).copied().unwrap_or(0.0);
            if !v.is_finite() {
                continue;
            }
            out.push(Bar3DBlock::new(cx, cy, acc, acc + v, hw, hd, si));
            acc += v;
        }
    }
    out
}

pub fn grid_columns(n_rows: usize, n_cols: usize, matrix: &[f64], hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    if n_rows == 0 || n_cols == 0 {
        return Vec::new();
    }
    (0..n_rows)
        .flat_map(|r| (0..n_cols).map(move |c| (r, c)))
        .enumerate()
        .filter_map(|(idx, (r, c))| {
            matrix
                .get(r * n_cols + c)
                .map(|&v| Bar3DBlock::new(c as f64, r as f64, 0.0, v, hw, hd, idx))
        })
        .collect()
}

pub fn panel_columns(values: &[f64], classes: &[usize], gap: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    let mut cursor = 0.0;
    let mut previous: Option<usize> = None;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let class = classes.get(i).copied().unwrap_or(0);
            if matches!(previous, Some(p) if p != class) {
                cursor += gap;
            }
            previous = Some(class);
            let (z0, z1) = if v >= 0.0 { (0.0, v) } else { (v, 0.0) };
            let block = Bar3DBlock::new(cursor, 0.0, z0, z1, hw, hd, class);
            cursor += 1.0;
            block
        })
        .collect()
}

pub fn plate_columns(values: &[f64], thickness: f64, hw: f64, hd: f64) -> Vec<Bar3DBlock> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| Bar3DBlock::new(i as f64, 0.0, v - thickness, v, hw, hd, i))
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

    #[test]
    fn transposing_swaps_the_footprint_axes_and_keeps_tone() {
        let flipped = transposed(vec![Bar3DBlock::new(1.0, 2.0, 0.0, 1.0, 0.3, 0.1, 0).with_tone(0.4)]);
        assert_eq!((flipped[0].cx, flipped[0].cy, flipped[0].hw, flipped[0].hd), (2.0, 1.0, 0.1, 0.3));
        assert_eq!(flipped[0].tone, Some(0.4));
    }

    #[test]
    fn panels_leave_a_gap_whenever_the_class_changes_and_keep_the_sign() {
        let panels = panel_columns(&[1.0, -2.0, 3.0], &[0, 0, 1], 1.5, 0.3, 0.3);
        assert_eq!(panels.iter().map(|b| b.cx).collect::<Vec<_>>(), vec![0.0, 1.0, 3.5]);
        assert_eq!(panels.iter().map(|b| b.ci).collect::<Vec<_>>(), vec![0, 0, 1]);
        assert_eq!((panels[1].z0, panels[1].z1), (-2.0, 0.0));
        assert_eq!(panel_columns(&[1.0, 2.0], &[], 1.5, 0.3, 0.3)[1].cx, 1.0);
    }

    #[test]
    fn plates_hug_the_top_of_each_value() {
        let plates = plate_columns(&[3.0, 5.0], 0.5, 0.5, 0.4);
        assert_eq!((plates[1].z0, plates[1].z1, plates[1].cx), (4.5, 5.0, 1.0));
    }
}
