use super::config::ScatterConfig;
use crate::plot::statistical::common::{push_b, push_f2, push_i, Frame};

pub struct ScatterLayout {
    pub n: usize,
    pub xmin2: f64,
    pub xmax2: f64,
    pub ymin2: f64,
    pub ymax2: f64,
    pub xr2: f64,
    pub yr2: f64,
}

pub fn compute_layout(cfg: &ScatterConfig) -> Option<ScatterLayout> {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return None;
    }
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for i in 0..n {
        if cfg.x_values[i] < xmin {
            xmin = cfg.x_values[i];
        }
        if cfg.x_values[i] > xmax {
            xmax = cfg.x_values[i];
        }
        if cfg.y_values[i] < ymin {
            ymin = cfg.y_values[i];
        }
        if cfg.y_values[i] > ymax {
            ymax = cfg.y_values[i];
        }
    }
    let xr = (xmax - xmin).max(1e-9);
    let yr = (ymax - ymin).max(1e-9);
    let xpad = xr * 0.06;
    let ypad = yr * 0.08;
    let xmin2 = xmin - xpad;
    let xmax2 = xmax + xpad;
    let ymin2 = ymin - ypad;
    let ymax2 = ymax + ypad;
    Some(ScatterLayout {
        n,
        xmin2,
        xmax2,
        ymin2,
        ymax2,
        xr2: xmax2 - xmin2,
        yr2: ymax2 - ymin2,
    })
}

pub fn point_px(layout: &ScatterLayout, frame: &Frame, x: f64, y: f64) -> (i32, i32) {
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

pub fn make_frame(cfg: &ScatterConfig, n: usize, legend_w: i32) -> Frame {
    Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        38,
        52,
        legend_w,
        n * 220 + 4096,
    )
}

