use super::common::{max_for, prepare, Prepared};
use super::config::BulletConfig;
use super::variant::BulletVariant;
use crate::plot::statistical::_3d::budget::{Buckets, Budget};
use crate::plot::statistical::_3d::bullets::{bullets, Bands, Bar, Gauge, Plan};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::format_axis_label;

pub const HEIGHT_RATIO: f64 = 0.8;
pub const COLORMAP: &str = "bullet";
const ROW_CAP: usize = 600;

fn plan(variant: BulletVariant) -> Plan {
    use BulletVariant::*;
    let column = Plan { bands: Bands::Range, bar: Bar::Column, target: true, ghost: false };
    match variant {
        Basic => column,
        Stacked => Plan { bands: Bands::Zones, ..column },
        Thermo => Plan { bands: Bands::Rail, bar: Bar::Tube, ..column },
        Segmented => Plan { bands: Bands::Signal, ..column },
        Minimal => Plan { bands: Bands::None, ..column },
        Dot => Plan { bands: Bands::Track, bar: Bar::Dot, ..column },
        Progress => Plan { bands: Bands::Track, bar: Bar::Wide, target: false, ..column },
        Compare => Plan { bands: Bands::Track, ghost: true, ..column },
    }
}

fn sound(value: f64) -> f64 {
    if value.is_finite() { value } else { 0.0 }
}

fn gauges(prepared: &Prepared) -> Vec<Gauge> {
    (0..prepared.n)
        .map(|i| {
            let prior = if prepared.comparisons[i] > 0.0 { prepared.comparisons[i] } else { prepared.ranges[i] };
            Gauge {
                value: sound(prepared.values[i]),
                target: sound(prepared.targets[i]),
                prior: sound(prior),
                max: sound(max_for(prepared, i)).max(1e-9),
                range: sound(prepared.ranges[i]),
            }
        })
        .collect()
}

fn names_of(labels: &[String], gauges: &[Gauge]) -> Vec<String> {
    labels
        .iter()
        .zip(gauges)
        .flat_map(|(label, gauge)| {
            [
                format!("{label} · range"),
                format!("{label} · {} of {}", format_axis_label(gauge.value), format_axis_label(gauge.max)),
                format!("{label} · target {}", format_axis_label(gauge.target)),
                format!("{label} · prior {}", format_axis_label(gauge.prior)),
            ]
        })
        .collect()
}

pub fn layout_3d(cfg: &BulletConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &BulletConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values.len());
    let buckets = Buckets::new(n, budget.points.min(ROW_CAP));
    if buckets.is_identity() {
        return bullets_3d(cfg);
    }
    let labels = buckets.first(&cfg.labels[..n]);
    let values = buckets.mean(&cfg.values[..n]);
    let targets = buckets.mean(cfg.targets);
    let max_vals = buckets.mean(cfg.max_vals);
    let ranges = buckets.mean(cfg.ranges);
    let comparisons = buckets.mean(cfg.comparisons);
    bullets_3d(&BulletConfig {
        variant: cfg.variant,
        labels: &labels,
        values: &values,
        targets: &targets,
        max_vals: &max_vals,
        ranges: &ranges,
        comparisons: &comparisons,
        sort_order: cfg.sort_order,
        ..BulletConfig::default()
    })
}

