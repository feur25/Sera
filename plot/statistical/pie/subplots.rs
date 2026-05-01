use super::config::PieConfig;
use super::basic::{render_pie_svg, PiePiece};
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, hex6, escape_xml, truncate};

pub fn render_subplots(cfg: &PieConfig) -> String {
    let n_pies = cfg.series.len();
    if n_pies == 0 {
        return super::basic::render_single(cfg, cfg.donut);
    }
    let labels = cfg.labels.to_vec();
    let label_names: Vec<String> = if labels.is_empty() {
        (0..cfg.series.iter().map(|s| s.len()).max().unwrap_or(0))
            .map(|i| format!("S{}", i + 1)).collect()
    } else { labels };
    let n_cats = label_names.len();

    let cols = if cfg.subplot_cols > 0 {
        cfg.subplot_cols
    } else if n_pies <= 2 { n_pies.max(1) }
    else if n_pies <= 4 { 2 }
    else { 3 };
    let rows = (n_pies + cols - 1) / cols;

    let w = cfg.width.max(420);
    let h = cfg.height.max(320);

    let mut buf = Vec::<u8>::with_capacity(n_pies * n_cats * 380 + 2048);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, w); push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, h); push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, w); push_b(&mut buf, b" ");
    push_i(&mut buf, h); push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    let title_h: f64 = if cfg.title.is_empty() { 8.0 } else { 30.0 };
    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, w / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#e2e8f0\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let legend_h: f64 = 30.0;
    let avail_h = h as f64 - title_h - legend_h - 10.0;
    let avail_w = w as f64 - 20.0;
    let cell_w = avail_w / cols as f64;
    let cell_h = avail_h / rows as f64;

    let totals: Vec<f64> = cfg.series.iter().map(|s| s.iter().filter(|v| v.is_finite() && **v >= 0.0).sum()).collect();
    let max_total = totals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);

    for pi in 0..n_pies {
        let r_row = pi / cols;
        let c_col = pi % cols;
        let area_x = 10.0 + c_col as f64 * cell_w;
        let area_y = title_h + r_row as f64 * cell_h;

        let s = &cfg.series[pi];
        let n = n_cats.min(s.len());
        let lab_slice: Vec<String> = label_names[..n].to_vec();
        let val_slice: Vec<f64> = s[..n].to_vec();

        let pull_empty: Vec<f64> = Vec::new();
        let pull_ref: &[f64] = &pull_empty;

        let radius_scale = if cfg.proportional {
            (totals[pi] / max_total).sqrt().max(0.15)
        } else { 1.0 };

        let sub_title = cfg.subplot_titles.get(pi).cloned().unwrap_or_default();

        let piece = PiePiece {
            area_x,
            area_y,
            area_w: cell_w,
            area_h: cell_h,
            donut: cfg.donut,
            radius_scale,
            draw_legend: false,
            title_top: !sub_title.is_empty(),
            title: sub_title,
            palette_offset: 0,
        };
        render_pie_svg(&mut buf, cfg, &lab_slice, &val_slice, pull_ref, &piece);
    }

    let legend_y = (h as f64 - legend_h * 0.5) as i32;
    let mut acc_x: i32 = 12;
    let max_x = w - 12;
    for i in 0..n_cats {
        let c = palette_color(cfg.palette, i);
        let hx = hex6(c);
        let label = truncate(&label_names[i], 18);
        let est_w = 18 + label.len() as i32 * 6 + 18;
        if acc_x + est_w > max_x { break; }
        push_b(&mut buf, b"<g data-legend=\"1\" data-series=\""); push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\"><rect x=\""); push_i(&mut buf, acc_x);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, legend_y - 6);
        push_b(&mut buf, b"\" width=\"12\" height=\"12\" rx=\"3\" fill=\"#");
        buf.extend_from_slice(&hx); push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, acc_x + 16);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, legend_y + 4);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#cbd5e1\">");
        escape_xml(&mut buf, label);
        push_b(&mut buf, b"</text></g>");
        acc_x += est_w;
    }

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
