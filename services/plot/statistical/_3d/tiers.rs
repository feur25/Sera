use super::super::bar::Bar3DBlock;

pub const HALF: f64 = 5.0;
pub const STAGE_T: f64 = 3.0;
pub const GAP: f64 = 0.3;
const CENTRE: f64 = HALF;
const LAYERS: usize = 4;
const BARREL: [f64; LAYERS] = [0.86, 1.0, 1.0, 0.86];
const TIP: f64 = 0.45;
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

fn spot(slot: usize, count: usize, reverse: bool) -> f64 {
    let row = if reverse { count - 1 - slot } else { slot };
    row as f64 * (STAGE_T + GAP)
}

fn painted(block: Bar3DBlock, tone: Option<f64>) -> Bar3DBlock {
    match tone {
        Some(t) => block.with_tone(t),
        None => block,
    }
}

fn section(width: f64) -> (f64, f64) {
    let s = width.max(0.0) * HALF;
    (CENTRE - s, CENTRE + s)
}

fn slab(tier: &Tier, x: f64, length: f64, width: f64, lane: f64) -> Bar3DBlock {
    let (z0, z1) = section(width);
    painted(Bar3DBlock::new(x + length / 2.0, lane, z0, z1, length / 2.0, width.max(0.0) * HALF, tier.class), tier.tone)
}

fn layers(tier: &Tier, x: f64, reverse: bool, lane: f64, width_at: impl Fn(usize) -> f64) -> Vec<Bar3DBlock> {
    let each = STAGE_T / LAYERS as f64;
    (0..LAYERS)
        .map(|j| {
            let along = if reverse { LAYERS - 1 - j } else { j };
            slab(tier, x + each * along as f64, each, width_at(j), lane)
        })
        .collect()
}

fn chevron(tier: &Tier, x: f64, reverse: bool, lane: f64) -> Vec<Bar3DBlock> {
    let s = tier.width.max(0.0) * HALF;
    let (full, point) = ((CENTRE - s, CENTRE + s), (CENTRE - s * TIP, CENTRE + s * TIP));
    let (start, end) = if reverse { (point, full) } else { (full, point) };
    vec![painted(Bar3DBlock::sloped(x + STAGE_T / 2.0, lane, start, end, STAGE_T / 2.0, s, tier.class), tier.tone)]
}

