use super::config::LineConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], values=[12,19,15,22,28,24]"
)]

pub fn render(cfg: &LineConfig) -> String {
    let series: Vec<(String, Vec<f64>)> = if cfg.series.is_empty() {
        if cfg.values.is_empty() {
            return String::new();
        }
        vec![(cfg.title.to_string(), cfg.values.to_vec())]
    } else {
        cfg.series.to_vec()
    };
    let n_ser = series.len();
    if n_ser == 0 {
        return String::new();
    }

    let cols = cfg.spark_cols.max(1);
    let rows = (n_ser + cols - 1) / cols;
    let cell_w = cfg.spark_cell_w.max(80);
    let cell_h = cfg.spark_cell_h.max(40);
    let pad = 14i32;
    let total_w = (cols as i32) * (cell_w + pad) + pad;
    let total_h = (rows as i32) * (cell_h + pad) + pad + 28;

    let hid = html_id();
    let mut b = Vec::<u8>::with_capacity(n_ser * 600 + 2048);
    html_prefix(&mut b, cfg.title, hid);
    push_b(
        &mut b,
        b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"",
    );
    push_i(&mut b, total_w);
    push_b(&mut b, b"\" height=\"");
    push_i(&mut b, total_h);
    push_b(&mut b, b"\" viewBox=\"0 0 ");
    push_i(&mut b, total_w);
    push_b(&mut b, b" ");
    push_i(&mut b, total_h);
    push_b(
        &mut b,
        b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );

    if !cfg.title.is_empty() {
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, total_w / 2);
        push_b(&mut b, b"\" y=\"18\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut b, cfg.title);
        push_b(&mut b, b"</text>");
    }

    for (si, (name, vals)) in series.iter().enumerate() {
        let r = (si / cols) as i32;
        let c = (si % cols) as i32;
        let x0 = pad + c * (cell_w + pad);
        let y0 = 28 + pad + r * (cell_h + pad);
        let color = if cfg.color_hex != 0 {
            cfg.color_hex
        } else {
            palette_color(cfg.palette, si)
        };
        let hx = hex6(color);

        push_b(&mut b, b"<rect x=\"");
        push_i(&mut b, x0);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, y0);
        push_b(&mut b, b"\" width=\"");
        push_i(&mut b, cell_w);
        push_b(&mut b, b"\" height=\"");
        push_i(&mut b, cell_h);
        push_b(
            &mut b,
            b"\" fill=\"#f8fafc\" stroke=\"#e2e8f0\" stroke-width=\"1\" rx=\"4\"/>",
        );

        if !name.is_empty() {
            push_b(&mut b, b"<text x=\"");
            push_i(&mut b, x0 + 6);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, y0 + 12);
            push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"#475569\">");
            escape_xml(&mut b, name);
            push_b(&mut b, b"</text>");
        }

        let n = vals.len();
        if n < 2 {
            continue;
        }
        let mx = vals
            .iter()
            .cloned()
            .filter(|v| v.is_finite())
            .fold(f64::NEG_INFINITY, f64::max);
        let mn = vals
            .iter()
            .cloned()
            .filter(|v| v.is_finite())
            .fold(f64::INFINITY, f64::min);
        let range = (mx - mn).max(1e-12);
        let inset_t = 18i32;
        let inset_b = 6i32;
        let plot_h = cell_h - inset_t - inset_b;
        let step_x = (cell_w as f64 - 12.0) / (n - 1) as f64;

        if let Some(last) = vals.last() {
            push_b(&mut b, b"<text x=\"");
            push_i(&mut b, x0 + cell_w - 6);
            push_b(&mut b, b"\" y=\"");
            push_i(&mut b, y0 + 12);
            push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#1f2937\">");
            push_f2(&mut b, *last);
            push_b(&mut b, b"</text>");
        }

        push_b(&mut b, b"<polyline fill=\"none\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"");
        push_f2(&mut b, cfg.stroke_width);
        push_b(
            &mut b,
            b"\" stroke-linecap=\"round\" stroke-linejoin=\"round\" points=\"",
        );
        for i in 0..n {
            let v = vals[i];
            let xi = x0 + 6 + (i as f64 * step_x) as i32;
            let yi = y0 + inset_t + ((1.0 - (v - mn) / range) * plot_h as f64) as i32;
            if i > 0 {
                b.push(b' ');
            }
            push_i(&mut b, xi);
            b.push(b',');
            push_i(&mut b, yi);
        }
        push_b(&mut b, b"\"/>");
    }

    push_b(&mut b, b"</svg>");
    html_suffix(&mut b, hid, "[]");
    unsafe { String::from_utf8_unchecked(b) }
}
