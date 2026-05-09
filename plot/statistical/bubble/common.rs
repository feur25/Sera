use super::config::BubbleConfig;
use crate::plot::statistical::common::Frame;

pub struct BubbleLayout {
    pub n: usize,
    pub xmin2: f64,
    pub xmax2: f64,
    pub ymin2: f64,
    pub ymax2: f64,
    pub xr2: f64,
    pub yr2: f64,
    pub smin: f64,
    pub smax_abs: f64,
    pub indices: Vec<usize>,
}

pub fn compute_layout(cfg: &BubbleConfig) -> Option<BubbleLayout> {
    let n = cfg.x_values.len().min(cfg.y_values.len()).min(cfg.sizes.len());
    if n == 0 { return None; }

    let mut xmin = f64::INFINITY; let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY; let mut ymax = f64::NEG_INFINITY;
    let mut smin_abs = f64::INFINITY; let mut smax_abs = f64::NEG_INFINITY;
    for i in 0..n {
        if cfg.x_values[i] < xmin { xmin = cfg.x_values[i]; }
        if cfg.x_values[i] > xmax { xmax = cfg.x_values[i]; }
        if cfg.y_values[i] < ymin { ymin = cfg.y_values[i]; }
        if cfg.y_values[i] > ymax { ymax = cfg.y_values[i]; }
        let a = cfg.sizes[i].abs();
        if a < smin_abs { smin_abs = a; }
        if a > smax_abs { smax_abs = a; }
    }
    let xr = (xmax - xmin).max(1e-9);
    let yr = (ymax - ymin).max(1e-9);
    let xpad = xr * 0.08; let ypad = yr * 0.08;
    let xmin2 = xmin - xpad; let xmax2 = xmax + xpad;
    let ymin2 = ymin - ypad; let ymax2 = ymax + ypad;

    let mut indices: Vec<usize> = (0..n).collect();
    let asc = cfg.sort_order == "asc" || cfg.sort_order == "ascending";
    if asc {
        indices.sort_by(|&a, &b| cfg.sizes[a].abs().partial_cmp(&cfg.sizes[b].abs()).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        indices.sort_by(|&a, &b| cfg.sizes[b].abs().partial_cmp(&cfg.sizes[a].abs()).unwrap_or(std::cmp::Ordering::Equal));
    }

    Some(BubbleLayout {
        n,
        xmin2, xmax2, ymin2, ymax2,
        xr2: xmax2 - xmin2, yr2: ymax2 - ymin2,
        smin: smin_abs, smax_abs,
        indices,
    })
}

pub fn radius(cfg: &BubbleConfig, layout: &BubbleLayout, i: usize) -> f64 {
    let sr = (layout.smax_abs - layout.smin).max(1e-9);
    let sn = (cfg.sizes[i].abs() - layout.smin) / sr;
    cfg.min_size + sn * (cfg.max_size - cfg.min_size)
}

pub fn point_px(layout: &BubbleLayout, frame: &Frame, x: f64, y: f64) -> (i32, i32) {
    let cx = frame.pl + (((x - layout.xmin2) / layout.xr2) * frame.pw as f64) as i32;
    let cy = frame.pt + frame.ph - (((y - layout.ymin2) / layout.yr2) * frame.ph as f64) as i32;
    (cx, cy)
}

pub fn lerp_color(a: u32, b: u32, t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let ar = ((a >> 16) & 0xFF) as f64;
    let ag = ((a >> 8) & 0xFF) as f64;
    let ab = (a & 0xFF) as f64;
    let br = ((b >> 16) & 0xFF) as f64;
    let bg = ((b >> 8) & 0xFF) as f64;
    let bb = (b & 0xFF) as f64;
    let r = (ar + (br - ar) * t).round() as u32;
    let g = (ag + (bg - ag) * t).round() as u32;
    let bl = (ab + (bb - ab) * t).round() as u32;
    (r << 16) | (g << 8) | bl
}

pub fn make_frame(cfg: &BubbleConfig, n: usize, legend_w: i32) -> Frame {
    Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 38, 52, legend_w, n * 320 + 4096)
}


