use super::config::LineConfig;
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, truncate, Frame};

#[crate::chart_demo(
    "x_labels=[\"W1\",\"W2\",\"W3\",\"W4\",\"W5\",\"W6\",\"W7\",\"W8\",\"W9\",\"W10\",\"W11\",\"W12\",\"W13\",\"W14\"], values=[60,95,140,175,205,260,290,300,345,368,410,430,455,470], title=\"Pace\", show_points=True, pace_target=500"
)]

pub fn render(cfg: &LineConfig) -> String {
    let n = cfg.values.len().min(cfg.labels.len());
    if n < 2 {
        return String::new();
    }
    let values = &cfg.values[..n];
    let labels = &cfg.labels[..n];
    let target = cfg.pace_target.unwrap_or_else(|| {
        values.iter().cloned().filter(|v| v.is_finite()).fold(f64::NEG_INFINITY, f64::max) * 1.15
    });

    let proj_k = (n / 3).clamp(2, 6).min(n);
    let recent_start = n - proj_k;
    let slope = (values[n - 1] - values[recent_start]) / (proj_k - 1).max(1) as f64;
    let proj_steps = ((n as f64) * 0.28).ceil().max(3.0) as usize;
    let proj_end_value = values[n - 1] + slope * proj_steps as f64;
    let total_steps = n + proj_steps;

    let mut max_val = values.iter().cloned().filter(|v| v.is_finite()).fold(f64::NEG_INFINITY, f64::max).max(target).max(proj_end_value);
    let mut min_val = values.iter().cloned().filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min).min(target).min(proj_end_value);
    if !max_val.is_finite() || !min_val.is_finite() {
        return String::new();
    }
    if (max_val - min_val).abs() < 1e-9 {
        max_val += 1.0;
        min_val -= 1.0;
    }
    let pad_v = (max_val - min_val).abs() * 0.12;
    let y_min = min_val - pad_v;
    let y_max = max_val + pad_v;

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 48, 54, 60, total_steps * 140 + 4096);
    f.open(cfg.title, true);

    let (pl, pt, pw, ph) = (f.pl, f.pt, f.pw, f.ph);
    let step_x = pw as f64 / (total_steps - 1).max(1) as f64;
    let y_range = (y_max - y_min).max(1e-9);
    let x_at = |i: usize| pl as f64 + i as f64 * step_x;
    let y_at = |v: f64| pt as f64 + (1.0 - (v - y_min) / y_range) * ph as f64;

    let ideal_pace = |i: usize| values[0] + (target - values[0]) * (i as f64 / (n - 1).max(1) as f64);
    let ahead_hx = hex6(cfg.pace_ahead_color);
    let behind_hx = hex6(cfg.pace_behind_color);

    f.y_grid(5, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for i in 0..n - 1 {
        let ahead = values[i] >= ideal_pace(i);
        let hx = if ahead { &ahead_hx } else { &behind_hx };
        let (x1, x2) = (x_at(i), x_at(i + 1));
        let (ay1, ay2) = (y_at(values[i]), y_at(values[i + 1]));
        let (py1, py2) = (y_at(ideal_pace(i)), y_at(ideal_pace(i + 1)));
        push_b(&mut f.buf, b"<path d=\"M");
        push_f2(&mut f.buf, x1);
        f.buf.push(b',');
        push_f2(&mut f.buf, ay1);
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x2);
        f.buf.push(b',');
        push_f2(&mut f.buf, ay2);
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x2);
        f.buf.push(b',');
        push_f2(&mut f.buf, py2);
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x1);
        f.buf.push(b',');
        push_f2(&mut f.buf, py1);
        push_b(&mut f.buf, b" Z\" fill=\"#");
        f.buf.extend_from_slice(hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.10\" stroke=\"none\"/>");
    }

    push_b(&mut f.buf, b"<line x1=\"");
    push_f2(&mut f.buf, x_at(0));
    push_b(&mut f.buf, b"\" y1=\"");
    push_f2(&mut f.buf, y_at(values[0]));
    push_b(&mut f.buf, b"\" x2=\"");
    push_f2(&mut f.buf, x_at(n - 1));
    push_b(&mut f.buf, b"\" y2=\"");
    push_f2(&mut f.buf, y_at(target));
    push_b(&mut f.buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1.4\" stroke-dasharray=\"5,4\"/>");

    push_b(&mut f.buf, b"<line x1=\"");
    push_f2(&mut f.buf, x_at(0));
    push_b(&mut f.buf, b"\" y1=\"");
    push_f2(&mut f.buf, y_at(target));
    push_b(&mut f.buf, b"\" x2=\"");
    push_f2(&mut f.buf, x_at(total_steps - 1));
    push_b(&mut f.buf, b"\" y2=\"");
    push_f2(&mut f.buf, y_at(target));
    push_b(&mut f.buf, b"\" stroke=\"#334155\" stroke-width=\"1.4\" stroke-dasharray=\"2,3\"/>");

    push_b(&mut f.buf, b"<g class=\"sp-val\"><text x=\"");
    push_f2(&mut f.buf, x_at(total_steps - 1));
    push_b(&mut f.buf, b"\" y=\"");
    push_f2(&mut f.buf, y_at(target) - 6.0);
    push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"10\" fill=\"#334155\">Target ");
    push_f2(&mut f.buf, target);
    push_b(&mut f.buf, b"</text></g>");

    push_b(&mut f.buf, b"<line x1=\"");
    push_f2(&mut f.buf, x_at(n - 1));
    push_b(&mut f.buf, b"\" y1=\"");
    push_i(&mut f.buf, pt);
    push_b(&mut f.buf, b"\" x2=\"");
    push_f2(&mut f.buf, x_at(n - 1));
    push_b(&mut f.buf, b"\" y2=\"");
    push_i(&mut f.buf, pt + ph);
    push_b(&mut f.buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\" stroke-dasharray=\"3,3\"/>");

    let final_ahead = proj_end_value >= target;
    let proj_hx = if final_ahead { &ahead_hx } else { &behind_hx };
    push_b(&mut f.buf, b"<line x1=\"");
    push_f2(&mut f.buf, x_at(n - 1));
    push_b(&mut f.buf, b"\" y1=\"");
    push_f2(&mut f.buf, y_at(values[n - 1]));
    push_b(&mut f.buf, b"\" x2=\"");
    push_f2(&mut f.buf, x_at(total_steps - 1));
    push_b(&mut f.buf, b"\" y2=\"");
    push_f2(&mut f.buf, y_at(proj_end_value));
    push_b(&mut f.buf, b"\" stroke=\"#");
    f.buf.extend_from_slice(proj_hx);
    push_b(&mut f.buf, b"\" stroke-width=\"2.2\" stroke-dasharray=\"6,4\" stroke-linecap=\"round\"/>");

    push_b(&mut f.buf, b"<polyline data-idx=\"0\" points=\"");
    for i in 0..n {
        if i > 0 {
            f.buf.push(b' ');
        }
        push_f2(&mut f.buf, x_at(i));
        f.buf.push(b',');
        push_f2(&mut f.buf, y_at(values[i]));
    }
    push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#1e293b\" stroke-width=\"");
    push_f2(&mut f.buf, cfg.stroke_width + 0.6);
    push_b(&mut f.buf, b"\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>");

    if cfg.show_points {
        for i in 0..n {
            let ahead = values[i] >= ideal_pace(i);
            let hx = if ahead { &ahead_hx } else { &behind_hx };
            push_b(&mut f.buf, b"<circle cx=\"");
            push_f2(&mut f.buf, x_at(i));
            push_b(&mut f.buf, b"\" cy=\"");
            push_f2(&mut f.buf, y_at(values[i]));
            push_b(&mut f.buf, b"\" r=\"3.4\" fill=\"#");
            f.buf.extend_from_slice(hx);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.2\"/>");
        }
    }

    push_b(&mut f.buf, b"<g class=\"sp-val\"><circle cx=\"");
    push_f2(&mut f.buf, x_at(total_steps - 1));
    push_b(&mut f.buf, b"\" cy=\"");
    push_f2(&mut f.buf, y_at(proj_end_value));
    push_b(&mut f.buf, b"\" r=\"4.5\" fill=\"#");
    f.buf.extend_from_slice(proj_hx);
    push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.4\"/>");
    let arrow = if final_ahead { "\u{2713}" } else { "\u{2717}" };
    let label = format!("{} ~{:.0}", arrow, proj_end_value);
    push_b(&mut f.buf, b"<text x=\"");
    push_f2(&mut f.buf, x_at(total_steps - 1));
    push_b(&mut f.buf, b"\" y=\"");
    push_f2(&mut f.buf, y_at(proj_end_value) - 10.0);
    push_b(&mut f.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"10\" fill=\"#");
    f.buf.extend_from_slice(proj_hx);
    push_b(&mut f.buf, b"\">");
    escape_xml(&mut f.buf, &label);
    push_b(&mut f.buf, b"</text></g>");

    let tick_step = ((n as f64 / 10.0).ceil() as usize).max(1);
    for i in (0..n).step_by(tick_step) {
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, x_at(i));
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, pt + ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&labels[i], 10));
        push_b(&mut f.buf, b"</text>");
    }
    push_b(&mut f.buf, b"<text x=\"");
    push_f2(&mut f.buf, (x_at(n - 1) + x_at(total_steps - 1)) / 2.0);
    push_b(&mut f.buf, b"\" y=\"");
    push_i(&mut f.buf, pt + ph + 14);
    push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-style=\"italic\" fill=\"#94a3b8\">projected</text>");

    f.html("[]")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg_with<'a>(values: &'a [f64], labels: &'a [String], target: f64) -> LineConfig<'a> {
        LineConfig {
            variant: super::super::LineVariant::Pace,
            values,
            labels,
            show_points: true,
            gridlines: true,
            pace_target: Some(target),
            ..Default::default()
        }
    }

    #[test]
    fn pace_render_draws_the_target_line_at_the_configured_value() {
        let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels, 100.0));
        assert!(out.contains("Target 100"), "the target line's label must show the configured target value: {out}");
    }

    #[test]
    fn pace_render_marks_a_series_that_exceeds_its_own_ideal_pace_as_ahead() {
        let values = vec![50.0, 70.0, 90.0, 110.0, 130.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels, 140.0));
        assert!(out.contains("#16a34a"), "a series consistently above its own linear glidepath must render at least one ahead-colored (green) mark: {out}");
    }

    #[test]
    fn pace_render_projects_a_dashed_continuation_past_the_last_real_point() {
        let values = vec![10.0, 12.0, 15.0, 19.0, 24.0, 30.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels, 60.0));
        assert!(out.contains("projected"), "pace must label its extrapolated projection zone: {out}");
    }

    #[test]
    fn pace_render_returns_empty_string_below_two_points() {
        let values = vec![5.0];
        let labels: Vec<String> = vec!["a".to_string()];
        assert_eq!(render(&cfg_with(&values, &labels, 10.0)), "");
    }

    #[test]
    fn pace_chart_target_and_projection_labels_disappear_after_no_value_is_chained() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("line/pace.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("pace demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        assert!(html.contains("class=\"sp-val\""), "pace's target/projection labels must be wrapped in sp-val: {html}");
        let hidden = crate::bindings::method_registry::apply_by_name(&html, "no_value", "{}").expect("no_value() must apply cleanly to a pace chart");
        assert!(hidden.contains(".sp-val{display:none!important}"), "no_value() must inject the shared sp-val hiding rule: {hidden}");
    }

    #[test]
    fn pace_chart_honors_custom_ahead_and_behind_colors_through_the_real_builder() {
        let input = serde_json::json!({
            "title": "t",
            "x_labels": ["a","b","c","d","e"],
            "values": [90.0, 92.0, 95.0, 97.0, 99.0],
            "variant": "pace",
            "pace_target": 100.0,
            "pace_ahead_color": 65280,
            "pace_behind_color": 255
        }).to_string();
        let html = crate::plot::statistical::build_line(&input);
        assert!(html.contains("#00ff00") || html.contains("#0000ff"), "custom pace_ahead_color/pace_behind_color must reach the rendered marks: {html}");
    }

    #[test]
    fn pace_chart_still_accepts_grid_then_show_legend_without_corruption() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("line/pace.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("pace demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        let html = crate::bindings::method_registry::apply_by_name(&html, "grid", "{}").expect("grid() must apply cleanly");
        let html = crate::bindings::method_registry::apply_by_name(&html, "show_legend", "{}").expect("show_legend() must apply cleanly after grid()");
        assert!(html.contains("class=\"sp-gl\""), "gridlines must survive both chained calls: {html}");
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("line/pace.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/line-pace.html", html).unwrap();
        }
    }
}
