use super::common::prepare;
use super::config::LollipopConfig;
use super::variant::LollipopVariant;
use crate::plot::statistical::_3d::budget::{Buckets, Budget};
use crate::plot::statistical::_3d::curves::{points_of, toned_ribbon};
use crate::plot::statistical::_3d::generic::{diverging_columns, panel_columns, radial_diverging_columns, transposed};
use crate::plot::statistical::_3d::ohlc::{DOWN, UP};
use crate::plot::statistical::_3d::stems::{classed, finite, group_ids, heads, opposed, ring_radius, toned, Head, Tips};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.8;
pub const COLORMAP: &str = "updown";
const STEM_HW: f64 = 0.05;
const HEAD_HW: f64 = 0.3;
const RING_SLOT: f64 = 1.0;
const PANEL_GAP: f64 = 1.5;
const LINK_DEPTH: f64 = 0.06;
const SIGN_NAMES: [&str; 2] = ["positive", "negative"];

#[derive(Clone, Copy)]
enum Layout {
    Row,
    Ring,
    Panels,
    Duel,
}

#[derive(Clone, Copy)]
enum Tone {
    Plain,
    Signed,
    Graded,
    Split,
}

#[derive(Clone, Copy)]
struct Recipe {
    layout: Layout,
    head: Head,
    tone: Tone,
    centered: bool,
    link: bool,
    swap: bool,
}

impl Recipe {
    const fn of(layout: Layout) -> Self {
        Self { layout, head: Head::Cube, tone: Tone::Plain, centered: false, link: false, swap: false }
    }

    const fn headed(mut self, head: Head) -> Self {
        self.head = head;
        self
    }

    const fn toned(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    const fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    const fn linked(mut self) -> Self {
        self.link = true;
        self
    }

    const fn swapped(mut self) -> Self {
        self.swap = true;
        self
    }
}

fn recipe(variant: LollipopVariant) -> Recipe {
    use LollipopVariant::*;
    let row = Recipe::of(Layout::Row);
    match variant {
        Basic => row,
        Cleveland => row.swapped(),
        Diverging => row.toned(Tone::Signed).centered(),
        Circular => Recipe::of(Layout::Ring),
        Office => Recipe::of(Layout::Panels),
        ConditionalColor => row.toned(Tone::Split).swapped(),
        Trend => row.toned(Tone::Graded).linked(),
        Custom => row.headed(Head::Diamond),
        Duel => Recipe::of(Layout::Duel),
    }
}

fn centred(values: &[f64]) -> Vec<f64> {
    let mean = values.iter().sum::<f64>() / values.len().max(1) as f64;
    values.iter().map(|v| v - mean).collect()
}

fn graded(values: &[f64]) -> Vec<f64> {
    let peak = values.iter().map(|v| v.abs()).fold(1e-12, f64::max);
    values.iter().map(|v| (0.5 + 0.5 * v / peak).clamp(0.0, 1.0)).collect()
}

fn spread(plan: Recipe, values: &[f64], ids: &[usize]) -> Vec<Bar3DBlock> {
    let radius = ring_radius(values.len(), RING_SLOT);
    match plan.layout {
        Layout::Row => diverging_columns(values, STEM_HW, STEM_HW),
        Layout::Ring => radial_diverging_columns(values, radius, STEM_HW, STEM_HW),
        Layout::Panels => panel_columns(values, ids, PANEL_GAP, STEM_HW, STEM_HW),
        Layout::Duel => classed(radial_diverging_columns(&opposed(values, ids), radius, STEM_HW, STEM_HW), ids),
    }
}

fn shaded(plan: Recipe, stems: Vec<Bar3DBlock>, values: &[f64]) -> Vec<Bar3DBlock> {
    match plan.tone {
        Tone::Plain => stems,
        Tone::Signed => toned(stems, &values.iter().map(|v| if *v >= 0.0 { UP } else { DOWN }).collect::<Vec<_>>()),
        Tone::Graded => toned(stems, &graded(values)),
        Tone::Split => classed(stems, &values.iter().map(|v| usize::from(*v < 0.0)).collect::<Vec<_>>()),
    }
}

fn linked(values: &[f64]) -> Vec<Bar3DBlock> {
    let tones = graded(values);
    let segments: Vec<f64> = tones.windows(2).map(|w| (w[0] + w[1]) / 2.0).collect();
    toned_ribbon(&points_of(values), &segments, 0.0, LINK_DEPTH, 0)
}

fn names_of(plan: Recipe, labels: &[String], groups: Vec<String>) -> Vec<String> {
    match (plan.layout, plan.tone) {
        (_, Tone::Split) => SIGN_NAMES.iter().map(|name| name.to_string()).collect(),
        (Layout::Panels | Layout::Duel, _) if !groups.is_empty() => groups,
        _ => labels.to_vec(),
    }
}

pub fn layout_3d(cfg: &LollipopConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &LollipopConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values.len());
    let buckets = Buckets::new(n, budget.points);
    if buckets.is_identity() {
        return lollipops_3d(cfg);
    }
    let labels = buckets.first(&cfg.labels[..n]);
    let values = buckets.mean(&cfg.values[..n]);
    let groups = if cfg.groups.len() == n { buckets.first(cfg.groups) } else { Vec::new() };
    lollipops_3d(&LollipopConfig {
        variant: cfg.variant,
        labels: &labels,
        values: &values,
        groups: &groups,
        sort_order: cfg.sort_order,
        ..LollipopConfig::default()
    })
}

