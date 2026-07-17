use super::config::VennConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo("labels=[\"Set A\",\"Set B\",\"Set C\"], values=[40,30,20,15,10,5,8]")]
pub fn render(cfg: &VennConfig) -> String {
    render_impl(cfg, false, false, false, false)
}

pub fn render_euler(cfg: &VennConfig)    -> String { render_impl(cfg, true,  false, false, false) }
pub fn render_filled(cfg: &VennConfig)   -> String { render_impl(cfg, false, true,  false, false) }
pub fn render_gradient(cfg: &VennConfig) -> String { render_impl(cfg, false, false, true,  false) }
pub fn render_minimal(cfg: &VennConfig)  -> String { render_impl(cfg, false, false, false, false)  }
pub fn render_exclusive(cfg: &VennConfig) -> String { render_impl(cfg, false, false, false, true)  }

struct SetCircle {
    cx: f64,
    cy: f64,
    r:  f64,
    ci: usize,
}

fn layout(n: usize, cx: f64, cy: f64, base_r: f64, euler: bool, values: &[f64]) -> Vec<SetCircle> {
    use std::f64::consts::PI;
    if n == 0 { return Vec::new(); }

    let radii: Vec<f64> = if euler && !values.is_empty() {
        let max_v = values[..n].iter().copied().fold(0.0f64, f64::max).max(1.0);
        (0..n).map(|i| {
            let v = values.get(i).copied().unwrap_or(1.0);
            base_r * (v / max_v).sqrt().max(0.35)
        }).collect()
    } else {
        vec![base_r; n]
    };

    let mut circles = Vec::with_capacity(n);
    match n {
        1 => circles.push(SetCircle { cx, cy, r: radii[0], ci: 0 }),
        2 => {
            let off = radii[0] * 0.55;
            circles.push(SetCircle { cx: cx - off, cy, r: radii[0], ci: 0 });
            circles.push(SetCircle { cx: cx + off, cy, r: radii[1], ci: 1 });
        }
        3 => {
            let off = base_r * 0.5;
            circles.push(SetCircle { cx: cx - off, cy: cy + off * 0.5, r: radii[0], ci: 0 });
            circles.push(SetCircle { cx: cx + off, cy: cy + off * 0.5, r: radii[1], ci: 1 });
            circles.push(SetCircle { cx, cy: cy - off * 0.65, r: radii[2], ci: 2 });
        }
        _ => {
            for i in 0..n {
                let angle = 2.0 * PI * i as f64 / n as f64 - PI / 2.0;
                let orbit = base_r * 0.55;
                circles.push(SetCircle {
                    cx: cx + orbit * angle.cos(),
                    cy: cy + orbit * angle.sin(),
                    r:  radii[i],
                    ci: i,
                });
            }
        }
    }
    circles
}

fn render_impl(cfg: &VennConfig, euler: bool, filled: bool, gradient: bool, exclusive: bool) -> String {
    let n = cfg.labels.len();
    if n == 0 { return String::new(); }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let title_off = if cfg.title.is_empty() { 0.0 } else { 22.0 };
    let cx = w / 2.0;
    let cy = (h + title_off) / 2.0;
    let base_r = ((w.min(h) - title_off) / 2.0 - 24.0).max(40.0) * 0.72;

    let circles = layout(n, cx, cy, base_r, euler, cfg.values);

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n * 300 + 4096);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if gradient {
        push_b(&mut buf, b"<defs>");
        for (i, c) in circles.iter().enumerate() {
            let color = palette_color(cfg.palette, c.ci);
            let hx = hex6(color);
            push_b(&mut buf, b"<radialGradient id=\"vg");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\"><stop offset=\"0%\" stop-color=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stop-opacity=\"0.65\"/><stop offset=\"100%\" stop-color=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stop-opacity=\"0.15\"/></radialGradient>");
        }
        push_b(&mut buf, b"</defs>");
    }

    if exclusive {
        push_b(&mut buf, b"<defs>");
        for (i, c) in circles.iter().enumerate() {
            push_b(&mut buf, b"<mask id=\"vex");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" maskUnits=\"userSpaceOnUse\"><rect x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" fill=\"black\"/><circle cx=\"");
            push_f2(&mut buf, c.cx);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, c.cy);
            push_b(&mut buf, b"\" r=\"");
            push_f2(&mut buf, c.r);
            push_b(&mut buf, b"\" fill=\"white\"/>");
            for (j, o) in circles.iter().enumerate() {
                if j == i { continue; }
                push_b(&mut buf, b"<circle cx=\"");
                push_f2(&mut buf, o.cx);
                push_b(&mut buf, b"\" cy=\"");
                push_f2(&mut buf, o.cy);
                push_b(&mut buf, b"\" r=\"");
                push_f2(&mut buf, o.r);
                push_b(&mut buf, b"\" fill=\"black\"/>");
            }
            push_b(&mut buf, b"</mask>");
        }
        push_b(&mut buf, b"</defs>");
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, w / 2.0);
        push_b(&mut buf, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for (i, c) in circles.iter().enumerate() {
        let color = palette_color(cfg.palette, c.ci);
        let hx = hex6(color);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, c.cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, c.cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, c.r);
        push_b(&mut buf, b"\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"");
        if gradient {
            push_b(&mut buf, b" fill=\"url(#vg");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b")\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"1.5\" stroke-opacity=\"0.6\"");
        } else if exclusive {
            push_b(&mut buf, b" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.14\" stroke=\"none\"");
        } else if filled {
            push_b(&mut buf, b" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.55\" stroke=\"none\"");
        } else if cfg.opacity < 0.05 {
            push_b(&mut buf, b" fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"2\"");
        } else {
            push_b(&mut buf, b" fill=\"#");
            buf.extend_from_slice(&hx);
            let op_bytes = format!("\" fill-opacity=\"{:.2}\" stroke=\"#", cfg.opacity);
            buf.extend_from_slice(op_bytes.as_bytes());
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"1.5\" stroke-opacity=\"0.5\"");
        }
        push_b(&mut buf, b"/>");
    }

    if exclusive {
        for (i, c) in circles.iter().enumerate() {
            let color = palette_color(cfg.palette, c.ci);
            let hx = hex6(color);
            push_b(&mut buf, b"<circle cx=\"");
            push_f2(&mut buf, c.cx);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, c.cy);
            push_b(&mut buf, b"\" r=\"");
            push_f2(&mut buf, c.r);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.7\" mask=\"url(#vex");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b")\"/>");
        }
    }

    if cfg.show_labels {
        for (i, c) in circles.iter().enumerate() {
            let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
            let dx = c.cx - cx;
            let dy = c.cy - cy;
            let len = (dx * dx + dy * dy).sqrt();
            let (lx, ly) = if len < 1.0 {
                (c.cx, c.cy - c.r * 0.5)
            } else {
                (c.cx + dx / len * c.r * 0.72, c.cy + dy / len * c.r * 0.72)
            };

            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly + 4.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#1e293b\">");
            escape_xml(&mut buf, label);
            push_b(&mut buf, b"</text>");

            if let Some(&v) = cfg.values.get(i) {
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, lx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, ly + 17.0);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">");
                let vs = format!("{}", v as i64);
                buf.extend_from_slice(vs.as_bytes());
                push_b(&mut buf, b"</text>");
            }
        }
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}
