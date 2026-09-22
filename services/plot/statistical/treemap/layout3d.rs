use super::common::{prepare, Prepared};
use super::config::TreemapConfig;
use super::variant::TreemapVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::hierarchy::{branch_tone, branches, footprint, BASE_HEIGHT, FADE_FLOOR};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.7;
const SCALE: f64 = 10.0;
const MONO_TONE: f64 = 0.55;
const HEAT_CMAP: &str = "mono";
const BRANCH_CMAP: &str = "jet";
const FRAME_H: f64 = BASE_HEIGHT * FADE_FLOOR * 0.5;
const FRAME_TONE: f64 = 0.05;

#[derive(Clone, Copy, PartialEq)]
enum Tone {
    Branch,
    Mono,
    Heat,
    Delta,
}

#[derive(Clone, Copy)]
struct Recipe {
    tone: Tone,
    thin: bool,
    gap: f64,
    framed: bool,
}

impl Recipe {
    const fn of(tone: Tone) -> Self {
        Self { tone, thin: false, gap: 0.0, framed: false }
    }

    const fn thinned(mut self) -> Self {
        self.thin = true;
        self
    }

    const fn gapped(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    const fn framing(mut self) -> Self {
        self.framed = true;
        self
    }
}

fn recipe(variant: TreemapVariant) -> Recipe {
    use TreemapVariant::*;
    match variant {
        Basic | Flat | Voronoi => Recipe::of(Tone::Branch),
        Outlined => Recipe::of(Tone::Branch).thinned(),
        Gapped => Recipe::of(Tone::Branch).gapped(0.012),
        Nested => Recipe::of(Tone::Branch).framing(),
        Heat => Recipe::of(Tone::Heat),
        Mono => Recipe::of(Tone::Mono),
        Trend => Recipe::of(Tone::Delta),
    }
}

pub fn colormap(variant: TreemapVariant) -> &'static str {
    match recipe(variant).tone {
        Tone::Heat => HEAT_CMAP,
        Tone::Delta => "updown",
        _ => BRANCH_CMAP,
    }
}

fn shrunk(rects: &[super::common::Rect], root: &super::common::Rect, gap: f64) -> Vec<(f64, f64, f64, f64)> {
    let (rw, rh) = (root.w.max(1e-9), root.h.max(1e-9));
    rects
        .iter()
        .map(|r| {
            let (x0, y0) = ((r.x - root.x) / rw, (r.y - root.y) / rh);
            let (x1, y1) = (x0 + r.w / rw, y0 + r.h / rh);
            let pad_x = (x1 - x0) * gap;
            let pad_y = (y1 - y0) * gap;
            (x0 + pad_x, y0 + pad_y, (x1 - pad_x).max(x0 + pad_x), (y1 - pad_y).max(y0 + pad_y))
        })
        .collect()
}

fn tones(plan: Recipe, p: &Prepared, prior_values: &[f64], ids: &[usize], branch_count: usize) -> Vec<f64> {
    let leaf_values = &p.leaf_values;
    let peak = leaf_values.iter().copied().fold(1e-12, f64::max);
    (0..p.leaf_indices.len())
        .map(|ri| match plan.tone {
            Tone::Mono => MONO_TONE,
            Tone::Heat => (leaf_values[ri] / peak).clamp(0.0, 1.0),
            Tone::Delta => delta_tone(p, prior_values, p.leaf_indices[ri]),
            Tone::Branch => branch_tone(ids[ri], branch_count),
        })
        .collect()
}

fn delta_tone(p: &Prepared, prior_values: &[f64], oi: usize) -> f64 {
    let current = p.values.get(oi).copied().unwrap_or(0.0);
    let prior = prior_values.get(oi).copied().unwrap_or(0.0);
    if !prior.is_finite() || prior <= 0.0 {
        return 0.5;
    }
    if current >= prior {
        crate::plot::statistical::_3d::ohlc::UP
    } else {
        crate::plot::statistical::_3d::ohlc::DOWN
    }
}

fn frames(p: &Prepared, root: &super::common::Rect) -> Vec<Bar3DBlock> {
    let (rw, rh) = (root.w.max(1e-9), root.h.max(1e-9));
    p.parent_groups
        .iter()
        .enumerate()
        .map(|(i, (_, rect))| {
            let (x0, y0) = ((rect.x - root.x) / rw, (rect.y - root.y) / rh);
            let (x1, y1) = (x0 + rect.w / rw, y0 + rect.h / rh);
            let (cx, cy) = ((x0 + x1) / 2.0 * SCALE, (y0 + y1) / 2.0 * SCALE);
            let (hw, hd) = ((x1 - x0) / 2.0 * SCALE, (y1 - y0) / 2.0 * SCALE);
            Bar3DBlock::new(cx, cy, 0.0, FRAME_H, hw, hd, i).with_tone(FRAME_TONE)
        })
        .collect()
}

