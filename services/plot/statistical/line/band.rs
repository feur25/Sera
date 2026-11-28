use super::config::LineConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate, Frame};

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\",\"Jul\"], series=[[10,12,11,14,16,15,18],[18,22,21,26,29,28,32]], series_names=[\"Forecast low\",\"Forecast high\"], variant=\"band\""
)]

pub fn render(cfg: &LineConfig) -> String {
    let n_pairs = cfg.series.len() / 2;
    let n_x = cfg.x_labels.len();
    if n_pairs == 0 || n_x == 0 {
        return String::new();
    }

    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    for (_, vals) in cfg.series {
        for &v in vals {
            if v.is_finite() {
                y_min = y_min.min(v);
                y_max = y_max.max(v);
            }
        }
    }
    if !y_min.is_finite() || !y_max.is_finite() {
        return String::new();
    }
    let pad_v = (y_max - y_min).abs().max(1e-9) * 0.08;
    y_min -= pad_v;
    y_max += pad_v;

    let legend_w = if n_pairs > 1 { 160 } else { 0 };
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        44,
        legend_w,
        n_x * n_pairs * 220 + 4096,
    );
    f.open(cfg.title, true);
    f.y_grid(6, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let step = f.pw as f64 / (n_x - 1).max(1) as f64;
    let y_range = (y_max - y_min).max(1e-9);
    let x_at = |i: usize| f.pl as f64 + i as f64 * step;
    let y_at = |v: f64| f.pt as f64 + (1.0 - (v - y_min) / y_range) * f.ph as f64;

    let mut group_names: Vec<String> = Vec::with_capacity(n_pairs);
    for g in 0..n_pairs {
        let (name_raw, low) = &cfg.series[2 * g];
        let (_, high) = &cfg.series[2 * g + 1];
        let name = name_raw
            .trim_end_matches(|c: char| c.is_whitespace())
            .trim_end_matches("_low")
            .trim_end_matches(" low")
            .trim_end_matches("Low")
            .to_string();
        group_names.push(name.clone());
        let color = palette_color(cfg.palette, g);
        let hx = hex6(color);
        let n = n_x.min(low.len()).min(high.len());
        if n == 0 {
            continue;
        }

        push_b(&mut f.buf, b"<g data-series=\"");
        push_i(&mut f.buf, g as i32);
        push_b(&mut f.buf, b"\" data-lbl=\"");
        escape_xml(&mut f.buf, &name);
        push_b(&mut f.buf, b"\">");

        push_b(&mut f.buf, b"<path d=\"M ");
        for i in 0..n {
            if i > 0 {
                push_b(&mut f.buf, b" L ");
            }
            push_f2(&mut f.buf, x_at(i));
            f.buf.push(b',');
            push_f2(&mut f.buf, y_at(high[i]));
        }
        for i in (0..n).rev() {
            push_b(&mut f.buf, b" L ");
            push_f2(&mut f.buf, x_at(i));
            f.buf.push(b',');
            push_f2(&mut f.buf, y_at(low[i]));
        }
        push_b(&mut f.buf, b" Z\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.20\" stroke=\"none\"/>");

        for (vals, dash) in [(&low, true), (&high, true)] {
            push_b(&mut f.buf, b"<polyline points=\"");
            for i in 0..n {
                if i > 0 {
                    f.buf.push(b' ');
                }
                push_f2(&mut f.buf, x_at(i));
                f.buf.push(b',');
                push_f2(&mut f.buf, y_at(vals[i]));
            }
            push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.55\"");
            if dash {
                push_b(&mut f.buf, b" stroke-dasharray=\"3,3\"");
            }
            push_b(&mut f.buf, b"/>");
        }

        push_b(&mut f.buf, b"<polyline points=\"");
        for i in 0..n {
            if i > 0 {
                f.buf.push(b' ');
            }
            let mid = (low[i] + high[i]) / 2.0;
            push_f2(&mut f.buf, x_at(i));
            f.buf.push(b',');
            push_f2(&mut f.buf, y_at(mid));
        }
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2.4\"/>");

        let circ_step = ((n as f64 / 200.0).ceil() as usize).max(1);
        for i in (0..n).step_by(circ_step) {
            let mid = (low[i] + high[i]) / 2.0;
            push_b(&mut f.buf, b"<circle data-idx=\"");
            push_i(&mut f.buf, (g * n_x + i) as i32);
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &name);
            push_b(&mut f.buf, b"\" data-x=\"");
            escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 16));
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, mid);
            push_b(&mut f.buf, b"\" cx=\"");
            push_f2(&mut f.buf, x_at(i));
            push_b(&mut f.buf, b"\" cy=\"");
            push_f2(&mut f.buf, y_at(mid));
            push_b(&mut f.buf, b"\" r=\"3.2\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\"/>");
        }
        push_b(&mut f.buf, b"</g>");
    }

    for (i, lbl) in cfg.x_labels.iter().enumerate() {
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, x_at(i));
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(lbl, 12));
        push_b(&mut f.buf, b"</text>");
    }

    if n_pairs > 1 {
        let name_refs: Vec<&str> = group_names.iter().map(String::as_str).collect();
        f.legend_pos(&name_refs, cfg.palette, cfg.legend_position);
    }

    f.html(&slots_to_json(cfg.hover))
}
