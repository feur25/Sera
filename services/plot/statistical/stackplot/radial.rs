use super::common::{finalize, prepare};
use super::config::StackplotConfig;
use crate::plot::statistical::common::{
    escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate, Frame,
};

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\",\"Jul\",\"Aug\"], series=[[10,14,12,18,20,16,13,17],[8,9,11,10,13,12,9,10],[5,6,7,9,8,7,6,8]], series_names=[\"A\",\"B\",\"C\"]"
)]
pub fn render(cfg: &StackplotConfig) -> String {
    let p = match prepare(cfg, false) {
        Some(v) => v,
        None => return String::new(),
    };

    let legend_w: i32 = 160;
    let mut f = Frame::new_html(cfg.title, cfg.width, cfg.height, 20, 42, 20, legend_w, p.n_pts * p.n_ser * 40 + 4096);
    f.open(cfg.title, false);

    let cx = f.pl + f.pw / 2;
    let cy = f.pt + f.ph / 2;
    let r_max = (f.pw.min(f.ph) as f64 / 2.0 - 14.0).max(10.0);
    let r_min = r_max * 0.20;

    push_b(&mut f.buf, b"<defs>");
    for si in 0..p.n_ser {
        let c = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<radialGradient id=\"spRadG");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" cx=\"50%\" cy=\"50%\" r=\"65%\">");
        push_b(&mut f.buf, b"<stop offset=\"0%\" stop-color=\"#");
        f.buf.extend_from_slice(&c);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.45\"/>");
        push_b(&mut f.buf, b"<stop offset=\"100%\" stop-color=\"#");
        f.buf.extend_from_slice(&c);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.92\"/>");
        push_b(&mut f.buf, b"</radialGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    let ring_n = 4;
    for ri in 0..=ring_n {
        let rr = r_min + (r_max - r_min) * (ri as f64 / ring_n as f64);
        push_b(&mut f.buf, b"<circle cx=\"");
        push_i(&mut f.buf, cx);
        push_b(&mut f.buf, b"\" cy=\"");
        push_i(&mut f.buf, cy);
        push_b(&mut f.buf, b"\" r=\"");
        push_f2(&mut f.buf, rr);
        push_b(&mut f.buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.6\" class=\"sp-gl\"/>");
        if ri > 0 {
            let v = p.ymin + (p.ymax - p.ymin) * (ri as f64 / ring_n as f64);
            let label_a = -std::f64::consts::FRAC_PI_2 + 0.34;
            push_b(&mut f.buf, b"<text x=\"");
            push_f2(&mut f.buf, cx as f64 + rr * label_a.cos() + 3.0);
            push_b(&mut f.buf, b"\" y=\"");
            push_f2(&mut f.buf, cy as f64 + rr * label_a.sin() - 2.0);
            push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\" class=\"sp-yt\">");
            push_f2(&mut f.buf, v);
            push_b(&mut f.buf, b"</text>");
        }
    }

    let angle_at = |i: usize| -std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * (i as f64) / (p.n_pts as f64);
    let radius_of = |v: f64| r_min + (v - p.ymin) / (p.ymax - p.ymin).max(1e-12) * (r_max - r_min);

    for i in 0..p.n_pts {
        let a = angle_at(i);
        let (sx, sy) = (cx as f64 + r_min * a.cos(), cy as f64 + r_min * a.sin());
        let (ex, ey) = (cx as f64 + r_max * a.cos(), cy as f64 + r_max * a.sin());
        push_b(&mut f.buf, b"<line x1=\"");
        push_f2(&mut f.buf, sx);
        push_b(&mut f.buf, b"\" y1=\"");
        push_f2(&mut f.buf, sy);
        push_b(&mut f.buf, b"\" x2=\"");
        push_f2(&mut f.buf, ex);
        push_b(&mut f.buf, b"\" y2=\"");
        push_f2(&mut f.buf, ey);
        push_b(&mut f.buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.6\" class=\"sp-gl\"/>");
        let lx = cx as f64 + (r_max + 12.0) * a.cos();
        let ly = cy as f64 + (r_max + 12.0) * a.sin();
        let anchor: &[u8] = if a.cos() > 0.3 {
            b"start"
        } else if a.cos() < -0.3 {
            b"end"
        } else {
            b"middle"
        };
        push_b(&mut f.buf, b"<text x=\"");
        push_f2(&mut f.buf, lx);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, ly + 3.0);
        push_b(&mut f.buf, b"\" text-anchor=\"");
        f.buf.extend_from_slice(anchor);
        push_b(&mut f.buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 10));
        push_b(&mut f.buf, b"</text>");
    }

    for si in 0..p.n_ser {
        let color = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<path data-idx=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" d=\"M");
        for i in 0..p.n_pts {
            let a = angle_at(i);
            let r = radius_of(p.tops[si][i]);
            let x = cx as f64 + r * a.cos();
            let y = cy as f64 + r * a.sin();
            if i > 0 {
                push_b(&mut f.buf, b"L");
            }
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b",");
            push_f2(&mut f.buf, y);
        }
        push_b(&mut f.buf, b"Z M");
        for i in 0..p.n_pts {
            let a = angle_at(i);
            let r = radius_of(p.bottoms[si][i]);
            let x = cx as f64 + r * a.cos();
            let y = cy as f64 + r * a.sin();
            if i > 0 {
                push_b(&mut f.buf, b"L");
            }
            push_f2(&mut f.buf, x);
            push_b(&mut f.buf, b",");
            push_f2(&mut f.buf, y);
        }
        push_b(&mut f.buf, b"Z\" fill=\"url(#spRadG");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b")\" fill-rule=\"evenodd\" stroke=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stroke-width=\"1\" stroke-linejoin=\"round\"/>");
    }

    push_b(&mut f.buf, b"<circle cx=\"");
    push_i(&mut f.buf, cx);
    push_b(&mut f.buf, b"\" cy=\"");
    push_i(&mut f.buf, cy);
    push_b(&mut f.buf, b"\" r=\"");
    push_f2(&mut f.buf, r_min - 2.0);
    push_b(&mut f.buf, b"\" fill=\"#ffffff\" stroke=\"#e2e8f0\" stroke-width=\"1\"/>");

    let leg_x = cfg.width - 146;
    for (si, (name, _)) in cfg.series.iter().enumerate() {
        crate::plot::statistical::common::svg_legend_item(
            &mut f.buf,
            si as i32,
            name,
            palette_color(cfg.palette, si),
            leg_x,
            f.pt + 6 + si as i32 * 18,
            18,
        );
    }

    finalize(f, cfg)
}
