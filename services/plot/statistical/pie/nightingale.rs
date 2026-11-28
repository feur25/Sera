use super::common::{open_svg, write_title};
use super::config::PieConfig;
use crate::plot::statistical::common::{
    apply_sort, escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

#[crate::chart_demo(
    "labels=[\"1\",\"2\",\"3\",\"4\",\"5\",\"6\",\"7\",\"8\"], values=[1.0,0.802,0.853,0.879,0.892,0.886,0.910,0.966], gridlines=True, variant=\"nightingale\""
)]

pub fn render(cfg: &PieConfig) -> String {
    let (labels, values) = apply_sort(cfg.labels, cfg.values, cfg.sort_order);
    let n = labels.len().min(values.len());
    if n == 0 {
        return String::new();
    }
    let vmax = values[..n].iter().cloned().fold(0.0_f64, f64::max).max(1e-9);

    let w = cfg.width;
    let h = cfg.height;
    let title_pad = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let mut buf = Vec::<u8>::with_capacity(n * 280 + 2048);
    open_svg(&mut buf, w, h);
    write_title(&mut buf, w, cfg.title);

    let cx = w as f64 / 2.0;
    let cy = title_pad + (h as f64 - title_pad) / 2.0;
    let r_max = ((w.min(h) as f64) * 0.5 - 56.0 - title_pad * 0.3).max(30.0);

    for ring in 1..=5 {
        let rr = r_max * ring as f64 / 5.0;
        push_b(&mut buf, b"<circle class=\"sp-gl\" cx=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, rr);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#e2e8f0\" stroke-width=\"0.7\"");
        if !cfg.gridlines {
            push_b(&mut buf, b" style=\"display:none\"");
        }
        push_b(&mut buf, b"/>");
        push_b(&mut buf, b"<text class=\"sp-gl\" x=\"");
        push_f2(&mut buf, cx + 4.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy - rr - 2.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#94a3b8\"");
        if !cfg.gridlines {
            push_b(&mut buf, b" style=\"display:none\"");
        }
        push_b(&mut buf, b">");
        push_f2(&mut buf, vmax * ring as f64 / 5.0);
        push_b(&mut buf, b"</text>");
    }

    let slice = TAU / n as f64;
    for i in 0..n {
        let a0 = -FRAC_PI_2 + slice * i as f64;
        let a1 = a0 + slice;
        let r = r_max * (values[i] / vmax).clamp(0.0, 1.0);
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        let x0 = cx + r * a0.cos();
        let y0 = cy + r * a0.sin();
        let x1 = cx + r * a1.cos();
        let y1 = cy + r * a1.sin();
        let large: u8 = if slice > PI { 1 } else { 0 };

        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" data-lbl=\"");
        escape_xml(&mut buf, &labels[i]);
        push_b(&mut buf, b"\" data-v=\"");
        push_f2(&mut buf, values[i]);
        push_b(&mut buf, b"\" d=\"M");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b",");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b" L");
        push_f2(&mut buf, x0);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y0);
        push_b(&mut buf, b" A");
        push_f2(&mut buf, r);
        push_b(&mut buf, b",");
        push_f2(&mut buf, r);
        push_b(&mut buf, b" 0 ");
        buf.push(large + b'0');
        push_b(&mut buf, b",1 ");
        push_f2(&mut buf, x1);
        push_b(&mut buf, b",");
        push_f2(&mut buf, y1);
        push_b(&mut buf, b" Z\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" fill-opacity=\"0.85\" stroke=\"#1e293b\" stroke-width=\"1.2\"/>");

        let mid = (a0 + a1) / 2.0;
        let lr = r_max + 16.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx + lr * mid.cos());
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + lr * mid.sin() + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, truncate(&labels[i], 10));
        push_b(&mut buf, b"</text>");

        if cfg.show_pct {
            let vr = r + 12.0;
            push_b(&mut buf, b"<text class=\"sp-val\" x=\"");
            push_f2(&mut buf, cx + vr * mid.cos());
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy + vr * mid.sin() + 3.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#475569\">");
            push_f2(&mut buf, values[i]);
            push_b(&mut buf, b"</text>");
        }
    }

    push_b(&mut buf, b"</svg>");
    unsafe { String::from_utf8_unchecked(buf) }
}
