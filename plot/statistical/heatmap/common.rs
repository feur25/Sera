use super::config::HeatmapConfig;
use crate::html::hover::{build_chart_html, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, lerp_color, palette_color, push_b, push_f2, push_i, truncate};

pub fn clone_cfg<'a>(cfg: &HeatmapConfig<'a>) -> HeatmapConfig<'a> {
    HeatmapConfig {
        title: cfg.title, x_label: cfg.x_label, y_label: cfg.y_label,
        gridlines: cfg.gridlines, sort_order: cfg.sort_order, hover: cfg.hover,
        legend_position: cfg.legend_position, width: cfg.width, height: cfg.height,
        variant: cfg.variant, row_labels: cfg.row_labels, col_labels: cfg.col_labels,
        flat_matrix: cfg.flat_matrix, show_values: cfg.show_values,
        value_min_cell: cfg.value_min_cell, value_decimals: cfg.value_decimals,
        col_label_angle: cfg.col_label_angle, color_low: cfg.color_low,
        color_mid: cfg.color_mid, color_high: cfg.color_high, palette: cfg.palette,
        discrete_steps: cfg.discrete_steps, log_scale: cfg.log_scale,
        diverging: cfg.diverging, x_widths: cfg.x_widths, y_heights: cfg.y_heights,
        annotate: cfg.annotate, categorical: cfg.categorical,
        smooth: cfg.smooth, viridis: cfg.viridis,
        contour: cfg.contour, contour_levels: cfg.contour_levels,
        colorscale: cfg.colorscale, colorbar_position: cfg.colorbar_position,
        origin_lower: cfg.origin_lower,
        bubble_mode: cfg.bubble_mode, marginal_mode: cfg.marginal_mode,
        confusion_mode: cfg.confusion_mode, pivot_mode: cfg.pivot_mode,
        cluster_mode: cfg.cluster_mode,
        row_totals: cfg.row_totals, col_totals: cfg.col_totals,
        secondary_matrix: cfg.secondary_matrix,
    }
}


pub fn finite_minmax(data: &[f64]) -> (f64, f64) {
    let mut mn = f64::INFINITY;
    let mut mx = f64::NEG_INFINITY;
    for &v in data { if v.is_finite() { if v < mn { mn = v; } if v > mx { mx = v; } } }
    if !mn.is_finite() { return (0.0, 1.0); }
    (mn, mx)
}

pub fn map_value_to_t(val: f64, vmin: f64, vmax: f64, log: bool, diverging: bool) -> f64 {
    if !val.is_finite() { return 0.5; }
    if diverging {
        let m = vmin.abs().max(vmax.abs()).max(1e-12);
        return ((val / m) * 0.5 + 0.5).clamp(0.0, 1.0);
    }
    if log {
        let lmin = (vmin.max(0.0) + 1.0).ln();
        let lmax = (vmax.max(0.0) + 1.0).ln();
        let lv = (val.max(0.0) + 1.0).ln();
        return ((lv - lmin) / (lmax - lmin).max(1e-12)).clamp(0.0, 1.0);
    }
    let r = (vmax - vmin).max(1e-12);
    ((val - vmin) / r).clamp(0.0, 1.0)
}

pub fn quantize_t(t: f64, steps: usize) -> f64 {
    if steps == 0 { return t; }
    let s = steps.max(2) as f64;
    ((t * s).floor().min(s - 1.0)) / (s - 1.0)
}

pub fn cell_color(t: f64, cfg: &HeatmapConfig) -> u32 {
    if cfg.discrete_steps > 0 && !cfg.palette.is_empty() {
        let s = cfg.discrete_steps.max(2);
        let bin = ((t * s as f64).floor() as usize).min(s - 1);
        return palette_color(cfg.palette, bin);
    }
    let qt = quantize_t(t, cfg.discrete_steps);
    if !cfg.colorscale.is_empty() {
        return colorscale_color(cfg.colorscale, qt);
    }
    if cfg.viridis {
        return viridis_color(qt);
    }
    if cfg.diverging {
        return colorscale_color("rdbu_r", qt);
    }
    lerp_color(qt, cfg.color_low, cfg.color_mid, cfg.color_high)
}

