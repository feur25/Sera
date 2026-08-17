use super::config::CircosConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open};

#[crate::chart_demo(
    "labels=[\"Radiohead\",\"Nirvana\",\"Pink Floyd\",\"Led Zeppelin\",\"Queen\",\"The Beatles\",\"Madonna\",\"Michael Jackson\",\"Beyonce\",\"Taylor Swift\",\"Ariana Grande\",\"Rihanna\",\"Miles Davis\",\"John Coltrane\",\"Duke Ellington\",\"Ella Fitzgerald\",\"Nina Simone\",\"Chet Baker\",\"Daft Punk\",\"Aphex Twin\",\"Kraftwerk\",\"Boards of Canada\",\"Four Tet\",\"Bonobo\",\"Kendrick Lamar\",\"Nas\",\"Outkast\",\"J Dilla\",\"MF DOOM\",\"Wu-Tang Clan\"], categories=[\"Rock\",\"Rock\",\"Rock\",\"Rock\",\"Rock\",\"Rock\",\"Pop\",\"Pop\",\"Pop\",\"Pop\",\"Pop\",\"Pop\",\"Jazz\",\"Jazz\",\"Jazz\",\"Jazz\",\"Jazz\",\"Jazz\",\"Electronic\",\"Electronic\",\"Electronic\",\"Electronic\",\"Electronic\",\"Electronic\",\"HipHop\",\"HipHop\",\"HipHop\",\"HipHop\",\"HipHop\",\"HipHop\"], axes=[\"10-19\",\"20-29\",\"30-39\",\"40-49\",\"50-59\",\"60+\"], matrix=[[17.8,7.6,23.2,20.3,12.4,10.2],[17.1,19.0,16.9,22.3,3.1,14.6],[13.6,19.9,26.9,11.4,8.9,14.0],[2.4,10.3,8.0,6.6,3.1,15.8],[4.3,6.5,14.5,28.3,3.5,10.1],[11.9,17.9,26.8,28.1,7.0,9.5],[18.6,32.2,19.2,4.7,5.2,6.2],[13.6,19.3,12.6,6.7,2.1,9.5],[19.0,21.9,19.2,14.4,11.3,13.1],[31.2,5.3,18.2,16.0,17.7,16.4],[19.9,16.5,3.9,13.4,3.1,3.2],[12.7,8.9,8.1,2.9,2.0,4.7],[3.8,8.5,2.5,17.7,26.1,10.7],[6.5,8.3,8.6,4.2,34.6,45.7],[10.4,10.7,3.5,3.8,16.3,15.6],[16.9,4.9,2.4,19.1,23.0,10.7],[11.8,2.5,11.5,19.6,35.1,33.4],[6.7,8.6,5.0,15.9,23.2,36.9],[7.9,9.6,26.6,19.7,17.3,16.5],[16.7,24.5,9.7,11.3,8.4,2.5],[2.5,11.2,10.7,14.5,19.2,10.1],[18.9,31.7,30.7,8.6,6.0,6.1],[5.5,9.1,21.2,18.2,17.1,10.6],[13.8,26.2,5.6,13.9,18.4,16.1],[27.9,21.2,5.2,16.2,8.0,16.4],[35.1,18.3,9.2,19.0,15.0,5.1],[7.7,9.4,18.3,16.5,4.6,16.9],[35.4,27.7,8.3,11.9,4.4,2.3],[35.1,27.4,11.5,18.8,9.8,17.7],[30.4,11.6,6.5,7.3,6.3,12.6]], series=[[45.9,32.1,72.1,25.8,62.9,49.3,24.6,60.6,23.0,54.7,25.6,27.3,54.0,86.1,29.9,37.9,70.2,95.8,66.2,51.7,98.1,23.7,88.7,43.2,31.5,29.4,44.7,85.3,34.5,66.5],[23.4,14.2,35.1,9.4,23.0,19.8,12.8,27.7,9.9,27.2,11.9,11.6,29.6,45.2,12.3,18.7,33.8,54.5,35.2,21.8,58.4,9.0,40.3,23.3,12.2,13.9,16.1,44.1,18.7,32.8]], series_names=[\"Total Plays\",\"Total Listeners\"], edges_i=[3,3,4,7,10,11,13,15,13,22,22,21,29,28,28,1,27,22,12,5,23], edges_j=[4,5,5,11,9,10,12,14,12,18,21,22,25,24,24,25,2,7,23,20,2], variant=\"basic\", title=\"Listening Habits by Cluster\", width=1180, height=1040"
)]
pub fn render(cfg: &CircosConfig) -> String {
    let n = cfg.item_labels.len();
    if n == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w * 0.37;
    let cy = h * 0.535;
    let r = 330.0_f64.min(w * 0.4).min(h * 0.38);

    let mut group_order: Vec<&str> = Vec::new();
    for g in cfg.item_groups {
        if !group_order.contains(&g.as_str()) {
            group_order.push(g.as_str());
        }
    }
    let n_groups = group_order.len().max(1);
    let group_of = |i: usize| -> usize {
        cfg.item_groups
            .get(i)
            .and_then(|g| group_order.iter().position(|x| *x == g.as_str()))
            .unwrap_or(0)
    };

    let gap = 0.3_f64.to_radians();
    let avail = std::f64::consts::TAU - gap * n_groups as f64;
    let mut group_span: Vec<(f64, f64)> = Vec::with_capacity(n_groups);
    let mut cursor = -std::f64::consts::FRAC_PI_2;
    for gi in 0..n_groups {
        let count = cfg.item_groups.iter().filter(|g| group_order.iter().position(|x| *x == g.as_str()) == Some(gi)).count();
        let span = avail * count as f64 / n as f64;
        group_span.push((cursor, cursor + span));
        cursor += span + gap;
    }

    let item_gap = 0.12_f64.to_radians();
    let mut item_angle = vec![0.0_f64; n];
    let mut item_slot = vec![0.0_f64; n];
    let mut counts_seen = vec![0usize; n_groups];
    for i in 0..n {
        let gi = group_of(i);
        let (g1, g2) = group_span[gi];
        let count = cfg.item_groups.iter().filter(|g| group_order.iter().position(|x| *x == g.as_str()) == Some(gi)).count().max(1);
        let slot = (g2 - g1) / count as f64;
        let k = counts_seen[gi];
        item_angle[i] = g1 + slot * (k as f64 + 0.5);
        item_slot[i] = slot - item_gap;
        counts_seen[gi] += 1;
    }

    let r_bound_out = r - 3.0;
    let r_bound_in = r_bound_out - 12.0;

    let n_bar = cfg.bar_series.len();
    let bar_band_total = 140.0;
    let per_bar_h = if n_bar > 0 { (bar_band_total - (n_bar as f64 - 1.0) * 5.0) / n_bar as f64 } else { 0.0 };
    let mut bar_rings: Vec<(f64, f64)> = Vec::with_capacity(n_bar);
    let mut bc = r_bound_in - 6.0;
    for _ in 0..n_bar {
        bar_rings.push((bc - per_bar_h, bc));
        bc -= per_bar_h + 5.0;
    }
    let after_bars = if n_bar > 0 { bc + 5.0 - 10.0 } else { r_bound_in - 6.0 };

    let n_cat = cfg.heat_categories.len();
    let heat_band_total = 130.0_f64.min((after_bars - 60.0).max(0.0));
    let cat_h = if n_cat > 0 { heat_band_total / n_cat as f64 } else { 0.0 };
    let r_heat_out = after_bars;
    let r_heat_in = (r_heat_out - heat_band_total).max(48.0);

    let ink = "#1f2530";
    let sub_ink = "#6b7280";

    let mut b = Vec::<u8>::with_capacity(8192 + n * 400 + cfg.link_sources.len() * 120);
    svg_open(&mut b, cfg.width, cfg.height);

    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"32\" y=\"42\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"22\" font-weight=\"800\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);

    let e = cfg.link_sources.len().min(cfg.link_targets.len());
    push_b(&mut b, b"<g fill=\"none\" stroke=\"#334155\" stroke-opacity=\"0.30\" stroke-width=\"0.8\">");
    for k in 0..e {
        let si = cfg.link_sources[k];
        let ti = cfg.link_targets[k];
        if si < 0 || ti < 0 {
            continue;
        }
        let (si, ti) = (si as usize, ti as usize);
        if si >= n || ti >= n {
            continue;
        }
        let sa = item_angle[si];
        let ta = item_angle[ti];
        let (sx, sy) = (cx + r_heat_in * sa.cos(), cy + r_heat_in * sa.sin());
        let (tx, ty) = (cx + r_heat_in * ta.cos(), cy + r_heat_in * ta.sin());
        push_b(&mut b, b"<path d=\"M");
        push_f2(&mut b, sx);
        push_b(&mut b, b",");
        push_f2(&mut b, sy);
        push_b(&mut b, b" Q");
        push_f2(&mut b, cx);
        push_b(&mut b, b",");
        push_f2(&mut b, cy);
        push_b(&mut b, b" ");
        push_f2(&mut b, tx);
        push_b(&mut b, b",");
        push_f2(&mut b, ty);
        push_b(&mut b, b"\"/>");
    }
    push_b(&mut b, b"</g>");

    if n_cat > 0 && !cfg.heat_matrix.is_empty() {
        let heat_max = cfg.heat_matrix.iter().flat_map(|row| row.iter().copied()).fold(0.0_f64, f64::max).max(1e-9);
        for ci in 0..n_cat {
            let ring_out = r_heat_out - ci as f64 * cat_h;
            let ring_in = ring_out - cat_h;
            for i in 0..n {
                let v = cfg.heat_matrix.get(i).and_then(|row| row.get(ci)).copied().unwrap_or(0.0);
                let t = (v / heat_max).clamp(0.0, 1.0);
                let color = heat_color(t);
                let a1 = item_angle[i] - item_slot[i] / 2.0;
                let a2 = item_angle[i] + item_slot[i] / 2.0;
                push_b(&mut b, b"<path fill=\"");
                push_b(&mut b, color.as_bytes());
                push_b(&mut b, b"\" d=\"");
                ring_wedge(&mut b, cx, cy, ring_in, ring_out, a1, a2);
                push_b(&mut b, b"\"/>");
            }
        }
    }

    for (bi, &(ring_in, ring_out)) in bar_rings.iter().enumerate() {
        let color = hex6(palette_color(cfg.palette, bi));
        let max_v = cfg.bar_series[bi].1.iter().copied().fold(0.0_f64, f64::max).max(1e-9);
        for i in 0..n {
            let v = cfg.bar_series[bi].1.get(i).copied().unwrap_or(0.0).max(0.0);
            let frac = (v / max_v).clamp(0.0, 1.0);
            let bar_out = ring_in + (ring_out - ring_in) * frac;
            let a1 = item_angle[i] - item_slot[i] / 2.0;
            let a2 = item_angle[i] + item_slot[i] / 2.0;
            push_b(&mut b, b"<path fill=\"#");
            b.extend_from_slice(&color);
            push_b(&mut b, b"\" fill-opacity=\"0.88\" data-idx=\"");
            push_i(&mut b, (bi * n + i) as i32);
            push_b(&mut b, b"\" d=\"");
            ring_wedge(&mut b, cx, cy, ring_in, bar_out, a1, a2);
            push_b(&mut b, b"\"/>");
        }
    }

    for gi in 0..n_groups {
        let (a1, a2) = group_span[gi];
        let color = hex6(palette_color(cfg.palette, gi));
        push_b(&mut b, b"<path fill=\"#");
        b.extend_from_slice(&color);
        push_b(&mut b, b"\" d=\"");
        ring_wedge(&mut b, cx, cy, r_bound_in, r_bound_out, a1, a2);
        push_b(&mut b, b"\"/>");
    }

    for i in 0..n {
        let a = item_angle[i];
        let (lx, ly) = (cx + (r + 8.0) * a.cos(), cy + (r + 8.0) * a.sin());
        let deg = a.to_degrees();
        let (render_deg, anchor) = if a.cos() < 0.0 { (deg + 180.0, "end") } else { (deg, "start") };
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly);
        push_b(&mut b, b"\" transform=\"rotate(");
        push_f2(&mut b, render_deg);
        push_b(&mut b, b" ");
        push_f2(&mut b, lx);
        push_b(&mut b, b" ");
        push_f2(&mut b, ly);
        push_b(&mut b, b")\" text-anchor=\"");
        push_b(&mut b, anchor.as_bytes());
        push_b(&mut b, b"\" dominant-baseline=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" font-weight=\"600\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\" data-idx=\"");
        push_i(&mut b, i as i32);
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &cfg.item_labels[i]);
        push_b(&mut b, b"</text>");

        slots.push(HoverSlot::new(cfg.item_labels[i].clone()).kv("Group", cfg.item_groups.get(i).cloned().unwrap_or_default()));
    }

    let lx0 = cx + r + 190.0;
    let mut ly0 = cy - r + 6.0;
    let line_h = 24.0;

    push_b(&mut b, b"<text x=\"");
    push_f2(&mut b, lx0);
    push_b(&mut b, b"\" y=\"");
    push_f2(&mut b, ly0);
    push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"800\" fill=\"");
    push_b(&mut b, ink.as_bytes());
    push_b(&mut b, b"\">ITEM NAME</text>");
    ly0 += line_h;

    push_b(&mut b, b"<text x=\"");
    push_f2(&mut b, lx0);
    push_b(&mut b, b"\" y=\"");
    push_f2(&mut b, ly0);
    push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"800\" fill=\"");
    push_b(&mut b, ink.as_bytes());
    push_b(&mut b, b"\">CLUSTER BOUNDARIES</text>");
    ly0 += line_h;

    for (bi, (name, _)) in cfg.bar_series.iter().enumerate() {
        let color = hex6(palette_color(cfg.palette, bi));
        push_b(&mut b, b"<rect x=\"");
        push_f2(&mut b, lx0 - 16.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly0 - 9.0);
        push_b(&mut b, b"\" width=\"10\" height=\"10\" fill=\"#");
        b.extend_from_slice(&color);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly0);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"800\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &name.to_uppercase());
        push_b(&mut b, b"</text>");
        ly0 += line_h;
    }

    if n_cat > 0 {
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly0);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"800\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">CATEGORY INTENSITY</text>");
        ly0 += 18.0;
        for k in 0..10 {
            let t = k as f64 / 9.0;
            push_b(&mut b, b"<rect x=\"");
            push_f2(&mut b, lx0 - 16.0 + k as f64 * 12.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, ly0 - 9.0);
            push_b(&mut b, b"\" width=\"11\" height=\"11\" fill=\"");
            push_b(&mut b, heat_color(t).as_bytes());
            push_b(&mut b, b"\"/>");
        }
        ly0 += line_h;
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly0 - 14.0);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"");
        push_b(&mut b, sub_ink.as_bytes());
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &cfg.heat_categories.join(" / "));
        push_b(&mut b, b"</text>");
        ly0 += line_h;
    }

    if e > 0 {
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, lx0 - 16.0);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, ly0 - 4.0);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, lx0 - 2.0);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, ly0 - 4.0);
        push_b(&mut b, b"\" stroke=\"#334155\" stroke-width=\"1.2\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly0);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"800\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">CO-OCCURRENCE</text>");
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

