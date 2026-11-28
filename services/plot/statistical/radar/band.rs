use super::common::{angle_at, draw_grid, finalize, open_svg, prepare, project};
use super::config::RadarConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i, svg_legend_item};

#[crate::chart_demo(
    "axes=[\"Speed\",\"Power\",\"Range\",\"Cost\",\"Style\"], series=[[55,45,50,25,50],[80,70,75,45,78],[30,60,35,50,40],[50,85,55,70,65]], series_names=[\"A low\",\"A high\",\"B low\",\"B high\"], variant=\"band\""
)]

pub fn render(cfg: &RadarConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n_pairs = p.n_ser / 2;
    if n_pairs == 0 {
        return String::new();
    }
    let mut b = Vec::<u8>::with_capacity(4096 + n_pairs * p.n_axes * 100);
    open_svg(&mut b, cfg, &p);
    draw_grid(&mut b, cfg, &p);
    let mut group_names: Vec<String> = Vec::with_capacity(n_pairs);

    let pt_at = |vals: &[f64], ai: usize| -> (f64, f64) {
        let v = vals.get(ai).copied().unwrap_or(0.0).max(0.0);
        let frac = (v / p.global_max).min(1.0);
        let a = angle_at(ai, p.n_axes);
        project(p.layout.plot_cx, p.layout.plot_cy, p.layout.r, frac, a)
    };

    for g in 0..n_pairs {
        let (name_raw, low) = &cfg.series[2 * g];
        let (_, high) = &cfg.series[2 * g + 1];
        let name = name_raw
            .trim_end_matches(|c: char| c.is_whitespace())
            .trim_end_matches("_low")
            .trim_end_matches(" low")
            .trim_end_matches("Low")
            .to_string();
        group_names.push(name.clone());
        let color = palette_color(cfg.palette, g);
        let hx = hex6(color);

        let high_pts: Vec<(f64, f64)> = (0..p.n_axes).map(|ai| pt_at(high, ai)).collect();
        let low_pts: Vec<(f64, f64)> = (0..p.n_axes).map(|ai| pt_at(low, ai)).collect();

        push_b(&mut b, b"<g data-series=\"");
        push_i(&mut b, g as i32);
        push_b(&mut b, b"\" data-idx=\"");
        push_i(&mut b, g as i32);
        push_b(&mut b, b"\" data-lbl=\"");
        crate::plot::statistical::common::escape_xml(&mut b, &name);
        push_b(&mut b, b"\">");

        push_b(&mut b, b"<path d=\"M ");
        for (i, &(x, y)) in high_pts.iter().enumerate() {
            if i > 0 {
                push_b(&mut b, b" L ");
            }
            push_f2(&mut b, x);
            b.push(b',');
            push_f2(&mut b, y);
        }
        push_b(&mut b, b" L ");
        for (x, y) in low_pts.iter().rev() {
            push_f2(&mut b, *x);
            b.push(b',');
            push_f2(&mut b, *y);
            push_b(&mut b, b" L ");
        }
        push_f2(&mut b, high_pts[0].0);
        b.push(b',');
        push_f2(&mut b, high_pts[0].1);
        push_b(&mut b, b" Z\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" fill-opacity=\"0.22\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1\" stroke-opacity=\"0.5\"/>");

        let mid_pts: Vec<(f64, f64)> = (0..p.n_axes)
            .map(|ai| {
                let (hx_, hy_) = high_pts[ai];
                let (lx_, ly_) = low_pts[ai];
                ((hx_ + lx_) / 2.0, (hy_ + ly_) / 2.0)
            })
            .collect();
        push_b(&mut b, b"<polygon points=\"");
        for (i, &(x, y)) in mid_pts.iter().enumerate() {
            if i > 0 {
                b.push(b' ');
            }
            push_f2(&mut b, x);
            b.push(b',');
            push_f2(&mut b, y);
        }
        push_b(
            &mut b,
            b"\" fill=\"none\" stroke=\"#",
        );
        b.extend_from_slice(&hx);
        push_b(
            &mut b,
            b"\" stroke-width=\"2\" stroke-dasharray=\"5,3\"/>",
        );
        for &(x, y) in &mid_pts {
            push_b(&mut b, b"<circle cx=\"");
            push_f2(&mut b, x);
            push_b(&mut b, b"\" cy=\"");
            push_f2(&mut b, y);
            push_b(&mut b, b"\" r=\"3\" fill=\"#");
            b.extend_from_slice(&hx);
            push_b(&mut b, b"\" stroke=\"#fff\" stroke-width=\"1.2\"/>");
        }
        push_b(&mut b, b"</g>");
    }

    if n_pairs > 1 {
        let lx = cfg.width - p.layout.legend_w + 10;
        let lt = (cfg.height / 2 - n_pairs as i32 * 11).max(p.layout.title_h + 10);
        for (g, name) in group_names.iter().enumerate() {
            svg_legend_item(
                &mut b,
                g as i32,
                name,
                palette_color(cfg.palette, g),
                lx,
                lt + g as i32 * 24,
                16,
            );
        }
    }
    finalize(b, cfg)
}
