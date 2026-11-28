pub(super) fn next_mid() -> usize {
    super::MARKER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

pub(super) fn parse_pts(pts: Vec<Vec<f64>>) -> Vec<(f64, f64)> {
    pts.into_iter()
        .filter_map(|p| {
            if p.len() >= 2 {
                Some((p[0], p[1]))
            } else {
                None
            }
        })
        .collect()
}

fn line_intersect(p1: (f64, f64), p2: (f64, f64), a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d1 = a * p1.0 + b * p1.1 - c;
    let d2 = a * p2.0 + b * p2.1 - c;
    let denom = d1 - d2;
    if denom.abs() < 1e-12 {
        return None;
    }
    let t = d1 / denom;
    Some((p1.0 + t * (p2.0 - p1.0), p1.1 + t * (p2.1 - p1.1)))
}

fn clip_half_plane(poly: &[(f64, f64)], a: f64, b: f64, c: f64) -> Vec<(f64, f64)> {
    let n = poly.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n + 1);
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let cur_in = a * cur.0 + b * cur.1 <= c;
        let prev_in = a * prev.0 + b * prev.1 <= c;
        if cur_in != prev_in {
            if let Some(ix) = line_intersect(prev, cur, a, b, c) {
                out.push(ix);
            }
        }
        if cur_in {
            out.push(cur);
        }
    }
    out
}

pub(super) fn voronoi_cells(sites: &[(f64, f64)], bx: f64, by: f64, bw: f64, bh: f64) -> Vec<Vec<(f64, f64)>> {
    let bbox = vec![(bx, by), (bx + bw, by), (bx + bw, by + bh), (bx, by + bh)];
    sites
        .iter()
        .enumerate()
        .map(|(i, &(sx, sy))| {
            let mut poly = bbox.clone();
            for (j, &(ox, oy)) in sites.iter().enumerate() {
                if i == j || poly.is_empty() {
                    continue;
                }
                let mx = (sx + ox) / 2.0;
                let my = (sy + oy) / 2.0;
                let a = ox - sx;
                let b = oy - sy;
                let c = a * mx + b * my;
                poly = clip_half_plane(&poly, a, b, c);
            }
            poly
        })
        .collect()
}

pub(super) fn polar_xy(cx: f64, cy: f64, r: f64, deg: f64) -> (f64, f64) {
    let rad = (deg - 90.0) * std::f64::consts::PI / 180.0;
    (cx + r * rad.cos(), cy + r * rad.sin())
}

pub(super) fn pts_to_svg(pts: &[(f64, f64)]) -> String {
    pts.iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn catmull_rom(pts: &[(f64, f64)], tension: f64) -> String {
    let n = pts.len();
    if n == 0 {
        return String::new();
    }
    if n == 1 {
        return format!("M {:.2},{:.2}", pts[0].0, pts[0].1);
    }
    let mut d = format!("M {:.2},{:.2}", pts[0].0, pts[0].1);
    for i in 0..n - 1 {
        let p0 = if i == 0 { pts[0] } else { pts[i - 1] };
        let p1 = pts[i];
        let p2 = pts[i + 1];
        let p3 = if i + 2 < n { pts[i + 2] } else { pts[n - 1] };
        let cp1x = p1.0 + (p2.0 - p0.0) * tension / 6.0;
        let cp1y = p1.1 + (p2.1 - p0.1) * tension / 6.0;
        let cp2x = p2.0 - (p3.0 - p1.0) * tension / 6.0;
        let cp2y = p2.1 - (p3.1 - p1.1) * tension / 6.0;
        d.push_str(&format!(
            " C {:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
            cp1x, cp1y, cp2x, cp2y, p2.0, p2.1
        ));
    }
    d
}
