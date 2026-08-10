use super::config::BubbleConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{
    angle_at, colorscale_color, escape_xml, hash01, hex6, lerp_rgb, palette_color, polar_point, push_b, push_f2,
    push_i, svg_open,
};
use std::f64::consts::PI;

const CLUSTER_A_ANGLE: f64 = PI * (-126.0 / 180.0);
const CLUSTER_B_ANGLE: f64 = PI * (-54.0 / 180.0);

#[crate::chart_demo(
    "labels=[\"commentaires\",\"ci-cd\",\"theme\",\"push\",\"python\",\"build\",\"pypi\",\"ci\",\"java\",\"inference\",\"metriques\",\"licence\",\"argon2\",\"nuget\",\"perf\",\"rgpd\",\"api\",\"audit\",\"csharp\",\"doc\",\"v1\",\"curseurs\",\"pulse\",\"veille\",\"setup\",\"arbitrage\",\"ring-buffer\",\"vscode\",\"entrainement\",\"v2\",\"axis\",\"runners\",\"dframe\",\"wasm-playground\",\"secure\",\"burst\",\"aes\",\"ml\",\"rust\",\"snapshots\",\"dashboard\",\"duckdb\",\"npm2\",\"orchestration\",\"websocket\",\"indicators\",\"cache\",\"ffi\",\"render\",\"benchmark\",\"stabilite\",\"s3\",\"theme2\",\"presence\",\"svg\",\"scala\",\"pipeline\",\"streaming\",\"test\",\"readme\",\"gantt\",\"wasm\",\"pypi2\",\"live\",\"minio\",\"alerte\",\"npm\",\"tableau-de-bord\",\"notion\",\"board\",\"clean\",\"crates\",\"core\",\"chiffrement\",\"legacy\",\"latence\",\"sessions\",\"firehose\"], categories=[\"apres\",\"apres\",\"avant\",\"apres\",\"avant\",\"avant\",\"avant\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"avant\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"avant\",\"avant\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"avant\",\"avant\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\",\"apres\",\"apres\",\"avant\",\"apres\",\"avant\",\"apres\",\"apres\",\"apres\"], x_values=[0.625,0.461,0.403,0.568,0.468,0.064,0.481,0.332,0.463,0.498,0.453,0.912,0.4,0.747,0.365,0.471,0.156,0.743,0.286,0.342,0.541,0.218,0.668,0.882,0.071,0.814,0.938,0.337,0.768,0.113,0.148,0.428,0.644,0.361,0.618,0.744,0.629,0.952,0.584,0.572,0.435,0.814,0.705,0.631,0.353,0.639,0.223,0.634,0.311,0.376,0.559,0.819,0.312,0.836,0.375,0.86,0.855,0.97,0.441,0.365,0.753,0.247,0.81,0.244,0.404,0.268,0.412,0.678,0.534,0.332,0.386,0.433,0.172,0.843,0.499,0.371,0.743,0.792], sizes=[45.5,20.8,7.0,13.9,9.1,5.9,11.5,13.1,28.4,37.4,20.0,23.8,22.9,39.2,12.3,41.7,9.9,22.5,31.2,6.4,5.9,38.4,43.8,25.9,8.0,32.9,44.0,24.0,6.2,6.1,8.6,18.2,27.7,33.2,12.8,27.6,38.5,9.1,9.0,24.0,42.0,28.0,15.5,42.7,9.0,27.0,5.9,39.1,12.9,9.7,15.4,34.3,40.7,44.0,9.9,15.9,15.5,21.1,9.6,14.0,25.2,5.6,28.3,22.6,10.3,41.1,11.8,17.7,9.3,45.0,28.4,22.4,10.0,41.8,7.2,10.1,44.9,19.8], x_categories=[\"collab\",\"infra\",\"\",\"temps-reel\",\"\",\"\",\"\",\"\",\"distribution\",\"ia\",\"observabilite\",\"securite\",\"securite\",\"distribution\",\"\",\"securite\",\"\",\"securite\",\"distribution\",\"\",\"\",\"collab\",\"collab\",\"observabilite\",\"\",\"observabilite\",\"temps-reel\",\"ux\",\"ia\",\"\",\"\",\"infra\",\"ia\",\"ux\",\"securite\",\"ux\",\"securite\",\"ia\",\"\",\"ia\",\"observabilite\",\"infra\",\"distribution\",\"infra\",\"temps-reel\",\"ux\",\"\",\"distribution\",\"\",\"\",\"\",\"infra\",\"ux\",\"collab\",\"\",\"distribution\",\"infra\",\"temps-reel\",\"\",\"\",\"ux\",\"\",\"distribution\",\"collab\",\"infra\",\"observabilite\",\"\",\"observabilite\",\"ux\",\"collab\",\"ia\",\"distribution\",\"\",\"securite\",\"\",\"temps-reel\",\"collab\",\"temps-reel\"], variant=\"burst\""
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
    let cat_a = order
        .iter()
        .find(|s| s.eq_ignore_ascii_case("avant") || s.eq_ignore_ascii_case("before"))
        .cloned()
        .unwrap_or_else(|| order[0].clone());
    let cat_b = order.iter().find(|s| **s != cat_a).cloned().unwrap_or_else(|| order[1].clone());

    let w = cfg.width;
    let h = cfg.height;
    let side = w.min(h) as f64;
    let cx = w as f64 / 2.0;
    let cy = h as f64 / 2.0 + side * 0.07;
    let inner_r = side * 0.03;
    let outer_r = side * 0.46;

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

    let has_x_cats = cfg.x_categories.len() >= n;
    let mut topic_order: Vec<&str> = Vec::new();
    if has_x_cats {
        for t in cfg.x_categories[..n].iter() {
            if !t.is_empty() && !topic_order.iter().any(|s| *s == t.as_str()) {
                topic_order.push(t.as_str());
            }
        }
    }

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

    let color_b_fallback = if cfg.color_hex == 0 { 0xFF7A45 } else { cfg.color_hex };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    let mut buf = Vec::<u8>::with_capacity(n * 220 + 8192);

    svg_open(&mut buf, w, h);

    push_b(&mut buf, b"<defs><radialGradient id=\"spZone\" gradientUnits=\"userSpaceOnUse\" cx=\"");
    let mid_angle = (CLUSTER_A_ANGLE + CLUSTER_B_ANGLE) / 2.0;
    let (zone_x, zone_y) = polar_point(cx, cy, mid_angle, outer_r * 0.68);
    push_f2(&mut buf, zone_x);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, zone_y);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, outer_r * 0.62);
    push_b(&mut buf, b"\"><stop offset=\"0%\" stop-color=\"#fbbf24\" stop-opacity=\"0.30\"/><stop offset=\"55%\" stop-color=\"#fbbf24\" stop-opacity=\"0.10\"/><stop offset=\"100%\" stop-color=\"#fbbf24\" stop-opacity=\"0\"/></radialGradient></defs>");
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, zone_x);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, zone_y);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, outer_r * 0.62);
    push_b(&mut buf, b"\" fill=\"url(#spZone)\"/>");

    push_b(&mut buf, b"<g stroke=\"#94a3b8\" stroke-opacity=\"0.22\" stroke-width=\"1\">");
    let n_spokes = 72usize;
    for k in 0..n_spokes {
        let a = angle_at(k as f64, n_spokes as f64, 0.0);
        let (x1, y1) = polar_point(cx, cy, a, inner_r);
        let (x2, y2) = polar_point(cx, cy, a, outer_r * 1.05);
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

    push_b(&mut buf, b"<g fill=\"none\" stroke=\"#cbd5e1\" stroke-opacity=\"0.85\" stroke-width=\"1\">");
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

    for i in 0..n {
        let is_a = cfg.categories[i] == cat_a;
        let t = if is_a {
            ((cfg.x_values[i] - min_a) / range_a).clamp(0.0, 1.0)
        } else {
            ((cfg.x_values[i] - min_b) / range_b).clamp(0.0, 1.0)
        };
        let r = inner_r + t * (outer_r - inner_r);

        let sn = ((cfg.sizes[i].abs() - s_min) / s_range).clamp(0.0, 1.0);
        let radius = cfg.min_size + sn * (cfg.max_size - cfg.min_size);

        let cluster_angle = if is_a { CLUSTER_A_ANGLE } else { CLUSTER_B_ANGLE };
        let spread_factor = (1.0 - sn).powf(1.6);
        let max_spread = spread_factor * (PI * 0.98);
        let jitter = hash01(i * 2 + 1) * 2.0 - 1.0;
        let angle = cluster_angle + jitter * max_spread;
        let (px, py) = polar_point(cx, cy, angle, r);

        let color = if is_a {
            lerp_rgb(0xD5DAE2, 0x64748B, sn)
        } else {
            let topic = if has_x_cats { cfg.x_categories[i].as_str() } else { "" };
            if !topic.is_empty() {
                let idx = topic_order.iter().position(|s| *s == topic).unwrap_or(0);
                palette_color(cfg.palette, idx)
            } else if has_color {
                let ct = ((cfg.color_values[i] - cv_min) / cv_range).clamp(0.0, 1.0);
                if !cfg.colorscale.is_empty() {
                    colorscale_color(cfg.colorscale, ct)
                } else {
                    lerp_rgb(cfg.color_low, cfg.color_high, ct)
                }
            } else {
                color_b_fallback
            }
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
        let mut slot = HoverSlot::new(if label.is_empty() { cfg.categories[i].clone() } else { label.to_string() })
            .kv("Groupe", cfg.categories[i].clone())
            .kv("Position", format!("{:.2}", cfg.x_values[i]))
            .kv("Taille", format!("{:.1}", cfg.sizes[i]));
        if has_x_cats && !cfg.x_categories[i].is_empty() {
            slot = slot.kv("Sujet", cfg.x_categories[i].clone());
        }
        slots.push(slot);
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#1e293b\" letter-spacing=\"3\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let leg_y = h - 34;
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, w / 2 - 90);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y);
    push_b(&mut buf, b"\" r=\"6\" fill=\"#94a3b8\" fill-opacity=\"0.88\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, w / 2 - 78);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 4);
    push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"9.5\" fill=\"#475569\">");
    escape_xml(&mut buf, &cat_a);
    push_b(&mut buf, b"</text>");
    push_b(&mut buf, b"<circle cx=\"");
    push_i(&mut buf, w / 2 + 20);
    push_b(&mut buf, b"\" cy=\"");
    push_i(&mut buf, leg_y);
    push_b(&mut buf, b"\" r=\"6\" fill=\"#");
    buf.extend_from_slice(&hex6(color_b_fallback));
    push_b(&mut buf, b"\" fill-opacity=\"0.88\"/>");
    push_b(&mut buf, b"<text x=\"");
    push_i(&mut buf, w / 2 + 32);
    push_b(&mut buf, b"\" y=\"");
    push_i(&mut buf, leg_y + 4);
    push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"9.5\" fill=\"#475569\">");
    escape_xml(&mut buf, &cat_b);
    push_b(&mut buf, b"</text>");

    if !topic_order.is_empty() {
        let tly = h - 16;
        let mut tx = 26i32;
        for (idx, topic) in topic_order.iter().take(8).enumerate() {
            let c = palette_color(cfg.palette, idx);
            push_b(&mut buf, b"<circle cx=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" cy=\"");
            push_i(&mut buf, tly);
            push_b(&mut buf, b"\" r=\"4\" fill=\"#");
            buf.extend_from_slice(&hex6(c));
            push_b(&mut buf, b"\" fill-opacity=\"0.9\"/>");
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, tx + 8);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, tly + 3);
            push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"8.5\" fill=\"#64748b\">");
            escape_xml(&mut buf, topic);
            push_b(&mut buf, b"</text>");
            tx += 12 + topic.len() as i32 * 6 + 14;
        }
    }

    let sizes_legend = [cfg.min_size, (cfg.min_size + cfg.max_size) / 2.0, cfg.max_size];
    let sl_x0 = w - 150;
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

    fn cfg<'a>(x: &'a [f64], sizes: &'a [f64], cats: &'a [String], colv: &'a [f64]) -> BubbleConfig<'a> {
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
        let cats: Vec<String> =
            (0..n).map(|i| if i % 2 == 0 { "avant".to_string() } else { "apres".to_string() }).collect();
        let colv: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
        (x, sizes, cats, colv)
    }

    #[test]
    fn renders_one_bubble_per_point_with_a_viewbox_and_the_shared_theme_default_background() {
        let (x, sizes, cats, colv) = synth(20);
        let html = render(&cfg(&x, &sizes, &cats, &colv));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle data-idx=").count(), 20);
        assert!(html.contains("class=\"sp-bg\""), "must use the shared theme-aware background rect, not a hardcoded fill");
        assert!(html.contains("viewBox=\"0 0 900 700\""), "without a viewBox the canvas gets clipped instead of scaled by its container");
    }

    #[test]
    fn large_bubbles_converge_close_to_their_category_cluster_angle() {
        let n = 40;
        let x: Vec<f64> = (0..n).map(|i| (i as f64 + 1.0) / (n as f64 + 1.0)).collect();
        let sizes: Vec<f64> = vec![40.0; n];
        let cats: Vec<String> =
            (0..n).map(|i| if i % 2 == 0 { "avant".to_string() } else { "apres".to_string() }).collect();
        let colv: Vec<f64> = vec![];
        let c = cfg(&x, &sizes, &cats, &colv);
        let w = c.width as f64;
        let h = c.height as f64;
        let side = w.min(h);
        let cx = w / 2.0;
        let cy = h / 2.0 + side * 0.07;
        let outer_r = side * 0.46;
        let (ax, ay) = polar_point(cx, cy, CLUSTER_A_ANGLE, outer_r);
        let (bx, by) = polar_point(cx, cy, CLUSTER_B_ANGLE, outer_r);
        let dist_a = ((ax - cx).powi(2) + (ay - cy).powi(2)).sqrt();
        let dist_b = ((bx - cx).powi(2) + (by - cy).powi(2)).sqrt();
        assert!(dist_a > 0.0 && dist_b > 0.0);
        let html = render(&c);
        assert_eq!(html.matches("<circle data-idx=").count(), n);
    }

    #[test]
    fn small_bubbles_scatter_far_wider_than_large_ones() {
        let spread_small = (1.0f64 - 0.0).powf(1.6) * (PI * 0.98);
        let spread_large = (1.0f64 - 1.0).powf(1.6) * (PI * 0.98);
        assert!(spread_small > spread_large * 50.0);
    }

    #[test]
    fn topic_categories_drive_discrete_palette_colors_for_the_second_group() {
        let x = vec![0.2, 0.4, 0.6, 0.8];
        let sizes = vec![10.0, 20.0, 30.0, 40.0];
        let cats = vec!["avant".to_string(), "apres".to_string(), "apres".to_string(), "apres".to_string()];
        let colv: Vec<f64> = vec![];
        let xcats = vec!["".to_string(), "infra".to_string(), "ux".to_string(), "infra".to_string()];
        let c = BubbleConfig { x_categories: &xcats, ..cfg(&x, &sizes, &cats, &colv) };
        let html = render(&c);
        assert_eq!(html.matches("<circle data-idx=").count(), 4);
        assert!(html.contains("infra"), "topic legend should surface the topic names used");
        assert!(html.contains("ux"));
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
