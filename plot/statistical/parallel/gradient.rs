use super::common::{prepare, open, axes_grid, finalize, write_polyline, write_dots};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml};

pub const DEMO_KWARGS: &str = "axes=[\"Speed\",\"Power\",\"Range\",\"Cost\"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=[\"A\",\"B\",\"C\"]";
pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = open(cfg, &p);
    axes_grid(&mut b, cfg, &p);
    let axis = if cfg.color_axis >= 0 && (cfg.color_axis as usize) < p.n_axes {
        cfg.color_axis as usize
    } else { 0 };
    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let val = if axis < v.len() { v[axis] } else { 0.0 };
        let frac = ((val - p.mins[axis]) / (p.maxs[axis] - p.mins[axis])).clamp(0.0, 1.0);
        let col = ramp(frac);
        write_polyline(&mut b, &p, v, col, 4.0, 0.10, si, &cfg.series_names[si]);
        write_polyline(&mut b, &p, v, col, 1.8, 0.85, si, &cfg.series_names[si]);
        write_dots(&mut b, &p, v, col, 2.6, 0.9, si);
    }
    let lx = cfg.width - p.pad_r + 14;
    let ly0 = p.pad_t + 10;
    let lh = 160i32;
    push_b(&mut b, b"<text x=\""); push_i(&mut b, lx);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, ly0 - 8);
    push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"#475569\">");
    escape_xml(&mut b, &cfg.axes[axis]);
    push_b(&mut b, b"</text>");
    for k in 0..lh {
        let f = 1.0 - (k as f64 / lh as f64);
        let c = ramp(f);
        let hx = crate::plot::statistical::common::hex6(c);
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, lx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly0 + k);
        push_b(&mut b, b"\" width=\"14\" height=\"1\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
    }
    push_b(&mut b, b"<text x=\""); push_i(&mut b, lx + 18);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, ly0 + 6);
    push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
    push_f2(&mut b, p.maxs[axis]);
    push_b(&mut b, b"</text>");
    push_b(&mut b, b"<text x=\""); push_i(&mut b, lx + 18);
    push_b(&mut b, b"\" y=\""); push_i(&mut b, ly0 + lh);
    push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
    push_f2(&mut b, p.mins[axis]);
    push_b(&mut b, b"</text>");
    finalize(b, cfg)
}

fn ramp(t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let stops: [(f64, u32); 5] = [
        (0.00, 0x440154),
        (0.25, 0x3B528B),
        (0.50, 0x21918C),
        (0.75, 0x5EC962),
        (1.00, 0xFDE725),
    ];
    for w in stops.windows(2) {
        let (t0, c0) = w[0]; let (t1, c1) = w[1];
        if t <= t1 {
            let f = ((t - t0) / (t1 - t0)).clamp(0.0, 1.0);
            return lerp(c0, c1, f);
        }
    }
    stops[stops.len() - 1].1
}

fn lerp(a: u32, b: u32, t: f64) -> u32 {
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


