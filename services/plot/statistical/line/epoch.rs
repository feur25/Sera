use super::config::LineConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, moving_average, push_b, push_f2, push_i, truncate, Frame,
};

#[crate::chart_demo(
    "x_labels=[\"W1\",\"W2\",\"W3\",\"W4\",\"W5\",\"W6\",\"W7\",\"W8\",\"W9\",\"W10\",\"W11\",\"W12\",\"W13\",\"W14\",\"W15\",\"W16\"], values=[40,41,39,42,40,55,72,88,102,98,85,90,96,101,108,115], title=\"Epoch\", show_points=True"
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

    let bounds = chapter_boundaries(values, n);
    let line_color: u32 = 0x0F172A;
    let rise_color: u32 = 0xB91C1C;
    let fall_color: u32 = 0x1D4ED8;
    let flat_color: u32 = 0x64748B;

    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 64, 54, 26, n * 160 + 4096);
    f.open(cfg.title, true);

    let (pl, pt, pw, ph) = (f.pl, f.pt, f.pw, f.ph);
    let step_x = pw as f64 / (n - 1).max(1) as f64;
    let y_range = (y_max - y_min).max(1e-9);
    let x_at = |i: usize| pl as f64 + i as f64 * step_x;
    let y_at = |v: f64| pt as f64 + (1.0 - (v - y_min) / y_range) * ph as f64;

    push_b(&mut f.buf, b"<defs><linearGradient id=\"spEpochG\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"#");
    f.buf.extend_from_slice(&hex6(line_color));
    push_b(&mut f.buf, b"\" stop-opacity=\"0.16\"/><stop offset=\"1\" stop-color=\"#");
    f.buf.extend_from_slice(&hex6(line_color));
    push_b(&mut f.buf, b"\" stop-opacity=\"0.01\"/></linearGradient></defs>");

    for w in bounds.windows(2) {
        let (s, e) = (w[0], w[1]);
        let denom = values[s].abs().max(1e-6);
        let pct = (values[e] - values[s]) / denom * 100.0;
        let band_color = if pct.abs() < 2.0 {
            flat_color
        } else if pct > 0.0 {
            rise_color
        } else {
            fall_color
        };
        let bx = x_at(s);
        let bw = (x_at(e) - x_at(s)).max(1.0);
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, bx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt);
        push_b(&mut f.buf, b"\" width=\"");
        push_f2(&mut f.buf, bw);
        push_b(&mut f.buf, b"\" height=\"");
        push_i(&mut f.buf, f.ph);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&hex6(band_color));
        push_b(&mut f.buf, b"\" fill-opacity=\"0.07\"/>");
    }

    f.y_grid(5, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for &b in &bounds[1..bounds.len() - 1] {
        let bx = x_at(b);
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, bx);
        push_b(&mut f.buf, b"\" y1=\"");
        push_i(&mut f.buf, f.pt);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, bx);
        push_b(&mut f.buf, b"\" y2=\"");
        push_i(&mut f.buf, f.pt + f.ph);
        push_b(&mut f.buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" stroke-dasharray=\"3,3\"/>");
    }

    let hx = hex6(line_color);
    push_b(&mut f.buf, b"<path d=\"M");
    for i in 0..n {
        if i > 0 {
            push_b(&mut f.buf, b" L");
        }
        push_f2(&mut f.buf, x_at(i));
        f.buf.push(b',');
        push_f2(&mut f.buf, y_at(values[i]));
    }
    push_b(&mut f.buf, b" L");
    push_f2(&mut f.buf, x_at(n - 1));
    f.buf.push(b',');
    push_i(&mut f.buf, f.pt + f.ph);
    push_b(&mut f.buf, b" L");
    push_f2(&mut f.buf, x_at(0));
    f.buf.push(b',');
    push_i(&mut f.buf, f.pt + f.ph);
    push_b(&mut f.buf, b" Z\" fill=\"url(#spEpochG)\" stroke=\"none\"/>");

    push_b(&mut f.buf, b"<polyline data-idx=\"0\" points=\"");
    for i in 0..n {
        if i > 0 {
            f.buf.push(b' ');
        }
        push_f2(&mut f.buf, x_at(i));
        f.buf.push(b',');
        push_f2(&mut f.buf, y_at(values[i]));
    }
    push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#");
    f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\"");
    push_f2(&mut f.buf, cfg.stroke_width + 0.6);
    push_b(&mut f.buf, b"\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>");

    if cfg.show_points {
        for &b in &bounds {
            push_b(&mut f.buf, b"<circle cx=\"");
            push_f2(&mut f.buf, x_at(b));
            push_b(&mut f.buf, b"\" cy=\"");
            push_f2(&mut f.buf, y_at(values[b]));
            push_b(&mut f.buf, b"\" r=\"4\" fill=\"#fff\" stroke=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke-width=\"2\"/>");
        }
    }

    for w in bounds.windows(2) {
        let (s, e) = (w[0], w[1]);
        let denom = values[s].abs().max(1e-6);
        let pct = (values[e] - values[s]) / denom * 100.0;
        let (badge_color, arrow) = if pct.abs() < 2.0 {
            (flat_color, "\u{2192}")
        } else if pct > 0.0 {
            (rise_color, "\u{25B2}")
        } else {
            (fall_color, "\u{25BC}")
        };
        let bhx = hex6(badge_color);
        let cx = (x_at(s) + x_at(e)) / 2.0;
        let by = f.pt + 16;
        let label = format!("{} {:+.0}%", arrow, pct);
        let bw = (14 + label.chars().count() as i32 * 7).max(46);
        push_b(&mut f.buf, b"<g class=\"sp-val\">");
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, cx - bw as f64 / 2.0);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, by - 12);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, bw);
        push_b(&mut f.buf, b"\" height=\"20\" rx=\"10\" fill=\"#");
        f.buf.extend_from_slice(&bhx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.13\" stroke=\"#");
        f.buf.extend_from_slice(&bhx);
        push_b(&mut f.buf, b"\" stroke-opacity=\"0.4\"/>");
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, by + 3);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"11\" fill=\"#");
        f.buf.extend_from_slice(&bhx);
        push_b(&mut f.buf, b"\">");
        escape_xml(&mut f.buf, &label);
        push_b(&mut f.buf, b"</text></g>");
    }

    let tick_step = ((n as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n).step_by(tick_step) {
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, x_at(i));
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&labels[i], 10));
        push_b(&mut f.buf, b"</text>");
    }

    f.html("[]")
}

