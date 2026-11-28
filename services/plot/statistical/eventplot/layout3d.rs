use super::config::EventplotConfig;
use super::variant::EventplotVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::curves::ribbon_blocks;
use crate::plot::statistical::_3d::spread::grouped_by_label;
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::kde::{kde_eval, scott_bw};

pub const HEIGHT_RATIO: f64 = 0.5;
pub const COLORMAP: &str = "jet";
const ROW_PITCH: f64 = 1.2;
const TICK_HW: f64 = 0.03;
const TICK_HEIGHT: f64 = 0.4;
const RIBBON_DEPTH: f64 = 0.08;
const CURVE_HEIGHT: f64 = 0.7;
const CURVE_POINTS: usize = 48;
const CAP: usize = 4000;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Ticks,
    Density,
    Connected,
}

fn recipe(variant: EventplotVariant) -> Glyph {
    use EventplotVariant::*;
    match variant {
        Basic => Glyph::Ticks,
        Density => Glyph::Density,
        Connected => Glyph::Connected,
    }
}

fn ticks(events: &[f64], row: f64, class: usize) -> Vec<Bar3DBlock> {
    events.iter().map(|&x| Bar3DBlock::new(x, row, 0.0, TICK_HEIGHT, TICK_HW, RIBBON_DEPTH, class)).collect()
}

pub fn layout_3d(cfg: &EventplotConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &EventplotConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let groups = grouped_by_label(cfg.categories, cfg.x_values);
    let groups: Vec<(String, Vec<f64>)> = if groups.is_empty() && !cfg.x_values.is_empty() {
        vec![("Events".to_string(), cfg.x_values.to_vec())]
    } else {
        groups
    };
    if groups.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let glyph = recipe(cfg.variant);
    let names: Vec<String> = groups.iter().map(|(name, _)| name.clone()).collect();
    let cap = (budget.elements() / groups.len().max(1)).max(8).min(CAP);
    let blocks = groups
        .iter()
        .enumerate()
        .flat_map(|(row, (_, raw))| {
            let keep = even_indices(raw.len(), cap);
            let events = pick(raw, &keep);
            let row_y = row as f64 * ROW_PITCH;
            match glyph {
                Glyph::Ticks => ticks(&events, row_y, row),
                Glyph::Connected => {
                    let mut sorted = events.clone();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let mut out = ticks(&events, row_y, row);
                    out.extend(sorted.windows(2).map(|w| {
                        Bar3DBlock::new((w[0] + w[1]) / 2.0, row_y, TICK_HEIGHT * 0.4, TICK_HEIGHT * 0.6, (w[1] - w[0]).abs() / 2.0, RIBBON_DEPTH * 0.5, row)
                    }));
                    out
                }
                Glyph::Density => {
                    let mut out = ticks(&events, row_y, row);
                    let (lo, hi) = events.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
                    if lo.is_finite() && hi > lo {
                        let bw = scott_bw(&events).max((hi - lo) * 0.01);
                        let curve: Vec<(f64, f64)> = (0..CURVE_POINTS)
                            .map(|i| {
                                let x = lo + (hi - lo) * i as f64 / (CURVE_POINTS - 1).max(1) as f64;
                                (x, kde_eval(&events, x, bw))
                            })
                            .collect();
                        let peak = curve.iter().map(|p| p.1).fold(1e-12, f64::max);
                        let scaled: Vec<(f64, f64)> = curve.iter().map(|&(x, v)| (x, row_y + TICK_HEIGHT + v / peak * CURVE_HEIGHT)).collect();
                        let joined = vec![true; scaled.len().saturating_sub(1)];
                        out.extend(ribbon_blocks(&scaled, &joined, row_y, RIBBON_DEPTH, row, None));
                    }
                    out
                }
            }
        })
        .collect();
    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn categories() -> Vec<String> {
        ["A", "A", "A", "A", "A", "B", "B", "B", "B", "B"].iter().map(|s| s.to_string()).collect()
    }

    fn x() -> Vec<f64> {
        vec![1.0, 2.0, 2.5, 4.0, 5.0, 5.5, 6.0, 2.0, 3.0, 7.0]
    }

    fn draw(variant: EventplotVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let c = categories();
        let xv = x();
        let cfg = EventplotConfig { variant, categories: &c, x_values: &xv, ..EventplotConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_eventplot_variant_draws_something_and_names_every_row() {
        for &variant in EventplotVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn each_row_sits_at_its_own_depth() {
        let (blocks, _) = draw(EventplotVariant::Basic);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0).round() as i64).collect();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn connected_and_density_add_more_blocks_than_plain_ticks() {
        let (basic, _) = draw(EventplotVariant::Basic);
        let (connected, _) = draw(EventplotVariant::Connected);
        let (density, _) = draw(EventplotVariant::Density);
        assert!(connected.len() > basic.len());
        assert!(density.len() > basic.len());
    }

    #[test]
    fn no_categories_falls_back_to_a_single_events_row() {
        let xv = x();
        let cfg = EventplotConfig { x_values: &xv, ..EventplotConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, vec!["Events".to_string()]);
        assert!(!blocks.is_empty());
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&EventplotConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_event_count_stays_within_a_bounded_block_count() {
        let categories: Vec<String> = (0..200_000).map(|i| if i % 2 == 0 { "A".to_string() } else { "B".to_string() }).collect();
        let xv: Vec<f64> = (0..200_000).map(|i| (i % 1000) as f64).collect();
        let cfg = EventplotConfig { categories: &categories, x_values: &xv, ..EventplotConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 12_000);
    }
}
