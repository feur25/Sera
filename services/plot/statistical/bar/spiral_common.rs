use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, lerp_color, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title, truncate,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

fn mix2(a: u32, b: u32) -> u32 {
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    (((ar + br) / 2) << 16) | (((ag + bg) / 2) << 8) | ((ab + bb) / 2)
}

#[allow(clippy::too_many_arguments)]
fn overlay_wedge(buf: &mut Vec<u8>, cx: f64, cy: f64, a0: f64, a1: f64, r0: f64, r1: f64, color: u32, data_idx: i32, value: f64, label: &str) {
    let x00 = cx + r0 * a0.cos();
    let y00 = cy + r0 * a0.sin();
    let x01 = cx + r1 * a0.cos();
    let y01 = cy + r1 * a0.sin();
    let x11 = cx + r1 * a1.cos();
    let y11 = cy + r1 * a1.sin();
    let x10 = cx + r0 * a1.cos();
    let y10 = cy + r0 * a1.sin();
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" d=\"M");
    push_f2(buf, x00);
    push_b(buf, b",");
    push_f2(buf, y00);
    push_b(buf, b" L");
    push_f2(buf, x01);
    push_b(buf, b",");
    push_f2(buf, y01);
    push_b(buf, b" L");
    push_f2(buf, x11);
    push_b(buf, b",");
    push_f2(buf, y11);
    push_b(buf, b" L");
    push_f2(buf, x10);
    push_b(buf, b",");
    push_f2(buf, y10);
    push_b(buf, b" Z\" fill=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" fill-opacity=\"0.74\"/>");
}

fn dashed_arc(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64) {
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    push_b(buf, b"<path fill=\"none\" stroke=\"#cbd5e1\" stroke-width=\"1\" stroke-dasharray=\"1,3\" d=\"M");
    push_f2(buf, x0);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b" A");
    push_f2(buf, r);
    push_b(buf, b",");
    push_f2(buf, r);
    push_b(buf, b" 0 0,1 ");
    push_f2(buf, x1);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"\"/>");
}

