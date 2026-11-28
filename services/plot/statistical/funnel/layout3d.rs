use super::common::prepare;
use super::config::FunnelConfig;
use super::variant::FunnelVariant;
use crate::plot::statistical::_3d::budget::{Buckets, Budget};
use crate::plot::statistical::_3d::tiers::{split, stack, Profile, Tier, HALF};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::format_axis_label;

const LANE: f64 = 2.6 * HALF;
const TONE_LOW: f64 = 0.1;
const TONE_HIGH: f64 = 0.9;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Single,
    Rate,
    Compare,
    Grouped,
}

#[derive(Clone, Copy)]
struct Recipe {
    profile: Profile,
    reverse: bool,
    pyramid: bool,
    mode: Mode,
    cmap: &'static str,
}

impl Recipe {
    const fn of(profile: Profile) -> Self {
        Self { profile, reverse: false, pyramid: false, mode: Mode::Single, cmap: "cyber" }
    }

    const fn reversed(mut self) -> Self {
        self.reverse = true;
        self
    }

    const fn pyramidal(mut self) -> Self {
        self.pyramid = true;
        self
    }

    const fn in_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    const fn colored(mut self, cmap: &'static str) -> Self {
        self.cmap = cmap;
        self
    }
}

fn recipe(variant: FunnelVariant) -> Recipe {
    use FunnelVariant::*;
    match variant {
        Basic => Recipe::of(Profile::Taper),
        Stepped => Recipe::of(Profile::Flat),
        Rounded => Recipe::of(Profile::Barrel),
        Chevron => Recipe::of(Profile::Chevron),
        Pyramid => Recipe::of(Profile::Taper).pyramidal(),
        Inverted => Recipe::of(Profile::Taper).reversed(),
        Conversion => Recipe::of(Profile::Flat).in_mode(Mode::Rate).colored("updown"),
        Compare => Recipe::of(Profile::Flat).in_mode(Mode::Compare),
        Grouped => Recipe::of(Profile::Flat).in_mode(Mode::Grouped),
    }
}

fn effective(cfg: &FunnelConfig) -> FunnelVariant {
    if cfg.variant == FunnelVariant::Basic && cfg.series.len() > 1 {
        if cfg.stage_labels.is_empty() { FunnelVariant::Grouped } else { FunnelVariant::Compare }
    } else {
        cfg.variant
    }
}

pub fn colormap(cfg: &FunnelConfig) -> &'static str {
    recipe(effective(cfg)).cmap
}

fn clean(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { v.max(0.0) } else { 0.0 }).collect()
}

fn peak(values: &[f64]) -> f64 {
    values.iter().copied().fold(0.0_f64, f64::max).max(1e-12)
}

fn area_share(value: f64, top: f64) -> f64 {
    (value / top).sqrt()
}

fn part_share(part: f64, total: f64, top: f64) -> f64 {
    if total > 0.0 { area_share(total, top) * part / total } else { 0.0 }
}

fn percent(value: f64, base: f64) -> f64 {
    if base > 0.0 { (value / base * 100.0).round() } else { 0.0 }
}

fn series_tone(index: usize, count: usize) -> f64 {
    if count > 1 { TONE_LOW + (TONE_HIGH - TONE_LOW) * index as f64 / (count - 1) as f64 } else { 0.5 }
}

fn pyramidal(shares: &[f64]) -> Vec<f64> {
    let n = shares.len().max(1) as f64;
    let base: Vec<f64> = shares.iter().enumerate().map(|(k, s)| s * (n - k as f64) / n).collect();
    (0..base.len()).map(|k| base[k].max(base.get(k + 1).copied().unwrap_or(0.0))).collect()
}

fn pooled(labels: &[String], values: &[f64], cap: usize) -> (Vec<String>, Vec<f64>) {
    let n = labels.len().min(values.len());
    let buckets = Buckets::new(n, cap);
    (buckets.first(&labels[..n]), buckets.mean(&values[..n]))
}