pub fn stack(tiers: &[Tier], profile: Profile, reverse: bool, lane: f64) -> Vec<Bar3DBlock> {
    let count = tiers.len();
    tiers
        .iter()
        .enumerate()
        .flat_map(|(k, tier)| {
            let x = spot(k, count, reverse);
            let next = tiers.get(k + 1).map(|t| t.width).unwrap_or(tier.width * TAIL);
            match profile {
                Profile::Flat => vec![slab(tier, x, STAGE_T, tier.width, lane)],
                Profile::Barrel => layers(tier, x, reverse, lane, |j| BARREL[j] * tier.width),
                Profile::Taper => layers(tier, x, reverse, lane, |j| {
                    tier.width + (next - tier.width) * (j as f64 + 0.5) / LAYERS as f64
                }),
                Profile::Chevron => chevron(tier, x, reverse, lane),
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
            let x = spot(k, count, reverse);
            let total = parts.iter().map(|t| t.width.max(0.0)).sum::<f64>();
            let (z0, z1) = section(total);
            let mut cursor = -total * HALF;
            parts
                .iter()
                .map(|tier| {
                    let length = 2.0 * HALF * tier.width.max(0.0);
                    let block = Bar3DBlock::new(x + STAGE_T / 2.0, cursor + length / 2.0, z0, z1, STAGE_T / 2.0, length / 2.0, tier.class);
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
    fn flat_stages_follow_the_flow_axis_with_a_square_section_proportional_to_their_value() {
        let blocks = stack(&tiers(&[1.0, 0.5, 0.25]), Profile::Flat, false, 0.0);
        assert_eq!(blocks.len(), 3);
        assert_eq!((blocks[0].hd, blocks[1].hd, blocks[2].hd), (HALF, HALF / 2.0, HALF / 4.0));
        assert!(blocks[0].cx < blocks[1].cx && blocks[1].cx < blocks[2].cx);
        assert!((blocks[1].cx - blocks[0].cx - (STAGE_T + GAP)).abs() < 1e-9);
        for block in &blocks {
            assert!(((block.z0 + block.z1) / 2.0 - CENTRE).abs() < 1e-9);
            assert!(((block.z1 - block.z0) / 2.0 - block.hd).abs() < 1e-9);
        }
    }

    #[test]
    fn reversing_puts_the_first_stage_at_the_far_end_of_the_flow() {
        let blocks = stack(&tiers(&[1.0, 0.5]), Profile::Flat, true, 0.0);
        assert!(blocks[0].cx > blocks[1].cx);
    }

    #[test]
    fn tapering_stages_narrow_layer_by_layer_towards_the_next_stage() {
        let blocks = stack(&tiers(&[1.0, 0.4]), Profile::Taper, false, 0.0);
        assert_eq!(blocks.len(), 8);
        let first: Vec<f64> = blocks[..4].iter().map(|b| b.hd).collect();
        assert!(first.windows(2).all(|w| w[0] > w[1]));
        assert!(first[0] < HALF && first[3] > 0.4 * HALF);
        assert!(blocks[0].cx < blocks[3].cx);
    }

    #[test]
    fn the_last_stage_tapers_to_a_fraction_of_itself() {
        let blocks = stack(&tiers(&[1.0]), Profile::Taper, false, 0.0);
        assert!(blocks[3].hd < blocks[0].hd && blocks[3].hd > 0.0);
    }

    #[test]
    fn barrels_bulge_in_the_middle_layers() {
        let blocks = stack(&tiers(&[1.0]), Profile::Barrel, false, 0.0);
        let widths: Vec<f64> = blocks.iter().map(|b| b.hd).collect();
        assert_eq!(widths, vec![0.86 * HALF, HALF, HALF, 0.86 * HALF]);
    }

    #[test]
    fn a_chevron_is_one_wedge_narrowing_towards_the_flow_and_mirrored_when_reversed() {
        let ahead = stack(&tiers(&[1.0]), Profile::Chevron, false, 0.0)[0];
        let (start, end) = ((ahead.z0, ahead.z1), ahead.end.unwrap());
        assert!(end.1 - end.0 < start.1 - start.0);
        let back = stack(&tiers(&[1.0]), Profile::Chevron, true, 0.0)[0];
        assert!(back.end.unwrap().1 - back.end.unwrap().0 > back.z1 - back.z0);
    }

    #[test]
    fn stacks_can_be_moved_sideways_and_carry_class_and_tone() {
        let mut one = tiers(&[0.5]);
        one[0].tone = Some(0.3);
        let blocks = stack(&one, Profile::Flat, false, 7.0);
        assert_eq!((blocks[0].cy, blocks[0].ci, blocks[0].tone), (7.0, 0, Some(0.3)));
    }

    #[test]
    fn split_stages_lay_their_parts_side_by_side_across_the_full_section() {
        let stage = vec![Tier { width: 0.3, class: 0, tone: None }, Tier { width: 0.2, class: 1, tone: Some(0.9) }];
        let blocks = split(&[stage], false);
        assert_eq!(blocks.len(), 2);
        let total = 2.0 * HALF * 0.5;
        assert!((blocks[0].cy - blocks[0].hd + total / 2.0).abs() < 1e-9);
        assert!((blocks[1].cy + blocks[1].hd - total / 2.0).abs() < 1e-9);
        assert_eq!((blocks[0].z1 - blocks[0].z0, blocks[1].tone), (total, Some(0.9)));
    }

    #[test]
    fn no_stages_draw_nothing() {
        assert!(stack(&[], Profile::Taper, false, 0.0).is_empty());
        assert!(split(&[], false).is_empty());
    }
}