fn ring_wedge(buf: &mut Vec<u8>, cx: f64, cy: f64, r1: f64, r2: f64, a1: f64, a2: f64) {
    let (s1x, s1y) = (cx + r2 * a1.cos(), cy + r2 * a1.sin());
    let (s2x, s2y) = (cx + r2 * a2.cos(), cy + r2 * a2.sin());
    let (e1x, e1y) = (cx + r1 * a2.cos(), cy + r1 * a2.sin());
    let (e2x, e2y) = (cx + r1 * a1.cos(), cy + r1 * a1.sin());
    let large = if a2 - a1 > std::f64::consts::PI { 1 } else { 0 };
    push_b(buf, b"M");
    push_f2(buf, s1x);
    push_b(buf, b",");
    push_f2(buf, s1y);
    push_b(buf, b"A");
    push_f2(buf, r2);
    push_b(buf, b",");
    push_f2(buf, r2);
    push_b(buf, b" 0 ");
    push_i(buf, large);
    push_b(buf, b" 1 ");
    push_f2(buf, s2x);
    push_b(buf, b",");
    push_f2(buf, s2y);
    push_b(buf, b"L");
    push_f2(buf, e1x);
    push_b(buf, b",");
    push_f2(buf, e1y);
    push_b(buf, b"A");
    push_f2(buf, r1);
    push_b(buf, b",");
    push_f2(buf, r1);
    push_b(buf, b" 0 ");
    push_i(buf, large);
    push_b(buf, b" 0 ");
    push_f2(buf, e2x);
    push_b(buf, b",");
    push_f2(buf, e2y);
    push_b(buf, b"Z");
}