fn single(plan: Recipe, cfg: &FunnelConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let (labels, values) = pooled(cfg.labels, cfg.values, budget.points);
    let sorted = FunnelConfig { labels: &labels, values: &values, sort_order: cfg.sort_order, ..FunnelConfig::default() };
    let Some(prepared) = prepare(&sorted) else {
        return (Vec::new(), Vec::new());
    };
    let values = clean(&prepared.values);
    let top = peak(&values);
    let mut shares: Vec<f64> = values.iter().map(|v| area_share(*v, top)).collect();
    if plan.pyramid {
        shares = pyramidal(&shares);
    }
    let rates: Vec<f64> = (0..values.len())
        .map(|k| if k == 0 { 1.0 } else { (values[k] / values[k - 1].max(1e-12)).clamp(0.0, 1.0) })
        .collect();
    let tiers: Vec<Tier> = shares
        .iter()
        .enumerate()
        .map(|(k, &width)| Tier { width, class: k, tone: (plan.mode == Mode::Rate).then(|| rates[k]) })
        .collect();
    let names = prepared
        .labels
        .iter()
        .zip(&values)
        .enumerate()
        .map(|(k, (label, &value))| {
            let base = format!("{label} · {} ({}% of first)", format_axis_label(value), percent(value, values[0]));
            if plan.mode == Mode::Rate { format!("{base} · {}% of previous", (rates[k] * 100.0).round()) } else { base }
        })
        .collect();
    (stack(&tiers, plan.profile, plan.reverse, 0.0), names)
}

fn compare(plan: Recipe, cfg: &FunnelConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let count = cfg.series.len();
    let cleaned: Vec<(String, Vec<f64>)> = cfg.series.iter().map(|(name, values)| (name.clone(), clean(values))).collect();
    let top = peak(&cleaned.iter().flat_map(|(_, v)| v.iter().copied()).collect::<Vec<_>>());
    let (mut blocks, mut names) = (Vec::new(), Vec::new());
    for (s, (name, values)) in cleaned.iter().enumerate() {
        let stages: Vec<String> = (0..values.len())
            .map(|k| cfg.stage_labels.get(s).and_then(|l| l.get(k)).cloned().unwrap_or_else(|| format!("Stage {}", k + 1)))
            .collect();
        let (stages, values) = pooled(&stages, values, budget.points);
        let offset = names.len();
        let tiers: Vec<Tier> = values
            .iter()
            .enumerate()
            .map(|(k, v)| Tier { width: area_share(*v, top), class: offset + k, tone: Some(series_tone(s, count)) })
            .collect();
        blocks.extend(stack(&tiers, plan.profile, plan.reverse, s as f64 * LANE));
        names.extend(stages.iter().zip(&values).map(|(stage, v)| format!("{name} · {stage} · {}", format_axis_label(*v))));
    }
    (blocks, names)
}

fn grouped(plan: Recipe, cfg: &FunnelConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let count = cfg.series.len();
    let stages_n = cfg.labels.len();
    if stages_n == 0 || count == 0 {
        return (Vec::new(), Vec::new());
    }
    let buckets = Buckets::new(stages_n, budget.points);
    let labels = buckets.first(cfg.labels);
    let columns: Vec<Vec<f64>> = cfg.series.iter().map(|(_, values)| buckets.mean(&clean(values))).collect();
    let totals: Vec<f64> = (0..labels.len()).map(|k| columns.iter().map(|c| c.get(k).copied().unwrap_or(0.0)).sum()).collect();
    let top = peak(&totals);
    let stages: Vec<Vec<Tier>> = (0..labels.len())
        .map(|k| {
            (0..count)
                .map(|s| Tier { width: part_share(columns[s].get(k).copied().unwrap_or(0.0), totals[k], top), class: k * count + s, tone: Some(series_tone(s, count)) })
                .collect()
        })
        .collect();
    let names = labels
        .iter()
        .enumerate()
        .flat_map(|(k, label)| {
            cfg.series.iter().enumerate().map(move |(s, (name, _))| (k, s, label, name))
        })
        .map(|(k, s, label, name)| format!("{label} · {name} · {}", format_axis_label(columns[s].get(k).copied().unwrap_or(0.0))))
        .collect();
    (split(&stages, plan.reverse), names)
}

