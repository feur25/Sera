use super::super::bar::Bar3DBlock;
use super::zone::cube_height;
use std::collections::HashMap;
use std::f64::consts::TAU;

const MIN_RING: f64 = 2.5;
const DIAMOND: [f64; 4] = [0.35, 0.85, 0.85, 0.35];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Head {
    Cube,
    Diamond,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tips {
    Far,
    Both,
}

impl Head {
    fn layers(self) -> &'static [f64] {
        match self {
            Head::Cube => &[1.0],
            Head::Diamond => &DIAMOND,
        }
    }
}

pub fn finite(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| if v.is_finite() { *v } else { 0.0 }).collect()
}

pub fn ring_radius(n: usize, slot: f64) -> f64 {
    (n as f64 * slot / TAU).max(MIN_RING)
}

pub fn group_ids(groups: &[String], n: usize) -> (Vec<usize>, Vec<String>) {
    if groups.len() != n {
        return (vec![0; n], Vec::new());
    }
    let mut slots: HashMap<&str, usize> = HashMap::new();
    let mut names: Vec<String> = Vec::new();
    let ids = groups
        .iter()
        .map(|group| {
            *slots.entry(group.as_str()).or_insert_with(|| {
                names.push(group.clone());
                names.len() - 1
            })
        })
        .collect();
    (ids, names)
}

pub fn opposed(values: &[f64], ids: &[usize]) -> Vec<f64> {
    values
        .iter()
        .zip(ids)
        .map(|(v, id)| if id % 2 == 0 { v.abs() } else { -v.abs() })
        .collect()
}

pub fn classed(mut blocks: Vec<Bar3DBlock>, classes: &[usize]) -> Vec<Bar3DBlock> {
    for (block, &class) in blocks.iter_mut().zip(classes) {
        block.ci = class;
    }
    blocks
}

pub fn toned(mut blocks: Vec<Bar3DBlock>, tones: &[f64]) -> Vec<Bar3DBlock> {
    for (block, &tone) in blocks.iter_mut().zip(tones) {
        block.tone = Some(tone.clamp(0.0, 1.0));
    }
    blocks
}

fn far_end(stem: &Bar3DBlock) -> f64 {
    if stem.z1.abs() >= stem.z0.abs() { stem.z1 } else { stem.z0 }
}

fn head_at(stem: &Bar3DBlock, z: f64, style: Head, width: f64, tall: f64) -> Vec<Bar3DBlock> {
    let layers = style.layers();
    let each = tall / layers.len() as f64;
    layers
        .iter()
        .enumerate()
        .map(|(k, scale)| {
            let z0 = z - tall / 2.0 + each * k as f64;
            let block = Bar3DBlock::new(stem.cx, stem.cy, z0, z0 + each, width * scale, width * scale, stem.ci);
            match stem.tone {
                Some(tone) => block.with_tone(tone),
                None => block,
            }
        })
        .collect()
}

pub fn heads(stems: &[Bar3DBlock], tips: Tips, style: Head, width: f64, height_ratio: f64) -> Vec<Bar3DBlock> {
    let tall = cube_height(stems, width * 2.0, height_ratio);
    stems
        .iter()
        .flat_map(|stem| {
            let ends = match tips {
                Tips::Far => vec![far_end(stem)],
                Tips::Both => vec![stem.z0, stem.z1],
            };
            ends.into_iter().flat_map(move |z| head_at(stem, z, style, width, tall))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::statistical::_3d::generic::diverging_columns;
    use crate::plot::statistical::_3d::zone::{fit, Fit};

    fn stems() -> Vec<Bar3DBlock> {
        toned(classed(diverging_columns(&[10.0, -20.0, 5.0], 0.05, 0.05), &[7, 8, 9]), &[1.0, 0.0, 0.5])
    }

    #[test]
    fn heads_sit_on_the_far_end_of_each_stem_and_inherit_class_and_tone() {
        let tops = heads(&stems(), Tips::Far, Head::Cube, 0.3, 0.8);
        assert_eq!(tops.len(), 3);
        for (head, expected) in tops.iter().zip([10.0, -20.0, 5.0]) {
            assert!(((head.z0 + head.z1) / 2.0 - expected).abs() < 1e-9);
        }
        assert_eq!(tops.iter().map(|h| h.ci).collect::<Vec<_>>(), vec![7, 8, 9]);
        assert_eq!(tops[1].tone, Some(0.0));
    }

    #[test]
    fn both_tips_put_a_head_on_each_end() {
        let both = heads(&[Bar3DBlock::new(0.0, 0.0, 2.0, 9.0, 0.05, 0.05, 0)], Tips::Both, Head::Cube, 0.3, 0.8);
        for (head, expected) in both.iter().zip([2.0, 9.0]) {
            assert!(((head.z0 + head.z1) / 2.0 - expected).abs() < 1e-9);
        }
    }

    #[test]
    fn a_diamond_head_is_four_tapering_layers_stacked_around_the_tip() {
        let gem = heads(&stems()[..1], Tips::Far, Head::Diamond, 0.4, 0.8);
        assert_eq!(gem.len(), 4);
        assert!(gem[0].hw < gem[1].hw && gem[3].hw < gem[2].hw);
        assert!((gem[0].z1 - gem[1].z0).abs() < 1e-9);
        let middle = (gem[0].z0 + gem[3].z1) / 2.0;
        assert!((middle - 10.0).abs() < 1e-9);
    }

    #[test]
    fn head_cubes_look_square_once_the_scene_is_fitted() {
        let stems: Vec<Bar3DBlock> = (0..8).map(|i| Bar3DBlock::new(i as f64, 0.0, 0.0, 50.0 + i as f64, 0.05, 0.05, i)).collect();
        let cubes = heads(&stems, Tips::Far, Head::Cube, 0.3, 0.8);
        let zone = fit(&stems, 0.8, None, Fit::Uniform);
        let wide = 0.6 / zone.scale[0];
        let tall = (cubes[0].z1 - cubes[0].z0) / zone.scale[2];
        assert!((wide - tall).abs() < 1e-9);
    }

    #[test]
    fn groups_are_numbered_by_first_appearance_and_missing_groups_share_one_id() {
        let names: Vec<String> = ["b", "a", "b", "c"].iter().map(|s| s.to_string()).collect();
        assert_eq!(group_ids(&names, 4), (vec![0, 1, 0, 2], vec!["b".to_string(), "a".to_string(), "c".to_string()]));
        assert_eq!(group_ids(&names, 5), (vec![0; 5], Vec::<String>::new()));
    }

    #[test]
    fn opposed_values_face_each_other_by_the_parity_of_their_group() {
        assert_eq!(opposed(&[3.0, -4.0, 5.0, 6.0], &[0, 1, 0, 1]), vec![3.0, -4.0, 5.0, -6.0]);
    }

    #[test]
    fn rings_grow_with_the_count_and_keep_a_floor() {
        assert_eq!(ring_radius(3, 1.0), MIN_RING);
        assert!(ring_radius(600, 1.0) > ring_radius(60, 1.0));
    }

    #[test]
    fn non_finite_values_become_zero() {
        assert_eq!(finite(&[1.0, f64::NAN, f64::INFINITY, -2.0]), vec![1.0, 0.0, 0.0, -2.0]);
    }

    #[test]
    fn classes_and_tones_apply_by_position_and_tones_are_clamped() {
        let out = toned(classed(vec![Bar3DBlock::new(0.0, 0.0, 0.0, 1.0, 0.1, 0.1, 0)], &[4]), &[3.0]);
        assert_eq!((out[0].ci, out[0].tone), (4, Some(1.0)));
    }
}
