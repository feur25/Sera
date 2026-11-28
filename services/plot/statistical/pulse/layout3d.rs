use super::config::PulseConfig;
use super::variant::PulseVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::generic::radial_columns;
use crate::plot::statistical::_3d::lineage::{paths, Point, EDGE_STEPS};
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::TAU;

const RADIUS: f64 = 3.4;

#[derive(Clone, Copy)]
struct Recipe {
    hw: f64,
    wire: bool,
}

impl Recipe {
    const fn of(hw: f64) -> Self {
        Self { hw, wire: false }
    }

    const fn wired(mut self) -> Self {
        self.wire = true;
        self
    }
}

fn recipe(variant: PulseVariant) -> Recipe {
    use PulseVariant::*;
    match variant {
        Radial => Recipe::of(0.42),
        Outlined => Recipe::of(0.16),
        Filled => Recipe::of(0.55),
        Dot => Recipe::of(0.18).wired(),
        Wave => Recipe::of(0.14).wired(),
    }
}

fn pulse_3d(cfg: &PulseConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let values = &cfg.values[..n];
    let mut blocks = radial_columns(values, RADIUS, plan.hw, plan.hw);
    for (i, b) in blocks.iter_mut().enumerate() {
        b.tone = Some(i as f64 / n.max(1) as f64);
    }
    if plan.wire {
        let tops: Vec<Point> = (0..n)
            .map(|i| {
                let a = TAU * i as f64 / n as f64 - std::f64::consts::FRAC_PI_2;
                (RADIUS * a.cos(), RADIUS * a.sin(), values[i].max(0.0))
            })
            .collect();
        let mut closed = tops.clone();
        if let Some(&first) = tops.first() {
            closed.push(first);
        }
        blocks.extend(paths(&[closed], 0.05, EDGE_STEPS * n.max(1), |_| n, |_| 0.5));
    }
    (blocks, cfg.labels[..n].to_vec())
}

pub fn layout_3d(cfg: &PulseConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &PulseConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    pulse_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn week() -> (Vec<String>, Vec<f64>) {
        (
            ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].iter().map(|s| s.to_string()).collect(),
            vec![0.4, 0.7, 0.9, 0.6, 0.8, 0.3, 0.5],
        )
    }

    fn draw(variant: PulseVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, values) = week();
        let cfg = PulseConfig { variant, labels: &labels, values: &values, ..PulseConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_label_and_names_every_label() {
        for &variant in PulseVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 7, "{variant:?}");
            assert!(blocks.len() >= 7, "{variant:?}");
        }
    }

    #[test]
    fn a_higher_value_stands_taller() {
        let (blocks, names) = draw(PulseVariant::Radial);
        let wed = blocks[names.iter().position(|n| n == "Wed").unwrap()];
        let sat = blocks[names.iter().position(|n| n == "Sat").unwrap()];
        assert!(wed.z1 > sat.z1);
    }

    #[test]
    fn wave_and_dot_add_a_closing_wire_that_radial_does_not_have() {
        let (radial, _) = draw(PulseVariant::Radial);
        let (wave, _) = draw(PulseVariant::Wave);
        assert!(wave.len() > radial.len());
    }

    #[test]
    fn filled_columns_are_wider_than_outlined_columns() {
        let (filled, _) = draw(PulseVariant::Filled);
        let (outlined, _) = draw(PulseVariant::Outlined);
        assert!(filled[0].hw > outlined[0].hw);
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&PulseConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
