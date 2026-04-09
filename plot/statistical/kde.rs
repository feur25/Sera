use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open, svg_title, svg_axis_lines, svg_y_label, svg_x_label, svg_hgrid, svg_tick_y, truncate};
use crate::html::hover::build_chart_html;

pub struct KdeConfig<'a> {
    pub title: &'a str,
    pub series: &'a [(String, Vec<f64>)],
    pub palette: &'a [u32],
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub bandwidth: f64,
    pub filled: bool,
    pub fill_opacity: u8,
    pub gridlines: bool,
    pub width: i32,
    pub height: i32,
    pub n_points: usize,
}

impl<'a> Default for KdeConfig<'a> {
    fn default() -> Self {
        Self {
            title: "",
            series: &[],
            palette: &[],
            x_label: "",
            y_label: "Density",
            bandwidth: 0.0,
            filled: true,
            fill_opacity: 50,
            gridlines: true,
            width: 900,
            height: 420,
            n_points: 200,
        }
    }
}

pub fn scott_bw(vals: &[f64]) -> f64 {
    let n = vals.len();
    if n < 2 { return 1.0; }
    let mean = vals.iter().sum::<f64>() / n as f64;
    let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
    1.06 * var.sqrt() * (n as f64).powf(-0.2)
}

pub fn kde_eval(vals: &[f64], x: f64, bw: f64) -> f64 {
    let c = 1.0 / (bw * (2.0 * std::f64::consts::PI).sqrt() * vals.len() as f64);
    vals.iter().map(|&v| { let u = (x - v) / bw; c * (-0.5 * u * u).exp() }).sum()
}

pub fn render_kde_html(cfg: &KdeConfig) -> String {
    let n_ser = cfg.series.len();
    if n_ser == 0 { return String::new(); }

    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let pad_l: i32 = 56;
    let pad_t: i32 = 42;
    let pad_b: i32 = 52;
    let plot_w = cfg.width - pad_l - legend_w;
    let plot_h = cfg.height - pad_t - pad_b;
    let n_pts = cfg.n_points.max(80);

    let all_vals: Vec<f64> = cfg.series.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if all_vals.is_empty() { return String::new(); }

    let x_min = all_vals.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = all_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let x_range = (x_max - x_min).max(1e-12);
    let x_pad = x_range * 0.12;
    let x0 = x_min - x_pad;
    let x1 = x_max + x_pad;
    let xr = x1 - x0;

    let xs: Vec<f64> = (0..n_pts).map(|i| x0 + xr * i as f64 / (n_pts - 1) as f64).collect();

    let curves: Vec<Vec<f64>> = cfg.series.iter().map(|(_, vals)| {
        let bw = if cfg.bandwidth > 0.0 { cfg.bandwidth } else { scott_bw(vals).max(1e-12) };
        xs.iter().map(|&x| kde_eval(vals, x, bw)).collect()
    }).collect();

    let y_max = curves.iter().flat_map(|c| c.iter().copied()).fold(0.0_f64, f64::max).max(1e-12);

    let mut b = Vec::<u8>::with_capacity(n_ser * n_pts * 24 + 2048);
    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, (cfg.width - legend_w) / 2 + pad_l, 26);

    let n_yticks: i32 = 5;
    if cfg.gridlines {
        for ti in 0..=n_yticks {
            let y = pad_t + plot_h - (plot_h as f64 * ti as f64 / n_yticks as f64) as i32;
            svg_hgrid(&mut b, pad_l, pad_l + plot_w, y);
        }
    }
    svg_y_label(&mut b, cfg.y_label, 14, pad_t, plot_h);
    for ti in 0..=n_yticks {
        let frac = ti as f64 / n_yticks as f64;
        let y = pad_t + plot_h - (plot_h as f64 * frac) as i32;
        svg_tick_y(&mut b, pad_l - 4, y + 4, frac * y_max);
    }
    svg_axis_lines(&mut b, pad_l, pad_t, plot_w, plot_h);

    let base_y = (pad_t + plot_h) as f64;
    for (si, ys) in curves.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        let pts: Vec<(f64, f64)> = xs.iter().zip(ys.iter()).map(|(&x, &y)| {
            let sx = pad_l as f64 + (x - x0) / xr * plot_w as f64;
            let sy = pad_t as f64 + plot_h as f64 - y / y_max * plot_h as f64;
            (sx, sy)
        }).collect();
        if cfg.filled {
            push_b(&mut b, b"<path d=\"M");
            push_f2(&mut b, pts[0].0); push_b(&mut b, b","); push_f2(&mut b, base_y);
            push_b(&mut b, b" L"); push_f2(&mut b, pts[0].0); push_b(&mut b, b","); push_f2(&mut b, pts[0].1);
            for &(sx, sy) in pts.iter().skip(1) {
                push_b(&mut b, b" L"); push_f2(&mut b, sx); push_b(&mut b, b","); push_f2(&mut b, sy);
            }
            let last = pts.last().unwrap();
            push_b(&mut b, b" L"); push_f2(&mut b, last.0); push_b(&mut b, b","); push_f2(&mut b, base_y);
            push_b(&mut b, b" Z\" fill=\"#"); b.extend_from_slice(&hx);
            let op = cfg.fill_opacity as f64 / 255.0;
            push_b(&mut b, b"\" fill-opacity=\""); push_f2(&mut b, op); push_b(&mut b, b"\"/>");
        }
        push_b(&mut b, b"<polyline points=\"");
        for (i, &(sx, sy)) in pts.iter().enumerate() {
            if i > 0 { b.push(b' '); }
            push_f2(&mut b, sx); b.push(b','); push_f2(&mut b, sy);
        }
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\"/>");
    }

    let n_xticks: i32 = 6;
    for ti in 0..=n_xticks {
        let frac = ti as f64 / n_xticks as f64;
        let x = pad_l + (plot_w as f64 * frac) as i32;
        let val = x0 + xr * frac;
        push_b(&mut b, b"<text x=\""); push_i(&mut b, x);
        push_b(&mut b, b"\" y=\""); push_i(&mut b, pad_t + plot_h + 14);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#9ca3af\">");
        push_f2(&mut b, val);
        push_b(&mut b, b"</text>");
    }
    svg_x_label(&mut b, cfg.x_label, pad_l + plot_w / 2, cfg.height - 4);

    if n_ser > 1 {
        let lx = cfg.width - legend_w + 12;
        for (si, (sname, _)) in cfg.series.iter().enumerate() {
            let color = palette_color(cfg.palette, si);
            let hx = hex6(color);
            let ly = pad_t + si as i32 * 22;
            push_b(&mut b, b"<rect x=\""); push_i(&mut b, lx);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, ly - 3);
            push_b(&mut b, b"\" width=\"14\" height=\"3\" rx=\"1\" fill=\"#"); b.extend_from_slice(&hx); push_b(&mut b, b"\"/>");
            push_b(&mut b, b"<text x=\""); push_i(&mut b, lx + 18);
            push_b(&mut b, b"\" y=\""); push_i(&mut b, ly + 4);
            push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
            escape_xml(&mut b, truncate(sname, 14));
            push_b(&mut b, b"</text>");
        }
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, "[]")
}
