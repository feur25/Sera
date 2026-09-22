use super::common::prepare_with;
use super::config::GaugeConfig;
use super::variant::GaugeVariant;
use crate::plot::statistical::_3d::budget::{Buckets, Budget};
use crate::plot::statistical::_3d::curves::{points_of, toned_ribbon};
use crate::plot::statistical::_3d::dial::{
    fill, halo, hub, inner_ring, needle, segmented, ticks, track, Sweep,
};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::format_axis_label;

pub const HEIGHT_RATIO: f64 = 0.6;
pub const COLORMAP: &str = "dial";
const HISTORY_ROW: f64 = -4.6;
const HISTORY_DEPTH: f64 = 0.12;

#[derive(Clone, Copy)]
struct Recipe {
    sweep: fn() -> Sweep,
    bands: bool,
    needle: bool,
    segmented: bool,
    glow: bool,
    concentric: bool,
    ticked: bool,
}

impl Recipe {
    const fn of(sweep: fn() -> Sweep) -> Self {
        Self {
            sweep,
            bands: false,
            needle: false,
            segmented: false,
            glow: false,
            concentric: false,
            ticked: false,
        }
    }

    const fn banded(mut self) -> Self {
        self.bands = true;
        self
    }

    const fn needled(mut self) -> Self {
        self.needle = true;
        self
    }

    const fn chunked(mut self) -> Self {
        self.segmented = true;
        self
    }

    const fn glowing(mut self) -> Self {
        self.glow = true;
        self
    }

    const fn dual(mut self) -> Self {
        self.concentric = true;
        self
    }

    const fn ticked(mut self) -> Self {
        self.ticked = true;
        self
    }
}

fn recipe(variant: GaugeVariant) -> Recipe {
    use GaugeVariant::*;
    match variant {
        Basic => Recipe::of(Sweep::half).banded().needled(),
        Radial => Recipe::of(Sweep::full),
        Arc270 => Recipe::of(Sweep::arc270).banded(),
        Sleek => Recipe::of(Sweep::half),
        Tick => Recipe::of(Sweep::half).ticked(),
        Segmented => Recipe::of(Sweep::half).chunked(),
        Glow => Recipe::of(Sweep::half).glowing(),
        Concentric => Recipe::of(Sweep::half).dual(),
        Sparkline => Recipe::of(Sweep::half),
    }
}

fn history_ribbon(history: &[f64], cap: usize) -> Vec<Bar3DBlock> {
    if history.len() < 2 {
        return Vec::new();
    }
    let buckets = Buckets::new(history.len(), cap);
    let kept = buckets.mean(history);
    let lo = kept.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = kept.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (hi - lo).max(1e-9);
    let scaled: Vec<f64> = kept.iter().map(|v| (v - lo) / span * 2.0).collect();
    let n = scaled.len();
    let tones: Vec<f64> = (0..n).map(|i| i as f64 / (n - 1).max(1) as f64).collect();
    toned_ribbon(&points_of(&scaled), &tones, HISTORY_ROW, HISTORY_DEPTH, 1)
}

