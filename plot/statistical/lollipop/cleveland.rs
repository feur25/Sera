use super::common::{color_for, data_attrs, dot, finalize, open, prepare};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, push_b, push_f2, push_i, svg_axis_lines, svg_vgrid, truncate,
};

#[crate::chart_demo("labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], series=[[24,38,17,42,29],[18,28,22,35,33]], series_names=[\"2023\",\"2024\"]")]

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let (mut b, pl, pt, pw, ph) = open(cfg, 140, 42, 28, 44);
    let max_val = p.vmax.max(0.0);
    let min_val = p.vmin.min(0.0);
    let range = (max_val - min_val).max(1.0);
    let n_xticks: i32 = 5;
    if cfg.gridlines {
        for ti in 0..=n_xticks {
            let x = pl + (pw as f64 * ti as f64 / n_xticks as f64) as i32;
            svg_vgrid(&mut b, x, pt, pt + ph);
        }
    }
    svg_axis_lines(&mut b, pl, pt, pw, ph);
    let baseline = pl + ((-min_val) / range * pw as f64) as i32;
    let step = ph as f64 / p.n as f64;
    for i in 0..p.n {
        let cy = pt + (step * 0.5 + step * i as f64) as i32;
        let v = p.values[i];
        let x_v = pl + (((v - min_val) / range) * pw as f64) as i32;
        let col = color_for(cfg, &p, i);
        let hx = hex6(col);
        push_b(&mut b, b"<line x1=\"");
        push_i(&mut b, pl);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, cy);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, pl + pw);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, cy);
        push_b(
            &mut b,
            b"\" stroke=\"#e5e7eb\" stroke-width=\"0.8\" stroke-dasharray=\"2,3\"/>",
        );
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\"");
        push_i(&mut b, baseline);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, cy);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, x_v);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, cy);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\"/>");
        dot(&mut b, &p, i, x_v, cy, 6, col);
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, pl - 8);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, cy + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut b, truncate(&p.labels[i], 18));
        push_b(&mut b, b"</text>");
        if cfg.show_values {
            let dx = if v >= 0.0 { x_v + 9 } else { x_v - 9 };
            let anchor: &[u8] = if v >= 0.0 { b"start" } else { b"end" };
            push_b(&mut b, b"<text x=\"");
            push_i(&mut b, dx);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, cy + 4);
            push_b(&mut b, b"\" text-anchor=\"");
            b.extend_from_slice(anchor);
            push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            push_f2(&mut b, v);
            push_b(&mut b, b"</text>");
        }
    }
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = pl + (pw as f64 * frac) as i32;
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, x);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, pt + ph + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#9ca3af\">");
        push_f2(&mut b, min_val + frac * range);
        push_b(&mut b, b"</text>");
    }
    finalize(b, cfg)
}