fn lollipops_3d(cfg: &LollipopConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(prepared) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let raw = finite(&prepared.values[..prepared.n]);
    let values = if plan.centered { centred(&raw) } else { raw };
    let (ids, groups) = group_ids(&prepared.groups, prepared.n);
    let stems = shaded(plan, spread(plan, &values, &ids), &values);
    let mut blocks = stems.clone();
    blocks.extend(heads(&stems, Tips::Far, plan.head, HEAD_HW, HEIGHT_RATIO));
    if plan.link {
        blocks.extend(linked(&values));
    }
    if plan.swap {
        blocks = transposed(blocks);
    }
    (blocks, names_of(plan, &prepared.labels[..prepared.n], groups))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<String> {
        ["north", "north", "south", "south", "south"].iter().map(|g| g.to_string()).collect()
    }

    fn draw(variant: LollipopVariant, values: &[f64], groups: &[String]) -> (Vec<Bar3DBlock>, Vec<String>) {
        let labels: Vec<String> = (0..values.len()).map(|i| format!("L{i}")).collect();
        let cfg = LollipopConfig { variant, labels: &labels, values, groups, ..LollipopConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_stem_and_a_head_per_value_plus_its_own_extras() {
        for &variant in LollipopVariant::all() {
            let (blocks, names) = draw(variant, &[3.0, 5.0, 2.0, 8.0, 4.0], &groups());
            let expected = match variant {
                LollipopVariant::Custom => 5 + 5 * 4,
                LollipopVariant::Trend => 10 + 4,
                _ => 10,
            };
            assert_eq!(blocks.len(), expected, "{variant:?}");
            assert!(!names.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn diverging_stems_grow_from_the_mean_with_a_tone_per_side() {
        let (blocks, _) = draw(LollipopVariant::Diverging, &[10.0, 20.0, 30.0], &[]);
        assert_eq!((blocks[0].z0, blocks[0].z1, blocks[0].tone), (-10.0, 0.0, Some(DOWN)));
        assert_eq!((blocks[2].z0, blocks[2].z1, blocks[2].tone), (0.0, 10.0, Some(UP)));
    }

    #[test]
    fn conditional_color_splits_the_stems_into_sign_classes_and_runs_horizontally() {
        let (blocks, names) = draw(LollipopVariant::ConditionalColor, &[3.0, -2.0, 5.0, -1.0, 0.0], &[]);
        assert_eq!(blocks[..5].iter().map(|b| b.ci).collect::<Vec<_>>(), vec![0, 1, 0, 1, 0]);
        assert_eq!(names, vec!["positive", "negative"]);
        assert!(blocks.iter().all(|b| b.cx == 0.0));
    }

    #[test]
    fn cleveland_runs_the_categories_along_the_depth_axis() {
        let (blocks, names) = draw(LollipopVariant::Cleveland, &[3.0, 5.0, 2.0], &[]);
        assert!(blocks.iter().all(|b| b.cx == 0.0));
        assert_eq!(blocks[2].cy, 2.0);
        assert_eq!(names, vec!["L0", "L1", "L2"]);
    }

    #[test]
    fn office_lays_the_groups_side_by_side_with_a_gap_and_names_them() {
        let (blocks, names) = draw(LollipopVariant::Office, &[3.0, 5.0, 2.0, 8.0, 4.0], &groups());
        assert_eq!(names, vec!["north", "south"]);
        assert_eq!(blocks[..5].iter().map(|b| b.ci).collect::<Vec<_>>(), vec![0, 0, 1, 1, 1]);
        assert_eq!(blocks[2].cx, 1.0 + 1.0 + PANEL_GAP);
    }

    #[test]
    fn duel_sends_every_second_group_below_the_ring_plane() {
        let sides: Vec<String> = ["a", "b", "a", "b"].iter().map(|g| g.to_string()).collect();
        let (blocks, names) = draw(LollipopVariant::Duel, &[5.0, 6.0, 7.0, 8.0], &sides);
        assert!(blocks[0].z1 > 0.0 && blocks[0].z0 == 0.0);
        assert!(blocks[1].z0 < 0.0 && blocks[1].z1 == 0.0);
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn circular_places_every_stem_on_the_same_ring() {
        let (blocks, _) = draw(LollipopVariant::Circular, &[3.0, 5.0, 2.0, 8.0, 4.0, 6.0], &[]);
        let radius = ring_radius(6, RING_SLOT);
        for stem in &blocks[..6] {
            assert!((stem.cx.hypot(stem.cy) - radius).abs() < 1e-9);
        }
    }

    #[test]
    fn trend_grades_the_tones_and_links_the_heads_with_sloped_segments() {
        let (blocks, _) = draw(LollipopVariant::Trend, &[-2.0, 0.0, 4.0], &[]);
        assert_eq!(blocks[..3].iter().map(|b| b.tone).collect::<Vec<_>>(), vec![Some(0.25), Some(0.5), Some(1.0)]);
        assert_eq!(blocks.iter().filter(|b| b.end.is_some()).count(), 2);
    }

    #[test]
    fn custom_heads_taper_towards_their_ends() {
        let (blocks, _) = draw(LollipopVariant::Custom, &[3.0], &[]);
        assert_eq!(blocks.len(), 5);
        assert!(blocks[1].hw < blocks[2].hw && blocks[4].hw < blocks[3].hw);
        assert!(blocks[2].hw < HEAD_HW);
    }

    #[test]
    fn long_inputs_are_pooled_to_the_budget_with_matching_names() {
        let labels: Vec<String> = (0..1000).map(|i| format!("L{i}")).collect();
        let values: Vec<f64> = (0..1000).map(|i| (i % 17) as f64).collect();
        let cfg = LollipopConfig { labels: &labels, values: &values, ..LollipopConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::new(Some(100)));
        assert_eq!((blocks.len(), names.len()), (200, 100));
        assert_eq!(names[1], "L10");
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = draw(LollipopVariant::Basic, &[], &[]);
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let (blocks, _) = draw(LollipopVariant::Basic, &[1.0, f64::NAN, f64::INFINITY, 2.0], &[]);
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.cx.is_finite()));
    }
}
