use super::config::AreaConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, sort_indices, sorted, truncate, Frame,
};

#[derive(Clone, Copy, PartialEq)]
pub enum StackMode {
    None,
    Stacked,
    Percent,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Curve {
    Linear,
    Step,
    Spline,
}

pub fn ordered_series(cfg: &AreaConfig) -> Vec<(String, Vec<f64>)> {
    let n_ser = cfg.series.len();
    if cfg.sort_order != "none" && !cfg.sort_order.is_empty() && n_ser > 1 {
        let totals: Vec<f64> = cfg
            .series
            .iter()
            .map(|(_, v)| v.iter().sum::<f64>())
            .collect();
        let names: Vec<String> = cfg.series.iter().map(|(n, _)| n.clone()).collect();
        sorted(&sort_indices(n_ser, &totals, &names, cfg.sort_order), cfg.series)
    } else {
        cfg.series.to_vec()
    }
}

fn append_curve(buf: &mut Vec<u8>, pts: &[(f64, f64)], curve: Curve) {
    match curve {
        Curve::Linear => {
            for &(x, y) in pts {
                push_b(buf, b" L");
                push_f2(buf, x);
                buf.push(b',');
                push_f2(buf, y);
            }
        }
        Curve::Step => {
            let mut prev_y = pts[0].1;
            for (i, &(x, y)) in pts.iter().enumerate() {
                if i > 0 {
                    let mid_x = (pts[i - 1].0 + x) / 2.0;
                    push_b(buf, b" L");
                    push_f2(buf, mid_x);
                    buf.push(b',');
                    push_f2(buf, prev_y);
                    push_b(buf, b" L");
                    push_f2(buf, mid_x);
                    buf.push(b',');
                    push_f2(buf, y);
                }
                push_b(buf, b" L");
                push_f2(buf, x);
                buf.push(b',');
                push_f2(buf, y);
                prev_y = y;
            }
        }
        Curve::Spline => {
            let n = pts.len();
            if n < 3 {
                for &(x, y) in pts.iter().skip(1) {
                    push_b(buf, b" L");
                    push_f2(buf, x);
                    buf.push(b',');
                    push_f2(buf, y);
                }
                return;
            }
            for i in 0..n - 1 {
                let p0 = if i == 0 { pts[0] } else { pts[i - 1] };
                let p1 = pts[i];
                let p2 = pts[i + 1];
                let p3 = if i + 2 < n { pts[i + 2] } else { pts[i + 1] };
                let c1x = p1.0 + (p2.0 - p0.0) / 6.0;
                let c1y = p1.1 + (p2.1 - p0.1) / 6.0;
                let c2x = p2.0 - (p3.0 - p1.0) / 6.0;
                let c2y = p2.1 - (p3.1 - p1.1) / 6.0;
                push_b(buf, b" C");
                push_f2(buf, c1x);
                buf.push(b',');
                push_f2(buf, c1y);
                buf.push(b' ');
                push_f2(buf, c2x);
                buf.push(b',');
                push_f2(buf, c2y);
                buf.push(b' ');
                push_f2(buf, p2.0);
                buf.push(b',');
                push_f2(buf, p2.1);
            }
        }
    }
}

pub fn render_with(cfg: &AreaConfig, stack: StackMode, curve: Curve, gradient: bool) -> String {
    render_with_style(cfg, stack, curve, gradient, None, None)
}

#[allow(clippy::too_many_arguments)]
pub fn render_with_style(
    cfg: &AreaConfig,
    stack: StackMode,
    curve: Curve,
    gradient: bool,
    stroke_override: Option<u32>,
    fill_opacity_override: Option<f64>,
) -> String {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser == 0 {
        return String::new();
    }
    let series = ordered_series(cfg);

    let mut stacked_sums: Vec<Vec<f64>> = Vec::with_capacity(n_ser);
    let mut totals: Vec<f64> = vec![0.0; n_pts];
    if stack != StackMode::None {
        let mut running = vec![0.0_f64; n_pts];
        for (_, svals) in series.iter() {
            for i in 0..n_pts {
                let v = svals.get(i).copied().unwrap_or(0.0).max(0.0);
                running[i] += v;
                totals[i] += v;
            }
            stacked_sums.push(running.clone());
        }
    }

    let max_val = match stack {
        StackMode::Percent => 100.0,
        StackMode::Stacked => stacked_sums
            .last()
            .map(|s| s.iter().copied().fold(0.0_f64, f64::max))
            .unwrap_or(1.0)
            .max(1.0),
        StackMode::None => series
            .iter()
            .flat_map(|(_, v)| v.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max)
            .max(1.0),
    };

    let value_at = |si: usize, i: usize| -> f64 {
        match stack {
            StackMode::Stacked => stacked_sums[si][i],
            StackMode::Percent => {
                if totals[i] > 0.0 {
                    stacked_sums[si][i] / totals[i] * 100.0
                } else {
                    0.0
                }
            }
            StackMode::None => series[si].1.get(i).copied().unwrap_or(0.0),
        }
    };
    let base_val_at = |si: usize, i: usize| -> f64 {
        if si == 0 || stack == StackMode::None {
            return 0.0;
        }
        match stack {
            StackMode::Stacked => stacked_sums[si - 1][i],
            StackMode::Percent => {
                if totals[i] > 0.0 {
                    stacked_sums[si - 1][i] / totals[i] * 100.0
                } else {
                    0.0
                }
            }
            StackMode::None => 0.0,
        }
    };

    let legend_w: i32 = 160;
    let auto_hover = cfg.hover.is_empty();
    let n_total = n_pts * n_ser;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        n_total * 60 + 2048,
    );
    let step_x = f.pw as f64 / (n_pts - 1).max(1) as f64;
    let base_y = (f.pt + f.ph) as f64;
    f.open(cfg.title, true);
    f.y_grid_rc(6, 0.0, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    if gradient {
        push_b(&mut f.buf, b"<defs>");
        for si in 0..n_ser {
            let color = palette_color(cfg.palette, si);
            let hx = hex6(color);
            push_b(&mut f.buf, b"<linearGradient id=\"spAreaG");
            push_i(&mut f.buf, si as i32);
            push_b(&mut f.buf, b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">");
            push_b(&mut f.buf, b"<stop offset=\"0\" stop-color=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stop-opacity=\"0.65\"/>");
            push_b(&mut f.buf, b"<stop offset=\"1\" stop-color=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stop-opacity=\"0.02\"/>");
            push_b(&mut f.buf, b"</linearGradient>");
        }
        push_b(&mut f.buf, b"</defs>");
    }

    for si in (0..n_ser).rev() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let top_pts: Vec<(f64, f64)> = (0..n_pts)
            .map(|i| {
                let x = f.pl as f64 + i as f64 * step_x;
                let frac = (value_at(si, i) / max_val).clamp(0.0, 1.0);
                let y = f.pt as f64 + (1.0 - frac) * f.ph as f64;
                (x, y)
            })
            .collect();

        push_b(&mut f.buf, b"<path data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" d=\"M");
        push_f2(&mut f.buf, top_pts[0].0);
        f.buf.push(b',');
        push_f2(&mut f.buf, top_pts[0].1);
        append_curve(&mut f.buf, &top_pts, curve);

        if stack != StackMode::None && si > 0 {
            let bottom_pts: Vec<(f64, f64)> = (0..n_pts)
                .rev()
                .map(|i| {
                    let x = f.pl as f64 + i as f64 * step_x;
                    let frac = (base_val_at(si, i) / max_val).clamp(0.0, 1.0);
                    let y = f.pt as f64 + (1.0 - frac) * f.ph as f64;
                    (x, y)
                })
                .collect();
            push_b(&mut f.buf, b" L");
            push_f2(&mut f.buf, bottom_pts[0].0);
            f.buf.push(b',');
            push_f2(&mut f.buf, bottom_pts[0].1);
            append_curve(&mut f.buf, &bottom_pts, curve);
        } else {
            push_b(&mut f.buf, b" L");
            push_f2(&mut f.buf, top_pts[n_pts - 1].0);
            f.buf.push(b',');
            push_f2(&mut f.buf, base_y);
            push_b(&mut f.buf, b" L");
            push_f2(&mut f.buf, top_pts[0].0);
            f.buf.push(b',');
            push_f2(&mut f.buf, base_y);
        }
        push_b(&mut f.buf, b" Z\" fill=\"");
        if gradient {
            push_b(&mut f.buf, b"url(#spAreaG");
            push_i(&mut f.buf, si as i32);
            push_b(&mut f.buf, b")");
        } else {
            push_b(&mut f.buf, b"#");
            f.buf.extend_from_slice(&hx);
        }
        push_b(&mut f.buf, b"\" fill-opacity=\"");
        push_f2(
            &mut f.buf,
            fill_opacity_override.unwrap_or(if gradient { 1.0 } else { 0.35 }),
        );
        push_b(&mut f.buf, b"\" stroke=\"#");
        match stroke_override {
            Some(sc) => f.buf.extend_from_slice(&hex6(sc)),
            None => f.buf.extend_from_slice(&hx),
        }
        push_b(&mut f.buf, b"\" stroke-width=\"");
        push_f2(&mut f.buf, if stroke_override.is_some() { 2.0 } else { 1.5 });
        push_b(&mut f.buf, b"\"/>");
    }

