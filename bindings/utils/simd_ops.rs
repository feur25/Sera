const PAR_MINMAX_THRESHOLD: usize = 50_000;

#[inline]
fn scan_minmax(values: &[f64]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for &v in values.iter() {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }

    (min, max)
}

pub fn find_minmax(values: &[f64]) -> (f64, f64) {
    if values.len() < PAR_MINMAX_THRESHOLD {
        return scan_minmax(values);
    }
    use rayon::prelude::*;
    let chunk = (values.len() / rayon::current_num_threads().max(1)).max(PAR_MINMAX_THRESHOLD / 4);
    values
        .par_chunks(chunk)
        .map(scan_minmax)
        .reduce(
            || (f64::INFINITY, f64::NEG_INFINITY),
            |(min1, max1), (min2, max2)| (min1.min(min2), max1.max(max2)),
        )
}

pub fn compute_hex_colors_batch_into(len: usize, out: &mut Vec<u32>) {
    out.clear();
    out.reserve(len);
    for i in 0..len {
        let hue = (i as u32 * 360) / len as u32;
        let r = ((100 + (hue / 2) % 156) & 0xFF) << 16;
        let g = ((100 + (hue / 4) % 156) & 0xFF) << 8;
        let b = (200 + (hue % 55)) & 0xFF;
        out.push(r | g | b);
    }
}
