use super::config::BarConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, escape_xml, push_hex, Frame};

pub const DEMO_KWARGS: &str = "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21]], series_names=[\"2023\",\"2024\"], super_categories=[\"H1\",\"H1\",\"H2\",\"H2\"]";

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len().max(1);
    if n_cats == 0 { return String::new(); }

    let max_val: f64 = if cfg.series.is_empty() {
        cfg.values.iter().cloned().filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1.0)
    } else {
        cfg.series.iter()
            .flat_map(|(_, v)| v.iter().copied())
            .filter(|v| v.is_finite())
            .fold(0.0f64, f64::max).max(1.0)
    };

    let legend_w = if cfg.series.is_empty() { 0 } else { 160 };
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 70, legend_w, n_cats * n_ser * 200 + 4096);
    f.open(cfg.title, true);
    f.y_grid(6, 0.0, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cat_w = f.pw as f64 / n_cats as f64;
    let bar_w = (cat_w / (n_ser as f64 + 0.6) * 0.9) as i32;

    for ci in 0..n_cats {
        let cx = f.pl + (ci as f64 * cat_w + cat_w / 2.0) as i32;
        if cfg.series.is_empty() {
            let v = cfg.values.get(ci).copied().unwrap_or(0.0);
            let color = palette_color(cfg.palette, ci);
            let h = ((v / max_val) * f.ph as f64) as i32;
            let bx = cx - bar_w / 2;
            let by = f.pt + f.ph - h;
            push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, bx);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, by);
            push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bar_w);
            push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, h.max(1));
            push_b(&mut f.buf, b"\" fill=\""); push_hex(&mut f.buf, color);
            push_b(&mut f.buf, b"\"/>");
        } else {
            let group_w = cat_w * 0.85;
            let bw = (group_w / n_ser as f64) as i32;
            for (si, (_, vals)) in cfg.series.iter().enumerate() {
                let v = vals.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() { continue; }
                let color = palette_color(cfg.palette, si);
                let h = ((v / max_val) * f.ph as f64) as i32;
                let bx = cx - (group_w / 2.0) as i32 + si as i32 * bw;
                let by = f.pt + f.ph - h;
                push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, by);
                push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, (bw - 2).max(1));
                push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, h.max(1));
                push_b(&mut f.buf, b"\" fill=\""); push_hex(&mut f.buf, color);
                push_b(&mut f.buf, b"\"/>");
            }
        }
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut f.buf, &cfg.category_labels[ci]);
        push_b(&mut f.buf, b"</text>");
    }

    if !cfg.super_categories.is_empty() {
        let mut start = 0usize;
        while start < n_cats {
            let cur = cfg.super_categories.get(start).map(|s| s.as_str()).unwrap_or("");
            let mut end = start + 1;
            while end < n_cats {
                let nxt = cfg.super_categories.get(end).map(|s| s.as_str()).unwrap_or("");
                if nxt != cur { break; }
                end += 1;
            }
            let x1 = f.pl + (start as f64 * cat_w) as i32 + 4;
            let x2 = f.pl + (end as f64 * cat_w) as i32 - 4;
            let y_line = f.pt + f.ph + 32;
            let y_text = f.pt + f.ph + 50;
            push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, x1);
            push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_line);
            push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, x2);
            push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_line);
            push_b(&mut f.buf, b"\" stroke=\"#94a3b8\" stroke-width=\"1\"/>");
            push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, (x1 + x2) / 2);
            push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y_text);
            push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#334155\">");
            escape_xml(&mut f.buf, cur);
            push_b(&mut f.buf, b"</text>");
            start = end;
        }
    }

    if !cfg.series.is_empty() {
        let names: Vec<&str> = cfg.series.iter().map(|(n, _)| n.as_str()).collect();
        f.legend_pos(&names, cfg.palette, cfg.legend_position);
    }
    f.html("[]")
}


