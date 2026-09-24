use super::variant::PieVariant;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::generic::radial_columns;
use crate::plot::statistical::bar::Bar3DBlock;
use std::f64::consts::PI;

const RADIUS: f64 = 3.0;
const NESTED_RADIUS: f64 = 4.4;
const HW: f64 = 0.28;
const NESTED_HW: f64 = 0.24;

fn half_ring_columns(values: &[f64], radius: f64, hw: f64) -> Vec<Bar3DBlock> {
    let n = values.len();
    if n == 0 {
        return Vec::new();
    }
    (0..n)
        .map(|i| {
            let a = if n > 1 { PI - PI * i as f64 / (n as f64 - 1.0) } else { PI / 2.0 };
            Bar3DBlock::new(radius * a.cos(), radius * a.sin(), 0.0, values[i].max(0.0), hw, hw, i)
        })
        .collect()
}

fn pie_3d(labels: &[String], values: &[f64], secondary_values: &[f64], variant: PieVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = labels.len().min(values.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let vals = &values[..n];
    let names = labels[..n].to_vec();
    let blocks = match variant {
        PieVariant::Semi => half_ring_columns(vals, RADIUS, HW),
        PieVariant::Nested => {
            let mut blocks = radial_columns(vals, RADIUS, HW, HW);
            blocks.extend(radial_columns(secondary_values, NESTED_RADIUS, NESTED_HW, NESTED_HW));
            blocks
        }
        _ => radial_columns(vals, RADIUS, HW, HW),
    };
    (blocks, names)
}

pub fn layout_3d(labels: &[String], values: &[f64], secondary_values: &[f64], variant: PieVariant, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(labels, values, secondary_values, variant, _budget).0
}

pub fn layout_named(labels: &[String], values: &[f64], secondary_values: &[f64], variant: PieVariant, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    pie_3d(labels, values, secondary_values, variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slices() -> (Vec<String>, Vec<f64>) {
        (["A", "B", "C"].iter().map(|s| s.to_string()).collect(), vec![30.0, 50.0, 20.0])
    }

    fn draw(variant: PieVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (labels, values) = slices();
        layout_named(&labels, &values, &[], variant, &Budget::default())
    }

    #[test]
    fn every_variant_draws_one_column_per_slice_and_names_it() {
        for &variant in PieVariant::all() {
            let (blocks, names) = draw(variant);
            assert_eq!(names.len(), 3, "{variant:?}");
            assert_eq!(blocks.len(), 3, "{variant:?}");
        }
    }

    #[test]
    fn nested_adds_a_second_ring_from_secondary_values() {
        let (labels, values) = slices();
        let secondary = vec![10.0, 15.0, 5.0];
        let (blocks, _) = layout_named(&labels, &values, &secondary, PieVariant::Nested, &Budget::default());
        assert_eq!(blocks.len(), 6);
        let inner_r = blocks[0].cx.hypot(blocks[0].cy);
        let outer_r = blocks[3].cx.hypot(blocks[3].cy);
        assert!(outer_r > inner_r);
    }

    #[test]
    fn semi_spans_only_a_half_circle() {
        let (blocks, _) = draw(PieVariant::Semi);
        assert!(blocks.iter().all(|b| b.cy >= -1e-9));
    }

    #[test]
    fn empty_input_draws_nothing() {
        let (blocks, names) = layout_named(&[], &[], &[], PieVariant::Basic, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
