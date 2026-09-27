use super::config::PieConfig;
use super::variant::PieVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::{branch_tone, MIN_SPAN};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::{PI, TAU};

const HEIGHT: f64 = 1.0;
const CELL_ROWS: usize = 10;
const CELL_COLS: usize = 10;
const DONUT_INNER: f64 = 0.42;
const NESTED_INNER1: f64 = 0.32;
const NESTED_OUTER1: f64 = 0.62;
const NESTED_INNER2: f64 = 0.68;
const NESTED_OUTER2: f64 = 1.0;
const NIGHT_INNER: f64 = 0.15;
const NIGHT_GAP: f64 = 0.94;
const PULL_FRAC: f64 = 0.18;
const SUBPLOT_GAP_FRAC: f64 = 2.3;

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

pub fn layout_3d(cfg: &PieConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &PieConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let values = finite(&cfg.values[..n]);
    let names = names_for(n, cfg.labels);
    (waffle_cells(&values, HEIGHT), names)
}

#[derive(Default)]
pub struct Wedges {
    pub a0: Vec<f64>,
    pub a1: Vec<f64>,
    pub inner_r: Vec<f64>,
    pub outer_r: Vec<f64>,
    pub cx: Vec<f64>,
    pub cy: Vec<f64>,
    pub color_idx: Vec<f64>,
    pub names: Vec<String>,
}

fn ring(w: &mut Wedges, spans: &[(f64, f64)], inner: f64, outer: f64, cx: f64, cy: f64, names: &[String]) {
    for (i, &(a0, a1)) in spans.iter().enumerate() {
        if a1 - a0 <= MIN_SPAN {
            continue;
        }
        w.a0.push(a0);
        w.a1.push(a1);
        w.inner_r.push(inner);
        w.outer_r.push(outer);
        w.cx.push(cx);
        w.cy.push(cy);
        w.color_idx.push(i as f64);
        w.names.push(names.get(i).cloned().unwrap_or_default());
    }
}

fn nested(w: &mut Wedges, values: &[f64], secondary: &[f64], names: &[String], secondary_labels: &[String]) {
    ring(w, &value_spans(values, 0.0, TAU), NESTED_INNER1, NESTED_OUTER1, 0.0, 0.0, names);
    let sec_names = names_for(secondary.len(), secondary_labels);
    ring(w, &value_spans(secondary, 0.0, TAU), NESTED_INNER2, NESTED_OUTER2, 0.0, 0.0, &sec_names);
}

