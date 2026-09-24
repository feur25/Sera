use super::config::PieConfig;
use super::variant::PieVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::{branch_tone, rings, RING_THICK};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::TAU;

const HOLE: f64 = 0.75;
const DONUT_HOLE: f64 = HOLE + 2.2;
const HEIGHT: f64 = 1.0;
const PULL_FACTOR: f64 = 0.35;
const SPOKE_HOLE: f64 = 0.5;
const SPOKE_RANGE: f64 = 3.2;
const RADIAL_STEP: f64 = 0.4;
const SUBPLOT_GAP: f64 = 7.5;
const CELL_ROWS: usize = 10;
const CELL_COLS: usize = 10;

fn finite(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { v.max(0.0) } else { 0.0 }).collect()
}

fn value_spans(values: &[f64], start: f64, sweep_total: f64) -> Vec<(f64, f64)> {
    let n = values.len();
    let total: f64 = values.iter().map(|v| v.max(0.0)).sum();
    if total <= 0.0 {
        let step = sweep_total / n.max(1) as f64;
        return (0..n).map(|i| (start + i as f64 * step, start + (i as f64 + 1.0) * step)).collect();
    }
    let mut a = start;
    values
        .iter()
        .map(|&v| {
            let sweep = v.max(0.0) / total * sweep_total;
            let span = (a, a + sweep);
            a += sweep;
            span
        })
        .collect()
}

fn ring_wedges(spans: &[(f64, f64)], hole: f64, height: f64, n_tones: usize) -> Vec<Bar3DBlock> {
    let n = spans.len();
    let order: Vec<usize> = (0..n).collect();
    let depth = vec![0usize; n];
    rings(&order, &depth, spans, 0, hole, |_| height, |i| branch_tone(i, n_tones))
}

fn nested_wedges(values: &[f64], secondary: &[f64], height: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    let m = secondary.len();
    let order: Vec<usize> = (0..n + m).collect();
    let mut depth = vec![0usize; n + m];
    for d in depth.iter_mut().skip(n) {
        *d = 1;
    }
    let mut spans = value_spans(values, 0.0, TAU);
    spans.extend(value_spans(secondary, 0.0, TAU));
    let n_tones = n.max(m).max(1);
    rings(&order, &depth, &spans, 1, HOLE, |_| height, |i| branch_tone(if i < n { i } else { i - n }, n_tones))
}

fn auto_pull(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    let mut out = vec![0.0; n];
    let mut best: Option<(usize, f64)> = None;
    for (i, &v) in values.iter().enumerate() {
        if best.map_or(true, |(_, mv)| v > mv) {
            best = Some((i, v));
        }
    }
    if let Some((idx, _)) = best {
        out[idx] = 1.0;
    }
    out
}

fn apply_pull(mut blocks: Vec<Bar3DBlock>, pull: &[f64]) -> Vec<Bar3DBlock> {
    for b in blocks.iter_mut() {
        let p = pull.get(b.ci).copied().unwrap_or(0.0).max(0.0);
        if p > 0.0 {
            let k = 1.0 + p * PULL_FACTOR;
            b.cx *= k;
            b.cy *= k;
        }
    }
    blocks
}

fn nightingale_wedges(values: &[f64], height: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    let vmax = values.iter().cloned().filter(|v| v.is_finite()).fold(0.0_f64, f64::max).max(1e-9);
    let step_a = TAU / n as f64;
    let mut out = Vec::new();
    for i in 0..n {
        let theta = i as f64 * step_a + step_a * 0.5;
        let r_out = SPOKE_HOLE + (values[i].max(0.0) / vmax) * SPOKE_RANGE;
        let steps = (((r_out - SPOKE_HOLE) / RADIAL_STEP).ceil() as usize).max(1);
        for k in 0..steps {
            let r = SPOKE_HOLE + (r_out - SPOKE_HOLE) * (k as f64 + 0.5) / steps as f64;
            out.push(Bar3DBlock::new(r * theta.cos(), r * theta.sin(), 0.0, height, RING_THICK, RING_THICK, i).with_tone(branch_tone(i, n)));
        }
    }
    out
}

