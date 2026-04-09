use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open, svg_title, svg_axis_lines, svg_y_label, svg_x_label};
use crate::html::hover::build_chart_html;

pub struct ViolinConfig<'a> {
    pub title: &'a str,
    pub categories: &'a [String],
    pub values: &'a [f64],
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub palette: &'a [u32],
    pub gridlines: bool,
    pub width: i32,
    pub height: i32,
}

impl<'a> Default for ViolinConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            categories: &[],
            values: &[],
            x_label: "",
            y_label: "",
            palette: &[],
            gridlines: true,
            width: 900,
            height: 500,
        }
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    let idx = p * (sorted.len() - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    let frac = idx - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi.min(sorted.len() - 1)] * frac
}

fn density_at(points: &[f64], x: f64, bw: f64) -> f64 {
    let inv_bw = 1.0 / bw;
    let c = 1.0 / (bw * (2.0 * std::f64::consts::PI).sqrt());
    points.iter()
        .map(|&p| { let z = (x - p) * inv_bw; c * (-0.5 * z * z).exp() })
        .sum::<f64>()
}

pub fn render_violin_html(cfg: &ViolinConfig) -> String {
    let n_vals = cfg.values.len();
    if n_vals == 0 || cfg.categories.is_empty() { return String::new(); }

    let mut cat_set: Vec<String> = Vec::new();
    for cat in cfg.categories.iter() {
        if !cat_set.contains(cat) { cat_set.push(cat.clone()); }
    }
    cat_set.sort();
    let n_cats = cat_set.len();

    let mut cat_vals: Vec<Vec<f64>> = vec![Vec::new(); n_cats];
    for i in 0..n_vals.min(cfg.categories.len()) {
        if let Some(ci) = cat_set.iter().position(|c| c == &cfg.categories[i]) {
            cat_vals[ci].push(cfg.values[i]);
        }
    }

    let global_min = cfg.values.iter().copied().fold(f64::INFINITY, f64::min);
    let global_max = cfg.values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = (global_max - global_min).max(1e-12);

    let pad_l: i32 = 60;
    let pad_t: i32 = 46;
    let pad_b: i32 = 52;
    let plot_w = cfg.width - pad_l - 20;
    let plot_h = cfg.height - pad_t - pad_b;
    let col_w = plot_w / n_cats as i32;
    let violin_half = (col_w as f64 * 0.38) as i32;

    let val_to_y = |v: f64| -> i32 {
        pad_t + ((1.0 - (v - global_min) / range) * plot_h as f64) as i32
    };

    let mut b = Vec::<u8>::with_capacity(n_cats * 600 + 2048);
    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, cfg.width / 2, 26);

    let n_yticks: i32 = 6;
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let v = global_min + frac * range;
        let y = pad_t + ((1.0 - frac) * plot_h as f64) as i32;
        if cfg.gridlines && ti > 0 {
            push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
            push_b(&mut b, b"\" y1=\""); push_i(&mut b, y);
            push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
            push_b(&mut b, b"\" y2=\""); push_i(&mut b, y);
            push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\"/>");
        }
        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 4);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, y + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        if v.abs() >= 1_000_000.0 { push_f2(&mut b, v / 1_000_000.0); push_b(&mut b, b"M"); }
        else if v.abs() >= 1000.0 { push_i(&mut b, v as i32); }
        else { push_f2(&mut b, v); }
        push_b(&mut b, b"</text>");
    }

    svg_axis_lines(&mut b, pad_l, pad_t, plot_w, plot_h);

    for (ci, cat) in cat_set.iter().enumerate() {
        let vals = &cat_vals[ci];
        if vals.is_empty() { continue; }
        let mut sorted = vals.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n_v = sorted.len();
        let bw = 1.06 * sorted.iter().copied()
            .map(|x| (x - sorted[n_v / 2]).powi(2))
            .sum::<f64>().sqrt()
            / (n_v as f64).powf(0.2)
            .max(range * 0.05);
        let cx = pad_l + ci as i32 * col_w + col_w / 2;
        let n_steps = 40usize;
        let color = palette_color(cfg.palette, ci);
        let hx = hex6(color);
        let dens: Vec<f64> = (0..=n_steps).map(|si| {
            let frac = si as f64 / n_steps as f64;
            let v = global_min + frac * range;
            density_at(&sorted, v, bw)
        }).collect();
        let max_dens = dens.iter().copied().fold(0.0_f64, f64::max).max(1e-10);
        let mut pts_r: Vec<(i32, i32)> = Vec::with_capacity(n_steps + 2);
        let mut pts_l: Vec<(i32, i32)> = Vec::with_capacity(n_steps + 2);
        for si in 0..=n_steps {
            let frac = si as f64 / n_steps as f64;
            let v = global_min + frac * range;
            let y = val_to_y(v);
            let w = (dens[si] / max_dens * violin_half as f64) as i32;
            pts_r.push((cx + w, y));
            pts_l.push((cx - w, y));
        }
        push_b(&mut b, b"<path data-idx=\"");
        push_i(&mut b, ci as i32);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, cat);
        push_b(&mut b, b"\" d=\"M ");
        push_i(&mut b, pts_r[0].0); b.push(b' '); push_i(&mut b, pts_r[0].1);
        for (px, py) in &pts_r[1..] {
            push_b(&mut b, b" L "); push_i(&mut b, *px); b.push(b' '); push_i(&mut b, *py);
        }
        for (px, py) in pts_l.iter().rev() {
            push_b(&mut b, b" L "); push_i(&mut b, *px); b.push(b' '); push_i(&mut b, *py);
        }
        push_b(&mut b, b" Z\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" opacity=\"0.7\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1\"/>");
        let q1 = percentile(&sorted, 0.25);
        let q3 = percentile(&sorted, 0.75);
        let med = percentile(&sorted, 0.5);
        let iqr_half_w = (violin_half as f64 * 0.18) as i32;
        let y_q1 = val_to_y(q1);
        let y_q3 = val_to_y(q3);
        let y_med = val_to_y(med);
        push_b(&mut b, b"<rect x=\""); push_i(&mut b, cx - iqr_half_w);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, y_q3);
        push_b(&mut b, b"\" width=\""); push_i(&mut b, iqr_half_w * 2);
        push_b(&mut b, b"\" height=\""); push_i(&mut b, (y_q1 - y_q3).abs().max(2));
        push_b(&mut b, b"\" fill=\"#fff\" opacity=\"0.75\" rx=\"2\"/>");
        push_b(&mut b, b"<line x1=\""); push_i(&mut b, cx - iqr_half_w);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, y_med);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, cx + iqr_half_w);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, y_med);
        push_b(&mut b, b"\" stroke=\"#1a202c\" stroke-width=\"2\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, cx);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t + plot_h + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\">");
        let short = if cat.len() > 10 { &cat[..10] } else { cat };
        escape_xml(&mut b, short);
        push_b(&mut b, b"</text>");
    }

    svg_y_label(&mut b, cfg.y_label, 14, pad_t, plot_h);
    svg_x_label(&mut b, cfg.x_label, pad_l + plot_w / 2, cfg.height - 4);

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
