use super::config::HexbinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};
use crate::plot::statistical::common::colorscale_color;
use std::collections::HashMap;

pub struct Bounds {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

pub fn data_bounds(cfg: &HexbinConfig) -> Option<Bounds> {
    data_bounds_raw(cfg.x_values, cfg.y_values)
}

pub fn data_bounds_raw(x_values: &[f64], y_values: &[f64]) -> Option<Bounds> {
    let n = x_values.len().min(y_values.len());
    if n == 0 {
        return None;
    }
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for i in 0..n {
        let x = x_values[i];
        let y = y_values[i];
        if x < xmin {
            xmin = x;
        }
        if x > xmax {
            xmax = x;
        }
        if y < ymin {
            ymin = y;
        }
        if y > ymax {
            ymax = y;
        }
    }
    let xpad = (xmax - xmin).max(1e-9) * 0.05;
    let ypad = (ymax - ymin).max(1e-9) * 0.05;
    Some(Bounds {
        xmin: xmin - xpad,
        xmax: xmax + xpad,
        ymin: ymin - ypad,
        ymax: ymax + ypad,
    })
}

pub fn make_frame(cfg: &HexbinConfig) -> Frame {
    Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 38, 52, 92, cfg.x_values.len() * 48 + 8192)
}

pub struct Bin {
    pub cx: f64,
    pub cy: f64,
    pub count: u32,
}

pub struct Prepared {
    pub bins: Vec<Bin>,
    pub r: f64,
    pub min_count: u32,
    pub max_count: u32,
}

pub fn prepare(cfg: &HexbinConfig, frame: &Frame, bounds: &Bounds) -> Option<Prepared> {
    prepare_raw(cfg.x_values, cfg.y_values, cfg.gridsize, frame.pl, frame.pt, frame.pw, frame.ph, bounds)
}

#[allow(clippy::too_many_arguments)]
pub fn prepare_raw(
    x_values: &[f64],
    y_values: &[f64],
    gridsize: usize,
    pl: i32,
    pt: i32,
    pw: i32,
    ph: i32,
    bounds: &Bounds,
) -> Option<Prepared> {
    let n = x_values.len().min(y_values.len());
    if n == 0 {
        return None;
    }
    let xr = (bounds.xmax - bounds.xmin).max(1e-9);
    let yr = (bounds.ymax - bounds.ymin).max(1e-9);
    let gridsize = gridsize.max(2) as f64;
    let r = (pw as f64 / gridsize / 1.5).max(2.0);
    let sx = r * 1.5;
    let sy = r * 3f64.sqrt();

    let mut counts: HashMap<(i64, i64, bool), u32> = HashMap::new();
    for i in 0..n {
        let px = pl as f64 + (x_values[i] - bounds.xmin) / xr * pw as f64;
        let py = pt as f64 + ph as f64 - (y_values[i] - bounds.ymin) / yr * ph as f64;

        let c1 = (px / sx).round();
        let rr1 = (py / sy).round();
        let cx1 = c1 * sx;
        let cy1 = rr1 * sy;

        let c2 = ((px - sx / 2.0) / sx).round();
        let rr2 = ((py - sy / 2.0) / sy).round();
        let cx2 = c2 * sx + sx / 2.0;
        let cy2 = rr2 * sy + sy / 2.0;

        let d1 = (px - cx1).powi(2) + (py - cy1).powi(2);
        let d2 = (px - cx2).powi(2) + (py - cy2).powi(2);

        let key = if d1 <= d2 {
            (c1 as i64, rr1 as i64, false)
        } else {
            (c2 as i64, rr2 as i64, true)
        };
        *counts.entry(key).or_insert(0) += 1;
    }
    if counts.is_empty() {
        return None;
    }

    let mut bins: Vec<Bin> = counts
        .into_iter()
        .map(|((c, rr, offset), count)| {
            let (cx, cy) = if offset {
                (c as f64 * sx + sx / 2.0, rr as f64 * sy + sy / 2.0)
            } else {
                (c as f64 * sx, rr as f64 * sy)
            };
            Bin { cx, cy, count }
        })
        .collect();
    bins.sort_by(|a, b| a.cy.partial_cmp(&b.cy).unwrap().then(a.cx.partial_cmp(&b.cx).unwrap()));

    let min_count = bins.iter().map(|b| b.count).min().unwrap_or(0);
    let max_count = bins.iter().map(|b| b.count).max().unwrap_or(1);

    Some(Prepared {
        bins,
        r,
        min_count,
        max_count,
    })
}