fn waffle_cells(values: &[f64], height: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    let n_cells = CELL_ROWS * CELL_COLS;
    let total: f64 = values.iter().map(|v| v.max(0.0)).sum();
    if n == 0 || total <= 0.0 {
        return Vec::new();
    }
    let raw: Vec<f64> = values.iter().map(|&v| v.max(0.0) / total * n_cells as f64).collect();
    let mut counts: Vec<usize> = raw.iter().map(|&r| r.floor() as usize).collect();
    let assigned: usize = counts.iter().sum();
    let mut remainders: Vec<(usize, f64)> = raw.iter().enumerate().map(|(i, &r)| (i, r - r.floor())).collect();
    remainders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut leftover = n_cells.saturating_sub(assigned);
    let mut ri = 0;
    while leftover > 0 && ri < remainders.len() {
        counts[remainders[ri].0] += 1;
        leftover -= 1;
        ri += 1;
    }
    let mut cell_cat: Vec<usize> = Vec::with_capacity(n_cells);
    for (i, &c) in counts.iter().enumerate() {
        for _ in 0..c {
            cell_cat.push(i);
        }
    }
    while cell_cat.len() < n_cells {
        cell_cat.push(n - 1);
    }
    let gap = 1.0;
    let half = 0.42;
    let ox = -(CELL_COLS as f64 - 1.0) * gap / 2.0;
    let oy = (CELL_ROWS as f64 - 1.0) * gap / 2.0;
    (0..n_cells)
        .map(|idx| {
            let row = idx / CELL_COLS;
            let col = idx % CELL_COLS;
            let cat = cell_cat[idx];
            Bar3DBlock::new(ox + col as f64 * gap, oy - row as f64 * gap, 0.0, height, half, half, cat).with_tone(branch_tone(cat, n))
        })
        .collect()
}

fn subplot_grid(series: &[Vec<f64>], proportional: bool, height: f64) -> Vec<Bar3DBlock> {
    let n_pies = series.len();
    if n_pies == 0 {
        return Vec::new();
    }
    let n_cats = series.iter().map(|s| s.len()).max().unwrap_or(0);
    let cols = (n_pies as f64).sqrt().ceil().max(1.0) as usize;
    let totals: Vec<f64> = series.iter().map(|s| s.iter().filter(|v| v.is_finite() && **v >= 0.0).sum()).collect();
    let max_total = totals.iter().cloned().fold(0.0_f64, f64::max).max(1e-9);
    let mut out = Vec::new();
    for (pi, vals) in series.iter().enumerate() {
        let row = pi / cols;
        let col = pi % cols;
        let ox = col as f64 * SUBPLOT_GAP;
        let oy = -(row as f64) * SUBPLOT_GAP;
        let scale = if proportional { (totals[pi] / max_total).sqrt().max(0.25) } else { 1.0 };
        let spans = value_spans(vals, 0.0, TAU);
        for mut b in ring_wedges(&spans, HOLE * scale, height, n_cats) {
            b.cx += ox;
            b.cy += oy;
            out.push(b);
        }
    }
    out
}

fn subplot_names(labels: &[String], series: &[Vec<f64>]) -> Vec<String> {
    if !labels.is_empty() {
        return labels.to_vec();
    }
    let n_cats = series.iter().map(|s| s.len()).max().unwrap_or(0);
    (0..n_cats).map(|i| format!("S{}", i + 1)).collect()
}

fn names_for(n: usize, labels: &[String]) -> Vec<String> {
    (0..n).map(|i| labels.get(i).cloned().unwrap_or_else(|| format!("Slice {}", i + 1))).collect()
}

fn pie_3d(cfg: &PieConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    if !cfg.series.is_empty() && matches!(cfg.variant, PieVariant::Subplots | PieVariant::Proportional) {
        let series: Vec<Vec<f64>> = cfg.series.iter().map(|s| finite(s)).collect();
        let blocks = subplot_grid(&series, cfg.proportional || matches!(cfg.variant, PieVariant::Proportional), HEIGHT);
        let names = subplot_names(cfg.labels, cfg.series);
        return (blocks, names);
    }

    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let values = finite(&cfg.values[..n]);
    let values = &values[..];
    let secondary_values = finite(cfg.secondary_values);
    let names = names_for(n, cfg.labels);

    let blocks = match cfg.variant {
        PieVariant::Semi => {
            let spans = value_spans(values, 0.0, std::f64::consts::PI);
            ring_wedges(&spans, HOLE, HEIGHT, n)
        }
        PieVariant::Donut | PieVariant::Kpi => {
            let spans = value_spans(values, 0.0, TAU);
            ring_wedges(&spans, DONUT_HOLE, HEIGHT, n)
        }
        PieVariant::Nested => nested_wedges(values, &secondary_values, HEIGHT),
        PieVariant::Nightingale => nightingale_wedges(values, HEIGHT),
        PieVariant::Waffle => waffle_cells(values, HEIGHT),
        PieVariant::Exploded => {
            let spans = value_spans(values, 0.0, TAU);
            let base = ring_wedges(&spans, HOLE, HEIGHT, n);
            let pull = if cfg.pull.is_empty() { auto_pull(values) } else { finite(cfg.pull) };
            apply_pull(base, &pull)
        }
        _ => {
            let spans = value_spans(values, 0.0, TAU);
            ring_wedges(&spans, HOLE, HEIGHT, n)
        }
    };

    let names = if matches!(cfg.variant, PieVariant::Nested) {
        let mut all = names;
        all.extend(names_for(cfg.secondary_values.len(), cfg.secondary_labels));
        all
    } else {
        names
    };

    (blocks, names)
}

