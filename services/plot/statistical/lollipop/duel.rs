use super::common::{prepare, unique_groups};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open_rescalable,
};

fn arc(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, a0: f64, a1: f64, stroke: u32, width: f64, opacity: f64) {
    let x0 = cx + r * a0.cos();
    let y0 = cy + r * a0.sin();
    let x1 = cx + r * a1.cos();
    let y1 = cy + r * a1.sin();
    let large = if (a1 - a0).abs() > std::f64::consts::PI { 1 } else { 0 };
    let sweep_flag = if a1 > a0 { 1 } else { 0 };
    push_b(buf, b"<path fill=\"none\" stroke=\"#");
    buf.extend_from_slice(&hex6(stroke));
    push_b(buf, b"\" stroke-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\" stroke-width=\"");
    push_f2(buf, width);
    push_b(buf, b"\" d=\"M");
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
    buf.push(sweep_flag + b'0');
    push_b(buf, b" ");
    push_f2(buf, x1);
    push_b(buf, b",");
    push_f2(buf, y1);
    push_b(buf, b"\"/>");
}

fn lollipop_head(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, frac: f64, color: u32) {
    push_b(buf, b"<circle cx=\"");
    push_f2(buf, cx);
    push_b(buf, b"\" cy=\"");
    push_f2(buf, cy);
    push_b(buf, b"\" r=\"");
    push_f2(buf, r);
    push_b(buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"1.1\"/>");
    if frac >= 0.999 {
        push_b(buf, b"<circle cx=\"");
        push_f2(buf, cx);
        push_b(buf, b"\" cy=\"");
        push_f2(buf, cy);
        push_b(buf, b"\" r=\"");
        push_f2(buf, r);
        push_b(buf, b"\" fill=\"#");
        buf.extend_from_slice(&hex6(color));
        push_b(buf, b"\"/>");
        return;
    }
    let ha0 = -std::f64::consts::FRAC_PI_2;
    let ha1 = ha0 + frac * std::f64::consts::TAU;
    let x0 = cx + r * ha0.cos();
    let y0 = cy + r * ha0.sin();
    let x1 = cx + r * ha1.cos();
    let y1 = cy + r * ha1.sin();
    let large = if ha1 - ha0 > std::f64::consts::PI { 1 } else { 0 };
    push_b(buf, b"<path fill=\"#");
    buf.extend_from_slice(&hex6(color));
    push_b(buf, b"\" d=\"M");
    push_f2(buf, cx);
    push_b(buf, b",");
    push_f2(buf, cy);
    push_b(buf, b" L");
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
    push_b(buf, b" Z\"/>");
}

fn callout(buf: &mut Vec<u8>, ax: f64, ay: f64, bx: f64, by: f64, lines: &[&str], accent: u32) {
    let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as f64 * 5.6 + 20.0;
    let h = lines.len() as f64 * 14.0 + 12.0;
    let bx0 = bx - w / 2.0;
    let by0 = by - h;
    push_b(buf, b"<line x1=\"");
    push_f2(buf, ax);
    push_b(buf, b"\" y1=\"");
    push_f2(buf, ay);
    push_b(buf, b"\" x2=\"");
    push_f2(buf, bx);
    push_b(buf, b"\" y2=\"");
    push_f2(buf, by);
    push_b(buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(accent));
    push_b(buf, b"\" stroke-width=\"1\" stroke-dasharray=\"2,2\"/>");
    push_b(buf, b"<circle cx=\"");
    push_f2(buf, ax);
    push_b(buf, b"\" cy=\"");
    push_f2(buf, ay);
    push_b(buf, b"\" r=\"2.4\" fill=\"#");
    buf.extend_from_slice(&hex6(accent));
    push_b(buf, b"\"/>");
    push_b(buf, b"<rect x=\"");
    push_f2(buf, bx0);
    push_b(buf, b"\" y=\"");
    push_f2(buf, by0);
    push_b(buf, b"\" width=\"");
    push_f2(buf, w);
    push_b(buf, b"\" height=\"");
    push_f2(buf, h);
    push_b(buf, b"\" rx=\"5\" fill=\"#0f172a\" stroke=\"#");
    buf.extend_from_slice(&hex6(accent));
    push_b(buf, b"\" stroke-width=\"1\" opacity=\"0.94\"/>");
    for (li, line) in lines.iter().enumerate() {
        let font_size: &[u8] = if li == 0 { b"10" } else { b"9" };
        push_b(buf, b"<text x=\"");
        push_f2(buf, bx);
        push_b(buf, b"\" y=\"");
        push_f2(buf, by0 + 16.0 + li as f64 * 14.0);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"");
        push_b(buf, font_size);
        push_b(buf, b"\" font-weight=\"");
        push_b(buf, if li == 0 { b"700" } else { b"400" });
        push_b(buf, b"\" fill=\"#");
        buf.extend_from_slice(&hex6(if li == 0 { 0xf8fafc } else { 0x94a3b8 }));
        push_b(buf, b"\">");
        escape_xml(buf, line);
        push_b(buf, b"</text>");
    }
}

