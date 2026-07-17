use super::common::{area_path, finalize, open_frame, x_labels_row, Prepared};
use super::config::StackplotConfig;
use crate::plot::statistical::common::{hex6, palette_color, push_b, push_i, svg_legend_item};

fn prepare_normalized(cfg: &StackplotConfig) -> Option<Prepared> {
    let n_pts = cfg.x_labels.len();
    let n_ser = cfg.series.len();
    if n_pts < 2 || n_ser == 0 {
        return None;
    }

    let mut totals = vec![0.0f64; n_pts];
    for (_, vals) in cfg.series {
        for (i, total) in totals.iter_mut().enumerate() {
            *total += vals.get(i).copied().unwrap_or(0.0).max(0.0);
        }
    }

    let mut bottoms = vec![vec![0.0f64; n_pts]; n_ser];
    let mut tops = vec![vec![0.0f64; n_pts]; n_ser];
    for i in 0..n_pts {
        let total = totals[i].max(1e-9);
        let mut cursor = 0.0f64;
        for (si, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(i).copied().unwrap_or(0.0).max(0.0) / total;
            bottoms[si][i] = cursor;
            cursor += v;
            tops[si][i] = cursor;
        }
    }

    Some(Prepared {
        n_pts,
        n_ser,
        bottoms,
        tops,
        ymin: 0.0,
        ymax: 1.0,
    })
}

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\"], series=[[10,14,12,18,20],[8,9,11,10,13],[5,6,7,9,8]], series_names=[\"A\",\"B\",\"C\"]"
)]

pub fn render(cfg: &StackplotConfig) -> String {
    let p = match prepare_normalized(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut f = open_frame(cfg, &p);
    let step = f.pw as f64 / (p.n_pts - 1).max(1) as f64;
    x_labels_row(&mut f, cfg, &p, step);
    let (pl, pt, ph) = (f.pl, f.pt, f.ph);
    for si in 0..p.n_ser {
        let color = hex6(palette_color(cfg.palette, si));
        push_b(&mut f.buf, b"<path data-idx=\"");
        push_i(&mut f.buf, si as i32);
        push_b(&mut f.buf, b"\" d=\"");
        area_path(&mut f.buf, &p, si, pl, pt, ph, step);
        push_b(&mut f.buf, b"\" fill=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" fill-opacity=\"0.85\" stroke=\"#");
        f.buf.extend_from_slice(&color);
        push_b(&mut f.buf, b"\" stroke-width=\"1\"/>");
    }
    let leg_x = cfg.width - 146;
    for (si, (name, _)) in cfg.series.iter().enumerate() {
        svg_legend_item(&mut f.buf, si as i32, name, palette_color(cfg.palette, si), leg_x, f.pt + 6 + si as i32 * 18, 18);
    }
    finalize(f, cfg)
}