pub fn layout_3d(cfg: &FunnelConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &FunnelConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let plan = recipe(effective(cfg));
    match plan.mode {
        Mode::Single | Mode::Rate => single(plan, cfg, budget),
        Mode::Compare => compare(plan, cfg, budget),
        Mode::Grouped => grouped(plan, cfg, budget),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels() -> Vec<String> {
        ["Visits", "Signups", "Trial", "Paid", "Renewed"].iter().map(|s| s.to_string()).collect()
    }

    fn draw(variant: FunnelVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let names = labels();
        let cfg = FunnelConfig { variant, labels: &names, values: &[1000.0, 520.0, 210.0, 85.0, 40.0], ..FunnelConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    fn series() -> Vec<(String, Vec<f64>)> {
        vec![("Montreal".to_string(), vec![39.0, 27.0, 20.0, 11.0, 3.0]), ("Toronto".to_string(), vec![52.0, 36.0, 18.0, 14.0, 5.0])]
    }

    #[test]
    fn every_single_series_variant_stacks_five_stages_in_its_own_profile() {
        for &variant in FunnelVariant::all() {
            if matches!(variant, FunnelVariant::Compare | FunnelVariant::Grouped) {
                continue;
            }
            let (blocks, names) = draw(variant);
            let per_stage = match variant {
                FunnelVariant::Basic | FunnelVariant::Pyramid | FunnelVariant::Inverted | FunnelVariant::Rounded => 4,
                _ => 1,
            };
            assert_eq!(blocks.len(), 5 * per_stage, "{variant:?}");
            assert_eq!(names.len(), 5, "{variant:?}");
        }
    }

    #[test]
    fn hover_names_carry_the_value_and_the_share_of_the_first_stage() {
        let (_, names) = draw(FunnelVariant::Stepped);
        assert_eq!(names[1], "Signups · 520 (52% of first)");
    }

    #[test]
    fn conversion_tones_each_stage_by_its_rate_from_the_previous_one() {
        let (blocks, names) = draw(FunnelVariant::Conversion);
        assert_eq!(blocks[0].tone, Some(1.0));
        assert!((blocks[1].tone.unwrap() - 0.52).abs() < 1e-9);
        assert!(names[1].ends_with("52% of previous"));
        assert_eq!(colormap(&FunnelConfig { variant: FunnelVariant::Conversion, ..FunnelConfig::default() }), "updown");
    }

    #[test]
    fn inverted_funnels_stand_on_their_widest_stage() {
        let (normal, _) = draw(FunnelVariant::Stepped);
        let (upside, _) = draw(FunnelVariant::Inverted);
        assert!(normal[0].cx < normal[4].cx);
        assert!(upside[0].cx > upside[4].cx);
    }

    #[test]
    fn the_pyramid_narrows_faster_than_the_plain_funnel() {
        let (plain, _) = draw(FunnelVariant::Basic);
        let (pyramid, _) = draw(FunnelVariant::Pyramid);
        assert!(pyramid[16].hd < plain[16].hd);
    }

    #[test]
    fn several_series_without_stage_labels_become_a_grouped_funnel() {
        let names = labels();
        let data = series();
        let cfg = FunnelConfig { labels: &names, series: &data, ..FunnelConfig::default() };
        let (blocks, out) = layout_named(&cfg, &Budget::default());
        assert_eq!((blocks.len(), out.len()), (10, 10));
        assert_eq!(out[0], "Visits · Montreal · 39");
        assert_ne!(blocks[0].tone, blocks[1].tone);
        assert_eq!(colormap(&cfg), "cyber");
    }

    #[test]
    fn several_series_with_their_own_stage_labels_become_side_by_side_funnels() {
        let data = vec![("A".to_string(), vec![10.0, 5.0]), ("B".to_string(), vec![8.0, 4.0, 2.0])];
        let stages = vec![vec!["x".to_string(), "y".to_string()], vec!["p".to_string(), "q".to_string(), "r".to_string()]];
        let cfg = FunnelConfig { series: &data, stage_labels: &stages, ..FunnelConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!((blocks.len(), names.len()), (5, 5));
        assert_eq!(names[2], "B · p · 8");
        assert!(blocks[2].cy > blocks[1].cy);
        assert_eq!(blocks[2].ci, 2);
    }

    #[test]
    fn long_inputs_are_pooled_to_the_budget_with_matching_names() {
        let names: Vec<String> = (0..1000).map(|i| format!("S{i}")).collect();
        let values: Vec<f64> = (0..1000).map(|i| 1000.0 - i as f64).collect();
        let cfg = FunnelConfig { variant: FunnelVariant::Stepped, labels: &names, values: &values, ..FunnelConfig::default() };
        let (blocks, out) = layout_named(&cfg, &Budget::new(Some(100)));
        assert_eq!((blocks.len(), out.len()), (100, 100));
        assert!(out[1].starts_with("S10 "));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&FunnelConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_and_negative_values_are_flattened_instead_of_poisoning_the_scene() {
        let names = labels();
        let cfg = FunnelConfig { labels: &names, values: &[f64::NAN, f64::INFINITY, -5.0, 10.0, 3.0], ..FunnelConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.hw.is_finite() && b.hd.is_finite()));
    }
}
