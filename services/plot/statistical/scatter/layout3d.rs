use super::config::ScatterConfig;
use super::variant::ScatterVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::curves::marker_blocks;
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.5;
pub const COLORMAP: &str = "jet";
const POINT_SIZE: f64 = 0.09;
const POINT_HEIGHT: f64 = 0.12;
const SIZED_MIN: f64 = 0.05;
const SIZED_MAX: f64 = 0.34;
const ROW_PITCH: f64 = 3.2;
const RUG_TICK: f64 = 0.06;
const RUG_HW: f64 = 0.03;
const LINE_TONE: f64 = 0.5;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Points,
    Sized,
    Hue,
    Regression,
    Residual,
    Facet,
    WideForm,
    Rug,
}

fn recipe(variant: ScatterVariant) -> Glyph {
    use ScatterVariant::*;
    match variant {
        Basic | Symbols | Labeled | Categorical | DualStyle => Glyph::Points,
        ContinuousHue => Glyph::Hue,
        Sized => Glyph::Sized,
        Regression => Glyph::Regression,
        Residual => Glyph::Residual,
        Facet => Glyph::Facet,
        WideForm => Glyph::WideForm,
        Rug => Glyph::Rug,
    }
}

fn class_of(categories: &[String], i: usize) -> usize {
    if categories.is_empty() {
        return 0;
    }
    let key = categories.get(i).map(|s| s.as_str()).unwrap_or("");
    categories.iter().position(|c| c == key).unwrap_or(0)
}

fn linear_fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let n = xs.len().min(ys.len());
    if n < 2 {
        return None;
    }
    let (mx, my) = (xs[..n].iter().sum::<f64>() / n as f64, ys[..n].iter().sum::<f64>() / n as f64);
    let (mut num, mut den) = (0.0, 0.0);
    for i in 0..n {
        num += (xs[i] - mx) * (ys[i] - my);
        den += (xs[i] - mx).powi(2);
    }
    if den < 1e-12 {
        return None;
    }
    let slope = num / den;
    Some((slope, my - slope * mx))
}

