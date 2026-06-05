use super::common::{color_for, data_attrs, dot, finalize, open, prepare, unique_groups};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, push_b, push_f2, push_i, svg_hgrid, svg_tick_y, svg_y_label,
};

#[crate::chart_demo(
    "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]"
)]

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let (mut b, pl, pt, pw, ph) = open(cfg, 64, 56, 26, 52);
    let max_val = p.vmax + (p.vmax - p.vmin).abs() * 0.08;
    let min_val = (p.vmin - (p.vmax - p.vmin).abs() * 0.08).min(0.0);
    let range = (max_val - min_val).max(1.0);
    let n_yticks: i32 = 5;
    if cfg.gridlines {
        for ti in 0..=n_yticks {
            let y = pt + ph - (ph as f64 * ti as f64 / n_yticks as f64) as i32;
            svg_hgrid(&mut b, pl, pl + pw, y);
        }
    }
    svg_y_label(&mut b, cfg.y_label, 12, pt, ph);
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let y = pt + ph - (ph as f64 * frac) as i32;
        svg_tick_y(&mut b, pl - 4, y + 4, min_val + frac * range);
    }

    let groups = unique_groups(&p);
    if groups.is_empty() {
        for i in 0..p.n {
            let step = pw as f64 / p.n as f64;
            let cx = pl + (step * 0.5 + step * i as f64) as i32;
            let y_v = pt + ph - (((p.values[i] - min_val) / range) * ph as f64) as i32;
            let col = color_for(cfg, &p, i);
            let hx = hex6(col);
            push_b(&mut b, b"<line");
            data_attrs(&mut b, &p, i);
            push_b(&mut b, b" x1=\"");
            push_i(&mut b, cx);
            push_b(&mut b, b"\" y1=\"");
            push_i(&mut b, pt + ph);
            push_b(&mut b, b"\" x2=\"");
            push_i(&mut b, cx);
            push_b(&mut b, b"\" y2=\"");
            push_i(&mut b, y_v);
            push_b(&mut b, b"\" stroke=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"1.6\" opacity=\"0.7\"/>");
            dot(&mut b, &p, i, cx, y_v, 4, col);
        }
        return finalize(b, cfg);
    }

    let mut group_indices: Vec<Vec<usize>> = vec![Vec::new(); groups.len()];
    for i in 0..p.n {
        if let Some(pos) = groups.iter().position(|g| *g == p.groups[i]) {
            group_indices[pos].push(i);
        }
    }

    let total = p.n as f64;
    let mut x_cursor = pl as f64;
    let gap_pct = 0.012_f64;
    let total_gap = pw as f64 * gap_pct * (groups.len() as f64 - 1.0).max(0.0);
    let usable = pw as f64 - total_gap;
    for (gi, idxs) in group_indices.iter().enumerate() {
        let count = idxs.len();
        if count == 0 {
            continue;
        }
        let g_w = usable * (count as f64 / total);
        let g_x0 = x_cursor;
        let g_x1 = x_cursor + g_w;
        let mut g_mean = 0.0_f64;
        for &i in idxs.iter() {
            g_mean += p.values[i];
        }
        g_mean /= count as f64;
        let pal = if !cfg.palette.is_empty() {
            crate::plot::statistical::common::palette_color(cfg.palette, gi)
        } else {
            0x6366F1
        };
        let hx_g = hex6(pal);

        let y_mean = pt + ph - (((g_mean - min_val) / range) * ph as f64) as i32;
        push_b(&mut b, b"<rect x=\"");
        push_f2(&mut b, g_x0);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, pt);
        push_b(&mut b, b"\" width=\"");
        push_f2(&mut b, g_w);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, ph);
        push_b(&mut b, b"\" fill=\"#");
        b.extend_from_slice(&hx_g);
        push_b(&mut b, b"\" opacity=\"0.04\"/>");
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, g_x0 + 4.0);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, y_mean);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, g_x1 - 4.0);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, y_mean);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx_g);
        push_b(
            &mut b,
            b"\" stroke-width=\"4\" stroke-linecap=\"round\" opacity=\"0.85\"/>",
        );

        let g_cx = (g_x0 + g_x1) / 2.0;
        push_b(&mut b, b"<rect x=\"");
        push_f2(&mut b, g_cx - 38.0);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, pt - 26);
        push_b(
            &mut b,
            b"\" width=\"76\" height=\"18\" rx=\"4\" fill=\"none\" stroke=\"#",
        );
        b.extend_from_slice(&hx_g);
        push_b(&mut b, b"\" stroke-width=\"1\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, g_cx);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, pt - 13);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#");
        b.extend_from_slice(&hx_g);
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &groups[gi]);
        push_b(&mut b, b"</text>");

        let inner_step = g_w / count as f64;
        for (k, &i) in idxs.iter().enumerate() {
            let cx = g_x0 + inner_step * 0.5 + inner_step * k as f64;
            let y_v = pt + ph - (((p.values[i] - min_val) / range) * ph as f64) as i32;
            push_b(&mut b, b"<line data-idx=\"");
            push_i(&mut b, i as i32);
            push_b(&mut b, b"\" data-y=\"");
            push_f2(&mut b, p.values[i]);
            push_b(&mut b, b"\" data-lbl=\"");
            escape_xml(&mut b, &p.labels[i]);
            push_b(&mut b, b"\" x1=\"");
            push_f2(&mut b, cx);
            push_b(&mut b, b"\" y1=\"");
            push_i(&mut b, y_mean);
            push_b(&mut b, b"\" x2=\"");
            push_f2(&mut b, cx);
            push_b(&mut b, b"\" y2=\"");
            push_i(&mut b, y_v);
            push_b(&mut b, b"\" stroke=\"#");
            b.extend_from_slice(&hx_g);
            push_b(&mut b, b"\" stroke-width=\"1.2\" opacity=\"0.55\"/>");
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, cx);
            push_b(&mut b, b"\" cy=\"");
            push_i(&mut b, y_v);
            push_b(&mut b, b"\" r=\"3.2\" fill=\"#");
            b.extend_from_slice(&hx_g);
            push_b(&mut b, b"\" opacity=\"0.9\"/>");
        }
        x_cursor = g_x1 + pw as f64 * gap_pct;
    }
    finalize(b, cfg)
}
