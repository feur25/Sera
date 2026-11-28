use super::common::{finalize, open_frame, prepare, x_labels_row, y_px};
use super::config::StackplotConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], series=[[10,14,12,18,20,17],[8,9,11,10,13,15],[5,6,7,9,8,11]], series_names=[\"A\",\"B\",\"C\"]"
)]
pub fn render(cfg: &StackplotConfig) -> String {
    let p = match prepare(cfg, false) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open_frame(cfg, &p);
    let step = f.pw as f64 / (p.n_pts - 1).max(1) as f64;
    x_labels_row(&mut f, cfg, &p, step);
    let (pl, pt, ph) = (f.pl, f.pt, f.ph);

    push_b(&mut f.buf, b"<defs>");
    for si in 0..p.n_ser {
        let c = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<linearGradient id=\"spRibG");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">");
        push_b(&mut f.buf, b"<stop offset=\"0%\" stop-color=\"#");
        f.buf.extend_from_slice(&c);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.95\"/>");
        push_b(&mut f.buf, b"<stop offset=\"100%\" stop-color=\"#");
        f.buf.extend_from_slice(&c);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.55\"/>");
        push_b(&mut f.buf, b"</linearGradient>");
        push_b(&mut f.buf, b"<filter id=\"spRibShadow");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"160%\">");
        push_b(&mut f.buf, b"<feDropShadow dx=\"0\" dy=\"3\" stdDeviation=\"3\" flood-color=\"#");
        f.buf.extend_from_slice(&c);
        push_b(&mut f.buf, b"\" flood-opacity=\"0.35\"/>");
        push_b(&mut f.buf, b"</filter>");
    }
    push_b(&mut f.buf, b"</defs>");

    for si in 0..p.n_ser {
        let color = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<path data-idx=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" filter=\"url(#spRibShadow");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b")\" d=\"");
        smooth_area_path(&mut f.buf, &p, si, pl, pt, ph, step);
        push_b(&mut f.buf, b"\" fill=\"url(#spRibG");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b")\" stroke=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stroke-width=\"1.4\" stroke-linejoin=\"round\"/>");
    }

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

fn smooth_area_path(
    buf: &mut Vec<u8>,
    p: &super::common::Prepared,
    si: usize,
    pl: i32,
    pt: i32,
    ph: i32,
    step: f64,
) {
    let top_pts: Vec<(f64, f64)> = (0..p.n_pts)
        .map(|i| (pl as f64 + i as f64 * step, y_px(pt, ph, p.ymin, p.ymax, p.tops[si][i])))
        .collect();
    let bot_pts: Vec<(f64, f64)> = (0..p.n_pts)
        .map(|i| (pl as f64 + i as f64 * step, y_px(pt, ph, p.ymin, p.ymax, p.bottoms[si][i])))
        .collect();

    let (x0, y0) = top_pts[0];
    push_b(buf, b"M");
    push_f2(buf, x0);
    push_b(buf, b",");
    push_f2(buf, y0);
    push_smooth(buf, &top_pts);
    push_b(buf, b"L");
    let last = bot_pts[bot_pts.len() - 1];
    push_f2(buf, last.0);
    push_b(buf, b",");
    push_f2(buf, last.1);
    let mut rev = bot_pts.clone();
    rev.reverse();
    push_smooth(buf, &rev);
    push_b(buf, b"Z");
}

/// Emits smoothed segments continuing from the path's current point (`pts[0]`)
/// through the rest of `pts`: each interior point becomes a quadratic control,
/// curving to the midpoint of itself and its successor, with a final straight
/// segment into the last point.
fn push_smooth(buf: &mut Vec<u8>, pts: &[(f64, f64)]) {
    if pts.len() < 3 {
        for (x, y) in pts.iter().skip(1) {
            push_b(buf, b"L");
            push_f2(buf, *x);
            push_b(buf, b",");
            push_f2(buf, *y);
        }
        return;
    }
    for i in 1..pts.len() - 1 {
        let (cx, cy) = pts[i];
        let (nx, ny) = pts[i + 1];
        let mx = (cx + nx) / 2.0;
        let my = (cy + ny) / 2.0;
        push_b(buf, b"Q");
        push_f2(buf, cx);
        push_b(buf, b",");
        push_f2(buf, cy);
        push_b(buf, b" ");
        push_f2(buf, mx);
        push_b(buf, b",");
        push_f2(buf, my);
    }
    let last = pts[pts.len() - 1];
    push_b(buf, b"L");
    push_f2(buf, last.0);
    push_b(buf, b",");
    push_f2(buf, last.1);
}
