use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i,
    sort_indices, sorted, svg_axis_lines, svg_legend_item, svg_x_label, svg_y_label, Frame,
};

#[derive(Clone)]
pub struct BoxStats {
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub whisker_lo: f64,
    pub whisker_hi: f64,
    pub outliers: Vec<f64>,
    pub n: usize,
    pub mean: f64,
}

pub fn quartile(s: &[f64], q: f64) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let pos = q * (s.len().saturating_sub(1)) as f64;
    let lo = pos.floor() as usize;
    let hi = (pos.ceil() as usize).min(s.len() - 1);
    if lo == hi {
        s[lo]
    } else {
        s[lo] + (s[hi] - s[lo]) * (pos - lo as f64)
    }
}

pub fn compute_box(vals: &[f64]) -> BoxStats {
    let mut s: Vec<f64> = vals.iter().copied().filter(|v| v.is_finite()).collect();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if s.is_empty() {
        return BoxStats {
            q1: 0.0, median: 0.0, q3: 0.0,
            whisker_lo: 0.0, whisker_hi: 0.0,
            outliers: Vec::new(), n: 0, mean: 0.0,
        };
    }
    let q1 = quartile(&s, 0.25);
    let median = quartile(&s, 0.5);
    let q3 = quartile(&s, 0.75);
    let iqr = q3 - q1;
    let fence_lo = q1 - 1.5 * iqr;
    let fence_hi = q3 + 1.5 * iqr;
    let whisker_lo = s.iter().copied().filter(|&v| v >= fence_lo).fold(q1, f64::min);
    let whisker_hi = s.iter().copied().filter(|&v| v <= fence_hi).fold(q3, f64::max);
    let outliers: Vec<f64> = s.iter().copied().filter(|&v| v < fence_lo || v > fence_hi).collect();
    let n = s.len();
    let mean = s.iter().sum::<f64>() / n as f64;
    BoxStats { q1, median, q3, whisker_lo, whisker_hi, outliers, n, mean }
}

pub fn group_values(cats: &[String], vals: &[f64]) -> (Vec<String>, Vec<Vec<f64>>) {
    let n = cats.len().min(vals.len());
    let mut keys: Vec<String> = Vec::new();
    let mut groups: Vec<Vec<f64>> = Vec::new();
    for i in 0..n {
        match keys.iter().position(|c| c == &cats[i]) {
            Some(p) => groups[p].push(vals[i]),
            None => {
                keys.push(cats[i].clone());
                groups.push(vec![vals[i]]);
            }
        }
    }
    (keys, groups)
}

pub struct GlobalRange {
    pub y_min: f64,
    pub y_max: f64,
    pub range_y: f64,
}

pub fn global_range(stats: &[BoxStats]) -> GlobalRange {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for s in stats {
        lo = lo.min(s.whisker_lo);
        hi = hi.max(s.whisker_hi);
        for &o in &s.outliers {
            lo = lo.min(o);
            hi = hi.max(o);
        }
    }
    if !lo.is_finite() || !hi.is_finite() {
        lo = 0.0;
        hi = 1.0;
    }
    let pad = (hi - lo).abs().max(1.0) * 0.06;
    let y_min = lo - pad;
    let y_max = hi + pad;
    GlobalRange { y_min, y_max, range_y: (y_max - y_min).max(1.0) }
}

pub fn sorted_groups(
    cats: Vec<String>,
    groups: Vec<Vec<f64>>,
    stats: Vec<BoxStats>,
    sort_order: &str,
) -> (Vec<String>, Vec<Vec<f64>>, Vec<BoxStats>) {
    let medians: Vec<f64> = stats.iter().map(|s| s.median).collect();
    let idx = sort_indices(cats.len(), &medians, &cats, sort_order);
    let cats = sorted(&idx, &cats);
    let groups = sorted(&idx, &groups);
    let stats = sorted(&idx, &stats);
    (cats, groups, stats)
}

