use super::common::{bin_to_edges, compute_bins, group_indices};
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_legend_item, Frame};

#[crate::chart_demo("values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]")]

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() { return String::new(); }
    let (_, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = edges.len().saturating_sub(1);
    if n_bins == 0 { return String::new(); }

    let mut series: Vec<(String, Vec<u64>, u32)> = Vec::new();

    if !cfg.categories.is_empty() {
        let n = cfg.values.len();
        let (groups, idx) = group_indices(cfg.categories, n);
        let n_groups = groups.len().max(1);
        let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
        for i in 0..n {
            let gi = if i < idx.len() { idx[i] } else { 0 };
            buckets[gi.min(n_groups - 1)].push(cfg.values[i]);
        }
        for (gi, name) in groups.iter().enumerate() {
            let counts = bin_to_edges(&buckets[gi], &edges).0;
            series.push((name.clone(), counts, palette_color(cfg.palette, gi)));
        }
    } else if let Some(ov) = cfg.overlay_values {
        let a = bin_to_edges(cfg.values, &edges).0;
        let b = bin_to_edges(ov, &edges).0;
        let (n0, n1) = match cfg.series_names {
            Some((a, b)) => (a.to_string(), b.to_string()),
            None => ("A".to_string(), "B".to_string()),
        };
        series.push((n0, a, cfg.color));
        series.push((n1, b, cfg.overlay_color));
    } else {
        let a = bin_to_edges(cfg.values, &edges).0;
        let name = cfg.series_names.map(|(a, _)| a.to_string()).unwrap_or_default();
        series.push((name, a, cfg.color));
    }

    let max_count = series.iter().flat_map(|s| s.1.iter().copied()).max().unwrap_or(1).max(1) as f64;
    let n_series = series.len();
    let has_legend = n_series > 1 || cfg.series_names.is_some();
    let legend_w = if has_legend { 140 } else { 20 };
    let pad_l: i32 = 56;
    let pad_r: i32 = legend_w;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, pad_l, 40, 50, pad_r, n_bins * n_series * 360 + 2048);
    f.open(cfg.title, false);
    f.y_grid_int(5, max_count, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let opacity = if n_series >= 3 { "0.45" } else { "0.55" };

    for (si, (name, counts, color)) in series.iter().enumerate() {
        let hx = hex6(*color);
        for (i, &cnt) in counts.iter().enumerate() {
            if cnt == 0 { continue; }
            let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
            let x = f.pl + (i as f64 * bw_px) as i32;
            let y = f.pt + f.ph - bh;
            let w_px = (bw_px as i32 - cfg.gap.max(0)).max(1);
            push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, (si * n_bins + i) as i32);
            push_b(&mut f.buf, b"\" data-series=\""); push_i(&mut f.buf, si as i32);
            push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, name);
            push_b(&mut f.buf, b"\" data-kv-Range=\""); push_f2(&mut f.buf, edges[i]); f.buf.extend_from_slice("\u{2013}".as_bytes()); push_f2(&mut f.buf, edges.get(i+1).copied().unwrap_or(edges[i]));
            push_b(&mut f.buf, b"\" data-kv-Count=\""); push_i(&mut f.buf, cnt as i32);
            push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, w_px);
            push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, bh);
            push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\""); f.buf.extend_from_slice(opacity.as_bytes());
            push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" stroke-opacity=\"0.9\" stroke-width=\"1\"/>");
        }
    }

    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = f.pl + (i as f64 * bw_px) as i32;
        let val = if i < edges.len() { edges[i] } else { *edges.last().unwrap_or(&0.0) };
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }

    if has_legend {
        let lx = cfg.width - legend_w + 10;
        for (si, (name, _, color)) in series.iter().enumerate() {
            svg_legend_item(&mut f.buf, si as i32, name, *color, lx, f.pt + 6 + (si as i32) * 22, 18);
        }
    }

    let slots_json;
    let json: &str = if cfg.hover.is_empty() { "[]" } else { slots_json = slots_to_json(cfg.hover); &slots_json };
    f.html(json)
}

