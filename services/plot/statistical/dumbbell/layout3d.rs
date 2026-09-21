use super::common::prepare;
use super::config::DumbbellConfig;
use super::variant::DumbbellVariant;
use crate::plot::statistical::_3d::budget::{pick, Buckets, Budget};
use crate::plot::statistical::_3d::generic::plate_columns;
use crate::plot::statistical::_3d::ohlc::{DOWN, UP};
use crate::plot::statistical::_3d::stems::{classed, dashed, finite, headed_at, toned, Head};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.8;
const START_TONE: f64 = 0.1;
const END_TONE: f64 = 0.9;
const STEM_TONE: f64 = 0.5;
const DASHES: usize = 12;
const DASHED_ROWS: usize = 400;
const HALO: f64 = 2.0;
const PODIUM: f64 = 0.04;
const TILE_HW: f64 = 0.45;

#[derive(Clone, Copy)]
enum Weight {
    Fine,
    Light,
    Heavy,
}

impl Weight {
    const fn stem(self) -> f64 {
        match self {
            Weight::Fine => 0.03,
            Weight::Light => 0.05,
            Weight::Heavy => 0.16,
        }
    }

    const fn head(self) -> f64 {
        match self {
            Weight::Fine => 0.14,
            Weight::Light => 0.24,
            Weight::Heavy => 0.4,
        }
    }
}

#[derive(Clone, Copy)]
struct Recipe {
    weight: Weight,
    end: Head,
    dashed: bool,
    halo: bool,
    ranked: bool,
    directional: bool,
    cmap: &'static str,
}

impl Recipe {
    const fn of(weight: Weight) -> Self {
        Self { weight, end: Head::Cube, dashed: false, halo: false, ranked: false, directional: false, cmap: "cyber" }
    }

    const fn pointing(mut self) -> Self {
        self.end = Head::Arrow;
        self
    }

    const fn dotted(mut self) -> Self {
        self.dashed = true;
        self
    }

    const fn haloed(mut self) -> Self {
        self.halo = true;
        self
    }

    const fn ranked(mut self) -> Self {
        self.ranked = true;
        self
    }

    const fn directional(mut self) -> Self {
        self.directional = true;
        self.cmap = "updown";
        self
    }
}

fn recipe(variant: DumbbellVariant) -> Recipe {
    use DumbbellVariant::*;
    let light = Recipe::of(Weight::Light);
    match variant {
        Basic => light,
        Arrow => light.pointing(),
        Delta => light.directional(),
        Barbell => Recipe::of(Weight::Heavy),
        Glow => light.haloed(),
        Dotted => Recipe::of(Weight::Fine).dotted(),
        Ranked => light.ranked(),
    }
}

pub fn colormap(variant: DumbbellVariant) -> &'static str {
    recipe(variant).cmap
}

struct Rows {
    labels: Vec<String>,
    start: Vec<f64>,
    end: Vec<f64>,
}

impl Rows {
    fn ordered(plan: Recipe, labels: &[String], start: &[f64], end: &[f64]) -> Self {
        let (start, end) = (finite(start), finite(end));
        let mut order: Vec<usize> = (0..start.len().min(end.len())).collect();
        if plan.ranked {
            order.sort_by(|&a, &b| end[b].total_cmp(&end[a]));
        }
        Self { labels: pick(labels, &order), start: pick(&start, &order), end: pick(&end, &order) }
    }

    fn len(&self) -> usize {
        self.start.len()
    }

    fn span(&self) -> (f64, f64) {
        let both = self.start.iter().chain(&self.end).copied();
        both.fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)))
    }
}

fn direction_tones(rows: &Rows) -> Vec<f64> {
    rows.start.iter().zip(&rows.end).map(|(s, e)| if e >= s { UP } else { DOWN }).collect()
}

fn side(stems: &[Bar3DBlock], side: usize, tones: &[f64]) -> Vec<Bar3DBlock> {
    stems
        .iter()
        .zip(tones)
        .enumerate()
        .map(|(i, (stem, &tone))| Bar3DBlock { ci: 2 * i + side, ..*stem }.with_tone(tone))
        .collect()
}

