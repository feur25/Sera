use super::config::BubbleConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{
    angle_at, colorscale_color, escape_xml, hash01, hex6, lerp_rgb, polar_point, push_b, push_f2, push_i,
};
use std::f64::consts::PI;

#[crate::chart_demo(
    "labels=[\"streaming\",\"csharp\",\"nuget\",\"benchmark\",\"rust\",\"crossfilter\",\"r\",\"indicators\",\"secure\",\"theme\",\"python\",\"doc\",\"studio\",\"svg\",\"pulse\",\"board\",\"video\",\"cache\",\"export\",\"pypi\",\"npm2\",\"ffi\",\"build\",\"arbitrage\",\"scala\",\"rse\",\"wasm\",\"npm\",\"stabilite\",\"webhook\",\"ci\",\"dframe\",\"wgpu\",\"ml\",\"perf\",\"burst\",\"gpu\",\"gif\",\"axis\",\"v1\",\"realtime\",\"dashboard\",\"clean2\",\"alerte\",\"java\",\"clean\",\"licence\",\"candlestick\",\"template\",\"powerbi\",\"firehose\",\"collab\",\"notion\",\"live\",\"render\",\"test\",\"media\",\"veille\",\"core\",\"api\",\"gantt\"], categories=[\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"avant\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"avant\",\"apres\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"avant\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\"], x_values=[0.222,0.678,0.625,0.142,0.549,0.869,0.724,0.904,0.817,0.54,0.404,0.446,0.684,0.155,0.234,0.297,0.735,0.25,0.748,0.31,0.889,0.415,0.377,0.669,0.179,0.806,0.282,0.381,0.276,0.383,0.145,0.762,0.423,0.603,0.195,0.651,0.327,0.645,0.365,0.283,0.225,0.649,0.909,0.946,0.732,0.325,0.683,0.308,0.614,0.2,0.308,0.779,0.164,0.327,0.529,0.058,0.182,0.898,0.071,0.095,0.313], sizes=[20.6,26.3,13.0,14.2,21.9,14.9,30.2,39.1,19.6,12.4,11.0,7.5,12.4,20.6,10.3,29.3,17.8,19.5,10.6,16.2,45.3,46.0,15.8,22.5,8.7,40.8,13.0,13.3,15.0,42.9,9.9,34.7,19.3,36.0,7.1,24.0,17.8,19.2,18.5,14.1,30.1,22.1,21.4,26.4,44.6,23.0,11.8,36.1,40.4,14.7,35.8,12.5,18.2,18.3,19.6,14.5,44.6,41.1,21.7,19.0,44.1], color_values=[0.98,0.87,0.68,0.29,0.35,0.58,0.57,0.62,0.94,0.12,0.17,0.19,0.71,0.24,0.9,0.72,0.91,0.22,0.61,0.25,0.83,0.54,0.15,0.79,0.96,0.79,0.35,0.18,0.38,0.6,0.11,0.9,0.54,0.95,0.33,0.55,0.89,0.59,0.18,0.28,0.62,0.73,0.84,0.79,0.51,0.93,0.99,0.97,0.81,0.68,0.57,0.62,0.72,0.99,0.1,0.12,0.62,0.71,0.39,0.31,0.94], colorscale=\"turbo\", variant=\"burst\""
)]

