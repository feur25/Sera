use super::config::BarConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_legend_item,
    svg_open_rescalable, svg_title, svg_vgrid_vis, truncate,
};

fn row_extent(cfg: &BarConfig, n: usize) -> (f64, f64) {
    if !cfg.series.is_empty() {
        let mut vmax = 0.0_f64;
        let mut vmin = 0.0_f64;
        for i in 0..n {
            let mut pos_acc = 0.0_f64;
            let mut neg_acc = 0.0_f64;
            for (_, vals) in cfg.series {
                let v = vals.get(i).copied().unwrap_or(0.0);
                if !v.is_finite() {
                    continue;
                }
                if v >= 0.0 {
                    pos_acc += v;
                } else {
                    neg_acc += v;
                }
            }
            vmax = vmax.max(pos_acc);
            vmin = vmin.min(neg_acc);
        }
        return (vmin, vmax);
    }
    let vmax = cfg.values[..n].iter().cloned().fold(0.0_f64, f64::max).max(0.0);
    let vmin = cfg.values[..n].iter().cloned().fold(0.0_f64, f64::min).min(0.0);
    (vmin, vmax)
}

#[crate::chart_demo(
    "title=\"Extremes regionaux\", y_label=\"Robustesse Score median\", labels=[\"Consecrated Snowfield\",\"Gravesite Plain\",\"Scadu Altus\",\"Jagged Peak\",\"Liurnia of the Lakes\",\"Limgrave\",\"Charo's Hidden Grave\",\"Weeping Peninsula\"], values=[0.55,0.54,0.53,0.41,-0.35,-0.38,-0.41,-0.47], sort_order=\"desc\", variant=\"diverging\""
)]