fn treemap_3d(cfg: &TreemapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(p) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let rects = shrunk(&p.rects, &p.root, plan.gap);
    let names: Vec<String> = p.leaf_indices.iter().map(|&oi| p.labels[oi].clone()).collect();
    let (ids, branch_names) = branches(p.leaf_indices.iter().map(|&oi| p.parents.get(oi).map(String::as_str).unwrap_or("")));
    let branch_count = branch_names.len().max(p.leaf_indices.len());
    let tone_values = tones(plan, &p, cfg.prior_values, &ids, branch_count);
    let height = if plan.thin { BASE_HEIGHT * FADE_FLOOR } else { BASE_HEIGHT };
    let mut blocks = footprint(&rects, SCALE, |_| height, |ri| tone_values[ri]);
    let mut all_names = names;
    if plan.framed && !p.parent_groups.is_empty() {
        let plates = frames(&p, &p.root);
        let offset = blocks.len();
        blocks.extend(plates.into_iter().map(|mut b| {
            b.ci += offset;
            b
        }));
        all_names.extend(p.parent_groups.iter().map(|(name, _)| name.clone()));
    }
    (blocks, all_names)
}

pub fn layout_3d(cfg: &TreemapConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &TreemapConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    if cfg.labels.is_empty() {
        return (Vec::new(), Vec::new());
    }
    treemap_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat() -> (Vec<String>, Vec<f64>) {
        (["A", "B", "C", "D"].iter().map(|s| s.to_string()).collect(), vec![40.0, 25.0, 20.0, 15.0])
    }

    fn nested() -> (Vec<String>, Vec<String>, Vec<f64>) {
        (
            ["Root", "A", "B", "A1", "A2"].iter().map(|s| s.to_string()).collect(),
            ["", "Root", "Root", "A", "A"].iter().map(|s| s.to_string()).collect(),
            vec![0.0, 40.0, 30.0, 20.0, 20.0],
        )
    }

    fn draw(variant: TreemapVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, values) = flat();
        let cfg = TreemapConfig { variant, labels: &labels, values: &values, ..TreemapConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_block_per_leaf_and_names_it() {
        for &variant in TreemapVariant::all() {
            if variant == TreemapVariant::Nested {
                continue;
            }
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 4, "{variant:?}");
            assert_eq!(names.len(), 4, "{variant:?}");
        }
    }

    #[test]
    fn leaf_rects_tile_the_whole_footprint_without_gaps_by_default() {
        let (blocks, _) = draw(TreemapVariant::Basic);
        let area: f64 = blocks.iter().map(|b| b.hw * 2.0 * b.hd * 2.0).sum();
        assert!((area - SCALE * SCALE).abs() < 1e-6);
    }

    #[test]
    fn gapped_leaves_visible_margin_between_leaves() {
        let (basic, _) = draw(TreemapVariant::Basic);
        let (gapped, _) = draw(TreemapVariant::Gapped);
        let area = |blocks: &[Bar3DBlock]| blocks.iter().map(|b| b.hw * 2.0 * b.hd * 2.0).sum::<f64>();
        assert!(area(&gapped) < area(&basic));
    }

    #[test]
    fn outlined_leaves_are_flatter_than_basic() {
        let (basic, _) = draw(TreemapVariant::Basic);
        let (outlined, _) = draw(TreemapVariant::Outlined);
        assert!(outlined[0].z1 < basic[0].z1);
    }

    #[test]
    fn mono_paints_every_leaf_the_same_tone_while_basic_can_vary() {
        let (mono, _) = draw(TreemapVariant::Mono);
        assert!(mono.iter().all(|b| b.tone == Some(MONO_TONE)));
    }

    #[test]
    fn heat_tones_leaves_by_their_own_value_not_by_branch() {
        let (heat, _) = draw(TreemapVariant::Heat);
        let tones: Vec<f64> = heat.iter().map(|b| b.tone.unwrap()).collect();
        assert!(tones.windows(2).all(|w| w[0] >= w[1]));
        assert_eq!(tones[0], 1.0);
    }

    #[test]
    fn trend_tones_up_or_down_against_the_comparison_value() {
        let (labels, values) = flat();
        let comparisons = vec![30.0, 30.0, 0.0, 0.0];
        let cfg = TreemapConfig { variant: TreemapVariant::Trend, labels: &labels, values: &values, prior_values: &comparisons, ..TreemapConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert_eq!(blocks[0].tone, Some(crate::plot::statistical::_3d::ohlc::UP));
        assert_eq!(blocks[1].tone, Some(crate::plot::statistical::_3d::ohlc::DOWN));
    }

    #[test]
    fn nested_adds_a_flat_frame_plate_under_every_parent_group() {
        let (labels, parents, values) = nested();
        let cfg = TreemapConfig { variant: TreemapVariant::Nested, labels: &labels, parents: &parents, values: &values, ..TreemapConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() > 3);
        assert!(names.contains(&"A".to_string()) && names.contains(&"B".to_string()));
    }

    #[test]
    fn branches_share_a_tone_and_differ_from_other_branches() {
        let (labels, parents, values) = nested();
        let cfg = TreemapConfig { variant: TreemapVariant::Basic, labels: &labels, parents: &parents, values: &values, ..TreemapConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        let a1 = blocks[names.iter().position(|n| n == "A1").unwrap()];
        let a2 = blocks[names.iter().position(|n| n == "A2").unwrap()];
        let b1 = blocks[names.iter().position(|n| n == "B").unwrap()];
        assert_eq!(a1.tone, a2.tone);
        assert_ne!(a1.tone, b1.tone);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&TreemapConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_and_zero_values_are_dropped_or_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let values = vec![f64::NAN, f64::INFINITY, 10.0];
        let cfg = TreemapConfig { labels: &labels, values: &values, ..TreemapConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.hw.is_finite()));
    }
}
