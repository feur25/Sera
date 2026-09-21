use super::super::bar::Bar3DBlock;
use super::zone::cube_height;

pub const SCALE: f64 = 100.0;
pub const PARTS: usize = 4;

const BACK_Y: f64 = 0.2;
const FRONT_Y: f64 = -0.2;
const BAND_HW: f64 = 0.44;
const RAIL_HW: f64 = 0.12;
const BAND_HD: f64 = 0.16;
const BAR_HW: f64 = 0.2;
const WIDE_HW: f64 = 0.34;
const TUBE_HW: f64 = 0.09;
const PAIR_HW: f64 = 0.13;
const PAIR_SHIFT: f64 = 0.19;
const MARK_HW: f64 = 0.47;
const MARK_HD: f64 = 0.36;
const MARK_THICK: f64 = SCALE * 0.014;
const FLOOR: f64 = SCALE * 0.004;
const BEAD_HW: f64 = 0.24;
const BULB_HW: f64 = 0.22;

pub mod tone {
    pub const RED: f64 = 0.0;
    pub const AMBER: f64 = 0.1;
    pub const GREEN: f64 = 0.2;
    pub const SLATE_A: f64 = 0.3;
    pub const SLATE_B: f64 = 0.4;
    pub const SLATE_C: f64 = 0.5;
    pub const TRACK: f64 = 0.6;
    pub const VALUE: f64 = 0.7;
    pub const GHOST: f64 = 0.8;
    pub const TARGET: f64 = 0.9;
    pub const TUBE: f64 = 1.0;
}

const ZONES: [(f64, f64); 3] = [(0.40, tone::SLATE_A), (0.75, tone::SLATE_B), (1.0, tone::SLATE_C)];
const SIGNAL: [(f64, f64); 3] = [(0.40, tone::RED), (0.75, tone::AMBER), (1.0, tone::GREEN)];
const DEFAULT_RANGE: f64 = 0.75;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    Band,
    Value,
    Target,
    Prior,
}

