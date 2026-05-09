use super::common::{prepare, finalize, point};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color, svg_open, svg_title};

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b: Vec<u8> = Vec::with_capacity(p.n_series * p.n_axes * 300 + 4096);
    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, cfg.width / 2, 26);

    for ai in 0..p.n_axes {
        let x = p.pad_l + (ai as f64 / (p.n_axes - 1) as f64 * p.plot_w as f64) as i32;
        push_b(&mut b, b"<line x1=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, p.pad_t);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, p.pad_t + p.plot_h);
        push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, p.pad_t + p.plot_h + 18);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut b, &cfg.axes[ai]);
        push_b(&mut b, b"</text>");
    }

    push_b(&mut b, b"<defs>");
    for si in 0..p.n_series {
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);
        push_b(&mut b, b"<linearGradient id=\"arcg"); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" gradientUnits=\"userSpaceOnUse\" x1=\"");
        push_i(&mut b, p.pad_l);
        push_b(&mut b, b"\" y1=\"0\" x2=\"");
        push_i(&mut b, p.pad_l + p.plot_w);
        push_b(&mut b, b"\" y2=\"0\">");
        push_b(&mut b, b"<stop offset=\"0%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.2\"/>");
        push_b(&mut b, b"<stop offset=\"50%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.7\"/>");
        push_b(&mut b, b"<stop offset=\"100%\" stop-color=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stop-opacity=\"0.2\"/>");
        push_b(&mut b, b"</linearGradient>");
    }
    push_b(&mut b, b"</defs>");

    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);

        for seg in 0..p.n_axes.saturating_sub(1) {
            if seg >= v.len() || seg + 1 >= v.len() { break; }
            let (x1, y1) = point(&p, seg, v[seg]);
            let (x2, y2) = point(&p, seg + 1, v[seg + 1]);
            let mx = (x1 + x2) / 2.0;
            let ctrl1x = x1 + (mx - x1) * 0.5;
            let ctrl2x = x2 - (mx - x1) * 0.5;

            push_b(&mut b, b"<path data-idx=\""); push_i(&mut b, si as i32);
            push_b(&mut b, b"\" data-series=\""); push_i(&mut b, si as i32);
            push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &cfg.series_names[si]);
            push_b(&mut b, b"\" fill=\"none\" stroke=\"url(#arcg"); push_i(&mut b, si as i32);
            push_b(&mut b, b")\" stroke-width=\"2.5\" stroke-opacity=\"0.85\" d=\"M ");
            push_f2(&mut b, x1); push_b(&mut b, b" "); push_f2(&mut b, y1);
            push_b(&mut b, b" C ");
            push_f2(&mut b, ctrl1x); push_b(&mut b, b" "); push_f2(&mut b, y1);
            push_b(&mut b, b" ");
            push_f2(&mut b, ctrl2x); push_b(&mut b, b" "); push_f2(&mut b, y2);
            push_b(&mut b, b" ");
            push_f2(&mut b, x2); push_b(&mut b, b" "); push_f2(&mut b, y2);
            push_b(&mut b, b"\"/>");
        }

        for ai in 0..p.n_axes.min(v.len()) {
            let (x, y) = point(&p, ai, v[ai]);
            push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, x);
            push_b(&mut b, b"\" cy=\""); push_f2(&mut b, y);
            push_b(&mut b, b"\" r=\"4\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" fill-opacity=\"0.9\"/>");
        }
    }

    for (li, name) in cfg.series_names.iter().enumerate() {
        let col = palette_color(cfg.palette, li);
        let hx = hex6(col);
        let lx = cfg.width - p.pad_r + 14;
        let ly = p.pad_t + 4 + (li as i32) * 20;
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, lx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly);
        push_b(&mut b, b"\" width=\"12\" height=\"3\" rx=\"1\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, lx + 16);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly + 9);
        push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut b, name);
        push_b(&mut b, b"</text>");
    }

    finalize(b, cfg)
}


