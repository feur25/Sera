use super::config::StackplotConfig;
use crate::html::hover::slots_to_json;
use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i, truncate, Frame};

pub struct Prepared {
    pub n_pts: usize,
    pub n_ser: usize,
    pub bottoms: Vec<Vec<f64>>,
    pub tops: Vec<Vec<f64>>,
    pub ymin: f64,
    pub ymax: f64,
}

pub fn prepare(cfg: &StackplotConfig, streamgraph: bool) -> Option<Prepared> {
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
        let mut cursor = if streamgraph { -totals[i] / 2.0 } else { 0.0 };
        for (si, (_, vals)) in cfg.series.iter().enumerate() {
            let v = vals.get(i).copied().unwrap_or(0.0).max(0.0);
            bottoms[si][i] = cursor;
            cursor += v;
            tops[si][i] = cursor;
        }
    }

    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;
    for si in 0..n_ser {
        for i in 0..n_pts {
            if bottoms[si][i] < ymin {
                ymin = bottoms[si][i];
            }
            if tops[si][i] > ymax {
                ymax = tops[si][i];
            }
        }
    }
    if ymin > 0.0 {
        ymin = 0.0;
    }
    if ymax <= ymin {
        ymax = ymin + 1.0;
    }

    Some(Prepared {
        n_pts,
        n_ser,
        bottoms,
        tops,
        ymin,
        ymax,
    })
}

pub fn open_frame(cfg: &StackplotConfig, p: &Prepared) -> Frame {
    let legend_w: i32 = 160;
    let mut f = Frame::new_html(
        cfg.title,
        cfg.width,
        cfg.height,
        56,
        42,
        52,
        legend_w,
        p.n_pts * p.n_ser * 40 + 4096,
    );
    f.open(cfg.title, true);
    f.y_grid_rc(6, p.ymin, p.ymax, cfg.gridlines);
    f.axes(cfg.x_label, cfg.y_label);
    f
}

pub fn x_labels_row(f: &mut Frame, cfg: &StackplotConfig, p: &Prepared, step: f64) {
    let tick_step = ((p.n_pts as f64 / 12.0).ceil() as usize).max(1);
    for i in (0..p.n_pts).step_by(tick_step) {
        let x = f.pl + (i as f64 * step) as i32;
        push_b(&mut f.buf, b"<text x=\"");
        push_i(&mut f.buf, x);
        push_b(&mut f.buf, b"\" y=\"");
        push_i(&mut f.buf, f.pt + f.ph + 14);
        push_b(&mut f.buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#6b7280\" class=\"sp-xt\">");
        escape_xml(&mut f.buf, truncate(&cfg.x_labels[i], 12));
        push_b(&mut f.buf, b"</text>");
    }
}

pub fn y_px(pt: i32, ph: i32, ymin: f64, ymax: f64, v: f64) -> f64 {
    pt as f64 + (1.0 - (v - ymin) / (ymax - ymin).max(1e-12)) * ph as f64
}

pub fn area_path(buf: &mut Vec<u8>, p: &Prepared, si: usize, pl: i32, pt: i32, ph: i32, step: f64) {
    push_b(buf, b"M");
    for i in 0..p.n_pts {
        let x = pl as f64 + i as f64 * step;
        let y = y_px(pt, ph, p.ymin, p.ymax, p.tops[si][i]);
        if i > 0 {
            push_b(buf, b"L");
        }
        push_f2(buf, x);
        push_b(buf, b",");
        push_f2(buf, y);
    }
    for i in (0..p.n_pts).rev() {
        let x = pl as f64 + i as f64 * step;
        let y = y_px(pt, ph, p.ymin, p.ymax, p.bottoms[si][i]);
        push_b(buf, b"L");
        push_f2(buf, x);
        push_b(buf, b",");
        push_f2(buf, y);
    }
    push_b(buf, b"Z");
}

pub fn finalize(f: Frame, cfg: &StackplotConfig) -> String {
    let slots_json;
    let json: &str = if cfg.hover.is_empty() {
        "[]"
    } else {
        slots_json = slots_to_json(cfg.hover);
        &slots_json
    };
    f.html(json)
}
