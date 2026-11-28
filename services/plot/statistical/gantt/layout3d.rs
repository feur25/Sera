use super::common::{prepare, Prepared};
use super::config::GanttConfig;
use super::variant::GanttVariant;
use crate::plot::statistical::_3d::budget::{Buckets, Budget};
use crate::plot::statistical::_3d::timeline::{timeline, Plan, Task};
use crate::plot::statistical::_3d::zone::Fit;
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::common::format_axis_label;

pub const HEIGHT_RATIO: f64 = 0.12;
pub const COLORMAP: &str = "cyber";
pub const FIT: Fit = Fit::Stretch;
const TONE_LOW: f64 = 0.08;
const TONE_HIGH: f64 = 0.92;
const TONE_MID: f64 = 0.5;

fn plan(variant: GanttVariant) -> Plan {
    match variant {
        GanttVariant::Basic => Plan { progress: false, milestones: false },
        GanttVariant::Progress => Plan { progress: true, milestones: false },
        GanttVariant::Milestone => Plan { progress: false, milestones: true },
    }
}

fn tone_of(order: &[String], category: &str) -> f64 {
    match order.iter().position(|known| known == category) {
        Some(at) if order.len() > 1 => TONE_LOW + (TONE_HIGH - TONE_LOW) * at as f64 / (order.len() - 1) as f64,
        _ => TONE_MID,
    }
}

fn sound(value: f64) -> f64 {
    if value.is_finite() { value } else { 0.0 }
}

fn tasks(prepared: &Prepared) -> Vec<Task> {
    (0..prepared.n)
        .map(|i| Task {
            start: sound(prepared.start[i]),
            end: sound(prepared.end[i]),
            progress: sound(prepared.progress[i]),
            tone: tone_of(&prepared.cat_order, &prepared.categories[i]),
        })
        .collect()
}

fn names_of(prepared: &Prepared, tasks: &[Task]) -> Vec<String> {
    prepared
        .labels
        .iter()
        .zip(&prepared.categories)
        .zip(tasks)
        .flat_map(|((label, category), task)| {
            let span = format!("{} → {}", format_axis_label(task.start), format_axis_label(task.end));
            let head = if category.is_empty() { label.clone() } else { format!("{label} · {category}") };
            [format!("{head} · {span}"), format!("{label} · {}% done", (task.progress * 100.0).round())]
        })
        .collect()
}

pub fn layout_3d(cfg: &GanttConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &GanttConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values_start.len()).min(cfg.values_end.len());
    let buckets = Buckets::new(n, budget.points);
    if buckets.is_identity() {
        return gantt_3d(cfg);
    }
    let labels = buckets.first(&cfg.labels[..n]);
    let start = buckets.mean(&cfg.values_start[..n]);
    let end = buckets.mean(&cfg.values_end[..n]);
    let categories = if cfg.categories.len() >= n { buckets.first(&cfg.categories[..n]) } else { Vec::new() };
    let progress = if cfg.progress.len() >= n { buckets.mean(&cfg.progress[..n]) } else { Vec::new() };
    gantt_3d(&GanttConfig {
        variant: cfg.variant,
        labels: &labels,
        values_start: &start,
        values_end: &end,
        categories: &categories,
        progress: &progress,
        sort_order: cfg.sort_order,
        ..GanttConfig::default()
    })
}

fn gantt_3d(cfg: &GanttConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(prepared) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let rows = tasks(&prepared);
    (timeline(&rows, plan(cfg.variant)), names_of(&prepared, &rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draw(variant: GanttVariant, end: [f64; 4], progress: &[f64]) -> (Vec<Bar3DBlock>, Vec<String>) {
        let labels: Vec<String> = ["Design", "Build", "Test", "Launch"].iter().map(|s| s.to_string()).collect();
        let categories: Vec<String> = ["Plan", "Dev", "Dev", "Plan"].iter().map(|s| s.to_string()).collect();
        let cfg = GanttConfig {
            variant,
            labels: &labels,
            values_start: &[0.0, 5.0, 12.0, 18.0],
            values_end: &end,
            categories: &categories,
            progress,
            ..GanttConfig::default()
        };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_its_bars_and_extras_and_names_two_parts_per_row() {
        let cases = [
            (GanttVariant::Basic, [6.0, 14.0, 19.0, 22.0], 4),
            (GanttVariant::Progress, [6.0, 14.0, 19.0, 22.0], 4 + 3),
            (GanttVariant::Milestone, [6.0, 14.0, 19.0, 18.0], 3 + 2),
        ];
        for (variant, end, expected) in cases {
            let (blocks, names) = draw(variant, end, &[1.0, 0.6, 0.25, 0.0]);
            assert_eq!(blocks.len(), expected, "{variant:?}");
            assert_eq!(names.len(), 8, "{variant:?}");
        }
    }

    #[test]
    fn categories_share_a_tone_and_differ_from_each_other() {
        let (blocks, _) = draw(GanttVariant::Basic, [6.0, 14.0, 19.0, 22.0], &[]);
        assert_eq!(blocks[1].tone, blocks[2].tone);
        assert_eq!(blocks[0].tone, blocks[3].tone);
        assert_ne!(blocks[0].tone, blocks[1].tone);
    }

    #[test]
    fn hover_names_carry_the_task_category_and_span() {
        let (_, names) = draw(GanttVariant::Progress, [6.0, 14.0, 19.0, 22.0], &[1.0, 0.6, 0.25, 0.0]);
        assert_eq!(&names[..4], ["Design · Plan · 0 → 6", "Design · 100% done", "Build · Dev · 5 → 14", "Build · 60% done"]);
    }

    #[test]
    fn a_single_category_or_none_uses_the_middle_tone() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let cfg = GanttConfig { labels: &labels, values_start: &[0.0, 1.0], values_end: &[2.0, 3.0], ..GanttConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.tone == Some(TONE_MID)));
    }

    #[test]
    fn long_inputs_are_pooled_to_the_budget_with_matching_names() {
        let labels: Vec<String> = (0..1000).map(|i| format!("T{i}")).collect();
        let start: Vec<f64> = (0..1000).map(|i| i as f64).collect();
        let end: Vec<f64> = (0..1000).map(|i| i as f64 + 5.0).collect();
        let cfg = GanttConfig { labels: &labels, values_start: &start, values_end: &end, ..GanttConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::new(Some(100)));
        assert_eq!((blocks.len(), names.len()), (100, 200));
        assert!(names[2].starts_with("T10 "));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&GanttConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let cfg = GanttConfig {
            labels: &labels,
            values_start: &[f64::NAN, 1.0],
            values_end: &[f64::INFINITY, f64::NEG_INFINITY],
            ..GanttConfig::default()
        };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.iter().all(|b| b.z0.is_finite() && b.z1.is_finite() && b.cx.is_finite() && b.hw.is_finite()));
    }
}