pub fn hex_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64) {
    push_b(buf, b"M ");
    for k in 0..6 {
        let ang = std::f64::consts::PI / 3.0 * k as f64;
        let x = cx + r * ang.cos();
        let y = cy + r * ang.sin();
        if k == 0 {
            push_f2(buf, x);
            buf.push(b' ');
            push_f2(buf, y);
        } else {
            push_b(buf, b" L ");
            push_f2(buf, x);
            buf.push(b' ');
            push_f2(buf, y);
        }
    }
    push_b(buf, b" Z");
}

pub fn highlight_cutoff(bins: &[Bin]) -> u32 {
    let mut counts: Vec<u32> = bins.iter().map(|b| b.count).collect();
    counts.sort_unstable();
    let cutoff_idx = (counts.len() as f64 * 0.85) as usize;
    counts.get(cutoff_idx.min(counts.len().saturating_sub(1))).copied().unwrap_or(0)
}

#[allow(clippy::too_many_arguments)]
pub fn draw_hex_cell(
    buf: &mut Vec<u8>,
    idx: usize,
    bin: &Bin,
    radius: f64,
    col: u32,
    opacity: Option<f64>,
    stroke: bool,
    label: bool,
) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_i(buf, bin.count as i32);
    push_b(buf, b"\" d=\"");
    hex_path(buf, bin.cx, bin.cy, radius);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(col));
    if let Some(op) = opacity {
        push_b(buf, b"\" fill-opacity=\"");
        push_f2(buf, op);
        if stroke {
            push_b(buf, b"\" stroke=\"#fff\" stroke-width=\"1.4\"/>");
        } else {
            push_b(buf, b"\"/>");
        }
    } else if stroke {
        push_b(buf, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");
    } else {
        push_b(buf, b"\"/>");
    }
    if label {
        push_b(buf, b"<text x=\"");
        push_f2(buf, bin.cx);
        push_b(buf, b"\" y=\"");
        push_f2(buf, bin.cy + 3.5);
        push_b(
            buf,
            b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#fff\" pointer-events=\"none\">",
        );
        push_i(buf, bin.count as i32);
        push_b(buf, b"</text>");
    }
}

pub fn bin_color(cfg: &HexbinConfig, p: &Prepared, count: u32) -> u32 {
    bin_color_raw(cfg.colorscale, p.min_count, p.max_count, count)
}

pub fn bin_color_raw(colorscale: &str, min_count: u32, max_count: u32, count: u32) -> u32 {
    let span = (max_count - min_count).max(1) as f64;
    let t = (count - min_count) as f64 / span;
    let scale = if colorscale.is_empty() { "viridis" } else { colorscale };
    colorscale_color(scale, t)
}

pub fn legend_bar(frame: &mut Frame, cfg: &HexbinConfig, p: &Prepared) {
    let bar_x = frame.pl + frame.pw + 18;
    let bar_y = frame.pt + 6;
    let bar_w = 14;
    let bar_h = (frame.ph - 12).max(40);
    let scale = if cfg.colorscale.is_empty() { "viridis" } else { cfg.colorscale };
    let grad_id = "sphb";
    push_b(&mut frame.buf, b"<defs><linearGradient id=\"");
    push_b(&mut frame.buf, grad_id.as_bytes());
    push_b(&mut frame.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    let stops = 8usize;
    for s in 0..stops {
        let t = s as f64 / (stops - 1) as f64;
        let col = colorscale_color(scale, t);
        push_b(&mut frame.buf, b"<stop offset=\"");
        push_f2(&mut frame.buf, t);
        push_b(&mut frame.buf, b"\" stop-color=\"#");
        frame.buf.extend_from_slice(&hex6(col));
        push_b(&mut frame.buf, b"\"/>");
    }
    push_b(&mut frame.buf, b"</linearGradient></defs>");
    push_b(&mut frame.buf, b"<rect x=\"");
    push_i(&mut frame.buf, bar_x);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, bar_y);
    push_b(&mut frame.buf, b"\" width=\"");
    push_i(&mut frame.buf, bar_w);
    push_b(&mut frame.buf, b"\" height=\"");
    push_i(&mut frame.buf, bar_h);
    push_b(&mut frame.buf, b"\" fill=\"url(#");
    push_b(&mut frame.buf, grad_id.as_bytes());
    push_b(&mut frame.buf, b")\" stroke=\"#475569\" stroke-width=\"0.5\"/>");
    let tx = bar_x + bar_w + 4;
    push_b(&mut frame.buf, b"<text x=\"");
    push_i(&mut frame.buf, tx);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, bar_y + 8);
    push_b(
        &mut frame.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
    );
    push_i(&mut frame.buf, p.max_count as i32);
    push_b(&mut frame.buf, b"</text>");
    push_b(&mut frame.buf, b"<text x=\"");
    push_i(&mut frame.buf, tx);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, bar_y + bar_h);
    push_b(
        &mut frame.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
    );
    push_i(&mut frame.buf, p.min_count as i32);
    push_b(&mut frame.buf, b"</text>");
}

