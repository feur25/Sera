use super::common::{area_path, finalize, open_frame, prepare, x_labels_row};
use super::config::StackplotConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_i, svg_legend_item};

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\"], series=[[10,14,12,18,20],[8,9,11,10,13],[5,6,7,9,8]], series_names=[\"A\",\"B\",\"C\"]"
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
        let color = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<linearGradient id=\"spg");
        push_i(&mut f.buf, si as i32);
        push_b(
            &mut f.buf,
            b"\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0%\" stop-color=\"#",
        );
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.95\"/><stop offset=\"100%\" stop-color=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stop-opacity=\"0.35\"/></linearGradient>");
    }
    push_b(&mut f.buf, b"</defs>");

    for si in 0..p.n_ser {
        let color = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<path data-idx=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" d=\"");
        area_path(&mut f.buf, &p, si, pl, pt, ph, step);
        push_b(&mut f.buf, b"\" fill=\"url(#spg");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b")\" stroke=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stroke-width=\"1\"/>");
    }
    let leg_x = cfg.width - 146;
    for (si, (name, _)) in cfg.series.iter().enumerate() {
        svg_legend_item(&mut f.buf, si as i32, name, palette_color(cfg.palette, si), leg_x, f.pt + 6 + si as i32 * 18, 18);
    }
    finalize(f, cfg)
}
