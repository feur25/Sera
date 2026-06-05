use super::common::{color_for, data_attrs, dot, finalize, open, prepare, x_tick_label};
use super::config::LollipopConfig;
use crate::plot::statistical::common::{
    hex6, push_b, push_f2, push_i, svg_axis_lines, svg_hgrid, svg_tick_y, svg_y_label,
};

#[crate::chart_demo(
    "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]"
)]

pub fn render(cfg: &LollipopConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
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
    for i in 0..p.n {
        let cx = pl + (step * 0.5 + step * i as f64) as i32;
        let v = p.values[i];
        let y_v = pt + ph - (((v - min_val) / range) * ph as f64) as i32;
        let col = color_for(cfg, &p, i);
        let hx = hex6(col);
        push_b(&mut b, b"<line");
        data_attrs(&mut b, &p, i);
        push_b(&mut b, b" x1=\"");
        push_i(&mut b, cx);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, baseline);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, cx);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, y_v);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.8\"/>");
        dot(&mut b, &p, i, cx, y_v, 5, col);
        if cfg.show_values {
            push_b(&mut b, b"<text x=\"");
            push_i(&mut b, cx);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, y_v - 9);
            push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
            push_f2(&mut b, v);
            push_b(&mut b, b"</text>");
        }
        if i % tick_step == 0 {
            x_tick_label(&mut b, cx, pt + ph + 14, &p.labels[i]);
        }
    }
    finalize(b, cfg)
}
