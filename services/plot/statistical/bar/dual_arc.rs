use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, lerp_color, oblique_quad, palette_color, push_b, push_f2, push_i, push_quad_path, push_wedge_path, svg_open_rescalable, svg_title, truncate,
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

#[allow(clippy::too_many_arguments)]
fn wedge_bar(buf: &mut Vec<u8>, cx: f64, cy: f64, a0: f64, a1: f64, r0: f64, r1: f64, color: u32, opacity: f64, data_idx: i32, value: f64, label: &str) {
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
    push_b(buf, b"\" fill-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\" style=\"mix-blend-mode:multiply\"/>");
}

#[allow(clippy::too_many_arguments)]
fn oblique_bar(buf: &mut Vec<u8>, base: (f64, f64), dir: (f64, f64), length: f64, width: f64, color: u32, opacity: f64, data_idx: i32, value: f64, label: &str) {
    let quad = oblique_quad(base, dir, length, width);
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" d=\"");
    push_quad_path(buf, quad);
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" fill-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\" style=\"mix-blend-mode:multiply\"/>");
}

fn one_series<'a>(name_buf: &'a mut Vec<(String, Vec<f64>)>, values: &[f64]) -> &'a [(String, Vec<f64>)] {
    name_buf.push((String::new(), values.to_vec()));
    name_buf
}

const EFFIC_LEVELS: [&str; 4] = ["Pas du tout efficace", "Plutot pas efficace", "Plutot efficace", "Tres efficace"];

#[crate::chart_demo(
    "labels=[\"Participation a des salons\",\"Visite de salons\",\"Prospection telephonique\",\"Voyages et conventions d'affaires\",\"Site web\",\"Mailing\",\"Emailing\",\"Reponses aux appels d'offres publics\",\"Recommandations des clients\",\"Reseaux de professionnels\",\"Club d'entreprises\",\"Action des CCI\",\"Action des organismes de promotion du design\"], series_names=[\"Designer independant\",\"Agence de design\"], series=[[7.3,4.3,3.2,6.7,11.7,2.9,9.3,4.2,36.6,33.3,18.6,20.3,15.8],[4.7,3.9,2.5,2.9,8.7,3.8,6.7,4.4,20.2,18.2,11.4,10.9,9.6]], series2=[[2.6,2.4,1.8,2.7,3.3,1.5,1.9,2.5,3.8,3.6,2.9,3.4,3.5],[2.3,2.1,1.6,2.5,3.6,1.4,1.7,2.8,3.7,3.5,2.6,3.2,3.3]], palette=[3039066,6271907], color_low=1976888, color_high=14832700, base_variant=\"multicategory\", variant=\"dual_arc\", width=760, height=900"
)]