pub fn make_frame(cfg: &super::config::BoxplotConfig, n_cats: usize, legend_w: i32) -> Frame {
    Frame::new_html(cfg.title, cfg.width, cfg.height, 62, 44, 54, legend_w, n_cats * 600 + 4096)
}

pub fn rng_next(state: &mut u64) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    ((*state >> 11) & ((1u64 << 53) - 1)) as f64 / ((1u64 << 53) as f64)
}

pub fn draw_box_vertical(
    f: &mut Frame,
    cx: i32,
    box_hw: i32,
    st: &BoxStats,
    color: u32,
    fill_opacity: f64,
    stroke_width: f64,
    notch: bool,
    cat: &str,
    series_idx: i32,
    y_min: f64, range_y: f64,
) {
    let pt = f.pt;
    let ph = f.ph;
    let hx = hex6(color);
    let yv = |v: f64| pt + ph - ((v - y_min) / range_y * ph as f64) as i32;
    let y_q1 = yv(st.q1);
    let y_med = yv(st.median);
    let y_q3 = yv(st.q3);
    let y_wlo = yv(st.whisker_lo);
    let y_whi = yv(st.whisker_hi);
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

    if notch && st.n > 1 {
        let ci = 1.57 * (st.q3 - st.q1) / (st.n as f64).sqrt();
        let yn_lo = yv(st.median - ci);
        let yn_hi = yv(st.median + ci);
        let nw = (box_hw as f64 * 0.55).max(2.0) as i32;
        push_b(&mut f.buf, b"<polygon data-idx=\""); push_i(&mut f.buf, series_idx);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"\" data-kv-Median=\""); push_f2(&mut f.buf, st.median);
        push_b(&mut f.buf, b"\" data-kv-Q1=\""); push_f2(&mut f.buf, st.q1);
        push_b(&mut f.buf, b"\" data-kv-Q3=\""); push_f2(&mut f.buf, st.q3);
        push_b(&mut f.buf, b"\" data-kv-N=\""); push_i(&mut f.buf, st.n as i32);
        push_b(&mut f.buf, b"\" points=\"");
        let pts = [
            (cx - box_hw, box_top),
            (cx + box_hw, box_top),
            (cx + box_hw, yn_hi),
            (cx + nw,    y_med),
            (cx + box_hw, yn_lo),
            (cx + box_hw, box_bot),
            (cx - box_hw, box_bot),
            (cx - box_hw, yn_lo),
            (cx - nw,    y_med),
            (cx - box_hw, yn_hi),
        ];
        for (i, (x, y)) in pts.iter().enumerate() {
            if i > 0 { push_b(&mut f.buf, b" "); }
            push_i(&mut f.buf, *x);
            push_b(&mut f.buf, b",");
            push_i(&mut f.buf, *y);
        }
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, fill_opacity);
        push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, stroke_width);
        push_b(&mut f.buf, b"\"/>");
    } else {
        push_b(&mut f.buf, b"<rect data-idx=\""); push_i(&mut f.buf, series_idx);
        push_b(&mut f.buf, b"\" data-lbl=\""); escape_xml(&mut f.buf, cat);
        push_b(&mut f.buf, b"\" data-kv-Median=\""); push_f2(&mut f.buf, st.median);
        push_b(&mut f.buf, b"\" data-kv-Q1=\""); push_f2(&mut f.buf, st.q1);
        push_b(&mut f.buf, b"\" data-kv-Q3=\""); push_f2(&mut f.buf, st.q3);
        push_b(&mut f.buf, b"\" data-kv-Min=\""); push_f2(&mut f.buf, st.whisker_lo);
        push_b(&mut f.buf, b"\" data-kv-Max=\""); push_f2(&mut f.buf, st.whisker_hi);
        push_b(&mut f.buf, b"\" data-kv-Mean=\""); push_f2(&mut f.buf, st.mean);
        push_b(&mut f.buf, b"\" data-kv-N=\""); push_i(&mut f.buf, st.n as i32);
        push_b(&mut f.buf, b"\" data-kv-Outliers=\""); push_i(&mut f.buf, st.outliers.len() as i32);
        push_b(&mut f.buf, b"\" x=\""); push_i(&mut f.buf, cx - box_hw);
        push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, box_top);
        push_b(&mut f.buf, b"\" width=\""); push_i(&mut f.buf, box_hw * 2);
        push_b(&mut f.buf, b"\" height=\""); push_i(&mut f.buf, box_h);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\""); push_f2(&mut f.buf, fill_opacity);
        push_b(&mut f.buf, b"\" rx=\"3\" stroke=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" stroke-width=\""); push_f2(&mut f.buf, stroke_width);
        push_b(&mut f.buf, b"\"/>");
    }

    push_b(&mut f.buf, b"<line x1=\""); push_i(&mut f.buf, cx - box_hw);
    push_b(&mut f.buf, b"\" y1=\""); push_i(&mut f.buf, y_med);
    push_b(&mut f.buf, b"\" x2=\""); push_i(&mut f.buf, cx + box_hw);
    push_b(&mut f.buf, b"\" y2=\""); push_i(&mut f.buf, y_med);
    push_b(&mut f.buf, b"\" stroke=\"#"); f.buf.extend_from_slice(&hx);
    push_b(&mut f.buf, b"\" stroke-width=\"2.4\"/>");
}

