use super::common::{bar_data_attrs, finalize, label_left, open_frame, prepare, x_at};
use super::config::GanttConfig;
use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
use crate::plot::statistical::heatmap::common::colorscale_color;

#[crate::chart_demo(
    "labels=[\"Design\",\"Build\",\"Test\",\"Launch\"], start=[0,5,12,18], end=[6,14,19,22], colorscale=\"turbo\""
)]

pub fn render(cfg: &GanttConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let scale = if cfg.colorscale.is_empty() {
        "turbo"
    } else {
        cfg.colorscale
    };
    let durations: Vec<f64> = (0..p.n).map(|i| p.end[i] - p.start[i]).collect();
    let dmin = durations.iter().cloned().fold(f64::INFINITY, f64::min);
    let dmax = durations.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let drange = (dmax - dmin).max(1e-9);

    let mut f = open_frame(cfg, &p);
    let pitch = f.ph as f64 / p.n as f64;
    let bar_h = (pitch * 0.55).max(6.0);
    for i in 0..p.n {
        let cy = f.pt + (i as f64 * pitch + pitch / 2.0) as i32;
        let x1 = x_at(&f, &p, p.start[i]);
        let x2 = x_at(&f, &p, p.end[i]);
        let w = (x2 - x1).max(2);
        let t = (durations[i] - dmin) / drange;
        let color = hex6(colorscale_color(scale, t));
        label_left(&mut f, &p, i, cy);

        push_b(&mut f.buf, b"<rect");
        bar_data_attrs(&mut f, &p, i);
        push_b(&mut f.buf, b" x=\"");
        push_i(&mut f.buf, x1);
        push_b(&mut f.buf, b"\" y=\"");
        push_f2(&mut f.buf, cy as f64 - bar_h / 2.0);
        push_b(&mut f.buf, b"\" width=\"");
        push_i(&mut f.buf, w);
        push_b(&mut f.buf, b"\" height=\"");
        push_f2(&mut f.buf, bar_h);
        push_b(&mut f.buf, b"\" rx=\"3\" fill=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\"/>");

        if bar_h > 10.0 {
            push_b(&mut f.buf, b"<text x=\"");
            push_i(&mut f.buf, x1 + w + 6);
            push_b(&mut f.buf, b"\" y=\"");
            push_i(&mut f.buf, cy + 3);
            push_b(
                &mut f.buf,
                b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">",
            );
            push_f2(&mut f.buf, durations[i]);
            push_b(&mut f.buf, b"</text>");
        }
    }
    finalize(f, cfg)
}