pub fn layout_3d(cfg: &PieConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &PieConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    pie_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(variant: PieVariant, labels: &'static [&'static str], values: &'static [f64]) -> (Vec<String>, Vec<f64>, PieVariant) {
        (labels.iter().map(|s| s.to_string()).collect(), values.to_vec(), variant)
    }

    fn draw(variant: PieVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, values, variant) = cfg(variant, &["A", "B", "C"], &[30.0, 50.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        layout_named(&c, &Budget::default())
    }

    #[test]
    fn every_variant_draws_something_and_names_every_slice() {
        for &variant in PieVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names.len(), 3, "{variant:?}");
        }
    }

    #[test]
    fn a_slices_share_of_the_whole_circle_is_proportional_to_its_value_not_equal_spacing() {
        let (blocks, _) = draw(PieVariant::Basic);
        let angle_of = |b: &Bar3DBlock| b.cy.atan2(b.cx).rem_euclid(TAU);
        let span_of = |ci: usize| {
            let mut angs: Vec<f64> = blocks.iter().filter(|b| b.ci == ci).map(angle_of).collect();
            angs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            (angs.len(), angs)
        };
        let (count_a, _) = span_of(0);
        let (count_b, _) = span_of(1);
        let (count_c, _) = span_of(2);
        assert!(count_b > count_a && count_a > count_c, "B(50) > A(30) > C(20) in angular block count, got {count_a},{count_b},{count_c}");
    }

    #[test]
    fn every_basic_wedge_sits_at_the_same_radius_forming_one_disc() {
        let (blocks, _) = draw(PieVariant::Basic);
        let radii: Vec<f64> = blocks.iter().map(|b| b.cx.hypot(b.cy)).collect();
        let r0 = radii[0];
        assert!(radii.iter().all(|r| (r - r0).abs() < 1e-6));
    }

    #[test]
    fn donut_leaves_a_bigger_hole_at_the_centre_than_basic() {
        let basic = draw(PieVariant::Basic).0;
        let donut = draw(PieVariant::Donut).0;
        let r = |bs: &[Bar3DBlock]| bs[0].cx.hypot(bs[0].cy);
        assert!(r(&donut) > r(&basic));
    }

    #[test]
    fn nested_places_the_secondary_ring_farther_out_than_the_primary_one() {
        let (labels, values, variant) = cfg(PieVariant::Nested, &["A", "B", "C"], &[30.0, 50.0, 20.0]);
        let secondary_values = vec![10.0, 15.0, 5.0];
        let secondary_labels: Vec<String> = ["X", "Y", "Z"].iter().map(|s| s.to_string()).collect();
        let c = PieConfig { variant, labels: &labels, values: &values, secondary_values: &secondary_values, secondary_labels: &secondary_labels, ..PieConfig::default() };
        let (blocks, names) = layout_named(&c, &Budget::default());
        assert_eq!(names, vec!["A", "B", "C", "X", "Y", "Z"]);
        let inner_r = blocks.iter().filter(|b| b.ci < 3).map(|b| b.cx.hypot(b.cy)).fold(f64::INFINITY, f64::min);
        let outer_r = blocks.iter().filter(|b| b.ci >= 3).map(|b| b.cx.hypot(b.cy)).fold(0.0, f64::max);
        assert!(outer_r > inner_r);
    }

    #[test]
    fn semi_spans_only_a_half_circle() {
        let (blocks, _) = draw(PieVariant::Semi);
        assert!(blocks.iter().all(|b| b.cy >= -1e-6));
    }

    #[test]
    fn exploded_pushes_the_pulled_slice_farther_from_the_centre() {
        let (labels, values, variant) = cfg(PieVariant::Exploded, &["A", "B", "C"], &[30.0, 90.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        let (blocks, _) = layout_named(&c, &Budget::default());
        let r = |ci: usize| blocks.iter().filter(|b| b.ci == ci).map(|b| b.cx.hypot(b.cy)).fold(0.0, f64::max);
        assert!(r(1) > r(0) && r(1) > r(2));
    }

    #[test]
    fn nightingale_gives_the_largest_value_the_longest_spoke() {
        let (labels, values, variant) = cfg(PieVariant::Nightingale, &["A", "B", "C"], &[10.0, 90.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        let (blocks, _) = layout_named(&c, &Budget::default());
        let r = |ci: usize| blocks.iter().filter(|b| b.ci == ci).map(|b| b.cx.hypot(b.cy)).fold(0.0, f64::max);
        assert!(r(1) > r(0) && r(1) > r(2));
    }

    #[test]
    fn nightingale_keeps_every_slice_at_the_same_fixed_angle_regardless_of_its_value() {
        let (blocks, _) = draw(PieVariant::Nightingale);
        let angle_of = |b: &Bar3DBlock| b.cy.atan2(b.cx).rem_euclid(TAU);
        let step = TAU / 3.0;
        for ci in 0..3 {
            let angles: Vec<f64> = blocks.iter().filter(|b| b.ci == ci).map(angle_of).collect();
            assert!(!angles.is_empty(), "slice {ci} drew nothing");
            let expected = (ci as f64 * step + step * 0.5).rem_euclid(TAU);
            for a in angles {
                assert!((a - expected).abs() < 1e-6, "slice {ci} block at {a}, expected {expected}");
            }
        }
    }

    #[test]
    fn waffle_gives_the_larger_share_more_cells_than_the_smaller_one() {
        let (labels, values, variant) = cfg(PieVariant::Waffle, &["A", "B"], &[80.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        let (blocks, _) = layout_named(&c, &Budget::default());
        assert_eq!(blocks.len(), 100);
        let count = |ci: usize| blocks.iter().filter(|b| b.ci == ci).count();
        assert!(count(0) > count(1));
    }

    #[test]
    fn subplots_draws_one_ring_per_series_using_the_shared_category_labels() {
        let labels: Vec<String> = ["A", "B", "C", "D"].iter().map(|s| s.to_string()).collect();
        let series = vec![vec![40.0, 25.0, 20.0, 15.0], vec![30.0, 30.0, 20.0, 20.0], vec![50.0, 20.0, 15.0, 15.0]];
        let c = PieConfig { variant: PieVariant::Subplots, labels: &labels, series: &series, ..PieConfig::default() };
        let (blocks, names) = layout_named(&c, &Budget::default());
        assert_eq!(names, vec!["A", "B", "C", "D"]);
        let centers: std::collections::HashSet<(i64, i64)> = blocks.iter().map(|b| ((b.cx * 10.0) as i64, (b.cy * 10.0) as i64)).collect();
        assert!(centers.len() > 4, "subplots must not all land on the same ring");
    }

    #[test]
    fn proportional_shrinks_a_smaller_total_series_ring() {
        let labels: Vec<String> = ["A", "B"].iter().map(|s| s.to_string()).collect();
        let series = vec![vec![90.0, 90.0], vec![5.0, 5.0]];
        let c = PieConfig { variant: PieVariant::Proportional, labels: &labels, series: &series, proportional: true, ..PieConfig::default() };
        let (blocks, _) = layout_named(&c, &Budget::default());
        let local_ring_hole = |pi_blocks: &[&Bar3DBlock]| {
            let n = pi_blocks.len() as f64;
            let (cx0, cy0) = (pi_blocks.iter().map(|b| b.cx).sum::<f64>() / n, pi_blocks.iter().map(|b| b.cy).sum::<f64>() / n);
            pi_blocks.iter().map(|b| (b.cx - cx0).hypot(b.cy - cy0)).fold(f64::INFINITY, f64::min)
        };
        let (big_total_left, small_total_right): (Vec<&Bar3DBlock>, Vec<&Bar3DBlock>) = blocks.iter().partition(|b| b.cx < SUBPLOT_GAP / 2.0);
        assert!(local_ring_hole(&small_total_right) < local_ring_hole(&big_total_left));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&PieConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_and_negative_values_never_poison_the_scene() {
        let labels: Vec<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let values = vec![f64::NAN, f64::INFINITY, -5.0];
        for &variant in PieVariant::all() {
            let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
            let (blocks, _) = layout_named(&c, &Budget::default());
            assert!(blocks.iter().all(|b| b.cx.is_finite() && b.cy.is_finite() && b.z0.is_finite() && b.z1.is_finite()), "{variant:?}");
        }
    }
}