pub fn render(cfg: &BubbleConfig) -> String {
    let n = cfg.x_values.len().min(cfg.sizes.len()).min(cfg.categories.len());
    if n == 0 {
        return String::new();
    }

    let mut order: Vec<String> = Vec::new();
    for c in cfg.categories[..n].iter() {
        if !order.iter().any(|s| s == c) {
            order.push(c.clone());
        }
    }
    if order.len() < 2 {
        return String::new();
    }
    let cat_a = order[0].clone();
    let cat_b = order[1].clone();

    let w = cfg.width;
    let h = cfg.height;
    let side = w.min(h) as f64;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0 - side * 0.03;
    let inner_r = side * 0.025;
    let outer_r = side * 0.42;

    let mut min_a = f64::INFINITY;
    let mut max_a = f64::NEG_INFINITY;
    let mut min_b = f64::INFINITY;
    let mut max_b = f64::NEG_INFINITY;
    for i in 0..n {
        let v = cfg.x_values[i];
        if cfg.categories[i] == cat_a {
            min_a = min_a.min(v);
            max_a = max_a.max(v);
        } else {
            min_b = min_b.min(v);
            max_b = max_b.max(v);
        }
    }
    let range_a = (max_a - min_a).max(1e-9);
    let range_b = (max_b - min_b).max(1e-9);

    let has_color = cfg.color_values.len() >= n;
    let (cv_min, cv_max) = if has_color {
        let mut mn = f64::INFINITY;
        let mut mx = f64::NEG_INFINITY;
        for &v in &cfg.color_values[..n] {
            if v.is_finite() {
                mn = mn.min(v);
                mx = mx.max(v);
            }
        }
        (mn, mx)
    } else {
        (0.0, 1.0)
    };
    let cv_range = (cv_max - cv_min).max(1e-9);

    let mut s_min = f64::INFINITY;
    let mut s_max = f64::NEG_INFINITY;
    for &v in &cfg.sizes[..n] {
        let a = v.abs();
        s_min = s_min.min(a);
        s_max = s_max.max(a);
    }
    let s_range = (s_max - s_min).max(1e-9);

    let color_b = if cfg.color_hex == 0 { 0xFF7A45 } else { cfg.color_hex };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    let mut buf = Vec::<u8>::with_capacity(n * 220 + 8192);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#050507\"/>");

    push_b(&mut buf, b"<g stroke=\"#ffffff\" stroke-opacity=\"0.045\" stroke-width=\"1\">");
    let n_spokes = 72usize;
    for k in 0..n_spokes {
        let a = angle_at(k as f64, n_spokes as f64, 0.0);
        let (x1, y1) = polar_point(cx, cy, a, inner_r);
        let (x2, y2) = polar_point(cx, cy, a, outer_r * 1.08);
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x2);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y2);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<g fill=\"none\" stroke=\"#ffffff\" stroke-opacity=\"0.07\" stroke-width=\"1\">");
    let n_rings = 7usize;
    for k in 1..=n_rings {
        let r = inner_r + (outer_r - inner_r) * (k as f64 / n_rings as f64);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, cy - outer_r * 1.1);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, cy + outer_r * 1.1);
    push_b(&mut buf, b"\" stroke=\"#ffffff\" stroke-opacity=\"0.16\" stroke-width=\"1\" stroke-dasharray=\"2,5\"/>");

    for i in 0..n {
        let is_a = cfg.categories[i] == cat_a;
        let (t, center_angle) = if is_a {
            (((cfg.x_values[i] - min_a) / range_a).clamp(0.0, 1.0), PI)
        } else {
            (((cfg.x_values[i] - min_b) / range_b).clamp(0.0, 1.0), 0.0)
        };
        let r = inner_r + t * (outer_r - inner_r);
        let max_spread = t * (PI * 0.5 * 0.94);
        let jitter = hash01(i * 2 + 1) * 2.0 - 1.0;
        let angle = center_angle + jitter * max_spread;
        let (px, py) = polar_point(cx, cy, angle, r);

        let sn = (cfg.sizes[i].abs() - s_min) / s_range;
        let radius = cfg.min_size + sn * (cfg.max_size - cfg.min_size);

        let color = if has_color {
            let ct = ((cfg.color_values[i] - cv_min) / cv_range).clamp(0.0, 1.0);
            if !cfg.colorscale.is_empty() {
                colorscale_color(cfg.colorscale, ct)
            } else {
                lerp_rgb(cfg.color_low, cfg.color_high, ct)
            }
        } else if is_a {
            0xA8B0BE
        } else {
            color_b
        };
        let hx = hex6(color);

        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, px);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, py);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, radius);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.88\"/>");

        let label = cfg.labels.get(i).map(|s| s.as_str()).unwrap_or("");
        slots.push(
            HoverSlot::new(if label.is_empty() { cfg.categories[i].clone() } else { label.to_string() })
                .kv("Groupe", cfg.categories[i].clone())
                .kv("Position", format!("{:.2}", cfg.x_values[i]))
                .kv("Taille", format!("{:.1}", cfg.sizes[i])),
        );
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#f1f5f9\" letter-spacing=\"3\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let leg_y = h - 34;
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, w / 2 - 90);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y);
    push_b(&mut buf, b"\" r=\"6\" fill=\"#A8B0BE\" fill-opacity=\"0.88\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, w / 2 - 78);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 4);
    push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"9.5\" fill=\"#cbd5e1\">");
    escape_xml(&mut buf, &cat_a);
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, w / 2 + 20);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y);
    push_b(&mut buf, b"\" r=\"6\" fill=\"#");
    buf.extend_from_slice(&hex6(color_b));
    push_b(&mut buf, b"\" fill-opacity=\"0.88\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, w / 2 + 32);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 4);
    push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"9.5\" fill=\"#cbd5e1\">");
    escape_xml(&mut buf, &cat_b);
    push_b(&mut buf, b"</text>");

    let sizes_legend = [cfg.min_size, (cfg.min_size + cfg.max_size) / 2.0, cfg.max_size];
    let sl_x0 = 26;
    for (k, &r) in sizes_legend.iter().enumerate() {
        let sy = h - 26 - r as i32;
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, sl_x0 + k as i32 * 46);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, sy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#94a3b8\" stroke-opacity=\"0.55\" stroke-width=\"1\"/>");
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::statistical::bubble::config::BubbleConfig;

    fn cfg<'a>(
        x: &'a [f64],
        sizes: &'a [f64],
        cats: &'a [String],
        colv: &'a [f64],
    ) -> BubbleConfig<'a> {
        BubbleConfig {
            title: "Test",
            x_values: x,
            sizes,
            categories: cats,
            color_values: colv,
            width: 900,
            height: 700,
            ..BubbleConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<f64>, Vec<f64>, Vec<String>, Vec<f64>) {
        let x: Vec<f64> = (0..n).map(|i| (i as f64 + 1.0) / (n as f64 + 1.0)).collect();
        let sizes: Vec<f64> = (0..n).map(|i| 5.0 + (i % 7) as f64 * 4.0).collect();
        let cats: Vec<String> = (0..n).map(|i| if i % 2 == 0 { "avant".to_string() } else { "apres".to_string() }).collect();
        let colv: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
        (x, sizes, cats, colv)
    }

    #[test]
    fn renders_one_bubble_per_point_on_a_black_background() {
        let (x, sizes, cats, colv) = synth(20);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle data-idx=").count(), 20);
        assert!(html.contains("fill=\"#050507\""));
    }

    #[test]
    fn left_half_points_stay_in_the_left_hemisphere_and_right_in_the_right() {
        let (x, sizes, cats, colv) = synth(30);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        let cx = 450.0;
        for cap in html.match_indices("<circle data-idx=") {
            let start = cap.0;
            let chunk = &html[start..(start + 200).min(html.len())];
            let idx: usize = chunk
                .split("data-idx=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .and_then(|s| s.parse().ok())
                .unwrap();
            let cx_val: f64 = chunk
                .split("cx=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .and_then(|s| s.parse().ok())
                .unwrap();
            if idx % 2 == 0 {
                assert!(cx_val <= cx + 1.0, "avant point {idx} should stay left, got cx={cx_val}");
            } else {
                assert!(cx_val >= cx - 1.0, "apres point {idx} should stay right, got cx={cx_val}");
            }
        }
    }

    #[test]
    fn single_category_returns_empty_string_instead_of_a_broken_chart() {
        let x = vec![0.1, 0.5, 0.9];
        let sizes = vec![5.0, 10.0, 15.0];
        let cats = vec!["only".to_string(); 3];
        let colv: Vec<f64> = vec![];
        assert!(render(&cfg(&x, &sizes, &cats, &colv)).is_empty());
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let x: Vec<f64> = vec![];
        let sizes: Vec<f64> = vec![];
        let cats: Vec<String> = vec![];
        let colv: Vec<f64> = vec![];
        assert!(render(&cfg(&x, &sizes, &cats, &colv)).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_burst_stays_fast() {
        let (x, sizes, cats, colv) = synth(1200);
        let start = std::time::Instant::now();
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
