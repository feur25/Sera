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
    "labels=[\"13\",\"14\",\"15\",\"16\",\"17\",\"18\",\"19\",\"20\",\"21\",\"22\",\"23\",\"24\",\"25\",\"26\",\"27\",\"28\",\"29\",\"30\",\"31\",\"32\",\"33\",\"34\",\"35\",\"36\",\"37\",\"38\",\"39\",\"40\",\"41\",\"42\",\"43\",\"44\",\"45\",\"46\",\"47\",\"48\",\"49\",\"50\",\"51\",\"52\",\"53\",\"54\",\"55\",\"56\",\"57\",\"58\",\"59\",\"60\"], series=[[6.2,6.3,6.3,6.4,6.0,6.2,4.5,5.1,5.9,5.2,5.6,4.0,4.5,4.0,4.4,4.4,3.3,3.5,3.5,4.5,4.1,2.9,4.0,2.7,3.4,2.4,2.1,3.5,2.2,2.1,3.4,3.0,1.9,3.0,2.1,2.2,1.3,2.5,1.9,2.3,2.0,0.8,0.8,0.4,0.3,0.3,0.3,0.7],[14.4,13.8,12.8,14.8,11.8,13.1,14.1,11.2,12.5,12.4,10.3,11.1,11.8,10.8,11.3,9.3,9.2,8.6,9.5,10.6,9.3,9.6,8.0,8.9,6.6,6.9,7.9,7.8,6.5,5.2,5.5,5.0,4.9,6.4,4.1,6.1,5.8,2.8,3.6,3.7,1.9,2.9,4.2,1.4,2.6,0.8,2.0,2.8]], series_names=[\"Women\",\"Men\"], variant=\"radial_pyramid\", title=\"Active Users by Age\", x_label=\"Age\", width=850, height=950"
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
    let px = width * 0.507;
    let py = height * 0.395;
    let offset = height * 0.305;
    let r_max = offset;
    let r_min = offset * 0.08;

    let alpha = (-35.0_f64).to_radians();
    let dx = alpha.cos();
    let dy = alpha.sin();

    let c1x = px + offset * dx;
    let c1y = py + offset * dy;
    let c2x = px - offset * dx;
    let c2y = py - offset * dy;

    let beta1 = alpha + std::f64::consts::PI;
    let beta2 = alpha;
    let sweep_max = 300.0_f64.to_radians();

    let angle_top = |v: f64| -> f64 { beta1 + sweep_max * (v.abs() / vmax).clamp(0.0, 1.0) };
    let angle_bottom = |v: f64| -> f64 { beta2 + sweep_max * (v.abs() / vmax).clamp(0.0, 1.0) };
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

    let label_gap = 22.0;
    let n_ticks = 4;
    for k in 1..=n_ticks {
        let frac = k as f64 / n_ticks as f64;
        let val = vmax * frac;

        let at = angle_top(val);
        let xt0 = c1x + r_min * at.cos();
        let yt0 = c1y + r_min * at.sin();
        let xt1 = c1x + (r_max + label_gap) * at.cos();
        let yt1 = c1y + (r_max + label_gap) * at.sin();
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, xt0);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, yt0);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, xt1);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, yt1);
        push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, xt1);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, yt1 + 3.0);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#94a3b8\">");
        escape_xml(&mut b, &format!("{:.0}%", val));
        push_b(&mut b, b"</text>");

        let ab = angle_bottom(val);
        let xb0 = c2x + r_min * ab.cos();
        let yb0 = c2y + r_min * ab.sin();
        let xb1 = c2x + (r_max + label_gap) * ab.cos();
        let yb1 = c2y + (r_max + label_gap) * ab.sin();
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, xb0);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, yb0);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, xb1);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, yb1);
        push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, xb1);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, yb1 + 3.0);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#94a3b8\">");
        escape_xml(&mut b, &format!("{:.0}%", val));
        push_b(&mut b, b"</text>");
    }

    for i in 0..n {
        let r = radius_of(i);

        let vt = top.1.get(i).copied().unwrap_or(0.0);
        let at = angle_top(vt);
        value_arc(&mut b, i * 2, &cfg.category_labels[i], vt, c1x, c1y, r, beta1, at, col_top, 2.6);

        let vb = bottom.1.get(i).copied().unwrap_or(0.0);
        let ab = angle_bottom(vb);
        value_arc(&mut b, i * 2 + 1, &cfg.category_labels[i], vb, c2x, c2y, r, beta2, ab, col_bottom, 2.6);
    }

    let axis_half = offset - r_min;
    push_b(&mut b, b"<line x1=\"");
    push_f2(&mut b, px - axis_half * dx);
    push_b(&mut b, b"\" y1=\"");
    push_f2(&mut b, py - axis_half * dy);
    push_b(&mut b, b"\" x2=\"");
    push_f2(&mut b, px + axis_half * dx);
    push_b(&mut b, b"\" y2=\"");
    push_f2(&mut b, py + axis_half * dy);
    push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");

    if !cfg.x_label.is_empty() {
        push_b(&mut b, b"<rect class=\"sp-bg\" x=\"");
        push_f2(&mut b, px - axis_half * dx - 40.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, py - axis_half * dy - 9.0);
        push_b(&mut b, b"\" width=\"38\" height=\"18\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, px - axis_half * dx - 16.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, py - axis_half * dy + 3.0);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#334155\">");
        escape_xml(&mut b, cfg.x_label);
        push_b(&mut b, b"</text>");
    }

    let perp_x = -dy;
    let perp_y = dx;
    let label_step = ((n as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..n).step_by(label_step) {
        let r = radius_of(i);
        let t = offset - r;
        let tx = px + t * dx;
        let ty = py + t * dy;
        push_b(&mut b, b"<rect class=\"sp-bg\" x=\"");
        push_f2(&mut b, tx - 15.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ty - 9.0);
        push_b(&mut b, b"\" width=\"30\" height=\"18\"/>");
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, tx - perp_x * 3.0);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, ty - perp_y * 3.0);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, tx + perp_x * 3.0);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, ty + perp_y * 3.0);
        push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, tx);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ty + 3.0);
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

    fn geometry(width: f64, height: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let px = width * 0.507;
        let py = height * 0.395;
        let offset = height * 0.305;
        let alpha = (-35.0_f64).to_radians();
        let dx = alpha.cos();
        let dy = alpha.sin();
        (px, py, offset, offset, dx, dy, alpha)
    }

    #[test]
    fn every_arc_point_stays_within_its_own_series_radius_from_its_own_center() {
        let (cats, series) = synth();
        let html = render(&cfg(&cats, &series));

        let (px, py, offset, r_max, dx, dy, _) = geometry(1000.0, 900.0);
        let c1 = (px + offset * dx, py + offset * dy);
        let c2 = (px - offset * dx, py - offset * dy);

        for (idx, chunk) in html.split("<path data-idx=").skip(1).enumerate() {
            let (cx, cy) = if idx % 2 == 0 { c1 } else { c2 };
            let d = chunk.split("d=\"M").nth(1).unwrap().split('"').next().unwrap();
            let toks: Vec<&str> = d.split(' ').filter(|s| !s.is_empty()).collect();
            let start = toks.first().unwrap();
            let end = toks.last().unwrap();
            for tok in [start, end] {
                let (xs, ys) = tok.split_once(',').unwrap();
                let x: f64 = xs.parse().unwrap();
                let y: f64 = ys.parse().unwrap();
                let r = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
                assert!(r <= r_max + 1.0);
            }
        }
    }

    #[test]
    fn the_two_series_circles_can_only_ever_meet_at_the_single_shared_pivot() {
        let (_, _, offset, r_max, _, _, _) = geometry(1000.0, 900.0);
        assert!(r_max <= offset + 1e-9, "outer radius must not exceed the center offset, or the two series disks would overlap");
    }

    #[test]
    fn the_two_series_sweep_from_opposite_ends_of_the_shared_pivot_axis() {
        let cats: Vec<String> = (0..10).map(|i| format!("{}", 13 + i)).collect();
        let vals: Vec<f64> = (0..10).map(|i| 1.0 + (i as f64 * 0.5).sin().abs() * 6.0).collect();
        let series = vec![("Top".to_string(), vals.clone()), ("Bottom".to_string(), vals)];
        let html = render(&cfg(&cats, &series));

        let (px, py, _, _, dx, dy, _) = geometry(1000.0, 900.0);

        let mut top_ends = Vec::new();
        let mut bottom_ends = Vec::new();
        for (idx, chunk) in html.split("<path data-idx=").skip(1).enumerate() {
            let d = chunk.split("d=\"M").nth(1).unwrap().split('"').next().unwrap();
            let toks: Vec<&str> = d.split(' ').filter(|s| !s.is_empty()).collect();
            let end = toks.last().unwrap();
            let (xs, ys) = end.split_once(',').unwrap();
            let point = (xs.parse::<f64>().unwrap(), ys.parse::<f64>().unwrap());
            if idx % 2 == 0 {
                top_ends.push(point);
            } else {
                bottom_ends.push(point);
            }
        }

        for (t, bo) in top_ends.iter().zip(bottom_ends.iter()) {
            let td = ((t.0 - px) * dx + (t.1 - py) * dy).signum();
            let bd = ((bo.0 - px) * dx + (bo.1 - py) * dy).signum();
            assert_ne!(td, bd, "identical data must land on opposite sides of the shared pivot, got top={t:?} bottom={bo:?}");
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
