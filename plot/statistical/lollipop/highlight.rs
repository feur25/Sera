use super::common::{prepare, open, finalize, dot, data_attrs, x_tick_label};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, svg_axis_lines, svg_hgrid, svg_y_label, svg_tick_y};

#[crate::chart_demo("labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]")]

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let (mut b, pl, pt, pw, ph) = open(cfg, 56, 42, 20, 52);
    let max_val = p.vmax.max(0.0);
    let min_val = p.vmin.min(0.0);
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
    svg_axis_lines(&mut b, pl, pt, pw, ph);
    let baseline = pt + ph - ((-min_val) / range * ph as f64) as i32;
    let step = pw as f64 / p.n as f64;
    let tick_step = ((p.n as f64 / 14.0).ceil() as usize).max(1);
    let highlight_idx = if cfg.highlight_index >= 0 {
        cfg.highlight_index as usize
    } else {
        let mut best = 0usize;
        let mut best_v = f64::NEG_INFINITY;
        for i in 0..p.n {
            if p.values[i] > best_v { best_v = p.values[i]; best = i; }
        }
        best
    };
    let dim_color: u32 = 0xCBD5E1;
    let hi_color: u32 = if cfg.color_hex != 0 { cfg.color_hex } else { 0xEF4444 };
    for i in 0..p.n {
        let cx = pl + (step * 0.5 + step * i as f64) as i32;
        let v = p.values[i];
        let y_v = pt + ph - (((v - min_val) / range) * ph as f64) as i32;
        let is_hi = i == highlight_idx;
        let col = if is_hi { hi_color } else { dim_color };
        let hx = hex6(col);
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, baseline);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, y_v);
        push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
        if is_hi { push_b(&mut b, b"\" stroke-width=\"2.6\"/>"); }
        else { push_b(&mut b, b"\" stroke-width=\"1.4\" opacity=\"0.7\"/>"); }
        dot(&mut b, &p, i, cx, y_v, if is_hi { 7 } else { 4 }, col);
        if is_hi {
            push_b(&mut b, b"<text x=\""); push_i(&mut b, cx);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, y_v - 12);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"700\" font-size=\"11\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\">");
            push_f2(&mut b, v);
            push_b(&mut b, b"</text>");
        }
        if i % tick_step == 0 {
            x_tick_label(&mut b, cx, pt + ph + 14, &p.labels[i]);
        }
    }
    finalize(b, cfg)
}

