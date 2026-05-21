use super::config::BarConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, escape_xml, push_hex, Frame};

#[crate::chart_demo("labels=[\"North\",\"South\",\"East\",\"West\"], series=[[42,28,33,17],[18,38,22,30],[24,15,28,20]], series_names=[\"Product A\",\"Product B\",\"Product C\"], widths=[2.0,1.5,1.0,1.2]")]

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }

    let widths: Vec<f64> = if cfg.widths.len() == n_cats {
        cfg.widths.to_vec()
    } else {
        vec![1.0; n_cats]
    };
    let total_w: f64 = widths.iter().sum::<f64>().max(1.0);

    let cat_totals: Vec<f64> = (0..n_cats).map(|ci| {
        cfg.series.iter()
            .filter_map(|(_, v)| v.get(ci).copied())
            .filter(|v| v.is_finite() && *v >= 0.0).sum()
    }).collect();

    let legend_w = 160;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n_cats * n_ser * 220 + 4096);
    f.open(cfg.title, true);
    f.y_grid(5, 0.0, 100.0, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let mut x_acc = 0.0f64;
    for ci in 0..n_cats {
        let w_norm = widths[ci] / total_w;
        let bx = f.pl + (x_acc * f.pw as f64) as i32;
        let bw = (w_norm * f.pw as f64) as i32;
        let total = cat_totals[ci].max(1e-9);
        let mut acc = 0.0f64;
        for (si, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(ci).copied().unwrap_or(0.0);
            if !v.is_finite() || v < 0.0 { continue; }
            let color = palette_color(cfg.palette, si);
            let p_start = acc / total;
            let p_end = (acc + v) / total;
            let y_top = f.pt + ((1.0 - p_end) * f.ph as f64) as i32;
            let y_bot = f.pt + ((1.0 - p_start) * f.ph as f64) as i32;
            let h = (y_bot - y_top).max(1);
            acc += v;
            push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, bx);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y_top);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bw.max(1));
            push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, h);
            push_b(&mut f.buf, b"\" fill=\""); push_hex(&mut f.buf, color);
            push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"1\"/>");

            if cfg.show_text && bw >= 30 && h >= 18 {
                push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, bx + bw / 2);
                push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y_top + h / 2 + 3);
                push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#fff\" font-weight=\"600\">");
                push_f2(&mut f.buf, v);
                push_b(&mut f.buf, b"</text>");
            }
        }

        let cx = bx + bw / 2;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut f.buf, &cfg.category_labels[ci]);
        push_b(&mut f.buf, b"</text>");
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 28);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#94a3b8\">w=");
        push_f2(&mut f.buf, widths[ci]);
        push_b(&mut f.buf, b"</text>");
        x_acc += w_norm;
    }

    let names: Vec<&str> = cfg.series.iter().map(|(n, _)| n.as_str()).collect();
    f.legend_pos(&names, cfg.palette, cfg.legend_position);
    f.html("[]")
}

