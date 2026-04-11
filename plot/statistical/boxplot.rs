use super::common::{sorted, sort_indices, palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_axis_lines, svg_x_label, svg_y_label, svg_legend_item, Frame};
use crate::html::hover::slots_to_json;

crate::chart_config!(BoxplotConfig, 900, 500;
    struct {
        pub category_labels: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
    }
    defaults {
        category_labels: &[],
        values: &[],
        palette: &[],
    }
);

fn quartile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let pos = q * (sorted.len().saturating_sub(1)) as f64;
    let lo = pos.floor() as usize;
    let hi = (pos.ceil() as usize).min(sorted.len() - 1);
    if lo == hi { sorted[lo] } else { sorted[lo] + (sorted[hi] - sorted[lo]) * (pos - lo as f64) }
}

#[derive(Clone)]
struct BoxStats {
    q1: f64, median: f64, q3: f64,
    whisker_lo: f64, whisker_hi: f64,
    outliers: Vec<f64>,
}

fn compute_box(vals: &[f64]) -> BoxStats {
    let mut s = vals.to_vec();
    s.retain(|v| v.is_finite());
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if s.is_empty() {
        return BoxStats { q1: 0.0, median: 0.0, q3: 0.0, whisker_lo: 0.0, whisker_hi: 0.0, outliers: Vec::new() };
    }
    let q1 = quartile(&s, 0.25);
    let median = quartile(&s, 0.5);
    let q3 = quartile(&s, 0.75);
    let iqr = q3 - q1;
    let fence_lo = q1 - 1.5 * iqr;
    let fence_hi = q3 + 1.5 * iqr;
    let whisker_lo = s.iter().cloned().filter(|&v| v >= fence_lo).fold(q1, f64::min);
    let whisker_hi = s.iter().cloned().filter(|&v| v <= fence_hi).fold(q3, f64::max);
    let outliers: Vec<f64> = s.iter().cloned().filter(|&v| v < fence_lo || v > fence_hi).collect();
    BoxStats { q1, median, q3, whisker_lo, whisker_hi, outliers }
}

