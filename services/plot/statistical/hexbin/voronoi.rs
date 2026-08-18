use super::common::{data_bounds, finalize};
use super::config::HexbinConfig;
use crate::plot::statistical::common::{
    colorscale_color, hex6, push_b, push_f2, push_i, svg_open_rescalable, svg_title, Frame,
};

#[crate::chart_demo(
    "title=\"Density Voronoi - Particle Tracking\", x=[29.57,30.24,30.76,32.71,31.63,33.55,35.4,31.79,35.31,35.27,37.31,35.35,39.48,38.56,40.35,40.28,31.28,36.97,42.28,43.4,41.17,42.34,44.49,45.09,41.83,42.88,44.11,42.97,48.31,48.09,51.42,43.45,51.14,48.96,48.48,51.74,52.43,49.46,54.57,52.07,50.69,47.9,52.3,58.15,55.93,58.97,57.11,59.57,61.11,63.86,63.35,57.59,59.26,62.73,53.73,63.1,60.12,67.75,61.86,62.38,64.58,65.52,68.08,72.85,70.67,66.78,72.95,71.26,73.46,69.58,47.51,53.85,51.33,45.59,56.57,63.69,48.3,58.1,50.8,55.53,62.64,56.72,56.46,55.67,55.71,62.17,44.64,56.38,60.23,49.03,53.82,57.72,50.53,57.93,57.4,50.72,55.02,43.83,53.16,66.41,68.45,57.1,50.86,53.3,42.37,27.46,45.68,93.9,55.35,16.86,95.0,54.05,7.49,50.27,17.11,9.69,59.12,24.58,87.47,59.07,42.25,52.19,21.61,24.91,66.48,32.35,8.96,97.85,9.03,27.46,86.56,37.47,82.04,60.72,64.78,80.44,65.69,14.89,12.28,28.15], y=[7.31,11.92,4.24,10.27,12.92,18.23,15.13,11.74,22.16,17.6,13.33,22.22,20.41,23.48,19.15,18.66,23.4,30.45,23.18,25.16,29.11,34.67,33.91,35.54,31.87,37.85,46.31,33.69,44.89,43.66,41.05,40.41,37.88,45.78,48.78,56.29,45.68,45.9,48.08,54.45,51.4,49.36,56.46,55.84,57.7,61.72,55.21,61.9,61.37,61.16,64.17,60.31,71.64,69.81,71.02,67.09,70.39,67.29,78.08,72.29,69.89,71.13,77.1,79.32,74.1,81.72,74.82,85.02,86.62,82.8,56.56,71.16,57.56,51.84,61.59,54.65,58.66,58.93,64.07,60.6,59.87,72.22,52.63,66.86,59.16,50.9,57.4,50.14,59.86,59.15,43.14,54.64,59.18,50.38,63.12,58.05,73.13,60.56,54.2,54.93,51.46,51.07,55.5,56.62,71.02,77.27,42.61,97.56,70.97,30.48,57.6,73.81,58.08,83.86,94.23,19.84,66.82,13.51,25.64,61.46,58.03,91.73,70.75,40.0,30.8,74.18,46.0,97.63,22.46,91.59,86.41,17.14,69.54,96.77,2.75,30.74,92.14,13.08,55.11,60.06], width=1040, height=860, variant=\"voronoi\""
)]
pub fn render(cfg: &HexbinConfig) -> String {
    let n = cfg.x_values.len().min(cfg.y_values.len());
    if n == 0 {
        return String::new();
    }
    let bounds = match data_bounds(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 22, 40, 24, 150, n * 220 + 8192);
    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;

    svg_open_rescalable(&mut f.buf, f.w, f.h, f.pl, f.pt, f.pw, f.ph);
    svg_title(&mut f.buf, cfg.title, f.pl + f.pw / 2, 26);

    let xr = (bounds.xmax - bounds.xmin).max(1e-9);
    let yr = (bounds.ymax - bounds.ymin).max(1e-9);
    let pts: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let px = f.pl as f64 + (cfg.x_values[i] - bounds.xmin) / xr * f.pw as f64;
            let py = f.pt as f64 + f.ph as f64 - (cfg.y_values[i] - bounds.ymin) / yr * f.ph as f64;
            (px, py)
        })
        .collect();

    let rect = (f.pl as f64, f.pt as f64, (f.pl + f.pw) as f64, (f.pt + f.ph) as f64);
    let cells: Vec<Vec<(f64, f64)>> = (0..n).map(|i| voronoi_cell(i, &pts, rect)).collect();
    let areas: Vec<f64> = cells.iter().map(|c| polygon_area(c)).collect();

    let has_values = cfg.values.len() >= n;
    let raw: Vec<f64> = if has_values {
        cfg.values[..n].to_vec()
    } else {
        areas.iter().map(|a| 1.0 / a.max(1.0)).collect()
    };
    let vmin = raw.iter().cloned().fold(f64::INFINITY, f64::min);
    let vmax = raw.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let scale = if cfg.colorscale.is_empty() { "inferno" } else { cfg.colorscale };

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| raw[a].partial_cmp(&raw[b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank_t = vec![0.0_f64; n];
    for (r, &i) in order.iter().enumerate() {
        rank_t[i] = if n > 1 { r as f64 / (n - 1) as f64 } else { 0.0 };
    }

    push_b(&mut f.buf, b"<g>");
    for i in 0..n {
        if cells[i].len() < 3 {
            continue;
        }
        let col = colorscale_color(scale, rank_t[i]);
        draw_cell(&mut f.buf, i, &cells[i], col, raw[i]);
    }
    push_b(&mut f.buf, b"</g>");

    push_b(&mut f.buf, b"<g pointer-events=\"none\">");
    for &(px, py) in &pts {
        push_b(&mut f.buf, b"<circle cx=\"");
        push_f2(&mut f.buf, px);
        push_b(&mut f.buf, b"\" cy=\"");
        push_f2(&mut f.buf, py);
        push_b(&mut f.buf, b"\" r=\"1.7\" fill=\"#");
        f.buf.extend_from_slice(&hex6(ink));
        push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"0.6\" fill-opacity=\"0.92\"/>");
    }
    push_b(&mut f.buf, b"</g>");

    legend(&mut f, scale, vmin, vmax, ink, sub);
    finalize(f, cfg)
}