pub fn colorscale_color(name: &str, t: f64) -> u32 {
    let t = t.clamp(0.0, 1.0);
    let stops: &[u32] = match name.to_ascii_lowercase().as_str() {
        "viridis" => &[0x440154, 0x3B528B, 0x21908C, 0x5DC863, 0xFDE725],
        "plasma" => &[0x0D0887, 0x6A00A8, 0xB12A90, 0xE16462, 0xFCA636, 0xF0F921],
        "inferno" => &[0x000004, 0x420A68, 0x932667, 0xDD513A, 0xFCA50A, 0xFCFFA4],
        "magma" => &[0x000004, 0x3B0F70, 0x8C2981, 0xDE4968, 0xFE9F6D, 0xFCFDBF],
        "cividis" => &[0x00224E, 0x35446B, 0x666970, 0x958F78, 0xCAB969, 0xFEE838],
        "turbo" => &[0x30123B, 0x4145AB, 0x4675ED, 0x39A2FC, 0x1BCFD4, 0x24EC9F, 0x61FC6C, 0xA4FC3D, 0xD1E834, 0xF3C63A, 0xFE9B2D, 0xF36315, 0xC92903, 0x7A0403],
        "rdbu_r" | "rdbu" => &[0x053061, 0x2166AC, 0x4393C3, 0x92C5DE, 0xD1E5F0, 0xF7F7F7, 0xFDDBC7, 0xF4A582, 0xD6604D, 0xB2182B, 0x67001F],
        "blues" => &[0xF7FBFF, 0xDEEBF7, 0xC6DBEF, 0x9ECAE1, 0x6BAED6, 0x4292C6, 0x2171B5, 0x08519C, 0x08306B],
        "reds" => &[0xFFF5F0, 0xFEE0D2, 0xFCBBA1, 0xFC9272, 0xFB6A4A, 0xEF3B2C, 0xCB181D, 0xA50F15, 0x67000D],
        "greens" => &[0xF7FCF5, 0xE5F5E0, 0xC7E9C0, 0xA1D99B, 0x74C476, 0x41AB5D, 0x238B45, 0x006D2C, 0x00441B],
        _ => &[0x440154, 0x3B528B, 0x21908C, 0x5DC863, 0xFDE725],
    };
    if name.eq_ignore_ascii_case("rdbu_r") {
        return interp_stops(stops, 1.0 - t);
    }
    interp_stops(stops, t)
}

fn interp_stops(stops: &[u32], t: f64) -> u32 {
    let n = stops.len() - 1;
    let p = t * n as f64;
    let i0 = (p.floor() as usize).min(n);
    let i1 = (i0 + 1).min(n);
    let f = p - i0 as f64;
    let a = stops[i0]; let b = stops[i1];
    let ar = ((a >> 16) & 0xFF) as f64; let ag = ((a >> 8) & 0xFF) as f64; let ab = (a & 0xFF) as f64;
    let br = ((b >> 16) & 0xFF) as f64; let bg = ((b >> 8) & 0xFF) as f64; let bb = (b & 0xFF) as f64;
    let r = (ar + (br - ar) * f).round() as u32;
    let g = (ag + (bg - ag) * f).round() as u32;
    let bl = (ab + (bb - ab) * f).round() as u32;
    (r << 16) | (g << 8) | bl
}

pub fn viridis_color(t: f64) -> u32 {
    let stops: [u32; 5] = [0x440154, 0x3B528B, 0x21908C, 0x5DC863, 0xFDE725];
    let n = stops.len() - 1;
    let p = (t.clamp(0.0, 1.0)) * n as f64;
    let i0 = p.floor() as usize;
    let i1 = (i0 + 1).min(n);
    let f = p - i0 as f64;
    let a = stops[i0]; let b = stops[i1];
    let ar = ((a >> 16) & 0xFF) as f64; let ag = ((a >> 8) & 0xFF) as f64; let ab = (a & 0xFF) as f64;
    let br = ((b >> 16) & 0xFF) as f64; let bg = ((b >> 8) & 0xFF) as f64; let bb = (b & 0xFF) as f64;
    let r = (ar + (br - ar) * f).round() as u32;
    let g = (ag + (bg - ag) * f).round() as u32;
    let bl = (ab + (bb - ab) * f).round() as u32;
    (r << 16) | (g << 8) | bl
}

pub fn categorical_color(val: f64, cfg: &HeatmapConfig) -> u32 {
    let key = if val.is_finite() { val.to_bits() as usize } else { 0 };
    palette_color(cfg.palette, key.wrapping_mul(2654435761) as usize % 256)
}

