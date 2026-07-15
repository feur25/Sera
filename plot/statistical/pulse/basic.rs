use super::config::PulseConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], values=[0.4,0.7,0.9,0.6,0.8,0.3,0.5]")]
pub fn render(cfg: &PulseConfig) -> String {
    render_impl(cfg, false, false, false)
}

pub fn render_wave(cfg: &PulseConfig)     -> String { render_wave_impl(cfg) }
pub fn render_dot(cfg: &PulseConfig)      -> String { render_dot_impl(cfg) }
pub fn render_filled(cfg: &PulseConfig)   -> String { render_impl(cfg, true,  false, false) }
pub fn render_gradient(cfg: &PulseConfig) -> String { render_impl(cfg, false, false, true)  }

fn render_impl(cfg: &PulseConfig, filled: bool, _wave: bool, gradient: bool) -> String {
    use std::f64::consts::PI;
    let n = cfg.labels.len().max(cfg.values.len());
    if n == 0 { return String::new(); }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let title_h = if cfg.title.is_empty() { 0.0 } else { 22.0 };
    let cx = w / 2.0;
    let cy = (h + title_h) / 2.0;
    let outer_r = (w.min(h) / 2.0 - 52.0 - title_h / 2.0).max(40.0);
    let inner_r = cfg.inner_r.min(outer_r * 0.35);

    let max_v = cfg.values.iter().copied().fold(0.0f64, f64::max).max(1e-9);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 200 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, inner_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, outer_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.5\"/>");

    let slice_angle = 2.0 * PI / n as f64;

    if gradient {
        push_b(&mut buf, b"<defs>");
        for i in 0..n {
            let color = palette_color(cfg.palette, i);
            let hx = hex6(color);
            push_b(&mut buf, b"<radialGradient id=\"pg");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" cx=\"50%\" cy=\"50%\" r=\"50%\"><stop offset=\"0%\" stop-color=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stop-opacity=\"0.2\"/><stop offset=\"100%\" stop-color=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stop-opacity=\"0.85\"/></radialGradient>");
        }
        push_b(&mut buf, b"</defs>");
    }

    for i in 0..n {
        let v = cfg.values.get(i).copied().unwrap_or(0.0) / max_v;
        let bar_r = inner_r + v * (outer_r - inner_r);
        let angle_start = -PI / 2.0 + i as f64 * slice_angle;
        let angle_end   = angle_start + slice_angle * 0.82;
        let mid_angle   = (angle_start + angle_end) / 2.0;

        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);

        let x1i = cx + inner_r * angle_start.cos();
        let y1i = cy + inner_r * angle_start.sin();
        let x2i = cx + inner_r * angle_end.cos();
        let y2i = cy + inner_r * angle_end.sin();
        let x1o = cx + bar_r * angle_start.cos();
        let y1o = cy + bar_r * angle_start.sin();
        let x2o = cx + bar_r * angle_end.cos();
        let y2o = cy + bar_r * angle_end.sin();

        push_b(&mut buf, b"<path fill=\"");
        if gradient {
            push_b(&mut buf, b"url(#pg");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b")\"");
        } else {
            push_b(&mut buf, b"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"");
            if filled { push_b(&mut buf, b"0.75\""); } else { push_b(&mut buf, b"0.60\""); }
        }
        push_b(&mut buf, b" stroke=\"#fff\" stroke-width=\"0.5\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" d=\"M");
        push_f2(&mut buf, x1i); push_b(&mut buf, b","); push_f2(&mut buf, y1i);
        push_b(&mut buf, b"A"); push_f2(&mut buf, inner_r); push_b(&mut buf, b","); push_f2(&mut buf, inner_r);
        push_b(&mut buf, b" 0 0,1 "); push_f2(&mut buf, x2i); push_b(&mut buf, b","); push_f2(&mut buf, y2i);
        push_b(&mut buf, b"L"); push_f2(&mut buf, x2o); push_b(&mut buf, b","); push_f2(&mut buf, y2o);
        push_b(&mut buf, b"A"); push_f2(&mut buf, bar_r); push_b(&mut buf, b","); push_f2(&mut buf, bar_r);
        push_b(&mut buf, b" 0 0,0 "); push_f2(&mut buf, x1o); push_b(&mut buf, b","); push_f2(&mut buf, y1o);
        push_b(&mut buf, b"Z\"/>");

        if cfg.show_labels {
            let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
            let label_r = outer_r + 16.0;
            let lx = cx + label_r * mid_angle.cos();
            let ly = cy + label_r * mid_angle.sin();
            let anchor: &[u8] = if mid_angle.cos() > 0.15 { b"start" }
                else if mid_angle.cos() < -0.15 { b"end" }
                else { b"middle" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#374151\">");
            escape_xml(&mut buf, label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn render_wave_impl(cfg: &PulseConfig) -> String {
    use std::f64::consts::PI;
    let n = cfg.values.len();
    if n < 2 { return render_impl(cfg, false, false, false); }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let title_h = if cfg.title.is_empty() { 0.0 } else { 22.0 };
    let cx = w / 2.0;
    let cy = (h + title_h) / 2.0;
    let outer_r = (w.min(h) / 2.0 - 52.0 - title_h / 2.0).max(40.0);
    let inner_r = cfg.inner_r.min(outer_r * 0.35);
    let max_v = cfg.values.iter().copied().fold(0.0f64, f64::max).max(1e-9);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 80 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx); push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, inner_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");

    let color0 = palette_color(cfg.palette, 0);
    let hx0 = hex6(color0);

    let mut path_pts: Vec<(f64, f64)> = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let k = i % n;
        let v = cfg.values[k] / max_v;
        let r = inner_r + v * (outer_r - inner_r);
        let angle = -PI / 2.0 + 2.0 * PI * i as f64 / n as f64;
        path_pts.push((cx + r * angle.cos(), cy + r * angle.sin()));
    }

    push_b(&mut buf, b"<path fill=\"#");
    buf.extend_from_slice(&hx0);
    push_b(&mut buf, b"\" fill-opacity=\"0.25\" stroke=\"#");
    buf.extend_from_slice(&hx0);
    push_b(&mut buf, b"\" stroke-width=\"2\" d=\"M");
    push_f2(&mut buf, path_pts[0].0); push_b(&mut buf, b","); push_f2(&mut buf, path_pts[0].1);

    for i in 1..path_pts.len() {
        let (px, py) = path_pts[i - 1];
        let (nx, ny) = path_pts[i];
        let cpx = (px + nx) / 2.0;
        let cpy = (py + ny) / 2.0;
        push_b(&mut buf, b"Q"); push_f2(&mut buf, px); push_b(&mut buf, b","); push_f2(&mut buf, py);
        push_b(&mut buf, b" "); push_f2(&mut buf, cpx); push_b(&mut buf, b","); push_f2(&mut buf, cpy);
    }
    push_b(&mut buf, b"Z\"/>");

    if cfg.show_labels {
        for i in 0..n {
            let v = cfg.values[i] / max_v;
            let r = inner_r + v * (outer_r - inner_r) + 12.0;
            let angle = -PI / 2.0 + 2.0 * PI * i as f64 / n as f64;
            let lx = cx + r * angle.cos();
            let ly = cy + r * angle.sin();
            let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
            let anchor: &[u8] = if angle.cos() > 0.15 { b"start" } else if angle.cos() < -0.15 { b"end" } else { b"middle" };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#374151\">");
            escape_xml(&mut buf, label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn render_dot_impl(cfg: &PulseConfig) -> String {
    use std::f64::consts::PI;
    let n = cfg.values.len();
    if n == 0 { return String::new(); }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let title_h = if cfg.title.is_empty() { 0.0 } else { 22.0 };
    let cx = w / 2.0;
    let cy = (h + title_h) / 2.0;
    let outer_r = (w.min(h) / 2.0 - 52.0 - title_h / 2.0).max(40.0);
    let inner_r = cfg.inner_r.min(outer_r * 0.35);
    let max_v = cfg.values.iter().copied().fold(0.0f64, f64::max).max(1e-9);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 120 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx); push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, inner_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx); push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, outer_r);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#f1f5f9\" stroke-width=\"0.5\"/>");

    for i in 0..n {
        let v = cfg.values[i] / max_v;
        let r = inner_r + v * (outer_r - inner_r);
        let angle = -PI / 2.0 + 2.0 * PI * i as f64 / n as f64;
        let px = cx + r * angle.cos();
        let py = cy + r * angle.sin();
        let dot_r = (v * 6.0 + 2.5).min(10.0);
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);

        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, cx + inner_r * angle.cos());
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, cy + inner_r * angle.sin());
        push_b(&mut buf, b"\" x2=\""); push_f2(&mut buf, px);
        push_b(&mut buf, b"\" y2=\""); push_f2(&mut buf, py);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.35\"/>");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, dot_r);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.8\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"/>");

        if cfg.show_labels {
            let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
            let lr = r + dot_r + 10.0;
            let lx = cx + lr * angle.cos();
            let ly = cy + lr * angle.sin();
            let anchor: &[u8] = if angle.cos() > 0.15 { b"start" } else if angle.cos() < -0.15 { b"end" } else { b"middle" };
            push_b(&mut buf, b"<text x=\""); push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\""); push_f2(&mut buf, ly + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"");
            buf.extend_from_slice(anchor);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#374151\">");
            escape_xml(&mut buf, label);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
