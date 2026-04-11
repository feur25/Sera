use super::common::{palette_color, push_b, push_i, push_f2, escape_xml, hex6, svg_open, svg_title, svg_legend_item, truncate};
use crate::html::hover::{build_chart_html, slots_to_json};
use std::f64::consts::PI;

crate::chart_config!(RadarConfig, 700, 560;
    struct {
        pub axes: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub filled: bool,
        pub fill_opacity: u8,
    }
    defaults {
        axes: &[],
        series: &[],
        palette: &[],
        filled: true,
        fill_opacity: 50,
    }
);

pub fn render_radar_html(cfg: &RadarConfig) -> String {
    let n_axes = cfg.axes.len();
    let n_ser = cfg.series.len();
    if n_axes < 3 || n_ser == 0 { return String::new(); }

    let legend_w: i32 = if n_ser > 1 { 140 } else { 20 };
    let title_h: i32 = if cfg.title.is_empty() { 0 } else { 34 };
    let plot_cx = (cfg.width - legend_w) / 2;
    let plot_cy = title_h + (cfg.height - title_h) / 2;
    let label_pad: f64 = 24.0;
    let r = (((cfg.width - legend_w) / 2 - 64) as f64)
        .min(((cfg.height - title_h) / 2 - 44) as f64)
        .max(20.0);

    let global_max = cfg.series.iter()
        .flat_map(|(_, v)| v.iter().copied())
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let n_rings = 5;
    let mut b = Vec::<u8>::with_capacity(4096 + n_ser * n_axes * 80);
    svg_open(&mut b, cfg.width, cfg.height);
    svg_title(&mut b, cfg.title, plot_cx, if title_h > 0 { 24 } else { 0 });

    for ring in 1..=n_rings {
        let frac = ring as f64 / n_rings as f64;
        let rr = r * frac;
        push_b(&mut b, b"<polygon points=\"");
        for ai in 0..n_axes {
            let angle = PI / 2.0 - 2.0 * PI * ai as f64 / n_axes as f64;
            if ai > 0 { b.push(b' '); }
            push_f2(&mut b, plot_cx as f64 + rr * angle.cos());
            b.push(b',');
            push_f2(&mut b, plot_cy as f64 - rr * angle.sin());
        }
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.9\" class=\"sp-gl\"/>");
        let lx = plot_cx as f64 + 4.0;
        let ly = plot_cy as f64 - rr - 2.0;
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly);
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
        let val = frac * global_max;
        if val.fract() == 0.0 { push_i(&mut b, val as i32); } else { push_f2(&mut b, val); }
        push_b(&mut b, b"</text>");
    }

    for ai in 0..n_axes {
        let angle = PI / 2.0 - 2.0 * PI * ai as f64 / n_axes as f64;
        let epx = plot_cx as f64 + r * angle.cos();
        let epy = plot_cy as f64 - r * angle.sin();
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, plot_cx as f64);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, plot_cy as f64);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, epx);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, epy);
        push_b(&mut b, b"\" stroke=\"#cbd5e1\" stroke-width=\"0.8\" class=\"sp-ax-y\"/>");
        let anchor = if angle.cos().abs() < 0.12 { "middle" } else if angle.cos() > 0.0 { "start" } else { "end" };
        let lx = plot_cx as f64 + (r + label_pad) * angle.cos();
        let ly = plot_cy as f64 - (r + label_pad) * angle.sin();
        let dy = if angle.sin() > 0.3 { -4.0 } else if angle.sin() < -0.3 { 12.0 } else { 4.0 };
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, lx);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, ly + dy);
        push_b(&mut b, b"\" text-anchor=\"");
        b.extend_from_slice(anchor.as_bytes());
        push_b(&mut b, b"\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"11\" font-weight=\"600\" fill=\"#374151\">");
        escape_xml(&mut b, &cfg.axes[ai]);
        push_b(&mut b, b"</text>");
    }

    for (si, (_, vals)) in cfg.series.iter().enumerate() {
        let color = palette_color(cfg.palette, si);
        let hx = hex6(color);
        push_b(&mut b, b"<g data-series=\""); push_i(&mut b, si as i32); push_b(&mut b, b"\">");
        let points: Vec<(f64, f64)> = (0..n_axes).map(|ai| {
            let v = vals.get(ai).copied().unwrap_or(0.0).max(0.0);
            let frac = (v / global_max).min(1.0);
            let angle = PI / 2.0 - 2.0 * PI * ai as f64 / n_axes as f64;
            (plot_cx as f64 + r * frac * angle.cos(), plot_cy as f64 - r * frac * angle.sin())
        }).collect();

        if cfg.filled {
            push_b(&mut b, b"<polygon points=\"");
            for (i, &(px, py)) in points.iter().enumerate() {
                if i > 0 { b.push(b' '); }
                push_f2(&mut b, px); b.push(b','); push_f2(&mut b, py);
            }
            push_b(&mut b, b"\" fill=\"#"); b.extend_from_slice(&hx);
            let op = cfg.fill_opacity as f64 / 255.0;
            push_b(&mut b, b"\" fill-opacity=\""); push_f2(&mut b, op);
            push_b(&mut b, b"\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"2\" stroke-linejoin=\"round\"/>");
        } else {
            push_b(&mut b, b"<polygon points=\"");
            for (i, &(px, py)) in points.iter().enumerate() {
                if i > 0 { b.push(b' '); }
                push_f2(&mut b, px); b.push(b','); push_f2(&mut b, py);
            }
            push_b(&mut b, b"\" fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke-width=\"2\"/>");
        }

        for &(px, py) in &points {
            push_b(&mut b, b"<circle cx=\""); push_f2(&mut b, px);
            push_b(&mut b, b"\" cy=\""); push_f2(&mut b, py);
            push_b(&mut b, b"\" r=\"3.5\" fill=\"#"); b.extend_from_slice(&hx);
            push_b(&mut b, b"\"/>");
        }
        push_b(&mut b, b"</g>");
    }

    if n_ser > 1 {
        let lx = cfg.width - legend_w + 10;
        let lt = (cfg.height / 2 - n_ser as i32 * 11).max(title_h + 10);
        for (si, (sname, _)) in cfg.series.iter().enumerate() {
            svg_legend_item(&mut b, si as i32, sname, palette_color(cfg.palette, si), lx, lt + si as i32 * 24, 16);
        }
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}
