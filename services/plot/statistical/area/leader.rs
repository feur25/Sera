use super::config::AreaConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate, Frame};

#[crate::chart_demo(
    "x_labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\",\"Q5\",\"Q6\",\"Q7\",\"Q8\",\"Q9\",\"Q10\"], series=[[40,46,52,58,66,70,74,80,88,96],[38,44,55,63,68,72,90,94,92,90],[30,36,42,49,58,86,84,82,80,78]], series_names=[\"Atlas\",\"Nova\",\"Vertex\"], title=\"Leader\""
)]

pub fn render(cfg: &AreaConfig) -> String {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser < 2 {
        return String::new();
    }

    let mut vmax = f64::NEG_INFINITY;
    let mut vmin = f64::INFINITY;
    for (_, vals) in cfg.series {
        for &v in vals.iter().take(n_pts) {
            if v.is_finite() {
                vmax = vmax.max(v);
                vmin = vmin.min(v);
            }
        }
    }
    if !vmax.is_finite() || !vmin.is_finite() {
        return String::new();
    }
    vmin = vmin.min(0.0);
    let pad_v = (vmax - vmin).abs().max(1e-9) * 0.1;
    let y_min = vmin - pad_v;
    let y_max = vmax + pad_v;

    let leader_at = |i: usize| -> usize {
        let mut best = 0usize;
        let mut best_v = f64::NEG_INFINITY;
        for (s, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(i).copied().unwrap_or(f64::NEG_INFINITY);
            if v.is_finite() && v > best_v {
                best_v = v;
                best = s;
            }
        }
        best
    };

    let leaders: Vec<usize> = (0..n_pts).map(leader_at).collect();
    let mut changes: Vec<usize> = (1..n_pts).filter(|&i| leaders[i] != leaders[i - 1]).collect();
    let max_badges = 6usize;
    if changes.len() > max_badges {
        let stride = (changes.len() as f64 / max_badges as f64).ceil() as usize;
        changes = changes.into_iter().step_by(stride.max(1)).collect();
    }

    let legend_w: i32 = 160;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 48, 52, legend_w, n_pts * n_ser * 90 + 4096);
    f.open(cfg.title, true);
    f.y_grid(5, y_min, y_max, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let (pl, pt, pw, ph) = (f.pl, f.pt, f.pw, f.ph);
    let step_x = pw as f64 / (n_pts - 1).max(1) as f64;
    let y_range = (y_max - y_min).max(1e-9);
    let x_at = |i: usize| pl as f64 + i as f64 * step_x;
    let y_at = |v: f64| pt as f64 + (1.0 - (v - y_min) / y_range) * ph as f64;
    let base_y = y_at(0.0_f64.max(y_min).min(y_max));

    for (si, (_, vals)) in cfg.series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let n = n_pts.min(vals.len());
        if n < 2 {
            continue;
        }
        push_b(&mut f.buf, b"<path data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" d=\"M");
        push_f2(&mut f.buf, x_at(0));
        f.buf.push(b',');
        push_f2(&mut f.buf, base_y);
        for i in 0..n {
            push_b(&mut f.buf, b" L");
            push_f2(&mut f.buf, x_at(i));
            f.buf.push(b',');
            push_f2(&mut f.buf, y_at(vals[i]));
        }
        push_b(&mut f.buf, b" L");
        push_f2(&mut f.buf, x_at(n - 1));
        f.buf.push(b',');
        push_f2(&mut f.buf, base_y);
        push_b(&mut f.buf, b" Z\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.10\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-opacity=\"0.55\" stroke-width=\"1.3\"/>");
    }

    for i in 0..n_pts - 1 {
        let color = palette_color(cfg.palette, leaders[i]);
        let hx = hex6(color);
        let (x1, x2) = (x_at(i), x_at(i + 1));
        let v1 = cfg.series[leaders[i]].1.get(i).copied().unwrap_or(0.0);
        let v2 = cfg.series[leaders[i]].1.get(i + 1).copied().unwrap_or(0.0);
        let (y1, y2) = (y_at(v1), y_at(v2));
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, y1);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, y2);
        push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"5\" stroke-linecap=\"round\"/>");
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, y1);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, x2);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, y2);
        push_b(&mut f.buf, b"\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"3\" stroke-linecap=\"round\"/>");
    }

    for (bi, &i) in changes.iter().enumerate() {
        let new_leader = leaders[i];
        let color = palette_color(cfg.palette, new_leader);
        let hx = hex6(color);
        let v = cfg.series[new_leader].1.get(i).copied().unwrap_or(0.0);
        let cx = x_at(i);
        let cy = y_at(v);
        let up = bi % 2 == 0;
        let ly = if up { cy - 20.0 } else { cy + 20.0 };
        push_b(&mut f.buf, b"<g class=\"sp-val\">");
        push_b(&mut f.buf, b"<rect x=\"");
        push_f2(&mut f.buf, cx - 5.0);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, cy - 5.0);
        push_b(&mut f.buf, b"\" width=\"10\" height=\"10\" transform=\"rotate(45 ");
        push_f2(&mut f.buf, cx);
        f.buf.push(b' ');
        push_f2(&mut f.buf, cy);
        push_b(&mut f.buf, b")\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1.4\"/>");
        let at_edge = i >= n_pts - 2;
        let anchor: &[u8] = if at_edge { b"end" } else { b"middle" };
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, if at_edge { cx + 6.0 } else { cx });
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, ly);
        push_b(&mut f.buf, b"\" text-anchor=\"");
        f.buf.extend_from_slice(anchor);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"10\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\">");
        escape_xml(&mut f.buf, &cfg.series[new_leader].0);
        push_b(&mut f.buf, b"</text></g>");
    }

    let leg_x = cfg.width - legend_w + 14;
    for (si, (sname, _)) in cfg.series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let ly = pt + 6 + si as i32 * 18;
        push_b(&mut f.buf, b"<g data-legend=\"1\" data-series=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\"><rect x=\"");
        push_i(&mut f.buf, leg_x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ly);
        push_b(&mut f.buf, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\"/>");
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, leg_x + 16);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, ly + 10);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\">");
        escape_xml(&mut f.buf, truncate(sname, 18));
        push_b(&mut f.buf, b"</text></g>");
    }

    let tick_step = ((n_pts as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..n_pts).step_by(tick_step) {
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, x_at(i));
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, pt + ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 10));
        push_b(&mut f.buf, b"</text>");
    }

    f.html(&slots_to_json(cfg.hover))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg_with<'a>(x_labels: &'a [String], series: &'a [(String, Vec<f64>)]) -> AreaConfig<'a> {
        AreaConfig {
            x_labels,
            series,
            legend_position: "right",
            ..Default::default()
        }
    }

    #[test]
    fn leader_render_returns_empty_string_with_fewer_than_two_series() {
        let x_labels: Vec<String> = (0..5).map(|i| i.to_string()).collect();
        let series = vec![("solo".to_string(), vec![1.0, 2.0, 3.0, 4.0, 5.0])];
        assert_eq!(render(&cfg_with(&x_labels, &series)), "");
    }

    #[test]
    fn leader_render_detects_a_clean_lead_change_between_two_crossing_series() {
        let x_labels: Vec<String> = (0..6).map(|i| i.to_string()).collect();
        let series = vec![
            ("A".to_string(), vec![50.0, 45.0, 40.0, 35.0, 30.0, 25.0]),
            ("B".to_string(), vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]),
        ];
        let out = render(&cfg_with(&x_labels, &series));
        assert!(out.contains("class=\"sp-val\""), "a series that overtakes another must produce at least one lead-change badge: {out}");
        assert!(out.contains(">B<") || out.contains(">B</text>"), "the overtaking series' name must appear on its lead-change badge: {out}");
    }

    #[test]
    fn leader_render_draws_one_area_path_per_series() {
        let x_labels: Vec<String> = (0..4).map(|i| i.to_string()).collect();
        let series = vec![
            ("X".to_string(), vec![1.0, 2.0, 3.0, 4.0]),
            ("Y".to_string(), vec![4.0, 3.0, 2.0, 1.0]),
            ("Z".to_string(), vec![2.0, 2.0, 2.0, 2.0]),
        ];
        let out = render(&cfg_with(&x_labels, &series));
        for i in 0..3 {
            assert!(out.contains(&format!("data-series=\"{i}\"")), "expected one area path for series {i}: {out}");
        }
    }

    #[test]
    fn leader_chart_badges_disappear_after_no_value_is_chained() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("area/leader.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("leader demo payload");
        let html = crate::bindings::fn_registry::iter_entries().find(|f| f.name == input.builder).map(|f| (f.invoke)(&input.json)).unwrap();
        assert!(html.contains("class=\"sp-val\""), "leader's lead-change badges must be wrapped in sp-val: {html}");
        let hidden = crate::bindings::method_registry::apply_by_name(&html, "no_value", "{}").expect("no_value() must apply cleanly to a leader chart");
        assert!(hidden.contains(".sp-val{display:none!important}"), "no_value() must inject the shared sp-val hiding rule: {hidden}");
    }

    #[test]
    fn leader_chart_still_accepts_grid_then_show_legend_without_corruption() {
        let input = crate::plot::chart_demo_registry::iter_entries()
            .find(|e| e.file.replace('\\', "/").ends_with("area/leader.rs"))
            .and_then(crate::plot::chart_demo_registry::demo_payload)
            .expect("leader demo payload");
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
            if !entry.file.replace('\\', "/").ends_with("area/leader.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/area-leader.html", html).unwrap();
        }
    }
}