pub fn finalize(frame: Frame, cfg: &HexbinConfig) -> String {
    frame.html(&slots_to_json(cfg.hover))
}

pub const MAGNITUDE_COLORS: [u32; 5] = [0x3b82f6, 0x22c55e, 0xf59e0b, 0xd946ef, 0x111827];
pub const MAGNITUDE_LABELS: [&str; 5] = ["Ones", "Tens", "Hundreds", "Thousands", "10 Thousands"];

pub fn magnitude_band(count: u32) -> usize {
    if count == 0 {
        0
    } else {
        ((count as f64).log10().floor() as usize).min(4)
    }
}

pub fn draw_nested_cell(buf: &mut Vec<u8>, idx: usize, bin: &Bin, base_r: f64) {
    let band = magnitude_band(bin.count);
    let r = base_r * (0.5 + band as f64 * 0.13);
    let outer = MAGNITUDE_COLORS[band];
    let hx_outer = hex6(outer);
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_i(buf, bin.count as i32);
    push_b(buf, b"\" d=\"");
    hex_path(buf, bin.cx, bin.cy, r);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hx_outer);
    push_b(buf, b"\" fill-opacity=\"0.88\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
    if band > 0 {
        let inner_col = MAGNITUDE_COLORS[band - 1];
        let hx_inner = hex6(inner_col);
        push_b(buf, b"<path d=\"");
        hex_path(buf, bin.cx, bin.cy, r * 0.55);
        push_b(buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx_inner);
        push_b(buf, b"\" fill-opacity=\"0.9\" pointer-events=\"none\"/>");
    }
}

pub fn bin_color_log(colorscale: &str, min_count: u32, max_count: u32, count: u32) -> u32 {
    let lmin = (min_count as f64 + 1.0).ln();
    let lmax = (max_count as f64 + 1.0).ln();
    let span = (lmax - lmin).max(1e-9);
    let t = ((count as f64 + 1.0).ln() - lmin) / span;
    let scale = if colorscale.is_empty() { "viridis" } else { colorscale };
    colorscale_color(scale, t.clamp(0.0, 1.0))
}

pub fn legend_bar_log(frame: &mut Frame, cfg: &HexbinConfig, p: &Prepared) {
    legend_bar(frame, cfg, p);
    let x = frame.pl + frame.pw + 18;
    push_b(&mut frame.buf, b"<text x=\"");
    push_i(&mut frame.buf, x);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, frame.pt - 6);
    push_b(
        &mut frame.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-style=\"italic\" fill=\"#94a3b8\">log scale</text>",
    );
}

pub struct WBin {
    pub cx: f64,
    pub cy: f64,
    pub count: u32,
    pub avg: f64,
}

pub struct WPrepared {
    pub bins: Vec<WBin>,
    pub r: f64,
    pub min_avg: f64,
    pub max_avg: f64,
}