fn heat_color(t: f64) -> String {
    let stops = [(0xff, 0xf6, 0xd6), (0xfb, 0x9a, 0x4b), (0xc9, 0x27, 0x37), (0x5c, 0x0a, 0x2e)];
    let t = t.clamp(0.0, 1.0);
    let seg = (stops.len() - 1) as f64 * t;
    let i = (seg.floor() as usize).min(stops.len() - 2);
    let local = seg - i as f64;
    let (r0, g0, bl0) = stops[i];
    let (r1, g1, bl1) = stops[i + 1];
    let r = (r0 as f64 + (r1 as i32 - r0 as i32) as f64 * local) as u8;
    let g = (g0 as f64 + (g1 as i32 - g0 as i32) as f64 * local) as u8;
    let bl = (bl0 as f64 + (bl1 as i32 - bl0 as i32) as f64 * local) as u8;
    format!("#{r:02x}{g:02x}{bl:02x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(
        item_labels: &'a [String],
        item_groups: &'a [String],
        bar_series: &'a [(String, Vec<f64>)],
        heat_categories: &'a [String],
        heat_matrix: &'a [Vec<f64>],
        link_sources: &'a [i32],
        link_targets: &'a [i32],
    ) -> CircosConfig<'a> {
        CircosConfig {
            title: "Test",
            item_labels,
            item_groups,
            bar_series,
            heat_categories,
            heat_matrix,
            link_sources,
            link_targets,
            width: 960,
            height: 1040,
            ..CircosConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<String>, Vec<(String, Vec<f64>)>, Vec<String>, Vec<Vec<f64>>, Vec<i32>, Vec<i32>) {
        let groups = ["A", "B", "C"];
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let item_groups: Vec<String> = (0..n).map(|i| groups[i % 3].to_string()).collect();
        let bar1: Vec<f64> = (0..n).map(|i| 10.0 + (i as f64 * 0.4).sin().abs() * 40.0).collect();
        let bar2: Vec<f64> = (0..n).map(|i| 5.0 + (i as f64 * 0.6).cos().abs() * 25.0).collect();
        let bar_series = vec![("S1".to_string(), bar1), ("S2".to_string(), bar2)];
        let cats: Vec<String> = vec!["c1".into(), "c2".into(), "c3".into()];
        let matrix: Vec<Vec<f64>> = (0..n).map(|i| vec![1.0 + i as f64, 2.0, 3.0 + i as f64 * 0.5]).collect();
        let src: Vec<i32> = (0..n as i32 / 2).collect();
        let tgt: Vec<i32> = (n as i32 / 2..n as i32).collect();
        (labels, item_groups, bar_series, cats, matrix, src, tgt)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("circos/basic.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/circos-basic.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_label_per_item() {
        let (l, g, bs, hc, hm, ls, lt) = synth(12);
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        assert!(!html.is_empty());
        assert_eq!(html.matches("dominant-baseline=\"middle\"").count(), l.len());
    }

    #[test]
    fn renders_one_wedge_per_bar_series_per_item() {
        let (l, g, bs, hc, hm, ls, lt) = synth(12);
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        let bar_wedges = html.matches("fill-opacity=\"0.88\"").count();
        assert_eq!(bar_wedges, l.len() * bs.len());
    }

    #[test]
    fn renders_one_link_path_per_edge() {
        let (l, g, bs, hc, hm, ls, lt) = synth(12);
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        let n_links = ls.len().min(lt.len());
        let link_block = html.split("stroke-opacity=\"0.30\"").nth(1).unwrap();
        let link_block = link_block.split("</g>").next().unwrap();
        assert_eq!(link_block.matches("<path d=\"M").count(), n_links);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_t: Vec<(String, Vec<f64>)> = vec![];
        let empty_m: Vec<Vec<f64>> = vec![];
        let empty_i: Vec<i32> = vec![];
        assert!(render(&cfg(&empty_s, &empty_s, &empty_t, &empty_s, &empty_m, &empty_i, &empty_i)).is_empty());
    }

    #[test]
    fn heat_color_stays_within_valid_hex_bounds_across_the_full_range() {
        for i in 0..=20 {
            let t = i as f64 / 20.0;
            let c = heat_color(t);
            assert_eq!(c.len(), 7);
            assert!(c.starts_with('#'));
        }
    }

    #[test]
    fn perf_rendering_a_large_circos_stays_fast() {
        let (l, g, bs, hc, hm, ls, lt) = synth(300);
        let start = std::time::Instant::now();
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 400, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