fn draw_cell(buf: &mut Vec<u8>, idx: usize, poly: &[(f64, f64)], col: u32, val: f64) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-y=\"");
    push_f2(buf, val);
    push_b(buf, b"\" d=\"M ");
    push_f2(buf, poly[0].0);
    buf.push(b' ');
    push_f2(buf, poly[0].1);
    for &(x, y) in &poly[1..] {
        push_b(buf, b" L ");
        push_f2(buf, x);
        buf.push(b' ');
        push_f2(buf, y);
    }
    push_b(buf, b" Z\" fill=\"#");
    buf.extend_from_slice(&hex6(col));
    push_b(buf, b"\" fill-opacity=\"0.94\" stroke=\"#fff\" stroke-width=\"1.2\"/>");
}

#[allow(clippy::too_many_arguments)]
fn legend(f: &mut Frame, scale: &str, vmin: f64, vmax: f64, ink: u32, sub: u32) {
    let bar_x = f.pl + f.pw + 22;
    let bar_y = f.pt + 10;
    let bar_w = 14;
    let bar_h = (f.ph - 60).max(60);

    push_b(&mut f.buf, b"<defs><linearGradient id=\"spvor\" x1=\"0\" y1=\"1\" x2=\"0\" y2=\"0\">");
    for s in 0..8 {
        let t = s as f64 / 7.0;
        let col = colorscale_color(scale, t);
        push_b(&mut f.buf, b"<stop offset=\"");
        push_f2(&mut f.buf, t);
        push_b(&mut f.buf, b"\" stop-color=\"#");
        f.buf.extend_from_slice(&hex6(col));
        push_b(&mut f.buf, b"\"/>");
    }
    push_b(&mut f.buf, b"</linearGradient></defs>");

    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y - 8);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" fill=\"#");
    f.buf.extend_from_slice(&hex6(sub));
    push_b(&mut f.buf, b"\" letter-spacing=\"0.6\">DENSITY</text>");

    push_b(&mut f.buf, b"<rect x=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y);
    push_b(&mut f.buf, b"\" width=\"");
    push_i(&mut f.buf, bar_w);
    push_b(&mut f.buf, b"\" height=\"");
    push_i(&mut f.buf, bar_h);
    push_b(&mut f.buf, b"\" fill=\"url(#spvor)\" stroke=\"#334155\" stroke-width=\"0.6\" rx=\"2\"/>");

    let tx = bar_x + bar_w + 6;
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + 9);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\">");
    push_f2(&mut f.buf, vmax);
    push_b(&mut f.buf, b"</text>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + bar_h);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\">");
    push_f2(&mut f.buf, vmin);
    push_b(&mut f.buf, b"</text>");

    let py = bar_y + bar_h + 28;
    push_b(&mut f.buf, b"<circle cx=\"");
    push_i(&mut f.buf, bar_x + 6);
    push_b(&mut f.buf, b"\" cy=\"");
    push_i(&mut f.buf, py);
    push_b(&mut f.buf, b"\" r=\"3\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\"/>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, tx);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, py + 3);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(&hex6(ink));
    push_b(&mut f.buf, b"\">Particle tracking</text>");
}