    let hover_step = ((n_pts as f64 / 30.0).ceil() as usize).max(1);
    for si in 0..n_ser {
        let (sname, svals) = &series[si];
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let mut sname_esc = Vec::with_capacity(sname.len() + 8);
        escape_xml(&mut sname_esc, sname);
        for i in (0..n_pts).step_by(hover_step) {
            let frac = (value_at(si, i) / max_val).clamp(0.0, 1.0);
            let x = f.pl as f64 + i as f64 * step_x;
            let y = f.pt as f64 + (1.0 - frac) * f.ph as f64;
            let idx = (si * n_pts + i) as i32;
            push_b(&mut f.buf, b"<circle data-series=\"");
            push_i(&mut f.buf, si as i32);
            push_b(&mut f.buf, b"\" data-idx=\"");
            push_i(&mut f.buf, idx);
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, svals.get(i).copied().unwrap_or(0.0));
            push_b(&mut f.buf, b"\" data-lbl=\"");
            f.buf.extend_from_slice(&sname_esc);
            push_b(&mut f.buf, b"\" cx=\"");
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b"\" cy=\"");
            push_f2(&mut f.buf, y);
            push_b(&mut f.buf, b"\" r=\"2.5\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\".8\" opacity=\"0\"/>");
        }
    }

    let tick_step = ((n_pts as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n_pts).step_by(tick_step) {
        let x = f.pl as f64 + i as f64 * step_x;
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 12));
        push_b(&mut f.buf, b"</text>");
    }

    let leg_x = cfg.width - legend_w + 14;
    for (si, (sname, _)) in series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let ly = f.pt + 6 + si as i32 * 18;
        push_b(&mut f.buf, b"<g data-legend=\"1\" style=\"display:none\" data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\"><rect x=\"");
        push_i(&mut f.buf, leg_x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ly);
        push_b(&mut f.buf, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\".5\"/>");
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, leg_x + 16);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ly + 10);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
        escape_xml(&mut f.buf, truncate(sname, 18));
        push_b(&mut f.buf, b"</text></g>");
    }

    let slots_json;
    let json: &str = if auto_hover {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
