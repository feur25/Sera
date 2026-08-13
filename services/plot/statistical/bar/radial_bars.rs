use super::config::BarConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, lerp_color, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title, truncate,
};
use std::f64::consts::PI;

#[allow(clippy::too_many_arguments)]
fn ray_bar(buf: &mut Vec<u8>, cx: f64, cy: f64, a: f64, r0: f64, r1: f64, half_w: f64, color: u32, data_idx: i32, value: f64, label: &str) {
    let ca = a.cos();
    let sa = a.sin();
    let px = -sa * half_w;
    let py = ca * half_w;
    let x0 = cx + r0 * ca;
    let y0 = cy + r0 * sa;
    let x1 = cx + r1 * ca;
    let y1 = cy + r1 * sa;
    push_b(buf, b"<path data-idx=\"");
    push_i(buf, data_idx);
    push_b(buf, b"\" data-v=\"");
    push_f2(buf, value);
    push_b(buf, b"\" data-lbl=\"");
    escape_xml(buf, label);
    push_b(buf, b"\" d=\"M");
    push_f2(buf, x0 - px);
    push_b(buf, b",");
    push_f2(buf, y0 - py);
    push_b(buf, b" L");
    push_f2(buf, x1 - px);
    push_b(buf, b",");
    push_f2(buf, y1 - py);
    push_b(buf, b" L");
    push_f2(buf, x1 + px);
    push_b(buf, b",");
    push_f2(buf, y1 + py);
    push_b(buf, b" L");
    push_f2(buf, x0 + px);
    push_b(buf, b",");
    push_f2(buf, y0 + py);
    push_b(buf, b" Z\" fill=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" rx=\"1\"/>");
}

fn arc_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64) {
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    let large = if (a1 - a0).abs() > PI { 1 } else { 0 };
    push_b(buf, b"<path fill=\"none\" d=\"M");
    push_f2(buf, x0);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_b(buf, b" A");
    push_f2(buf, r);
    push_b(buf, b",");
    push_f2(buf, r);
    push_b(buf, b" 0 ");
    buf.push(large + b'0');
    push_b(buf, b",1 ");
    push_f2(buf, x1);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"\"");
}

#[crate::chart_demo(
    "labels=[\"Apex Capital\",\"Sterling Partners\",\"Meridian Group\",\"Blackrock Ventures\",\"Pacific Bridge\",\"Northgate Capital\",\"Horizon Equity\",\"Beacon Partners\",\"Charter Group\",\"Thames Capital\",\"Kensington Partners\",\"Regent Equity\",\"Lakeside Ventures\",\"Prairie Capital\",\"Redwood Partners\",\"Summit Ventures\",\"Capitol Equity\",\"Federal Partners\"], values=[92.4,78.1,61.5,45.2,58.9,41.3,33.7,52.6,29.4,66.8,48.0,31.2,37.9,24.6,44.1,27.8,35.5,22.3], super_categories=[\"New York\",\"New York\",\"New York\",\"New York\",\"San Francisco\",\"San Francisco\",\"San Francisco\",\"Boston\",\"Boston\",\"London\",\"London\",\"London\",\"Chicago\",\"Chicago\",\"Menlo Park\",\"Menlo Park\",\"Washington DC\",\"Washington DC\"], color_low=2262390, color_high=1364199, variant=\"radial_bars\", width=900, height=780"
)]