pub fn prepare_weighted(cfg: &HexbinConfig, frame: &Frame, bounds: &Bounds) -> Option<WPrepared> {
    let n = cfg.x_values.len().min(cfg.y_values.len()).min(cfg.values.len());
    if n == 0 {
        return None;
    }
    let xr = (bounds.xmax - bounds.xmin).max(1e-9);
    let yr = (bounds.ymax - bounds.ymin).max(1e-9);
    let gridsize = cfg.gridsize.max(2) as f64;
    let r = (frame.pw as f64 / gridsize / 1.5).max(2.0);
    let sx = r * 1.5;
    let sy = r * 3f64.sqrt();

    let mut acc: HashMap<(i64, i64, bool), (u32, f64)> = HashMap::new();
    for i in 0..n {
        let px = frame.pl as f64 + (cfg.x_values[i] - bounds.xmin) / xr * frame.pw as f64;
        let py = frame.pt as f64 + frame.ph as f64 - (cfg.y_values[i] - bounds.ymin) / yr * frame.ph as f64;

        let c1 = (px / sx).round();
        let rr1 = (py / sy).round();
        let cx1 = c1 * sx;
        let cy1 = rr1 * sy;

        let c2 = ((px - sx / 2.0) / sx).round();
        let rr2 = ((py - sy / 2.0) / sy).round();
        let cx2 = c2 * sx + sx / 2.0;
        let cy2 = rr2 * sy + sy / 2.0;

        let d1 = (px - cx1).powi(2) + (py - cy1).powi(2);
        let d2 = (px - cx2).powi(2) + (py - cy2).powi(2);

        let key = if d1 <= d2 {
            (c1 as i64, rr1 as i64, false)
        } else {
            (c2 as i64, rr2 as i64, true)
        };
        let entry = acc.entry(key).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += cfg.values[i];
    }
    if acc.is_empty() {
        return None;
    }

    let mut bins: Vec<WBin> = acc
        .into_iter()
        .map(|((c, rr, offset), (count, sum))| {
            let (cx, cy) = if offset {
                (c as f64 * sx + sx / 2.0, rr as f64 * sy + sy / 2.0)
            } else {
                (c as f64 * sx, rr as f64 * sy)
            };
            WBin { cx, cy, count, avg: sum / count.max(1) as f64 }
        })
        .collect();
    bins.sort_by(|a, b| a.cy.partial_cmp(&b.cy).unwrap().then(a.cx.partial_cmp(&b.cx).unwrap()));

    let min_avg = bins.iter().map(|b| b.avg).fold(f64::INFINITY, f64::min);
    let max_avg = bins.iter().map(|b| b.avg).fold(f64::NEG_INFINITY, f64::max);

    Some(WPrepared { bins, r, min_avg, max_avg })
}

pub fn bin_color_value(colorscale: &str, min_v: f64, max_v: f64, v: f64) -> u32 {
    let span = (max_v - min_v).max(1e-9);
    let t = (v - min_v) / span;
    let scale = if colorscale.is_empty() { "viridis" } else { colorscale };
    colorscale_color(scale, t.clamp(0.0, 1.0))
}

pub fn draw_weighted_cell(buf: &mut Vec<u8>, idx: usize, bin: &WBin, radius: f64, col: u32) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, bin.avg);
    push_b(buf, b"\" d=\"");
    hex_path(buf, bin.cx, bin.cy, radius);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\"/>");
}

pub fn legend_bar_value(frame: &mut Frame, cfg: &HexbinConfig, p: &WPrepared) {
    let bar_x = frame.pl + frame.pw + 18;
    let bar_y = frame.pt + 6;
    let bar_w = 14;
    let bar_h = (frame.ph - 12).max(40);
    let scale = if cfg.colorscale.is_empty() { "viridis" } else { cfg.colorscale };
    let grad_id = "sphbv";
    push_b(&mut frame.buf, b"<defs><linearGradient id=\"");
    push_b(&mut frame.buf, grad_id.as_bytes());
    push_b(&mut frame.buf, b"\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    let stops = 8usize;
    for s in 0..stops {
        let t = s as f64 / (stops - 1) as f64;
        let col = colorscale_color(scale, t);
        push_b(&mut frame.buf, b"<stop offset=\"");
        push_f2(&mut frame.buf, t);
        push_b(&mut frame.buf, b"\" stop-color=\"#");
        frame.buf.extend_from_slice(&hex6(col));
        push_b(&mut frame.buf, b"\"/>");
    }
    push_b(&mut frame.buf, b"</linearGradient></defs>");
    push_b(&mut frame.buf, b"<rect x=\"");
    push_i(&mut frame.buf, bar_x);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, bar_y);
    push_b(&mut frame.buf, b"\" width=\"");
    push_i(&mut frame.buf, bar_w);
    push_b(&mut frame.buf, b"\" height=\"");
    push_i(&mut frame.buf, bar_h);
    push_b(&mut frame.buf, b"\" fill=\"url(#");
    push_b(&mut frame.buf, grad_id.as_bytes());
    push_b(&mut frame.buf, b")\" stroke=\"#475569\" stroke-width=\"0.5\"/>");
    let tx = bar_x + bar_w + 4;
    push_b(&mut frame.buf, b"<text x=\"");
    push_i(&mut frame.buf, tx);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, bar_y + 8);
    push_b(
        &mut frame.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
    );
    push_f2(&mut frame.buf, p.max_avg);
    push_b(&mut frame.buf, b"</text>");
    push_b(&mut frame.buf, b"<text x=\"");
    push_i(&mut frame.buf, tx);
    push_b(&mut frame.buf, b"\" y=\"");
    push_i(&mut frame.buf, bar_y + bar_h);
    push_b(
        &mut frame.buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">",
    );
    push_f2(&mut frame.buf, p.min_avg);
    push_b(&mut frame.buf, b"</text>");
}

