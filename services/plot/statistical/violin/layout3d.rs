use super::config::ViolinConfig;
use super::variant::ViolinVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::generic::transposed;
use crate::plot::statistical::_3d::spread::{box_blocks, grouped_by_label, point_blocks, violin_blocks, BoxStyle, Group, Scatter};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::boxplot::common::compute_box;

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "jet";
const SLICES: usize = 24;
const MAX_HW: f64 = 0.36;
const THIN_HW: f64 = 0.14;
const DEPTH: f64 = 0.3;
const PLATE_TONE: f64 = 0.5;
const POINT_SIZE: f64 = 0.09;
const ROW_PITCH: f64 = 1.4;

#[derive(Clone, Copy, PartialEq)]
enum Overlay {
    None,
    Box,
    Quartile,
    Mean,
    Points,
    Strip,
}

#[derive(Clone, Copy, PartialEq)]
enum Side {
    Both,
    Right,
    Alternating,
}

#[derive(Clone, Copy)]
struct Recipe {
    overlay: Overlay,
    side: Side,
    horizontal: bool,
}

impl Recipe {
    const fn of(overlay: Overlay) -> Self {
        Self { overlay, side: Side::Both, horizontal: false }
    }

    const fn sided(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    const fn flipped(mut self) -> Self {
        self.horizontal = true;
        self
    }
}

fn recipe(variant: ViolinVariant) -> Recipe {
    use ViolinVariant::*;
    match variant {
        Basic => Recipe::of(Overlay::None),
        Box => Recipe::of(Overlay::Box),
        Quartile => Recipe::of(Overlay::Quartile),
        Mean => Recipe::of(Overlay::Mean),
        Points => Recipe::of(Overlay::Points),
        Strip => Recipe::of(Overlay::Strip),
        Horizontal => Recipe::of(Overlay::None).flipped(),
        Split => Recipe::of(Overlay::None).sided(Side::Alternating),
        Half => Recipe::of(Overlay::None).sided(Side::Right),
    }
}

fn halved(mut b: Bar3DBlock, right: bool) -> Bar3DBlock {
    let half = b.hw / 2.0;
    b.cx += if right { half } else { -half };
    b.hw = half;
    b
}

fn half_of(blocks: Vec<Bar3DBlock>, right: bool) -> Vec<Bar3DBlock> {
    blocks.into_iter().map(|b| halved(b, right)).collect()
}

fn plate(g: &Group, z: f64, hw: f64) -> Bar3DBlock {
    let thin = (hw * 0.03).max(1e-4);
    Bar3DBlock::new(g.cx, g.cy, z - thin, z + thin, hw * 1.08, DEPTH * 1.08, g.class).with_tone(PLATE_TONE)
}

fn overlay_blocks(groups: &[Group], overlay: Overlay) -> Vec<Bar3DBlock> {
    match overlay {
        Overlay::None => Vec::new(),
        Overlay::Box => box_blocks(groups, &BoxStyle { hw: MAX_HW * 0.5, depth: DEPTH, whisker_hw: MAX_HW * 0.08, notch: false, median: true }),
        Overlay::Quartile => groups
            .iter()
            .flat_map(|g| {
                let s = compute_box(g.samples);
                [plate(g, s.q1, MAX_HW), plate(g, s.median, MAX_HW), plate(g, s.q3, MAX_HW)]
            })
            .collect(),
        Overlay::Mean => groups.iter().map(|g| plate(g, compute_box(g.samples).mean, MAX_HW)).collect(),
        Overlay::Points => point_blocks(groups, Scatter::Aligned, 0.0, POINT_SIZE),
        Overlay::Strip => point_blocks(groups, Scatter::Jitter, THIN_HW * 1.6, POINT_SIZE),
    }
}

pub fn layout_3d(cfg: &ViolinConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &ViolinConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let owned = grouped_by_label(cfg.categories, cfg.values);
    if owned.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let names: Vec<String> = owned.iter().map(|(name, _)| name.clone()).collect();
    let groups: Vec<Group> = owned.iter().enumerate().map(|(i, (_, samples))| Group { samples, cx: i as f64 * ROW_PITCH, cy: 0.0, class: i }).collect();
    let hw = if matches!(plan.overlay, Overlay::Points | Overlay::Strip) { THIN_HW } else { MAX_HW };
    let mut blocks = violin_blocks(&groups, SLICES, hw, DEPTH);
    blocks.extend(overlay_blocks(&groups, plan.overlay));
    let blocks = match plan.side {
        Side::Both => blocks,
        Side::Right => half_of(blocks, true),
        Side::Alternating => blocks.into_iter().map(|b| { let right = b.ci % 2 == 0; halved(b, right) }).collect(),
    };
    let blocks = if plan.horizontal { transposed(blocks) } else { blocks };
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn categories() -> Vec<String> {
        ["A", "A", "A", "A", "A", "A", "B", "B", "B", "B", "B", "B"].iter().map(|s| s.to_string()).collect()
    }

    fn values() -> Vec<f64> {
        vec![1.2, 2.4, 2.7, 3.1, 3.5, 3.8, 5.0, 6.2, 6.6, 7.0, 7.4, 7.9]
    }

    fn draw(variant: ViolinVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let c = categories();
        let v = values();
        let cfg = ViolinConfig { variant, categories: &c, values: &v, ..ViolinConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_violin_variant_draws_something_and_names_every_category() {
        for &variant in ViolinVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn basic_violin_widens_where_the_samples_are_dense() {
        let (blocks, names) = draw(ViolinVariant::Basic);
        let a = names.iter().position(|n| n == "A").unwrap();
        let widest = blocks.iter().filter(|b| b.ci == a).map(|b| b.hw).fold(0.0, f64::max);
        assert!((widest - MAX_HW).abs() < 1e-9);
    }

    #[test]
    fn box_overlay_adds_a_highlighted_median_plate() {
        let (blocks, _) = draw(ViolinVariant::Box);
        assert!(blocks.iter().any(|b| b.tone == Some(PLATE_TONE)));
    }

    #[test]
    fn quartile_overlay_adds_three_plates_per_category() {
        let (basic, _) = draw(ViolinVariant::Basic);
        let (quartile, _) = draw(ViolinVariant::Quartile);
        assert_eq!(quartile.len(), basic.len() + 3 * 2);
    }

    #[test]
    fn points_place_every_raw_sample_and_strip_jitters_it_sideways() {
        let (points, names) = draw(ViolinVariant::Points);
        let a = names.iter().position(|n| n == "A").unwrap();
        let aligned_x: std::collections::BTreeSet<i64> =
            points.iter().filter(|b| b.ci == a && b.hw < THIN_HW * 0.5).map(|b| (b.cx * 1000.0) as i64).collect();
        assert_eq!(aligned_x.len(), 1);
        let (strip, _) = draw(ViolinVariant::Strip);
        let jittered_x: std::collections::BTreeSet<i64> =
            strip.iter().filter(|b| b.ci == a && b.hw < THIN_HW * 0.5).map(|b| (b.cx * 1000.0) as i64).collect();
        assert!(jittered_x.len() > 1);
    }

    #[test]
    fn horizontal_swaps_the_footprint_axes_of_the_basic_layout() {
        let (basic, names) = draw(ViolinVariant::Basic);
        let (horizontal, _) = draw(ViolinVariant::Horizontal);
        assert_eq!(basic.len(), horizontal.len());
        let b = names.iter().position(|n| n == "B").unwrap();
        let basic_b = basic.iter().find(|blk| blk.ci == b).unwrap();
        let horizontal_b = horizontal.iter().find(|blk| blk.ci == b).unwrap();
        assert_eq!(basic_b.cx, horizontal_b.cy);
        assert_eq!(basic_b.cy, horizontal_b.cx);
    }

    #[test]
    fn half_only_extends_to_one_side_of_the_centre_line() {
        let (blocks, names) = draw(ViolinVariant::Half);
        let a = names.iter().position(|n| n == "A").unwrap();
        assert!(blocks.iter().filter(|b| b.ci == a).all(|b| b.cx >= a as f64 * ROW_PITCH));
    }

    #[test]
    fn split_alternates_sides_between_neighbouring_categories() {
        let (blocks, names) = draw(ViolinVariant::Split);
        let a = names.iter().position(|n| n == "A").unwrap();
        let b = names.iter().position(|n| n == "B").unwrap();
        let a_right = blocks.iter().filter(|blk| blk.ci == a).all(|blk| blk.cx >= a as f64 * ROW_PITCH);
        let b_left = blocks.iter().filter(|blk| blk.ci == b).all(|blk| blk.cx <= b as f64 * ROW_PITCH);
        assert!(a_right && b_left);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&ViolinConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_sample_set_still_produces_a_bounded_number_of_blocks() {
        let categories: Vec<String> = (0..200_000).map(|i| if i % 2 == 0 { "A".to_string() } else { "B".to_string() }).collect();
        let values: Vec<f64> = (0..200_000).map(|i| ((i * 7919) % 10_007) as f64).collect();
        let cfg = ViolinConfig { categories: &categories, values: &values, ..ViolinConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 10_000);
    }
}