pub fn render_boxplot_html(cfg: &BoxplotConfig) -> String {
    let title = cfg.title;
    let category_labels = cfg.category_labels;
    let values = cfg.values;
    let width = cfg.width;
    let height = cfg.height;
    let palette = cfg.palette;
    let hover = cfg.hover;
    let x_label = cfg.x_label;
    let y_label = cfg.y_label;
    let gridlines = cfg.gridlines;

    let n = category_labels.len().min(values.len());
    if n == 0 { return String::new(); }

    let mut cats: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<f64>> = Vec::new();
    for i in 0..n {
        let cat = &category_labels[i];
        match cats.iter().position(|c| c == cat) {
            Some(pos) => groups[pos].push(values[i]),
            None => { cats.push(cat.clone()); groups.push(vec![values[i]]); }
        }
    }
    let n_cats = cats.len();
    let stats: Vec<BoxStats> = groups.iter().map(|v| compute_box(v)).collect();
    let medians: Vec<f64> = stats.iter().map(|s| s.median).collect();
    let sort_idx = sort_indices(n_cats, &medians, &cats, cfg.sort_order);
    let cats  = sorted(&sort_idx, &cats);
    let stats = sorted(&sort_idx, &stats);

    let global_min = stats.iter().fold(f64::INFINITY, |acc, s| {
        acc.min(s.whisker_lo).min(s.outliers.iter().cloned().fold(s.whisker_lo, f64::min))
    });
    let global_max = stats.iter().fold(f64::NEG_INFINITY, |acc, s| {
        acc.max(s.whisker_hi).max(s.outliers.iter().cloned().fold(s.whisker_hi, f64::max))
    });
    let pad_v = (global_max - global_min) * 0.06;
    let y_min = global_min - pad_v;
    let y_max = global_max + pad_v;
    let range_y = (y_max - y_min).max(1.0);

    let legend_w: i32 = if n_cats > 1 { 130 } else { 20 };
    let pad_l: i32 = 62;
    let plot_w = width - pad_l - legend_w;
    let plot_h = height - 44 - 54;

    let slot_w = plot_w as f64 / n_cats as f64;
    let box_hw = (slot_w * 0.28) as i32;

    let mut f = Frame::new_html(title, width, height, 62, 44, 54, legend_w, n_cats * 500 + 2048);
    f.open(title, false);
    f.y_grid(5, y_min, y_max, gridlines);
    svg_axis_lines(&mut f.buf, f.pl, f.pt, f.pw, f.ph);

    for (ci, (cat, st)) in cats.iter().zip(stats.iter()).enumerate() {
        let cx = f.pl + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(palette, ci);
        let hx = hex6(color);

        push_b(&mut f.buf, b"<g data-series=\""); push_i(&mut f.buf, ci as i32); push_b(&mut f.buf, b"\">");

        let y_q1    = f.pt + f.ph - ((st.q1 - y_min) / range_y * f.ph as f64) as i32;
        let y_med   = f.pt + f.ph - ((st.median - y_min) / range_y * f.ph as f64) as i32;
        let y_q3    = f.pt + f.ph - ((st.q3 - y_min) / range_y * f.ph as f64) as i32;
        let y_wlo   = f.pt + f.ph - ((st.whisker_lo - y_min) / range_y * f.ph as f64) as i32;
        let y_whi   = f.pt + f.ph - ((st.whisker_hi - y_min) / range_y * f.ph as f64) as i32;

        let box_top = y_q3.min(y_q1);
        let box_bot = y_q3.max(y_q1);
        let box_h = (box_bot - box_top).max(2);

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_whi);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, box_top);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, box_bot);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_wlo);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - box_hw / 2);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_whi);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + box_hw / 2);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_whi);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - box_hw / 2);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_wlo);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + box_hw / 2);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_wlo);
        push_b(&mut f.buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, ci as i32);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"\" data-kv-Median=\""); push_f2(&mut f.buf, st.median);
        push_b(&mut f.buf, b"\" data-kv-Q1=\""); push_f2(&mut f.buf, st.q1);
        push_b(&mut f.buf, b"\" data-kv-Q3=\""); push_f2(&mut f.buf, st.q3);
        push_b(&mut f.buf, b"\" data-kv-Min=\""); push_f2(&mut f.buf, st.whisker_lo);
        push_b(&mut f.buf, b"\" data-kv-Max=\""); push_f2(&mut f.buf, st.whisker_hi);
        push_b(&mut f.buf, b"\" data-kv-Outliers=\""); push_i(&mut f.buf, st.outliers.len() as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, cx - box_hw);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, box_top);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, box_hw * 2);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, box_h);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.28\" rx=\"3\" stroke=\"#");
        f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"1.6\"/>");

        push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - box_hw);
        push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + box_hw);
        push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_med);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\"2.4\"/>");

        for &ov in &st.outliers {
            let oy = f.pt + f.ph - ((ov - y_min) / range_y * f.ph as f64) as i32;
            push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
            push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, oy);
            push_b(&mut f.buf, b"\" r=\"3\" fill=\"#"); f.buf.extend_from_slice(&hx);
            push_b(&mut f.buf, b"\" fill-opacity=\"0.7\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
        }

        push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut f.buf, if cat.len() <= 14 { cat } else { &cat[..14] });
        push_b(&mut f.buf, b"</text>");
        push_b(&mut f.buf, b"</g>");

    }

    if n_cats > 1 {
        let lx = width - legend_w + 12;
        for (ci, cat) in cats.iter().enumerate() {
            svg_legend_item(&mut f.buf, ci as i32, cat, palette_color(palette, ci), lx, f.pt + ci as i32 * 22, 14);
        }
    }

    svg_x_label(&mut f.buf, x_label, f.pl + f.pw / 2, f.h - 4);
    svg_y_label(&mut f.buf, y_label, 14, f.pt, f.ph);

    f.html(&slots_to_json(hover))
}
