use super::super::bar::Bar3DBlock;

pub const HALF: f64 = 5.0;
pub const STAGE_H: f64 = 1.6;
pub const GAP: f64 = 0.15;
const LAYERS: usize = 4;
const BARREL: [f64; LAYERS] = [0.86, 1.0, 1.0, 0.86];
const DIP: f64 = 0.55;
const TAIL: f64 = 0.8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Profile {
    Flat,
    Taper,
    Barrel,
    Chevron,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tier {
    pub width: f64,
    pub class: usize,
    pub tone: Option<f64>,
}

fn level(slot: usize, count: usize, reverse: bool) -> f64 {
    let row = if reverse { slot } else { count - 1 - slot };
    row as f64 * (STAGE_H + GAP)
}

fn painted(block: Bar3DBlock, tone: Option<f64>) -> Bar3DBlock {
    match tone {
        Some(t) => block.with_tone(t),
        None => block,
    }
}

fn slab(tier: &Tier, floor: f64, origin: (f64, f64)) -> Bar3DBlock {
    let w = tier.width.max(0.0) * HALF;
    painted(Bar3DBlock::new(origin.0, origin.1, floor, floor + STAGE_H, w, w, tier.class), tier.tone)
}

fn layers(tier: &Tier, floor: f64, reverse: bool, origin: (f64, f64), width_at: impl Fn(usize) -> f64) -> Vec<Bar3DBlock> {
    let each = STAGE_H / LAYERS as f64;
    (0..LAYERS)
        .map(|j| {
            let z0 = floor + each * (if reverse { j } else { LAYERS - 1 - j }) as f64;
            let w = width_at(j).max(0.0) * HALF;
            painted(Bar3DBlock::new(origin.0, origin.1, z0, z0 + each, w, w, tier.class), tier.tone)
        })
        .collect()
}

fn chevron(tier: &Tier, floor: f64, reverse: bool, origin: (f64, f64)) -> Vec<Bar3DBlock> {
    let w = tier.width.max(0.0) * HALF;
    let shift = if reverse { STAGE_H * DIP } else { -STAGE_H * DIP };
    let (edge, centre) = ((floor, floor + STAGE_H), (floor + shift, floor + STAGE_H + shift));
    vec![
        painted(Bar3DBlock::sloped(origin.0 - w / 2.0, origin.1, edge, centre, w / 2.0, w, tier.class), tier.tone),
        painted(Bar3DBlock::sloped(origin.0 + w / 2.0, origin.1, centre, edge, w / 2.0, w, tier.class), tier.tone),
    ]
}

pub fn stack(tiers: &[Tier], profile: Profile, reverse: bool, origin: (f64, f64)) -> Vec<Bar3DBlock> {
    let count = tiers.len();
    tiers
        .iter()
        .enumerate()
        .flat_map(|(k, tier)| {
            let floor = level(k, count, reverse);
            let next = tiers.get(k + 1).map(|t| t.width).unwrap_or(tier.width * TAIL);
            match profile {
                Profile::Flat => vec![slab(tier, floor, origin)],
                Profile::Barrel => layers(tier, floor, reverse, origin, |j| BARREL[j] * tier.width),
                Profile::Taper => layers(tier, floor, reverse, origin, |j| {
                    tier.width + (next - tier.width) * (j as f64 + 0.5) / LAYERS as f64
                }),
                Profile::Chevron => chevron(tier, floor, reverse, origin),
            }
        })
        .collect()
}

pub fn split(stages: &[Vec<Tier>], reverse: bool) -> Vec<Bar3DBlock> {
    let count = stages.len();
    stages
        .iter()
        .enumerate()
        .flat_map(|(k, parts)| {
            let floor = level(k, count, reverse);
            let full = 2.0 * HALF * parts.iter().map(|t| t.width.max(0.0)).sum::<f64>();
            let mut cursor = -full / 2.0;
            parts
                .iter()
                .map(|tier| {
                    let length = 2.0 * HALF * tier.width.max(0.0);
                    let block = Bar3DBlock::new(cursor + length / 2.0, 0.0, floor, floor + STAGE_H, length / 2.0, full / 2.0, tier.class);
                    cursor += length;
                    painted(block, tier.tone)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiers(widths: &[f64]) -> Vec<Tier> {
        widths.iter().enumerate().map(|(class, &width)| Tier { width, class, tone: None }).collect()
    }

    #[test]
    fn flat_stages_stack_from_the_top_with_widths_proportional_to_their_value() {
        let blocks = stack(&tiers(&[1.0, 0.5, 0.25]), Profile::Flat, false, (0.0, 0.0));
        assert_eq!(blocks.len(), 3);
        assert_eq!((blocks[0].hw, blocks[1].hw, blocks[2].hw), (HALF, HALF / 2.0, HALF / 4.0));
        assert!(blocks[0].z0 > blocks[1].z0 && blocks[1].z0 > blocks[2].z0);
        assert!((blocks[0].z0 - blocks[1].z1 - GAP).abs() < 1e-9);
        assert_eq!(blocks[0].hw, blocks[0].hd);
    }

    #[test]
    fn reversing_puts_the_first_stage_at_the_bottom() {
        let blocks = stack(&tiers(&[1.0, 0.5]), Profile::Flat, true, (0.0, 0.0));
        assert!(blocks[0].z0 < blocks[1].z0);
    }

    #[test]
    fn tapering_stages_narrow_layer_by_layer_towards_the_next_stage() {
        let blocks = stack(&tiers(&[1.0, 0.4]), Profile::Taper, false, (0.0, 0.0));
        assert_eq!(blocks.len(), 8);
        let first: Vec<f64> = blocks[..4].iter().map(|b| b.hw).collect();
        assert!(first.windows(2).all(|w| w[0] > w[1]));
        assert!(first[0] < HALF && first[3] > 0.4 * HALF);
        assert!(blocks[0].z0 > blocks[3].z0);
    }

    #[test]
    fn the_last_stage_tapers_to_a_fraction_of_itself() {
        let blocks = stack(&tiers(&[1.0]), Profile::Taper, false, (0.0, 0.0));
        assert!(blocks[3].hw < blocks[0].hw && blocks[3].hw > 0.0);
    }

    #[test]
    fn barrels_bulge_in_the_middle_layers() {
        let blocks = stack(&tiers(&[1.0]), Profile::Barrel, false, (0.0, 0.0));
        let widths: Vec<f64> = blocks.iter().map(|b| b.hw).collect();
        assert_eq!(widths, vec![0.86 * HALF, HALF, HALF, 0.86 * HALF]);
    }

    #[test]
    fn a_chevron_is_two_wedges_dipping_towards_the_next_stage() {
        let blocks = stack(&tiers(&[1.0]), Profile::Chevron, false, (0.0, 0.0));
        assert_eq!(blocks.len(), 2);
        let (left, right) = (blocks[0], blocks[1]);
        assert_eq!(left.end, Some((right.z0, right.z1)));
        assert_eq!(right.end, Some((left.z0, left.z1)));
        assert!(right.z0 < left.z0);
        let up = stack(&tiers(&[1.0]), Profile::Chevron, true, (0.0, 0.0));
        assert!(up[1].z0 > up[0].z0);
    }

    #[test]
    fn stacks_can_be_moved_off_the_axis_and_carry_class_and_tone() {
        let mut one = tiers(&[0.5]);
        one[0].tone = Some(0.3);
        let blocks = stack(&one, Profile::Flat, false, (4.0, 7.0));
        assert_eq!((blocks[0].cx, blocks[0].cy, blocks[0].ci, blocks[0].tone), (4.0, 7.0, 0, Some(0.3)));
    }

    #[test]
    fn split_stages_lay_their_parts_side_by_side_across_the_full_width() {
        let stage = vec![Tier { width: 0.3, class: 0, tone: None }, Tier { width: 0.2, class: 1, tone: Some(0.9) }];
        let blocks = split(&[stage], false);
        assert_eq!(blocks.len(), 2);
        let total = 2.0 * HALF * 0.5;
        assert!((blocks[0].cx - blocks[0].hw + total / 2.0).abs() < 1e-9);
        assert!((blocks[1].cx + blocks[1].hw - total / 2.0).abs() < 1e-9);
        assert_eq!((blocks[0].hd, blocks[1].tone), (total / 2.0, Some(0.9)));
    }

    #[test]
    fn no_stages_draw_nothing() {
        assert!(stack(&[], Profile::Taper, false, (0.0, 0.0)).is_empty());
        assert!(split(&[], false).is_empty());
    }
}
