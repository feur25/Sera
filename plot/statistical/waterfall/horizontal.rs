use super::common::{prepare, finalize, bar_color, COLOR_POS, COLOR_NEG, COLOR_TOTAL};
use super::config::WaterfallConfig;
use crate::plot::statistical::common::{push_b, push_i, push_f2, hex6, escape_xml};

#[crate::chart_demo("labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"End\"], values=[100,30,-15,40,155]")]

pub fn render(cfg: &WaterfallConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 2048);

    let pad_l: i32 = 130;
    let pad_r: i32 = 36;
    let pad_t: i32 = 46;
    let pad_b: i32 = 44;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_h = cfg.height - pad_t - pad_b;
    let row_h = (plot_h / p.n as i32).max(14);
    let bar_h = (row_h - 8).max(6);

    let min_x = p.layout.min_val;
    let max_x = p.layout.max_val;
    let range = (max_x - min_x).max(1e-12);
    let zero_x = pad_l + (((0.0 - min_x) / range) * plot_w as f64) as i32;

    push_b(&mut b, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut b, cfg.width); push_b(&mut b, b"\" height=\"");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, cfg.width); push_b(&mut b, b" ");
    push_i(&mut b, cfg.height); push_b(&mut b, b"\">");
    push_b(&mut b, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");
    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cfg.width / 2);
        push_b(&mut b, b"\" y=\"26\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    let n_xticks: i32 = 5;
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let v = min_x + frac * range;
        let x = pad_l + (frac * plot_w as f64) as i32;
        if cfg.gridlines && ti > 0 {
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, x);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, x);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
            push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>");
        }
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t + plot_h + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        if v.abs() >= 1_000_000.0 { push_f2(&mut b, v / 1_000_000.0); push_b(&mut b, b"M"); }
        else if v.abs() >= 1_000.0 { push_i(&mut b, v as i32); }
        else { push_f2(&mut b, v); }
        push_b(&mut b, b"</text>");
    }

    push_b(&mut b, b"<line x1=\""); push_i(&mut b, zero_x);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, zero_x);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\"/>");
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, pad_t + plot_h);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");

    let val_to_x = |v: f64| pad_l + (((v - min_x) / range) * plot_w as f64) as i32;

    for i in 0..p.n {
        let row_y = pad_t + i as i32 * row_h + (row_h - bar_h) / 2;
        let cy = row_y + bar_h / 2;
        let xs = val_to_x(p.starts[i]);
        let xe = val_to_x(p.ends[i]);
        let left = xs.min(xe);
        let w = (xe - xs).abs().max(2);
        let hx = hex6(bar_color(&p, i));

        push_b(&mut b, b"<rect data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-y=\""); push_f2(&mut b, p.values[i]);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &p.labels[i]); push_b(&mut b, b"\"");
        push_b(&mut b, b" x=\""); push_i(&mut b, left);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, row_y);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, w);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, bar_h);
        push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" rx=\"2\" opacity=\"0.92\"/>");

        if i + 1 < p.n && !p.is_total[i + 1] {
            let next_y = pad_t + (i as i32 + 1) * row_h + (row_h - bar_h) / 2;
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, xe);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, row_y + bar_h);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, xe);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, next_y);
            push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" stroke-dasharray=\"2,2\"/>");
        }

        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 8);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, cy + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        let lbl = &p.labels[i];
        let short = if lbl.len() > 18 { &lbl[..18] } else { lbl.as_str() };
        escape_xml(&mut b, short);
        push_b(&mut b, b"</text>");

        if cfg.show_text && w > 16 {
            let v = if p.is_total[i] { p.ends[i] } else { p.values[i] };
            push_b(&mut b, b"<text x=\""); push_i(&mut b, xe + 6);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, cy + 4);
            push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#374151\" pointer-events=\"none\">");
            if v.abs() >= 1_000_000.0 { push_f2(&mut b, v / 1_000_000.0); push_b(&mut b, b"M"); }
            else if v.abs() >= 1_000.0 { push_f2(&mut b, v / 1_000.0); push_b(&mut b, b"k"); }
            else { push_f2(&mut b, v); }
            push_b(&mut b, b"</text>");
        }
    }
    let _ = COLOR_POS; let _ = COLOR_NEG; let _ = COLOR_TOTAL;
    finalize(b, cfg)
}

