use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, lerp_color, palette_color, push_b, push_f2, push_i, push_wedge_path, svg_open_rescalable, svg_title, truncate,
};
use std::f64::consts::{FRAC_PI_2, TAU};

fn mix2(a: u32, b: u32) -> u32 {
    let ar = (a >> 16) & 0xFF;
    let ag = (a >> 8) & 0xFF;
    let ab = a & 0xFF;
    let br = (b >> 16) & 0xFF;
    let bg = (b >> 8) & 0xFF;
    let bb = b & 0xFF;
    (((ar + br) / 2) << 16) | (((ag + bg) / 2) << 8) | ((ab + bb) / 2)
}

fn one_series<'a>(name_buf: &'a mut Vec<(String, Vec<f64>)>, values: &[f64]) -> &'a [(String, Vec<f64>)] {
    name_buf.push((String::new(), values.to_vec()));
    name_buf
}

#[allow(clippy::too_many_arguments)]
fn wedge_bar(buf: &mut Vec<u8>, cx: f64, cy: f64, a0: f64, a1: f64, r0: f64, r1: f64, color: u32, data_idx: i32, value: f64, label: &str) {
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" d=\"");
    push_wedge_path(buf, cx, cy, a0, a1, r0, r1);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" fill-opacity=\"0.8\" style=\"mix-blend-mode:multiply\"/>");
}

#[crate::chart_demo(
    "labels=[\"Participation a des salons\",\"Visite de salons\",\"Prospection telephonique\",\"Voyages et conventions d'affaires\",\"Site web\",\"Mailing\",\"Emailing\",\"Reponses aux appels d'offres publics\",\"Recommandations des clients\",\"Reseaux de professionnels\",\"Club d'entreprises\",\"Action des CCI\",\"Action des organismes de promotion du design\"], series_names=[\"Designer independant\",\"Agence de design\"], series=[[7.3,4.3,3.2,6.7,11.7,2.9,9.3,4.2,36.6,33.3,18.6,20.3,15.8],[4.7,3.9,2.5,2.9,8.7,3.8,6.7,4.4,20.2,18.2,11.4,10.9,9.6]], palette=[3039066,6271907], base_variant=\"multicategory\", variant=\"dual_arc\", width=860, height=800"
)]

