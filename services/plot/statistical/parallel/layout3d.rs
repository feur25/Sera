use super::common::prepare;
use super::config::ParallelConfig;
use super::variant::ParallelVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::curves::{catmull_rom, points_of, ribbon_blocks, Point};
use crate::plot::statistical::bar::Bar3DBlock;

const ROW_STEP: f64 = 1.2;
const AXIS_SPAN: f64 = 8.0;

#[derive(Clone, Copy, PartialEq)]
enum Tone {
    BySeries,
    ByCategory,
    Highlighted,
    Faded,
}

#[derive(Clone, Copy)]
struct Recipe {
    smooth: bool,
    tone: Tone,
    depth: f64,
}

impl Recipe {
    const fn of(tone: Tone) -> Self {
        Self { smooth: false, tone, depth: 0.16 }
    }

    const fn smoothed(mut self) -> Self {
        self.smooth = true;
        self
    }

    const fn deep(mut self, d: f64) -> Self {
        self.depth = d;
        self
    }
}

fn recipe(variant: ParallelVariant) -> Recipe {
    use ParallelVariant::*;
    match variant {
        Basic => Recipe::of(Tone::BySeries),
        Smooth | Arc => Recipe::of(Tone::BySeries).smoothed(),
        Categorical | Lineage | Chronicle => Recipe::of(Tone::ByCategory),
        Highlight => Recipe::of(Tone::Highlighted),
        Density => Recipe::of(Tone::Faded),
        Ribbon => Recipe::of(Tone::BySeries).deep(0.32),
    }
}

fn parallel_3d(cfg: &ParallelConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(p) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let plan = recipe(cfg.variant);
    let axis_scale = if p.n_axes > 1 { AXIS_SPAN / (p.n_axes as f64 - 1.0) } else { 1.0 };
    let mut blocks = Vec::new();
    for si in 0..p.n_series {
        let vals: Vec<f64> = (0..p.n_axes)
            .map(|ai| {
                let raw = cfg.series_values[si].get(ai).copied().unwrap_or(p.mins[ai]);
                (raw - p.mins[ai]) / (p.maxs[ai] - p.mins[ai])
            })
            .collect();
        let mut points: Vec<Point> = points_of(&vals).into_iter().map(|(x, y)| (x * axis_scale, y)).collect();
        if plan.smooth {
            points = catmull_rom(&points, 4, 0.5);
        }
        let joined = vec![true; points.len().saturating_sub(1)];
        let tone = match plan.tone {
            Tone::BySeries => si as f64 / p.n_series.max(1) as f64,
            Tone::ByCategory => {
                let cat = cfg.categories.get(si).copied().unwrap_or(0).max(0) as f64;
                (cat / 8.0).min(1.0)
            }
            Tone::Highlighted => {
                if si as i32 == cfg.highlight_index {
                    1.0
                } else {
                    0.12
                }
            }
            Tone::Faded => 0.3,
        };
        blocks.extend(ribbon_blocks(&points, &joined, si as f64 * ROW_STEP, plan.depth, si, Some(tone)));
    }
    (blocks, cfg.series_names.to_vec())
}

pub fn layout_3d(cfg: &ParallelConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &ParallelConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    parallel_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> (Vec<String>, Vec<String>, Vec<Vec<f64>>) {
        (
            ["Speed", "Power", "Range"].iter().map(|s| s.to_string()).collect(),
            ["Car A", "Car B", "Car C"].iter().map(|s| s.to_string()).collect(),
            vec![vec![120.0, 300.0, 400.0], vec![180.0, 420.0, 250.0], vec![90.0, 180.0, 600.0]],
        )
    }

    fn draw(variant: ParallelVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (axes, names, series_values) = data();
        let cfg = ParallelConfig { variant, axes: &axes, series_names: &names, series_values: &series_values, ..ParallelConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_ribbon_per_series_and_names_every_series() {
        for &variant in ParallelVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 3, "{variant:?}");
            assert!(!blocks.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn each_series_occupies_its_own_row_at_a_distinct_y() {
        let (blocks, _) = draw(ParallelVariant::Basic);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0) as i64).collect();
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn smooth_densifies_more_segments_than_basic() {
        let (basic, _) = draw(ParallelVariant::Basic);
        let (smooth, _) = draw(ParallelVariant::Smooth);
        assert!(smooth.len() > basic.len());
    }

    #[test]
    fn highlight_gives_the_highlighted_series_a_much_higher_tone() {
        let (axes, names, series_values) = data();
        let cfg = ParallelConfig {
            variant: ParallelVariant::Highlight,
            axes: &axes,
            series_names: &names,
            series_values: &series_values,
            highlight_index: 1,
            ..ParallelConfig::default()
        };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let hi = blocks.iter().find(|b| b.ci == 1).unwrap();
        let lo = blocks.iter().find(|b| b.ci == 0).unwrap();
        assert!(hi.tone.unwrap() > lo.tone.unwrap());
    }

    #[test]
    fn ribbon_variant_is_thicker_than_basic() {
        let (basic, _) = draw(ParallelVariant::Basic);
        let (ribbon, _) = draw(ParallelVariant::Ribbon);
        assert!(ribbon[0].hd > basic[0].hd);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&ParallelConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
