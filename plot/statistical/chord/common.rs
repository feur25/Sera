use crate::plot::statistical::common::{hex6, palette_color, push_b, push_f2, push_i};
use std::f64::consts::PI;

pub struct ChordLayout {
    pub cx: f64,
    pub cy: f64,
    pub r_out: f64,
    pub r_in: f64,
    pub r_label: f64,
    pub arcs: Vec<(f64, f64)>,
    pub sub_arcs: Vec<Vec<(f64, f64)>>,
    pub totals: Vec<f64>,
}

pub fn compute_layout(cfg: &super::config::ChordConfig) -> Option<ChordLayout> {
    let n = cfg.labels.len().min(cfg.matrix.len());
    if n == 0 { return None; }

    let cx = cfg.width as f64 / 2.0;
    let cy = cfg.height as f64 / 2.0;
    let r_out = (cfg.width.min(cfg.height) as f64 / 2.0 - 40.0).max(50.0);
    let r_in  = r_out - cfg.arc_width;

    let mut totals = vec![0.0f64; n];
    for i in 0..n {
        for j in 0..n {
            if let Some(row) = cfg.matrix.get(i) {
                totals[i] += row.get(j).copied().unwrap_or(0.0);
            }
        }
    }
    let grand_total: f64 = totals.iter().sum();
    if grand_total <= 0.0 { return None; }

    let gap_rad = cfg.gap_deg * PI / 180.0;
    let total_gap = gap_rad * n as f64;
    let avail = 2.0 * PI - total_gap;

    let mut arcs = Vec::with_capacity(n);
    let mut sub_arcs = Vec::with_capacity(n);
    let mut cursor = -PI / 2.0;

    for i in 0..n {
        let span = avail * totals[i] / grand_total;
        let a1 = cursor;
        let a2 = cursor + span;
        arcs.push((a1, a2));

        let mut subs = Vec::with_capacity(n);
        let mut sc = a1;
        for j in 0..n {
            let v = cfg.matrix.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
            let s_span = span * v / totals[i].max(1e-12);
            subs.push((sc, sc + s_span));
            sc += s_span;
        }
        sub_arcs.push(subs);
        cursor = a2 + gap_rad;
    }

    Some(ChordLayout { cx, cy, r_out, r_in, r_label: r_out + 14.0, arcs, sub_arcs, totals })
}

pub fn arc_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r1: f64, r2: f64, a1: f64, a2: f64) {
    let (s1x, s1y) = (cx + r2 * a1.cos(), cy + r2 * a1.sin());
    let (s2x, s2y) = (cx + r2 * a2.cos(), cy + r2 * a2.sin());
    let (e1x, e1y) = (cx + r1 * a2.cos(), cy + r1 * a2.sin());
    let (e2x, e2y) = (cx + r1 * a1.cos(), cy + r1 * a1.sin());
    let large = if a2 - a1 > PI { 1 } else { 0 };
    push_b(buf, b"M");
    push_f2(buf, s1x); push_b(buf, b","); push_f2(buf, s1y);
    push_b(buf, b"A"); push_f2(buf, r2); push_b(buf, b","); push_f2(buf, r2);
    push_b(buf, b" 0 "); push_i(buf, large); push_b(buf, b" 1 ");
    push_f2(buf, s2x); push_b(buf, b","); push_f2(buf, s2y);
    push_b(buf, b"L"); push_f2(buf, e1x); push_b(buf, b","); push_f2(buf, e1y);
    push_b(buf, b"A"); push_f2(buf, r1); push_b(buf, b","); push_f2(buf, r1);
    push_b(buf, b" 0 "); push_i(buf, large); push_b(buf, b" 0 ");
    push_f2(buf, e2x); push_b(buf, b","); push_f2(buf, e2y);
    push_b(buf, b"Z");
}

pub fn ribbon_path(buf: &mut Vec<u8>, cx: f64, cy: f64, r: f64, sa1: f64, sa2: f64, ta1: f64, ta2: f64) {
    let sm = (sa1 + sa2) / 2.0;
    let tm = (ta1 + ta2) / 2.0;
    let (sx, sy) = (cx + r * sm.cos(), cy + r * sm.sin());
    let (tx, ty) = (cx + r * tm.cos(), cy + r * tm.sin());
    let large_s = if sa2 - sa1 > PI { 1 } else { 0 };
    let large_t = if ta2 - ta1 > PI { 1 } else { 0 };

    let (s1x, s1y) = (cx + r * sa1.cos(), cy + r * sa1.sin());
    let (s2x, s2y) = (cx + r * sa2.cos(), cy + r * sa2.sin());
    let (t1x, t1y) = (cx + r * ta1.cos(), cy + r * ta1.sin());
    let (t2x, t2y) = (cx + r * ta2.cos(), cy + r * ta2.sin());

    push_b(buf, b"M"); push_f2(buf, s1x); push_b(buf, b","); push_f2(buf, s1y);
    push_b(buf, b"A"); push_f2(buf, r); push_b(buf, b","); push_f2(buf, r);
    push_b(buf, b" 0 "); push_i(buf, large_s); push_b(buf, b" 1 ");
    push_f2(buf, s2x); push_b(buf, b","); push_f2(buf, s2y);
    push_b(buf, b"Q"); push_f2(buf, cx); push_b(buf, b","); push_f2(buf, cy);
    push_b(buf, b" "); push_f2(buf, t2x); push_b(buf, b","); push_f2(buf, t2y);
    push_b(buf, b"A"); push_f2(buf, r); push_b(buf, b","); push_f2(buf, r);
    push_b(buf, b" 0 "); push_i(buf, large_t); push_b(buf, b" 0 ");
    push_f2(buf, t1x); push_b(buf, b","); push_f2(buf, t1y);
    push_b(buf, b"Q"); push_f2(buf, cx); push_b(buf, b","); push_f2(buf, cy);
    push_b(buf, b" "); push_f2(buf, s1x); push_b(buf, b","); push_f2(buf, s1y);
    push_b(buf, b"Z");
}