fn voronoi_cell(i: usize, pts: &[(f64, f64)], rect: (f64, f64, f64, f64)) -> Vec<(f64, f64)> {
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

fn polygon_area(poly: &[(f64, f64)]) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::common::make_frame;

    fn cfg<'a>(x: &'a [f64], y: &'a [f64]) -> HexbinConfig<'a> {
        HexbinConfig {
            title: "Test",
            x_values: x,
            y_values: y,
            width: 1040,
            height: 860,
            ..HexbinConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<f64>, Vec<f64>) {
        let x: Vec<f64> = (0..n).map(|i| ((i as f64 * 0.37).sin() * 0.5 + 0.5) * 100.0).collect();
        let y: Vec<f64> = (0..n).map(|i| ((i as f64 * 0.53).cos() * 0.5 + 0.5) * 100.0).collect();
        (x, y)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("hexbin/voronoi.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/hexbin-voronoi.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_cell_per_site() {
        let (x, y) = synth(24);
        let html = render(&cfg(&x, &y));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=\"").count(), x.len());
    }

    #[test]
    fn renders_one_dot_per_site() {
        let (x, y) = synth(24);
        let html = render(&cfg(&x, &y));
        let dots = html.matches("r=\"1.7\" fill=\"#").count();
        assert_eq!(dots, x.len());
    }

    #[test]
    fn every_cell_polygon_stays_inside_the_plot_rect() {
        let (x, y) = synth(30);
        let bounds = data_bounds(&cfg(&x, &y)).unwrap();
        let f = make_frame(&cfg(&x, &y));
        let xr = (bounds.xmax - bounds.xmin).max(1e-9);
        let yr = (bounds.ymax - bounds.ymin).max(1e-9);
        let pts: Vec<(f64, f64)> = (0..x.len())
            .map(|i| {
                let px = f.pl as f64 + (x[i] - bounds.xmin) / xr * f.pw as f64;
                let py = f.pt as f64 + f.ph as f64 - (y[i] - bounds.ymin) / yr * f.ph as f64;
                (px, py)
            })
            .collect();
        let rect = (f.pl as f64, f.pt as f64, (f.pl + f.pw) as f64, (f.pt + f.ph) as f64);
        for i in 0..pts.len() {
            let poly = voronoi_cell(i, &pts, rect);
            for (px, py) in poly {
                assert!(px >= rect.0 - 0.01 && px <= rect.2 + 0.01, "x {px} escaped [{}, {}]", rect.0, rect.2);
                assert!(py >= rect.1 - 0.01 && py <= rect.3 + 0.01, "y {py} escaped [{}, {}]", rect.1, rect.3);
            }
        }
    }

    #[test]
    fn voronoi_cells_tile_the_plot_rect_without_gaps() {
        let (x, y) = synth(18);
        let bounds = data_bounds(&cfg(&x, &y)).unwrap();
        let f = make_frame(&cfg(&x, &y));
        let xr = (bounds.xmax - bounds.xmin).max(1e-9);
        let yr = (bounds.ymax - bounds.ymin).max(1e-9);
        let pts: Vec<(f64, f64)> = (0..x.len())
            .map(|i| {
                let px = f.pl as f64 + (x[i] - bounds.xmin) / xr * f.pw as f64;
                let py = f.pt as f64 + f.ph as f64 - (y[i] - bounds.ymin) / yr * f.ph as f64;
                (px, py)
            })
            .collect();
        let rect = (f.pl as f64, f.pt as f64, (f.pl + f.pw) as f64, (f.pt + f.ph) as f64);
        let rect_area = (rect.2 - rect.0) * (rect.3 - rect.1);
        let total: f64 = (0..pts.len()).map(|i| polygon_area(&voronoi_cell(i, &pts, rect))).sum();
        assert!((total - rect_area).abs() / rect_area < 1e-6, "cells do not tile the rect: {total} vs {rect_area}");
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty: Vec<f64> = vec![];
        assert!(render(&cfg(&empty, &empty)).is_empty());
    }

    #[test]
    fn perf_rendering_many_particles_stays_fast() {
        let (x, y) = synth(300);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &y));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 600, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
