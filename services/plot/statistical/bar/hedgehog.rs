use super::config::BarConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, push_b, push_f2, push_i, svg_open_rescalable, svg_title};

#[crate::chart_demo(
    "labels=[\"Chico to Seattle\",\"Fresno to Portland\",\"Reno to Denver\",\"Modesto to Austin\",\"Bakersfield to Sacramento\",\"Chico to Sacramento\",\"Stockton to Boise\",\"Fresno to Denver\",\"Reno to Portland\",\"Modesto to Sacramento\",\"Chico to Portland\",\"Bakersfield to Boise\",\"Stockton to Sacramento\",\"Fresno to Boise\",\"Reno to Sacramento\",\"Modesto to Boise\",\"Chico to Denver\",\"Bakersfield to Denver\",\"Stockton to Denver\",\"Fresno to Sacramento\",\"New York to San Juan\",\"New York to Memphis\",\"New York to Jackson\",\"New York to Shreveport\",\"New York to Mobile\",\"New York to Fresno\",\"New York to Bakersfield\",\"New York to Stockton\",\"New York to Modesto\",\"New York to Reno\",\"Los Angeles to Memphis\",\"Los Angeles to Jackson\",\"Los Angeles to Shreveport\",\"Los Angeles to Mobile\",\"Los Angeles to San Juan\",\"Seattle to Memphis\",\"Seattle to Jackson\",\"Seattle to Fresno\",\"Seattle to Bakersfield\",\"Portland to Memphis\",\"Portland to Jackson\",\"Portland to Fresno\",\"Denver to Memphis\",\"Denver to Jackson\",\"Denver to Fresno\",\"Sacramento to Memphis\",\"Sacramento to Jackson\",\"Boise to Memphis\",\"Boise to Jackson\",\"Austin to Memphis\",\"Austin to Jackson\"], values=[9.0,6.0,4.0,3.0,2.0,7.0,5.0,3.5,2.5,1.5,5.5,4.5,3.0,2.0,6.5,1.0,4.0,2.5,1.5,0.5,-42.0,-38.0,-36.0,-35.0,-33.0,-30.0,-28.0,-27.0,-26.0,-24.0,-31.0,-29.0,-27.0,-25.0,-23.0,-22.0,-20.0,-19.0,-18.0,-21.0,-19.0,-17.0,-16.0,-15.0,-14.0,-13.0,-12.0,-11.0,-10.0,-9.0,-8.0], variant=\"hedgehog\", width=700, height=380"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for &v in &cfg.values[..n] {
        vmin = vmin.min(v);
        vmax = vmax.max(v);
    }
    vmin = vmin.min(0.0);
    vmax = vmax.max(0.0);
    let vr = (vmax - vmin).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let pad_l = 96.0;
    let pad_r = 210.0;
    let pad_t = 40.0;
    let pad_b = 34.0;
    let plot_w = (w as f64 - pad_l - pad_r).max(40.0);
    let plot_h = (h as f64 - pad_t - pad_b).max(40.0);
    let origin_x = pad_l;
    let hist_x = pad_l + plot_w;

    let y_of = |v: f64| -> f64 { pad_t + plot_h * (1.0 - (v - vmin) / vr) };
    let origin_y = y_of(0.0);

    let color_low = if cfg.color_low == 0 { 0x636EFA } else { cfg.color_low };
    let color_high = if cfg.color_high == 0 { 0xF43F5E } else { cfg.color_high };
    let low_hx = hex6(color_low);
    let high_hx = hex6(color_high);

    let mut buf = Vec::<u8>::with_capacity(n * 180 + 8192);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 18);

    push_b(&mut buf, b"<g fill=\"none\" stroke-linecap=\"round\">");
    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    let mut max_idx = 0usize;
    let mut min_idx = 0usize;
    for i in 0..n {
        let v = cfg.values[i];
        if v > cfg.values[max_idx] {
            max_idx = i;
        }
        if v < cfg.values[min_idx] {
            min_idx = i;
        }
        let ey = y_of(v);
        let ex = hist_x - 6.0;
        let hx = if v >= 0.0 { &high_hx } else { &low_hx };
        let c1x = origin_x + plot_w * 0.42;
        let c2x = origin_x + plot_w * 0.78;

        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(hx);
        push_b(&mut buf, b"\" stroke-width=\"1.1\" stroke-opacity=\"0.10\" d=\"M");
        push_f2(&mut buf, origin_x);
        push_b(&mut buf, b",");
        push_f2(&mut buf, origin_y);
        push_b(&mut buf, b" C");
        push_f2(&mut buf, c1x);
        push_b(&mut buf, b",");
        push_f2(&mut buf, origin_y);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, c2x);
        push_b(&mut buf, b",");
        push_f2(&mut buf, ey);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ex);
        push_b(&mut buf, b",");
        push_f2(&mut buf, ey);
        push_b(&mut buf, b"\"/>");

        slots.push(HoverSlot::new(cfg.labels[i].clone()).kv("Delta", format!("{:.1}", v)));
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, origin_x);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, origin_y);
    push_b(&mut buf, b"\" r=\"3.5\" fill=\"#0f172a\"/>");
    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, origin_x);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, origin_y);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, origin_x);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, origin_y + 30.0);
    push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\" stroke-dasharray=\"1,2\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, origin_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, origin_y + 44.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" fill=\"#64748b\">origin</text>");

    let n_bins = 44usize;
    let bin_h = plot_h / n_bins as f64;
    let mut counts = vec![0usize; n_bins];
    for &v in &cfg.values[..n] {
        let t = ((v - vmin) / vr).clamp(0.0, 0.999999);
        let bin = ((1.0 - t) * n_bins as f64).floor().min(n_bins as f64 - 1.0) as usize;
        counts[bin] += 1;
    }
    let max_count = counts.iter().copied().max().unwrap_or(1).max(1) as f64;
    let hist_max_w = 88.0;
    let bin_center = |bin: usize| -> f64 { vmax - (bin as f64 + 0.5) / n_bins as f64 * vr };

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, hist_x);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, hist_x);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    let write_side = |buf: &mut Vec<u8>, want_high: bool, hx: &[u8; 6]| {
        let mut pts: Vec<(f64, f64)> = Vec::new();
        for bin in 0..n_bins {
            let bc = bin_center(bin);
            if (bc >= 0.0) != want_high {
                continue;
            }
            let y = pad_t + (bin as f64 + 0.5) * bin_h;
            let width = counts[bin] as f64 / max_count * hist_max_w;
            pts.push((hist_x + width, y));
        }
        if pts.is_empty() {
            return;
        }
        push_b(buf, b"<path fill=\"#");
        buf.extend_from_slice(hx);
        push_b(buf, b"\" fill-opacity=\"0.85\" d=\"M");
        push_f2(buf, hist_x);
        push_b(buf, b",");
        push_f2(buf, pts[0].1);
        for &(x, y) in &pts {
            push_b(buf, b" L");
            push_f2(buf, x);
            push_b(buf, b",");
            push_f2(buf, y);
        }
        push_b(buf, b" L");
        push_f2(buf, hist_x);
        push_b(buf, b",");
        push_f2(buf, pts[pts.len() - 1].1);
        push_b(buf, b" Z\"/>");
    };
    write_side(&mut buf, true, &high_hx);
    write_side(&mut buf, false, &low_hx);

    push_b(&mut buf, b"<g font-family=\"-apple-system,Arial,sans-serif\" fill=\"#64748b\" font-size=\"9\">");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, hist_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, pad_t - 12.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\">higher</text>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, hist_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, origin_y - 6.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\">same</text>");
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, hist_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, pad_t + plot_h + 16.0);
    push_b(&mut buf, b"\" text-anchor=\"middle\">lower</text>");
    push_b(&mut buf, b"</g>");

    let n_pos = cfg.values[..n].iter().filter(|&&v| v >= 0.0).count();
    let pct_pos = (n_pos as f64 / n as f64 * 100.0).round();
    let pct_neg = 100.0 - pct_pos;
    let text_x = hist_x + hist_max_w + 14.0;

    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, text_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, y_of(vmax * 0.5).max(pad_t + 24.0));
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"18\" font-weight=\"800\" fill=\"#");
    buf.extend_from_slice(&high_hx);
    push_b(&mut buf, b"\">");
    let s = format!("{:.0}%", pct_pos);
    buf.extend_from_slice(s.as_bytes());
    push_b(&mut buf, b"</text>");

    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, text_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, y_of(vmin * 0.5).min(pad_t + plot_h - 24.0));
    push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"18\" font-weight=\"800\" fill=\"#");
    buf.extend_from_slice(&low_hx);
    push_b(&mut buf, b"\">");
    let s = format!("{:.0}%", pct_neg);
    buf.extend_from_slice(s.as_bytes());
    push_b(&mut buf, b"</text>");

    for &idx in &[max_idx, min_idx] {
        let ey = y_of(cfg.values[idx]);
        let hx = if cfg.values[idx] >= 0.0 { &high_hx } else { &low_hx };
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, hist_x - 6.0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, ey);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, text_x);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ey);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(hx);
        push_b(&mut buf, b"\" stroke-width=\"1\" stroke-dasharray=\"1,2\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, text_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ey - 4.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" fill=\"#");
        buf.extend_from_slice(hx);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.labels[idx]);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], values: &'a [f64]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            values,
            width: 700,
            height: 400,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Flow {i}")).collect();
        let values: Vec<f64> = (0..n).map(|i| ((i as f64 * 3.7).sin() * 40.0) - 10.0).collect();
        (labels, values)
    }

    #[test]
    fn renders_one_curve_per_flow_from_a_shared_origin() {
        let (labels, values) = synth(60);
        let html = render(&cfg(&labels, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 60);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn positive_and_negative_flows_split_into_the_two_histogram_halves() {
        let labels = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let values = vec![10.0, -5.0, -8.0];
        let html = render(&cfg(&labels, &values));
        assert_eq!(html.matches("<path fill=\"#").count(), 2);
    }

    #[test]
    fn the_percentage_split_reflects_the_share_above_and_below_zero() {
        let labels: Vec<String> = (0..10).map(|i| format!("F{i}")).collect();
        let values: Vec<f64> = vec![5.0, 5.0, 5.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
        let html = render(&cfg(&labels, &values));
        assert!(html.contains(">30%<"));
        assert!(html.contains(">70%<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        assert!(render(&cfg(&labels, &values)).is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_flow_fan_stays_fast() {
        let (labels, values) = synth(2000);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