#[crate::chart_demo(
    "labels=[\"1929\",\"1929\",\"1929\",\"1934\",\"1943\",\"1953\",\"1953\",\"1960\",\"1964\",\"1970\",\"1974\",\"1974\",\"1980\",\"1980\",\"1986\",\"1986\",\"1989\",\"1994\",\"1994\",\"1994\",\"1999\",\"1999\",\"2002\",\"2002\",\"2002\",\"2002\",\"2005\",\"2005\",\"2008\",\"2008\",\"2008\",\"2009\",\"2010\",\"2011\",\"2011\",\"2013\",\"2013\",\"2015\",\"2015\",\"2015\",\"2017\",\"2017\",\"2019\",\"2019\",\"2023\",\"2023\",\"2023\"], values=[20,51,84,65,56,12,71,29,74,29,38,54,72,88,13,48,80,41,60,75,32,90,37,44,58,78,54,63,44,45,89,35,8,74,83,50,86,15,22,79,51,64,18,56,46,49,88], color_groups=[\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Barcelona\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Barcelona\",\"Barcelona\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Barcelona\",\"Barcelona\",\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Barcelona\",\"Real Madrid\",\"Real Madrid\"], palette=[3316734,15547189], title=\"El Clasico - Every Goal, by Minute and Era\", width=1000, height=800"
)]

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };

    let cx = 96.0;
    let cy = 730.0;
    let r_min = 54.0;
    let r_max = 600.0;
    let r_band = r_max + 16.0;

    let a0 = 0.0_f64;
    let a1 = -std::f64::consts::FRAC_PI_2;
    let minute_angle = |m: f64| -> f64 { a0 + (a1 - a0) * (m.clamp(0.0, 90.0) / 90.0) };

    let teams = unique_groups(&p);
    let color_of_team = |g: &str| -> u32 {
        if cfg.palette.is_empty() {
            return cfg.color_hex;
        }
        match teams.iter().position(|u| u.as_str() == g) {
            Some(idx) => palette_color(cfg.palette, idx),
            None => cfg.color_hex,
        }
    };

    let mut bounds: Vec<(usize, usize)> = Vec::new();
    let mut start_i = 0usize;
    while start_i < p.n {
        let cur = p.labels[start_i].as_str();
        let mut end_i = start_i + 1;
        while end_i < p.n && p.labels[end_i].as_str() == cur {
            end_i += 1;
        }
        bounds.push((start_i, end_i));
        start_i = end_i;
    }
    let n_matches = bounds.len().max(1);
    let radius_of = |mi: usize| -> f64 { r_min + (r_max - r_min) * mi as f64 / (n_matches - 1).max(1) as f64 };
    let max_goals = bounds.iter().map(|&(s, e)| e - s).max().unwrap_or(1).max(1);

    let mut b = Vec::<u8>::with_capacity(8192);
    svg_open_rescalable(&mut b, cfg.width, cfg.height, 0, 0, cfg.width, cfg.height);
    push_b(&mut b, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    push_b(&mut b, b"<title>");
    escape_xml(&mut b, if cfg.title.is_empty() { "Chart" } else { cfg.title });
    push_b(&mut b, b"</title>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"24\" y=\"30\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"16\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    for (ti, t) in teams.iter().take(2).enumerate() {
        let ly = 54.0 + ti as f64 * 18.0;
        let col = color_of_team(t);
        push_b(&mut b, b"<circle cx=\"30\" cy=\"");
        push_f2(&mut b, ly - 4.0);
        push_b(&mut b, b"\" r=\"5\" fill=\"#");
        b.extend_from_slice(&hex6(col));
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\"42\" y=\"");
        push_f2(&mut b, ly);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#334155\">");
        escape_xml(&mut b, t);
        push_b(&mut b, b"</text>");
    }

    arc(&mut b, cx, cy, r_band, a0, a1, 0x0f172a, 10.0, 1.0);

    for m in [0.0_f64, 45.0, 90.0] {
        let am = minute_angle(m);
        let rl = r_band + 20.0;
        let xl = cx + rl * am.cos();
        let yl = cy + rl * am.sin();
        let anchor: &[u8] = if m <= 0.0 { b"start" } else { b"middle" };
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, xl);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, yl + 3.0);
        push_b(&mut b, b"\" text-anchor=\"");
        b.extend_from_slice(anchor);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#475569\">");
        let s = format!("{}'", m as i32);
        b.extend_from_slice(s.as_bytes());
        push_b(&mut b, b"</text>");
    }

    let mut fastest_idx = 0usize;
    for i in 1..p.n {
        if p.values[i] < p.values[fastest_idx] {
            fastest_idx = i;
        }
    }
    let mut biggest_mi = 0usize;
    for (mi, (s, e)) in bounds.iter().enumerate() {
        let (bs, be) = bounds[biggest_mi];
        if e - s > be - bs {
            biggest_mi = mi;
        }
    }

    for (mi, &(s, e)) in bounds.iter().enumerate() {
        let r = radius_of(mi);
        let mut c0 = 0i32;
        let mut c1 = 0i32;
        for i in s..e {
            if teams.first().map(|t| t.as_str()) == Some(p.groups[i].as_str()) {
                c0 += 1;
            } else if teams.get(1).map(|t| t.as_str()) == Some(p.groups[i].as_str()) {
                c1 += 1;
            }
        }
        let arc_col = if c0 > c1 {
            color_of_team(teams.first().map(|s| s.as_str()).unwrap_or(""))
        } else if c1 > c0 {
            color_of_team(teams.get(1).map(|s| s.as_str()).unwrap_or(""))
        } else {
            0x475569
        };
        arc(&mut b, cx, cy, r, a0, a1, arc_col, 1.1, 0.5);

        let frac = ((e - s) as f64 / max_goals as f64).max(0.16);
        for i in s..e {
            let am = minute_angle(p.values[i]);
            let col = color_of_team(&p.groups[i]);
            let x0 = cx + (r - 4.0) * am.cos();
            let y0 = cy + (r - 4.0) * am.sin();
            let x1 = cx + (r + 2.0) * am.cos();
            let y1 = cy + (r + 2.0) * am.sin();
            push_b(&mut b, b"<line data-idx=\"");
            push_i(&mut b, i as i32);
            push_b(&mut b, b"\" data-y=\"");
            push_f2(&mut b, p.values[i]);
            push_b(&mut b, b"\" data-lbl=\"");
            escape_xml(&mut b, &p.labels[i]);
            push_b(&mut b, b"\" x1=\"");
            push_f2(&mut b, x0);
            push_b(&mut b, b"\" y1=\"");
            push_f2(&mut b, y0);
            push_b(&mut b, b"\" x2=\"");
            push_f2(&mut b, x1);
            push_b(&mut b, b"\" y2=\"");
            push_f2(&mut b, y1);
            push_b(&mut b, b"\" stroke=\"#");
            b.extend_from_slice(&hex6(col));
            push_b(&mut b, b"\" stroke-width=\"1.6\"/>");
            lollipop_head(&mut b, x1, y1, 5.6, frac, col);
        }

        let lr = cx + r;
        let lx = lr;
        let ly = cy + 14.0;
        let deg = -55.0;
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\" transform=\"rotate(");
        push_f2(&mut b, deg);
        push_b(&mut b, b" ");
        push_f2(&mut b, lx);
        push_b(&mut b, b" ");
        push_f2(&mut b, ly);
        push_b(&mut b, b")\">");
        escape_xml(&mut b, &p.labels[s]);
        push_b(&mut b, b"</text>");

        if mi == biggest_mi {
            let anchor_i = e - 1;
            let am = minute_angle(p.values[anchor_i]);
            let ax = cx + r * am.cos();
            let ay = cy + r * am.sin();
            let line1 = format!("{} - {} goals", p.labels[s], e - s);
            callout(&mut b, ax, ay, ax + 90.0, ay - 60.0, &[&line1, "busiest fixture on record"], arc_col);
        }
    }

    {
        let am = minute_angle(p.values[fastest_idx]);
        let mi = bounds.iter().position(|&(s, e)| fastest_idx >= s && fastest_idx < e).unwrap_or(0);
        let r = radius_of(mi);
        let ax = cx + r * am.cos();
        let ay = cy + r * am.sin();
        let line1 = format!("{}' - {}", p.values[fastest_idx] as i32, p.labels[fastest_idx]);
        let col = color_of_team(&p.groups[fastest_idx]);
        callout(&mut b, ax, ay, ax - 70.0, ay - 90.0, &[&line1, "earliest goal on record"], col);
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    crate::html::hover::build_chart_html(cfg.title, &svg, &crate::html::hover::slots_to_json(cfg.hover))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], values: &'a [f64], groups: &'a [String]) -> LollipopConfig<'a> {
        LollipopConfig {
            title: "Test",
            labels,
            values,
            groups,
            width: 1000,
            height: 800,
            ..LollipopConfig::default()
        }
    }

    fn synth() -> (Vec<String>, Vec<f64>, Vec<String>) {
        let years = ["1929", "1934", "1953", "1974", "1994", "2011", "2023"];
        let mut labels = Vec::new();
        let mut values = Vec::new();
        let mut groups = Vec::new();
        for (yi, y) in years.iter().enumerate() {
            let n = 1 + yi % 3;
            for k in 0..n {
                labels.push(y.to_string());
                values.push(((yi * 13 + k * 7) % 90) as f64 + 1.0);
                groups.push(if (yi + k) % 2 == 0 { "Real Madrid".to_string() } else { "Barcelona".to_string() });
            }
        }
        (labels, values, groups)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("lollipop/duel.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/lollipop-duel.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_arc_per_match_and_one_dot_per_goal() {
        let (labels, values, groups) = synth();
        let n_goals = labels.len();
        let n_matches = { let mut s: Vec<&str> = labels.iter().map(|s| s.as_str()).collect(); s.dedup(); s.len() };
        let html = render(&cfg(&labels, &values, &groups));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<line data-idx=").count(), n_goals);
        assert_eq!(html.matches("<path fill=\"none\"").count(), n_matches + 1);
    }

    #[test]
    fn every_goal_dot_stays_within_the_outer_band_radius() {
        let (labels, values, groups) = synth();
        let cfg_v = cfg(&labels, &values, &groups);
        let html = render(&cfg_v);
        let cx = 96.0_f64;
        let cy = 730.0_f64;
        let r_band = 616.0_f64;
        for line in html.split("<line data-idx=").skip(1) {
            let x2 = line.split("x2=\"").nth(1).unwrap().split('"').next().unwrap().parse::<f64>().unwrap();
            let y2 = line.split("y2=\"").nth(1).unwrap().split('"').next().unwrap().parse::<f64>().unwrap();
            let r = ((x2 - cx).powi(2) + (y2 - cy).powi(2)).sqrt();
            assert!(r <= r_band + 1.0);
        }
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        let groups: Vec<String> = vec![];
        assert!(render(&cfg(&labels, &values, &groups)).is_empty());
    }

    #[test]
    fn perf_rendering_many_matches_stays_fast() {
        let mut labels = Vec::new();
        let mut values = Vec::new();
        let mut groups = Vec::new();
        for y in 1900..2200 {
            for k in 0..2 {
                labels.push(format!("{y}"));
                values.push(((y * 3 + k * 11) % 90) as f64 + 1.0);
                groups.push(if (y + k) % 2 == 0 { "A".to_string() } else { "B".to_string() });
            }
        }
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &values, &groups));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 300, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