pub fn render(cfg: &BarConfig) -> String {
    let stacked = !cfg.series.is_empty() && !cfg.category_labels.is_empty();
    let (labels, values): (Vec<String>, Vec<f64>) = if stacked {
        (cfg.category_labels.to_vec(), Vec::new())
    } else {
        let (l, v) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
        (l, v)
    };
    let n = if stacked {
        labels.len()
    } else {
        labels.len().min(values.len())
    };
    if n == 0 {
        return String::new();
    }

    let legend_w = if stacked { 130 } else { 0 };
    let pad_l = 168;
    let pad_r = 28 + legend_w;
    let pad_t = if cfg.title.is_empty() { 12 } else { 42 };
    let pad_b = 40;
    let w = cfg.width;
    let h = cfg.height;
    let pw = (w - pad_l - pad_r).max(10);
    let ph = (h - pad_t - pad_b).max(10);

    let (vmin, vmax) = row_extent(cfg, n);
    let pad_v = (vmax - vmin).abs().max(1e-9) * 0.1;
    let x_max = vmax + pad_v;
    let x_min = vmin - pad_v;
    let range = (x_max - x_min).max(1e-9);
    let val_to_x = |v: f64| pad_l + ((v - x_min) / range * pw as f64) as i32;

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 4096);
    svg_open_rescalable(&mut buf, w, h, pad_l, pad_t, pw, ph);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 26);

    let n_xticks = 5;
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = pad_l + (pw as f64 * frac) as i32;
        svg_vgrid_vis(&mut buf, x, pad_t, pad_t + ph, cfg.gridlines);
    }
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let val = x_min + range * frac;
        let x = pad_l + (pw as f64 * frac) as i32;
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, x);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, pad_t + ph + 16);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        push_f2(&mut buf, val);
        push_b(&mut buf, b"</text>");
    }

    let zero_x = val_to_x(0.0);
    push_b(&mut buf, b"<line x1=\"");
    push_i(&mut buf, zero_x);
    push_b(&mut buf, b"\" y1=\"");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, zero_x);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, pad_t + ph);
    push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" class=\"sp-ax-y\"/>");

    let color_pos = if cfg.color_hex != 0 { cfg.color_hex } else { cfg.color_high };
    let color_neg = cfg.color_low;
    let row_h = ph as f64 / n as f64;
    let bar_h = (row_h * (1.0 - cfg.bar_gap.max(0.1))).max(4.0);

    let mut overlay_points: Vec<(i32, f64)> = Vec::with_capacity(n);

    for i in 0..n {
        let cy_top = pad_t as f64 + row_h * i as f64;
        let cy = cy_top + (row_h - bar_h) / 2.0;

        if stacked {
            let mut pos_acc = 0.0_f64;
            let mut neg_acc = 0.0_f64;
            for (si, (_, vals)) in cfg.series.iter().enumerate() {
                let v = vals.get(i).copied().unwrap_or(0.0);
                if !v.is_finite() || v == 0.0 {
                    continue;
                }
                let (from, to) = if v >= 0.0 {
                    let from = pos_acc;
                    pos_acc += v;
                    (from, pos_acc)
                } else {
                    let from = neg_acc;
                    neg_acc += v;
                    (neg_acc, from)
                };
                let x0 = val_to_x(from);
                let x1 = val_to_x(to);
                let color = palette_color(cfg.palette, si);
                let hx = hex6(color);
                push_b(&mut buf, b"<rect data-idx=\"");
                push_i(&mut buf, (i * cfg.series.len() + si) as i32);
                push_b(&mut buf, b"\" data-series=\"");
                push_i(&mut buf, si as i32);
                push_b(&mut buf, b"\" data-v=\"");
                push_f2(&mut buf, v);
                push_b(&mut buf, b"\" x=\"");
                push_i(&mut buf, x0.min(x1));
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy);
                push_b(&mut buf, b"\" width=\"");
                push_f2(&mut buf, (x1 - x0).unsigned_abs() as f64);
                push_b(&mut buf, b"\" height=\"");
                push_f2(&mut buf, bar_h);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\"/>");
            }
        } else {
            let v = values[i];
            let x_v = val_to_x(v);
            let color = if v >= 0.0 { color_pos } else { color_neg };
            let hx = hex6(color);
            let bx0 = zero_x.min(x_v);
            let bw = (zero_x - x_v).unsigned_abs() as f64;

            push_b(&mut buf, b"<rect data-idx=\"");
            push_i(&mut buf, i as i32);
            push_b(&mut buf, b"\" data-lbl=\"");
            escape_xml(&mut buf, &labels[i]);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"\" x=\"");
            push_i(&mut buf, bx0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy);
            push_b(&mut buf, b"\" width=\"");
            push_f2(&mut buf, bw);
            push_b(&mut buf, b"\" height=\"");
            push_f2(&mut buf, bar_h);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\"/>");

            if let (Some(&elo), Some(&ehi)) = (cfg.error_low.get(i), cfg.error_high.get(i)) {
                let wx0 = val_to_x(v - elo.abs());
                let wx1 = val_to_x(v + ehi.abs());
                let wy = cy + bar_h / 2.0;
                let cap = (bar_h * 0.35).max(3.0);
                for x in [wx0, wx1] {
                    push_b(&mut buf, b"<line x1=\"");
                    push_i(&mut buf, x);
                    push_b(&mut buf, b"\" y1=\"");
                    push_f2(&mut buf, wy - cap / 2.0);
                    push_b(&mut buf, b"\" x2=\"");
                    push_i(&mut buf, x);
                    push_b(&mut buf, b"\" y2=\"");
                    push_f2(&mut buf, wy + cap / 2.0);
                    push_b(&mut buf, b"\" stroke=\"#111827\" stroke-width=\"1.5\"/>");
                }
                push_b(&mut buf, b"<line x1=\"");
                push_i(&mut buf, wx0);
                push_b(&mut buf, b"\" y1=\"");
                push_f2(&mut buf, wy);
                push_b(&mut buf, b"\" x2=\"");
                push_i(&mut buf, wx1);
                push_b(&mut buf, b"\" y2=\"");
                push_f2(&mut buf, wy);
                push_b(&mut buf, b"\" stroke=\"#111827\" stroke-width=\"1.5\"/>");
            }

            if cfg.show_text {
                let inside = bw > 32.0;
                let (tx, anchor, fill): (i32, &[u8], &[u8]) = if inside {
                    let ex = if v >= 0.0 { x_v - 6 } else { x_v + 6 };
                    let anc: &[u8] = if v >= 0.0 { b"end" } else { b"start" };
                    (ex, anc, b"#ffffff")
                } else {
                    let ex = if v >= 0.0 { x_v + 6 } else { x_v - 6 };
                    let anc: &[u8] = if v >= 0.0 { b"start" } else { b"end" };
                    (ex, anc, b"#111827")
                };
                push_b(&mut buf, b"<text x=\"");
                push_i(&mut buf, tx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, cy + bar_h / 2.0 + 4.0);
                push_b(&mut buf, b"\" text-anchor=\"");
                buf.extend_from_slice(anchor);
                push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"");
                buf.extend_from_slice(fill);
                push_b(&mut buf, b"\">");
                push_f2(&mut buf, v);
                push_b(&mut buf, b"</text>");
            }
        }

        if let Some(&ov) = cfg.overlay_line.get(i) {
            overlay_points.push((val_to_x(ov), cy + bar_h / 2.0));
        }

        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l - 8);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + bar_h / 2.0 + 4.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, truncate(&labels[i], 22));
        push_b(&mut buf, b"</text>");
    }

    if overlay_points.len() > 1 {
        push_b(&mut buf, b"<polyline data-overlay=\"1\" fill=\"none\" stroke=\"#111827\" stroke-width=\"2\" points=\"");
        for (i, (x, y)) in overlay_points.iter().enumerate() {
            if i > 0 {
                push_b(&mut buf, b" ");
            }
            push_i(&mut buf, *x);
            push_b(&mut buf, b",");
            push_f2(&mut buf, *y);
        }
        push_b(&mut buf, b"\"/>");
        for (x, y) in &overlay_points {
            push_b(&mut buf, b"<circle cx=\"");
            push_i(&mut buf, *x);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, *y);
            push_b(&mut buf, b"\" r=\"3\" fill=\"#111827\"/>");
        }
        if !cfg.overlay_line_label.is_empty() {
            let (lx, ly) = overlay_points[0];
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly - 8.0);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#111827\">");
            escape_xml(&mut buf, cfg.overlay_line_label);
            push_b(&mut buf, b"</text>");
        }
    }

    if stacked {
        let lx = w - legend_w + 12;
        for (si, (name, _)) in cfg.series.iter().enumerate() {
            svg_legend_item(&mut buf, si as i32, name, palette_color(cfg.palette, si), lx, pad_t + si as i32 * 22, 16);
        }
    }

    if !cfg.y_label.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l + pw / 2);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, h - 6);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut buf, cfg.y_label);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::statistical::bar::variant::BarVariant;

    fn base_cfg() -> BarConfig<'static> {
        BarConfig {
            variant: BarVariant::Diverging,
            ..Default::default()
        }
    }

    #[test]
    fn simple_mode_colors_positive_bars_with_color_high_and_negative_bars_with_color_low() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let values = vec![5.0, -5.0];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            color_low: 0x111111,
            color_high: 0x222222,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(out.contains("fill=\"#222222\""), "positive bar should use color_high: {out}");
        assert!(out.contains("fill=\"#111111\""), "negative bar should use color_low: {out}");
    }

    #[test]
    fn color_hex_override_replaces_only_the_positive_color() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let values = vec![5.0, -5.0];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            color_hex: 0x333333,
            color_low: 0x111111,
            color_high: 0x222222,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(out.contains("fill=\"#333333\""), "color_hex should override the positive color: {out}");
        assert!(out.contains("fill=\"#111111\""), "negative bar should still use color_low: {out}");
        assert!(!out.contains("fill=\"#222222\""), "color_high should not appear once overridden: {out}");
    }

    #[test]
    fn stacked_mode_activates_when_series_and_category_labels_are_both_present() {
        let category_labels = vec!["Jan".to_string(), "Feb".to_string()];
        let series = vec![
            ("new".to_string(), vec![10.0, 12.0]),
            ("churn".to_string(), vec![-4.0, -3.0]),
        ];
        let cfg = BarConfig {
            category_labels: &category_labels,
            series: &series,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(out.contains("data-series=\"0\""));
        assert!(out.contains("data-series=\"1\""));
    }

    #[test]
    fn stacked_mode_draws_a_legend_naming_each_real_series_not_a_generic_positive_negative_label() {
        let category_labels = vec!["Jan".to_string(), "Feb".to_string()];
        let series = vec![
            ("new".to_string(), vec![10.0, 12.0]),
            ("churn".to_string(), vec![-4.0, -3.0]),
        ];
        let cfg = BarConfig {
            category_labels: &category_labels,
            series: &series,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(out.contains("data-legend=\"1\""));
        assert!(out.contains("aria-label=\"new\""));
        assert!(out.contains("aria-label=\"churn\""));
        assert!(!out.to_lowercase().contains("positive"));
        assert!(!out.to_lowercase().contains("negative"));
    }

    #[test]
    fn simple_mode_draws_no_legend_at_all() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let values = vec![5.0, -5.0];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(!out.contains("data-legend=\"1\""));
    }

    #[test]
    fn error_bars_render_a_whisker_when_both_bounds_are_present_for_a_row() {
        let labels = vec!["A".to_string()];
        let values = vec![10.0];
        let error_low = vec![2.0];
        let error_high = vec![3.0];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            error_low: &error_low,
            error_high: &error_high,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(out.contains("stroke=\"#111827\" stroke-width=\"1.5\""));
    }

    #[test]
    fn overlay_line_draws_a_connected_polyline_across_rows_with_a_matching_value() {
        let labels = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let values = vec![5.0, -2.0, 3.0];
        let overlay_line = vec![1.0, 0.5, 2.0];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            overlay_line: &overlay_line,
            overlay_line_label: "Overall Change",
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(out.contains("data-overlay=\"1\""));
        assert!(out.contains("Overall Change"));
    }

    #[test]
    fn overlay_line_is_absent_when_fewer_than_two_points_are_supplied() {
        let labels = vec!["A".to_string(), "B".to_string()];
        let values = vec![5.0, -2.0];
        let overlay_line = vec![1.0];
        let cfg = BarConfig {
            labels: &labels,
            values: &values,
            overlay_line: &overlay_line,
            ..base_cfg()
        };
        let out = render(&cfg);
        assert!(!out.contains("data-overlay=\"1\""));
    }
}