pub fn render(cfg: &BarConfig) -> String {
    let n = cfg.labels.len().min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let w = cfg.width;
    let h = cfg.height;
    let wf = w as f64;
    let hf = h as f64;

    let mut vmax = f64::NEG_INFINITY;
    let mut vmin = f64::INFINITY;
    for &v in &cfg.values[..n] {
        vmax = vmax.max(v);
        vmin = vmin.min(v);
    }
    let vr = (vmax - vmin).max(1e-9);

    let cx = wf / 2.0;
    let cy = hf * 0.86;
    let start = -170.0_f64.to_radians();
    let end = -10.0_f64.to_radians();
    let sweep = end - start;
    let angle = |i: usize| -> f64 { start + sweep * i as f64 / (n - 1).max(1) as f64 };

    let r_axis = hf * 0.30;
    let bar_max = hf * 0.185;
    let half_w = (r_axis * (sweep.abs() / n as f64) * 0.34).min(9.0);

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 8192);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 24);

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"1\">");
    arc_path(&mut buf, cx, cy, r_axis, start, end);
    push_b(&mut buf, b"/></g>");

    if !cfg.super_categories.is_empty() {
        let mut start_i = 0usize;
        while start_i < n {
            let cur = cfg.super_categories.get(start_i).map(|s| s.as_str()).unwrap_or("");
            let mut end_i = start_i + 1;
            while end_i < n && cfg.super_categories.get(end_i).map(|s| s.as_str()).unwrap_or("") == cur {
                end_i += 1;
            }
            let a0 = angle(start_i) - (angle(1) - angle(0)) * 0.4;
            let a1 = angle(end_i - 1) + (angle(1) - angle(0)) * 0.4;
            let r_in = r_axis - 16.0;
            push_b(&mut buf, b"<g stroke=\"#94a3b8\" stroke-width=\"1\">");
            arc_path(&mut buf, cx, cy, r_in, a0, a1);
            push_b(&mut buf, b"/></g>");
            let am = (a0 + a1) / 2.0;
            let lx = cx + (r_in - 8.0) * am.cos();
            let ly = cy + (r_in - 8.0) * am.sin();
            let deg = am.to_degrees() + 90.0;
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\" transform=\"rotate(");
            push_f2(&mut buf, deg);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, ly);
            push_b(&mut buf, b")\">");
            escape_xml(&mut buf, cur);
            push_b(&mut buf, b"</text>");
            start_i = end_i;
        }
    }

    for i in 0..n {
        let a = angle(i);
        let v = cfg.values[i];
        let t = ((v - vmin) / vr).clamp(0.0, 1.0);
        let len = 8.0 + t * (bar_max - 8.0);
        let re = r_axis + len;
        let color = if cfg.palette.is_empty() { lerp_color(t, cfg.color_low, ((cfg.color_low + cfg.color_high) / 2).max(1), cfg.color_high) } else { palette_color(cfg.palette, i) };
        ray_bar(&mut buf, cx, cy, a, r_axis, re, half_w, color, i as i32, v, &cfg.labels[i]);

        let lx = cx + (re + 6.0) * a.cos();
        let ly = cy + (re + 6.0) * a.sin();
        let deg = if a.cos() < 0.0 { a.to_degrees() + 180.0 } else { a.to_degrees() };
        let anchor: &[u8] = if a.cos() < 0.0 { b"end" } else { b"start" };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" font-weight=\"700\" fill=\"#1e293b\" transform=\"rotate(");
        push_f2(&mut buf, deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(&cfg.labels[i], 26));
        push_b(&mut buf, b" ");
        let s = format!("{:.1}", v);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</text>");
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

    fn cfg<'a>(labels: &'a [String], values: &'a [f64], super_categories: &'a [String]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            values,
            super_categories,
            width: 900,
            height: 780,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let values: Vec<f64> = (0..n).map(|i| 10.0 + (i as f64 * 0.6).sin().abs() * 80.0).collect();
        (labels, values)
    }

    #[test]
    fn renders_one_ray_per_category_with_a_tip_label() {
        let (labels, values) = synth(18);
        let empty: Vec<String> = vec![];
        let html = render(&cfg(&labels, &values, &empty));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 18);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn super_categories_get_their_own_inner_arc_and_label() {
        let (labels, values) = synth(6);
        let super_cats: Vec<String> = vec!["A".into(), "A".into(), "A".into(), "B".into(), "B".into(), "B".into()];
        let html = render(&cfg(&labels, &values, &super_cats));
        assert!(html.contains(">A<"));
        assert!(html.contains(">B<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        let empty: Vec<String> = vec![];
        assert!(render(&cfg(&labels, &values, &empty)).is_empty());
    }

    #[test]
    fn perf_rendering_many_rays_stays_fast() {
        let (labels, values) = synth(400);
        let empty: Vec<String> = vec![];
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &values, &empty));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
