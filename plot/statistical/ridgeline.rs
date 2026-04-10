use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open, svg_title, svg_x_label, truncate};
use crate::html::hover::build_chart_html;
use super::kde::{scott_bw, kde_eval};

pub struct RidgelineConfig<'a> {
    pub title: &'a str,
    pub values: &'a [f64],
    pub categories: &'a [String],
    pub palette: &'a [u32],
    pub x_label: &'a str,
    pub overlap: f64,
    pub bandwidth: f64,
    pub width: i32,
    pub height: i32,
    pub n_points: usize,
}

impl<'a> Default for RidgelineConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            values: &[],
            categories: &[],
            palette: &[],
            x_label: "",
            overlap: 0.5,
            bandwidth: 0.0,
            width: 900,
            height: 520,
            n_points: 60,
        }
    }
}

pub fn render_ridgeline_html(cfg: &RidgelineConfig) -> String {
    let n = cfg.values.len().min(cfg.categories.len());
    if n == 0 { return String::new(); }

    let mut group_order: Vec<String> = Vec::new();
    for cat in cfg.categories[..n].iter() {
        if !group_order.contains(cat) { group_order.push(cat.clone()); }
    }
    let n_groups = group_order.len();
    if n_groups == 0 { return String::new(); }

    let mut group_vals: Vec<Vec<f64>> = vec![Vec::new(); n_groups];
    for i in 0..n {
        if let Some(gi) = group_order.iter().position(|g| g == &cfg.categories[i]) {
            group_vals[gi].push(cfg.values[i]);
        }
    }

    let all_vals: Vec<f64> = cfg.values[..n].to_vec();
    let x_min = all_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let x_range = (x_max - x_min).max(1e-12);
    let x_pad = x_range * 0.12;
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
    let pad_r: i32 = 20;
    let plot_w = cfg.width - pad_l - pad_r;
    let plot_area_h = cfg.height - title_h - pad_b;
    let row_h = plot_area_h / n_groups as i32;
    let curve_h = (row_h as f64 * (1.0 + cfg.overlap.clamp(0.0, 2.0))) as i32;

    let mut b = Vec::<u8>::with_capacity(n_groups * n_pts * 24 + 2048);
    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, cfg.width / 2, if title_h > 0 { 24 } else { 0 });

    let n_xticks: i32 = 6;
    let axis_y = title_h + plot_area_h;
    push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l);
    push_b(&mut b, b"\" y1=\""); push_i(&mut b, axis_y);
    push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
    push_b(&mut b, b"\" y2=\""); push_i(&mut b, axis_y);
    push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\" class=\"sp-ax-x\"/>");

    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = pad_l + (plot_w as f64 * frac) as i32;
        let val = x0 + xr * frac;
        push_b(&mut b, b"<line x1=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, title_h);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, axis_y);
        push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\" class=\"sp-gl\"/>");
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, axis_y + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#9ca3af\" class=\"sp-xt\">");
        push_f2(&mut b, val);
        push_b(&mut b, b"</text>");
    }

    for (gi, ys) in curves.iter().enumerate().rev() {
        let color = palette_color(cfg.palette, gi);
        let hx = hex6(color);
        let base_y = title_h + (gi + 1) as i32 * row_h;

        let pts: Vec<(f64, f64)> = xs.iter().zip(ys.iter()).map(|(&x, &y)| {
            let sx = pad_l as f64 + (x - x0) / xr * plot_w as f64;
            let sy = base_y as f64 - y / global_max * curve_h as f64;
            (sx, sy)
        }).collect();

        push_b(&mut b, b"<path d=\"M");
        push_f2(&mut b, pts[0].0); push_b(&mut b, b","); push_f2(&mut b, base_y as f64);
        push_b(&mut b, b" L"); push_f2(&mut b, pts[0].0); push_b(&mut b, b","); push_f2(&mut b, pts[0].1);
        for &(sx, sy) in pts.iter().skip(1) {
            push_b(&mut b, b" L"); push_f2(&mut b, sx); push_b(&mut b, b","); push_f2(&mut b, sy);
        }
        let last = pts.last().unwrap();
        push_b(&mut b, b" L"); push_f2(&mut b, last.0); push_b(&mut b, b","); push_f2(&mut b, base_y as f64);
        push_b(&mut b, b" Z\" fill=\"#ffffff\"/>");

        push_b(&mut b, b"<path d=\"M");
        push_f2(&mut b, pts[0].0); push_b(&mut b, b","); push_f2(&mut b, base_y as f64);
        push_b(&mut b, b" L"); push_f2(&mut b, pts[0].0); push_b(&mut b, b","); push_f2(&mut b, pts[0].1);
        for &(sx, sy) in pts.iter().skip(1) {
            push_b(&mut b, b" L"); push_f2(&mut b, sx); push_b(&mut b, b","); push_f2(&mut b, sy);
        }
        push_b(&mut b, b" L"); push_f2(&mut b, last.0); push_b(&mut b, b","); push_f2(&mut b, base_y as f64);
        push_b(&mut b, b" Z\" fill=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\"0.22\"/>");

        push_b(&mut b, b"<polyline points=\"");
        for (i, &(sx, sy)) in pts.iter().enumerate() {
            if i > 0 { b.push(b' '); }
            push_f2(&mut b, sx); b.push(b','); push_f2(&mut b, sy);
        }
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\"/>");

        push_b(&mut b, b"<line x1=\""); push_i(&mut b, pad_l - 4);
        push_b(&mut b, b"\" y1=\""); push_i(&mut b, base_y);
        push_b(&mut b, b"\" x2=\""); push_i(&mut b, pad_l + plot_w);
        push_b(&mut b, b"\" y2=\""); push_i(&mut b, base_y);
        push_b(&mut b, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.6\"/>");

        push_b(&mut b, b"<text x=\""); push_i(&mut b, pad_l - 8);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, base_y - row_h / 2 + 4);
        push_b(&mut b, b"\" text-anchor=\"end\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#374151\">");
        escape_xml(&mut b, truncate(&group_order[gi], 14));
        push_b(&mut b, b"</text>");
    }

    svg_x_label(&mut b, cfg.x_label, pad_l + plot_w / 2, cfg.height - 4);
    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
