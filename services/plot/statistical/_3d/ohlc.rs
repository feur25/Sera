use super::super::bar::Bar3DBlock;
use super::budget::Buckets;

pub const UP: f64 = 1.0;
pub const DOWN: f64 = 0.0;

#[derive(Clone, Copy)]
pub struct Quotes<'a> {
    pub open: &'a [f64],
    pub high: &'a [f64],
    pub low: &'a [f64],
    pub close: &'a [f64],
}

impl<'a> Quotes<'a> {
    pub fn len(&self) -> usize {
        self.open.len().min(self.high.len()).min(self.low.len()).min(self.close.len())
    }

    pub fn is_up(&self, i: usize) -> bool {
        self.close[i] >= self.open[i]
    }

    pub fn floor(&self) -> f64 {
        self.low[..self.len()].iter().cloned().fold(f64::INFINITY, f64::min)
    }

    pub fn span(&self) -> f64 {
        let n = self.len();
        let top = self.high[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        (top - self.floor()).max(1e-9)
    }
}

pub struct Pooled {
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
}

impl Pooled {
    pub fn quotes(&self) -> Quotes<'_> {
        Quotes { open: &self.open, high: &self.high, low: &self.low, close: &self.close }
    }
}

pub fn pooled(q: &Quotes, volume: &[f64], buckets: &Buckets) -> Pooled {
    let n = q.len();
    Pooled {
        open: buckets.first(&q.open[..n]),
        high: buckets.high(&q.high[..n]),
        low: buckets.low(&q.low[..n]),
        close: buckets.last(&q.close[..n]),
        volume: if volume.is_empty() { Vec::new() } else { buckets.sum(volume) },
    }
}

pub fn direction_tone(up: bool, up_tone: f64, down_tone: f64) -> f64 {
    if up { up_tone } else { down_tone }
}

pub fn candle_blocks(q: &Quotes, body_hw: f64, wick_hw: f64, depth: f64, up_tone: f64) -> Vec<Bar3DBlock> {
    (0..q.len())
        .flat_map(|i| {
            let tone = direction_tone(q.is_up(i), up_tone, DOWN);
            let (lo, hi) = (q.open[i].min(q.close[i]), q.open[i].max(q.close[i]));
            [
                Bar3DBlock::new(i as f64, 0.0, q.low[i], q.high[i], wick_hw, depth * 0.5, i).with_tone(tone),
                Bar3DBlock::new(i as f64, 0.0, lo, hi.max(lo + q.span() * 0.004), body_hw, depth, i).with_tone(tone),
            ]
        })
        .collect()
}

pub fn stem_blocks(q: &Quotes, hw: f64, depth: f64) -> Vec<Bar3DBlock> {
    (0..q.len())
        .map(|i| {
            Bar3DBlock::new(i as f64, 0.0, q.low[i], q.high[i], hw, depth, i)
                .with_tone(direction_tone(q.is_up(i), UP, DOWN))
        })
        .collect()
}

pub fn tick_blocks(q: &Quotes, stem_hw: f64, tick_len: f64, depth: f64) -> Vec<Bar3DBlock> {
    let thick = q.span() * 0.012;
    (0..q.len())
        .flat_map(|i| {
            let tone = direction_tone(q.is_up(i), UP, DOWN);
            let reach = stem_hw + tick_len / 2.0;
            [
                Bar3DBlock::new(i as f64, 0.0, q.low[i], q.high[i], stem_hw, depth, i).with_tone(tone),
                Bar3DBlock::new(i as f64 - reach, 0.0, q.open[i] - thick, q.open[i] + thick, tick_len / 2.0, depth, i)
                    .with_tone(tone),
                Bar3DBlock::new(i as f64 + reach, 0.0, q.close[i] - thick, q.close[i] + thick, tick_len / 2.0, depth, i)
                    .with_tone(tone),
            ]
        })
        .collect()
}

pub fn track_blocks(values: &[f64], span: f64, depth: f64, row: f64, tone: f64) -> Vec<Bar3DBlock> {
    let thick = span * 0.012;
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let next = values.get(i + 1).copied().unwrap_or(v);
            Bar3DBlock::new(i as f64 + 0.5, row, v.min(next) - thick, v.max(next) + thick, 0.5, depth, i)
                .with_tone(tone)
        })
        .collect()
}

pub fn area_blocks(values: &[f64], floor: f64, depth: f64) -> Vec<Bar3DBlock> {
    let top = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (top - floor).max(1e-9);
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            Bar3DBlock::new(i as f64, 0.0, floor, v, 0.5, depth, i).with_tone(((v - floor) / range).clamp(0.0, 1.0))
        })
        .collect()
}

pub fn volume_blocks(q: &Quotes, volume: &[f64], share: f64, gap: f64, depth: f64) -> Vec<Bar3DBlock> {
    let peak = volume.iter().cloned().fold(0.0_f64, f64::max).max(1e-12);
    let floor = q.floor();
    let reach = q.span() * share;
    (0..q.len().min(volume.len()))
        .map(|i| {
            Bar3DBlock::new(i as f64, gap, floor, floor + volume[i] / peak * reach, 0.4, depth, i)
                .with_tone(direction_tone(q.is_up(i), UP, DOWN))
        })
        .collect()
}

