use super::common::{angle_at, project};
use super::config::RadarConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open};

#[crate::chart_demo(
    "axes=[\"1960\",\"1970\",\"1980\",\"1990\",\"2000\",\"2010\",\"2020\",\"2030\",\"2040\",\"2050\",\"2060\"], series=[[102.26,124.12,126.79,142.45,134.01,141.63,135.13,136.01,138.47,135.69,130.62],[54.97,48.53,47.32,49.62,52.10,54.33,63.17,67.17,79.47,91.58,102.19]], series_names=[\"Nacimientos\",\"Muertes\"], palette=[6274976,4020864], variant=\"petal\", title=\"Poblacion mundial en 100 anos\", width=920, height=1040"
)]
pub fn render(cfg: &RadarConfig) -> String {
    let n_axes = cfg.axes.len();
    let n_ser = cfg.series.len();
    if n_axes < 3 || n_ser == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let title_h = if cfg.title.is_empty() { 20.0 } else { 88.0 };
    let legend_h = 70.0;
    let plot_top = title_h;
    let plot_bottom = h - legend_h;
    let cy = plot_top + (plot_bottom - plot_top) / 2.0;
    let r = ((w / 2.0 - 70.0).min((plot_bottom - plot_top) / 2.0 - 46.0)).max(60.0);

    let global_max = cfg
        .series
        .iter()
        .flat_map(|(_, v)| v.iter().copied())
        .fold(0.0_f64, f64::max)
        .max(1.0);
    let step = nice_step(global_max);
    let n_rings = (global_max / step).ceil().max(1.0) as usize;

    let bg_hx = [0xff, 0xff, 0xff];
    let ink = "#33302c";
    let sub_ink = "#8a8378";

    let mut b = Vec::<u8>::with_capacity(4096 + n_axes * n_ser * 220);
    svg_open(&mut b, cfg.width, cfg.height);

    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"32\" y=\"46\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"26\" font-weight=\"800\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    for ring in 1..=n_rings {
        let frac = ring as f64 / n_rings as f64;
        let rr = r * frac;
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, cx);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, cy);
        push_b(&mut b, b"\" r=\"");
        push_f2(&mut b, rr);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"");
        push_b(&mut b, sub_ink.as_bytes());
        push_b(&mut b, b"\" stroke-opacity=\"0.35\" stroke-width=\"0.8\" stroke-dasharray=\"1.5,3\"/>");
    }

    let top_angle = angle_at(0, n_axes);
    for ring in 1..=n_rings {
        let frac = ring as f64 / n_rings as f64;
        let (_, ly) = project(cx, cy, r, frac, top_angle);
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, cx + 6.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly - 2.0);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" fill=\"");
        push_b(&mut b, sub_ink.as_bytes());
        push_b(&mut b, b"\">");
        push_i(&mut b, (step * ring as f64) as i32);
        push_b(&mut b, b"</text>");
    }

    for ai in 0..n_axes {
        let a = angle_at(ai, n_axes);
        let (ex, ey) = project(cx, cy, r, 1.0, a);
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, cx);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, cy);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, ex);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, ey);
        push_b(&mut b, b"\" stroke=\"");
        push_b(&mut b, sub_ink.as_bytes());
        push_b(&mut b, b"\" stroke-opacity=\"0.4\" stroke-width=\"0.8\"/>");
    }

    let mut order: Vec<usize> = (0..n_ser).collect();
    let totals: Vec<f64> = cfg.series.iter().map(|(_, v)| v.iter().sum::<f64>()).collect();
    order.sort_by(|x, y| totals[*y].partial_cmp(&totals[*x]).unwrap_or(std::cmp::Ordering::Equal));

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n_axes * n_ser);

    for &si in &order {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let hw = r * (0.100 - si.min(3) as f64 * 0.022).max(0.045);

        for ai in 0..n_axes {
            let v = cfg.series[si].1.get(ai).copied().unwrap_or(0.0).max(0.0);
            let frac = (v / global_max).min(1.0);
            let a = angle_at(ai, n_axes);
            let rv = r * frac;
            let dir = (a.cos(), -a.sin());
            let perp = (a.sin(), a.cos());

            let f1 = 0.32;
            let f2 = 0.68;
            let c1 = (cx + rv * f1 * dir.0 + hw * perp.0, cy + rv * f1 * dir.1 + hw * perp.1);
            let c2 = (cx + rv * f2 * dir.0 + hw * perp.0, cy + rv * f2 * dir.1 + hw * perp.1);
            let c3 = (cx + rv * f2 * dir.0 - hw * perp.0, cy + rv * f2 * dir.1 - hw * perp.1);
            let c4 = (cx + rv * f1 * dir.0 - hw * perp.0, cy + rv * f1 * dir.1 - hw * perp.1);
            let (tx, ty) = (cx + rv * dir.0, cy + rv * dir.1);

            push_b(&mut b, b"<path data-idx=\"");
            push_i(&mut b, (si * n_axes + ai) as i32);
            push_b(&mut b, b"\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" fill-opacity=\"0.42\" d=\"M");
            push_f2(&mut b, cx);
            push_b(&mut b, b",");
            push_f2(&mut b, cy);
            push_b(&mut b, b" C");
            push_f2(&mut b, c1.0);
            push_b(&mut b, b",");
            push_f2(&mut b, c1.1);
            push_b(&mut b, b" ");
            push_f2(&mut b, c2.0);
            push_b(&mut b, b",");
            push_f2(&mut b, c2.1);
            push_b(&mut b, b" ");
            push_f2(&mut b, tx);
            push_b(&mut b, b",");
            push_f2(&mut b, ty);
            push_b(&mut b, b" C");
            push_f2(&mut b, c3.0);
            push_b(&mut b, b",");
            push_f2(&mut b, c3.1);
            push_b(&mut b, b" ");
            push_f2(&mut b, c4.0);
            push_b(&mut b, b",");
            push_f2(&mut b, c4.1);
            push_b(&mut b, b" ");
            push_f2(&mut b, cx);
            push_b(&mut b, b",");
            push_f2(&mut b, cy);
            push_b(&mut b, b" Z\"/>");

            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, tx);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, ty);
            push_b(&mut b, b"\" r=\"4.2\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke=\"#");
            push_hex3(&mut b, &bg_hx);
            push_b(&mut b, b"\" stroke-width=\"1.6\"/>");

            let anchor = if dir.0.abs() < 0.18 { "middle" } else if dir.0 > 0.0 { "start" } else { "end" };
            let lx = tx + dir.0 * 11.0 + perp.0 * (hw + 4.0);
            let ly2 = ty + dir.1 * 11.0 + perp.1 * (hw + 4.0);
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, lx);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, ly2 + 3.0);
            push_b(&mut b, b"\" text-anchor=\"");
            push_b(&mut b, anchor.as_bytes());
            push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10.5\" font-weight=\"700\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\">");
            escape_xml(&mut b, &format!("{v:.2}"));
            push_b(&mut b, b"</text>");

            slots.push(HoverSlot::new(cfg.axes[ai].clone()).kv(&cfg.series[si].0, format!("{v:.2}")));
        }
    }

    let proj_start = n_axes - ((n_axes as f64 * 0.36).round() as usize).min(n_axes);
    let badge_r = 17.0;
    let label_r = r + 34.0;
    for ai in 0..n_axes {
        let a = angle_at(ai, n_axes);
        let (bx, by) = project(cx, cy, label_r, 1.0, a);
        let projected = ai >= proj_start;
        if projected {
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, bx);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, by);
            push_b(&mut b, b"\" r=\"");
            push_f2(&mut b, badge_r);
            push_b(&mut b, b"\" fill=\"none\" stroke=\"");
            push_b(&mut b, sub_ink.as_bytes());
            push_b(&mut b, b"\" stroke-width=\"1.4\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, bx);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, by + 3.5);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" fill=\"");
            push_b(&mut b, sub_ink.as_bytes());
            push_b(&mut b, b"\">");
        } else {
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, bx);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, by);
            push_b(&mut b, b"\" r=\"");
            push_f2(&mut b, badge_r);
            push_b(&mut b, b"\" fill=\"");
            push_b(&mut b, ink.as_bytes());
            push_b(&mut b, b"\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, bx);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, by + 3.5);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" fill=\"#");
            push_hex3(&mut b, &bg_hx);
            push_b(&mut b, b"\">");
        }
        escape_xml(&mut b, &cfg.axes[ai]);
        push_b(&mut b, b"</text>");
    }

    let legend_y = h - 30.0;
    let mut lx = 32.0;
    push_b(&mut b, b"<circle cx=\"");
    push_f2(&mut b, lx + 8.0);
    push_b(&mut b, b"\" cy=\"");
    push_f2(&mut b, legend_y);
    push_b(&mut b, b"\" r=\"9\" fill=\"none\" stroke=\"");
    push_b(&mut b, sub_ink.as_bytes());
    push_b(&mut b, b"\" stroke-width=\"1.3\"/>");
    push_b(&mut b, b"<text x=\"");
    push_f2(&mut b, lx + 26.0);
    push_b(&mut b, b"\" y=\"");
    push_f2(&mut b, legend_y + 4.0);
    push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" letter-spacing=\"0.5\" fill=\"");
    push_b(&mut b, sub_ink.as_bytes());
    push_b(&mut b, b"\">PROYECCIONES</text>");
    lx += 170.0;

    for si in 0..n_ser {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, lx + 6.0);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, legend_y);
        push_b(&mut b, b"\" r=\"6\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx + 18.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, legend_y + 4.0);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" letter-spacing=\"0.3\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &cfg.series[si].0.to_uppercase());
        push_b(&mut b, b"</text>");
        lx += 210.0;
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

fn push_hex3(buf: &mut Vec<u8>, rgb: &[u8; 3]) {
    for &c in rgb {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        buf.push(HEX[(c >> 4) as usize]);
        buf.push(HEX[(c & 0xf) as usize]);
    }
}

fn nice_step(max_v: f64) -> f64 {
    let raw = (max_v / 8.0).max(1e-9);
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = if norm < 1.5 {
        1.0
    } else if norm < 3.0 {
        2.0
    } else if norm < 7.0 {
        5.0
    } else {
        10.0
    };
    step * mag
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(axes: &'a [String], series: &'a [(String, Vec<f64>)]) -> RadarConfig<'a> {
        RadarConfig {
            title: "Test",
            axes,
            series,
            width: 800,
            height: 1000,
            ..RadarConfig::default()
        }
    }

    fn synth(n_axes: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        let axes: Vec<String> = (0..n_axes).map(|i| format!("{}", 1960 + i * 10)).collect();
        let births: Vec<f64> = (0..n_axes).map(|i| 100.0 + (i as f64 * 0.7).sin() * 20.0).collect();
        let deaths: Vec<f64> = (0..n_axes).map(|i| 50.0 + (i as f64 * 0.5).cos() * 15.0).collect();
        (axes, vec![("Births".to_string(), births), ("Deaths".to_string(), deaths)])
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("radar/petal.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/radar-petal.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_petal_per_axis_per_series() {
        let (axes, series) = synth(11);
        let html = render(&cfg(&axes, &series));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), axes.len() * series.len());
    }

    #[test]
    fn every_petal_tip_stays_within_the_declared_radius_of_its_own_center() {
        let (axes, series) = synth(11);
        let html = render(&cfg(&axes, &series));

        let w = 800.0_f64;
        let h = 1000.0_f64;
        let cx = w / 2.0;
        let title_h = 88.0;
        let legend_h = 70.0;
        let plot_top = title_h;
        let plot_bottom = h - legend_h;
        let cy = plot_top + (plot_bottom - plot_top) / 2.0;
        let r = ((w / 2.0 - 70.0).min((plot_bottom - plot_top) / 2.0 - 46.0)).max(60.0);

        for chunk in html.split("<path data-idx=").skip(1) {
            let d = chunk.split("d=\"M").nth(1).unwrap().split('"').next().unwrap();
            let toks: Vec<&str> = d.split(|c: char| c == ' ' || c == 'C' || c == 'Z').filter(|s| !s.is_empty()).collect();
            let tip = toks[3].split_once(',').unwrap();
            let tx: f64 = tip.0.parse().unwrap();
            let ty: f64 = tip.1.parse().unwrap();
            let dist = ((tx - cx).powi(2) + (ty - cy).powi(2)).sqrt();
            assert!(dist <= r + 1.0, "petal tip escaped the declared radius: {dist} > {r}");
        }
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let axes: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&axes, &series)).is_empty());
    }

    #[test]
    fn nice_step_always_yields_a_positive_step() {
        for v in [1.0, 9.0, 42.0, 142.45, 987.0, 10000.0] {
            assert!(nice_step(v) > 0.0);
        }
    }

    #[test]
    fn perf_rendering_many_axes_and_series_stays_fast() {
        let n_axes = 60;
        let axes: Vec<String> = (0..n_axes).map(|i| format!("{i}")).collect();
        let s1: Vec<f64> = (0..n_axes).map(|i| 50.0 + (i as f64).sin() * 10.0).collect();
        let s2: Vec<f64> = (0..n_axes).map(|i| 30.0 + (i as f64).cos() * 8.0).collect();
        let series = vec![("A".to_string(), s1), ("B".to_string(), s2)];
        let start = std::time::Instant::now();
        let html = render(&cfg(&axes, &series));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 300, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
