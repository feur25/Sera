use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title, truncate,
};

#[allow(clippy::too_many_arguments)]
fn arc(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64, stroke: u32, width: f64, opacity: f64, round: bool) {
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    let sweep_flag: u8 = if a1 > a0 { 1 } else { 0 };
    push_b(buf, b"<path fill=\"none\" stroke=\"#");
    buf.extend_from_slice(&hex6(stroke));
    push_b(buf, b"\" stroke-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, width);
    if round {
        push_b(buf, b"\" stroke-linecap=\"round");
    }
    push_b(buf, b"\" d=\"M");
    push_f2(buf, x0);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b" A");
    push_f2(buf, r);
    push_b(buf, b",");
    push_f2(buf, r);
    push_b(buf, b" 0 0,");
    buf.push(sweep_flag + b'0');
    push_b(buf, b" ");
    push_f2(buf, x1);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"\"/>");
}

#[allow(clippy::too_many_arguments)]
fn value_arc(buf: &mut Vec<u8>, idx: usize, lbl: &str, v: f64, cx: f64, cy: f64, r: f64, a0: f64, a1: f64, stroke: u32, width: f64) {
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    let sweep_flag: u8 = if a1 > a0 { 1 } else { 0 };
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, idx as i32);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, lbl);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, v);
    push_b(buf, b"\" fill=\"none\" stroke=\"#");
    buf.extend_from_slice(&hex6(stroke));
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, width);
    push_b(buf, b"\" stroke-linecap=\"round\" d=\"M");
    push_f2(buf, x0);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b" A");
    push_f2(buf, r);
    push_b(buf, b",");
    push_f2(buf, r);
    push_b(buf, b" 0 0,");
    buf.push(sweep_flag + b'0');
    push_b(buf, b" ");
    push_f2(buf, x1);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"\"/>");
}

