use super::common::group_values;
use super::config::BoxplotConfig;
use super::variant::BoxplotVariant;
use crate::plot::statistical::_3d::generic::transposed;
use crate::plot::statistical::_3d::spread::{
    box_blocks, letter_value_blocks, outlier_blocks, overall_span, point_blocks, violin_blocks, BoxStyle, Group,
    Scatter,
};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.9;
pub const COLORMAP: &str = "updown";
const BOX_HW: f64 = 0.3;
const SLIM_HW: f64 = 0.1;
const BOX_DEPTH: f64 = 0.3;
const WHISKER_HW: f64 = 0.04;
const VIOLIN_SLICES: usize = 16;
const POINT_SHARE: f64 = 0.03;
const GROUP_PITCH: f64 = 0.8;

#[derive(Clone, Copy)]
enum Body {
    Box,
    Notched,
    Slim,
    Violin,
    LetterValue,
}

#[derive(Clone, Copy)]
struct Recipe {
    body: Body,
    cloud: Option<Scatter>,
    outliers: bool,
    swap: bool,
    grouped: bool,
}

impl Recipe {
    const fn of(body: Body) -> Self {
        Self { body, cloud: None, outliers: false, swap: false, grouped: false }
    }

    const fn cloud(mut self, scatter: Scatter) -> Self {
        self.cloud = Some(scatter);
        self
    }

    const fn outliers(mut self) -> Self {
        self.outliers = true;
        self
    }

    const fn swapped(mut self) -> Self {
        self.swap = true;
        self
    }

    const fn grouped(mut self) -> Self {
        self.grouped = true;
        self
    }
}

fn recipe(variant: BoxplotVariant) -> Recipe {
    use BoxplotVariant::*;
    let plain = Recipe::of(Body::Box);
    match variant {
        Basic => plain,
        Horizontal => plain.swapped(),
        Notched => Recipe::of(Body::Notched),
        Grouped => plain.grouped(),
        Points => plain.cloud(Scatter::Aligned),
        Outliers => plain.outliers(),
        Strip => Recipe::of(Body::Slim).cloud(Scatter::Jitter),
        Swarm => Recipe::of(Body::Slim).cloud(Scatter::Swarm),
        Violin => Recipe::of(Body::Violin),
        LetterValue => Recipe::of(Body::LetterValue),
    }
}

fn split_evenly(samples: &[f64], parts: usize) -> Vec<&[f64]> {
    if parts == 0 || samples.is_empty() {
        return Vec::new();
    }
    let chunk = samples.len().div_ceil(parts);
    (0..parts)
        .map(|i| {
            let (start, end) = (i * chunk, ((i + 1) * chunk).min(samples.len()));
            if start >= samples.len() { &samples[0..0] } else { &samples[start..end] }
        })
        .collect()
}

fn categories(cfg: &BoxplotConfig) -> Vec<Vec<f64>> {
    if cfg.values.is_empty() {
        return cfg.series.to_vec();
    }
    group_values(cfg.category_labels, cfg.values).1
}

fn groups_of<'a>(plan: Recipe, flat: &'a [Vec<f64>], grouped: &'a [Vec<&'a [f64]>]) -> Vec<Group<'a>> {
    if plan.grouped && !grouped.is_empty() {
        let n_series = grouped.len() as f64;
        return grouped
            .iter()
            .enumerate()
            .flat_map(|(g, row)| {
                row.iter().enumerate().map(move |(c, samples)| Group {
                    samples,
                    cx: c as f64,
                    cy: (g as f64 - (n_series - 1.0) / 2.0) * GROUP_PITCH,
                    class: g,
                })
            })
            .collect();
    }
    flat.iter()
        .enumerate()
        .map(|(i, samples)| Group { samples, cx: i as f64, cy: 0.0, class: i })
        .collect()
}