fn legend_block(buf: &mut Vec<u8>, x: f64, y: f64, header: &str, entries: &[(String, Vec<f64>)], color_of: impl Fn(usize) -> u32) {
    push_b(buf, b"<text x=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y - 10.0);
    push_b(buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">");
    escape_xml(buf, header);
    push_b(buf, b"</text>");
    for (j, (name, _)) in entries.iter().enumerate() {
        let sy = y + j as f64 * 13.0;
        let color = color_of(j);
        push_b(buf, b"<rect x=\"");
        push_f2(buf, x);
        push_b(buf, b"\" y=\"");
        push_f2(buf, sy - 7.0);
        push_b(buf, b"\" width=\"9\" height=\"9\" rx=\"2\" fill=\"#");
        buf.extend_from_slice(&hex6(color));
        push_b(buf, b"\"/><text x=\"");
        push_f2(buf, x + 13.0);
        push_b(buf, b"\" y=\"");
        push_f2(buf, sy);
        push_b(buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.5\" fill=\"#475569\">");
        escape_xml(buf, name);
        push_b(buf, b"</text>");
    }
}

pub(crate) fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let per_rev = ((n as f64 / 3.2).ceil() as usize).clamp(12, 48);
    let angle_step = TAU / per_rev as f64;

    let mut vmax = f64::NEG_INFINITY;
    let mut vmin = f64::INFINITY;
    for &v in &cfg.values[..n] {
        vmax = vmax.max(v);
        vmin = vmin.min(v);
    }
    let vr = (vmax - vmin).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0 + 20.0;
    let r_max = (w.min(h) as f64) * 0.42;
    let r_hub = r_max * 0.06;
    let bar_max = (r_max - r_hub) * 0.28;
    let spiral_span = r_max - r_hub - bar_max;
    let growth = spiral_span / n as f64;

    let r_base = |i: usize| -> f64 { r_hub + growth * i as f64 };
    let theta = |i: usize| -> f64 { -FRAC_PI_2 + angle_step * i as f64 };

    let mid_color = mix2(cfg.color_low, cfg.color_high);

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 8192);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 24);

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"0.6\">");
    let n_spokes = (per_rev / 2).clamp(6, 16);
    for k in 0..n_spokes {
        let a = -FRAC_PI_2 + TAU * k as f64 / n_spokes as f64;
        let x0 = cx + r_hub * a.cos();
        let y0 = cy + r_hub * a.sin();
        let x1 = cx + r_max * a.cos();
        let y1 = cy + r_max * a.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<path fill=\"none\" stroke=\"#cbd5e1\" stroke-width=\"1\" stroke-dasharray=\"1,3\" d=\"M");
    for i in 0..n {
        let r = r_base(i);
        let a = theta(i);
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        if i > 0 {
            push_b(&mut buf, b" L");
        }
        push_f2(&mut buf, x);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y);
    }
    push_b(&mut buf, b"\"/>");

    let half_step = angle_step / 2.0;
    for i in 0..n {
        let a = theta(i);
        let slot_start = a - half_step;
        let rb = r_base(i);
        let value = cfg.values[i];
        let t = (value - vmin) / vr;
        let bar_len = 2.0 + t * (bar_max - 2.0);
        let re = rb + bar_len;
        let color = lerp_color(t, cfg.color_low, mid_color, cfg.color_high);
        overlay_wedge(&mut buf, cx, cy, slot_start, slot_start + angle_step, rb, re, color, i as i32, value, &cfg.labels[i]);
        push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&hex6(color));
        push_b(&mut buf, b"\" stroke-width=\"0.4\" stroke-opacity=\"0.5\" d=\"M");
        let x00 = cx + rb * slot_start.cos();
        let y00 = cy + rb * slot_start.sin();
        push_f2(&mut buf, x00);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y00);
        push_b(&mut buf, b"\"/>");

        if i % per_rev == 0 {
            let r_lab = rb - 8.0;
            let xl = cx + r_lab * a.cos();
            let yl = cy + r_lab * a.sin();
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, xl);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, yl + 3.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.5\" fill=\"#64748b\">");
            escape_xml(&mut buf, truncate(&cfg.labels[i], 10));
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    let leg_x = 20.0;
    let leg_y = h as f64 - 46.0;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y - 10.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">LONGUEUR = VALEUR</text>");
    let ref_ts = [0.0, 0.5, 1.0];
    for (k, &t) in ref_ts.iter().enumerate() {
        let bl = 2.0 + t * (bar_max - 2.0);
        let x0 = leg_x + k as f64 * 66.0;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x0 + bl);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"4\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y + 14.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
        let val = vmin + t * vr;
        let s = format!("{:.0}", val);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    let grad_x = w as f64 - 150.0;
    let grad_y = h as f64 - 46.0;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, grad_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y - 10.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">COULEUR = VALEUR</text>");
    push_b(&mut buf, b"<defs><linearGradient id=\"spSpiralGrad\" x1=\"0\" x2=\"1\" y1=\"0\" y2=\"0\">");
    push_b(&mut buf, b"<stop offset=\"0%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(cfg.color_low));
    push_b(&mut buf, b"\"/><stop offset=\"50%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(mid_color));
    push_b(&mut buf, b"\"/><stop offset=\"100%\" stop-color=\"#");
    buf.extend_from_slice(&hex6(cfg.color_high));
    push_b(&mut buf, b"\"/></linearGradient></defs>");
    push_b(&mut buf, b"<rect x=\"");
    push_f2(&mut buf, grad_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y - 6.0);
    push_b(&mut buf, b"\" width=\"120\" height=\"7\" fill=\"url(#spSpiralGrad)\" rx=\"2\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, grad_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y + 14.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
    let smin = format!("{:.0}", vmin);
    buf.extend_from_slice(smin.as_bytes());
    push_b(&mut buf, b"</text><text x=\"");
    push_f2(&mut buf, grad_x + 120.0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, grad_y + 14.0);
    push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
    let smax = format!("{:.0}", vmax);
    buf.extend_from_slice(smax.as_bytes());
    push_b(&mut buf, b"</text>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = crate::html::hover::slots_to_json(cfg.hover);
        &slots_json
    };
    crate::html::hover::build_chart_html(cfg.title, &svg, json)
}

const EFFIC_LEVELS: [&str; 4] = ["Pas du tout efficace", "Plutot pas efficace", "Plutot efficace", "Tres efficace"];

pub(crate) fn render_dual(cfg: &BarConfig) -> String {
    let n_series = cfg.series.len().min(cfg.series2.len());
    let n = cfg
        .labels
        .len()
        .min(cfg.series.iter().map(|(_, v)| v.len()).min().unwrap_or(0))
        .min(cfg.series2.iter().map(|(_, v)| v.len()).min().unwrap_or(0));
    if n < 2 || n_series == 0 {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let wf = w as f64;
    let hf = h as f64;

    let per_rev = ((n as f64 / 3.2).ceil() as usize).clamp(12, 48);
    let angle_step = TAU / per_rev as f64;
    let half_step = angle_step / 2.0;

    let cx1 = wf * 0.235;
    let cy1 = hf * 0.460;
    let r_max1 = hf * 0.300;
    let r_hub1 = r_max1 * 0.07;
    let bar_max1 = (r_max1 - r_hub1) * 0.26;
    let spiral_span1 = r_max1 - r_hub1 - bar_max1;
    let growth1 = spiral_span1 / n as f64;
    let r_base1 = |i: usize| -> f64 { r_hub1 + growth1 * i as f64 };
    let theta1 = |i: usize| -> f64 { -FRAC_PI_2 + angle_step * i as f64 };

    let mut vmax1 = f64::NEG_INFINITY;
    let mut vmin1 = f64::INFINITY;
    for (_, vals) in cfg.series {
        for &v in &vals[..n] {
            vmax1 = vmax1.max(v);
            vmin1 = vmin1.min(v);
        }
    }
    let vr1 = (vmax1 - vmin1).max(1e-9);

    let cx2 = wf * 0.860;
    let cy2 = hf * 0.375;
    let fan_r_max = hf * 0.240;
    let fan_r_hub = fan_r_max * 0.08;
    let fan_start = -PI;
    let fan_end = -FRAC_PI_2;
    let fan_step = (fan_end - fan_start) / n as f64;
    let fan_half = fan_step * 0.5 * 0.82;
    let fan_center = |i: usize| -> f64 { fan_start + fan_step * (i as f64 + 0.5) };

    let spine_p0 = (wf * 0.465, hf * 0.880);
    let spine_p1 = (wf * 0.674, hf * 0.300);
    let sdx = spine_p1.0 - spine_p0.0;
    let sdy = spine_p1.1 - spine_p0.1;
    let spine_len = (sdx * sdx + sdy * sdy).sqrt().max(1e-6);
    let ux = sdx / spine_len;
    let uy = sdy / spine_len;
    let row_gap = spine_len / (n - 1) as f64;
    let spine_deg = sdy.atan2(sdx).to_degrees();
    let px = -uy;
    let py = ux;

    let mut buf = Vec::<u8>::with_capacity(n * n_series * 260 + 12_000);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 20);

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"0.6\">");
    let n_spokes = (per_rev / 2).clamp(6, 14);
    for k in 0..n_spokes {
        let a = -FRAC_PI_2 + TAU * k as f64 / n_spokes as f64;
        let x0 = cx1 + r_hub1 * a.cos();
        let y0 = cy1 + r_hub1 * a.sin();
        let x1 = cx1 + r_max1 * a.cos();
        let y1 = cy1 + r_max1 * a.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<path fill=\"none\" stroke=\"#cbd5e1\" stroke-width=\"1\" stroke-dasharray=\"1,3\" d=\"M");
    for i in 0..n {
        let r = r_base1(i);
        let a = theta1(i);
        let x = cx1 + r * a.cos();
        let y = cy1 + r * a.sin();
        if i > 0 {
            push_b(&mut buf, b" L");
        }
        push_f2(&mut buf, x);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y);
    }
    push_b(&mut buf, b"\"/>");

    push_b(&mut buf, b"<g style=\"isolation:isolate\">");
    for i in 0..n {
        let a = theta1(i);
        let slot_start = a - half_step;
        let rb = r_base1(i);
        for j in 0..n_series {
            let value = cfg.series[j].1[i];
            let t = ((value - vmin1) / vr1).clamp(0.0, 1.0);
            let bar_len = 2.0 + t * (bar_max1 - 2.0);
            let re = rb + bar_len;
            let color = palette_color(cfg.palette, j);
            let mut lbl = cfg.labels[i].clone();
            lbl.push_str(" \u{2014} ");
            lbl.push_str(&cfg.series[j].0);
            overlay_wedge(&mut buf, cx1, cy1, slot_start, slot_start + angle_step, rb, re, color, (i * n_series + j) as i32, value, &lbl);
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx1);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy1);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub1);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    let fan_pad = fan_step * 0.35;
    for (li, &name) in EFFIC_LEVELS.iter().enumerate() {
        let t = li as f64 / 3.0;
        let r = fan_r_hub + t * (fan_r_max - fan_r_hub);
        dashed_arc(&mut buf, cx2, cy2, r, fan_start - fan_pad, fan_end + fan_pad);
        let a_lab = fan_end + fan_pad + 0.05;
        let xl = cx2 + (r + 3.0) * a_lab.cos();
        let yl = cy2 + (r + 3.0) * a_lab.sin();
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, xl);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, yl);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"6.6\" fill=\"#64748b\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<g style=\"isolation:isolate\">");
    for i in 0..n {
        let a = fan_center(i);
        let a0 = a - fan_half;
        let a1 = a + fan_half;
        for j in 0..n_series {
            let value = cfg.series2[j].1[i];
            let t = ((value - 1.0) / 3.0).clamp(0.0, 1.0);
            let re = fan_r_hub + t * (fan_r_max - fan_r_hub);
            let color = if j == 0 { cfg.color_low } else if j == 1 { cfg.color_high } else { palette_color(cfg.palette, j) };
            let mut lbl = cfg.labels[i].clone();
            lbl.push_str(" \u{2014} ");
            lbl.push_str(&cfg.series2[j].0);
            overlay_wedge(&mut buf, cx2, cy2, a0, a1, fan_r_hub, re, color, (n * n_series + i * n_series + j) as i32, value, &lbl);
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx2);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy2);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, fan_r_hub);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, spine_p0.0);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, spine_p0.1);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, spine_p1.0);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, spine_p1.1);
    push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");

    for i in 0..n {
        let lx = spine_p0.0 + i as f64 * row_gap * ux;
        let ly = spine_p0.1 + i as f64 * row_gap * uy;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" r=\"6.4\" fill=\"#fff\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 2.6);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" font-weight=\"700\" fill=\"#334155\">");
        let s = (i + 1).to_string();
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    let leg_x = 16.0;
    let leg_y = hf * 0.775;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y - 20.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">LONGUEUR = % D'UTILISATION</text>");
    let ref_ts = [0.0, 0.5, 1.0];
    for (k, &t) in ref_ts.iter().enumerate() {
        let bl = 2.0 + t * (bar_max1 - 2.0);
        let x0 = leg_x + k as f64 * 60.0;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, leg_y - 6.0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x0 + bl);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, leg_y - 6.0);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"4\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y + 6.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7\" fill=\"#94a3b8\">");
        let val = vmin1 + t * vr1;
        let s = format!("{:.0}%", val);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
    }

    legend_block(&mut buf, 16.0, 56.0, "MOYENS", cfg.series, |j| palette_color(cfg.palette, j));
    legend_block(&mut buf, 16.0, 56.0 + n_series as f64 * 13.0 + 16.0, "EFFICACITE", cfg.series2, |j| if j == 0 { cfg.color_low } else if j == 1 { cfg.color_high } else { palette_color(cfg.palette, j) });

    let idx_cols = 3.0;
    let idx_x0 = 16.0;
    let idx_row_h = 12.4;
    let idx_col_w = (wf - 32.0) / idx_cols;
    let idx_y0 = hf * 0.855;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, idx_x0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, idx_y0 - 8.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.6\" font-weight=\"700\" fill=\"#475569\">MOYENS ET NIVEAUX D'EFFICACITE, PAR NUMERO</text>");
    for i in 0..n {
        let col = (i as f64 / (n as f64 / idx_cols).ceil()).floor();
        let row = i as f64 - col * (n as f64 / idx_cols).ceil();
        let ix = idx_x0 + col * idx_col_w;
        let iy = idx_y0 + row * idx_row_h;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, ix);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, iy);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.2\" fill=\"#475569\"><tspan font-weight=\"700\">");
        let s = (i + 1).to_string();
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b".</tspan> ");
        escape_xml(&mut buf, truncate(&cfg.labels[i], 40));
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = crate::html::hover::slots_to_json(cfg.hover);
        &slots_json
    };
    crate::html::hover::build_chart_html(cfg.title, &svg, json)
}
