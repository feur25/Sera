use super::common::{data_attrs, dot, finalize, open, prepare, x_tick_label, x_tick_label_rotated};
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i, svg_axis_lines, svg_hgrid_vis, svg_tick_y, svg_y_label};
use super::config::LollipopConfig;

#[crate::chart_demo("labels=[\"1951\",\"1961\",\"1971\",\"1981\",\"1991\",\"2001\",\"2011\",\"2021\"], values=[-0.3,-0.15,0.05,0.22,0.38,0.61,0.85,1.1], variant=\"trend\"")]

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let pad_b = if p.n >= 8 { 72 } else { 52 };
    let (mut b, pl, pt, pw, ph) = open(cfg, 56, 42, 24, pad_b);
    let max_val = p.vmax.max(0.0);
    let min_val = p.vmin.min(0.0);
    let range = (max_val - min_val).max(1.0);
    let n_yticks: i32 = 5;
    for ti in 0..=n_yticks {
        let y = pt + ph - (ph as f64 * ti as f64 / n_yticks as f64) as i32;
        svg_hgrid_vis(&mut b, pl, pl + pw, y, cfg.gridlines);
    }
    svg_y_label(&mut b, cfg.y_label, 12, pt, ph);
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let y = pt + ph - (ph as f64 * frac) as i32;
        svg_tick_y(&mut b, pl - 4, y + 4, min_val + frac * range);
    }
    svg_axis_lines(&mut b, pl, pt, pw, ph);
    let baseline = pt + ph - ((-min_val) / range * ph as f64) as i32;
    let step = pw as f64 / p.n as f64;
    let tick_step = ((p.n as f64 / 14.0).ceil() as usize).max(1);

    let max_abs = p.values.iter().fold(0.0_f64, |a, v| a.max(v.abs())).max(1e-9);
    let cool: (f64, f64, f64) = (0x21 as f64, 0x96 as f64, 0xF3 as f64);
    let warm: (f64, f64, f64) = (0xE0 as f64, 0x5A as f64, 0x20 as f64);
    let neutral: (f64, f64, f64) = (0xCB as f64, 0xD5 as f64, 0xE1 as f64);
    let color_for_value = |v: f64| -> u32 {
        let t = (v.abs() / max_abs).clamp(0.0, 1.0);
        let (tr, tg, tb) = if v >= 0.0 { warm } else { cool };
        let r = (neutral.0 + (tr - neutral.0) * t) as u32;
        let g = (neutral.1 + (tg - neutral.1) * t) as u32;
        let bl = (neutral.2 + (tb - neutral.2) * t) as u32;
        (r << 16) | (g << 8) | bl
    };

    let mut xs = Vec::with_capacity(p.n);
    let mut ys = Vec::with_capacity(p.n);
    let mut peak_i = 0usize;
    let mut peak_abs = -1.0_f64;
    for i in 0..p.n {
        let cx = pl + (step * 0.5 + step * i as f64) as i32;
        let v = p.values[i];
        let y_v = pt + ph - (((v - min_val) / range) * ph as f64) as i32;
        xs.push(cx);
        ys.push(y_v);
        if v.abs() > peak_abs {
            peak_abs = v.abs();
            peak_i = i;
        }
    }

    if p.n >= 3 {
        let window = 3usize.max(p.n / 12).min(7) | 1;
        let half = window / 2;
        let mut smoothed = Vec::with_capacity(p.n);
        for i in 0..p.n {
            let lo = i.saturating_sub(half);
            let hi = (i + half + 1).min(p.n);
            let avg: f64 = p.values[lo..hi].iter().sum::<f64>() / (hi - lo) as f64;
            smoothed.push(pt + ph - (((avg - min_val) / range) * ph as f64) as i32);
        }
        push_b(&mut b, b"<path fill=\"none\" stroke=\"#94a3b8\" stroke-width=\"1.6\" stroke-dasharray=\"5,3\" opacity=\"0.7\" d=\"M");
        push_i(&mut b, xs[0]);
        push_b(&mut b, b",");
        push_i(&mut b, smoothed[0]);
        for i in 1..p.n {
            push_b(&mut b, b" L");
            push_i(&mut b, xs[i]);
            push_b(&mut b, b",");
            push_i(&mut b, smoothed[i]);
        }
        push_b(&mut b, b"\"/>");
    }

    for i in 0..p.n {
        let v = p.values[i];
        let col = color_for_value(v);
        let hx = crate::plot::statistical::common::hex6(col);
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\"");
        push_i(&mut b, xs[i]);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, baseline);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, xs[i]);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, ys[i]);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\"/>");
        dot(&mut b, &p, i, xs[i], ys[i], if i == peak_i { 6 } else { 4 }, col);
        if i % tick_step == 0 {
            if p.n >= 8 {
                x_tick_label_rotated(&mut b, xs[i], pt + ph + 14, &p.labels[i]);
            } else {
                x_tick_label(&mut b, xs[i], pt + ph + 14, &p.labels[i]);
            }
        }
    }

    let ax = xs[peak_i];
    let ay = ys[peak_i];
    let above = ay > pt + 30;
    let (tx, ty) = if above { (ax, ay - 30) } else { (ax, ay + 30) };
    push_b(&mut b, b"<path fill=\"none\" stroke=\"#1e293b\" stroke-width=\"1.3\" marker-end=\"url(#sp-lollipop-arrowhead)\" d=\"M");
    push_i(&mut b, tx);
    push_b(&mut b, b",");
    push_i(&mut b, if above { ty + 6 } else { ty - 6 });
    push_b(&mut b, b" L");
    push_i(&mut b, ax);
    push_b(&mut b, b",");
    push_i(&mut b, if above { ay - 8 } else { ay + 8 });
    push_b(&mut b, b"\"/>");
    push_b(&mut b, b"<defs><marker id=\"sp-lollipop-arrowhead\" markerWidth=\"7\" markerHeight=\"7\" refX=\"5\" refY=\"3\" orient=\"auto\"><path d=\"M0,0 L6,3 L0,6 Z\" fill=\"#1e293b\"/></marker></defs>");
    push_b(&mut b, b"<text x=\"");
    push_i(&mut b, tx);
    push_b(&mut b, b"\" y=\"");
    push_i(&mut b, if above { ty - 4 } else { ty + 14 });
    push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#1e293b\">");
    escape_xml(&mut b, &p.labels[peak_i]);
    push_b(&mut b, b" (");
    push_f2(&mut b, p.values[peak_i]);
    push_b(&mut b, b")</text>");

    finalize(b, cfg)
}