pub fn draw_marker(
    buf: &mut Vec<u8>,
    sym: &str,
    cx: i32,
    cy: i32,
    r: f64,
    fill_hex: &[u8; 6],
    stroke_hex: &[u8; 6],
    stroke_w: f64,
    fill_opacity: f64,
) {
    let rr = r.max(1.0);
    match sym {
        "square" => {
            let s = (rr * 1.7) as i32;
            let x = cx - s / 2;
            let y = cy - s / 2;
            push_b(buf, b"<rect x=\"");
            push_i(buf, x);
            push_b(buf, b"\" y=\"");
            push_i(buf, y);
            push_b(buf, b"\" width=\"");
            push_i(buf, s);
            push_b(buf, b"\" height=\"");
            push_i(buf, s);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(fill_hex);
            push_b(buf, b"\" fill-opacity=\"");
            push_f2(buf, fill_opacity);
            push_b(buf, b"\" stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w);
            push_b(buf, b"\"/>");
        }
        "diamond" => {
            let s = (rr * 1.25) as i32;
            push_b(buf, b"<polygon points=\"");
            push_i(buf, cx);
            buf.push(b',');
            push_i(buf, cy - s);
            buf.push(b' ');
            push_i(buf, cx + s);
            buf.push(b',');
            push_i(buf, cy);
            buf.push(b' ');
            push_i(buf, cx);
            buf.push(b',');
            push_i(buf, cy + s);
            buf.push(b' ');
            push_i(buf, cx - s);
            buf.push(b',');
            push_i(buf, cy);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(fill_hex);
            push_b(buf, b"\" fill-opacity=\"");
            push_f2(buf, fill_opacity);
            push_b(buf, b"\" stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w);
            push_b(buf, b"\"/>");
        }
        "triangle" | "triangle-up" => {
            let s = (rr * 1.25) as i32;
            push_b(buf, b"<polygon points=\"");
            push_i(buf, cx);
            buf.push(b',');
            push_i(buf, cy - s);
            buf.push(b' ');
            push_i(buf, cx + s);
            buf.push(b',');
            push_i(buf, cy + s);
            buf.push(b' ');
            push_i(buf, cx - s);
            buf.push(b',');
            push_i(buf, cy + s);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(fill_hex);
            push_b(buf, b"\" fill-opacity=\"");
            push_f2(buf, fill_opacity);
            push_b(buf, b"\" stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w);
            push_b(buf, b"\"/>");
        }
        "triangle-down" => {
            let s = (rr * 1.25) as i32;
            push_b(buf, b"<polygon points=\"");
            push_i(buf, cx - s);
            buf.push(b',');
            push_i(buf, cy - s);
            buf.push(b' ');
            push_i(buf, cx + s);
            buf.push(b',');
            push_i(buf, cy - s);
            buf.push(b' ');
            push_i(buf, cx);
            buf.push(b',');
            push_i(buf, cy + s);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(fill_hex);
            push_b(buf, b"\" fill-opacity=\"");
            push_f2(buf, fill_opacity);
            push_b(buf, b"\" stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w);
            push_b(buf, b"\"/>");
        }
        "cross" | "x" => {
            let s = (rr * 1.1) as i32;
            push_b(buf, b"<g stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w.max(1.5));
            push_b(buf, b"\" stroke-linecap=\"round\"><line x1=\"");
            push_i(buf, cx - s);
            push_b(buf, b"\" y1=\"");
            push_i(buf, cy - s);
            push_b(buf, b"\" x2=\"");
            push_i(buf, cx + s);
            push_b(buf, b"\" y2=\"");
            push_i(buf, cy + s);
            push_b(buf, b"\"/><line x1=\"");
            push_i(buf, cx - s);
            push_b(buf, b"\" y1=\"");
            push_i(buf, cy + s);
            push_b(buf, b"\" x2=\"");
            push_i(buf, cx + s);
            push_b(buf, b"\" y2=\"");
            push_i(buf, cy - s);
            push_b(buf, b"\"/></g>");
        }
        "plus" => {
            let s = (rr * 1.2) as i32;
            push_b(buf, b"<g stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w.max(1.5));
            push_b(buf, b"\" stroke-linecap=\"round\"><line x1=\"");
            push_i(buf, cx - s);
            push_b(buf, b"\" y1=\"");
            push_i(buf, cy);
            push_b(buf, b"\" x2=\"");
            push_i(buf, cx + s);
            push_b(buf, b"\" y2=\"");
            push_i(buf, cy);
            push_b(buf, b"\"/><line x1=\"");
            push_i(buf, cx);
            push_b(buf, b"\" y1=\"");
            push_i(buf, cy - s);
            push_b(buf, b"\" x2=\"");
            push_i(buf, cx);
            push_b(buf, b"\" y2=\"");
            push_i(buf, cy + s);
            push_b(buf, b"\"/></g>");
        }
        "star" => {
            let s = rr * 1.3;
            let inner = s * 0.45;
            push_b(buf, b"<polygon points=\"");
            for k in 0..10 {
                let ang = -std::f64::consts::FRAC_PI_2 + (k as f64) * std::f64::consts::PI / 5.0;
                let rad = if k % 2 == 0 { s } else { inner };
                let px = cx + (ang.cos() * rad) as i32;
                let py = cy + (ang.sin() * rad) as i32;
                if k > 0 {
                    buf.push(b' ');
                }
                push_i(buf, px);
                buf.push(b',');
                push_i(buf, py);
            }
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(fill_hex);
            push_b(buf, b"\" fill-opacity=\"");
            push_f2(buf, fill_opacity);
            push_b(buf, b"\" stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w);
            push_b(buf, b"\"/>");
        }
        _ => {
            push_b(buf, b"<circle cx=\"");
            push_i(buf, cx);
            push_b(buf, b"\" cy=\"");
            push_i(buf, cy);
            push_b(buf, b"\" r=\"");
            push_f2(buf, rr);
            push_b(buf, b"\" fill=\"#");
            buf.extend_from_slice(fill_hex);
            push_b(buf, b"\" fill-opacity=\"");
            push_f2(buf, fill_opacity);
            push_b(buf, b"\" stroke=\"#");
            buf.extend_from_slice(stroke_hex);
            push_b(buf, b"\" stroke-width=\"");
            push_f2(buf, stroke_w);
            push_b(buf, b"\"/>");
        }
    }
}

pub fn cycle_symbol(i: usize) -> &'static str {
    const SYMS: &[&str] = &[
        "circle",
        "square",
        "diamond",
        "triangle",
        "cross",
        "plus",
        "star",
        "triangle-down",
    ];
    SYMS[i % SYMS.len()]
}
