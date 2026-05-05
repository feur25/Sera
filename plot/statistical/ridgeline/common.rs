use super::config::RidgelineConfig;
use crate::plot::statistical::common::{sort_indices, sorted, palette_color, push_b, push_i, push_f2, escape_xml, svg_open, svg_title, svg_x_label, svg_y_label, svg_legend_item, truncate};
use crate::plot::statistical::kde::{scott_bw, kde_eval};
use crate::html::hover::{build_chart_html, slots_to_json};

pub struct Layout {
    pub title_h: i32,
    pub pad_l: i32,
    pub pad_b: i32,
    pub pad_r: i32,
    pub plot_w: i32,
    pub plot_area_h: i32,
    pub row_h: i32,
    pub curve_h: i32,
    pub axis_y: i32,
}

pub struct Prepared {
    pub group_order: Vec<String>,
    pub group_vals: Vec<Vec<f64>>,
    pub x0: f64,
    pub x1: f64,
    pub xr: f64,
    pub xs: Vec<f64>,
    pub curves: Vec<Vec<f64>>,
    pub global_max: f64,
    pub layout: Layout,
}

pub fn prepare(cfg: &RidgelineConfig, overlap_override: Option<f64>) -> Option<Prepared> {
    let n = cfg.values.len().min(cfg.categories.len());
    if n == 0 { return None; }

    let mut group_order: Vec<String> = Vec::new();
    for cat in cfg.categories[..n].iter() {
        if !group_order.contains(cat) { group_order.push(cat.clone()); }
    }
    let n_groups = group_order.len();
    if n_groups == 0 { return None; }

    let mut group_vals: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
    for i in 0..n {
        if let Some(gi) = group_order.iter().position(|g| g == &cfg.categories[i]) {
            group_vals[gi].push(cfg.values[i]);
        }
    }
    let means: Vec<f64> = group_vals.iter().map(|v| if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 }).collect();
    let sort_idx = sort_indices(n_groups, &means, &group_order, cfg.sort_order);
    let group_order = sorted(&sort_idx, &group_order);
    let group_vals  = sorted(&sort_idx, &group_vals);
    let n_groups = group_order.len();

    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    for v in &cfg.values[..n] {
        if v.is_finite() {
            if *v < x_min { x_min = *v; }
            if *v > x_max { x_max = *v; }
        }
    }
    if !x_min.is_finite() { return None; }
    let xr_raw = (x_max - x_min).max(1e-12);
    let x_pad = xr_raw * 0.12;
    let x0 = x_min - x_pad;
    let x1 = x_max + x_pad;
    let xr = x1 - x0;

    let n_pts = cfg.n_points.max(40);
    let xs: Vec<f64> = (0..n_pts).map(|i| x0 + xr * i as f64 / (n_pts - 1) as f64).collect();

    let curves: Vec<Vec<f64>> = group_vals.iter().map(|vals| {
        if vals.is_empty() { return vec![0.0; n_pts]; }
        let cap = 40usize;
        let step = if vals.len() > cap { (vals.len() + cap - 1) / cap } else { 1 };
        let sampled: Vec<f64> = if step > 1 { vals.iter().step_by(step).copied().collect() } else { vals.to_vec() };
        let bw = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(&sampled).max(1e-12) };
        xs.iter().map(|&x| kde_eval(&sampled, x, bw) * step as f64).collect()
    }).collect();

    let global_max = curves.iter().flat_map(|c| c.iter().copied()).fold(0.0_f64, f64::max).max(1e-12);

    let title_h: i32 = if cfg.title.is_empty() { 0 } else { 36 };
    let pad_l: i32 = 130;
    let pad_b: i32 = 44;
    let pad_r: i32 = if n_groups > 1 { 130 } else { 20 };
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_area_h = cfg.height - title_h - pad_b;
    let row_h = (plot_area_h / n_groups as i32).max(1);
    let ov = overlap_override.unwrap_or(cfg.overlap).clamp(0.0, 2.0);
    let curve_h = (row_h as f64 * (1.0 + ov)) as i32;
    let axis_y = title_h + plot_area_h;

    Some(Prepared {
        group_order, group_vals, x0, x1, xr, xs, curves, global_max,
        layout: Layout { title_h, pad_l, pad_b, pad_r, plot_w, plot_area_h, row_h, curve_h, axis_y },
    })
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &RidgelineConfig, layout: &Layout, x0: f64, xr: f64) {
    svg_open(buf, cfg.width, cfg.height);
    svg_title(buf, cfg.title, cfg.width / 2, if layout.title_h > 0 { 24 } else { 0 });
    let n_xticks: i32 = 6;
    push_b(buf, b"<line x1=\""); push_i(buf, layout.pad_l);
    push_b(buf, b"\" y1=\""); push_i(buf, layout.axis_y);
    push_b(buf, b"\" x2=\""); push_i(buf, layout.pad_l + layout.plot_w);
    push_b(buf, b"\" y2=\""); push_i(buf, layout.axis_y);
    push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\" class=\"sp-ax-x\"/>");
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = layout.pad_l + (layout.plot_w as f64 * frac) as i32;
        let val = x0 + xr * frac;
        if cfg.gridlines {
            push_b(buf, b"<line x1=\""); push_i(buf, x);
            push_b(buf, b"\" y1=\""); push_i(buf, layout.title_h);
            push_b(buf, b"\" x2=\""); push_i(buf, x);
            push_b(buf, b"\" y2=\""); push_i(buf, layout.axis_y);
            push_b(buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>");
        }
        push_b(buf, b"<text x=\""); push_i(buf, x);
        push_b(buf, b"\" y=\""); push_i(buf, layout.axis_y + 14);
        push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(buf, val);
        push_b(buf, b"</text>");
    }
}