fn bullets_3d(cfg: &BulletConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(prepared) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let rows = gauges(&prepared);
    (bullets(&rows, plan(cfg.variant), HEIGHT_RATIO), names_of(&prepared.labels[..prepared.n], &rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels() -> Vec<String> {
        ["Revenue", "Profit", "CSAT"].iter().map(|s| s.to_string()).collect()
    }

    fn draw(variant: BulletVariant, comparisons: &[f64]) -> (Vec<Bar3DBlock>, Vec<String>) {
        let names = labels();
        let cfg = BulletConfig {
            variant,
            labels: &names,
            values: &[80.0, 65.0, 4.2],
            targets: &[90.0, 70.0, 4.5],
            max_vals: &[120.0, 100.0, 5.0],
            comparisons,
            ..BulletConfig::default()
        };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_layers_its_bands_bar_and_marks_per_row() {
        for &variant in BulletVariant::all() {
            let (blocks, names) = draw(variant, &[70.0, 55.0, 3.8]);
            let per_row = match variant {
                BulletVariant::Basic | BulletVariant::Thermo | BulletVariant::Compare => 4,
                BulletVariant::Stacked | BulletVariant::Segmented => 5,
                BulletVariant::Dot => 3,
                BulletVariant::Minimal | BulletVariant::Progress => 2,
            };
            assert_eq!(blocks.len(), 3 * per_row, "{variant:?}");
            assert_eq!(names.len(), 12, "{variant:?}");
        }
    }

    #[test]
    fn hover_names_carry_the_raw_numbers_of_each_part() {
        let (_, names) = draw(BulletVariant::Basic, &[]);
        assert_eq!(&names[..4], ["Revenue · range", "Revenue · 80 of 120", "Revenue · target 90", "Revenue · prior 0"]);
    }

    #[test]
    fn rows_with_different_scales_share_the_percentage_axis() {
        let (blocks, _) = draw(BulletVariant::Minimal, &[]);
        let bars: Vec<&Bar3DBlock> = blocks.iter().filter(|b| b.ci % 4 == 1).collect();
        assert_eq!(bars.len(), 3);
        assert!((bars[0].z1 - 66.666).abs() < 0.01 && (bars[2].z1 - 84.0).abs() < 1e-9);
    }

    #[test]
    fn the_ghost_falls_back_to_the_range_when_no_comparison_is_given() {
        let names = labels();
        let cfg = BulletConfig {
            variant: BulletVariant::Compare,
            labels: &names,
            values: &[50.0, 50.0, 50.0],
            max_vals: &[100.0, 100.0, 100.0],
            ranges: &[60.0, 0.0, 0.0],
            ..BulletConfig::default()
        };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let priors: Vec<f64> = blocks.iter().filter(|b| b.ci % 4 == 3).map(|b| b.z1).collect();
        assert_eq!(priors.len(), 3);
        assert_eq!(priors[0], 60.0);
    }

    #[test]
    fn long_inputs_are_pooled_to_the_budget_with_matching_names() {
        let names: Vec<String> = (0..1000).map(|i| format!("R{i}")).collect();
        let values: Vec<f64> = (0..1000).map(|i| (i % 90) as f64).collect();
        let targets = vec![80.0; 1000];
        let cfg = BulletConfig { labels: &names, values: &values, targets: &targets, ..BulletConfig::default() };
        let (blocks, out) = layout_named(&cfg, &Budget::new(Some(100)));
        assert_eq!((blocks.len(), out.len()), (100 * 4, 100 * 4));
        assert!(out[4].starts_with("R10 "));
    }

    #[test]
    fn wide_walls_cap_the_rows_lower_than_the_shared_budget() {
        let names: Vec<String> = (0..5000).map(|i| format!("R{i}")).collect();
        let values = vec![50.0; 5000];
        let cfg = BulletConfig { variant: BulletVariant::Segmented, labels: &names, values: &values, ..BulletConfig::default() };
        let (blocks, out) = layout_named(&cfg, &Budget::new(Some(4000)));
        assert_eq!(out.len(), ROW_CAP * 4);
        assert_eq!(blocks.len(), ROW_CAP * 4);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&BulletConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let names = labels();
        let cfg = BulletConfig {
            labels: &names,
            values: &[f64::NAN, f64::INFINITY, 3.0],
            targets: &[f64::INFINITY, 1.0, f64::NAN],
            max_vals: &[f64::NAN, 0.0, f64::INFINITY],
            ..BulletConfig::default()
        };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.cx.is_finite()));
    }
}
