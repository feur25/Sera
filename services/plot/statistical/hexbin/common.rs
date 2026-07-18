use super::config::HexbinConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i, Frame};
use crate::plot::statistical::heatmap::common::colorscale_color;
use std::collections::HashMap;

pub struct Bounds {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

pub fn data_bounds(cfg: &HexbinConfig) -> Option<Bounds> {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return None;
    }
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for i in 0..n {
        let x = cfg.x_values[i];
        let y = cfg.y_values[i];
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
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return None;
    }
    let xr = (bounds.xmax - bounds.xmin).max(1e-9);
    let yr = (bounds.ymax - bounds.ymin).max(1e-9);
    let gridsize = cfg.gridsize.max(2) as f64;
    let r = (frame.pw as f64 / gridsize / 1.5).max(2.0);
    let sx = r * 1.5;
    let sy = r * 3f64.sqrt();

    let mut counts: HashMap<(i64, i64, bool), u32> = HashMap::new();
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

pub fn bin_color(cfg: &HexbinConfig, p: &Prepared, count: u32) -> u32 {
    let span = (p.max_count - p.min_count).max(1) as f64;
    let t = (count - p.min_count) as f64 / span;
    let scale = if cfg.colorscale.is_empty() { "viridis" } else { cfg.colorscale };
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
