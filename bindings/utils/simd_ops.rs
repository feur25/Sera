#[inline(always)]
pub fn find_minmax(values: &[f64]) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    
    for &v in values.iter() {
        if v < min { min = v; }
        if v > max { max = v; }
    }
    
    (min, max)
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
