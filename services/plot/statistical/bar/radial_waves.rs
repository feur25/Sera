use super::config::BarConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title};

#[allow(clippy::too_many_arguments)]
fn wave_arc(buf: &mut Vec<u8>, ox: f64, oy: f64, r: f64, a0: f64, a1: f64, stroke_w: f64, color: u32, data_idx: i32, value: f64, label: &str) {
    let x0 = ox + r * a0.cos();
    let y0 = oy + r * a0.sin();
    let x1 = ox + r * a1.cos();
    let y1 = oy + r * a1.sin();
    let large = if (a1 - a0).abs() > std::f64::consts::PI { 1 } else { 0 };
    let sweep = if a1 > a0 { 1 } else { 0 };
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" fill=\"none\" stroke-linecap=\"round\" d=\"M");
    push_f2(buf, x0);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b" A");
    push_f2(buf, r);
    push_b(buf, b",");
    push_f2(buf, r);
    push_b(buf, b" 0 ");
    buf.push(large + b'0');
    push_b(buf, b",");
    buf.push(sweep + b'0');
    push_b(buf, b" ");
    push_f2(buf, x1);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, stroke_w);
    push_b(buf, b"\" stroke-opacity=\"0.9\"/>");
}

#[crate::chart_demo(
    "labels=[\"13\",\"18\",\"23\",\"28\",\"33\",\"38\",\"43\",\"48\",\"53\",\"58\"], series_names=[\"Femmes\",\"Hommes\"], series=[[1.0,2.5,4.0,6.5,9.0,11.5,13.0,10.0,6.0,3.0],[1.5,3.0,5.5,8.0,11.0,13.0,12.0,9.0,5.5,2.5]], color_low=15654421, color_high=6995513, variant=\"radial_waves\", width=820, height=820"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.series.iter().map(|(_, v)| v.len()).min().unwrap_or(0));
    if n == 0 || cfg.series.len() < 2 {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let wf = w as f64;
    let hf = h as f64;

    let mut vmax = f64::NEG_INFINITY;
    for (_, vals) in &cfg.series[..2] {
        for &v in &vals[..n] {
            vmax = vmax.max(v);
        }
    }
    let vmax = vmax.max(1e-9);

    let ox = wf * 0.86;
    let oy = hf * 0.5;
    let r_base = hf * 0.035;
    let ring_gap = (hf * 0.40 - r_base) / n as f64;
    let start_angle = std::f64::consts::PI;
    let max_sweep = 128.0_f64.to_radians();
    let stroke_w = (ring_gap * 0.68).min(14.0);

    let mut buf = Vec::<u8>::with_capacity(n * 2 * 200 + 8192);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 24);

    let up_color = if cfg.palette.is_empty() { cfg.color_low } else { palette_color(cfg.palette, 0) };
    let down_color = if cfg.palette.len() > 1 { palette_color(cfg.palette, 1) } else { cfg.color_high };

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, ox - (r_base + n as f64 * ring_gap));
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, oy);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, ox - r_base);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, oy);
    push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1.4\"/>");

    for i in 0..n {
        let r = r_base + i as f64 * ring_gap + ring_gap / 2.0;
        let vu = cfg.series[0].1[i];
        let vd = cfg.series[1].1[i];
        let au = start_angle - (vu / vmax) * max_sweep;
        let ad = start_angle + (vd / vmax) * max_sweep;
        let mut lu = cfg.labels[i].clone();
        lu.push_str(" \u{2014} ");
        lu.push_str(&cfg.series[0].0);
        wave_arc(&mut buf, ox, oy, r, start_angle, au, stroke_w, up_color, i as i32, vu, &lu);
        let mut ld = cfg.labels[i].clone();
        ld.push_str(" \u{2014} ");
        ld.push_str(&cfg.series[1].0);
        wave_arc(&mut buf, ox, oy, r, start_angle, ad, stroke_w, down_color, (n + i) as i32, vd, &ld);

        let tx = ox - r;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, oy + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#334155\">");
        escape_xml(&mut buf, &cfg.labels[i]);
        push_b(&mut buf, b"</text>");
    }

    let leg_x = wf * 0.06;
    let leg_y = hf * 0.10;
    push_b(&mut buf, b"<rect x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y - 9.0);
    push_b(&mut buf, b"\" width=\"11\" height=\"11\" rx=\"2\" fill=\"#");
    buf.extend_from_slice(&hex6(up_color));
    push_b(&mut buf, b"\"/><text x=\"");
    push_f2(&mut buf, leg_x + 16.0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" fill=\"#334155\">");
    escape_xml(&mut buf, &cfg.series[0].0);
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<rect x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + 13.0);
    push_b(&mut buf, b"\" width=\"11\" height=\"11\" rx=\"2\" fill=\"#");
    buf.extend_from_slice(&hex6(down_color));
    push_b(&mut buf, b"\"/><text x=\"");
    push_f2(&mut buf, leg_x + 16.0);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y + 22.0);
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" fill=\"#334155\">");
    escape_xml(&mut buf, &cfg.series[1].0);
    push_b(&mut buf, b"</text>");

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

    fn cfg<'a>(labels: &'a [String], series: &'a [(String, Vec<f64>)]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            series,
            width: 820,
            height: 820,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        let labels: Vec<String> = (0..n).map(|i| format!("{}", 13 + i * 5)).collect();
        let up: Vec<f64> = (0..n).map(|i| 1.0 + (i as f64 * 0.5).sin().abs() * 12.0).collect();
        let down: Vec<f64> = (0..n).map(|i| 1.5 + (i as f64 * 0.4).cos().abs() * 12.0).collect();
        (labels, vec![("Femmes".to_string(), up), ("Hommes".to_string(), down)])
    }

    #[test]
    fn renders_an_up_wave_and_a_down_wave_per_category() {
        let (labels, series) = synth(10);
        let html = render(&cfg(&labels, &series));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 20);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn names_both_series_in_the_legend() {
        let (labels, series) = synth(10);
        let html = render(&cfg(&labels, &series));
        assert!(html.contains(">Femmes<"));
        assert!(html.contains(">Hommes<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series)).is_empty());
    }

    #[test]
    fn a_single_series_returns_empty_string() {
        let (labels, series) = synth(5);
        assert!(render(&cfg(&labels, &series[..1])).is_empty());
    }

    #[test]
    fn perf_rendering_many_age_rings_stays_fast() {
        let (labels, series) = synth(200);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &series));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
