use super::config::LineConfig;
use crate::plot::statistical::common::{
    colorscale_color, draw_point_callout, escape_xml, hex6, lerp_rgb, local_maxima_indices,
    push_b, push_f2, push_i, truncate, Frame,
};

#[crate::chart_demo(
    "x_labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\",\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], values=[102,108,101,96,89,94,110,124,119,131,127,142,138,151], title=\"Momentum\", show_points=True"
)]

pub fn render(cfg: &LineConfig) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n < 2 {
        return String::new();
    }
    let values = &cfg.values[..n];
    let labels = &cfg.labels[..n];

    let max_val = values.iter().cloned().filter(|v| v.is_finite()).fold(f64::NEG_INFINITY, f64::max);
    let min_val = values.iter().cloned().filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min);
    if !max_val.is_finite() || !min_val.is_finite() {
        return String::new();
    }
    let pad_v = (max_val - min_val).abs().max(1e-9) * 0.14;
    let y_min = min_val - pad_v;
    let y_max = max_val + pad_v;

    let slopes: Vec<f64> = (0..n - 1).map(|i| values[i + 1] - values[i]).collect();
    let max_abs_slope = slopes.iter().cloned().fold(0.0f64, |a, s| a.max(s.abs())).max(1e-9);
    let seg_color = |i: usize| -> u32 {
        let t = 0.5 + 0.5 * (slopes[i] / max_abs_slope);
        colorscale_color("rdbu", t.clamp(0.0, 1.0))
    };

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 64, 54, 48, n * 160 + 4096);
    f.open(cfg.title, true);
    f.y_grid(5, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let step_x = f.pw as f64 / (n - 1).max(1) as f64;
    let y_range = (y_max - y_min).max(1e-9);
    let x_at = |i: usize| f.pl as f64 + i as f64 * step_x;
    let y_at = |v: f64| f.pt as f64 + (1.0 - (v - y_min) / y_range) * f.ph as f64;
    let baseline_y = if y_min <= 0.0 && y_max >= 0.0 {
        y_at(0.0)
    } else {
        (f.pt + f.ph) as f64
    };

    push_b(&mut f.buf, b"<defs>");
    for i in 0..n - 1 {
        let hx = hex6(seg_color(i));
        push_b(&mut f.buf, b"<linearGradient id=\"spMomG");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.22\"/><stop offset=\"1\" stop-color=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.015\"/></linearGradient>");
    }
    push_b(&mut f.buf, b"<linearGradient id=\"spMomScale\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">");
    for step in 0..=4 {
        let t = step as f64 / 4.0;
        let hx = hex6(colorscale_color("rdbu", t));
        push_b(&mut f.buf, b"<stop offset=\"");
        push_f2(&mut f.buf, t);
        push_b(&mut f.buf, b"\" stop-color=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\"/>");
    }
    push_b(&mut f.buf, b"</linearGradient>");
    push_b(&mut f.buf, b"<filter id=\"spMomGlow\" x=\"-90%\" y=\"-90%\" width=\"280%\" height=\"280%\"><feGaussianBlur stdDeviation=\"5\" result=\"b\"/><feMerge><feMergeNode in=\"b\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter>");
    push_b(&mut f.buf, b"<style>.sp-mom-pulse{transform-origin:center;transform-box:fill-box;animation:spMomPulse 2.4s ease-in-out infinite}@keyframes spMomPulse{0%,100%{opacity:.55;r:9}50%{opacity:.12;r:14}}@media (prefers-reduced-motion:reduce){.sp-mom-pulse{animation:none}}</style>");
    push_b(&mut f.buf, b"</defs>");

    for i in 0..n - 1 {
        let (x1, y1, x2, y2) = (x_at(i), y_at(values[i]), x_at(i + 1), y_at(values[i + 1]));
        push_b(&mut f.buf, b"<path d=\"M");
        push_f2(&mut f.buf, x1);
        f.buf.push(b',');
        push_f2(&mut f.buf, y1);
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x2);
        f.buf.push(b',');
        push_f2(&mut f.buf, y2);
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x2);
        f.buf.push(b',');
        push_f2(&mut f.buf, baseline_y);
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x1);
        f.buf.push(b',');
        push_f2(&mut f.buf, baseline_y);
        push_b(&mut f.buf, b" Z\" fill=\"url(#spMomG");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b")\" stroke=\"none\"/>");
    }

    let seg_w = (cfg.stroke_width + 1.0).max(2.5);
    for i in 0..n - 1 {
        let hx = hex6(seg_color(i));
        push_b(&mut f.buf, b"<line data-seg=\"");
        push_i(&mut f.buf, i as i32);
        push_b(&mut f.buf, b"\" x1=\"");
        push_f2(&mut f.buf, x_at(i));
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, y_at(values[i]));
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, x_at(i + 1));
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, y_at(values[i + 1]));
        push_b(&mut f.buf, b"\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"");
        push_f2(&mut f.buf, seg_w);
        push_b(&mut f.buf, b"\" stroke-linecap=\"round\"/>");
    }

    if cfg.show_points {
        for i in 0..n {
            let color = if i == 0 {
                seg_color(0)
            } else {
                seg_color(i.min(n - 2))
            };
            let hx = hex6(color);
            push_b(&mut f.buf, b"<circle data-idx=\"");
            push_i(&mut f.buf, i as i32);
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &labels[i]);
            push_b(&mut f.buf, b"\" data-y=\"");
            push_f2(&mut f.buf, values[i]);
            push_b(&mut f.buf, b"\" cx=\"");
            push_f2(&mut f.buf, x_at(i));
            push_b(&mut f.buf, b"\" cy=\"");
            push_f2(&mut f.buf, y_at(values[i]));
            push_b(&mut f.buf, b"\" r=\"3\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\"/>");
        }
    }

    let top_n = ((n as f64 / 6.0).ceil() as usize).clamp(1, 3);
    let min_gap = (n / 6).max(1);
    let peaks = local_maxima_indices(values, top_n, min_gap);
    let negated: Vec<f64> = values.iter().map(|v| -v).collect();
    let troughs = local_maxima_indices(&negated, top_n, min_gap);
    for &i in &peaks {
        let lbl = format!("{:.1}", values[i]);
        draw_point_callout(&mut f.buf, x_at(i) as i32, y_at(values[i]) as i32, &lbl, 0xC2410C, true);
    }
    for &i in &troughs {
        let lbl = format!("{:.1}", values[i]);
        draw_point_callout(&mut f.buf, x_at(i) as i32, y_at(values[i]) as i32, &lbl, 0x1D4ED8, false);
    }

    let last = n - 1;
    let lx = x_at(last);
    let ly = y_at(values[last]);
    let last_color = seg_color(last - 1);
    let last_hx = hex6(last_color);
    let mid_hx = hex6(lerp_rgb(last_color, 0xFFFFFF, 0.22));
    push_b(&mut f.buf, b"<circle class=\"sp-mom-pulse\" cx=\"");
    push_f2(&mut f.buf, lx);
    push_b(&mut f.buf, b"\" cy=\"");
    push_f2(&mut f.buf, ly);
    push_b(&mut f.buf, b"\" r=\"9\" fill=\"none\" stroke=\"#");
    f.buf.extend_from_slice(&last_hx);
    push_b(&mut f.buf, b"\" stroke-width=\"1.6\" opacity=\"0.55\" filter=\"url(#spMomGlow)\"/>");
    push_b(&mut f.buf, b"<circle cx=\"");
    push_f2(&mut f.buf, lx);
    push_b(&mut f.buf, b"\" cy=\"");
    push_f2(&mut f.buf, ly);
    push_b(&mut f.buf, b"\" r=\"5\" fill=\"#");
    f.buf.extend_from_slice(&mid_hx);
    push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.5\" filter=\"url(#spMomGlow)\"/>");

    let bar_w = 96i32;
    let bar_x = f.pl + f.pw - bar_w;
    let bar_y = 36i32;
    push_b(&mut f.buf, b"<rect x=\"");
    push_i(&mut f.buf, bar_x);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y);
    push_b(&mut f.buf, b"\" width=\"");
    push_i(&mut f.buf, bar_w);
    push_b(&mut f.buf, b"\" height=\"6\" rx=\"3\" fill=\"url(#spMomScale)\"/>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, bar_x - 5);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + 6);
    push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">Falling</text>");
    push_b(&mut f.buf, b"<text x=\"");
    push_i(&mut f.buf, bar_x + bar_w + 5);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, bar_y + 6);
    push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">Rising</text>");

    let tick_step = ((n as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n).step_by(tick_step) {
        let xi = x_at(i);
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, xi);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&labels[i], 10));
        push_b(&mut f.buf, b"</text>");
    }

    f.html("[]")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg_with<'a>(values: &'a [f64], labels: &'a [String]) -> LineConfig<'a> {
        LineConfig {
            variant: super::super::LineVariant::Momentum,
            values,
            labels,
            show_points: true,
            gridlines: true,
            ..Default::default()
        }
    }

    #[test]
    fn momentum_render_produces_one_colored_segment_per_gap_between_points() {
        let values = vec![10.0, 12.0, 8.0, 20.0, 18.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels));
        for i in 0..values.len() - 1 {
            assert!(
                out.contains(&format!("data-seg=\"{i}\"")),
                "expected a segment marker for gap {i}, got: {out}"
            );
        }
    }

    #[test]
    fn momentum_render_colors_a_rising_segment_and_a_falling_segment_differently() {
        let values = vec![10.0, 40.0, 5.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels));
        let rising_start = out.find("data-seg=\"0\"").unwrap();
        let falling_start = out.find("data-seg=\"1\"").unwrap();
        let rising_stroke = &out[rising_start..rising_start + 120];
        let falling_stroke = &out[falling_start..falling_start + 120];
        assert_ne!(
            rising_stroke, falling_stroke,
            "a steep rise and a steep fall must not render with the same stroke color"
        );
    }

    #[test]
    fn momentum_render_places_a_glow_pulse_marker_on_the_final_point() {
        let values = vec![5.0, 9.0, 7.0, 15.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels));
        assert!(out.contains("sp-mom-pulse"), "expected the last-point pulse marker class: {out}");
        assert!(out.contains("spMomGlow"), "expected the glow filter to be defined and referenced: {out}");
    }

    #[test]
    fn momentum_render_respects_gridlines_flag_via_shared_sp_gl_convention() {
        let values = vec![1.0, 2.0, 3.0, 2.0, 4.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let mut cfg = cfg_with(&values, &labels);
        cfg.gridlines = false;
        let out = render(&cfg);
        assert!(out.contains("class=\"sp-gl\""), "gridlines must still be present in the DOM (hidden via style) for the universal grid() method to toggle: {out}");
    }

    #[test]
    fn momentum_render_returns_empty_string_below_two_points() {
        let values = vec![5.0];
        let labels: Vec<String> = vec!["a".to_string()];
        assert_eq!(render(&cfg_with(&values, &labels)), "");
    }

    #[test]
    fn momentum_chart_still_accepts_universal_chainable_methods_after_variant_specific_rendering() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("line/momentum.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("momentum demo payload");
        let html = crate::bindings::method_registry::apply_by_name(
            &crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap(),
            "grid",
            "{}",
        ).expect("grid() must apply cleanly to a momentum chart");
        let html = crate::bindings::method_registry::apply_by_name(&html, "show_legend", "{}")
            .expect("show_legend() must apply cleanly after grid() on a momentum chart");
        assert!(html.contains("class=\"sp-gl\""), "gridlines must survive both chained calls: {html}");
        assert!(html.contains("sp-mom-pulse"), "the variant's own pulse marker must survive both chained calls untouched: {html}");
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("line/momentum.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/line-momentum.html", html).unwrap();
        }
    }
}
