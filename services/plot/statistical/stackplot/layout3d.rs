use super::common::{prepare, Prepared};
use super::config::StackplotConfig;
use super::variant::StackplotVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::curves::{band_blocks, catmull_rom, Point};
use crate::plot::statistical::_3d::generic::radial_stacked_columns;
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.7;
pub const COLORMAP: &str = "jet";
const DEPTH: f64 = 0.4;
const THIN_DEPTH: f64 = 0.14;
const RADIAL_RADIUS: f64 = 2.6;
const RADIAL_HW: f64 = 0.24;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Bands,
    Radial,
}

#[derive(Clone, Copy)]
struct Recipe {
    glyph: Glyph,
    streamgraph: bool,
    normalized: bool,
    smooth: bool,
    thin: bool,
}

impl Recipe {
    const fn of(glyph: Glyph) -> Self {
        Self { glyph, streamgraph: false, normalized: false, smooth: false, thin: false }
    }

    const fn streamed(mut self) -> Self {
        self.streamgraph = true;
        self
    }

    const fn normalized(mut self) -> Self {
        self.normalized = true;
        self
    }

    const fn smoothed(mut self) -> Self {
        self.smooth = true;
        self.thin = true;
        self
    }
}

fn recipe(variant: StackplotVariant) -> Recipe {
    use StackplotVariant::*;
    match variant {
        Basic => Recipe::of(Glyph::Bands),
        Streamgraph => Recipe::of(Glyph::Bands).streamed(),
        Normalized => Recipe::of(Glyph::Bands).normalized(),
        Ribbon => Recipe::of(Glyph::Bands).smoothed(),
        Radial => Recipe::of(Glyph::Radial),
    }
}

fn prepare_percent(cfg: &StackplotConfig) -> Option<Prepared> {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser == 0 {
        return None;
    }
    let mut totals = vec![0.0_f64; n_pts];
    for (_, vals) in cfg.series {
        for i in 0..n_pts {
            totals[i] += vals.get(i).copied().unwrap_or(0.0).max(0.0);
        }
    }
    let mut bottoms = vec![vec![0.0_f64; n_pts]; n_ser];
    let mut tops = vec![vec![0.0_f64; n_pts]; n_ser];
    for i in 0..n_pts {
        let total = totals[i].max(1e-9);
        let mut cursor = 0.0_f64;
        for (si, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(i).copied().unwrap_or(0.0).max(0.0) / total * 100.0;
            bottoms[si][i] = cursor;
            cursor += v;
            tops[si][i] = cursor;
        }
    }
    Some(Prepared { n_pts, n_ser, bottoms, tops, ymin: 0.0, ymax: 100.0 })
}

fn track_of(values: &[f64]) -> Vec<Point> {
    values.iter().enumerate().map(|(i, &v)| (i as f64, v)).collect()
}

fn banded(p: &Prepared, depth: f64, smooth: bool) -> Vec<Bar3DBlock> {
    (0..p.n_ser)
        .flat_map(|si| {
            let (mut low, mut high) = (track_of(&p.bottoms[si]), track_of(&p.tops[si]));
            if smooth {
                low = catmull_rom(&low, 4, 0.5);
                high = catmull_rom(&high, 4, 0.5);
            }
            band_blocks(&low, &high, 0.0, depth, si)
        })
        .collect()
}

