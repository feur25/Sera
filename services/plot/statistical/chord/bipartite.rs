use super::common::arc_path;
use super::config::ChordConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open};

#[crate::chart_demo(
    "labels=[\"Jenny Slate\",\"Jake Lacy\",\"Obvious Child\",\"Crazy Ex-Girlfriend\",\"Rachel Bloom\",\"Broad City\",\"Master of None\",\"Atlanta\",\"Jennifer Lawrence\",\"Hillary Clinton\",\"Amy Poehler\",\"Meryl Streep\",\"Ashley Graham\",\"Jack Antonoff\",\"Taylor Swift\",\"Lupita Nyong'o\",\"Riz Ahmed\",\"Star Wars\",\"Adam Driver\",\"Donald Glover\",\"Zachary Quinto\",\"Amy Schumer\",\"Trainwreck\",\"Judd Apatow\",\"Gillian Jacobs\"], categories=[\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\"], axes=[\"Promoted Feminism\",\"Featured Millennial Malaise\",\"Cited Girls as Inspiration\",\"Depicted Realistic Sex\",\"Collaborated with Dunham\",\"Created Starring Women\",\"Guest Starred on Girls\"], edges_i=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,0,3,5,6,8,10,14,17,19], edges_j=[5,5,5,5,5,5,5,5,0,0,0,2,2,3,3,2,4,4,6,6,6,1,1,1,1,6,6,6,6,3,5,0,4,6], variant=\"bipartite\", title=\"How Girls Shaped Pop Culture\", width=1240, height=1080"
)]
pub fn render(cfg: &ChordConfig) -> String {
    let n_items = cfg.item_labels.len();
    let n_attrs = cfg.attr_labels.len();
    if n_items == 0 || n_attrs == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w * 0.50;
    let cy = h * 0.51;

    let mut group_order: Vec<&str> = Vec::new();
    for g in cfg.item_groups {
        if !group_order.contains(&g.as_str()) {
            group_order.push(g.as_str());
        }
    }
    group_order.reverse();
    let n_groups = group_order.len().max(1);

    let mut group_items: Vec<Vec<usize>> = vec![Vec::new(); n_groups];
    for (i, g) in cfg.item_groups.iter().enumerate() {
        let gi = group_order.iter().position(|x| *x == g.as_str()).unwrap_or(0);
        group_items[gi].push(i);
    }

    let top_deg = -90.0_f64;
    let ring_span = 191.0_f64.to_radians();
    let split_gap = 4.0_f64.to_radians();
    let items_span = 360.0_f64.to_radians() - ring_span - 2.0 * split_gap;

    let ring_start = top_deg.to_radians() + split_gap;
    let ring_end = ring_start + ring_span;
    let items_start = ring_end + split_gap;

    let group_gap = 6.0_f64.to_radians();
    let avail = items_span - group_gap * (n_groups.saturating_sub(1)) as f64;

    let mut item_angle = vec![0.0_f64; n_items];
    let mut group_span: Vec<(f64, f64)> = Vec::with_capacity(n_groups);
    let mut cursor = items_start;
    for items in &group_items {
        let m = items.len().max(1);
        let span = avail * items.len() as f64 / n_items as f64;
        for (k, &idx) in items.iter().enumerate() {
            item_angle[idx] = cursor + span * (k as f64 + 0.5) / m as f64;
        }
        group_span.push((cursor, cursor + span));
        cursor += span + group_gap;
    }

    let r_items = h * 0.335;
    let bracket_r = r_items + 148.0;

    let mut degree = vec![0u32; n_attrs];
    let e = cfg.link_items.len().min(cfg.link_attrs.len());
    for k in 0..e {
        let ai = cfg.link_attrs[k];
        if ai >= 0 && (ai as usize) < n_attrs {
            degree[ai as usize] += 1;
        }
    }
    let degree_total: u32 = degree.iter().sum::<u32>().max(1);

    let attr_gap = 1.4_f64.to_radians();
    let attr_avail = ring_span - attr_gap * (n_attrs.saturating_sub(1)) as f64;

    let mut attr_arc: Vec<(f64, f64)> = Vec::with_capacity(n_attrs);
    let mut ac = ring_start;
    for &d in &degree {
        let span = attr_avail * (d.max(1) as f64) / degree_total as f64;
        attr_arc.push((ac, ac + span));
        ac += span + attr_gap;
    }

    let r_attr_in = h * 0.26;
    let r_attr_out = r_attr_in + 96.0;
    let r_attr_mid = (r_attr_in + r_attr_out) / 2.0;

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n_items * 220 + n_attrs * 260 + e * 160 + 8192);
    html_prefix(&mut buf, cfg.title, hid);
    svg_open(&mut buf, cfg.width, cfg.height);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, w / 2.0);
        push_b(&mut buf, b"\" y=\"30\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"18\" font-weight=\"800\" fill=\"#1a202c\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n_items + n_attrs);

    push_b(&mut buf, b"<g fill=\"none\">");
    for k in 0..e {
        let ii = cfg.link_items[k];
        let ai = cfg.link_attrs[k];
        if ii < 0 || ai < 0 {
            continue;
        }
        let (ii, ai) = (ii as usize, ai as usize);
        if ii >= n_items || ai >= n_attrs {
            continue;
        }
        let a = item_angle[ii];
        let sx = cx + r_items * a.cos();
        let sy = cy + r_items * a.sin();
        let (ta1, ta2) = attr_arc[ai];
        let tam = (ta1 + ta2) / 2.0;
        let tx = cx + r_attr_in * tam.cos();
        let ty = cy + r_attr_in * tam.sin();
        let color = hex6(palette_color(cfg.palette, ai));

        push_b(&mut buf, b"<path stroke=\"#");
        buf.extend_from_slice(&color);
        push_b(&mut buf, b"\" stroke-width=\"1.1\" stroke-opacity=\"0.38\" data-idx=\"");
        push_i(&mut buf, k as i32);
        push_b(&mut buf, b"\" d=\"M");
        push_f2(&mut buf, sx);
        push_b(&mut buf, b",");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b" C");
        push_f2(&mut buf, cx + (sx - cx) * 0.42);
        push_b(&mut buf, b",");
        push_f2(&mut buf, cy + (sy - cy) * 0.42);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, cx + (tx - cx) * 0.42);
        push_b(&mut buf, b",");
        push_f2(&mut buf, cy + (ty - cy) * 0.42);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, tx);
        push_b(&mut buf, b",");
        push_f2(&mut buf, ty);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    for (gi, &(g1, g2)) in group_span.iter().enumerate() {
        let (x1, y1) = (cx + bracket_r * g1.cos(), cy + bracket_r * g1.sin());
        let (x2, y2) = (cx + bracket_r * g2.cos(), cy + bracket_r * g2.sin());
        let large = if g2 - g1 > std::f64::consts::PI { 1 } else { 0 };
        push_b(&mut buf, b"<path fill=\"none\" stroke=\"#1e293b\" stroke-width=\"1.4\" d=\"M");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b" A");
        push_f2(&mut buf, bracket_r);
        push_b(&mut buf, b",");
        push_f2(&mut buf, bracket_r);
        push_b(&mut buf, b" 0 ");
        push_i(&mut buf, large);
        push_b(&mut buf, b" 1 ");
        push_f2(&mut buf, x2);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y2);
        push_b(&mut buf, b"\"/>");

        let gm = (g1 + g2) / 2.0;
        let (lx, ly) = (cx + (bracket_r + 18.0) * gm.cos(), cy + (bracket_r + 18.0) * gm.sin());
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"13\" font-weight=\"800\" letter-spacing=\"0.6\" fill=\"#0f172a\">");
        escape_xml(&mut buf, &group_order.get(gi).copied().unwrap_or("").to_uppercase());
        push_b(&mut buf, b"</text>");
    }

    for i in 0..n_items {
        let a = item_angle[i];
        let x = cx + r_items * a.cos();
        let y = cy + r_items * a.sin();
        let deg = a.to_degrees();
        let render_deg = deg + 180.0;

        let mut tick_colors: Vec<[u8; 6]> = Vec::new();
        for k in 0..e {
            if cfg.link_items[k] == i as i32 {
                let ai = cfg.link_attrs[k];
                if ai >= 0 && (ai as usize) < n_attrs {
                    tick_colors.push(hex6(palette_color(cfg.palette, ai as usize)));
                }
            }
        }

        push_b(&mut buf, b"<g transform=\"translate(");
        push_f2(&mut buf, x);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, y);
        push_b(&mut buf, b") rotate(");
        push_f2(&mut buf, render_deg);
        push_b(&mut buf, b")\">");

        let mut tick_x = 8.0_f64;
        for c in &tick_colors {
            push_b(&mut buf, b"<rect x=\"");
            push_f2(&mut buf, tick_x);
            push_b(&mut buf, b"\" y=\"-4\" width=\"5\" height=\"8\" fill=\"#");
            buf.extend_from_slice(c);
            push_b(&mut buf, b"\"/>");
            tick_x += 7.0;
        }

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, tick_x + 4.0);
        push_b(&mut buf, b"\" y=\"3.5\" text-anchor=\"start\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10.5\" font-weight=\"600\" fill=\"#334155\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.item_labels[i]);
        push_b(&mut buf, b"</text></g>");

        slots.push(HoverSlot::new(cfg.item_labels[i].clone()).kv("Group", cfg.item_groups.get(i).cloned().unwrap_or_default()));
    }

    for (ai, &(a1, a2)) in attr_arc.iter().enumerate() {
        let color = hex6(palette_color(cfg.palette, ai));
        push_b(&mut buf, b"<path fill=\"#");
        buf.extend_from_slice(&color);
        push_b(&mut buf, b"\" data-idx=\"");
        push_i(&mut buf, (n_items + ai) as i32);
        push_b(&mut buf, b"\" d=\"");
        arc_path(&mut buf, cx, cy, r_attr_in, r_attr_out, a1, a2);
        push_b(&mut buf, b"\"/>");

        let am = (a1 + a2) / 2.0;
        let lx = cx + r_attr_mid * am.cos();
        let ly = cy + r_attr_mid * am.sin();
        let tangent = am - std::f64::consts::FRAC_PI_2;
        let mut deg = tangent.to_degrees();
        if tangent.cos() < 0.0 {
            deg += 180.0;
        }

        let upper_label = cfg.attr_labels[ai].to_uppercase();
        let lines = wrap_label(&upper_label);
        let line_h = 11.0;
        let first_dy = -(lines.len() as f64 - 1.0) * line_h / 2.0;

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" transform=\"rotate(");
        push_f2(&mut buf, deg);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b" ");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b")\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"8.5\" font-weight=\"800\" letter-spacing=\"0.1\" fill=\"#ffffff\">");
        for (li, line) in lines.iter().enumerate() {
            push_b(&mut buf, b"<tspan x=\"");
            push_f2(&mut buf, lx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ly + first_dy + li as f64 * line_h);
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, line);
            push_b(&mut buf, b"</tspan>");
        }
        push_b(&mut buf, b"</text>");

        slots.push(HoverSlot::new(cfg.attr_labels[ai].clone()).kv("Linked items", degree[ai].to_string()));
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(&slots));
    unsafe { String::from_utf8_unchecked(buf) }
}