pub fn layout_3d(cfg: &BoxplotConfig) -> Vec<Bar3DBlock> {
    let plan = recipe(cfg.variant);
    let flat = categories(cfg);
    let n_cats = cfg.category_labels.len().max(1);
    let split: Vec<Vec<&[f64]>> = if plan.grouped {
        cfg.series.iter().map(|s| split_evenly(s, n_cats)).collect()
    } else {
        Vec::new()
    };
    let groups = groups_of(plan, &flat, &split);
    if groups.is_empty() {
        return Vec::new();
    }
    let size = overall_span(&groups) * POINT_SHARE;
    let hw = if plan.grouped { BOX_HW * 0.5 } else { BOX_HW };
    let style = BoxStyle { hw, depth: BOX_DEPTH, whisker_hw: WHISKER_HW, notch: false, median: true };
    let mut blocks = match plan.body {
        Body::Box => box_blocks(&groups, &style),
        Body::Notched => box_blocks(&groups, &BoxStyle { notch: true, ..style }),
        Body::Slim => box_blocks(&groups, &BoxStyle { hw: SLIM_HW, ..style }),
        Body::Violin => violin_blocks(&groups, VIOLIN_SLICES, BOX_HW, BOX_DEPTH),
        Body::LetterValue => letter_value_blocks(&groups, cfg.boxen_depth.max(1), BOX_HW, BOX_DEPTH),
    };
    if let Some(scatter) = plan.cloud {
        blocks.extend(point_blocks(&groups, scatter, cfg.jitter.max(0.05) * BOX_HW / 0.35, size));
    }
    if plan.outliers {
        blocks.extend(outlier_blocks(&groups, size));
    }
    if plan.swap {
        blocks = transposed(blocks);
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels() -> Vec<String> {
        ["A", "B", "C"].iter().map(|s| s.to_string()).collect()
    }

    fn series() -> Vec<Vec<f64>> {
        vec![
            vec![1.2, 2.4, 2.7, 3.1, 3.5, 3.8, 4.2, 5.1, 6.0, 15.0, -6.0],
            vec![2.0, 2.8, 3.2, 3.6, 4.1, 4.5, 5.0, 5.7, 6.5],
            vec![1.8, 2.2, 2.6, 3.0, 3.4, 3.9, 4.3, 4.9, 5.5],
        ]
    }

    fn blocks_for(variant: BoxplotVariant) -> Vec<Bar3DBlock> {
        let names = labels();
        let data = series();
        layout_3d(&BoxplotConfig {
            variant,
            category_labels: &names,
            series: &data,
            ..BoxplotConfig::default()
        })
    }

    #[test]
    fn every_boxplot_variant_draws_something() {
        for variant in BoxplotVariant::all() {
            assert!(!blocks_for(*variant).is_empty(), "{} must draw blocks", variant.name());
        }
    }

    #[test]
    fn horizontal_is_the_basic_layout_with_swapped_footprint_axes() {
        let basic = blocks_for(BoxplotVariant::Basic);
        let horizontal = blocks_for(BoxplotVariant::Horizontal);
        assert_eq!(basic.len(), horizontal.len());
        assert_eq!(basic[3].cx, horizontal[3].cy);
        assert_eq!(basic[3].hw, horizontal[3].hd);
    }

    #[test]
    fn point_variants_add_one_cube_per_sample_on_top_of_the_boxes() {
        let boxes = blocks_for(BoxplotVariant::Basic).len();
        assert_eq!(blocks_for(BoxplotVariant::Points).len(), boxes + 29);
        assert!(blocks_for(BoxplotVariant::Outliers).len() > boxes);
    }

    #[test]
    fn grouped_spreads_each_series_across_the_depth_axis() {
        let names = labels();
        let data = vec![vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0]];
        let blocks = layout_3d(&BoxplotConfig {
            variant: BoxplotVariant::Grouped,
            category_labels: &names,
            series: &data,
            ..BoxplotConfig::default()
        });
        let mut rows: Vec<i64> = blocks.iter().map(|b| (b.cy * 100.0) as i64).collect();
        rows.sort();
        rows.dedup();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn values_with_repeated_labels_are_grouped_like_the_2d_chart() {
        let names: Vec<String> = ["A", "A", "A", "B", "B", "B"].iter().map(|s| s.to_string()).collect();
        let values = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let blocks = layout_3d(&BoxplotConfig {
            category_labels: &names,
            values: &values,
            ..BoxplotConfig::default()
        });
        assert_eq!(blocks.len(), 6);
    }
}