impl Part {
    pub fn class(self, row: usize) -> usize {
        row * PARTS + self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bands {
    None,
    Track,
    Rail,
    Range,
    Zones,
    Signal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bar {
    Column,
    Wide,
    Tube,
    Dot,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plan {
    pub bands: Bands,
    pub bar: Bar,
    pub target: bool,
    pub ghost: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gauge {
    pub value: f64,
    pub target: f64,
    pub prior: f64,
    pub max: f64,
    pub range: f64,
}

fn unit(value: f64, max: f64) -> f64 {
    (value / max.max(1e-12)).clamp(0.0, 1.0)
}

fn stops(bands: Bands, gauge: &Gauge) -> Vec<(f64, f64)> {
    match bands {
        Bands::None => Vec::new(),
        Bands::Track | Bands::Rail => vec![(1.0, tone::TRACK)],
        Bands::Range => {
            let cut = if gauge.range > 0.0 { unit(gauge.range, gauge.max) } else { DEFAULT_RANGE };
            vec![(cut, tone::SLATE_A), (1.0, tone::TRACK)]
        }
        Bands::Zones => ZONES.to_vec(),
        Bands::Signal => SIGNAL.to_vec(),
    }
}

fn backdrop(gauge: &Gauge, row: usize, plan: &Plan) -> Vec<Bar3DBlock> {
    let hw = if plan.bands == Bands::Rail { RAIL_HW } else { BAND_HW };
    let mut from = 0.0;
    stops(plan.bands, gauge)
        .into_iter()
        .filter_map(|(to, tone)| {
            let block = (to > from).then(|| {
                Bar3DBlock::new(row as f64, BACK_Y, from * SCALE, to * SCALE, hw, BAND_HD, Part::Band.class(row)).with_tone(tone)
            });
            from = to.max(from);
            block
        })
        .collect()
}

fn front_y(plan: &Plan) -> f64 {
    if plan.bands == Bands::None { 0.0 } else { FRONT_Y }
}

fn columns(gauge: &Gauge, row: usize, plan: &Plan) -> Vec<Bar3DBlock> {
    let y = front_y(plan);
    let column = |cx: f64, hw: f64, top: f64, part: Part, tone: f64| {
        Bar3DBlock::new(cx, y, 0.0, top.max(FLOOR), hw, BAND_HD, part.class(row)).with_tone(tone)
    };
    let value = unit(gauge.value, gauge.max) * SCALE;
    let at = row as f64;
    match (plan.bar, plan.ghost) {
        (Bar::Column, true) => vec![
            column(at - PAIR_SHIFT, PAIR_HW, value, Part::Value, tone::VALUE),
            column(at + PAIR_SHIFT, PAIR_HW, unit(gauge.prior, gauge.max) * SCALE, Part::Prior, tone::GHOST),
        ],
        (Bar::Column, false) => vec![column(at, BAR_HW, value, Part::Value, tone::VALUE)],
        (Bar::Wide, _) => vec![column(at, WIDE_HW, value, Part::Value, tone::VALUE)],
        (Bar::Tube, _) => vec![column(at, TUBE_HW, value, Part::Value, tone::TUBE)],
        (Bar::Dot, _) => Vec::new(),
    }
}

fn mark(gauge: &Gauge, row: usize) -> Option<Bar3DBlock> {
    (gauge.target > 0.0).then(|| {
        let z = unit(gauge.target, gauge.max) * SCALE;
        Bar3DBlock::new(row as f64, 0.0, z - MARK_THICK / 2.0, z + MARK_THICK / 2.0, MARK_HW, MARK_HD, Part::Target.class(row))
            .with_tone(tone::TARGET)
    })
}

fn beads(gauges: &[Gauge], plan: &Plan, frame: &[Bar3DBlock], height_ratio: f64) -> Vec<Bar3DBlock> {
    let hw = if plan.bar == Bar::Dot { BEAD_HW } else { BULB_HW };
    let tall = cube_height(frame, hw * 2.0, height_ratio);
    let y = front_y(plan);
    gauges
        .iter()
        .enumerate()
        .filter_map(|(row, gauge)| match plan.bar {
            Bar::Dot => {
                let z = unit(gauge.value, gauge.max) * SCALE;
                Some(Bar3DBlock::new(row as f64, y, z - tall / 2.0, z + tall / 2.0, hw, hw, Part::Value.class(row)).with_tone(tone::VALUE))
            }
            Bar::Tube => Some(Bar3DBlock::new(row as f64, y, 0.0, tall, hw, hw, Part::Value.class(row)).with_tone(tone::TUBE)),
            _ => None,
        })
        .collect()
}

pub fn bullets(gauges: &[Gauge], plan: Plan, height_ratio: f64) -> Vec<Bar3DBlock> {
    let mut blocks: Vec<Bar3DBlock> = Vec::new();
    for (row, gauge) in gauges.iter().enumerate() {
        blocks.extend(backdrop(gauge, row, &plan));
        blocks.extend(columns(gauge, row, &plan));
        if plan.target {
            blocks.extend(mark(gauge, row));
        }
    }
    if matches!(plan.bar, Bar::Dot | Bar::Tube) {
        let extra = beads(gauges, &plan, &blocks, height_ratio);
        blocks.extend(extra);
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gauge(value: f64, max: f64) -> Gauge {
        Gauge { value, target: 0.0, prior: 0.0, max, range: 0.0 }
    }

    fn plan(bands: Bands, bar: Bar) -> Plan {
        Plan { bands, bar, target: false, ghost: false }
    }

    #[test]
    fn zone_bands_partition_the_scale_without_gaps_and_carry_their_tones() {
        let blocks = backdrop(&gauge(50.0, 100.0), 0, &plan(Bands::Signal, Bar::Column));
        let spans: Vec<(f64, f64)> = blocks.iter().map(|b| (b.z0, b.z1)).collect();
        assert_eq!(spans, vec![(0.0, 40.0), (40.0, 75.0), (75.0, 100.0)]);
        assert_eq!(blocks.iter().map(|b| b.tone).collect::<Vec<_>>(), vec![Some(tone::RED), Some(tone::AMBER), Some(tone::GREEN)]);
    }

    #[test]
    fn a_range_band_cuts_at_the_given_range_or_three_quarters_by_default() {
        let given = Gauge { range: 60.0, ..gauge(50.0, 100.0) };
        assert_eq!(backdrop(&given, 0, &plan(Bands::Range, Bar::Column))[0].z1, 60.0);
        assert_eq!(backdrop(&gauge(50.0, 100.0), 0, &plan(Bands::Range, Bar::Column))[0].z1, 75.0);
        assert_eq!(backdrop(&gauge(50.0, 100.0), 0, &plan(Bands::None, Bar::Column)).len(), 0);
    }

    #[test]
    fn every_row_is_scaled_to_a_percentage_of_its_own_maximum() {
        let blocks = bullets(&[gauge(60.0, 120.0), gauge(2.5, 5.0)], plan(Bands::None, Bar::Column), 0.8);
        assert_eq!(blocks.len(), 2);
        assert_eq!((blocks[0].z1, blocks[1].z1), (50.0, 50.0));
    }

    #[test]
    fn values_beyond_the_maximum_are_clamped_and_zero_keeps_a_visible_floor() {
        let blocks = bullets(&[gauge(500.0, 100.0), gauge(0.0, 100.0)], plan(Bands::None, Bar::Column), 0.8);
        assert_eq!(blocks[0].z1, SCALE);
        assert!(blocks[1].z1 > 0.0);
    }

    #[test]
    fn the_target_is_a_plate_at_its_level_and_absent_without_a_target() {
        let with = Gauge { target: 90.0, ..gauge(50.0, 100.0) };
        let marked = bullets(&[with], Plan { target: true, ..plan(Bands::None, Bar::Column) }, 0.8);
        let plate = marked[1];
        assert!(((plate.z0 + plate.z1) / 2.0 - 90.0).abs() < 1e-9);
        assert_eq!((plate.ci, plate.tone), (Part::Target.class(0), Some(tone::TARGET)));
        assert_eq!(bullets(&[gauge(50.0, 100.0)], Plan { target: true, ..plan(Bands::None, Bar::Column) }, 0.8).len(), 1);
    }

    #[test]
    fn the_ghost_stands_beside_the_value_in_its_own_tone() {
        let both = Gauge { prior: 70.0, ..gauge(80.0, 100.0) };
        let blocks = bullets(&[both], Plan { ghost: true, ..plan(Bands::Track, Bar::Column) }, 0.8);
        let (value, ghost) = (blocks[1], blocks[2]);
        assert!(value.cx < 0.0 && ghost.cx > 0.0);
        assert_eq!((value.z1, ghost.z1, ghost.tone), (80.0, 70.0, Some(tone::GHOST)));
    }

    #[test]
    fn a_dot_replaces_the_bar_by_a_bead_at_the_value_and_a_tube_gets_a_bulb_at_its_base() {
        let dot = bullets(&[gauge(60.0, 100.0)], plan(Bands::Track, Bar::Dot), 0.8);
        assert_eq!(dot.len(), 2);
        assert!(((dot[1].z0 + dot[1].z1) / 2.0 - 60.0).abs() < 1e-9);
        let tube = bullets(&[gauge(60.0, 100.0)], plan(Bands::Rail, Bar::Tube), 0.8);
        assert_eq!(tube.len(), 3);
        assert_eq!(tube[2].z0, 0.0);
        assert!(tube[2].hw > tube[1].hw);
    }

    #[test]
    fn the_track_is_a_back_layer_and_the_bar_a_front_layer_unless_there_is_no_track() {
        let layered = bullets(&[gauge(60.0, 100.0)], plan(Bands::Track, Bar::Column), 0.8);
        assert!(layered[0].cy > layered[1].cy);
        assert_eq!(bullets(&[gauge(60.0, 100.0)], plan(Bands::None, Bar::Column), 0.8)[0].cy, 0.0);
    }

    #[test]
    fn classes_number_four_parts_per_row() {
        assert_eq!((Part::Band.class(2), Part::Prior.class(2)), (8, 11));
    }
}