pub fn layout_3d(cfg: &ScatterConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &ScatterConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let glyph = recipe(cfg.variant);

    if glyph == Glyph::WideForm && !cfg.series.is_empty() {
        let n = cfg.x_values.len();
        let keep = even_indices(n, budget.cloud() / cfg.series.len().max(1));
        let x = pick(cfg.x_values, &keep);
        let blocks: Vec<Bar3DBlock> = cfg
            .series
            .iter()
            .enumerate()
            .flat_map(|(s, (_, values))| {
                let ys = pick(values, &keep);
                let len = x.len().min(ys.len());
                (0..len).map(|i| Bar3DBlock::new(x[i], s as f64 * ROW_PITCH, 0.0, POINT_HEIGHT, POINT_SIZE, POINT_SIZE, s)).collect::<Vec<_>>()
            })
            .collect();
        let names: Vec<String> = cfg.series.iter().map(|(name, _)| name.clone()).collect();
        return (blocks, names);
    }

    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let keep = even_indices(n, budget.cloud());
    let x = pick(cfg.x_values, &keep);
    let y = pick(cfg.y_values, &keep);
    let categories = pick(cfg.categories, &keep);
    let color_values = pick(cfg.color_values, &keep);
    let labels = pick(cfg.labels, &keep);
    let names: Vec<String> = if labels.len() == keep.len() { labels } else { (0..keep.len()).map(|i| format!("Point {}", i + 1)).collect() };

    if glyph == Glyph::Facet {
        let order = {
            let mut o = Vec::new();
            for c in &categories {
                if !o.contains(c) {
                    o.push(c.clone());
                }
            }
            o
        };
        let blocks: Vec<Bar3DBlock> = (0..keep.len())
            .map(|i| {
                let row = order.iter().position(|c| c == categories.get(i).map(|s| s.as_str()).unwrap_or("")).unwrap_or(0);
                Bar3DBlock::new(x[i], row as f64 * ROW_PITCH, 0.0, POINT_HEIGHT, POINT_SIZE, POINT_SIZE, row)
            })
            .collect();
        return (blocks, names);
    }

    let (clo, chi) = color_values.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    let crange = (chi - clo).max(1e-9);
    let has_colors = color_values.len() == keep.len() && clo.is_finite();

    let mut blocks: Vec<Bar3DBlock> = (0..keep.len())
        .map(|i| {
            let class = class_of(&categories, i);
            match glyph {
                Glyph::Sized if has_colors => {
                    let frac = ((color_values[i] - clo) / crange).clamp(0.0, 1.0);
                    let size = SIZED_MIN + frac * (SIZED_MAX - SIZED_MIN);
                    Bar3DBlock::new(x[i], y[i], 0.0, POINT_HEIGHT, size, size, class).with_tone(frac)
                }
                Glyph::Hue if has_colors => {
                    let frac = ((color_values[i] - clo) / crange).clamp(0.0, 1.0);
                    Bar3DBlock::new(x[i], y[i], 0.0, POINT_HEIGHT, POINT_SIZE, POINT_SIZE, class).with_tone(frac)
                }
                _ => Bar3DBlock::new(x[i], y[i], 0.0, POINT_HEIGHT, POINT_SIZE, POINT_SIZE, class),
            }
        })
        .collect();

    if matches!(glyph, Glyph::Regression | Glyph::Residual) {
        if let Some((slope, intercept)) = linear_fit(&x, &y) {
            if glyph == Glyph::Residual {
                for (i, b) in blocks.iter_mut().enumerate() {
                    let fitted = slope * x[i] + intercept;
                    b.cy = y[i] - fitted;
                }
            } else {
                let (xlo, xhi) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
                if xhi > xlo {
                    let line = [(xlo, slope * xlo + intercept), (xhi, slope * xhi + intercept)];
                    blocks.extend(marker_blocks(&line, POINT_SIZE * 0.6, POINT_HEIGHT * 1.5, 0.0, blocks.len(), Some(LINE_TONE)));
                }
            }
        }
    }

    if glyph == Glyph::Rug {
        let (ylo, _) = y.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
        let (xlo, _) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
        for i in 0..keep.len() {
            blocks.push(Bar3DBlock::new(x[i], ylo - 0.6, 0.0, RUG_TICK, RUG_HW, RUG_HW, class_of(&categories, i)));
            blocks.push(Bar3DBlock::new(xlo - 0.6, y[i], 0.0, RUG_TICK, RUG_HW, RUG_HW, class_of(&categories, i)));
        }
    }

    (blocks, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xy() -> (Vec<f64>, Vec<f64>) {
        (vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2.1, 3.9, 6.2, 7.8, 10.1, 12.0])
    }

    fn draw(variant: ScatterVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (x, y) = xy();
        let cfg = ScatterConfig { variant, x_values: &x, y_values: &y, ..ScatterConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_scatter_variant_draws_every_point() {
        for &variant in ScatterVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert!(!names.is_empty(), "{variant:?}");
        }
    }

    #[test]
    fn categorical_gives_each_category_its_own_class() {
        let (x, y) = xy();
        let categories = vec!["A".to_string(), "A".to_string(), "B".to_string(), "B".to_string(), "C".to_string(), "C".to_string()];
        let cfg = ScatterConfig { variant: ScatterVariant::Categorical, x_values: &x, y_values: &y, categories: &categories, ..ScatterConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let classes: std::collections::BTreeSet<usize> = blocks.iter().map(|b| b.ci).collect();
        assert_eq!(classes.len(), 3);
    }

    #[test]
    fn sized_scales_marker_size_with_the_color_value() {
        let (x, y) = xy();
        let cv = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let cfg = ScatterConfig { variant: ScatterVariant::Sized, x_values: &x, y_values: &y, color_values: &cv, ..ScatterConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.last().unwrap().hw > blocks.first().unwrap().hw);
    }

    #[test]
    fn regression_adds_a_fit_line_and_residual_flattens_the_trend() {
        let (basic, _) = draw(ScatterVariant::Basic);
        let (regression, _) = draw(ScatterVariant::Regression);
        assert!(regression.len() > basic.len());
        let (residual, _) = draw(ScatterVariant::Residual);
        let max_abs_y = residual.iter().map(|b| b.cy.abs()).fold(0.0, f64::max);
        assert!(max_abs_y < 2.0);
    }

    #[test]
    fn facet_puts_every_category_on_its_own_row() {
        let (x, y) = xy();
        let categories = vec!["A".to_string(), "A".to_string(), "B".to_string(), "B".to_string(), "C".to_string(), "C".to_string()];
        let cfg = ScatterConfig { variant: ScatterVariant::Facet, x_values: &x, y_values: &y, categories: &categories, ..ScatterConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0).round() as i64).collect();
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn wide_form_gives_every_series_its_own_row() {
        let x = vec![1.0, 2.0, 3.0];
        let series = vec![("A".to_string(), vec![1.0, 2.0, 3.0]), ("B".to_string(), vec![3.0, 2.0, 1.0])];
        let cfg = ScatterConfig { variant: ScatterVariant::WideForm, x_values: &x, series: &series, ..ScatterConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!(names, vec!["A".to_string(), "B".to_string()]);
        let rows: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy * 1000.0).round() as i64).collect();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn rug_adds_ticks_on_both_margins() {
        let (basic, _) = draw(ScatterVariant::Basic);
        let (rug, _) = draw(ScatterVariant::Rug);
        assert_eq!(rug.len(), basic.len() * 3);
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&ScatterConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_huge_point_cloud_is_capped_by_the_budget() {
        let x: Vec<f64> = (0..300_000).map(|i| (i % 100) as f64).collect();
        let y: Vec<f64> = (0..300_000).map(|i| ((i * 3) % 100) as f64).collect();
        let cfg = ScatterConfig { x_values: &x, y_values: &y, ..ScatterConfig::default() };
        let budget = Budget::new(Some(600));
        let (blocks, _) = layout_named(&cfg, &budget);
        assert_eq!(blocks.len(), budget.cloud());
    }
}
