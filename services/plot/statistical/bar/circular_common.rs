use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title,
    truncate,
};
use std::f64::consts::{FRAC_PI_2, TAU};

fn unique_groups(color_groups: &[String], n: usize) -> Vec<String> {
    let mut uniq: Vec<String> = Vec::new();
    for g in &color_groups[..color_groups.len().min(n)] {
        if !uniq.iter().any(|u| u == g) {
            uniq.push(g.clone());
        }
    }
    uniq
}

pub(crate) fn render(cfg: &BarConfig, show_labels: bool, show_grid: bool, grouped: bool) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let use_groups = grouped && cfg.color_groups.len() == n;
    let groups = if use_groups {
        unique_groups(cfg.color_groups, n)
    } else {
        vec![String::new()]
    };
    let mut group_indices: Vec<Vec<usize>> = vec![Vec::new(); groups.len()];
    if use_groups {
        for i in 0..n {
            if let Some(gi) = groups.iter().position(|g| *g == cfg.color_groups[i]) {
                group_indices[gi].push(i);
            }
        }
    } else {
        group_indices[0] = (0..n).collect();
    }

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 4096);
    svg_open_rescalable(&mut buf, cfg.width, cfg.height, 0, 0, cfg.width, cfg.height);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, cfg.width / 2, 26);

    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0 + 10.0;
    let r_outer_max = (cfg.width.min(cfg.height) as f64) * 0.42;
    let r_inner = r_outer_max * 0.26;
    let vmax = cfg.values[..n].iter().cloned().fold(0.0_f64, f64::max).max(1e-9);

    if show_grid {
        for ring in 1..=4 {
            let rr = r_inner + (r_outer_max - r_inner) * ring as f64 / 4.0;
            push_b(&mut buf, b"<circle cx=\"");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, cy);
            push_b(&mut buf, b"\" r=\"");
            push_f2(&mut buf, rr);
            push_b(
                &mut buf,
                b"\" fill=\"none\" stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"2,3\"/>",
            );
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, cx + 4.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy - rr - 2.0);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#94a3b8\">");
            push_f2(&mut buf, vmax * ring as f64 / 4.0);
            push_b(&mut buf, b"</text>");
        }
    }

    let n_groups = groups.len();
    let gap_between = if n_groups > 1 { TAU * 0.03 } else { 0.0 };
    let usable = TAU - gap_between * n_groups as f64;
    let mut angle_cursor = -FRAC_PI_2;

    for (gi, idxs) in group_indices.iter().enumerate() {
        let count = idxs.len();
        if count == 0 {
            continue;
        }
        let group_angle = usable * (count as f64 / n as f64);
        let bar_slot = group_angle / count as f64;
        let bar_angle = bar_slot * 0.82;
        let bar_px_width = (bar_angle * r_inner).clamp(3.0, 26.0);

        for (k, &i) in idxs.iter().enumerate() {
            let theta = angle_cursor + bar_slot * (k as f64 + 0.5);
            let v = cfg.values[i];
            let r_end = r_inner + (r_outer_max - r_inner) * (v / vmax).clamp(0.0, 1.0);
            let x0 = cx + r_inner * theta.cos();
            let y0 = cy + r_inner * theta.sin();
            let x1 = cx + r_end * theta.cos();
            let y1 = cy + r_end * theta.sin();
            let color = if use_groups {
                palette_color(cfg.palette, gi)
            } else {
                palette_color(cfg.palette, i)
            };
            let hx = hex6(color);

            push_b(&mut buf, b"<line data-idx=\"");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"\" data-lbl=\"");
            escape_xml(&mut buf, &cfg.labels[i]);
            push_b(&mut buf, b"\" x1=\"");
            push_f2(&mut buf, x0);
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, y0);
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, x1);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, y1);
            push_b(&mut buf, b"\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" stroke-width=\"");
            push_f2(&mut buf, bar_px_width);
            push_b(&mut buf, b"\" stroke-linecap=\"butt\" opacity=\"0.92\"/>");

            if show_labels {
                let r_lab = r_outer_max + 16.0;
                let xl = cx + r_lab * theta.cos();
                let yl = cy + r_lab * theta.sin();
                let flip = theta.cos() < 0.0;
                let deg = theta.to_degrees() + if flip { 180.0 } else { 0.0 };
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, xl);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, yl + 3.0);
                push_b(&mut buf, b"\" transform=\"rotate(");
                push_f2(&mut buf, deg);
                push_b(&mut buf, b" ");
                push_f2(&mut buf, xl);
                push_b(&mut buf, b" ");
                push_f2(&mut buf, yl);
                push_b(&mut buf, b")\" text-anchor=\"");
                push_b(&mut buf, if flip { b"end" } else { b"start" });
                push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">");
                escape_xml(&mut buf, truncate(&cfg.labels[i], 14));
                push_b(&mut buf, b"</text>");
            }
        }
        angle_cursor += group_angle + gap_between;
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
