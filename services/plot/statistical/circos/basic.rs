use super::config::CircosConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_open};

#[crate::chart_demo(
    "labels=[\"Radiohead\",\"Nirvana\",\"Pink Floyd\",\"Led Zeppelin\",\"Queen\",\"The Beatles\",\"Madonna\",\"Michael Jackson\",\"Beyonce\",\"Taylor Swift\",\"Ariana Grande\",\"Rihanna\",\"Miles Davis\",\"John Coltrane\",\"Duke Ellington\",\"Ella Fitzgerald\",\"Nina Simone\",\"Chet Baker\",\"Daft Punk\",\"Aphex Twin\",\"Kraftwerk\",\"Boards of Canada\",\"Four Tet\",\"Bonobo\",\"Kendrick Lamar\",\"Nas\",\"Outkast\",\"J Dilla\",\"MF DOOM\",\"Wu-Tang Clan\"], categories=[\"Rock\",\"Rock\",\"Rock\",\"Rock\",\"Rock\",\"Rock\",\"Pop\",\"Pop\",\"Pop\",\"Pop\",\"Pop\",\"Pop\",\"Jazz\",\"Jazz\",\"Jazz\",\"Jazz\",\"Jazz\",\"Jazz\",\"Electronic\",\"Electronic\",\"Electronic\",\"Electronic\",\"Electronic\",\"Electronic\",\"HipHop\",\"HipHop\",\"HipHop\",\"HipHop\",\"HipHop\",\"HipHop\"], axes=[\"10-19\",\"20-29\",\"30-39\",\"40-49\",\"50-59\",\"60+\"], matrix=[[17.8,7.6,23.2,20.3,12.4,10.2],[17.1,19.0,16.9,22.3,3.1,14.6],[13.6,19.9,26.9,11.4,8.9,14.0],[2.4,10.3,8.0,6.6,3.1,15.8],[4.3,6.5,14.5,28.3,3.5,10.1],[11.9,17.9,26.8,28.1,7.0,9.5],[18.6,32.2,19.2,4.7,5.2,6.2],[13.6,19.3,12.6,6.7,2.1,9.5],[19.0,21.9,19.2,14.4,11.3,13.1],[31.2,5.3,18.2,16.0,17.7,16.4],[19.9,16.5,3.9,13.4,3.1,3.2],[12.7,8.9,8.1,2.9,2.0,4.7],[3.8,8.5,2.5,17.7,26.1,10.7],[6.5,8.3,8.6,4.2,34.6,45.7],[10.4,10.7,3.5,3.8,16.3,15.6],[16.9,4.9,2.4,19.1,23.0,10.7],[11.8,2.5,11.5,19.6,35.1,33.4],[6.7,8.6,5.0,15.9,23.2,36.9],[7.9,9.6,26.6,19.7,17.3,16.5],[16.7,24.5,9.7,11.3,8.4,2.5],[2.5,11.2,10.7,14.5,19.2,10.1],[18.9,31.7,30.7,8.6,6.0,6.1],[5.5,9.1,21.2,18.2,17.1,10.6],[13.8,26.2,5.6,13.9,18.4,16.1],[27.9,21.2,5.2,16.2,8.0,16.4],[35.1,18.3,9.2,19.0,15.0,5.1],[7.7,9.4,18.3,16.5,4.6,16.9],[35.4,27.7,8.3,11.9,4.4,2.3],[35.1,27.4,11.5,18.8,9.8,17.7],[30.4,11.6,6.5,7.3,6.3,12.6]], series=[[45.9,32.1,72.1,25.8,62.9,49.3,24.6,60.6,23.0,54.7,25.6,27.3,54.0,86.1,29.9,37.9,70.2,95.8,66.2,51.7,98.1,23.7,88.7,43.2,31.5,29.4,44.7,85.3,34.5,66.5],[23.4,14.2,35.1,9.4,23.0,19.8,12.8,27.7,9.9,27.2,11.9,11.6,29.6,45.2,12.3,18.7,33.8,54.5,35.2,21.8,58.4,9.0,40.3,23.3,12.2,13.9,16.1,44.1,18.7,32.8]], series_names=[\"Total Plays\",\"Total Listeners\"], edges_i=[3,3,4,7,10,11,13,15,13,22,22,21,29,28,28,1,27,22,12,5,23], edges_j=[4,5,5,11,9,10,12,14,12,18,21,22,25,24,24,25,2,7,23,20,2], variant=\"basic\", title=\"Listening Habits by Cluster\", width=1400, height=1300"
)]
pub fn render(cfg: &CircosConfig) -> String {
    let n = cfg.item_labels.len();
    if n == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w * 0.40;
    let cy = h * 0.54;
    let r = 380.0_f64.min(w * 0.4).min(h * 0.36);

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
    let group_count = |gi: usize| -> usize {
        cfg.item_groups.iter().filter(|g| group_order.iter().position(|x| *x == g.as_str()) == Some(gi)).count()
    };

    let gap = 0.3_f64.to_radians();
    let avail = std::f64::consts::TAU - gap * n_groups as f64;
    let mut group_span: Vec<(f64, f64)> = Vec::with_capacity(n_groups);
    let mut cursor = -std::f64::consts::FRAC_PI_2;
    for gi in 0..n_groups {
        let span = avail * group_count(gi) as f64 / n as f64;
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
        let count = group_count(gi).max(1);
        let slot = (g2 - g1) / count as f64;
        let k = counts_seen[gi];
        item_angle[i] = g1 + slot * (k as f64 + 0.5);
        item_slot[i] = slot - item_gap;
        counts_seen[gi] += 1;
    }

    let n_cat = cfg.heat_categories.len();

    let avg_age_idx: Vec<f64> = (0..n)
        .map(|i| {
            let row = cfg.heat_matrix.get(i);
            match row {
                Some(row) if !row.is_empty() && n_cat > 1 => {
                    let total: f64 = row.iter().sum();
                    if total <= 0.0 {
                        0.0
                    } else {
                        row.iter().enumerate().map(|(c, &v)| c as f64 * v).sum::<f64>() / total
                    }
                }
                _ => 0.0,
            }
        })
        .collect();
    let avg_age_t: Vec<f64> = avg_age_idx.iter().map(|&v| if n_cat > 1 { v / (n_cat as f64 - 1.0) } else { 0.0 }).collect();

    let age_spread: Vec<f64> = (0..n)
        .map(|i| {
            let row = cfg.heat_matrix.get(i);
            match row {
                Some(row) if !row.is_empty() && n_cat > 1 => {
                    let total: f64 = row.iter().sum();
                    if total <= 0.0 {
                        0.0
                    } else {
                        let mean = avg_age_idx[i];
                        let var = row.iter().enumerate().map(|(c, &v)| v * (c as f64 - mean).powi(2)).sum::<f64>() / total;
                        var.sqrt()
                    }
                }
                _ => 0.0,
            }
        })
        .collect();

    let ratio: Vec<f64> = (0..n)
        .map(|i| {
            let plays = cfg.bar_series.first().and_then(|(_, v)| v.get(i)).copied().unwrap_or(0.0);
            let listeners = cfg.bar_series.get(1).and_then(|(_, v)| v.get(i)).copied().unwrap_or(0.0).max(1e-9);
            plays / listeners
        })
        .collect();
    let ratio_max = ratio.iter().copied().fold(0.0_f64, f64::max).max(1e-9);
    let ratio_t: Vec<f64> = ratio.iter().map(|&v| (v / ratio_max).clamp(0.0, 1.0)).collect();

    let mut rank_order: Vec<usize> = (0..n).collect();
    let plays0: Vec<f64> = cfg.bar_series.first().map(|(_, v)| v.clone()).unwrap_or_default();
    rank_order.sort_by(|&a, &b| {
        plays0.get(b).copied().unwrap_or(0.0).partial_cmp(&plays0.get(a).copied().unwrap_or(0.0)).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut rank_t = vec![0.0_f64; n];
    let mut rank_pos = vec![0usize; n];
    for (pos, &idx) in rank_order.iter().enumerate() {
        rank_t[idx] = if n > 1 { pos as f64 / (n as f64 - 1.0) } else { 0.0 };
        rank_pos[idx] = pos;
    }

    let e = cfg.link_sources.len().min(cfg.link_targets.len());
    let mut degree = vec![0u32; n];
    for k in 0..e {
        let si = cfg.link_sources[k];
        let ti = cfg.link_targets[k];
        if si >= 0 && (si as usize) < n {
            degree[si as usize] += 1;
        }
        if ti >= 0 && (ti as usize) < n {
            degree[ti as usize] += 1;
        }
    }
    let degree_max = degree.iter().copied().max().unwrap_or(0).max(1) as f64;

    let mut ring_cursor = r;

    let degree_band = 14.0;
    let r_degree_in = ring_cursor - degree_band;
    ring_cursor = r_degree_in - 4.0;

    let age_band = 15.0;
    let r_age = ring_cursor - age_band / 2.0;
    ring_cursor -= age_band + 4.0;

    let r_bound_out = ring_cursor;
    let r_bound_in = ring_cursor - 12.0;
    ring_cursor = r_bound_in - 5.0;

    let n_bar = cfg.bar_series.len();
    let bar_band_total = 100.0;
    let per_bar_h = if n_bar > 0 { (bar_band_total - (n_bar as f64 - 1.0) * 5.0) / n_bar as f64 } else { 0.0 };
    let mut bar_rings: Vec<(f64, f64)> = Vec::with_capacity(n_bar);
    for _ in 0..n_bar {
        bar_rings.push((ring_cursor - per_bar_h, ring_cursor));
        ring_cursor -= per_bar_h + 5.0;
    }
    if n_bar > 0 {
        ring_cursor -= 3.0;
    }

    let has_ratio = cfg.bar_series.len() >= 2;
    let r_ratio_out = ring_cursor;
    let r_ratio_in = ring_cursor - 13.0;
    if has_ratio {
        ring_cursor = r_ratio_in - 3.0;
    }

    let has_rank = !plays0.is_empty();
    let r_rank_out = ring_cursor;
    let r_rank_in = ring_cursor - 13.0;
    if has_rank {
        ring_cursor = r_rank_in - 8.0;
    }

    let heat_band_total = 64.0_f64.min((ring_cursor - 90.0).max(0.0));
    let cat_h = if n_cat > 0 { heat_band_total / n_cat as f64 } else { 0.0 };
    let r_heat_out = ring_cursor;
    let r_heat_in = (r_heat_out - heat_band_total).max(70.0);
    ring_cursor = r_heat_in - 6.0;

    let has_comp = n_cat > 0 && !cfg.heat_matrix.is_empty();
    let comp_band = 16.0;
    let r_comp_out = ring_cursor;
    let r_comp_in = (ring_cursor - comp_band).max(44.0);
    let r_links = if has_comp { r_comp_in - 4.0 } else { r_heat_in - 4.0 }.max(40.0);

    let ink = "#1f2530";
    let sub_ink = "#6b7280";

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n * (6 + n_cat) + n_groups + e);

    let label_idx: Vec<i32> = (0..n)
        .map(|i| {
            let idx = slots.len() as i32;
            slots.push(HoverSlot::new(cfg.item_labels[i].clone()).kv("Cluster", cfg.item_groups.get(i).cloned().unwrap_or_default()));
            idx
        })
        .collect();

    let bound_idx: Vec<i32> = (0..n_groups)
        .map(|gi| {
            let idx = slots.len() as i32;
            slots.push(HoverSlot::new(group_order[gi].to_string()).kv("Artists in Cluster", group_count(gi).to_string()));
            idx
        })
        .collect();

    let age_idx: Vec<i32> = (0..n)
        .map(|i| {
            let idx = slots.len() as i32;
            let nearest = if n_cat > 0 { cfg.heat_categories.get(avg_age_idx[i].round() as usize).cloned().unwrap_or_default() } else { String::new() };
            slots.push(
                HoverSlot::new(cfg.item_labels[i].clone())
                    .kv("Skews Toward", nearest)
                    .kv("Age Spread (std dev)", format!("{:.2}", age_spread[i]))
                    .kv("Co-occurrence Links", degree[i].to_string()),
            );
            idx
        })
        .collect();

    let mut bar_idx: Vec<Vec<i32>> = Vec::with_capacity(n_bar);
    for (name, vals) in cfg.bar_series {
        let row: Vec<i32> = (0..n)
            .map(|i| {
                let idx = slots.len() as i32;
                slots.push(HoverSlot::new(cfg.item_labels[i].clone()).kv(name.clone(), format!("{:.1}", vals.get(i).copied().unwrap_or(0.0))));
                idx
            })
            .collect();
        bar_idx.push(row);
    }

    let ratio_idx: Vec<i32> = (0..n)
        .map(|i| {
            let idx = slots.len() as i32;
            slots.push(HoverSlot::new(cfg.item_labels[i].clone()).kv("Plays per Listener", format!("{:.2}", ratio[i])));
            idx
        })
        .collect();

    let rank_idx: Vec<i32> = (0..n)
        .map(|i| {
            let idx = slots.len() as i32;
            slots.push(HoverSlot::new(cfg.item_labels[i].clone()).kv("Play Rank", format!("#{} of {}", rank_pos[i] + 1, n)));
            idx
        })
        .collect();

    let mut heat_idx: Vec<Vec<i32>> = vec![Vec::with_capacity(n); n_cat];
    for (ci, row) in heat_idx.iter_mut().enumerate() {
        for i in 0..n {
            let idx = slots.len() as i32;
            let v = cfg.heat_matrix.get(i).and_then(|r| r.get(ci)).copied().unwrap_or(0.0);
            let total: f64 = cfg.heat_matrix.get(i).map(|r| r.iter().sum()).unwrap_or(0.0).max(1e-9);
            slots.push(
                HoverSlot::new(cfg.item_labels[i].clone())
                    .kv(cfg.heat_categories.get(ci).cloned().unwrap_or_default(), format!("{v:.1} plays"))
                    .kv("Share of Artist Total", format!("{:.0}%", v / total * 100.0)),
            );
            row.push(idx);
        }
    }

    let link_idx: Vec<i32> = (0..e)
        .map(|k| {
            let idx = slots.len() as i32;
            let si = cfg.link_sources.get(k).copied().unwrap_or(-1);
            let ti = cfg.link_targets.get(k).copied().unwrap_or(-1);
            let sname = if si >= 0 { cfg.item_labels.get(si as usize).cloned().unwrap_or_default() } else { String::new() };
            let tname = if ti >= 0 { cfg.item_labels.get(ti as usize).cloned().unwrap_or_default() } else { String::new() };
            slots.push(HoverSlot::new(format!("{sname} <-> {tname}")).kv("Type", "Co-occurrence"));
            idx
        })
        .collect();

    let mut b = Vec::<u8>::with_capacity(8192 + n * 800 + e * 160);
    svg_open(&mut b, cfg.width, cfg.height);

    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"32\" y=\"42\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"22\" font-weight=\"800\" fill=\"");
        push_b(&mut b, ink.as_bytes());
        push_b(&mut b, b"\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

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
        let (sx, sy) = (cx + r_links * sa.cos(), cy + r_links * sa.sin());
        let (tx, ty) = (cx + r_links * ta.cos(), cy + r_links * ta.sin());
        push_b(&mut b, b"<path data-idx=\"");
        push_i(&mut b, link_idx[k]);
        push_b(&mut b, b"\" d=\"M");
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

    if has_comp {
        for i in 0..n {
            let row = &cfg.heat_matrix[i];
            let total: f64 = row.iter().sum::<f64>().max(1e-9);
            let a0 = item_angle[i] - item_slot[i] / 2.0;
            let mut ac = a0;
            for (ci, &v) in row.iter().enumerate() {
                let seg = item_slot[i] * (v / total);
                let color = hex6(palette_color(cfg.palette, ci));
                push_b(&mut b, b"<path fill=\"#");
                b.extend_from_slice(&color);
                push_b(&mut b, b"\" data-idx=\"");
                push_i(&mut b, heat_idx[ci][i]);
                push_b(&mut b, b"\" d=\"");
                ring_wedge(&mut b, cx, cy, r_comp_in, r_comp_out, ac, ac + seg);
                push_b(&mut b, b"\"/>");
                ac += seg;
            }
        }
    }

    if n_cat > 0 && !cfg.heat_matrix.is_empty() {
        let heat_max = cfg.heat_matrix.iter().flat_map(|row| row.iter().copied()).fold(0.0_f64, f64::max).max(1e-9);
        for ci in 0..n_cat {
            let ring_out = r_heat_out - ci as f64 * cat_h;
            let ring_in = ring_out - cat_h;
            for i in 0..n {
                let v = cfg.heat_matrix.get(i).and_then(|row| row.get(ci)).copied().unwrap_or(0.0);
                let t = (v / heat_max).clamp(0.0, 1.0);
                let color = ramp(&WARM, t);
                let a1 = item_angle[i] - item_slot[i] / 2.0;
                let a2 = item_angle[i] + item_slot[i] / 2.0;
                push_b(&mut b, b"<path fill=\"");
                push_b(&mut b, color.as_bytes());
                push_b(&mut b, b"\" data-idx=\"");
                push_i(&mut b, heat_idx[ci][i]);
                push_b(&mut b, b"\" d=\"");
                ring_wedge(&mut b, cx, cy, ring_in, ring_out, a1, a2);
                push_b(&mut b, b"\"/>");
            }
        }
    }

    if has_rank {
        for i in 0..n {
            let color = ramp(&MONO, rank_t[i]);
            let a1 = item_angle[i] - item_slot[i] / 2.0;
            let a2 = item_angle[i] + item_slot[i] / 2.0;
            push_b(&mut b, b"<path fill=\"");
            push_b(&mut b, color.as_bytes());
            push_b(&mut b, b"\" data-idx=\"");
            push_i(&mut b, rank_idx[i]);
            push_b(&mut b, b"\" d=\"");
            ring_wedge(&mut b, cx, cy, r_rank_in, r_rank_out, a1, a2);
            push_b(&mut b, b"\"/>");
        }
    }

    if has_ratio {
        for i in 0..n {
            let color = ramp(&COOL, ratio_t[i]);
            let a1 = item_angle[i] - item_slot[i] / 2.0;
            let a2 = item_angle[i] + item_slot[i] / 2.0;
            push_b(&mut b, b"<path fill=\"");
            push_b(&mut b, color.as_bytes());
            push_b(&mut b, b"\" data-idx=\"");
            push_i(&mut b, ratio_idx[i]);
            push_b(&mut b, b"\" d=\"");
            ring_wedge(&mut b, cx, cy, r_ratio_in, r_ratio_out, a1, a2);
            push_b(&mut b, b"\"/>");
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
            push_i(&mut b, bar_idx[bi][i]);
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
        push_b(&mut b, b"\" data-idx=\"");
        push_i(&mut b, bound_idx[gi]);
        push_b(&mut b, b"\" d=\"");
        ring_wedge(&mut b, cx, cy, r_bound_in, r_bound_out, a1, a2);
        push_b(&mut b, b"\"/>");
    }

    for i in 0..n {
        let a = item_angle[i];
        let a1 = item_angle[i] - item_slot[i] / 2.0;
        let a2 = item_angle[i] + item_slot[i] / 2.0;
        let deg_frac = degree[i] as f64 / degree_max;
        let deg_out = r_degree_in + degree_band * deg_frac;
        push_b(&mut b, b"<path fill=\"#1f2530\" fill-opacity=\"0.75\" data-idx=\"");
        push_i(&mut b, age_idx[i]);
        push_b(&mut b, b"\" d=\"");
        ring_wedge(&mut b, cx, cy, r_degree_in, deg_out, a1, a2);
        push_b(&mut b, b"\"/>");

        let spread_px = (age_spread[i] / (n_cat.max(1) as f64)) * age_band;
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, cx + (r_age - spread_px) * a.cos());
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, cy + (r_age - spread_px) * a.sin());
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, cx + (r_age + spread_px) * a.cos());
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, cy + (r_age + spread_px) * a.sin());
        push_b(&mut b, b"\" stroke=\"#94a3b8\" stroke-width=\"1.2\" data-idx=\"");
        push_i(&mut b, age_idx[i]);
        push_b(&mut b, b"\"/>");

        let color = ramp(&WARM, avg_age_t[i]);
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, cx + r_age * a.cos());
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, cy + r_age * a.sin());
        push_b(&mut b, b"\" r=\"3.4\" fill=\"");
        push_b(&mut b, color.as_bytes());
        push_b(&mut b, b"\" stroke=\"#ffffff\" stroke-width=\"0.8\" data-idx=\"");
        push_i(&mut b, age_idx[i]);
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
        push_i(&mut b, label_idx[i]);
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &cfg.item_labels[i]);
        push_b(&mut b, b"</text>");
    }

    let lx0 = cx + r + 190.0;
    let mut ly0 = cy - r + 6.0;
    let line_h = 22.0;

    let legend_text = |b: &mut Vec<u8>, ly: f64, text: &str| {
        push_b(b, b"<text x=\"");
        push_f2(b, lx0);
        push_b(b, b"\" y=\"");
        push_f2(b, ly);
        push_b(b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10.5\" font-weight=\"800\" fill=\"");
        push_b(b, ink.as_bytes());
        push_b(b, b"\">");
        escape_xml(b, text);
        push_b(b, b"</text>");
    };

    legend_text(&mut b, ly0, "ITEM NAME");
    ly0 += line_h;

    push_b(&mut b, b"<rect x=\"");
    push_f2(&mut b, lx0 - 16.0);
    push_b(&mut b, b"\" y=\"");
    push_f2(&mut b, ly0 - 9.0);
    push_b(&mut b, b"\" width=\"10\" height=\"10\" fill=\"#1f2530\" fill-opacity=\"0.75\"/>");
    legend_text(&mut b, ly0, "CO-OCCURRENCE DEGREE");
    ly0 += line_h;
    legend_text(&mut b, ly0, "AVERAGE AGE INDEX + SPREAD");
    ly0 += line_h;
    legend_text(&mut b, ly0, "CLUSTER BOUNDARIES");
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
        legend_text(&mut b, ly0, &name.to_uppercase());
        ly0 += line_h;
    }

    if has_ratio {
        legend_text(&mut b, ly0, "PLAYS PER LISTENER");
        ly0 += line_h;
    }
    if has_rank {
        legend_text(&mut b, ly0, "PLAY RANK");
        ly0 += line_h;
    }

    if n_cat > 0 {
        legend_text(&mut b, ly0, "AGE GROUP INTENSITY");
        ly0 += 18.0;
        for k in 0..10 {
            let t = k as f64 / 9.0;
            push_b(&mut b, b"<rect x=\"");
            push_f2(&mut b, lx0 - 16.0 + k as f64 * 12.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, ly0 - 9.0);
            push_b(&mut b, b"\" width=\"11\" height=\"11\" fill=\"");
            push_b(&mut b, ramp(&WARM, t).as_bytes());
            push_b(&mut b, b"\"/>");
        }
        ly0 += line_h;
    }

    if has_comp {
        legend_text(&mut b, ly0, "AGE GROUP COMPOSITION");
        ly0 += 18.0;
        for (ci, cat) in cfg.heat_categories.iter().enumerate() {
            let color = hex6(palette_color(cfg.palette, ci));
            push_b(&mut b, b"<rect x=\"");
            push_f2(&mut b, lx0 - 16.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, ly0 - 9.0);
            push_b(&mut b, b"\" width=\"9\" height=\"9\" fill=\"#");
            b.extend_from_slice(&color);
            push_b(&mut b, b"\"/>");
            push_b(&mut b, b"<text x=\"");
            push_f2(&mut b, lx0 + 6.0);
            push_b(&mut b, b"\" y=\"");
            push_f2(&mut b, ly0);
            push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"");
            push_b(&mut b, sub_ink.as_bytes());
            push_b(&mut b, b"\">");
            escape_xml(&mut b, cat);
            push_b(&mut b, b"</text>");
            ly0 += 15.0;
        }
        ly0 += line_h - 15.0;
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
        legend_text(&mut b, ly0, "CO-OCCURRENCE");
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