pub fn layout_3d(cfg: &GaugeConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

fn safe(value: f64) -> f64 {
    if value.is_finite() { value } else { 0.0 }
}

pub fn layout_named(cfg: &GaugeConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let plan = recipe(cfg.variant);
    let sweep = (plan.sweep)();
    let p = prepare_with(cfg, 0.5, 0.5);
    let frac = safe(p.frac);
    let frac_cmp = safe(p.frac_cmp);
    let mut blocks = track(sweep, &p.thresholds, plan.bands, 0);
    if plan.segmented {
        blocks = segmented(sweep, frac, &p.thresholds, 0);
    } else {
        blocks.extend(fill(sweep, frac, &p.thresholds, crate::plot::statistical::_3d::dial::FILL_H, 0));
    }
    if plan.glow {
        blocks.extend(halo(sweep, frac, &p.thresholds, 0));
    }
    if plan.needle {
        blocks.extend(needle(sweep, frac, 0));
        blocks.push(hub(0));
    }
    if plan.ticked {
        blocks.extend(ticks(sweep, 0));
    }
    if plan.concentric {
        let cmp_frac = if cfg.comparison > 0.0 { frac_cmp } else { frac * 0.7 };
        blocks.extend(inner_ring(sweep, cmp_frac, 1));
    }
    let matches_variant = |v: GaugeVariant| cfg.variant == v;
    if matches_variant(GaugeVariant::Sparkline) {
        blocks.extend(history_ribbon(cfg.history, budget.points));
    }
    let value_text = format!("{} ({}-{})", format_axis_label(cfg.value), format_axis_label(cfg.min_val), format_axis_label(cfg.max_val));
    let label = if cfg.label.is_empty() { value_text.clone() } else { format!("{} · {value_text}", cfg.label) };
    let names = if plan.concentric {
        vec![label, format!("comparison {}", format_axis_label(cfg.comparison))]
    } else {
        vec![label]
    };
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(variant: GaugeVariant, value: f64) -> (Vec<Bar3DBlock>, Vec<String>) {
        let cfg = GaugeConfig { variant, value, min_val: 0.0, max_val: 100.0, label: "Score", ..GaugeConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_ring_and_names_the_gauge() {
        for &variant in GaugeVariant::all() {
            let (blocks, names) = draw(variant, 72.0);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names[0], "Score · 72 (0-100)", "{variant:?}");
        }
    }

    #[test]
    fn only_basic_carries_a_needle_and_a_hub() {
        let basic = draw(GaugeVariant::Basic, 50.0).0;
        assert!(basic.iter().any(|b| b.tone == Some(crate::plot::statistical::_3d::dial::tone::NEEDLE)));
        let sleek = draw(GaugeVariant::Sleek, 50.0).0;
        assert!(sleek.iter().all(|b| b.tone != Some(crate::plot::statistical::_3d::dial::tone::NEEDLE)));
    }

    #[test]
    fn radial_sweeps_the_full_circle_while_the_others_stay_in_the_upper_half() {
        let radial = draw(GaugeVariant::Radial, 50.0).0;
        assert!(radial.iter().any(|b| b.cy < -1e-9));
        let basic = draw(GaugeVariant::Basic, 50.0).0;
        assert!(basic.iter().all(|b| b.cy >= -1e-9));
    }

    #[test]
    fn segmented_uses_the_chunky_ring_and_others_use_the_continuous_one() {
        let segmented = draw(GaugeVariant::Segmented, 50.0).0;
        let continuous = draw(GaugeVariant::Sleek, 50.0).0;
        assert!(segmented.len() < continuous.len());
    }

    #[test]
    fn concentric_adds_a_second_ring_and_names_the_comparison() {
        let cfg = GaugeConfig { variant: GaugeVariant::Concentric, value: 80.0, min_val: 0.0, max_val: 100.0, comparison: 60.0, ..GaugeConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().any(|b| b.tone == Some(crate::plot::statistical::_3d::dial::tone::COMPARE)));
        assert_eq!(names[1], "comparison 60");
    }

    #[test]
    fn sparkline_adds_a_ribbon_when_history_is_given_and_skips_it_otherwise() {
        let with_history = GaugeConfig { variant: GaugeVariant::Sparkline, value: 72.0, history: &[55.0, 60.0, 58.0, 66.0, 72.0], ..GaugeConfig::default() };
        let without = GaugeConfig { variant: GaugeVariant::Sparkline, value: 72.0, ..GaugeConfig::default() };
        let (with_blocks, _) = layout_named(&with_history, &Budget::default());
        let (without_blocks, _) = layout_named(&without, &Budget::default());
        assert!(with_blocks.len() > without_blocks.len());
    }

    #[test]
    fn thresholds_band_the_ring_and_pick_the_fill_tone_by_the_current_value() {
        let cfg = GaugeConfig {
            variant: GaugeVariant::Basic,
            value: 85.0,
            min_val: 0.0,
            max_val: 100.0,
            thresholds: &[(0.0, 0), (50.0, 0), (90.0, 0)],
            ..GaugeConfig::default()
        };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().any(|b| b.tone == Some(crate::plot::statistical::_3d::dial::tone::WARN)));
    }

    #[test]
    fn a_long_history_is_pooled_to_the_budget() {
        let history: Vec<f64> = (0..1000).map(|i| (i % 37) as f64).collect();
        let cfg = GaugeConfig { variant: GaugeVariant::Sparkline, value: 50.0, history: &history, ..GaugeConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::new(Some(20)));
        let ribbon = blocks.iter().filter(|b| b.ci == 1).count();
        assert!(ribbon > 0 && ribbon < 40);
    }

    #[test]
    fn extreme_values_stay_finite() {
        let cfg = GaugeConfig { variant: GaugeVariant::Basic, value: f64::NAN, min_val: 0.0, max_val: 0.0, ..GaugeConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.cx.is_finite()));
    }
}
