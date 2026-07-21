use super::common::{fmt_val, mbar, BAR_CSS};
use super::config::PlotWebConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_i};

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

    let w = cfg.width;
    let h = cfg.height;
    let pad_l = 90i32;
    let pad_r = 90i32;
    let pad_t = 100i32;
    let pad_b = 110i32;
    let plot_w = w - pad_l - pad_r;
    let plot_h = h - pad_t - pad_b;
    let ax_base = pad_t + plot_h;

    let x_log: Vec<f64> = if cfg.x_log {
        cfg.x_values[..n].iter().map(|&v| v.max(1e-9_f64).log10()).collect()
    } else {
        cfg.x_values[..n].to_vec()
    };

    let x_min = x_log.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = x_log.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = cfg.y_values[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = cfg.y_values[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let has_sizes = !cfg.sizes.is_empty();
    let s_min = if has_sizes {
        cfg.sizes[..n.min(cfg.sizes.len())].iter().cloned().fold(f64::INFINITY, f64::min)
    } else { 0.0 };
    let s_max = if has_sizes {
        cfg.sizes[..n.min(cfg.sizes.len())].iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    } else { 1.0 };

    let norm_x = |i: usize| -> f64 {
        if x_max > x_min { (x_log[i] - x_min) / (x_max - x_min) } else { 0.5 }
    };
    let norm_y = |v: f64| -> f64 {
        if y_max > y_min { (v - y_min) / (y_max - y_min) } else { 0.5 }
    };
    let norm_s = |i: usize| -> f64 {
        if has_sizes && s_max > s_min && i < cfg.sizes.len() {
            (cfg.sizes[i] - s_min) / (s_max - s_min)
        } else { 0.5 }
    };

    let px = |nx: f64| -> i32 { pad_l + (nx * plot_w as f64) as i32 };
    let py = |ny: f64| -> i32 { pad_t + ((1.0 - ny) * plot_h as f64) as i32 };
    let node_r = |i: usize| -> i32 { (cfg.min_r + norm_s(i) * (cfg.max_r - cfg.min_r)) as i32 };

    let mut group_list: Vec<&str> = Vec::new();
    for g in cfg.groups[..n].iter() {
        if !group_list.contains(&g.as_str()) {
            group_list.push(g.as_str());
        }
    }

    let group_color = |g: &str| -> u32 {
        let idx = group_list.iter().position(|&x| x == g).unwrap_or(0);
        palette_color(cfg.palette, idx)
    };

    let mut xv_sorted = cfg.x_values[..n].to_vec();
    xv_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let med_x = xv_sorted[n / 2];

    let x_lbl = if cfg.x_label.is_empty() { "X" } else { cfg.x_label };
    let y_lbl = if cfg.y_label.is_empty() { "Y" } else { cfg.y_label };
    let s_lbl = if cfg.size_label.is_empty() { "Taille" } else { cfg.size_label };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for i in 0..n {
        let col = group_color(cfg.groups[i].as_str());
        let hx = hex6(col);
        let col_str = format!("#{}", std::str::from_utf8(&hx).unwrap_or("636efa"));
        let al = cfg.groups[i].replace('_', " ");
        let ratio = if med_x > 0.0 { cfg.x_values[i] / med_x } else { 1.0 };
        let (rtxt, rcolor) = if ratio >= 2.0 {
            (format!("x{:.1} vs mediane", ratio), "#e11d48")
        } else if ratio >= 1.0 {
            (format!("x{:.1} vs mediane", ratio), "#d97706")
        } else {
            (format!("{:.1}x sous mediane", 1.0 / ratio.max(1e-9)), "#059669")
        };
        let s_bar = if has_sizes {
            mbar(s_lbl, norm_s(i), "#0891b2", &fmt_val(cfg.sizes[i]))
        } else {
            String::new()
        };
        let html = format!(
            "<div class=\"pw-ap\" style=\"background:{cs}22;color:{cs};border:1px solid {cs}44\">{al}</div>\
             {bx}{by}{bs}<div class=\"pw-rt\" style=\"color:{rc};border-color:{rc}22\">{rt}</div>",
            cs = col_str,
            al = al,
            bx = mbar(x_lbl, norm_x(i), "#e11d48", &fmt_val(cfg.x_values[i])),
            by = mbar(y_lbl, norm_y(cfg.y_values[i]), "#6366f1", &fmt_val(cfg.y_values[i])),
            bs = s_bar,
            rc = rcolor,
            rt = rtxt,
        );
        slots.push(HoverSlot::new(&cfg.labels[i]).html(html));
    }

    let mut buf = Vec::<u8>::with_capacity(n * 512 + 8192);

    push_b(&mut buf, b"<svg id=\"pw\" xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, w);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    for &(za, zb, col, zlbl) in &[
        (0.0f64, 0.25f64, 0x0891b2u32, "OPTIMAL"),
        (0.25, 0.60, 0xd97706u32, ""),
        (0.60, 1.00, 0xe11d48u32, "CRITIQUE"),
    ] {
        let x1 = px(za);
        let x2 = px(zb);
        let hx = hex6(col);
        push_b(&mut buf, b"<rect x=\"");
        push_i(&mut buf, x1);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, pad_t);
        push_b(&mut buf, b"\" width=\"");
        push_i(&mut buf, x2 - x1);
        push_b(&mut buf, b"\" height=\"");
        push_i(&mut buf, plot_h);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"08\"/>");
        if !zlbl.is_empty() {
            let cx = (x1 + x2) / 2;
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, cx);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, pad_t - 14);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" font-weight=\"700\" fill=\"#");
            buf.extend_from_slice(&hx);
            push_b(&mut buf, b"55\" letter-spacing=\"2\">");
            push_b(&mut buf, zlbl.as_bytes());
            push_b(&mut buf, b"</text>");
        }
    }

    for xi in 0i32..6 {
        let gx = px(xi as f64 / 5.0);
        push_b(&mut buf, b"<line x1=\"");
        push_i(&mut buf, gx);
        push_b(&mut buf, b"\" y1=\"");
        push_i(&mut buf, pad_t - 8);
        push_b(&mut buf, b"\" x2=\"");
        push_i(&mut buf, gx);
        push_b(&mut buf, b"\" y2=\"");
        push_i(&mut buf, ax_base + 4);
        push_b(&mut buf, b"\" stroke=\"#0f172a\" stroke-width=\"1\"/>");
    }

    let mut group_map: std::collections::HashMap<&str, Vec<usize>> =
        std::collections::HashMap::new();
    for i in 0..n {
        group_map.entry(cfg.groups[i].as_str()).or_default().push(i);
    }

    for g in &group_list {
        if let Some(idxs) = group_map.get(g) {
            if idxs.len() < 2 { continue; }
            let col = group_color(g);
            let hx = hex6(col);
            let mut sorted = idxs.clone();
            sorted.sort_by(|&a, &b| {
                cfg.x_values[a].partial_cmp(&cfg.x_values[b]).unwrap_or(std::cmp::Ordering::Equal)
            });
            for pair in sorted.windows(2) {
                let (ai, bi) = (pair[0], pair[1]);
                let ax = px(norm_x(ai));
                let ay = py(norm_y(cfg.y_values[ai]));
                let bx = px(norm_x(bi));
                let by = py(norm_y(cfg.y_values[bi]));
                let mx = (ax + bx) / 2;
                push_b(&mut buf, b"<path d=\"M ");
                push_i(&mut buf, ax);
                push_b(&mut buf, b",");
                push_i(&mut buf, ay);
                push_b(&mut buf, b" C ");
                push_i(&mut buf, mx);
                push_b(&mut buf, b",");
                push_i(&mut buf, ay - 40);
                push_b(&mut buf, b" ");
                push_i(&mut buf, mx);
                push_b(&mut buf, b",");
                push_i(&mut buf, by - 40);
                push_b(&mut buf, b" ");
                push_i(&mut buf, bx);
                push_b(&mut buf, b",");
                push_i(&mut buf, by);
                push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"30\" stroke-width=\"1.5\"/>");
            }
        }
    }

    for i in 0..n {
        let xi = px(norm_x(i));
        let yi = py(norm_y(cfg.y_values[i]));
        let ri = node_r(i);
        let col = group_color(cfg.groups[i].as_str());
        let hx = hex6(col);

        if has_sizes && i < cfg.sizes.len() {
            let bh = (norm_s(i) * 28.0) as i32;
            if bh > 0 {
                push_b(&mut buf, b"<rect x=\"");
                push_i(&mut buf, xi - 3);
                push_b(&mut buf, b"\" y=\"");
                push_i(&mut buf, yi - ri - 6 - bh);
                push_b(&mut buf, b"\" width=\"6\" height=\"");
                push_i(&mut buf, bh);
                push_b(&mut buf, b"\" fill=\"#0891b255\" rx=\"2\"/>");
            }
        }

        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, xi);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, yi);
        push_b(&mut buf, b"\" r=\"");
        push_i(&mut buf, ri + 6);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"18\" stroke-width=\"6\"/>");

        push_b(&mut buf, b"<circle data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" cx=\"");
        push_i(&mut buf, xi);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, yi);
        push_b(&mut buf, b"\" r=\"");
        push_i(&mut buf, ri);
        push_b(&mut buf, b"\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"cc\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.5\"/>");
    }

    push_b(&mut buf, b"<line x1=\"");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\"");
    push_i(&mut buf, ax_base + 18);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, pad_l + plot_w);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, ax_base + 18);
    push_b(&mut buf, b"\" stroke=\"#0f172a\" stroke-width=\"1\"/>");

    if cfg.x_log {
        let d_min = x_min.floor() as i32;
        let d_max = x_max.ceil() as i32;
        for d in d_min..=d_max {
            let lv = d as f64;
            let t = if x_max > x_min { (lv - x_min) / (x_max - x_min) } else { 0.5 };
            if t < -0.01 || t > 1.01 { continue; }
            let tx = px(t.clamp(0.0, 1.0));
            let actual = 10.0f64.powi(d);
            let label = if d >= 3 { format!("{:.0}k", actual / 1000.0) }
                        else if d >= 0 { format!("{:.0}", actual) }
                        else if d >= -1 { format!("{:.1}", actual) }
                        else { format!("{:.0e}", actual) };
            push_b(&mut buf, b"<line x1=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y1=\"");
            push_i(&mut buf, ax_base + 14);
            push_b(&mut buf, b"\" x2=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y2=\"");
            push_i(&mut buf, ax_base + 22);
            push_b(&mut buf, b"\" stroke=\"#1e293b\" stroke-width=\"1\"/>");
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, ax_base + 34);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" fill=\"#334155\">");
            escape_xml(&mut buf, &label);
            push_b(&mut buf, b"</text>");
        }
    } else {
        for ti in 0..=5i32 {
            let t = ti as f64 / 5.0;
            let tx = px(t);
            let val = x_min + t * (x_max - x_min);
            push_b(&mut buf, b"<line x1=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y1=\"");
            push_i(&mut buf, ax_base + 14);
            push_b(&mut buf, b"\" x2=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y2=\"");
            push_i(&mut buf, ax_base + 22);
            push_b(&mut buf, b"\" stroke=\"#1e293b\" stroke-width=\"1\"/>");
            push_b(&mut buf, b"<text x=\"");
            push_i(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_i(&mut buf, ax_base + 34);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" fill=\"#334155\">");
            escape_xml(&mut buf, &fmt_val(val));
            push_b(&mut buf, b"</text>");
        }
    }

    for ti in 0..=4i32 {
        let t = ti as f64 / 4.0;
        let qy = py(t);
        let val = y_min + t * (y_max - y_min);
        push_b(&mut buf, b"<line x1=\"");
        push_i(&mut buf, pad_l - 6);
        push_b(&mut buf, b"\" y1=\"");
        push_i(&mut buf, qy);
        push_b(&mut buf, b"\" x2=\"");
        push_i(&mut buf, pad_l - 2);
        push_b(&mut buf, b"\" y2=\"");
        push_i(&mut buf, qy);
        push_b(&mut buf, b"\" stroke=\"#1e293b\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l - 14);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, qy + 3);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" fill=\"#334155\">");
        escape_xml(&mut buf, &fmt_val(val));
        push_b(&mut buf, b"</text>");
    }

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
          font-size=\"10\" font-weight=\"700\" fill=\"#1a2744\" letter-spacing=\"4\" class=\"sp-ttl\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    if !cfg.x_label.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_l + plot_w / 2);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, ax_base + 46);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
          font-size=\"7.5\" fill=\"#1e293b\" class=\"sp-xl\">");
        escape_xml(&mut buf, cfg.x_label);
        push_b(&mut buf, b"</text>");
    }

    if !cfg.y_label.is_empty() {
        let ym = pad_t + plot_h / 2;
        push_b(&mut buf, b"<text x=\"18\" y=\"");
        push_i(&mut buf, ym);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"system-ui,sans-serif\" \
          font-size=\"7.5\" fill=\"#1e293b\" transform=\"rotate(-90,18,");
        push_i(&mut buf, ym);
        push_b(&mut buf, b")\" class=\"sp-yl\">");
        escape_xml(&mut buf, cfg.y_label);
        push_b(&mut buf, b"</text>");
    }

    let item_w = 140i32;
    let leg_total = group_list.len() as i32 * item_w;
    let leg_x0 = (w - leg_total) / 2;
    let leg_y = ax_base + 64;
    for (ai, &g) in group_list.iter().enumerate() {
        let col = palette_color(cfg.palette, ai);
        let hx = hex6(col);
        let lx = leg_x0 + ai as i32 * item_w;
        push_b(&mut buf, b"<circle cx=\"");
        push_i(&mut buf, lx + 5);
        push_b(&mut buf, b"\" cy=\"");
        push_i(&mut buf, leg_y);
        push_b(&mut buf, b"\" r=\"4\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"cc\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, lx + 14);
        push_b(&mut buf, b"\" y=\"");
        push_i(&mut buf, leg_y + 4);
        push_b(&mut buf, b"\" font-family=\"system-ui,sans-serif\" font-size=\"7.5\" fill=\"#475569\">");
        escape_xml(&mut buf, &g.replace('_', " "));
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"</svg>");

    let body = unsafe { String::from_utf8_unchecked(buf) } + BAR_CSS;
    build_chart_html(cfg.title, &body, &slots_to_json(&slots))
}
