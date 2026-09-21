use super::super::bar::Bar3DBlock;
use super::zone::cube_height;
use std::collections::HashMap;
use std::f64::consts::TAU;

const MIN_RING: f64 = 2.5;
const GEM_STRETCH: f64 = 1.3;
const GEM_DEPTH: f64 = 0.6;
const PLATE: f64 = 0.22;
const ARROW_LAYERS: [f64; 3] = [1.0, 0.62, 0.28];
const ARROW_LENGTH: f64 = 1.7;
const DASH_DUTY: f64 = 0.55;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Head {
    Cube,
    Plate,
    Diamond,
    Arrow,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tips {
    Far,
    Both,
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

fn gem(stem: &Bar3DBlock, z: f64, width: f64, tall: f64) -> Vec<Bar3DBlock> {
    let half = tall * GEM_STRETCH / 2.0;
    let (tip, belly) = ((z, z), (z - half, z + half));
    let depth = width * GEM_DEPTH;
    vec![
        Bar3DBlock::sloped(stem.cx - width / 2.0, stem.cy, tip, belly, width / 2.0, depth, stem.ci),
        Bar3DBlock::sloped(stem.cx + width / 2.0, stem.cy, belly, tip, width / 2.0, depth, stem.ci),
    ]
}

fn arrow(stem: &Bar3DBlock, z: f64, width: f64, tall: f64) -> Vec<Bar3DBlock> {
    let dir = if z >= (stem.z0 + stem.z1) / 2.0 { 1.0 } else { -1.0 };
    let each = tall * ARROW_LENGTH / ARROW_LAYERS.len() as f64;
    ARROW_LAYERS
        .iter()
        .enumerate()
        .map(|(k, scale)| {
            let (a, b) = (z + dir * each * k as f64, z + dir * each * (k + 1) as f64);
            Bar3DBlock::new(stem.cx, stem.cy, a.min(b), a.max(b), width * scale, width * scale, stem.ci)
        })
        .collect()
}

fn head_at(stem: &Bar3DBlock, z: f64, style: Head, width: f64, tall: f64) -> Vec<Bar3DBlock> {
    let blocks = match style {
        Head::Cube => vec![Bar3DBlock::new(stem.cx, stem.cy, z - tall / 2.0, z + tall / 2.0, width, width, stem.ci)],
        Head::Plate => vec![Bar3DBlock::new(stem.cx, stem.cy, z - tall * PLATE / 2.0, z + tall * PLATE / 2.0, width, width, stem.ci)],
        Head::Diamond => gem(stem, z, width, tall),
        Head::Arrow => arrow(stem, z, width, tall),
    };
    blocks
        .into_iter()
        .map(|block| match stem.tone {
            Some(tone) => block.with_tone(tone),
            None => block,
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

pub fn headed_at(stems: &[Bar3DBlock], levels: &[f64], style: Head, width: f64, height_ratio: f64) -> Vec<Bar3DBlock> {
    let tall = cube_height(stems, width * 2.0, height_ratio);
    stems.iter().zip(levels).flat_map(|(stem, &z)| head_at(stem, z, style, width, tall)).collect()
}

pub fn dashed(stems: &[Bar3DBlock], dashes: usize) -> Vec<Bar3DBlock> {
    let count = dashes.max(1);
    stems
        .iter()
        .flat_map(|stem| {
            let (low, high) = stem.z_range();
            let step = (high - low) / count as f64;
            (0..count).map(move |k| {
                let z0 = low + step * k as f64;
                Bar3DBlock { z0, z1: z0 + step * DASH_DUTY, ..*stem }
            })
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
    fn a_diamond_head_is_a_rhombic_prism_made_of_two_wedges_meeting_at_the_belly() {
        let stone = heads(&stems()[..1], Tips::Far, Head::Diamond, 0.4, 0.8);
        assert_eq!(stone.len(), 2);
        let (left, right) = (stone[0], stone[1]);
        assert_eq!((left.z0, left.z1), (10.0, 10.0));
        assert_eq!(left.end, Some((right.z0, right.z1)));
        assert_eq!(right.end, Some((10.0, 10.0)));
        assert!((left.cx + right.cx).abs() < 1e-9);
        let (low, high) = left.z_range();
        assert!(((low + high) / 2.0 - 10.0).abs() < 1e-9);
    }

    #[test]
    fn heads_can_be_placed_at_explicit_levels_one_per_stem() {
        let rods = [Bar3DBlock::new(0.0, 0.0, 0.0, 10.0, 0.05, 0.05, 3), Bar3DBlock::new(1.0, 0.0, 0.0, 10.0, 0.05, 0.05, 4)];
        let placed = headed_at(&rods, &[2.0, 7.0], Head::Cube, 0.3, 0.8);
        assert_eq!(placed.len(), 2);
        assert!(((placed[1].z0 + placed[1].z1) / 2.0 - 7.0).abs() < 1e-9);
        assert_eq!((placed[0].ci, placed[1].cx), (3, 1.0));
    }

    #[test]
    fn plates_are_flatter_than_cubes_at_the_same_width() {
        let rod = [Bar3DBlock::new(0.0, 0.0, 0.0, 10.0, 0.05, 0.05, 0)];
        let cube = headed_at(&rod, &[10.0], Head::Cube, 0.3, 0.8)[0];
        let disc = headed_at(&rod, &[10.0], Head::Plate, 0.3, 0.8)[0];
        assert!((disc.z1 - disc.z0) < (cube.z1 - cube.z0) * 0.3);
        assert_eq!(disc.hw, cube.hw);
    }

    #[test]
    fn arrows_narrow_away_from_the_stem_in_the_direction_of_travel() {
        let rod = [Bar3DBlock::new(0.0, 0.0, 0.0, 10.0, 0.05, 0.05, 0)];
        let up = headed_at(&rod, &[10.0], Head::Arrow, 0.3, 0.8);
        assert_eq!(up.len(), 3);
        assert!(up.iter().all(|b| b.z0 >= 10.0 - 1e-9));
        assert!(up[0].hw > up[1].hw && up[1].hw > up[2].hw);
        assert!(up[0].z1 <= up[1].z0 + 1e-9 && up[1].z1 <= up[2].z0 + 1e-9);
        let down = headed_at(&rod, &[0.0], Head::Arrow, 0.3, 0.8);
        assert!(down.iter().all(|b| b.z1 <= 1e-9));
    }

    #[test]
    fn dashes_cut_every_stem_into_evenly_spaced_segments_with_gaps() {
        let rod = [Bar3DBlock::new(2.0, 1.0, 0.0, 10.0, 0.05, 0.05, 6).with_tone(0.5)];
        let cut = dashed(&rod, 4);
        assert_eq!(cut.len(), 4);
        assert!((cut[1].z0 - 2.5).abs() < 1e-9);
        assert!(cut[0].z1 < cut[1].z0);
        assert_eq!((cut[3].cx, cut[3].ci, cut[3].tone), (2.0, 6, Some(0.5)));
        assert_eq!(dashed(&rod, 0).len(), 1);
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