pub fn render(cfg: &BarConfig) -> String {
    let mut sbuf = Vec::new();
    let use_basic = cfg.base_variant.eq_ignore_ascii_case("basic");
    let series: &[(String, Vec<f64>)] = if use_basic { one_series(&mut sbuf, cfg.values) } else { cfg.series };

    let n_series = series.len();
    let n = cfg.labels.len().min(series.iter().map(|(_, v)| v.len()).min().unwrap_or(0));
    if n == 0 || n_series == 0 {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let wf = w as f64;
    let hf = h as f64;

    let mut vmax = f64::NEG_INFINITY;
    let mut vmin = f64::INFINITY;
    for (_, vals) in series {
        for &v in &vals[..n] {
            vmax = vmax.max(v);
            vmin = vmin.min(v);
        }
    }
    let vr = (vmax - vmin).max(1e-9);
    let mid = mix2(cfg.color_low, cfg.color_high);

    let pad_x = 48.0;
    let usable_w = wf - 2.0 * pad_x;
    let cat_w = usable_w / n as f64;
    let x_of = |i: usize| -> f64 { pad_x + cat_w * (i as f64 + 0.5) };

    let bars_top = 46.0;
    let bars_base = hf * 0.30;
    let bar_max_h = (bars_base - bars_top).max(10.0);

    let mut buf = Vec::<u8>::with_capacity(n * n_series * 240 + 12_000);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 22);

    push_b(&mut buf, b"<g style=\"isolation:isolate\">");
    for i in 0..n {
        let cxi = x_of(i);
        let group_w = cat_w * 0.7;
        let bw = group_w / n_series as f64;
        for (j, (_, vals)) in series.iter().enumerate() {
            let v = vals[i];
            let t = ((v - vmin) / vr).clamp(0.0, 1.0);
            let bh = 2.0 + t * (bar_max_h - 2.0);
            let bx = cxi - group_w / 2.0 + j as f64 * bw;
            let color = if use_basic { lerp_color(t, cfg.color_low, mid, cfg.color_high) } else { palette_color(cfg.palette, j) };
            let hx = hex6(color);
            push_b(&mut buf, b"<rect data-idx=\"");
            push_i(&mut buf, (i * n_series + j) as i32);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"\" data-lbl=\"");
            escape_xml(&mut buf, &cfg.labels[i]);
            push_b(&mut buf, b"\" x=\"");
            push_f2(&mut buf, bx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, bars_base - bh);
            push_b(&mut buf, b"\" width=\"");
            push_f2(&mut buf, (bw - 1.2).max(1.0));
            push_b(&mut buf, b"\" height=\"");
            push_f2(&mut buf, bh);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" rx=\"1.2\"/>");
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, pad_x);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, bars_base);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, wf - pad_x);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, bars_base);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    let label_angle = 78.0;
    for i in 0..n {
        let cxi = x_of(i);
        let ly = bars_base + 8.0;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, cxi);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, bars_base);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, cxi);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cxi);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 3.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" fill=\"#334155\" transform=\"rotate(");
        push_f2(&mut buf, label_angle);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, cxi);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ly + 3.0);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(&cfg.labels[i], 48));
        push_b(&mut buf, b"</text>");
    }

    let per_rev = ((n as f64 / 3.2).ceil() as usize).clamp(12, 48);
    let angle_step = TAU / per_rev as f64;
    let half_step = angle_step / 2.0;
    let cx2 = wf / 2.0;
    let cy2 = hf * 0.62;
    let r_max2 = hf * 0.24;
    let r_hub2 = r_max2 * 0.09;
    let bar_max2 = (r_max2 - r_hub2) * 0.30;
    let spiral_span = r_max2 - r_hub2 - bar_max2;
    let growth = spiral_span / n as f64;
    let r_base2 = |i: usize| -> f64 { r_hub2 + growth * i as f64 };
    let theta2 = |i: usize| -> f64 { -FRAC_PI_2 + angle_step * i as f64 };

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"0.6\">");
    let n_spokes = (per_rev / 2).clamp(6, 16);
    for k in 0..n_spokes {
        let a = -FRAC_PI_2 + TAU * k as f64 / n_spokes as f64;
        let x0 = cx2 + r_hub2 * a.cos();
        let y0 = cy2 + r_hub2 * a.sin();
        let x1 = cx2 + r_max2 * a.cos();
        let y1 = cy2 + r_max2 * a.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g style=\"isolation:isolate\">");
    for i in 0..n {
        let a = theta2(i);
        let slot_start = a - half_step;
        let rb = r_base2(i);
        for (j, (_, vals)) in series.iter().enumerate() {
            let v = vals[i];
            let t = ((v - vmin) / vr).clamp(0.0, 1.0);
            let bar_len = 2.0 + t * (bar_max2 - 2.0);
            let re = rb + bar_len;
            let color = if use_basic { lerp_color(t, cfg.color_low, mid, cfg.color_high) } else { palette_color(cfg.palette, j) };
            wedge_bar(&mut buf, cx2, cy2, slot_start, slot_start + angle_step, rb, re, color, (n * n_series + i * n_series + j) as i32, v, &cfg.labels[i]);
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx2);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy2);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub2);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    if n_series > 1 || !series[0].0.is_empty() {
        let leg_x = wf - 168.0;
        let leg_y = 40.0;
        for (j, (name, _)) in series.iter().enumerate() {
            let sy = leg_y + j as f64 * 14.0;
            let color = if use_basic { cfg.color_low } else { palette_color(cfg.palette, j) };
            push_b(&mut buf, b"<rect x=\"");
            push_f2(&mut buf, leg_x);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, sy - 8.0);
            push_b(&mut buf, b"\" width=\"10\" height=\"10\" rx=\"2\" fill=\"#");
            buf.extend_from_slice(&hex6(color));
            push_b(&mut buf, b"\"/><text x=\"");
            push_f2(&mut buf, leg_x + 14.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, sy);
            push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" fill=\"#475569\">");
            escape_xml(&mut buf, name);
            push_b(&mut buf, b"</text>");
        }
    }

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
            width: 860,
            height: 800,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize, n_series: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let series: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 5.0 + ((i + s) as f64 * 0.6).sin().abs() * 20.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        (labels, series)
    }

    #[test]
    fn renders_the_same_data_as_a_straight_bar_and_a_spiral_wedge() {
        let (labels, series) = synth(13, 2);
        let html = render(&cfg(&labels, &series));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<rect data-idx=").count(), 13 * 2);
        assert_eq!(html.matches("<path data-idx=").count(), 13 * 2);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn every_real_category_name_is_readable_once_not_in_a_side_legend() {
        let (labels, series) = synth(13, 2);
        let html = render(&cfg(&labels, &series));
        for l in &labels {
            assert_eq!(html.matches(&format!(">{l}<")).count(), 1);
        }
        assert!(!html.contains("PAR NUMERO"));
    }

    #[test]
    fn basic_variant_uses_flat_values_as_a_single_gradient_series() {
        let labels: Vec<String> = (0..8).map(|i| format!("Item {i}")).collect();
        let values: Vec<f64> = (0..8).map(|i| 4.0 + i as f64 * 2.0).collect();
        let html = render(&BarConfig {
            title: "Test",
            labels: &labels,
            values: &values,
            base_variant: "basic",
            width: 860,
            height: 800,
            ..BarConfig::default()
        });
        assert!(!html.is_empty());
        assert_eq!(html.matches("<rect data-idx=").count(), 8);
        assert_eq!(html.matches("<path data-idx=").count(), 8);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series)).is_empty());
    }

    #[test]
    fn perf_rendering_many_categories_stays_fast() {
        let (labels, series) = synth(400, 3);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &series));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