fn dumbbells_3d(cfg: &DumbbellConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(prepared) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let rows = Rows::ordered(plan, &prepared.labels, &prepared.start, &prepared.end);
    let n = rows.len();
    let (floor, ceiling) = rows.span();
    let (from, to) = (rows.start.iter().zip(&rows.end)).map(|(s, e)| (s.min(*e), s.max(*e))).unzip::<f64, f64, Vec<f64>, Vec<f64>>();
    let stems: Vec<Bar3DBlock> = (0..n)
        .map(|i| Bar3DBlock::new(i as f64, 0.0, from[i], to[i], plan.weight.stem(), plan.weight.stem(), 2 * i))
        .collect();
    let (start_tones, stem_tones, end_tones) = if plan.directional {
        let tones = direction_tones(&rows);
        (vec![STEM_TONE; n], tones.clone(), tones)
    } else {
        (vec![START_TONE; n], vec![STEM_TONE; n], vec![END_TONE; n])
    };
    let rods = toned(stems, &stem_tones);
    let start_rods = side(&rods, 0, &start_tones);
    let end_rods = side(&rods, 1, &end_tones);
    let head = plan.weight.head();
    let mut blocks = if plan.dashed { dashed(&rods, DASHES) } else { rods.clone() };
    blocks.extend(headed_at(&start_rods, &rows.start, Head::Cube, head, HEIGHT_RATIO));
    blocks.extend(headed_at(&end_rods, &rows.end, plan.end, head, HEIGHT_RATIO));
    if plan.halo {
        blocks.extend(headed_at(&start_rods, &rows.start, Head::Plate, head * HALO, HEIGHT_RATIO));
        blocks.extend(headed_at(&end_rods, &rows.end, Head::Plate, head * HALO, HEIGHT_RATIO));
    }
    if plan.ranked {
        let ranks: Vec<f64> = (0..n).map(|i| 1.0 - i as f64 / (n.max(2) - 1) as f64).collect();
        let tiles = plate_columns(&vec![floor; n], (ceiling - floor).max(1e-9) * PODIUM, TILE_HW, TILE_HW);
        blocks.extend(toned(classed(tiles, &(0..n).map(|i| 2 * i).collect::<Vec<_>>()), &ranks));
    }
    let names = rows
        .labels
        .iter()
        .flat_map(|label| [format!("{label} · {}", cfg.series_names.0), format!("{label} · {}", cfg.series_names.1)])
        .collect();
    (blocks, names)
}

