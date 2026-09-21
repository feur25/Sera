use super::common::prepare;
use super::config::WaterfallConfig;
use super::variant::WaterfallVariant;
use crate::plot::statistical::_3d::budget::{Budget, Buckets};
use crate::plot::statistical::_3d::generic::transposed;
use crate::plot::statistical::_3d::steps::{
    arrow_tips, floating_bars, running_track, stem_heads, Step, TOTAL_TONE,
};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.8;
pub const COLORMAP: &str = "updown";
const BAR_HW: f64 = 0.32;
const STEPPED_HW: f64 = 0.5;
const STEM_HW: f64 = 0.05;
const DEPTH: f64 = 0.35;
const TRACK_ROW: f64 = 1.2;
const TRACK_TONE: f64 = 0.5;

#[derive(Clone, Copy)]
enum Glyph {
    Bars,
    Contiguous,
    Lollipop,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    arrows: bool,
    graded: bool,
    trend: bool,
    swap: bool,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, arrows: false, graded: false, trend: false, swap: false }
    }

    const fn arrowed(mut self) -> Self {
        self.arrows = true;
        self
    }

    const fn graded(mut self) -> Self {
        self.graded = true;
        self
    }

    const fn trended(mut self) -> Self {
        self.trend = true;
        self
    }

    const fn swapped(mut self) -> Self {
        self.swap = true;
        self
    }
}

fn recipe(variant: WaterfallVariant) -> Recipe {
    use WaterfallVariant::*;
    let bars = Recipe::of(Glyph::Bars);
    match variant {
        Basic => bars,
        Stepped => Recipe::of(Glyph::Contiguous),
        Lollipop => Recipe::of(Glyph::Lollipop),
        Arrowed => bars.arrowed(),
        Delta => bars.graded(),
        Horizontal => bars.swapped(),
        Trend => bars.trended(),
    }
}

fn graded_tones(blocks: &mut [Bar3DBlock], steps: &[Step]) {
    let peak = steps
        .iter()
        .filter(|s| !s.total)
        .map(|s| (s.end - s.start).abs())
        .fold(1e-12, f64::max);
    for (block, step) in blocks.iter_mut().zip(steps) {
        block.tone = Some(if step.total {
            TOTAL_TONE
        } else {
            (0.5 + 0.5 * (step.end - step.start) / peak).clamp(0.0, 1.0)
        });
    }
}

pub fn layout_3d(cfg: &WaterfallConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    let n = cfg.labels.len().min(cfg.values.len());
    let buckets = Buckets::new(n, budget.points);
    if buckets.is_identity() {
        return stepped_3d(cfg);
    }
    let labels = buckets.first(&cfg.labels[..n]);
    let values = buckets.sum(&cfg.values[..n]);
    stepped_3d(&WaterfallConfig {
        variant: cfg.variant,
        labels: &labels,
        values: &values,
        sort_order: cfg.sort_order,
        ..WaterfallConfig::default()
    })
}

fn stepped_3d(cfg: &WaterfallConfig) -> Vec<Bar3DBlock> {
    let Some(prepared) = prepare(cfg) else {
        return Vec::new();
    };
    let steps: Vec<Step> = (0..prepared.n)
        .map(|i| Step { start: prepared.starts[i], end: prepared.ends[i], total: prepared.is_total[i] })
        .collect();
    let plan = recipe(cfg.variant);
    let mut blocks = match plan.glyph {
        Glyph::Bars => floating_bars(&steps, BAR_HW, DEPTH),
        Glyph::Contiguous => floating_bars(&steps, STEPPED_HW, DEPTH),
        Glyph::Lollipop => stem_heads(&steps, STEM_HW, BAR_HW * 0.8, DEPTH),
    };
    if plan.graded {
        graded_tones(&mut blocks, &steps);
    }
    if plan.arrows {
        blocks.extend(arrow_tips(&steps, BAR_HW * 0.9));
    }
    if plan.trend {
        blocks.extend(running_track(&steps, TRACK_ROW, 0.1, TRACK_TONE));
    }
    if plan.swap {
        blocks = transposed(blocks);
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocks_for(variant: WaterfallVariant) -> Vec<Bar3DBlock> {
        let labels: Vec<String> = ["Start", "Q1", "Q2", "Q3", "End"].iter().map(|s| s.to_string()).collect();
        let values = [100.0, 30.0, -15.0, 40.0, 155.0];
        layout_3d(
            &WaterfallConfig {
                variant,
                labels: &labels,
                values: &values,
                ..WaterfallConfig::default()
            },
            &Budget::default(),
        )
    }

    #[test]
    fn every_waterfall_variant_draws_something() {
        for variant in WaterfallVariant::all() {
            assert!(!blocks_for(*variant).is_empty(), "{} must draw blocks", variant.name());
        }
    }

    #[test]
    fn variants_add_their_own_glyphs_on_top_of_the_steps() {
        let base = blocks_for(WaterfallVariant::Basic).len();
        assert_eq!(base, 5);
        assert_eq!(blocks_for(WaterfallVariant::Lollipop).len(), 10);
        assert_eq!(blocks_for(WaterfallVariant::Arrowed).len(), 10);
        assert_eq!(blocks_for(WaterfallVariant::Trend).len(), 10);
    }

    #[test]
    fn delta_grades_the_tones_by_the_size_of_the_change() {
        let graded = blocks_for(WaterfallVariant::Delta);
        let plain = blocks_for(WaterfallVariant::Basic);
        assert_ne!(graded[1].tone, plain[1].tone);
        assert!(graded[3].tone.unwrap() > graded[1].tone.unwrap());
    }

    #[test]
    fn horizontal_turns_the_basic_layout_a_quarter_turn() {
        let basic = blocks_for(WaterfallVariant::Basic);
        let horizontal = blocks_for(WaterfallVariant::Horizontal);
        assert_eq!(basic[2].cx, horizontal[2].cy);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_3d(&WaterfallConfig::default(), &Budget::default()).is_empty());
    }

    #[test]
    fn long_ledgers_are_pooled_by_sum_so_the_final_total_is_preserved() {
        let n = 100_000;
        let labels: Vec<String> = (0..n).map(|i| format!("S{i}")).collect();
        let values: Vec<f64> = (0..n).map(|i| if i % 3 == 0 { -1.0 } else { 2.0 }).collect();
        let cfg = WaterfallConfig { labels: &labels, values: &values, ..WaterfallConfig::default() };
        let blocks = layout_3d(&cfg, &Budget::new(Some(400)));
        assert_eq!(blocks.len(), 400);
        let expected: f64 = values.iter().sum();
        let top = blocks.iter().map(|b| b.z1).fold(f64::MIN, f64::max);
        assert!((top - expected).abs() < expected.abs() * 0.01 + 5.0);
    }
}