#[crate::chart_demo(
    "labels=[\"13\",\"14\",\"15\",\"16\",\"17\",\"18\",\"19\",\"20\",\"21\",\"22\",\"23\",\"24\",\"25\",\"26\",\"27\",\"28\",\"29\",\"30\",\"31\",\"32\",\"33\",\"34\",\"35\",\"36\",\"37\",\"38\",\"39\",\"40\",\"41\",\"42\",\"43\",\"44\",\"45\",\"46\",\"47\",\"48\",\"49\",\"50\",\"51\",\"52\",\"53\",\"54\",\"55\",\"56\",\"57\",\"58\",\"59\",\"60\"], series=[[2.4,2.7,2.9,3.2,3.5,3.7,4.0,4.2,4.5,4.7,4.9,5.0,5.1,5.2,5.2,5.2,5.1,5.0,4.9,4.7,4.5,4.3,4.0,3.8,3.5,3.2,3.0,2.7,2.5,2.2,2.0,1.8,1.6,1.5,1.3,1.2,1.1,1.0,0.9,0.9,0.8,0.8,0.7,0.7,0.7,0.7,0.6,0.6],[4.9,5.4,5.9,6.4,7.0,7.5,8.1,8.7,9.2,9.8,10.3,10.8,11.3,11.8,12.2,12.5,12.8,13.0,13.2,13.3,13.3,13.2,13.1,12.9,12.7,12.3,12.0,11.5,11.1,10.6,10.1,9.5,8.9,8.4,7.8,7.2,6.7,6.1,5.6,5.1,4.7,4.3,3.9,3.5,3.2,2.9,2.6,2.4]], series_names=[\"Women\",\"Men\"], variant=\"radial_pyramid\", title=\"Active Users by Age\", x_label=\"Age\", width=1200, height=860"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.category_labels.len();
    if n == 0 || cfg.series.len() < 2 {
        return String::new();
    }
    let top = &cfg.series[0];
    let bottom = &cfg.series[1];

    let vmax_top = top.1.iter().fold(0.0_f64, |m, &v| m.max(v.abs())).max(1e-9);
    let vmax_bottom = bottom.1.iter().fold(0.0_f64, |m, &v| m.max(v.abs())).max(1e-9);

    let cx = 420.0;
    let cy = 430.0;
    let r_min = 30.0;
    let r_max = 340.0;

    let a0 = 0.0_f64;
    let a_top = -std::f64::consts::PI;
    let a_bottom = std::f64::consts::PI;

    let angle_top = |v: f64| -> f64 { a0 + (a_top - a0) * (v.abs() / vmax_top).clamp(0.0, 1.0) };
    let angle_bottom = |v: f64| -> f64 { a0 + (a_bottom - a0) * (v.abs() / vmax_bottom).clamp(0.0, 1.0) };
    let radius_of = |i: usize| -> f64 { r_min + (r_max - r_min) * i as f64 / (n - 1).max(1) as f64 };

    let col_top = palette_color(cfg.palette, 0);
    let col_bottom = palette_color(cfg.palette, 1);

    let mut b = Vec::<u8>::with_capacity(n * 320 + 4096);
    svg_open_rescalable(&mut b, cfg.width, cfg.height, 0, 0, cfg.width, cfg.height);
    push_b(&mut b, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut b, cfg.title, cfg.width / 2, 24);

    push_b(&mut b, b"<text x=\"24\" y=\"48\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#");
    b.extend_from_slice(&hex6(col_top));
    push_b(&mut b, b"\">");
    escape_xml(&mut b, &top.0);
    push_b(&mut b, b"</text>");
    push_b(&mut b, b"<text x=\"24\" y=\"68\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"13\" font-weight=\"700\" fill=\"#");
    b.extend_from_slice(&hex6(col_bottom));
    push_b(&mut b, b"\">");
    escape_xml(&mut b, &bottom.0);
    push_b(&mut b, b"</text>");

    push_b(&mut b, b"<line x1=\"");
    push_f2(&mut b, cx);
    push_b(&mut b, b"\" y1=\"");
    push_f2(&mut b, cy);
    push_b(&mut b, b"\" x2=\"");
    push_f2(&mut b, cx + r_max);
    push_b(&mut b, b"\" y2=\"");
    push_f2(&mut b, cy);
    push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");

    if !cfg.x_label.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, cx - 8.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, cy + 4.0);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#334155\">");
        escape_xml(&mut b, cfg.x_label);
        push_b(&mut b, b"</text>");
    }

    let label_step = ((n as f64 / 12.0).ceil() as usize).max(1);

    for i in 0..n {
        let r = radius_of(i);
        arc(&mut b, cx, cy, r, a0, a_top, 0xe2e8f0, 2.6, 1.0, true);
        arc(&mut b, cx, cy, r, a0, a_bottom, 0xe2e8f0, 2.6, 1.0, true);

        let vt = top.1.get(i).copied().unwrap_or(0.0);
        let at = angle_top(vt);
        value_arc(&mut b, i * 2, &cfg.category_labels[i], vt, cx, cy, r, a0, at, col_top, 5.4);

        let vb = bottom.1.get(i).copied().unwrap_or(0.0);
        let ab = angle_bottom(vb);
        value_arc(&mut b, i * 2 + 1, &cfg.category_labels[i], vb, cx, cy, r, a0, ab, col_bottom, 5.4);

        if i % label_step == 0 {
            let tx = cx + r;
            push_b(&mut b, b"<line x1=\"");
            push_f2(&mut b, tx);
            push_b(&mut b, b"\" y1=\"");
            push_f2(&mut b, cy - 3.0);
            push_b(&mut b, b"\" x2=\"");
            push_f2(&mut b, tx);
            push_b(&mut b, b"\" y2=\"");
            push_f2(&mut b, cy + 3.0);
            push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, tx);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, cy + 16.0);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">");
            escape_xml(&mut b, truncate(&cfg.category_labels[i], 6));
            push_b(&mut b, b"</text>");
        }
    }

    let n_ticks = 5;
    for k in 1..=n_ticks {
        let frac = k as f64 / n_ticks as f64;
        let vt = vmax_top * frac;
        let at = angle_top(vt);
        let rt = r_max + 12.0;
        let xt = cx + rt * at.cos();
        let yt = cy + rt * at.sin();
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, xt);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, yt);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#");
        b.extend_from_slice(&hex6(col_top));
        push_b(&mut b, b"\" transform=\"rotate(");
        push_f2(&mut b, at.to_degrees() + 90.0);
        push_b(&mut b, b" ");
        push_f2(&mut b, xt);
        push_b(&mut b, b" ");
        push_f2(&mut b, yt);
        push_b(&mut b, b")\">");
        escape_xml(&mut b, &format!("{:.0}", vt));
        push_b(&mut b, b"</text>");

        let vb = vmax_bottom * frac;
        let ab = angle_bottom(vb) - if k == n_ticks { 0.07 } else { 0.0 };
        let rb = r_max + 12.0;
        let xb = cx + rb * ab.cos();
        let yb = cy + rb * ab.sin();
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, xb);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, yb);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#");
        b.extend_from_slice(&hex6(col_bottom));
        push_b(&mut b, b"\" transform=\"rotate(");
        push_f2(&mut b, ab.to_degrees() - 90.0);
        push_b(&mut b, b" ");
        push_f2(&mut b, xb);
        push_b(&mut b, b" ");
        push_f2(&mut b, yb);
        push_b(&mut b, b")\">");
        escape_xml(&mut b, &format!("{:.0}", vb));
        push_b(&mut b, b"</text>");
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
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

    fn cfg<'a>(category_labels: &'a [String], series: &'a [(String, Vec<f64>)]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            category_labels,
            series,
            width: 1200,
            height: 860,
            ..BarConfig::default()
        }
    }

    fn synth() -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        let cats: Vec<String> = (0..10).map(|i| format!("{}", 13 + i)).collect();
        let top: Vec<f64> = (0..10).map(|i| 1.0 + (i as f64 * 0.5).sin().abs() * 6.0).collect();
        let bottom: Vec<f64> = (0..10).map(|i| 2.0 + (i as f64 * 0.7).cos().abs() * 9.0).collect();
        (cats, vec![("Women".to_string(), top), ("Men".to_string(), bottom)])
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("bar/radial_pyramid.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/bar-radial_pyramid.html", html).unwrap();
        }
    }

    #[test]
    fn renders_two_mirrored_value_arcs_per_category() {
        let (cats, series) = synth();
        let html = render(&cfg(&cats, &series));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), cats.len() * 2);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn every_value_arc_stays_within_the_declared_radius() {
        let (cats, series) = synth();
        let html = render(&cfg(&cats, &series));
        let cx = 420.0_f64;
        let cy = 430.0_f64;
        let r_max = 340.0_f64;
        for chunk in html.split("<path data-idx=").skip(1) {
            let d = chunk.split("d=\"M").nth(1).unwrap().split('"').next().unwrap();
            for pair in d.split(|c: char| c == ' ' || c == ',' || c == 'A').filter(|s| !s.is_empty()) {
                if let Some((xs, ys)) = pair.split_once(',') {
                    if let (Ok(x), Ok(y)) = (xs.parse::<f64>(), ys.parse::<f64>()) {
                        let r = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                        assert!(r <= r_max + 1.0);
                    }
                }
            }
        }
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let cats: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&cats, &series)).is_empty());
    }

    #[test]
    fn single_series_returns_empty_string() {
        let cats: Vec<String> = vec!["a".to_string()];
        let series: Vec<(String, Vec<f64>)> = vec![("Only".to_string(), vec![1.0])];
        assert!(render(&cfg(&cats, &series)).is_empty());
    }

    #[test]
    fn perf_rendering_many_categories_stays_fast() {
        let cats: Vec<String> = (0..300).map(|i| format!("{i}")).collect();
        let top: Vec<f64> = (0..300).map(|i| 1.0 + (i as f64 * 0.3).sin().abs() * 8.0).collect();
        let bottom: Vec<f64> = (0..300).map(|i| 1.0 + (i as f64 * 0.4).cos().abs() * 8.0).collect();
        let series = vec![("A".to_string(), top), ("B".to_string(), bottom)];
        let start = std::time::Instant::now();
        let html = render(&cfg(&cats, &series));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 300, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
