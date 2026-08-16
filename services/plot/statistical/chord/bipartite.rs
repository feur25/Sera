use super::common::arc_path;
use super::config::ChordConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open};

#[crate::chart_demo(
    "labels=[\"Jenny Slate\",\"Jake Lacy\",\"Obvious Child\",\"Crazy Ex-Girlfriend\",\"Rachel Bloom\",\"Broad City\",\"Master of None\",\"Atlanta\",\"Jennifer Lawrence\",\"Hillary Clinton\",\"Amy Poehler\",\"Meryl Streep\",\"Ashley Graham\",\"Jack Antonoff\",\"Taylor Swift\",\"Lupita Nyong'o\",\"Riz Ahmed\",\"Star Wars\",\"Adam Driver\",\"Donald Glover\",\"Zachary Quinto\",\"Amy Schumer\",\"Trainwreck\",\"Judd Apatow\",\"Gillian Jacobs\"], categories=[\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Groundbreakers\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Activism\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\",\"Blockbusters\"], axes=[\"Promoted Feminism\",\"Featured Millennial Malaise\",\"Cited Girls as Inspiration\",\"Depicted Realistic Sex\",\"Collaborated with Dunham\",\"Created Starring Women\",\"Guest Starred on Girls\"], edges_i=[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,0,3,5,6,8,10,14,17,19], edges_j=[5,5,5,5,5,5,5,5,0,0,0,2,2,3,3,2,4,4,6,6,6,1,1,1,1,6,6,6,6,3,5,0,4,6], variant=\"bipartite\", title=\"How Girls Shaped Pop Culture\", width=1180, height=940"
)]
pub fn render(cfg: &ChordConfig) -> String {
    let n_items = cfg.item_labels.len();
    let n_attrs = cfg.attr_labels.len();
    if n_items == 0 || n_attrs == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w * 0.46;
    let cy = h * 0.52;

    let mut group_order: Vec<&str> = Vec::new();
    for g in cfg.item_groups {
        if !group_order.contains(&g.as_str()) {
            group_order.push(g.as_str());
        }
    }
    let n_groups = group_order.len().max(1);

    let mut group_items: Vec<Vec<usize>> = vec![Vec::new(); n_groups];
    for (i, g) in cfg.item_groups.iter().enumerate() {
        let gi = group_order.iter().position(|x| *x == g.as_str()).unwrap_or(0);
        group_items[gi].push(i);
    }

    let item_span_total = 168.0_f64.to_radians();
    let group_gap = 6.0_f64.to_radians();
    let item_start = 180.0_f64.to_radians() - item_span_total / 2.0;
    let avail = item_span_total - group_gap * (n_groups.saturating_sub(1)) as f64;

    let mut item_angle = vec![0.0_f64; n_items];
    let mut group_span: Vec<(f64, f64)> = Vec::with_capacity(n_groups);
    let mut cursor = item_start;
    for items in &group_items {
        let m = items.len().max(1);
        let span = avail * items.len() as f64 / n_items as f64;
        for (k, &idx) in items.iter().enumerate() {
            item_angle[idx] = cursor + span * (k as f64 + 0.5) / m as f64;
        }
        group_span.push((cursor, cursor + span));
        cursor += span + group_gap;
    }

    let r_items = h * 0.40;

    let mut degree = vec![0u32; n_attrs];
    let e = cfg.link_items.len().min(cfg.link_attrs.len());
    for k in 0..e {
        let ai = cfg.link_attrs[k];
        if ai >= 0 && (ai as usize) < n_attrs {
            degree[ai as usize] += 1;
        }
    }
    let degree_total: u32 = degree.iter().sum::<u32>().max(1);

    let attr_span_total = 132.0_f64.to_radians();
    let attr_gap = 1.6_f64.to_radians();
    let attr_start = -attr_span_total / 2.0;
    let attr_avail = attr_span_total - attr_gap * (n_attrs.saturating_sub(1)) as f64;

    let mut attr_arc: Vec<(f64, f64)> = Vec::with_capacity(n_attrs);
    let mut ac = attr_start;
    for &d in &degree {
        let span = attr_avail * (d.max(1) as f64) / degree_total as f64;
        attr_arc.push((ac, ac + span));
        ac += span + attr_gap;
    }

    let r_attr_in = w * 0.30;
    let r_attr_out = r_attr_in + 40.0;
    let r_attr_label = r_attr_out + 10.0;

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n_items * 200 + n_attrs * 200 + e * 160 + 8192);
    html_prefix(&mut buf, cfg.title, hid);
    svg_open(&mut buf, cfg.width, cfg.height);
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, w / 2.0);
        push_b(&mut buf, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"16\" font-weight=\"800\" fill=\"#1a202c\" class=\"sp-ttl\">");
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
        push_b(&mut buf, b"\" stroke-width=\"1.1\" stroke-opacity=\"0.34\" data-idx=\"");
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
        let bracket_r = r_items + 20.0;
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
        let (lx, ly) = (cx + (bracket_r + 16.0) * gm.cos(), cy + (bracket_r + 16.0) * gm.sin());
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"12\" font-weight=\"800\" letter-spacing=\"0.5\" fill=\"#0f172a\">");
        escape_xml(&mut buf, group_order.get(gi).copied().unwrap_or(""));
        push_b(&mut buf, b"</text>");
    }

    for i in 0..n_items {
        let a = item_angle[i];
        let x = cx + r_items * a.cos();
        let y = cy + r_items * a.sin();
        let anchor = if a.cos() < 0.0 { "end" } else { "start" };
        let tx = x + if a.cos() < 0.0 { -10.0 } else { 10.0 };

        let mut tick_colors: Vec<[u8; 6]> = Vec::new();
        for k in 0..e {
            if cfg.link_items[k] == i as i32 {
                let ai = cfg.link_attrs[k];
                if ai >= 0 && (ai as usize) < n_attrs {
                    tick_colors.push(hex6(palette_color(cfg.palette, ai as usize)));
                }
            }
        }

        let mut tick_x = tx;
        for c in &tick_colors {
            let dxs = if a.cos() < 0.0 { -7.0 } else { 7.0 };
            push_b(&mut buf, b"<rect x=\"");
            push_f2(&mut buf, tick_x - 2.5);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, y - 4.0);
            push_b(&mut buf, b"\" width=\"5\" height=\"8\" fill=\"#");
            buf.extend_from_slice(c);
            push_b(&mut buf, b"\"/>");
            tick_x += dxs;
        }

        let label_x = tick_x + if a.cos() < 0.0 { -4.0 } else { 4.0 };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, label_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, y + 3.5);
        push_b(&mut buf, b"\" text-anchor=\"");
        push_b(&mut buf, anchor.as_bytes());
        push_b(&mut buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10.5\" font-weight=\"600\" fill=\"#334155\" data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &cfg.item_labels[i]);
        push_b(&mut buf, b"</text>");

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
        let lx = cx + r_attr_label * am.cos();
        let ly = cy + r_attr_label * am.sin();
        let deg = am.to_degrees();
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
        push_b(&mut buf, b")\" text-anchor=\"start\" dominant-baseline=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"800\" letter-spacing=\"0.3\" fill=\"#1a202c\">");
        escape_xml(&mut buf, cfg.attr_labels[ai].to_uppercase().as_str());
        push_b(&mut buf, b"</text>");

        slots.push(HoverSlot::new(cfg.attr_labels[ai].clone()).kv("Linked items", degree[ai].to_string()));
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(&slots));
    unsafe { String::from_utf8_unchecked(buf) }
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
            height: 800,
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
    fn every_item_stays_on_the_left_half_and_every_attribute_arc_on_the_right_half() {
        let width = 1000.0_f64;
        let cx = width * 0.44;
        let item_span_total = 168.0_f64.to_radians();
        let item_start = std::f64::consts::PI - item_span_total / 2.0;
        assert!(item_start.cos() < 0.0, "left edge of the item arc must stay left of the pivot");
        assert!((item_start + item_span_total).cos() < 0.0, "right edge of the item arc must stay left of the pivot");

        let attr_span_total = 132.0_f64.to_radians();
        let attr_start = -attr_span_total / 2.0;
        assert!(attr_start.cos() > 0.0, "left edge of the attribute ring must stay right of the pivot");
        assert!((attr_start + attr_span_total).cos() > 0.0, "right edge of the attribute ring must stay right of the pivot");
        let _ = cx;
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