pub fn moving_average(values: &[f64], window: usize) -> Vec<f64> {
    let w = window.max(1);
    (0..values.len())
        .map(|i| {
            let from = (i + 1).saturating_sub(w);
            let slice = &values[from..=i];
            slice.iter().sum::<f64>() / slice.len() as f64
        })
        .collect()
}

pub fn extreme_markers(q: &Quotes, size: f64) -> Vec<Bar3DBlock> {
    let n = q.len();
    if n == 0 {
        return Vec::new();
    }
    let peak = (0..n).max_by(|&a, &b| q.high[a].total_cmp(&q.high[b])).unwrap_or(0);
    let trough = (0..n).min_by(|&a, &b| q.low[a].total_cmp(&q.low[b])).unwrap_or(0);
    let lift = q.span() * 0.06;
    vec![
        Bar3DBlock::new(peak as f64, 0.0, q.high[peak] + lift, q.high[peak] + lift + size, size / 2.0, size / 2.0, peak)
            .with_tone(UP),
        Bar3DBlock::new(
            trough as f64,
            0.0,
            q.low[trough] - lift - size,
            q.low[trough] - lift,
            size / 2.0,
            size / 2.0,
            trough,
        )
        .with_tone(DOWN),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const OPEN: [f64; 4] = [10.0, 12.0, 11.0, 13.0];
    const HIGH: [f64; 4] = [14.0, 15.0, 13.0, 16.0];
    const LOW: [f64; 4] = [9.0, 10.0, 9.0, 12.0];
    const CLOSE: [f64; 4] = [12.0, 11.0, 12.0, 15.0];

    fn quotes() -> Quotes<'static> {
        Quotes { open: &OPEN, high: &HIGH, low: &LOW, close: &CLOSE }
    }

    #[test]
    fn candles_pair_a_wick_with_a_body_and_tone_by_direction() {
        let blocks = candle_blocks(&quotes(), 0.3, 0.05, 0.3, UP);
        assert_eq!(blocks.len(), 8);
        assert_eq!((blocks[0].z0, blocks[0].z1), (9.0, 14.0));
        assert_eq!((blocks[1].z0, blocks[1].z1), (10.0, 12.0));
        assert_eq!(blocks[0].tone, Some(UP));
        assert_eq!(blocks[2].tone, Some(DOWN));
    }

    #[test]
    fn ohlc_draws_a_stem_and_two_ticks_per_bar() {
        assert_eq!(tick_blocks(&quotes(), 0.05, 0.3, 0.1).len(), 12);
        assert_eq!(stem_blocks(&quotes(), 0.2, 0.2).len(), 4);
    }

    #[test]
    fn tracks_and_areas_follow_the_series() {
        let closes = quotes().close.to_vec();
        let track = track_blocks(&closes, 7.0, 0.1, 0.0, 0.5);
        assert_eq!(track.len(), 4);
        assert!(track[0].z0 < 11.0 && track[0].z1 > 12.0);
        let area = area_blocks(&closes, 9.0, 0.5);
        assert!(area.iter().all(|b| b.z0 == 9.0));
        assert_eq!(area[3].tone, Some(1.0));
    }

    #[test]
    fn volume_bars_share_the_price_floor_and_scale_to_the_peak() {
        let volume = [100.0, 50.0, 200.0, 25.0];
        let bars = volume_blocks(&quotes(), &volume, 0.5, 1.5, 0.3);
        assert_eq!(bars.len(), 4);
        assert!(bars.iter().all(|b| b.z0 == 9.0 && b.cy == 1.5));
        assert!(bars[2].z1 > bars[0].z1);
    }

    #[test]
    fn moving_average_warms_up_then_slides() {
        assert_eq!(moving_average(&[2.0, 4.0, 6.0, 8.0], 2), vec![2.0, 3.0, 5.0, 7.0]);
    }

    #[test]
    fn pooled_quotes_open_first_close_last_and_keep_the_extremes_and_the_volume() {
        let buckets = Buckets::new(4, 2);
        let volume = [1.0, 2.0, 3.0, 4.0];
        let p = pooled(&quotes(), &volume, &buckets);
        assert_eq!(p.open, vec![10.0, 11.0]);
        assert_eq!(p.close, vec![11.0, 15.0]);
        assert_eq!(p.high, vec![15.0, 16.0]);
        assert_eq!(p.low, vec![9.0, 9.0]);
        assert_eq!(p.volume, vec![3.0, 7.0]);
        assert_eq!(p.quotes().len(), 2);
    }

    #[test]
    fn extremes_mark_the_highest_high_and_the_lowest_low() {
        let markers = extreme_markers(&quotes(), 0.4);
        assert_eq!(markers.len(), 2);
        assert_eq!((markers[0].cx, markers[1].cx), (3.0, 0.0));
        assert!(markers[0].z0 > 16.0 && markers[1].z1 < 9.0);
    }
}
