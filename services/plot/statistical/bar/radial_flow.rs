use super::config::BarConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable, svg_title, truncate};
use std::collections::HashMap;

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
    push_b(buf, b"\"/>");
}

fn arc_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64) {
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    let large = if (a1 - a0).abs() > std::f64::consts::PI { 1 } else { 0 };
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

fn flow_curve(buf: &mut Vec<u8>, p0: (f64, f64), p1: (f64, f64), color: u32, width: f64) {
    let midx = p0.0 + (p1.0 - p0.0) * 0.55;
    push_b(buf, b"<path fill=\"none\" stroke=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" stroke-opacity=\"0.55\" stroke-width=\"");
    push_f2(buf, width);
    push_b(buf, b"\" d=\"M");
    push_f2(buf, p0.0);
    push_b(buf, b",");
    push_f2(buf, p0.1);
    push_b(buf, b" C");
    push_f2(buf, midx);
    push_b(buf, b",");
    push_f2(buf, p0.1);
    push_b(buf, b" ");
    push_f2(buf, midx);
    push_b(buf, b",");
    push_f2(buf, p1.1);
    push_b(buf, b" ");
    push_f2(buf, p1.0);
    push_b(buf, b",");
    push_f2(buf, p1.1);
    push_b(buf, b"\"/>");
}

#[crate::chart_demo(
    "labels=[\"KKR\",\"Blackstone\",\"Clayton Dubilier & Rice\",\"Warburg Pincus\",\"General Atlantic\",\"GTCR\",\"Thoma Bravo\",\"Francisco Partners\",\"Silver Lake\",\"Andreessen Horowitz\",\"Clearlake Capital Group\",\"Bain Capital\",\"Advent International\",\"Summit Partners\",\"CVC Capital Partners\",\"Hg\",\"Bridgepoint\",\"EQT\",\"Nordic Capital\",\"Partners Group\",\"Brookfield Asset Management\",\"Ardian\",\"PAI Partners\",\"Hillhouse Capital Group\",\"China Merchants Capital\"], values=[117.9,95.7,49.8,34.2,44.7,30.2,98.2,25.8,47.1,34.2,45.2,40.5,38.2,22.2,113.3,21.6,29.3,72.5,23.6,27.2,23.8,25.4,18.0,59.9,20.7], super_categories=[\"New York\",\"New York\",\"New York\",\"New York\",\"New York\",\"Chicago\",\"Chicago\",\"San Francisco\",\"San Francisco\",\"Menlo Park\",\"Santa Monica\",\"Boston\",\"Boston\",\"Washington DC\",\"London\",\"London\",\"London\",\"Stockholm\",\"Stockholm\",\"Zug\",\"Toronto\",\"Paris\",\"Paris\",\"Hong Kong\",\"Hong Kong\"], offset_groups=[\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"UK\",\"UK\",\"UK\",\"Sweden\",\"Sweden\",\"Switzerland\",\"Canada\",\"France\",\"France\",\"China\",\"China\"], palette=[3049182,1482885,13934615,15277708,1780298,9317439,8207041], variant=\"radial_flow\", width=1080, height=900"
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
    for &v in &cfg.values[..n] {
        vmax = vmax.max(v);
    }
    let vmax = vmax.max(1e-9);

    let cx = wf * 0.58;
    let cy = hf * 0.60;
    let start = -205.0_f64.to_radians();
    let end = 60.0_f64.to_radians();
    let sweep = end - start;
    let angle = |i: usize| -> f64 { start + sweep * i as f64 / (n - 1).max(1) as f64 };

    let r_city = hf * 0.22;
    let bar_max = hf * 0.14;
    let half_w = (r_city * (sweep.abs() / n as f64) * 0.30).min(8.0);

    let mut countries: Vec<String> = Vec::new();
    for oc in cfg.offset_groups.iter().take(n) {
        if !countries.iter().any(|c| c == oc) {
            countries.push(oc.clone());
        }
    }
    let n_countries = countries.len().max(1);
    let color_of = |country: &str| -> u32 {
        let idx = countries.iter().position(|c| c == country).unwrap_or(0);
        palette_color(cfg.palette, idx)
    };

    let mut buf = Vec::<u8>::with_capacity(n * 260 + 12_000);
    svg_open_rescalable(&mut buf, w, h, 0, 0, w, h);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    svg_title(&mut buf, cfg.title, w / 2, 24);

    let country_x = wf * 0.10;
    let country_top = cy - r_city * 0.85;
    let country_bottom = cy + r_city * 0.85;
    let country_gap = if n_countries > 1 { (country_bottom - country_top) / (n_countries - 1) as f64 } else { 0.0 };
    let country_pt: HashMap<String, (f64, f64)> = countries
        .iter()
        .enumerate()
        .map(|(ci, c)| (c.clone(), (country_x, country_top + ci as f64 * country_gap)))
        .collect();

    let mut city_angle_sum: HashMap<String, (f64, usize)> = HashMap::new();
    for i in 0..n {
        let city = cfg.super_categories.get(i).cloned().unwrap_or_default();
        let e = city_angle_sum.entry(city).or_insert((0.0, 0));
        e.0 += angle(i);
        e.1 += 1;
    }

    push_b(&mut buf, b"<g style=\"isolation:isolate\">");
    for i in 0..n {
        let country = cfg.offset_groups.get(i).map(|s| s.as_str()).unwrap_or("");
        let p0 = match country_pt.get(country) {
            Some(p) => *p,
            None => continue,
        };
        let a = angle(i);
        let p1 = (cx + (r_city - 10.0) * a.cos(), cy + (r_city - 10.0) * a.sin());
        flow_curve(&mut buf, p0, p1, color_of(country), 1.3);
    }
    push_b(&mut buf, b"</g>");

    {
        let mut seen_cities: Vec<String> = Vec::new();
        for i in 0..n {
            let city = cfg.super_categories.get(i).cloned().unwrap_or_default();
            if !seen_cities.contains(&city) {
                seen_cities.push(city);
            }
        }
        for city in &seen_cities {
            let a_mid = match city_angle_sum.get(city) {
                Some((sum, cnt)) if *cnt > 0 => sum / *cnt as f64,
                _ => continue,
            };
            let bx = cx + r_city * a_mid.cos();
            let by = cy + r_city * a_mid.sin();
            push_b(&mut buf, b"<circle cx=\"");
            push_f2(&mut buf, bx);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, by);
            push_b(&mut buf, b"\" r=\"5.5\" fill=\"#fff\" stroke=\"#94a3b8\" stroke-width=\"1.4\"/>");
        }
    }

    for (country, pt) in &country_pt {
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, pt.0);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, pt.1);
        push_b(&mut buf, b"\" r=\"5\" fill=\"#");
        buf.extend_from_slice(&hex6(color_of(country)));
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, pt.0 - 10.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, pt.1 + 3.5);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#1e293b\">");
        escape_xml(&mut buf, country);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<g stroke=\"#e2e8f0\" stroke-width=\"1\">");
    arc_path(&mut buf, cx, cy, r_city, start, end);
    push_b(&mut buf, b"/></g>");

    {
        let mut start_i = 0usize;
        while start_i < n {
            let cur = cfg.super_categories.get(start_i).map(|s| s.as_str()).unwrap_or("");
            let mut end_i = start_i + 1;
            while end_i < n && cfg.super_categories.get(end_i).map(|s| s.as_str()).unwrap_or("") == cur {
                end_i += 1;
            }
            let step = if n > 1 { sweep / (n - 1) as f64 } else { 0.0 };
            let a0 = angle(start_i) - step * 0.4;
            let a1 = angle(end_i - 1) + step * 0.4;
            let country = cfg.offset_groups.get(start_i).map(|s| s.as_str()).unwrap_or("");
            let col = color_of(country);
            push_b(&mut buf, b"<g stroke=\"#");
            buf.extend_from_slice(&hex6(col));
            push_b(&mut buf, b"\" stroke-width=\"3\">");
            arc_path(&mut buf, cx, cy, r_city, a0, a1);
            push_b(&mut buf, b"/></g>");
            let am = (a0 + a1) / 2.0;
            let lx = cx + (r_city - 14.0) * am.cos();
            let ly = cy + (r_city - 14.0) * am.sin();
            let deg = am.to_degrees() + 90.0;
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" fill=\"#fff\" font-weight=\"700\" transform=\"rotate(");
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
        let t = (v / vmax).clamp(0.0, 1.0);
        let len = 10.0 + t * (bar_max - 10.0);
        let re = r_city + len;
        let country = cfg.offset_groups.get(i).map(|s| s.as_str()).unwrap_or("");
        let color = color_of(country);
        ray_bar(&mut buf, cx, cy, a, r_city, re, half_w, color, i as i32, v, &cfg.labels[i]);

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
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8\" font-weight=\"700\" fill=\"#1e293b\" transform=\"rotate(");
        push_f2(&mut buf, deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, truncate(&cfg.labels[i], 26));
        push_b(&mut buf, b"<tspan class=\"sp-val\"> $");
        let s = format!("{:.1}B", v);
        buf.extend_from_slice(s.as_bytes());
        push_b(&mut buf, b"</tspan></text>");
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

    fn cfg<'a>(labels: &'a [String], values: &'a [f64], cities: &'a [String], countries: &'a [String]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            values,
            super_categories: cities,
            offset_groups: countries,
            width: 1080,
            height: 900,
            ..BarConfig::default()
        }
    }

    fn synth() -> (Vec<String>, Vec<f64>, Vec<String>, Vec<String>) {
        let labels: Vec<String> = (0..12).map(|i| format!("Firm {i}")).collect();
        let values: Vec<f64> = (0..12).map(|i| 10.0 + (i as f64 * 0.7).sin().abs() * 90.0).collect();
        let cities: Vec<String> = vec![
            "New York", "New York", "New York", "Chicago", "Chicago", "London", "London", "London", "Toronto", "Paris", "Paris", "Hong Kong",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let countries: Vec<String> = vec![
            "U.S.", "U.S.", "U.S.", "U.S.", "U.S.", "UK", "UK", "UK", "Canada", "France", "France", "China",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        (labels, values, cities, countries)
    }

    #[test]
    fn renders_one_ray_per_firm_and_one_flow_curve_per_country_city_pair() {
        let (labels, values, cities, countries) = synth();
        let html = render(&cfg(&labels, &values, &cities, &countries));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 12);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn every_country_gets_its_own_labeled_dot() {
        let (labels, values, cities, countries) = synth();
        let html = render(&cfg(&labels, &values, &cities, &countries));
        assert!(html.contains(">U.S.<"));
        assert!(html.contains(">UK<"));
        assert!(html.contains(">Canada<"));
        assert!(html.contains(">France<"));
        assert!(html.contains(">China<"));
    }

    #[test]
    fn every_city_gets_its_own_inner_arc_label() {
        let (labels, values, cities, countries) = synth();
        let html = render(&cfg(&labels, &values, &cities, &countries));
        assert!(html.contains(">New York<"));
        assert!(html.contains(">London<"));
        assert!(html.contains(">Hong Kong<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        let empty: Vec<String> = vec![];
        assert!(render(&cfg(&labels, &values, &empty, &empty)).is_empty());
    }

    #[test]
    fn perf_rendering_many_firms_stays_fast() {
        let labels: Vec<String> = (0..300).map(|i| format!("Firm {i}")).collect();
        let values: Vec<f64> = (0..300).map(|i| 5.0 + (i as f64 * 0.4).cos().abs() * 60.0).collect();
        let cities: Vec<String> = (0..300).map(|i| format!("City {}", i / 15)).collect();
        let countries: Vec<String> = (0..300).map(|i| format!("Country {}", i / 60)).collect();
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &values, &cities, &countries));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
