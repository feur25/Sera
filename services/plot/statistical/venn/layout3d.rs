use super::config::VennConfig;
use super::variant::VennVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::PI;

const ORBIT: f64 = 3.4;
const FOOT: f64 = 1.5;
const BASE_HEIGHT: f64 = 2.4;

#[derive(Clone, Copy)]
struct Recipe {
    by_value: bool,
    tone: f64,
}

impl Recipe {
    const fn of() -> Self {
        Self { by_value: false, tone: -1.0 }
    }

    const fn sized_by_value(mut self) -> Self {
        self.by_value = true;
        self
    }

    const fn toned(mut self, t: f64) -> Self {
        self.tone = t;
        self
    }
}

fn recipe(variant: VennVariant) -> Recipe {
    use VennVariant::*;
    match variant {
        Basic | Filled | Minimal => Recipe::of(),
        Euler => Recipe::of().sized_by_value(),
        Exclusive => Recipe::of().toned(0.3),
    }
}

fn positions(n: usize) -> Vec<(f64, f64)> {
    match n {
        0 => Vec::new(),
        1 => vec![(0.0, 0.0)],
        2 => vec![(-ORBIT * 0.4, 0.0), (ORBIT * 0.4, 0.0)],
        3 => vec![(-ORBIT * 0.42, ORBIT * 0.2), (ORBIT * 0.42, ORBIT * 0.2), (0.0, -ORBIT * 0.32)],
        _ => (0..n)
            .map(|i| {
                let a = 2.0 * PI * i as f64 / n as f64 - PI / 2.0;
                (ORBIT * a.cos(), ORBIT * a.sin())
            })
            .collect(),
    }
}

fn venn_3d(cfg: &VennConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.labels.len();
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let plan = recipe(cfg.variant);
    let pos = positions(n);
    let peak = cfg.values[..cfg.values.len().min(n)].iter().copied().fold(0.0f64, f64::max).max(1.0);
    let blocks = (0..n)
        .map(|i| {
            let (cx, cy) = pos[i];
            let h = if plan.by_value {
                let v = cfg.values.get(i).copied().unwrap_or(1.0);
                (BASE_HEIGHT * (v / peak).sqrt().max(0.25)).max(0.2)
            } else {
                BASE_HEIGHT
            };
            let tone = if plan.tone >= 0.0 { plan.tone } else { i as f64 / n.max(1) as f64 };
            Bar3DBlock::new(cx, cy, 0.0, h, FOOT, FOOT, i).with_tone(tone)
        })
        .collect();
    (blocks, cfg.labels.to_vec())
}

pub fn layout_3d(cfg: &VennConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &VennConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    venn_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sets() -> (Vec<String>, Vec<f64>) {
        (["Set A", "Set B", "Set C"].iter().map(|s| s.to_string()).collect(), vec![40.0, 30.0, 20.0, 15.0, 10.0, 5.0, 8.0])
    }

    fn draw(variant: VennVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, values) = sets();
        let cfg = VennConfig { variant, labels: &labels, values: &values, ..VennConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_set_and_names_every_set() {
        for &variant in VennVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(blocks.len(), 3, "{variant:?}");
            assert_eq!(names.len(), 3, "{variant:?}");
        }
    }

    #[test]
    fn euler_sizes_columns_by_their_own_value_while_basic_stays_uniform() {
        let (basic, _) = draw(VennVariant::Basic);
        assert!(basic.iter().all(|b| (b.z1 - basic[0].z1).abs() < 1e-9));
        let (euler, _) = draw(VennVariant::Euler);
        assert!(euler[0].z1 > euler[1].z1);
        assert!(euler[1].z1 > euler[2].z1);
    }

    #[test]
    fn three_sets_are_not_collinear() {
        let (blocks, _) = draw(VennVariant::Basic);
        let area2 = (blocks[1].cx - blocks[0].cx) * (blocks[2].cy - blocks[0].cy) - (blocks[2].cx - blocks[0].cx) * (blocks[1].cy - blocks[0].cy);
        assert!(area2.abs() > 1e-6);
    }

    #[test]
    fn a_single_set_still_draws_its_own_column_at_the_origin() {
        let labels = vec!["Only".to_string()];
        let values = vec![10.0];
        let cfg = VennConfig { labels: &labels, values: &values, ..VennConfig::default() };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert_eq!((blocks.len(), names.len()), (1, 1));
        assert_eq!((blocks[0].cx, blocks[0].cy), (0.0, 0.0));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&VennConfig::default(), &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