fn nightingale(w: &mut Wedges, values: &[f64], names: &[String]) {
    let n = values.len();
    if n == 0 {
        return;
    }
    let vmax = values.iter().cloned().filter(|v| v.is_finite()).fold(0.0_f64, f64::max).max(1e-9);
    let step = TAU / n as f64;
    for i in 0..n {
        let a0 = i as f64 * step;
        let a1 = a0 + step * NIGHT_GAP;
        let outer = (NIGHT_INNER + (values[i].max(0.0) / vmax) * (1.0 - NIGHT_INNER)).max(NIGHT_INNER + 0.05);
        w.a0.push(a0);
        w.a1.push(a1);
        w.inner_r.push(NIGHT_INNER);
        w.outer_r.push(outer);
        w.cx.push(0.0);
        w.cy.push(0.0);
        w.color_idx.push(i as f64);
        w.names.push(names.get(i).cloned().unwrap_or_default());
    }
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

fn apply_pull(w: &mut Wedges, pull: &[f64]) {
    for i in 0..w.a0.len() {
        let ci = w.color_idx[i] as usize;
        let p = pull.get(ci).copied().unwrap_or(0.0).max(0.0);
        if p > 0.0 {
            let mid = (w.a0[i] + w.a1[i]) / 2.0;
            w.cx[i] += mid.cos() * p * PULL_FRAC;
            w.cy[i] += mid.sin() * p * PULL_FRAC;
        }
    }
}

fn subplot_wedges(w: &mut Wedges, cfg: &PieConfig) {
    let series: Vec<Vec<f64>> = cfg.series.iter().map(|s| finite(s)).collect();
    let n_pies = series.len();
    if n_pies == 0 {
        return;
    }
    let cols = (n_pies as f64).sqrt().ceil().max(1.0) as usize;
    let proportional = cfg.proportional || matches!(cfg.variant, PieVariant::Proportional);
    let totals: Vec<f64> = series.iter().map(|s| s.iter().filter(|v| v.is_finite() && **v >= 0.0).sum()).collect();
    let max_total = totals.iter().cloned().fold(0.0_f64, f64::max).max(1e-9);
    let names = subplot_names(cfg.labels, cfg.series);
    for (pi, vals) in series.iter().enumerate() {
        let row = (pi / cols) as f64;
        let col = (pi % cols) as f64;
        let ox = col * SUBPLOT_GAP_FRAC;
        let oy = -row * SUBPLOT_GAP_FRAC;
        let scale = if proportional { (totals[pi] / max_total).sqrt().max(0.3) } else { 1.0 };
        ring(w, &value_spans(vals, 0.0, TAU), 0.0, scale, ox, oy, &names);
    }
}

pub fn wedges(cfg: &PieConfig) -> Option<Wedges> {
    let mut w = Wedges::default();
    if !cfg.series.is_empty() && matches!(cfg.variant, PieVariant::Subplots | PieVariant::Proportional) {
        subplot_wedges(&mut w, cfg);
    } else {
        let n = cfg.labels.len().min(cfg.values.len());
        if n == 0 {
            return None;
        }
        let values = finite(&cfg.values[..n]);
        let names = names_for(n, cfg.labels);
        match cfg.variant {
            PieVariant::Semi => ring(&mut w, &value_spans(&values, 0.0, PI), 0.0, 1.0, 0.0, 0.0, &names),
            PieVariant::Donut | PieVariant::Kpi => ring(&mut w, &value_spans(&values, 0.0, TAU), DONUT_INNER, 1.0, 0.0, 0.0, &names),
            PieVariant::Nested => nested(&mut w, &values, &finite(cfg.secondary_values), &names, cfg.secondary_labels),
            PieVariant::Nightingale => nightingale(&mut w, &values, &names),
            PieVariant::Exploded => {
                ring(&mut w, &value_spans(&values, 0.0, TAU), 0.0, 1.0, 0.0, 0.0, &names);
                let pull = if cfg.pull.is_empty() { auto_pull(&values) } else { finite(cfg.pull) };
                apply_pull(&mut w, &pull);
            }
            _ => ring(&mut w, &value_spans(&values, 0.0, TAU), 0.0, 1.0, 0.0, 0.0, &names),
        }
    }
    if w.a0.is_empty() {
        None
    } else {
        Some(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(variant: PieVariant, labels: &'static [&'static str], values: &'static [f64]) -> (Vec<String>, Vec<f64>, PieVariant) {
        (labels.iter().map(|s| s.to_string()).collect(), values.to_vec(), variant)
    }

    fn draw(variant: PieVariant) -> Wedges {
        let (labels, values, variant) = cfg(variant, &["A", "B", "C"], &[30.0, 50.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        wedges(&c).unwrap()
    }

    #[test]
    fn every_non_waffle_variant_draws_something_and_names_every_slice() {
        for &variant in PieVariant::all() {
            if matches!(variant, PieVariant::Waffle) {
                continue;
            }
            let w = draw(variant);
            assert!(!w.a0.is_empty(), "{variant:?}");
            assert_eq!(w.a0.len(), w.names.len(), "{variant:?}");
        }
    }

    #[test]
    fn a_slices_share_of_the_whole_circle_is_proportional_to_its_value_not_equal_spacing() {
        let w = draw(PieVariant::Basic);
        let span_of = |i: usize| w.a1[i] - w.a0[i];
        assert!(span_of(1) > span_of(0) && span_of(0) > span_of(2), "B(50) > A(30) > C(20) in angular span");
    }

    #[test]
    fn every_basic_wedge_sits_at_the_same_radius_forming_one_disc() {
        let w = draw(PieVariant::Basic);
        assert!(w.inner_r.iter().all(|&r| r == 0.0));
        let o0 = w.outer_r[0];
        assert!(w.outer_r.iter().all(|&r| (r - o0).abs() < 1e-9));
    }

    #[test]
    fn donut_leaves_a_bigger_hole_at_the_centre_than_basic() {
        let basic = draw(PieVariant::Basic);
        let donut = draw(PieVariant::Donut);
        assert!(donut.inner_r[0] > basic.inner_r[0]);
    }

    #[test]
    fn nested_places_the_secondary_ring_farther_out_than_the_primary_one() {
        let (labels, values, variant) = cfg(PieVariant::Nested, &["A", "B", "C"], &[30.0, 50.0, 20.0]);
        let secondary_values = vec![10.0, 15.0, 5.0];
        let secondary_labels: Vec<String> = ["X", "Y", "Z"].iter().map(|s| s.to_string()).collect();
        let c = PieConfig { variant, labels: &labels, values: &values, secondary_values: &secondary_values, secondary_labels: &secondary_labels, ..PieConfig::default() };
        let w = wedges(&c).unwrap();
        assert_eq!(w.names, vec!["A", "B", "C", "X", "Y", "Z"]);
        assert!(w.inner_r[3] > w.outer_r[0]);
    }

    #[test]
    fn semi_spans_only_a_half_circle() {
        let w = draw(PieVariant::Semi);
        assert!(w.a1.last().unwrap() - w.a0[0] <= PI + 1e-6);
    }

    #[test]
    fn exploded_pushes_the_pulled_slice_farther_from_the_centre() {
        let (labels, values, variant) = cfg(PieVariant::Exploded, &["A", "B", "C"], &[30.0, 90.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        let w = wedges(&c).unwrap();
        let r = |i: usize| w.cx[i].hypot(w.cy[i]);
        assert!(r(1) > r(0) && r(1) > r(2));
    }

    #[test]
    fn nightingale_gives_the_largest_value_the_longest_spoke() {
        let (labels, values, variant) = cfg(PieVariant::Nightingale, &["A", "B", "C"], &[10.0, 90.0, 20.0]);
        let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
        let w = wedges(&c).unwrap();
        assert!(w.outer_r[1] > w.outer_r[0] && w.outer_r[1] > w.outer_r[2]);
    }

    #[test]
    fn nightingale_keeps_every_slice_at_the_same_fixed_angle_regardless_of_its_value() {
        let w = draw(PieVariant::Nightingale);
        let step = TAU / 3.0;
        for i in 0..3 {
            let expected = i as f64 * step;
            assert!((w.a0[i] - expected).abs() < 1e-6, "slice {i} at {}, expected {expected}", w.a0[i]);
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
        let w = wedges(&c).unwrap();
        assert_eq!(w.names.len(), 12);
        assert!(w.names.iter().all(|n| labels.contains(n)));
        let centers: std::collections::HashSet<(i64, i64)> = w.cx.iter().zip(w.cy.iter()).map(|(&x, &y)| ((x * 10.0) as i64, (y * 10.0) as i64)).collect();
        assert!(centers.len() > 1, "subplots must not all land on the same grid position");
    }

    #[test]
    fn proportional_shrinks_a_smaller_total_series_ring() {
        let labels: Vec<String> = ["A", "B"].iter().map(|s| s.to_string()).collect();
        let series = vec![vec![90.0, 90.0], vec![5.0, 5.0]];
        let c = PieConfig { variant: PieVariant::Proportional, labels: &labels, series: &series, proportional: true, ..PieConfig::default() };
        let w = wedges(&c).unwrap();
        let (big_total, small_total): (Vec<usize>, Vec<usize>) = (0..w.a0.len()).partition(|&i| w.cx[i] < SUBPLOT_GAP_FRAC / 2.0);
        assert!(w.outer_r[small_total[0]] < w.outer_r[big_total[0]]);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(wedges(&PieConfig::default()).is_none());
    }

    #[test]
    fn non_finite_and_negative_values_never_poison_the_scene() {
        let labels: Vec<String> = ["A", "B", "C"].iter().map(|s| s.to_string()).collect();
        let values = vec![f64::NAN, f64::INFINITY, -5.0];
        for &variant in PieVariant::all() {
            let c = PieConfig { variant, labels: &labels, values: &values, ..PieConfig::default() };
            if matches!(variant, PieVariant::Waffle) {
                let (blocks, _) = layout_named(&c, &Budget::default());
                assert!(blocks.iter().all(|b| b.cx.is_finite() && b.cy.is_finite()), "{variant:?}");
                continue;
            }
            if let Some(w) = wedges(&c) {
                assert!(w.a0.iter().chain(&w.a1).chain(&w.cx).chain(&w.cy).all(|v| v.is_finite()), "{variant:?}");
            }
        }
    }
}