pub fn draw_outliers_vertical(
    f: &mut Frame,
    cx: i32,
    st: &BoxStats,
    color: u32,
    radius: f64,
    y_min: f64,
    range_y: f64,
) {
    let hx = hex6(color);
    for &ov in &st.outliers {
        let oy = f.pt + f.ph - ((ov - y_min) / range_y * f.ph as f64) as i32;
        push_b(&mut f.buf, b"<circle cx=\""); push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\""); push_i(&mut f.buf, oy);
        push_b(&mut f.buf, b"\" r=\""); push_f2(&mut f.buf, radius);
        push_b(&mut f.buf, b"\" fill=\"#"); f.buf.extend_from_slice(&hx);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.75\" stroke=\"#fff\" stroke-width=\"0.8\"/>");
    }
}

pub fn draw_cat_label(f: &mut Frame, cx: i32, cat: &str) {
    push_b(&mut f.buf, b"<text x=\""); push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" y=\""); push_i(&mut f.buf, f.pt + f.ph + 16);
    push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" class=\"sp-xt\">");
    let trimmed = if cat.len() <= 14 { cat } else { &cat[..14] };
    escape_xml(&mut f.buf, trimmed);
    push_b(&mut f.buf, b"</text>");
}

pub fn finish_frame(
    f: &mut Frame,
    cats: &[String],
    palette: &[u32],
    x_label: &str,
    y_label: &str,
    legend_w: i32,
) {
    if cats.len() > 1 && legend_w > 30 {
        let lx = f.w - legend_w + 12;
        for (ci, cat) in cats.iter().enumerate() {
            svg_legend_item(&mut f.buf, ci as i32, cat, palette_color(palette, ci), lx, f.pt + ci as i32 * 22, 14);
        }
    }
    svg_x_label(&mut f.buf, x_label, f.pl + f.pw / 2, f.h - 4);
    svg_y_label(&mut f.buf, y_label, 14, f.pt, f.ph);
}

pub fn open_axes(f: &mut Frame, title: &str, gridlines: bool, y_min: f64, y_max: f64) {
    f.open(title, false);
    f.y_grid(5, y_min, y_max, gridlines);
    svg_axis_lines(&mut f.buf, f.pl, f.pt, f.pw, f.ph);
}

pub fn open_axes_horizontal(f: &mut Frame, title: &str, gridlines: bool, x_min: f64, x_max: f64) {
    f.open(title, false);
    f.x_grid(5, x_min, x_max, gridlines);
    svg_axis_lines(&mut f.buf, f.pl, f.pt, f.pw, f.ph);
}
