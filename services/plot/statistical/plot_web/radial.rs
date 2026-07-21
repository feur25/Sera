use super::common::{fmt_val, mbar, BAR_CSS};
use super::config::PlotWebConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};
use std::f64::consts::PI;

#[crate::chart_demo(
    "x_values=[12,25,40,55,70,85], y_values=[18,42,28,55,38,72], labels=[\"A\",\"B\",\"C\",\"D\",\"E\",\"F\"], groups=[\"G1\",\"G1\",\"G2\",\"G2\",\"G3\",\"G3\"]"
)]
pub fn render(cfg: &PlotWebConfig) -> String {
    let n = [
        cfg.x_values.len(),
        cfg.y_values.len(),
        cfg.labels.len(),
        cfg.groups.len(),
    ]
    .iter()
    .copied()
    .min()
    .unwrap_or(0);
    if n == 0 {
        return String::new();
    }

    let w = cfg.width as f64;
    let h = cfg.height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0 + 10.0;
    let inner_r = 44.0f64;
    let outer_r = (w.min(h) / 2.0 - 60.0).max(80.0);

    let mut group_list: Vec<&str> = Vec::new();
    for g in cfg.groups[..n].iter() {
        if !group_list.contains(&g.as_str()) {
            group_list.push(g.as_str());
        }
    }
    let ng = group_list.len();
    if ng == 0 {
        return String::new();
    }

    let group_idx = |g: &str| -> usize {
        group_list.iter().position(|&x| x == g).unwrap_or(0)
    };
    let axis_angle = |gi: usize| -> f64 { 2.0 * PI * gi as f64 / ng as f64 - PI / 2.0 };
    let group_color = |g: &str| -> u32 { palette_color(cfg.palette, group_idx(g)) };

    let x_log: Vec<f64> = if cfg.x_log {
        cfg.x_values[..n].iter().map(|&v| v.max(1e-9_f64).log10()).collect()
    } else {
        cfg.x_values[..n].to_vec()
    };
    let x_min = x_log.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = x_log.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = cfg.y_values[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = cfg.y_values[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let has_sizes = cfg.sizes.len() >= n;
    let s_min = if has_sizes { cfg.sizes[..n].iter().cloned().fold(f64::INFINITY, f64::min) } else { 0.0 };
    let s_max = if has_sizes { cfg.sizes[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max) } else { 1.0 };

    let norm_x = |i: usize| -> f64 {
        if x_max > x_min { (x_log[i] - x_min) / (x_max - x_min) } else { 0.5 }
    };
    let norm_y = |v: f64| -> f64 {
        if y_max > y_min { (v - y_min) / (y_max - y_min) } else { 0.5 }
    };
    let norm_s = |i: usize| -> f64 {
        if has_sizes && s_max > s_min { (cfg.sizes[i] - s_min) / (s_max - s_min) } else { 0.5 }
    };

    let node_pos = |i: usize| -> (f64, f64) {
        let gi = group_idx(cfg.groups[i].as_str());
        let angle = axis_angle(gi);
        let r = inner_r + norm_x(i) * (outer_r - inner_r);
        (cx + r * angle.cos(), cy + r * angle.sin())
    };
    let node_r = |i: usize| -> f64 {
        cfg.min_r + norm_y(cfg.y_values[i]) * (cfg.max_r - cfg.min_r) / 2.0
    };

    let mut group_indices: Vec<Vec<usize>> = vec![Vec::new(); ng];
    for i in 0..n {
        let gi = group_idx(cfg.groups[i].as_str());
        group_indices[gi].push(i);
    }
    for gi in 0..ng {
        group_indices[gi].sort_by(|&a, &b| {
            cfg.x_values[a].partial_cmp(&cfg.x_values[b]).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    let y_thresh = y_min + (y_max - y_min) * 0.5;
    let top_nodes: Vec<Option<usize>> = (0..ng)
        .map(|gi| {
            group_indices[gi]
                .iter()
                .copied()
                .filter(|&i| cfg.y_values[i] >= y_thresh)
                .max_by(|&a, &b| {
                    cfg.y_values[a].partial_cmp(&cfg.y_values[b]).unwrap_or(std::cmp::Ordering::Equal)
                })
        })
        .collect();

    let x_lbl = if cfg.x_label.is_empty() { "X" } else { cfg.x_label };
    let y_lbl = if cfg.y_label.is_empty() { "Y" } else { cfg.y_label };
    let s_lbl = if cfg.size_label.is_empty() { "Taille" } else { cfg.size_label };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    let mut buf = Vec::<u8>::with_capacity(n * 512 + 8192);

    push_b(&mut buf, b"<svg id=\"pw\" xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    push_b(&mut buf, b"<defs>\
      <filter id=\"pw-glow\" x=\"-60%\" y=\"-60%\" width=\"220%\" height=\"220%\">\
      <feGaussianBlur stdDeviation=\"5\" result=\"blur\"/>\
      <feMerge><feMergeNode in=\"blur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>\
      </filter>\
      <filter id=\"pw-glow-sm\" x=\"-40%\" y=\"-40%\" width=\"180%\" height=\"180%\">\
      <feGaussianBlur stdDeviation=\"2.5\" result=\"blur\"/>\
      <feMerge><feMergeNode in=\"blur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>\
      </filter>\
      </defs>");

    for ti in 1..=4i32 {
        let r = inner_r + (ti as f64 / 4.0) * (outer_r - inner_r);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#1e293b\" stroke-width=\"0.5\" stroke-dasharray=\"3,5\"/>");
    }

    for gi in 0..ng {
        let angle = axis_angle(gi);
        let col = palette_color(cfg.palette, gi);
        let hx = hex6(col);
        let x1 = cx + inner_r * angle.cos();
        let y1 = cy + inner_r * angle.sin();
        let x2 = cx + outer_r * angle.cos();
        let y2 = cy + outer_r * angle.sin();
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x2);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y2);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"22\" stroke-width=\"1.5\"/>");
    }

    for i in 0..ng {
        if let Some(ni) = top_nodes[i] {
            let (ax2, ay2) = node_pos(ni);
            for j in (i + 1)..ng {
                if let Some(nj) = top_nodes[j] {
                    let (bx2, by2) = node_pos(nj);
                    let col_i = palette_color(cfg.palette, i);
                    let hxi = hex6(col_i);
                    let qx = cx + (ax2 - cx + bx2 - cx) * 0.18;
                    let qy = cy + (ay2 - cy + by2 - cy) * 0.18;
                    push_b(&mut buf, b"<path d=\"M ");
                    push_f2(&mut buf, ax2);
                    push_b(&mut buf, b",");
                    push_f2(&mut buf, ay2);
                    push_b(&mut buf, b" Q ");
                    push_f2(&mut buf, qx);
                    push_b(&mut buf, b",");
                    push_f2(&mut buf, qy);
                    push_b(&mut buf, b" ");
                    push_f2(&mut buf, bx2);
                    push_b(&mut buf, b",");
                    push_f2(&mut buf, by2);
                    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
                    buf.extend_from_slice(&hxi);
                    push_b(&mut buf, b"1c\" stroke-width=\"1\"/>");
                }
            }
        }
    }

    for gi in 0..ng {
        let angle = axis_angle(gi);
        let perp = angle + PI / 2.0;
        let col = palette_color(cfg.palette, gi);
        let hx = hex6(col);
        let idxs = &group_indices[gi];
        for pair in idxs.windows(2) {
            let (ai, bi) = (pair[0], pair[1]);
            let (ax2, ay2) = node_pos(ai);
            let (bx2, by2) = node_pos(bi);
            let mid_r = inner_r + (norm_x(ai) + norm_x(bi)) / 2.0 * (outer_r - inner_r);
            let cp_x = cx + mid_r * angle.cos() + 22.0 * perp.cos();
            let cp_y = cy + mid_r * angle.sin() + 22.0 * perp.sin();
            push_b(&mut buf, b"<path d=\"M ");
            push_f2(&mut buf, ax2);
            push_b(&mut buf, b",");
            push_f2(&mut buf, ay2);
            push_b(&mut buf, b" Q ");
            push_f2(&mut buf, cp_x);
            push_b(&mut buf, b",");
            push_f2(&mut buf, cp_y);
            push_b(&mut buf, b" ");
            push_f2(&mut buf, bx2);
            push_b(&mut buf, b",");
            push_f2(&mut buf, by2);
            push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"50\" stroke-width=\"1.5\"/>");
        }
    }

    for i in 0..n {
        let (xi, yi) = node_pos(i);
        let ri = node_r(i);
        let col = group_color(cfg.groups[i].as_str());
        let hx = hex6(col);
        let col_str = format!("#{}", std::str::from_utf8(&hx).unwrap_or("636efa"));
        let al = cfg.groups[i].replace('_', " ");

        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, xi);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, yi);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, ri + 10.0);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"12\" filter=\"url(#pw-glow)\" pointer-events=\"none\"/>");

        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_f2(&mut buf, xi);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, yi);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, ri);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"cc\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.5\" filter=\"url(#pw-glow-sm)\"/>");

        let s_bar = if has_sizes {
            mbar(s_lbl, norm_s(i), "#0891b2", &fmt_val(cfg.sizes[i]))
        } else {
            String::new()
        };
        let html = format!(
            "<div class=\"pw-ap\" style=\"background:{cs}22;color:{cs};border:1px solid {cs}44\">{al}</div>\
             {bx}{by}{bs}",
            cs = col_str,
            al = al,
            bx = mbar(x_lbl, norm_x(i), "#e11d48", &fmt_val(cfg.x_values[i])),
            by = mbar(y_lbl, norm_y(cfg.y_values[i]), "#6366f1", &fmt_val(cfg.y_values[i])),
            bs = s_bar,
        );
        slots.push(HoverSlot::new(&cfg.labels[i]).html(html));
    }

    for gi in 0..ng {
        let angle = axis_angle(gi);
        let col = palette_color(cfg.palette, gi);
        let hx = hex6(col);
        let lr = outer_r + 22.0;
        let lx = cx + lr * angle.cos();
        let ly = cy + lr * angle.sin() + 4.0;
        let anchor = if angle.cos() > 0.05 { "start" } else if angle.cos() < -0.05 { "end" } else { "middle" };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly);
        push_b(&mut buf, b"\" text-anchor=\"");
        push_b(&mut buf, anchor.as_bytes());
        push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"8.5\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"cc\">");
        escape_xml(&mut buf, &group_list[gi].replace('_', " "));
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"");
    push_f2(&mut buf, inner_r - 8.0);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#6366f118\" stroke-width=\"1\"/>");
    push_b(&mut buf, b"<circle cx=\"");
    push_f2(&mut buf, cx);
    push_b(&mut buf, b"\" cy=\"");
    push_f2(&mut buf, cy);
    push_b(&mut buf, b"\" r=\"3\" fill=\"#6366f133\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, w / 2.0);
        push_b(&mut buf, b"\" y=\"20\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
          font-size=\"11\" font-weight=\"700\" fill=\"#e2e8f0\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");

    let body = unsafe { String::from_utf8_unchecked(buf) } + BAR_CSS;
    build_chart_html(cfg.title, &body, &slots_to_json(&slots))
}