pub fn render(cfg: &BarConfig) -> String {
    let mut buf1 = Vec::new();
    let mut buf2 = Vec::new();
    let use_basic = cfg.base_variant.eq_ignore_ascii_case("basic");
    let series1: &[(String, Vec<f64>)] = if use_basic { one_series(&mut buf1, cfg.values) } else { cfg.series };
    let series2: &[(String, Vec<f64>)] = if use_basic { one_series(&mut buf2, cfg.values2) } else { cfg.series2 };

    let n_series = series1.len().min(series2.len());
    let n = cfg
        .labels
        .len()
        .min(series1.iter().map(|(_, v)| v.len()).min().unwrap_or(0))
        .min(series2.iter().map(|(_, v)| v.len()).min().unwrap_or(0));
    if n == 0 || n_series == 0 {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let wf = w as f64;
    let hf = h as f64;

    let per_rev = ((n as f64 / 3.2).ceil() as usize).clamp(12, 48);
    let angle_step = TAU / per_rev as f64;
    let half_step = angle_step / 2.0;

    let cx1 = wf * 0.44;
    let cy1 = hf * 0.735;
    let r_max1 = hf * 0.255;
    let r_hub1 = r_max1 * 0.07;
    let bar_max1 = (r_max1 - r_hub1) * 0.26;
    let spiral_span1 = r_max1 - r_hub1 - bar_max1;
    let growth1 = spiral_span1 / n as f64;
    let r_base1 = |i: usize| -> f64 { r_hub1 + growth1 * i as f64 };
    let theta1 = |i: usize| -> f64 { -FRAC_PI_2 + angle_step * i as f64 };

    let mut vmax1 = f64::NEG_INFINITY;
    let mut vmin1 = f64::INFINITY;
    for (_, vals) in series1 {
        for &v in &vals[..n] {
            vmax1 = vmax1.max(v);
            vmin1 = vmin1.min(v);
        }
    }
    let vr1 = (vmax1 - vmin1).max(1e-9);
    let mid1 = mix2(cfg.color_low, cfg.color_high);

    let spine_angle: f64 = -70.0_f64.to_radians();
    let spine_dir = (spine_angle.cos(), spine_angle.sin());
    let row_gap = 26.0;
    let spine_p0 = (wf * 0.52, hf * 0.475);
    let spine_pt = |i: usize| -> (f64, f64) { (spine_p0.0 + i as f64 * row_gap * spine_dir.0, spine_p0.1 + i as f64 * row_gap * spine_dir.1) };

    let bar_dir_angle: f64 = -84.0_f64.to_radians();
    let bar_dir = (bar_dir_angle.cos(), bar_dir_angle.sin());
    let bar_max_len = hf * 0.135;
    let bar_w = 8.0;

    let mut vmax2 = f64::NEG_INFINITY;
    let mut vmin2 = f64::INFINITY;
    for (_, vals) in series2 {
        for &v in &vals[..n] {
            vmax2 = vmax2.max(v);
            vmin2 = vmin2.min(v);
        }
    }
    let vr2 = (vmax2 - vmin2).max(1e-9);

    let mut buf = Vec::<u8>::with_capacity(n * n_series * 260 + 12_000);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 22);

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"0.6\">");
    let n_spokes = (per_rev / 2).clamp(6, 16);
    for k in 0..n_spokes {
        let a = -FRAC_PI_2 + TAU * k as f64 / n_spokes as f64;
        let x0 = cx1 + r_hub1 * a.cos();
        let y0 = cy1 + r_hub1 * a.sin();
        let x1 = cx1 + r_max1 * a.cos();
        let y1 = cy1 + r_max1 * a.sin();
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
        let a = theta1(i);
        let slot_start = a - half_step;
        let rb = r_base1(i);
        for j in 0..n_series {
            let value = series1[j].1[i];
            let t = ((value - vmin1) / vr1).clamp(0.0, 1.0);
            let bar_len = 2.0 + t * (bar_max1 - 2.0);
            let re = rb + bar_len;
            let color = if use_basic { lerp_color(t, cfg.color_low, mid1, cfg.color_high) } else { palette_color(cfg.palette, j) };
            let mut lbl = cfg.labels[i].clone();
            if !series1[j].0.is_empty() {
                lbl.push_str(" \u{2014} ");
                lbl.push_str(&series1[j].0);
            }
            wedge_bar(&mut buf, cx1, cy1, slot_start, slot_start + angle_step, rb, re, color, 0.78, (i * n_series + j) as i32, value, &lbl);
        }
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx1);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy1);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, r_hub1);
    push_b(&mut buf, b"\" fill=\"#f1f5f9\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    push_b(&mut buf, b"<g style=\"isolation:isolate\">");
    for i in 0..n {
        let base = spine_pt(i);
        for j in 0..n_series {
            let value = series2[j].1[i];
            let t = ((value - vmin2) / vr2).clamp(0.0, 1.0);
            let len = 4.0 + t * (bar_max_len - 4.0);
            let color = if use_basic { lerp_color(t, cfg.color_low, mid1, cfg.color_high) } else if j == 0 { cfg.color_low } else if j == 1 { cfg.color_high } else { palette_color(cfg.palette, j) };
            let mut lbl = cfg.labels[i].clone();
            if !series2[j].0.is_empty() {
                lbl.push_str(" \u{2014} ");
                lbl.push_str(&series2[j].0);
            }
            oblique_bar(&mut buf, base, bar_dir, len, bar_w, color, 0.82, (n * n_series + i * n_series + j) as i32, value, &lbl);
        }
    }
    push_b(&mut buf, b"</g>");

    if !use_basic {
        let ref_x = 18.0;
        let ref_y0 = hf - 95.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, ref_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ref_y0 - 14.0);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#475569\">ECHELLE EFFICACITE</text>");
        for (li, &name) in EFFIC_LEVELS.iter().enumerate() {
            let t = li as f64 / 3.0;
            let bl = 10.0 + t * 60.0;
            let ry = ref_y0 + li as f64 * 16.0;
            push_b(&mut buf, b"<line x1=\"");
            push_f2(&mut buf, ref_x);
            push_b(&mut buf, b"\" y1=\"");
            push_f2(&mut buf, ry);
            push_b(&mut buf, b"\" x2=\"");
            push_f2(&mut buf, ref_x + bl);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, ry);
            push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"4\"/>");
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, ref_x + 68.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ry + 3.0);
            push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"7.5\" fill=\"#64748b\">");
            escape_xml(&mut buf, name);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, spine_pt(0).0);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, spine_pt(0).1);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, spine_pt(n - 1).0);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, spine_pt(n - 1).1);
    push_b(&mut buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    for i in 0..n {
        let p = spine_pt(i);
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, p.0 - 4.0);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, p.1);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, p.0 + 4.0);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, p.1);
        push_b(&mut buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, p.0 + 9.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, p.1 + 3.2);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" fill=\"#334155\">");
        escape_xml(&mut buf, truncate(&cfg.labels[i], 42));
        push_b(&mut buf, b"</text>");
    }

    let leg_x = 18.0;
    let mut leg_y = 46.0;
    let leg_row = 14.0;
    for (j, (name, _)) in series1.iter().enumerate() {
        let nm = if name.is_empty() { "Serie" } else { name.as_str() };
        let color = if use_basic { cfg.color_low } else { palette_color(cfg.palette, j) };
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 8.0);
        push_b(&mut buf, b"\" width=\"10\" height=\"10\" rx=\"2\" fill=\"#");
        buf.extend_from_slice(&hex6(color));
        push_b(&mut buf, b"\"/><text x=\"");
        push_f2(&mut buf, leg_x + 14.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" fill=\"#475569\">");
        escape_xml(&mut buf, nm);
        push_b(&mut buf, b"</text>");
        leg_y += leg_row;
    }
    for (j, (name, _)) in series2.iter().enumerate() {
        let nm = if name.is_empty() { "Serie" } else { name.as_str() };
        let color = if use_basic { cfg.color_high } else if j == 0 { cfg.color_low } else if j == 1 { cfg.color_high } else { palette_color(cfg.palette, j) };
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 8.0);
        push_b(&mut buf, b"\" width=\"10\" height=\"10\" rx=\"2\" fill=\"#");
        buf.extend_from_slice(&hex6(color));
        push_b(&mut buf, b"\"/><text x=\"");
        push_f2(&mut buf, leg_x + 14.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" fill=\"#475569\">");
        escape_xml(&mut buf, nm);
        push_b(&mut buf, b"</text>");
        leg_y += leg_row;
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

    fn cfg<'a>(labels: &'a [String], series: &'a [(String, Vec<f64>)], series2: &'a [(String, Vec<f64>)]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            series,
            series2,
            width: 760,
            height: 900,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize, n_series: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>, Vec<(String, Vec<f64>)>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let series: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 5.0 + ((i + s) as f64 * 0.6).sin().abs() * 20.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        let series2: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 1.0 + ((i + s) as f64 * 0.5).sin().abs() * 3.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        (labels, series, series2)
    }

    #[test]
    fn renders_a_spiral_wedge_and_an_oblique_bar_per_item_per_series() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 13 * 2 * 2);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn draws_the_four_named_efficacy_reference_levels_in_series_mode() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        assert!(html.contains("Pas du tout efficace"));
        assert!(html.contains("Tres efficace"));
    }

    #[test]
    fn every_real_category_name_is_readable_directly_on_the_spine() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        for l in &labels {
            assert!(html.contains(&format!(">{l}<")));
        }
        assert!(!html.contains("CATEGORIES, PAR NUMERO"));
    }

    #[test]
    fn basic_variant_uses_values_and_a_single_gradient_series() {
        let labels: Vec<String> = (0..8).map(|i| format!("Item {i}")).collect();
        let values: Vec<f64> = (0..8).map(|i| 4.0 + i as f64 * 2.0).collect();
        let values2: Vec<f64> = (0..8).map(|i| 1.0 + i as f64 * 0.3).collect();
        let html = render(&BarConfig {
            title: "Test",
            labels: &labels,
            values: &values,
            values2: &values2,
            base_variant: "basic",
            width: 760,
            height: 900,
            ..BarConfig::default()
        });
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 8 * 2);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        let series2: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series, &series2)).is_empty());
    }

    #[test]
    fn missing_series2_returns_empty_string() {
        let (labels, series, _series2) = synth(13, 2);
        let empty: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series, &empty)).is_empty());
    }

    #[test]
    fn perf_rendering_many_categories_stays_fast() {
        let (labels, series, series2) = synth(300, 3);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &series, &series2));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
