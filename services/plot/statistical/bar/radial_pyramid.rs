use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title, truncate,
};

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
    "labels=[\"13\",\"14\",\"15\",\"16\",\"17\",\"18\",\"19\",\"20\",\"21\",\"22\",\"23\",\"24\",\"25\",\"26\",\"27\",\"28\",\"29\",\"30\",\"31\",\"32\",\"33\",\"34\",\"35\",\"36\",\"37\",\"38\",\"39\",\"40\",\"41\",\"42\",\"43\",\"44\",\"45\",\"46\",\"47\",\"48\",\"49\",\"50\",\"51\",\"52\",\"53\",\"54\",\"55\",\"56\",\"57\",\"58\",\"59\",\"60\"], series=[[6.2,6.3,6.3,6.4,6.0,6.2,4.5,5.1,5.9,5.2,5.6,4.0,4.5,4.0,4.4,4.4,3.3,3.5,3.5,4.5,4.1,2.9,4.0,2.7,3.4,2.4,2.1,3.5,2.2,2.1,3.4,3.0,1.9,3.0,2.1,2.2,1.3,2.5,1.9,2.3,2.0,0.8,0.8,0.4,0.3,0.3,0.3,0.7],[14.4,13.8,12.8,14.8,11.8,13.1,14.1,11.2,12.5,12.4,10.3,11.1,11.8,10.8,11.3,9.3,9.2,8.6,9.5,10.6,9.3,9.6,8.0,8.9,6.6,6.9,7.9,7.8,6.5,5.2,5.5,5.0,4.9,6.4,4.1,6.1,5.8,2.8,3.6,3.7,1.9,2.9,4.2,1.4,2.6,0.8,2.0,2.8]], series_names=[\"Women\",\"Men\"], variant=\"radial_pyramid\", title=\"Active Users by Age\", x_label=\"Age\", width=1050, height=980"
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
    let vmax = vmax_top.max(vmax_bottom);

    let width = cfg.width as f64;
    let height = cfg.height as f64;
    let max_radius = height * 0.30;
    let scale = max_radius / vmax;

    let pad_l = max_radius + 24.0;
    let pad_r = max_radius + 60.0;
    let x_left = pad_l;
    let x_right = (width - pad_r).max(x_left + 40.0);
    let y_axis = height * 0.52;

    let x_of = |i: usize| -> f64 { x_left + (x_right - x_left) * i as f64 / (n - 1).max(1) as f64 };

    let start_angle = std::f64::consts::PI;
    let sweep = 145.0_f64.to_radians();
    let angle_top = start_angle + sweep;
    let angle_bottom = start_angle - sweep;

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

    let n_ticks = 4;
    for k in 1..=n_ticks {
        let frac = k as f64 / n_ticks as f64;
        let val = vmax * frac;
        let dy = max_radius * frac;

        for sy in [-1.0_f64, 1.0_f64] {
            let gy = y_axis + sy * dy;
            push_b(&mut b, b"<line x1=\"");
            push_f2(&mut b, x_left);
            push_b(&mut b, b"\" y1=\"");
            push_f2(&mut b, gy);
            push_b(&mut b, b"\" x2=\"");
            push_f2(&mut b, x_right);
            push_b(&mut b, b"\" y2=\"");
            push_f2(&mut b, gy);
            push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"1\" stroke-dasharray=\"2,3\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, x_right + 10.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, gy + 3.0);
            push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#94a3b8\">");
            escape_xml(&mut b, &format!("{:.0}%", val));
            push_b(&mut b, b"</text>");
        }
    }

    let label_step = ((n as f64 / 10.0).ceil() as usize).max(1);

    for i in 0..n {
        let xi = x_of(i);

        let vt = top.1.get(i).copied().unwrap_or(0.0);
        let rt = vt.abs() * scale;
        value_arc(&mut b, i * 2, &cfg.category_labels[i], vt, xi, y_axis, rt, start_angle, angle_top, col_top, 2.6);

        let vb = bottom.1.get(i).copied().unwrap_or(0.0);
        let rb = vb.abs() * scale;
        value_arc(&mut b, i * 2 + 1, &cfg.category_labels[i], vb, xi, y_axis, rb, start_angle, angle_bottom, col_bottom, 2.6);
    }

    push_b(&mut b, b"<line x1=\"");
    push_f2(&mut b, x_left - 10.0);
    push_b(&mut b, b"\" y1=\"");
    push_f2(&mut b, y_axis);
    push_b(&mut b, b"\" x2=\"");
    push_f2(&mut b, x_right + 10.0);
    push_b(&mut b, b"\" y2=\"");
    push_f2(&mut b, y_axis);
    push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");

    if !cfg.x_label.is_empty() {
        push_b(&mut b, b"<rect class=\"sp-bg\" x=\"");
        push_f2(&mut b, x_left - 40.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, y_axis - 9.0);
        push_b(&mut b, b"\" width=\"38\" height=\"18\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, x_left - 16.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, y_axis + 3.0);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#334155\">");
        escape_xml(&mut b, cfg.x_label);
        push_b(&mut b, b"</text>");
    }

    for i in (0..n).step_by(label_step) {
        let xi = x_of(i);
        push_b(&mut b, b"<rect class=\"sp-bg\" x=\"");
        push_f2(&mut b, xi - 15.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, y_axis - 9.0);
        push_b(&mut b, b"\" width=\"30\" height=\"18\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, xi);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, y_axis + 3.0);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"#475569\">");
        escape_xml(&mut b, truncate(&cfg.category_labels[i], 6));
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
            width: 1000,
            height: 900,
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
    fn every_value_arc_is_anchored_on_the_shared_axis_at_its_own_category_position() {
        let (cats, series) = synth();
        let html = render(&cfg(&cats, &series));

        let width = 1000.0_f64;
        let height = 900.0_f64;
        let max_radius = height * 0.30;
        let pad_l = max_radius + 24.0;
        let pad_r = max_radius + 60.0;
        let x_left = pad_l;
        let x_right = (width - pad_r).max(x_left + 40.0);
        let y_axis = height * 0.52;
        let n = cats.len();
        let x_of = |i: usize| -> f64 { x_left + (x_right - x_left) * i as f64 / (n - 1).max(1) as f64 };

        for (idx, chunk) in html.split("<path data-idx=").skip(1).enumerate() {
            let cat_i = idx / 2;
            let cx = x_of(cat_i);
            let d = chunk.split("d=\"M").nth(1).unwrap().split('"').next().unwrap();
            let toks: Vec<&str> = d.split(' ').filter(|s| !s.is_empty()).collect();
            let start = toks.first().unwrap();
            let (xs, ys) = start.split_once(',').unwrap();
            let x: f64 = xs.parse().unwrap();
            let y: f64 = ys.parse().unwrap();
            assert!((y - y_axis).abs() < 1.0, "every arc must start exactly on the shared axis");
            assert!(x <= cx + 1.0, "the start point must sit at or left of its own category anchor");
        }
    }

    #[test]
    fn the_two_series_never_cross_the_shared_axis_into_each_others_half() {
        let (cats, series) = synth();
        let html = render(&cfg(&cats, &series));

        let height = 900.0_f64;
        let y_axis = height * 0.52;

        for (idx, chunk) in html.split("<path data-idx=").skip(1).enumerate() {
            let is_top = idx % 2 == 0;
            let d = chunk.split("d=\"M").nth(1).unwrap().split('"').next().unwrap();
            let toks: Vec<&str> = d.split(' ').filter(|s| !s.is_empty()).collect();
            let end = toks.last().unwrap();
            let (_, ys) = end.split_once(',').unwrap();
            let y: f64 = ys.parse().unwrap();
            if is_top {
                assert!(y <= y_axis + 1.0, "the top series must never dip below the shared axis");
            } else {
                assert!(y >= y_axis - 1.0, "the bottom series must never rise above the shared axis");
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
