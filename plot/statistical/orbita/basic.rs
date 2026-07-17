use super::config::OrbitaConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("series_names=[\"2021\",\"2022\",\"2023\"], labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], matrix=[[0.4,0.7,0.5,0.8],[0.6,0.3,0.9,0.5],[0.8,0.6,0.4,0.7]]")]
pub fn render(cfg: &OrbitaConfig) -> String {
    render_impl(cfg, false, false, false)
}

pub fn render_bubble(cfg: &OrbitaConfig)  -> String { render_impl(cfg, true,  false, false) }
pub fn render_trail(cfg: &OrbitaConfig)   -> String { render_impl(cfg, false, true,  false) }
pub fn render_glow(cfg: &OrbitaConfig)    -> String { render_impl(cfg, false, false, true)  }
pub fn render_minimal(cfg: &OrbitaConfig) -> String { render_impl(cfg, false, false, false) }

fn render_impl(cfg: &OrbitaConfig, bubble: bool, trail: bool, glow: bool) -> String {
    use std::f64::consts::PI;

    let ns = cfg.series_names.len();
    let nc = cfg.labels.len();
    if ns == 0 || nc == 0 || cfg.matrix.len() < ns * nc { return String::new(); }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let title_h = if cfg.title.is_empty() { 0.0 } else { 22.0 };
    let cx = w / 2.0;
    let cy = (h + title_h) / 2.0;

    let max_r = (w.min(h) / 2.0 - 48.0 - title_h / 2.0).max(40.0);
    let orbit_gap = ((max_r - cfg.inner_r) / ns as f64).min(cfg.orbit_gap);
    let max_v = cfg.matrix.iter().copied().fold(0.0f64, f64::max).max(1e-9);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(ns * nc * 150 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if glow {
        push_b(&mut buf, b"<defs>");
        for si in 0..ns {
            let color = palette_color(cfg.palette, si);
            let hx = hex6(color);
            push_b(&mut buf, b"<filter id=\"og");
            push_i(&mut buf, si as i32);
            push_b(&mut buf, b"\"><feGaussianBlur stdDeviation=\"3\" result=\"b\"/><feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter>");
        }
        push_b(&mut buf, b"</defs>");
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for si in 0..ns {
        let orbit_r = cfg.inner_r + (si as f64 + 0.5) * orbit_gap;
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, orbit_r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.8\" stroke-dasharray=\"3,3\"/>");

        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let sname = cfg.series_names.get(si).map(|s| s.as_str()).unwrap_or("");
        let label_r = orbit_r;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx - label_r - 4.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 4.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.8\">");
        escape_xml(&mut buf, sname);
        push_b(&mut buf, b"</text>");
    }

    if nc > 0 {
        let angle_step = 2.0 * PI / nc as f64;
        for ci in 0..nc {
            let angle = -PI / 2.0 + ci as f64 * angle_step;
            let max_orbit = cfg.inner_r + (ns as f64 + 0.2) * orbit_gap;
            push_b(&mut buf, b"<line x1=\"");
            push_f2(&mut buf, cx + cfg.inner_r * 0.6 * angle.cos());
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, cy + cfg.inner_r * 0.6 * angle.sin());
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, cx + max_orbit * angle.cos());
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, cy + max_orbit * angle.sin());
            push_b(&mut buf, b"\" stroke=\"#f1f5f9\" stroke-width=\"0.5\"/>");

            if cfg.show_labels {
                let lx = cx + (max_orbit + 14.0) * angle.cos();
                let ly = cy + (max_orbit + 14.0) * angle.sin();
                let anchor: &[u8] = if angle.cos() > 0.15 { b"start" } else if angle.cos() < -0.15 { b"end" } else { b"middle" };
                let label = cfg.labels.get(ci).map(|s| s.as_str()).unwrap_or("");
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, lx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, ly + 4.0);
                push_b(&mut buf, b"\" text-anchor=\"");
                buf.extend_from_slice(anchor);
                push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"500\" fill=\"#374151\">");
                escape_xml(&mut buf, label);
                push_b(&mut buf, b"</text>");
            }
        }

        for si in 0..ns {
            let orbit_r = cfg.inner_r + (si as f64 + 0.5) * orbit_gap;
            let color = palette_color(cfg.palette, si);
            let hx = hex6(color);
            let angle_step = 2.0 * PI / nc as f64;

            if trail {
                let pts: Vec<(f64, f64)> = (0..nc).map(|ci| {
                    let angle = -PI / 2.0 + ci as f64 * angle_step;
                    let v = cfg.matrix[si * nc + ci] / max_v;
                    let r = orbit_r + (v - 0.5) * orbit_gap * 0.4;
                    (cx + r * angle.cos(), cy + r * angle.sin())
                }).collect();

                if !pts.is_empty() {
                    push_b(&mut buf, b"<path fill=\"none\" stroke=\"#");
                    buf.extend_from_slice(&hx);
                    push_b(&mut buf, b"\" stroke-width=\"1.5\" stroke-opacity=\"0.6\" d=\"M");
                    push_f2(&mut buf, pts[0].0); push_b(&mut buf, b","); push_f2(&mut buf, pts[0].1);
                    for k in 1..pts.len() {
                        push_b(&mut buf, b"L"); push_f2(&mut buf, pts[k].0);
                        push_b(&mut buf, b","); push_f2(&mut buf, pts[k].1);
                    }
                    push_b(&mut buf, b"Z\"/>");
                }
            }

            for ci in 0..nc {
                let v = cfg.matrix[si * nc + ci] / max_v;
                let angle = -PI / 2.0 + ci as f64 * angle_step;
                let px = cx + orbit_r * angle.cos();
                let py = cy + orbit_r * angle.sin();
                let dot_r = if bubble { (v * 8.0 + 2.5).min(14.0) } else { 4.5 };

                push_b(&mut buf, b"<circle cx=\"");
                push_f2(&mut buf, px);
                push_b(&mut buf, b"\" cy=\""); push_f2(&mut buf, py);
                push_b(&mut buf, b"\" r=\""); push_f2(&mut buf, dot_r);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&hx);
                if glow {
                    push_b(&mut buf, b"\" filter=\"url(#og");
                    push_i(&mut buf, si as i32);
                    push_b(&mut buf, b")\"");
                } else {
                    push_b(&mut buf, b"\"");
                }
                push_b(&mut buf, b" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"1.2\" data-idx=\"");
                push_i(&mut buf, (si * nc + ci) as i32);
                push_b(&mut buf, b"\"/>");
            }
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
