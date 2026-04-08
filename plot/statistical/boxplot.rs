use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6};
use crate::html::hover::{HoverSlot, slots_to_json, build_chart_html};

fn quartile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let pos = q * (sorted.len().saturating_sub(1)) as f64;
    let lo = pos.floor() as usize;
    let hi = (pos.ceil() as usize).min(sorted.len() - 1);
    if lo == hi { sorted[lo] } else { sorted[lo] + (sorted[hi] - sorted[lo]) * (pos - lo as f64) }
}

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

pub fn render_boxplot_html(
    title: &str,
    category_labels: &[String],
    values: &[f64],
    width: i32,
    height: i32,
    palette: &[u32],
    hover: &[HoverSlot],
) -> String {
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

    let pad_l: i32 = 62;
    let pad_t: i32 = 44;
    let pad_b: i32 = 54;
    let pad_r: i32 = 20;
    let plot_w = width - pad_l - pad_r;
    let plot_h = height - pad_t - pad_b;

    let slot_w = plot_w as f64 / n_cats as f64;
    let box_hw = (slot_w * 0.28) as i32;

    let mut buf = Vec::<u8>::with_capacity(n_cats * 500 + 2048);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(&mut buf, width); push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, height); push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, width); push_b(&mut buf, b" ");
    push_i(&mut buf, height); push_b(&mut buf, b"\">");
    push_b(&mut buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");

    if !title.is_empty() {
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, width / 2);
        push_b(&mut buf, b"\" y=\"28\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
        escape_xml(&mut buf, title);
        push_b(&mut buf, b"</text>");
    }

    let n_yticks = 5i32;
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let ty = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        let val = y_min + frac * range_y;
        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, ty);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, ty);
        push_b(&mut buf, b"\" stroke=\"#e5e7eb\" stroke-width=\"0.6\" stroke-dasharray=\"3,3\"/>");
        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, pad_l - 4);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, ty + 4);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        if val.abs() >= 1000.0 { push_i(&mut buf, val as i32); }
        else { push_f2(&mut buf, val); }
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t);
    push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>");
    push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, pad_l + plot_w);
    push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#9ca3af\" stroke-width=\"1.2\"/>");

    for (ci, (cat, st)) in cats.iter().zip(stats.iter()).enumerate() {
        let cx = pad_l + (ci as f64 * slot_w + slot_w / 2.0) as i32;
        let color = palette_color(palette, ci);
        let hx = hex6(color);

        let y_q1    = pad_t + plot_h - ((st.q1 - y_min) / range_y * plot_h as f64) as i32;
        let y_med   = pad_t + plot_h - ((st.median - y_min) / range_y * plot_h as f64) as i32;
        let y_q3    = pad_t + plot_h - ((st.q3 - y_min) / range_y * plot_h as f64) as i32;
        let y_wlo   = pad_t + plot_h - ((st.whisker_lo - y_min) / range_y * plot_h as f64) as i32;
        let y_whi   = pad_t + plot_h - ((st.whisker_hi - y_min) / range_y * plot_h as f64) as i32;

        let box_top = y_q3.min(y_q1);
        let box_bot = y_q3.max(y_q1);
        let box_h = (box_bot - box_top).max(2);

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y_whi);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, box_top);
        push_b(&mut buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, box_bot);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y_wlo);
        push_b(&mut buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, cx - box_hw / 2);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y_whi);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, cx + box_hw / 2);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y_whi);
        push_b(&mut buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, cx - box_hw / 2);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y_wlo);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, cx + box_hw / 2);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y_wlo);
        push_b(&mut buf, b"\" stroke=\"#6b7280\" stroke-width=\"1.4\"/>");

        push_b(&mut buf, b"<rect data-idx=\""); push_i(&mut buf, ci as i32);
        push_b(&mut buf, b"\" data-lbl=\""); escape_xml(&mut buf, cat);
        push_b(&mut buf, b"\" data-kv-Median=\""); push_f2(&mut buf, st.median);
        push_b(&mut buf, b"\" data-kv-Q1=\""); push_f2(&mut buf, st.q1);
        push_b(&mut buf, b"\" data-kv-Q3=\""); push_f2(&mut buf, st.q3);
        push_b(&mut buf, b"\" data-kv-Min=\""); push_f2(&mut buf, st.whisker_lo);
        push_b(&mut buf, b"\" data-kv-Max=\""); push_f2(&mut buf, st.whisker_hi);
        push_b(&mut buf, b"\" data-kv-Outliers=\""); push_i(&mut buf, st.outliers.len() as i32);
        push_b(&mut buf, b"\" x=\""); push_i(&mut buf, cx - box_hw);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, box_top);
        push_b(&mut buf, b"\" width=\""); push_i(&mut buf, box_hw * 2);
        push_b(&mut buf, b"\" height=\""); push_i(&mut buf, box_h);
        push_b(&mut buf, b"\" fill=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.28\" stroke=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"1.6\"/>");

        push_b(&mut buf, b"<line x1=\""); push_i(&mut buf, cx - box_hw);
        push_b(&mut buf, b"\" y1=\""); push_i(&mut buf, y_med);
        push_b(&mut buf, b"\" x2=\""); push_i(&mut buf, cx + box_hw);
        push_b(&mut buf, b"\" y2=\""); push_i(&mut buf, y_med);
        push_b(&mut buf, b"\" stroke=\"#"); buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke-width=\"2.4\"/>");

        for &ov in &st.outliers {
            let oy = pad_t + plot_h - ((ov - y_min) / range_y * plot_h as f64) as i32;
            push_b(&mut buf, b"<circle cx=\""); push_i(&mut buf, cx);
            push_b(&mut buf, b"\" cy=\""); push_i(&mut buf, oy);
            push_b(&mut buf, b"\" r=\"3\" fill=\"#"); buf.extend_from_slice(&hx);
            push_b(&mut buf, b"\" fill-opacity=\"0.7\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
        }

        push_b(&mut buf, b"<text x=\""); push_i(&mut buf, cx);
        push_b(&mut buf, b"\" y=\""); push_i(&mut buf, pad_t + plot_h + 16);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
        escape_xml(&mut buf, if cat.len() <= 14 { cat } else { &cat[..14] });
        push_b(&mut buf, b"</text>");

    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(title, &svg, &slots_to_json(hover))
}
