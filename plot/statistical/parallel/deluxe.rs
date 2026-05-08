use super::common::{prepare, open, finalize, write_dots, point};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, escape_xml, hex6, palette_color};

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = open(cfg, &p);

    push_b(&mut b, b"<defs>");
    push_b(&mut b, b"<filter id=\"dlxpf\" x=\"-40%\" y=\"-40%\" width=\"280%\" height=\"280%\">");
    push_b(&mut b, b"<feGaussianBlur stdDeviation=\"3\" result=\"bk\"/>");
    push_b(&mut b, b"<feMerge><feMergeNode in=\"bk\"/><feMergeNode in=\"SourceGraphic\"/></feMerge>");
    push_b(&mut b, b"</filter>");
    push_b(&mut b, b"</defs>");

    push_b(&mut b, b"<rect x=\"0\" y=\"0\" width=\"");
    push_i(&mut b, cfg.width);
    push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height);
    push_b(&mut b, b"\" fill=\"#080d1a\" rx=\"4\"/>");

    for ai in 0..p.n_axes {
        let x = p.pad_l + (ai as f64 / (p.n_axes - 1) as f64 * p.plot_w as f64) as i32;
        push_b(&mut b, b"<line x1=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, p.pad_t);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, p.pad_t + p.plot_h);
        push_b(&mut b, b"\" stroke=\"#334155\" stroke-width=\"1\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, p.pad_t + p.plot_h + 18);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
        escape_xml(&mut b, &cfg.axes[ai]);
        push_b(&mut b, b"</text>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, p.pad_t + p.plot_h + 32);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#475569\">");
        let s = format!("{:.1}", p.mins[ai]);
        b.extend_from_slice(s.as_bytes());
        push_b(&mut b, b"</text>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, p.pad_t - 6);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#475569\">");
        let s = format!("{:.1}", p.maxs[ai]);
        b.extend_from_slice(s.as_bytes());
        push_b(&mut b, b"</text>");
    }

    for si in 0..p.n_series {
        let v = &cfg.series_values[si];
        let col = palette_color(cfg.palette, si);
        let hx = hex6(col);

        push_b(&mut b, b"<polyline data-idx=\""); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" data-series=\""); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &cfg.series_names[si]);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"5\" stroke-opacity=\"0.08\" stroke-linejoin=\"round\" filter=\"url(#dlxpf)\" points=\"");
        for ai in 0..p.n_axes {
            if ai >= v.len() { break; }
            let (x, y) = point(&p, ai, v[ai]);
            if ai > 0 { push_b(&mut b, b" "); }
            push_f2(&mut b, x); push_b(&mut b, b","); push_f2(&mut b, y);
        }
        push_b(&mut b, b"\"/>");

        push_b(&mut b, b"<polyline data-idx=\""); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" data-series=\""); push_i(&mut b, si as i32);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &cfg.series_names[si]);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.6\" stroke-opacity=\"0.9\" stroke-linejoin=\"round\" filter=\"url(#dlxpf)\" points=\"");
        for ai in 0..p.n_axes {
            if ai >= v.len() { break; }
            let (x, y) = point(&p, ai, v[ai]);
            if ai > 0 { push_b(&mut b, b" "); }
            push_f2(&mut b, x); push_b(&mut b, b","); push_f2(&mut b, y);
        }
        push_b(&mut b, b"\"/>");

        write_dots(&mut b, &p, v, col, 3.5, 1.0, si);
    }

    for (li, name) in cfg.series_names.iter().enumerate() {
        let col = palette_color(cfg.palette, li);
        let hx = hex6(col);
        let lx = cfg.width - p.pad_r + 14;
        let ly = p.pad_t + 4 + (li as i32) * 20;
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, lx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly);
        push_b(&mut b, b"\" width=\"12\" height=\"3\" rx=\"1\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\"0.9\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, lx + 16);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, ly + 9);
        push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#94a3b8\">");
        escape_xml(&mut b, name);
        push_b(&mut b, b"</text>");
    }

    finalize(b, cfg)
}