type Stop = (u8, u8, u8);
const WARM: [Stop; 4] = [(0xff, 0xf6, 0xd6), (0xfb, 0x9a, 0x4b), (0xc9, 0x27, 0x37), (0x5c, 0x0a, 0x2e)];
const COOL: [Stop; 4] = [(0xe8, 0xf1, 0xfb), (0x7f, 0xb8, 0xe6), (0x3a, 0x6f, 0xb5), (0x1b, 0x2a, 0x5e)];
const MONO: [Stop; 4] = [(0xf1, 0xf2, 0xf4), (0xb9, 0xbf, 0xc7), (0x6b, 0x74, 0x80), (0x1a, 0x1d, 0x22)];

fn ramp(stops: &[Stop; 4], t: f64) -> String {
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
            width: 1400,
            height: 1300,
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
        assert_eq!(link_block.matches("<path data-idx=").count(), n_links);
    }

    #[test]
    fn renders_one_age_dot_per_item() {
        let (l, g, bs, hc, hm, ls, lt) = synth(12);
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        assert_eq!(html.matches("stroke=\"#ffffff\" stroke-width=\"0.8\" data-idx=\"").count(), l.len());
    }

    #[test]
    fn every_data_idx_points_at_a_real_slot_in_the_hover_json() {
        let (l, g, bs, hc, hm, ls, lt) = synth(12);
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        let json_start = html.find("var data=").map(|p| p + "var data=".len()).unwrap();
        let json_tail = &html[json_start..];
        let end = json_tail.find(";\n\nvar dpts=").unwrap();
        let raw = &json_tail[..end];
        let parsed: serde_json::Value = serde_json::from_str(raw).expect("hover json must parse");
        let slot_count = parsed.as_array().expect("hover json must be an array").len();

        let svg_only = html.split("</svg>").next().unwrap();
        let mut max_idx = -1_i64;
        for chunk in svg_only.split("data-idx=\"").skip(1) {
            let idx_str = chunk.split('"').next().unwrap();
            let idx: i64 = idx_str.parse().expect("every data-idx attribute in the svg must be a plain integer");
            max_idx = max_idx.max(idx);
            assert!(idx >= 0 && (idx as usize) < slot_count, "data-idx {idx} has no matching hover slot (slot_count={slot_count})");
        }
        assert!(max_idx >= 0);
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
    fn every_ramp_stays_within_valid_hex_bounds_across_the_full_range() {
        for stops in [&WARM, &COOL, &MONO] {
            for i in 0..=20 {
                let t = i as f64 / 20.0;
                let c = ramp(stops, t);
                assert_eq!(c.len(), 7);
                assert!(c.starts_with('#'));
            }
        }
    }

    #[test]
    fn perf_rendering_a_large_circos_stays_fast() {
        let (l, g, bs, hc, hm, ls, lt) = synth(300);
        let start = std::time::Instant::now();
        let html = render(&cfg(&l, &g, &bs, &hc, &hm, &ls, &lt));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 600, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