pub fn ridge_label(buf: &mut Vec<u8>, layout: &Layout, base_y: i32, name: &str) {
    push_b(buf, b"<line x1=\""); push_i(buf, layout.pad_l - 4);
    push_b(buf, b"\" y1=\""); push_i(buf, base_y);
    push_b(buf, b"\" x2=\""); push_i(buf, layout.pad_l + layout.plot_w);
    push_b(buf, b"\" y2=\""); push_i(buf, base_y);
    push_b(buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.6\"/>");
    push_b(buf, b"<text x=\""); push_i(buf, layout.pad_l - 8);
    push_b(buf, b"\" y=\""); push_i(buf, base_y - layout.row_h / 2 + 4);
    push_b(buf, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#374151\">");
    escape_xml(buf, truncate(name, 14));
    push_b(buf, b"</text>");
}

pub fn project_pts(p: &Prepared, ys: &[f64], base_y: i32) -> Vec<(f64,f64)> {
    p.xs.iter().zip(ys.iter()).map(|(&x, &y)| {
        let sx = p.layout.pad_l as f64 + (x - p.x0) / p.xr * p.layout.plot_w as f64;
        let sy = base_y as f64 - y / p.global_max * p.layout.curve_h as f64;
        (sx, sy)
    }).collect()
}

pub fn close_svg(buf: &mut Vec<u8>, cfg: &RidgelineConfig, p: &Prepared, draw_legend: bool) {
    if draw_legend && p.group_order.len() > 1 {
        let legend_x = p.layout.pad_l + p.layout.plot_w + 8;
        for (gi, name) in p.group_order.iter().enumerate() {
            svg_legend_item(buf, gi as i32, name, palette_color(cfg.palette, gi), legend_x, p.layout.title_h + 10 + gi as i32 * 22, 12);
        }
    }
    svg_x_label(buf, cfg.x_label, p.layout.pad_l + p.layout.plot_w / 2, cfg.height - 4);
    svg_y_label(buf, cfg.y_label, 14, p.layout.title_h, p.layout.plot_area_h);
    push_b(buf, b"</svg>");
}

pub fn finalize(buf: Vec<u8>, cfg: &RidgelineConfig) -> String {
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

pub fn area_path(buf: &mut Vec<u8>, pts: &[(f64,f64)], base_y: f64) {
    push_b(buf, b"<path d=\"M");
    push_f2(buf, pts[0].0); push_b(buf, b","); push_f2(buf, base_y);
    push_b(buf, b" L"); push_f2(buf, pts[0].0); push_b(buf, b","); push_f2(buf, pts[0].1);
    for &(sx, sy) in pts.iter().skip(1) {
        push_b(buf, b" L"); push_f2(buf, sx); push_b(buf, b","); push_f2(buf, sy);
    }
    let last = pts.last().unwrap();
    push_b(buf, b" L"); push_f2(buf, last.0); push_b(buf, b","); push_f2(buf, base_y);
    push_b(buf, b" Z\"");
}

pub fn polyline(buf: &mut Vec<u8>, pts: &[(f64,f64)], hex: &[u8;6], stroke_w: f64) {
    push_b(buf, b"<polyline points=\"");
    for (i, &(sx, sy)) in pts.iter().enumerate() {
        if i > 0 { buf.push(b' '); }
        push_f2(buf, sx); buf.push(b','); push_f2(buf, sy);
    }
    push_b(buf, b"\" fill=\"none\" stroke=\"#"); buf.extend_from_slice(hex);
    push_b(buf, b"\" stroke-width=\""); push_f2(buf, stroke_w);
    push_b(buf, b"\" stroke-linejoin=\"round\"/>");
}

pub fn percentile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() { return 0.0; }
    if sorted.len() == 1 { return sorted[0]; }
    let pos = q * (sorted.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = (lo + 1).min(sorted.len() - 1);
    let frac = pos - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

pub fn x_to_px(p: &Prepared, x: f64) -> f64 {
    p.layout.pad_l as f64 + (x - p.x0) / p.xr * p.layout.plot_w as f64
}

pub fn y_at_x(p: &Prepared, ys: &[f64], x: f64, base_y: i32) -> f64 {
    if x <= p.x0 { return base_y as f64 - ys[0] / p.global_max * p.layout.curve_h as f64; }
    if x >= p.x1 { return base_y as f64 - ys[ys.len()-1] / p.global_max * p.layout.curve_h as f64; }
    let pos = (x - p.x0) / p.xr * (ys.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = (lo + 1).min(ys.len() - 1);
    let frac = pos - lo as f64;
    let y = ys[lo] * (1.0 - frac) + ys[hi] * frac;
    base_y as f64 - y / p.global_max * p.layout.curve_h as f64
}