pub fn draw_hex_cell_dashed(buf: &mut Vec<u8>, idx: usize, bin: &Bin, radius: f64, col: u32, dash: &str) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_i(buf, bin.count as i32);
    push_b(buf, b"\" d=\"");
    hex_path(buf, bin.cx, bin.cy, radius);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"1.5\" stroke-dasharray=\"");
    push_b(buf, dash.as_bytes());
    push_b(buf, b"\"/>");
}

pub fn hist_counts(values: &[f64], vmin: f64, vmax: f64, bins: usize) -> Vec<u32> {
    let mut counts = vec![0u32; bins.max(1)];
    let span = (vmax - vmin).max(1e-9);
    for &v in values {
        let mut idx = ((v - vmin) / span * bins as f64) as usize;
        if idx >= bins {
            idx = bins - 1;
        }
        counts[idx] += 1;
    }
    counts
}

pub fn voronoi_cell(i: usize, pts: &[(f64, f64)], rect: (f64, f64, f64, f64)) -> Vec<(f64, f64)> {
    let (x0, y0, x1, y1) = rect;
    let mut poly = vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)];
    let (sx, sy) = pts[i];
    for (j, &(ox, oy)) in pts.iter().enumerate() {
        if j == i || poly.is_empty() {
            continue;
        }
        let mx = (sx + ox) / 2.0;
        let my = (sy + oy) / 2.0;
        let dx = sx - ox;
        let dy = sy - oy;
        poly = clip_halfplane(&poly, mx, my, dx, dy);
    }
    poly
}

fn clip_halfplane(poly: &[(f64, f64)], mx: f64, my: f64, dx: f64, dy: f64) -> Vec<(f64, f64)> {
    let n = poly.len();
    if n == 0 {
        return Vec::new();
    }
    let inside = |p: (f64, f64)| dx * (p.0 - mx) + dy * (p.1 - my) >= 0.0;
    let mut out = Vec::with_capacity(n + 1);
    for k in 0..n {
        let cur = poly[k];
        let prev = poly[(k + n - 1) % n];
        let cur_in = inside(cur);
        let prev_in = inside(prev);
        if cur_in {
            if !prev_in {
                out.push(intersect(prev, cur, mx, my, dx, dy));
            }
            out.push(cur);
        } else if prev_in {
            out.push(intersect(prev, cur, mx, my, dx, dy));
        }
    }
    out
}

fn intersect(a: (f64, f64), b: (f64, f64), mx: f64, my: f64, dx: f64, dy: f64) -> (f64, f64) {
    let fa = dx * (a.0 - mx) + dy * (a.1 - my);
    let fb = dx * (b.0 - mx) + dy * (b.1 - my);
    let t = fa / (fa - fb);
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

pub fn polygon_area(poly: &[(f64, f64)]) -> f64 {
    if poly.len() < 3 {
        return 0.0;
    }
    let n = poly.len();
    let mut sum = 0.0;
    for k in 0..n {
        let (x1, y1) = poly[k];
        let (x2, y2) = poly[(k + 1) % n];
        sum += x1 * y2 - x2 * y1;
    }
    (sum * 0.5).abs()
}

pub fn magnitude_legend(frame: &mut Frame, max_count: u32) {
    let top_band = magnitude_band(max_count);
    let x = frame.pl + frame.pw + 14;
    let mut y = frame.pt + 8;
    for band in (0..=top_band).rev() {
        let col = MAGNITUDE_COLORS[band];
        let hx = hex6(col);
        push_b(&mut frame.buf, b"<text x=\"");
        push_i(&mut frame.buf, x);
        push_b(&mut frame.buf, b"\" y=\"");
        push_i(&mut frame.buf, y);
        push_b(
            &mut frame.buf,
            b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#374151\">",
        );
        push_b(&mut frame.buf, MAGNITUDE_LABELS[band].as_bytes());
        push_b(&mut frame.buf, b"</text>");
        for k in 0..4 {
            let cx = x + 10 + k * 16;
            let cy = y + 16;
            let r = 3.0 + k as f64 * 1.6;
            push_b(&mut frame.buf, b"<circle cx=\"");
            push_i(&mut frame.buf, cx);
            push_b(&mut frame.buf, b"\" cy=\"");
            push_i(&mut frame.buf, cy);
            push_b(&mut frame.buf, b"\" r=\"");
            push_f2(&mut frame.buf, r);
            push_b(&mut frame.buf, b"\" fill=\"#");
            frame.buf.extend_from_slice(&hx);
            push_b(&mut frame.buf, b"\" fill-opacity=\"0.85\"/>");
        }
        y += 34;
    }
}