pub fn layout_3d(cfg: &StackplotConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &StackplotConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let plan = recipe(cfg.variant);
    let names: Vec<String> = cfg.series.iter().map(|(name, _)| name.clone()).collect();
    if plan.glyph == Glyph::Radial {
        let clamped: Vec<(String, Vec<f64>)> = cfg
            .series
            .iter()
            .map(|(name, vals)| (name.clone(), vals.iter().map(|v| if v.is_finite() { v.max(0.0) } else { 0.0 }).collect()))
            .collect();
        let blocks = radial_stacked_columns(&clamped, cfg.x_labels.len(), RADIAL_RADIUS, RADIAL_HW, RADIAL_HW);
        return (blocks, names);
    }
    let prepared = if plan.normalized { prepare_percent(cfg) } else { prepare(cfg, plan.streamgraph) };
    let Some(p) = prepared else {
        return (Vec::new(), Vec::new());
    };
    let depth = if plan.thin { THIN_DEPTH } else { DEPTH };
    (banded(&p, depth, plan.smooth), names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series() -> Vec<(String, Vec<f64>)> {
        vec![
            ("A".to_string(), vec![10.0, 14.0, 12.0, 18.0, 20.0]),
            ("B".to_string(), vec![8.0, 9.0, 11.0, 10.0, 13.0]),
            ("C".to_string(), vec![5.0, 6.0, 7.0, 9.0, 8.0]),
        ]
    }

    fn labels() -> Vec<String> {
        (0..5).map(|i| i.to_string()).collect()
    }

    fn draw(variant: StackplotVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let xl = labels();
        let s = series();
        let cfg = StackplotConfig { variant, x_labels: &xl, series: &s, ..StackplotConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_stackplot_variant_draws_something_and_names_every_series() {
        for &variant in StackplotVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names, vec!["A".to_string(), "B".to_string(), "C".to_string()], "{variant:?}");
        }
    }

    #[test]
    fn basic_bands_stack_from_the_floor_series_by_series() {
        let (blocks, names) = draw(StackplotVariant::Basic);
        let b = names.iter().position(|n| n == "B").unwrap();
        assert!(blocks.iter().filter(|blk| blk.ci == b).all(|blk| blk.z0 > 0.0 || blk.end.map(|(z0, _)| z0 > 0.0).unwrap_or(false)));
    }

    #[test]
    fn streamgraph_centres_the_whole_stack_around_zero() {
        let (blocks, _) = draw(StackplotVariant::Streamgraph);
        let has_negative = blocks.iter().any(|b| b.z0 < 0.0 || b.end.map(|(z0, _)| z0 < 0.0).unwrap_or(false));
        assert!(has_negative);
    }

    #[test]
    fn normalized_tops_out_at_one_hundred() {
        let (blocks, names) = draw(StackplotVariant::Normalized);
        let c = names.iter().position(|n| n == "C").unwrap();
        for b in blocks.iter().filter(|blk| blk.ci == c) {
            let top = b.end.map(|(_, z1)| z1).unwrap_or(b.z1);
            assert!(top > 99.0 && top < 101.0, "{top}");
        }
    }

    #[test]
    fn radial_places_every_series_around_a_ring() {
        let (blocks, _) = draw(StackplotVariant::Radial);
        let radii: Vec<f64> = blocks.iter().map(|b| b.cx.hypot(b.cy)).collect();
        assert!(radii.iter().all(|&r| (r - RADIAL_RADIUS).abs() < 1e-6));
        let angles: std::collections::BTreeSet<i64> = blocks.iter().map(|b| (b.cy.atan2(b.cx) * 1000.0) as i64).collect();
        assert_eq!(angles.len(), 5);
    }

    #[test]
    fn ribbon_smooths_the_band_into_more_segments_than_basic() {
        let (basic, _) = draw(StackplotVariant::Basic);
        let (ribbon, _) = draw(StackplotVariant::Ribbon);
        assert!(ribbon.len() > basic.len());
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&StackplotConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn non_finite_values_are_flattened_instead_of_poisoning_the_scene() {
        let xl = labels();
        let s = vec![("A".to_string(), vec![f64::NAN, f64::INFINITY, -5.0, 3.0, 4.0])];
        for &variant in StackplotVariant::all() {
            let cfg = StackplotConfig { variant, x_labels: &xl, series: &s, ..StackplotConfig::default() };
            let (blocks, _) = layout_named(&cfg, &Budget::default());
            assert!(blocks.iter().all(|b| b.cx.is_finite() && b.cy.is_finite() && b.hw.is_finite()), "{variant:?}");
        }
    }
}
