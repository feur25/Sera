use crate::plot::statistical::common::hash01;

#[derive(Clone, Copy)]
pub struct Site {
    pub x: f64,
    pub y: f64,
    pub w: f64,
}

pub type Poly = Vec<(f64, f64)>;

pub fn sector_polygon(cx: f64, cy: f64, r: f64, theta0: f64, theta1: f64, segments: usize) -> Poly {
    let full = (theta1 - theta0).abs() >= 2.0 * std::f64::consts::PI - 1e-6;
    let mut pts = Vec::with_capacity(segments + 2);
    if !full {
        pts.push((cx, cy));
    }
    for k in 0..=segments {
        let t = theta0 + (theta1 - theta0) * (k as f64 / segments as f64);
        pts.push((cx + r * t.cos(), cy + r * t.sin()));
    }
    pts
}

pub fn polygon_area(poly: &[(f64, f64)]) -> f64 {
    if poly.len() < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    let n = poly.len();
    for i in 0..n {
        let (x0, y0) = poly[i];
        let (x1, y1) = poly[(i + 1) % n];
        a += x0 * y1 - x1 * y0;
    }
    (a * 0.5).abs()
}

pub fn polygon_centroid(poly: &[(f64, f64)]) -> (f64, f64) {
    let n = poly.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    let mut a = 0.0;
    let mut cx = 0.0;
    let mut cy = 0.0;
    for i in 0..n {
        let (x0, y0) = poly[i];
        let (x1, y1) = poly[(i + 1) % n];
        let cross = x0 * y1 - x1 * y0;
        a += cross;
        cx += (x0 + x1) * cross;
        cy += (y0 + y1) * cross;
    }
    a *= 0.5;
    if a.abs() < 1e-9 {
        let sx: f64 = poly.iter().map(|p| p.0).sum();
        let sy: f64 = poly.iter().map(|p| p.1).sum();
        return (sx / n as f64, sy / n as f64);
    }
    (cx / (6.0 * a), cy / (6.0 * a))
}

fn intersect(p0: (f64, f64), p1: (f64, f64), a: f64, b: f64, c: f64) -> (f64, f64) {
    let d0 = a * p0.0 + b * p0.1 - c;
    let d1 = a * p1.0 + b * p1.1 - c;
    let t = d0 / (d0 - d1);
    (p0.0 + t * (p1.0 - p0.0), p0.1 + t * (p1.1 - p0.1))
}

fn clip_half_plane(poly: &[(f64, f64)], a: f64, b: f64, c: f64) -> Poly {
    if poly.is_empty() {
        return Vec::new();
    }
    let n = poly.len();
    let mut out = Vec::with_capacity(n + 2);
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let cur_in = a * cur.0 + b * cur.1 < c;
        let prev_in = a * prev.0 + b * prev.1 < c;
        if cur_in {
            if !prev_in {
                out.push(intersect(prev, cur, a, b, c));
            }
            out.push(cur);
        } else if prev_in {
            out.push(intersect(prev, cur, a, b, c));
        }
    }
    out
}

pub fn power_cell(boundary: &[(f64, f64)], sites: &[Site], i: usize) -> Poly {
    let mut poly = boundary.to_vec();
    let si = sites[i];
    for (j, sj) in sites.iter().enumerate() {
        if j == i || poly.is_empty() {
            continue;
        }
        let a = 2.0 * (sj.x - si.x);
        let b = 2.0 * (sj.y - si.y);
        let c = (sj.x * sj.x + sj.y * sj.y - sj.w) - (si.x * si.x + si.y * si.y - si.w);
        poly = clip_half_plane(&poly, a, b, c);
    }
    poly
}

fn seed_in_boundary(cx: f64, cy: f64, r_max: f64, theta0: f64, theta1: f64, i: usize, n: usize) -> (f64, f64) {
    let golden = std::f64::consts::PI * (3.0 - 5.0f64.sqrt());
    let frac = (i as f64 + 0.5) / n.max(1) as f64;
    let r = r_max * 0.9 * frac.sqrt();
    let jitter = (hash01(i * 7 + 3) - 0.5) * 0.06 * (theta1 - theta0);
    let theta = theta0 + ((i as f64 * golden).rem_euclid(theta1 - theta0)) + jitter;
    (cx + r * theta.cos(), cy + r * theta.sin())
}

pub fn voronoi_treemap(
    cx: f64,
    cy: f64,
    r: f64,
    theta0: f64,
    theta1: f64,
    boundary: &[(f64, f64)],
    order: &[usize],
    target_areas: &[f64],
    iterations: usize,
) -> (Vec<Poly>, Vec<Site>) {
    let n = order.len();
    if n == 0 {
        return (Vec::new(), Vec::new());
    }
    let boundary_area = polygon_area(boundary).max(1e-6);
    let total_target: f64 = order.iter().map(|&oi| target_areas[oi]).sum::<f64>().max(1e-9);

    let mut sites: Vec<Site> = (0..n)
        .map(|i| {
            let (x, y) = seed_in_boundary(cx, cy, r, theta0, theta1, i, n);
            let frac = target_areas[order[i]] / total_target;
            Site {
                x,
                y,
                w: (boundary_area * frac / std::f64::consts::PI).max(1.0),
            }
        })
        .collect();

    for it in 0..iterations {
        let cells: Vec<Poly> = (0..n).map(|i| power_cell(boundary, &sites, i)).collect();
        for i in 0..n {
            let area = polygon_area(&cells[i]);
            let target = boundary_area * (target_areas[order[i]] / total_target);
            if area > 1e-6 {
                let ratio = (target / area).clamp(0.6, 1.7);
                sites[i].w *= ratio.powf(0.42);
            } else {
                sites[i].w *= 1.9;
            }
            if !cells[i].is_empty() && it % 3 == 2 && it > 4 {
                let (px, py) = polygon_centroid(&cells[i]);
                sites[i].x += (px - sites[i].x) * 0.4;
                sites[i].y += (py - sites[i].y) * 0.4;
            }
        }
    }

    let cells = (0..n).map(|i| power_cell(boundary, &sites, i)).collect();
    (cells, sites)
}