fn wrap_label(label: &str) -> Vec<&str> {
    let words: Vec<&str> = label.split_whitespace().collect();
    if words.len() <= 1 {
        return vec![label];
    }
    let mut lines = Vec::new();
    let mut start = 0usize;
    let mut acc = 0usize;
    for (i, word) in words.iter().enumerate() {
        acc += word.len() + 1;
        if acc >= 8 && i > start {
            lines.push(join_range(label, &words, start, i + 1));
            start = i + 1;
            acc = 0;
        }
    }
    if start < words.len() {
        lines.push(join_range(label, &words, start, words.len()));
    }
    lines
}

fn join_range<'a>(label: &'a str, words: &[&'a str], start: usize, end: usize) -> &'a str {
    if start >= end || end > words.len() {
        return label;
    }
    let a = words[start].as_ptr() as usize - label.as_ptr() as usize;
    let last = words[end - 1];
    let b = (last.as_ptr() as usize - label.as_ptr() as usize) + last.len();
    &label[a..b]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(
        item_labels: &'a [String],
        item_groups: &'a [String],
        attr_labels: &'a [String],
        link_items: &'a [i32],
        link_attrs: &'a [i32],
    ) -> ChordConfig<'a> {
        ChordConfig {
            title: "Test",
            item_labels,
            item_groups,
            attr_labels,
            link_items,
            link_attrs,
            width: 1000,
            height: 900,
            ..ChordConfig::default()
        }
    }

    fn synth(n_items: usize, n_attrs: usize) -> (Vec<String>, Vec<String>, Vec<String>, Vec<i32>, Vec<i32>) {
        let groups = ["Groundbreakers", "Activism", "Blockbusters"];
        let item_labels: Vec<String> = (0..n_items).map(|i| format!("Item {i}")).collect();
        let item_groups: Vec<String> = (0..n_items).map(|i| groups[i % 3].to_string()).collect();
        let attr_labels: Vec<String> = (0..n_attrs).map(|i| format!("Attribute {i}")).collect();
        let link_items: Vec<i32> = (0..n_items as i32).collect();
        let link_attrs: Vec<i32> = (0..n_items as i32).map(|i| i % n_attrs as i32).collect();
        (item_labels, item_groups, attr_labels, link_items, link_attrs)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("chord/bipartite.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/chord-bipartite.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_link_path_per_edge_and_one_arc_per_attribute() {
        let (il, ig, al, li, la) = synth(9, 3);
        let html = render(&cfg(&il, &ig, &al, &li, &la));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path stroke=\"#").count(), li.len());
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn items_and_the_attribute_ring_tile_the_full_circle_without_overlapping() {
        let top = (-90.0_f64).to_radians();
        let ring_span = 191.0_f64.to_radians();
        let split_gap = 4.0_f64.to_radians();
        let items_span = 2.0 * std::f64::consts::PI - ring_span - 2.0 * split_gap;
        let ring_start = top + split_gap;
        let ring_end = ring_start + ring_span;
        let items_start = ring_end + split_gap;
        let items_end = items_start + items_span;
        assert!((items_end + split_gap - (ring_start + 2.0 * std::f64::consts::PI)).abs() < 1e-9, "the two arcs plus their two gaps must close the full circle exactly");
    }

    #[test]
    fn wrap_label_never_drops_or_duplicates_characters() {
        let label = "CREATED BY AND STARRING WOMEN";
        let lines = wrap_label(label);
        let rejoined: String = lines.join(" ");
        assert_eq!(rejoined, label);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_i: Vec<i32> = vec![];
        assert!(render(&cfg(&empty_s, &empty_s, &empty_s, &empty_i, &empty_i)).is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_bipartite_graph_stays_fast() {
        let (il, ig, al, li, la) = synth(300, 12);
        let start = std::time::Instant::now();
        let html = render(&cfg(&il, &ig, &al, &li, &la));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 300, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