pub fn layout_3d(cfg: &DumbbellConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &DumbbellConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values_start.len()).min(cfg.values_end.len());
    let cap = if recipe(cfg.variant).dashed { budget.points.min(DASHED_ROWS) } else { budget.points };
    let buckets = Buckets::new(n, cap);
    if buckets.is_identity() {
        return dumbbells_3d(cfg);
    }
    let labels = buckets.first(&cfg.labels[..n]);
    let start = buckets.mean(&cfg.values_start[..n]);
    let end = buckets.mean(&cfg.values_end[..n]);
    dumbbells_3d(&DumbbellConfig {
        variant: cfg.variant,
        labels: &labels,
        values_start: &start,
        values_end: &end,
        series_names: cfg.series_names,
        sort_order: cfg.sort_order,
        ..DumbbellConfig::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const START: [f64; 5] = [20.0, 35.0, 15.0, 42.0, 28.0];
    const END: [f64; 5] = [60.0, 52.0, 38.0, 68.0, 55.0];

    fn labels(n: usize) -> Vec<String> {
        (0..n).map(|i| ((b'A' + (i % 26) as u8) as char).to_string()).collect()
    }

    fn draw(variant: DumbbellVariant, start: &[f64], end: &[f64]) -> (Vec<Bar3DBlock>, Vec<String>) {
        let names = labels(start.len());
        let cfg = DumbbellConfig { variant, labels: &names, values_start: start, values_end: end, ..DumbbellConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_rod_and_two_ends_plus_its_own_extras() {
        for &variant in DumbbellVariant::all() {
            let (blocks, names) = draw(variant, &START, &END);
            let expected = match variant {
                DumbbellVariant::Arrow => 5 + 5 + 5 * 3,
                DumbbellVariant::Glow => 15 + 10,
                DumbbellVariant::Dotted => 5 * DASHES + 10,
                DumbbellVariant::Ranked => 15 + 5,
                _ => 15,
            };
            assert_eq!(blocks.len(), expected, "{variant:?}");
            assert_eq!(names.len(), 10, "{variant:?}");
        }
    }

    #[test]
    fn every_row_names_its_two_ends_after_the_series() {
        let (_, names) = draw(DumbbellVariant::Basic, &START, &END);
        assert_eq!(&names[..4], ["A · Start", "A · End", "B · Start", "B · End"]);
    }

    #[test]
    fn the_ends_take_opposite_tones_and_the_rod_sits_in_between() {
        let (blocks, _) = draw(DumbbellVariant::Basic, &START, &END);
        assert_eq!(blocks[0].tone, Some(STEM_TONE));
        assert_eq!(blocks[5].tone, Some(START_TONE));
        assert_eq!(blocks[10].tone, Some(END_TONE));
        assert_eq!((blocks[5].ci, blocks[10].ci), (0, 1));
    }

    #[test]
    fn delta_tones_the_rod_and_the_end_by_the_direction_of_the_change() {
        let (blocks, _) = draw(DumbbellVariant::Delta, &[10.0, 50.0], &[30.0, 20.0]);
        assert_eq!((blocks[0].tone, blocks[1].tone), (Some(UP), Some(DOWN)));
        assert_eq!((blocks[4].tone, blocks[5].tone), (Some(UP), Some(DOWN)));
        assert_eq!(colormap(DumbbellVariant::Delta), "updown");
        assert_eq!(colormap(DumbbellVariant::Basic), "cyber");
    }

    #[test]
    fn ranked_orders_the_rows_by_their_end_value_and_lays_a_tile_under_each() {
        let (blocks, names) = draw(DumbbellVariant::Ranked, &START, &END);
        assert_eq!(&names[..6], ["D · Start", "D · End", "A · Start", "A · End", "E · Start", "E · End"]);
        let tiles = &blocks[15..];
        assert_eq!(tiles.len(), 5);
        assert!(tiles.iter().all(|t| t.z1 <= 15.0 + 1e-9));
        assert!(tiles[0].tone > tiles[4].tone);
    }

    #[test]
    fn arrows_point_beyond_the_end_in_the_direction_of_the_change() {
        let (falling, _) = draw(DumbbellVariant::Arrow, &[50.0], &[10.0]);
        let tips = &falling[2..];
        assert_eq!(tips.len(), 3);
        assert!(tips.iter().all(|t| t.z1 <= 10.0 + 1e-9));
        let (rising, _) = draw(DumbbellVariant::Arrow, &[10.0], &[50.0]);
        assert!(rising[2..].iter().all(|t| t.z0 >= 50.0 - 1e-9));
    }

    #[test]
    fn barbells_are_heavier_than_the_basic_dumbbell_and_dotted_ones_lighter() {
        let hw = |variant| draw(variant, &START, &END).0[0].hw;
        assert!(hw(DumbbellVariant::Barbell) > hw(DumbbellVariant::Basic));
        assert!(hw(DumbbellVariant::Dotted) < hw(DumbbellVariant::Basic));
    }

    #[test]
    fn glow_adds_a_wide_flat_halo_around_each_end() {
        let (blocks, _) = draw(DumbbellVariant::Glow, &START, &END);
        let (head, halo) = (blocks[5], blocks[15]);
        assert!(halo.hw > head.hw * 1.5);
        assert!(halo.z1 - halo.z0 < head.z1 - head.z0);
    }

    #[test]
    fn long_inputs_are_pooled_to_the_budget_with_matching_names() {
        let start: Vec<f64> = (0..1000).map(|i| (i % 13) as f64).collect();
        let end: Vec<f64> = (0..1000).map(|i| (i % 17) as f64 + 5.0).collect();
        let names = labels(1000);
        let cfg = DumbbellConfig { labels: &names, values_start: &start, values_end: &end, ..DumbbellConfig::default() };
        let (blocks, out) = layout_named(&cfg, &Budget::new(Some(100)));
        assert_eq!((blocks.len(), out.len()), (300, 200));
    }

    #[test]
    fn dotted_rows_are_capped_lower_so_the_dashes_stay_within_the_block_budget() {
        let start = vec![1.0; 5000];
        let end = vec![9.0; 5000];
        let names = labels(5000);
        let cfg = DumbbellConfig { variant: DumbbellVariant::Dotted, labels: &names, values_start: &start, values_end: &end, ..DumbbellConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::new(Some(4000)));
        assert_eq!(blocks.len(), DASHED_ROWS * DASHES + 2 * DASHED_ROWS);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = draw(DumbbellVariant::Basic, &[], &[]);
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let (blocks, _) = draw(DumbbellVariant::Basic, &[1.0, f64::NAN], &[f64::INFINITY, 2.0]);
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.cx.is_finite()));
    }
}
