use super::config::RadarConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, push_f2, escape_xml, svg_open, svg_title, svg_legend_item};
use crate::html::hover::{build_chart_html, slots_to_json};
use std::f64::consts::PI;

pub struct Layout {
    pub legend_w: i32,
    pub title_h: i32,
    pub plot_cx: f64,
    pub plot_cy: f64,
    pub r: f64,
}

pub struct Prepared {
    pub n_axes: usize,
    pub n_ser: usize,
    pub global_max: f64,
    pub layout: Layout,
}

pub fn angle_at(ai: usize, n_axes: usize) -> f64 {
    PI / 2.0 - 2.0 * PI * ai as f64 / n_axes as f64
}

pub fn project(cx: f64, cy: f64, r: f64, frac: f64, angle: f64) -> (f64, f64) {
    (cx + r * frac * angle.cos(), cy - r * frac * angle.sin())
}

pub fn prepare(cfg: &RadarConfig) -> Option<Prepared> {
    let n_axes = cfg.axes.len();
    let n_ser = cfg.series.len();
    if n_axes < 3 || n_ser == 0 { return None; }

    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let title_h: i32 = if cfg.title.is_empty() { 0 } else { 34 };
    let plot_cx = ((cfg.width - legend_w) / 2) as f64;
    let plot_cy = (title_h + (cfg.height - title_h) / 2) as f64;
    let r = (((cfg.width - legend_w) / 2 - 64) as f64)
        .min(((cfg.height - title_h) / 2 - 44) as f64)
        .max(20.0);

    let global_max = cfg.series.iter()
        .flat_map(|(_, v)| v.iter().copied())
        .fold(0.0_f64, f64::max)
        .max(1.0);

    Some(Prepared {
        n_axes, n_ser, global_max,
        layout: Layout { legend_w, title_h, plot_cx, plot_cy, r },
    })
}

pub fn series_points(cfg: &RadarConfig, p: &Prepared, si: usize) -> Vec<(f64, f64)> {
    let vals = &cfg.series[si].1;
    (0..p.n_axes).map(|ai| {
        let v = vals.get(ai).copied().unwrap_or(0.0).max(0.0);
        let frac = (v / p.global_max).min(1.0);
        let a = angle_at(ai, p.n_axes);
        project(p.layout.plot_cx, p.layout.plot_cy, p.layout.r, frac, a)
    }).collect()
}

pub fn open_svg(buf: &mut Vec<u8>, cfg: &RadarConfig, p: &Prepared) {
    svg_open(buf, cfg.width, cfg.height);
    svg_title(buf, cfg.title, p.layout.plot_cx as i32, if p.layout.title_h > 0 { 24 } else { 0 });
}

pub fn draw_grid(buf: &mut Vec<u8>, cfg: &RadarConfig, p: &Prepared) {
    let n_rings = 5;
    let label_pad = 24.0;
    let cx = p.layout.plot_cx;
    let cy = p.layout.plot_cy;
    let r = p.layout.r;

    for ring in 1..=n_rings {
        let frac = ring as f64 / n_rings as f64;
        let rr = r * frac;
        push_b(buf, b"<polygon points=\"");
        for ai in 0..p.n_axes {
            let a = angle_at(ai, p.n_axes);
            if ai > 0 { buf.push(b' '); }
            push_f2(buf, cx + rr * a.cos());
            buf.push(b',');
            push_f2(buf, cy - rr * a.sin());
        }
        push_b(buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.9\" class=\"sp-gl\"/>");
        let lx = cx + 4.0;
        let ly = cy - rr - 2.0;
        push_b(buf, b"<text x=\""); push_f2(buf, lx);
        push_b(buf, b"\" y=\""); push_f2(buf, ly);
        push_b(buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        let val = frac * p.global_max;
        if val.fract() == 0.0 { push_i(buf, val as i32); } else { push_f2(buf, val); }
        push_b(buf, b"</text>");
    }

    for ai in 0..p.n_axes {
        let a = angle_at(ai, p.n_axes);
        let epx = cx + r * a.cos();
        let epy = cy - r * a.sin();
        push_b(buf, b"<line x1=\""); push_f2(buf, cx);
        push_b(buf, b"\" y1=\""); push_f2(buf, cy);
        push_b(buf, b"\" x2=\""); push_f2(buf, epx);
        push_b(buf, b"\" y2=\""); push_f2(buf, epy);
        push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" class=\"sp-ax-y\"/>");
        let anchor = if a.cos().abs() < 0.12 { "middle" } else if a.cos() > 0.0 { "start" } else { "end" };
        let lx = cx + (r + label_pad) * a.cos();
        let ly = cy - (r + label_pad) * a.sin();
        let dy = if a.sin() > 0.3 { -4.0 } else if a.sin() < -0.3 { 12.0 } else { 4.0 };
        push_b(buf, b"<text x=\""); push_f2(buf, lx);
        push_b(buf, b"\" y=\""); push_f2(buf, ly + dy);
        push_b(buf, b"\" text-anchor=\""); buf.extend_from_slice(anchor.as_bytes());
        push_b(buf, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#374151\">");
        escape_xml(buf, &cfg.axes[ai]);
        push_b(buf, b"</text>");
    }
}

pub fn polygon_pts(buf: &mut Vec<u8>, pts: &[(f64, f64)]) {
    push_b(buf, b"<polygon points=\"");
    for (i, &(x, y)) in pts.iter().enumerate() {
        if i > 0 { buf.push(b' '); }
        push_f2(buf, x); buf.push(b','); push_f2(buf, y);
    }
    push_b(buf, b"\"");
}

pub fn dots(buf: &mut Vec<u8>, pts: &[(f64, f64)], hx: &[u8], r: f64) {
    for &(px, py) in pts {
        push_b(buf, b"<circle cx=\""); push_f2(buf, px);
        push_b(buf, b"\" cy=\""); push_f2(buf, py);
        push_b(buf, b"\" r=\""); push_f2(buf, r);
        push_b(buf, b"\" fill=\"#"); buf.extend_from_slice(hx);
        push_b(buf, b"\"/>");
    }
}

pub fn dots_outlined(buf: &mut Vec<u8>, pts: &[(f64, f64)], hx: &[u8], r: f64) {
    for &(px, py) in pts {
        push_b(buf, b"<circle cx=\""); push_f2(buf, px);
        push_b(buf, b"\" cy=\""); push_f2(buf, py);
        push_b(buf, b"\" r=\""); push_f2(buf, r);
        push_b(buf, b"\" fill=\"#ffffff\" stroke=\"#"); buf.extend_from_slice(hx);
        push_b(buf, b"\" stroke-width=\"2\"/>");
    }
}

pub fn draw_legend(buf: &mut Vec<u8>, cfg: &RadarConfig, p: &Prepared) {
    if p.n_ser <= 1 { return; }
    let lx = cfg.width - p.layout.legend_w + 10;
    let lt = (cfg.height / 2 - p.n_ser as i32 * 11).max(p.layout.title_h + 10);
    for (si, (sname, _)) in cfg.series.iter().enumerate() {
        svg_legend_item(buf, si as i32, sname, palette_color(cfg.palette, si), lx, lt + si as i32 * 24, 16);
    }
}

pub fn finalize(mut buf: Vec<u8>, cfg: &RadarConfig) -> String {
    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