fn chapter_boundaries(values: &[f64], n: usize) -> Vec<usize> {
    let window = (n / 6).max(2);
    let smooth = moving_average(values, window);
    let range = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - values.iter().cloned().fold(f64::INFINITY, f64::min);
    let min_swing = range.abs().max(1e-9) * 0.06;
    let swing_at = |t: usize| -> f64 {
        let lo = t.saturating_sub(window);
        let hi = (t + window).min(n - 1);
        (values[t] - values[lo]).abs().max((values[hi] - values[t]).abs())
    };
    let mut turns: Vec<usize> = Vec::new();
    let mut dir = 0i32;
    for i in 1..n {
        let d = smooth[i] - smooth[i - 1];
        if d.abs() < 1e-9 {
            continue;
        }
        let this_dir = if d > 0.0 { 1 } else { -1 };
        if dir != 0 && this_dir != dir {
            turns.push(i - 1);
        }
        dir = this_dir;
    }
    turns.retain(|&t| swing_at(t) >= min_swing);
    let max_chapters = 4usize;
    if turns.len() + 1 > max_chapters {
        let mut scored: Vec<(usize, f64)> = turns.iter().map(|&t| (t, swing_at(t))).collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut kept: Vec<usize> = scored.into_iter().take(max_chapters - 1).map(|(t, _)| t).collect();
        kept.sort_unstable();
        turns = kept;
    }
    let mut bounds = vec![0usize];
    for t in turns {
        if t > 0 && t < n - 1 && t > *bounds.last().unwrap() {
            bounds.push(t);
        }
    }
    if *bounds.last().unwrap() != n - 1 {
        bounds.push(n - 1);
    }
    bounds
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg_with<'a>(values: &'a [f64], labels: &'a [String]) -> LineConfig<'a> {
        LineConfig {
            variant: super::super::LineVariant::Epoch,
            values,
            labels,
            show_points: true,
            gridlines: true,
            ..Default::default()
        }
    }

    #[test]
    fn epoch_render_splits_a_clear_rise_then_fall_into_at_least_two_chapters() {
        let values = vec![10.0, 15.0, 22.0, 30.0, 40.0, 52.0, 46.0, 38.0, 30.0, 22.0, 15.0, 10.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels));
        assert!(out.matches("class=\"sp-val\"").count() >= 2, "a clear rise-then-fall series must be split into at least 2 narrative chapters: {out}");
    }

    #[test]
    fn epoch_render_labels_a_rising_chapter_with_a_positive_percentage_and_up_arrow() {
        let values = vec![10.0, 12.0, 15.0, 19.0, 24.0, 30.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels));
        assert!(out.contains("+200%"), "a value doubling from 10 to 30 must report roughly +200%: {out}");
    }

    #[test]
    fn epoch_render_keeps_the_line_a_single_color_unlike_the_momentum_variant() {
        let values = vec![5.0, 9.0, 7.0, 15.0, 11.0, 20.0];
        let labels: Vec<String> = (0..values.len()).map(|i| i.to_string()).collect();
        let out = render(&cfg_with(&values, &labels));
        assert_eq!(out.matches("data-idx=\"0\"").count(), 1, "epoch draws exactly one continuous polyline, not per-segment strokes: {out}");
    }

    #[test]
    fn epoch_render_returns_empty_string_below_two_points() {
        let values = vec![5.0];
        let labels: Vec<String> = vec!["a".to_string()];
        assert_eq!(render(&cfg_with(&values, &labels)), "");
    }

    #[test]
    fn epoch_chart_badges_disappear_after_no_value_is_chained() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("line/epoch.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("epoch demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        let hidden = crate::bindings::method_registry::apply_by_name(&html, "no_value", "{}").expect("no_value() must apply cleanly to an epoch chart");
        assert!(hidden.contains(".sp-val{display:none!important}"), "no_value() must inject the shared sp-val hiding rule: {hidden}");
    }

    #[test]
    fn epoch_chart_still_accepts_grid_then_show_legend_without_corruption() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("line/epoch.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("epoch demo payload");
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
            if !entry.file.replace('\\', "/").ends_with("line/epoch.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/line-epoch.html", html).unwrap();
        }
    }
}
