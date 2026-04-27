use super::config::BarConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, escape_xml, push_hex, Frame};

pub fn render(cfg: &BarConfig) -> String {
    let n_cats = cfg.category_labels.len();
    let n_ser = cfg.series.len();
    if n_cats == 0 || n_ser == 0 { return String::new(); }

    let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
    for (si, _) in cfg.series.iter().enumerate() {
        let g = cfg.offset_groups.get(si).cloned().unwrap_or_else(|| si.to_string());
        if let Some((_, list)) = groups.iter_mut().find(|(name, _)| name == &g) {
            list.push(si);
        } else {
            groups.push((g, vec![si]));
        }
    }
    let n_groups = groups.len() as f64;

    let mut max_val = 0.0f64;
    for ci in 0..n_cats {
        for (_, sl) in groups.iter() {
            let s: f64 = sl.iter()
                .filter_map(|&si| cfg.series[si].1.get(ci).copied())
                .filter(|v| v.is_finite() && *v >= 0.0).sum();
            if s > max_val { max_val = s; }
        }
    }
    max_val = max_val.max(1.0);

    let legend_w = 160;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 56, 42, 52, legend_w, n_cats * n_ser * 220 + 4096);
    f.open(cfg.title, true);
    f.y_grid(6, 0.0, max_val, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);

    let cat_w = f.pw as f64 / n_cats as f64;
    let group_w = cat_w * (1.0 - cfg.bar_gap) / n_groups;
    let bar_w = (group_w * (1.0 - cfg.bargroup_gap)) as i32;

    for ci in 0..n_cats {
        let cat_left = f.pl as f64 + ci as f64 * cat_w + cat_w * cfg.bar_gap / 2.0;
        for (gi, (_, sl)) in groups.iter().enumerate() {
            let gx = cat_left + gi as f64 * group_w + group_w / 2.0;
            let bx = gx as i32 - bar_w / 2;
            let mut acc = 0.0f64;
            for &si in sl {
                let v = cfg.series[si].1.get(ci).copied().unwrap_or(0.0);
                if !v.is_finite() || v < 0.0 { continue; }
                let color = palette_color(cfg.palette, si);
                let y_top = f.pt + (((max_val - acc - v) / max_val) * f.ph as f64) as i32;
                let h = ((v / max_val) * f.ph as f64) as i32;
                acc += v;
                push_b(&mut f.buf, b"<rect x=\""); push_i(&mut f.buf, bx);
                push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, y_top);
                push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, bar_w);
                push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, h.max(1));
                if cfg.corner_radius > 0 {
                    push_b(&mut f.buf, b"\" rx=\""); push_i(&mut f.buf, cfg.corner_radius);
                }
                push_b(&mut f.buf, b"\" fill=\""); push_hex(&mut f.buf, color);
                push_b(&mut f.buf, b"\" stroke=\"#fff\" stroke-width=\"0.5\"/>");
            }
        }
        let cx = f.pl + (ci as f64 * cat_w + cat_w / 2.0) as i32;
        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#475569\">");
        escape_xml(&mut f.buf, &cfg.category_labels[ci]);
        push_b(&mut f.buf, b"</text>");
    }

    let names: Vec<&str> = cfg.series.iter().map(|(n, _)| n.as_str()).collect();
    f.legend_pos(&names, cfg.palette, cfg.legend_position);
    f.html("[]")
}
