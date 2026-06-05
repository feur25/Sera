use super::common::{bin_to_edges, compute_bins, group_indices};
use super::config::HistogramConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, svg_legend_item, Frame,
};

#[crate::chart_demo("values=[2.1,2.3,2.7,3.1,3.4,3.6,3.9,4.0,4.2,4.5,4.6,4.8,5.0,5.3,5.7,6.1,6.3,6.5,6.8,7.0,3.2,4.1,5.2,4.7,3.8,4.4,5.1,4.9,5.5,6.2]")]

pub fn render(cfg: &HistogramConfig) -> String {
    if cfg.values.is_empty() {
        return String::new();
    }
    let n = cfg.values.len();
    let (groups, idx) = group_indices(cfg.categories, n);
    let n_groups = groups.len().max(1);
    let (_, edges) = compute_bins(cfg.values, cfg.bins);
    let n_bins = edges.len().saturating_sub(1);
    if n_bins == 0 {
        return String::new();
    }

    let mut series: Vec<Vec<u64>> = vec![vec![0u64; n_bins]; n_groups];
    let min = edges[0];
    let max = *edges.last().unwrap_or(&min);
    let range = (max - min).max(1e-12);
    let step = range / n_bins as f64;
    for i in 0..n {
        let v = cfg.values[i];
        if !v.is_finite() {
            continue;
        }
        let bi = (((v - min) / step) as usize).min(n_bins - 1);
        let gi = if i < idx.len() { idx[i] } else { 0 };
        series[gi.min(n_groups - 1)][bi] += 1;
    }

    let mut totals: Vec<u64> = vec![0u64; n_bins];
    for g in 0..n_groups {
        for b in 0..n_bins {
            totals[b] += series[g][b];
        }
    }
    let max_count = (*totals.iter().max().unwrap_or(&1)) as f64;

    let has_legend = !groups.is_empty();
    let legend_w = if has_legend { 130 } else { 20 };
    let pad_l: i32 = 52;
    let pad_r: i32 = legend_w;
    let plot_w = cfg.width - pad_l - pad_r;
    let bw_px = plot_w as f64 / n_bins as f64;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        pad_l,
        36,
        46,
        pad_r,
        n_bins * 320 + 2048,
    );
    f.open(cfg.title, false);
    f.y_grid_int(5, max_count, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    for b in 0..n_bins {
        let x = f.pl + (b as f64 * bw_px) as i32;
        let w_px = (bw_px as i32 - cfg.gap.max(0)).max(1);
        let mut acc_h: i32 = 0;
        for g in 0..n_groups {
            let cnt = series[g][b];
            if cnt == 0 {
                continue;
            }
            let bh = (cnt as f64 / max_count * f.ph as f64) as i32;
            let y = f.pt + f.ph - acc_h - bh;
            let col = palette_color(cfg.palette, g);
            let hx = hex6(col);
            push_b(&mut f.buf, b"<rect data-idx=\"");
            push_i(&mut f.buf, b as i32);
            push_b(&mut f.buf, b"\" data-series=\"");
            push_i(&mut f.buf, g as i32);
            push_b(&mut f.buf, b"\" data-lbl=\"");
            escape_xml(&mut f.buf, &groups[g]);
            push_b(&mut f.buf, b"\" data-kv-Range=\"");
            push_f2(&mut f.buf, edges[b]);
            f.buf.extend_from_slice("\u{2013}".as_bytes());
            push_f2(&mut f.buf, edges.get(b + 1).copied().unwrap_or(edges[b]));
            push_b(&mut f.buf, b"\" data-kv-Count=\"");
            push_i(&mut f.buf, cnt as i32);
            push_b(&mut f.buf, b"\" x=\"");
            push_i(&mut f.buf, x);
            push_b(&mut f.buf, b"\" y=\"");
            push_i(&mut f.buf, y);
            push_b(&mut f.buf, b"\" width=\"");
            push_i(&mut f.buf, w_px);
            push_b(&mut f.buf, b"\" height=\"");
            push_i(&mut f.buf, bh);
            push_b(&mut f.buf, b"\" fill=\"#");
            f.buf.extend_from_slice(&hx);
            push_b(
                &mut f.buf,
                b"\" fill-opacity=\"0.85\" stroke=\"#fff\" stroke-width=\"0.4\"/>",
            );
            acc_h += bh;
        }
    }

    let tick_step = ((n_bins as f64 / 8.0).ceil() as usize).max(1);
    for i in (0..=n_bins).step_by(tick_step) {
        let x = f.pl + (i as f64 * bw_px) as i32;
        let val = if i < edges.len() {
            edges[i]
        } else {
            *edges.last().unwrap_or(&0.0)
        };
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(&mut f.buf, val);
        push_b(&mut f.buf, b"</text>");
    }

    if has_legend {
        let lx = cfg.width - legend_w + 10;
        for (gi, name) in groups.iter().enumerate() {
            let col = palette_color(cfg.palette, gi);
            svg_legend_item(
                &mut f.buf,
                gi as i32,
                name,
                col,
                lx,
                f.pt + 4 + (gi as i32) * 20,
                18,
            );
        }
    }
    let _ = bin_to_edges;

    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