pub fn cumulative(arr: &[f64], total_px: i32) -> Vec<i32> {
    let n = arr.len();
    if n == 0 { return vec![0]; }
    let sum: f64 = arr.iter().filter(|v| v.is_finite() && **v > 0.0).sum::<f64>().max(1e-12);
    let mut out = Vec::with_capacity(n + 1);
    out.push(0);
    let mut acc = 0.0;
    for &v in arr {
        let w = if v.is_finite() && v > 0.0 { v } else { 0.0 };
        acc += w;
        out.push(((acc / sum) * total_px as f64).round() as i32);
    }
    out
}

pub fn render_core(cfg: &HeatmapConfig) -> String {
    let n_rows = cfg.row_labels.len();
    let col_lbls: &[String] = if cfg.col_labels.is_empty() { cfg.row_labels } else { cfg.col_labels };
    let n_cols = col_lbls.len();
    if n_rows == 0 || n_cols == 0 || cfg.flat_matrix.len() < n_rows * n_cols {
        return String::new();
    }
    let data = &cfg.flat_matrix[..n_rows * n_cols];
    let (vmin, vmax) = finite_minmax(data);

    let pad_left: i32 = 100;
    let pad_top: i32 = 88;
    let right_bar = !cfg.categorical && cfg.colorbar_position.eq_ignore_ascii_case("right");
    let pad_right: i32 = if right_bar { 90 } else { 24 };
    let pad_bottom: i32 = 52;

    let plot_w = (cfg.width - pad_left - pad_right).max(40);
    let cell_w_uni = (plot_w / n_cols as i32).max(4);
    let svg_h = if cfg.height > 0 {
        cfg.height
    } else {
        pad_top + cell_w_uni * n_rows as i32 + pad_bottom
    };
    let plot_h = (svg_h - pad_top - pad_bottom).max(40);
    let cell_h_uni = (plot_h / n_rows as i32).max(4);

    let use_unequal_x = !cfg.x_widths.is_empty() && cfg.x_widths.len() == n_cols;
    let use_unequal_y = !cfg.y_heights.is_empty() && cfg.y_heights.len() == n_rows;
    let xs: Vec<i32> = if use_unequal_x {
        cumulative(cfg.x_widths, plot_w)
    } else {
        (0..=n_cols as i32).map(|i| i * cell_w_uni).collect()
    };
    let ys: Vec<i32> = if use_unequal_y {
        cumulative(cfg.y_heights, plot_h)
    } else {
        (0..=n_rows as i32).map(|i| i * cell_h_uni).collect()
    };

    let svg_w = cfg.width;
    let auto_hover = cfg.hover.is_empty();
    let total_cells = n_rows * n_cols;
    let mut buf = Vec::<u8>::with_capacity(total_cells * 220 + 2048);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, svg_w);
    push_b(&mut buf, b" ");
    push_i(&mut buf, svg_h);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#f8f9fa\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, svg_w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for (col, lbl) in col_lbls.iter().enumerate() {
        let cx = pad_left + (xs[col] + xs[col + 1]) / 2;
        let cy = pad_top - 8;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, cy);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#4b5563\" transform=\"rotate(-");
        push_i(&mut buf, cfg.col_label_angle);
        push_b(&mut buf, b",");
        push_i(&mut buf, cx);
        push_b(&mut buf, b",");
        push_i(&mut buf, cy);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(lbl, 14));
        push_b(&mut buf, b"</text>");
    }

    let mut idx = 0usize;
    for row in 0..n_rows {
        let ry0 = pad_top + ys[row];
        let ry1 = pad_top + ys[row + 1];
        let rh = (ry1 - ry0).max(1);
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_left - 5);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ry0 + rh / 2 + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#4b5563\">");
        escape_xml(&mut buf, truncate(&cfg.row_labels[row], 14));
        push_b(&mut buf, b"</text>");

        for col in 0..n_cols {
            let val = data[row * n_cols + col];
            let color = if cfg.categorical {
                categorical_color(val, cfg)
            } else {
                let t = map_value_to_t(val, vmin, vmax, cfg.log_scale, cfg.diverging);
                cell_color(t, cfg)
            };
            let hx = hex6(color);
            let cx0 = pad_left + xs[col];
            let cx1 = pad_left + xs[col + 1];
            let cw = (cx1 - cx0).max(1);

            push_b(&mut buf, b"<rect data-idx=\"");
            push_i(&mut buf, idx as i32);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, val);
            push_b(&mut buf, b"\" data-r=\"");
            escape_xml(&mut buf, truncate(&cfg.row_labels[row], 16));
            push_b(&mut buf, b"\" data-c=\"");
            escape_xml(&mut buf, truncate(&col_lbls[col], 16));
            push_b(&mut buf, b"\" x=\"");
            push_i(&mut buf, cx0);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, ry0);
            push_b(&mut buf, b"\" width=\"");
            push_i(&mut buf, cw);
            push_b(&mut buf, b"\" height=\"");
            push_i(&mut buf, rh);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            if cfg.smooth {
                push_b(&mut buf, b"\" stroke=\"none\"/>");
            } else {
                push_b(&mut buf, b"\" rx=\"2\" stroke=\"#e2e8f0\" stroke-width=\"0.4\"/>");
            }

            let show = cfg.show_values || cfg.annotate;
            let min_cell = if cfg.annotate { 10 } else { cfg.value_min_cell };
            if show && cw >= min_cell && rh >= min_cell {
                let lum = 0.299 * ((color >> 16 & 0xFF) as f64)
                    + 0.587 * ((color >> 8 & 0xFF) as f64)
                    + 0.114 * ((color & 0xFF) as f64);
                let text_col = if lum > 140.0 { b"#1f2937".as_ref() } else { b"#f9fafb".as_ref() };
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, cx0 + cw / 2);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, ry0 + rh / 2);
                push_b(&mut buf, b"\" text-anchor=\"middle\" dominant-baseline=\"central\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"");
                buf.extend_from_slice(text_col);
                push_b(&mut buf, b"\">");
                push_f2(&mut buf, val);
                push_b(&mut buf, b"</text>");
            }
            idx += 1;
        }
    }

    if cfg.contour && cfg.contour_levels > 0 && !cfg.categorical {
        let levels = cfg.contour_levels.max(1);
        for li in 1..=levels {
            let t_thr = li as f64 / (levels + 1) as f64;
            let v_thr = vmin + (vmax - vmin) * t_thr;
            for row in 0..n_rows {
                let ry0 = pad_top + ys[row];
                let ry1 = pad_top + ys[row + 1];
                let cy = (ry0 + ry1) / 2;
                for col in 0..n_cols.saturating_sub(1) {
                    let a = data[row * n_cols + col];
                    let b = data[row * n_cols + col + 1];
                    if (a - v_thr) * (b - v_thr) < 0.0 {
                        let span = (b - a).abs().max(1e-12);
                        let frac = ((v_thr - a).abs() / span).clamp(0.0, 1.0);
                        let cx0 = pad_left + xs[col];
                        let cx1 = pad_left + xs[col + 1];
                        let cx = (cx0 as f64 + (cx1 - cx0) as f64 * (0.5 + frac * 0.0)) as i32;
                        let _ = frac;
                        push_b(&mut buf, b"<line x1=\"");
                        push_i(&mut buf, cx);
                        push_b(&mut buf, b"\" y1=\"");
                        push_i(&mut buf, ry0);
                        push_b(&mut buf, b"\" x2=\"");
                        push_i(&mut buf, cx);
                        push_b(&mut buf, b"\" y2=\"");
                        push_i(&mut buf, ry1);
                        push_b(&mut buf, b"\" stroke=\"#ffffff\" stroke-width=\"1.2\" stroke-opacity=\"0.85\"/>");
                    }
                }
            }
            for col in 0..n_cols {
                let cx0 = pad_left + xs[col];
                let cx1 = pad_left + xs[col + 1];
                let cx = (cx0 + cx1) / 2;
                for row in 0..n_rows.saturating_sub(1) {
                    let a = data[row * n_cols + col];
                    let b = data[(row + 1) * n_cols + col];
                    if (a - v_thr) * (b - v_thr) < 0.0 {
                        let ry0 = pad_top + ys[row];
                        let ry2 = pad_top + ys[row + 2];
                        push_b(&mut buf, b"<line x1=\"");
                        push_i(&mut buf, cx0);
                        push_b(&mut buf, b"\" y1=\"");
                        push_i(&mut buf, (ry0 + ry2) / 2);
                        push_b(&mut buf, b"\" x2=\"");
                        push_i(&mut buf, cx1);
                        push_b(&mut buf, b"\" y2=\"");
                        push_i(&mut buf, (ry0 + ry2) / 2);
                        push_b(&mut buf, b"\" stroke=\"#ffffff\" stroke-width=\"1.2\" stroke-opacity=\"0.85\"/>");
                        let _ = cx;
                    }
                }
            }
        }
    }

    if !cfg.categorical {
        let n_steps = if cfg.discrete_steps > 0 { cfg.discrete_steps.max(2) } else { 64 };
        let (lo_v, hi_v) = if cfg.diverging {
            let m = vmin.abs().max(vmax.abs());
            (-m, m)
        } else {
            (vmin, vmax)
        };
        let mid_v = (lo_v + hi_v) / 2.0;
        if right_bar {
            let bar_x = svg_w - pad_right + 14;
            let bar_w = 16;
            let bar_y0 = pad_top;
            let bar_y1 = svg_h - pad_bottom;
            let bar_h = (bar_y1 - bar_y0).max(20);
            let step_h = bar_h as f64 / n_steps as f64;
            for si in 0..n_steps {
                let t_top = 1.0 - si as f64 / (n_steps - 1).max(1) as f64;
                let color = cell_color(t_top, cfg);
                let hx = hex6(color);
                let sy = bar_y0 + (si as f64 * step_h) as i32;
                push_b(&mut buf, b"<rect x=\"");
                push_i(&mut buf, bar_x);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, sy);
                push_b(&mut buf, b"\" width=\"");
                push_i(&mut buf, bar_w);
                push_b(&mut buf, b"\" height=\"");
                push_i(&mut buf, (step_h as i32 + 1).max(1));
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke=\"none\"/>");
            }
            push_b(&mut buf, b"<rect x=\"");
            push_i(&mut buf, bar_x);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, bar_y0);
            push_b(&mut buf, b"\" width=\"");
            push_i(&mut buf, bar_w);
            push_b(&mut buf, b"\" height=\"");
            push_i(&mut buf, bar_h);
            push_b(&mut buf, b"\" fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"0.6\"/>");
            let n_ticks = 5;
            for ti in 0..n_ticks {
                let frac = ti as f64 / (n_ticks - 1) as f64;
                let v = hi_v - frac * (hi_v - lo_v);
                let ty = bar_y0 + (frac * bar_h as f64) as i32;
                push_b(&mut buf, b"<line x1=\"");
                push_i(&mut buf, bar_x + bar_w);
                push_b(&mut buf, b"\" y1=\"");
                push_i(&mut buf, ty);
                push_b(&mut buf, b"\" x2=\"");
                push_i(&mut buf, bar_x + bar_w + 4);
                push_b(&mut buf, b"\" y2=\"");
                push_i(&mut buf, ty);
                push_b(&mut buf, b"\" stroke=\"#64748b\" stroke-width=\"0.8\"/>");
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, bar_x + bar_w + 7);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, ty + 3);
                push_b(&mut buf, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">");
                push_f2(&mut buf, v);
                push_b(&mut buf, b"</text>");
            }
            if cfg.log_scale {
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, bar_x + bar_w / 2);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, bar_y0 - 6);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6366f1\" font-weight=\"700\">log</text>");
            }
        } else {
            let scale_y: i32 = svg_h - pad_bottom + 8;
            let scale_x0: f64 = pad_left as f64;
            let scale_w: f64 = (svg_w - pad_left - pad_right) as f64;
            let step_w = scale_w / n_steps as f64;
            for si in 0..n_steps {
                let t = si as f64 / (n_steps - 1).max(1) as f64;
                let color = cell_color(t, cfg);
                let hx = hex6(color);
                let sx = scale_x0 + si as f64 * step_w;
                push_b(&mut buf, b"<rect x=\"");
                push_i(&mut buf, sx as i32);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, scale_y);
                push_b(&mut buf, b"\" width=\"");
                push_i(&mut buf, (step_w as i32 + 1).max(1));
                push_b(&mut buf, b"\" height=\"12\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke=\"none\"/>");
            }
            let label_y = scale_y + 22;
            let scale_pw = svg_w - pad_left - pad_right;
            let label_static = b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">";
            for (lx, lv) in [(pad_left, lo_v), (pad_left + scale_pw / 2, mid_v), (pad_left + scale_pw, hi_v)] {
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, lx);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, label_y);
                buf.extend_from_slice(label_static);
                push_f2(&mut buf, lv);
                push_b(&mut buf, b"</text>");
            }
            if cfg.log_scale {
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, svg_w - pad_right);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, scale_y - 4);
                push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6366f1\" font-weight=\"700\">log scale</text>");
            }
        }
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let hover_json = if auto_hover { "[]".to_string() } else { slots_to_json(cfg.hover) };
    build_chart_html(cfg.title, &svg, &hover_json)
}
