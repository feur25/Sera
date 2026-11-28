use crate::plot::statistical::_3d::mesh3d::render_mesh3d_html;
use crate::plot::{apply_bg3d, parse_all};

const TET_DECOMP: [[usize; 4]; 6] = [
    [0, 1, 2, 6],
    [0, 2, 3, 6],
    [0, 3, 7, 6],
    [0, 7, 4, 6],
    [0, 4, 5, 6],
    [0, 5, 1, 6],
];

fn sub3(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn cross3(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}

fn dot3(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn norm3(a: (f64, f64, f64)) -> (f64, f64, f64) {
    let l = (a.0 * a.0 + a.1 * a.1 + a.2 * a.2).sqrt().max(1e-12);
    (a.0 / l, a.1 / l, a.2 / l)
}

fn centroid_pts(pts: &[(f64, f64, f64)]) -> (f64, f64, f64) {
    let n = pts.len() as f64;
    let mut c = (0.0, 0.0, 0.0);
    for p in pts {
        c.0 += p.0;
        c.1 += p.1;
        c.2 += p.2;
    }
    (c.0 / n, c.1 / n, c.2 / n)
}

pub fn march_tet(verts: [(f64, f64, f64); 4], vals: [f64; 4], threshold: f64) -> Vec<[(f64, f64, f64); 3]> {
    let above: Vec<usize> = (0..4).filter(|&i| vals[i] >= threshold).collect();
    let below: Vec<usize> = (0..4).filter(|&i| vals[i] < threshold).collect();
    if above.is_empty() || below.is_empty() {
        return Vec::new();
    }

    let lerp_edge = |i: usize, j: usize| -> (f64, f64, f64) {
        let denom = vals[j] - vals[i];
        let t = if denom.abs() < 1e-12 { 0.5 } else { (threshold - vals[i]) / denom };
        (
            verts[i].0 + t * (verts[j].0 - verts[i].0),
            verts[i].1 + t * (verts[j].1 - verts[i].1),
            verts[i].2 + t * (verts[j].2 - verts[i].2),
        )
    };

    let above_pts: Vec<(f64, f64, f64)> = above.iter().map(|&i| verts[i]).collect();
    let below_pts: Vec<(f64, f64, f64)> = below.iter().map(|&i| verts[i]).collect();
    let outward = sub3(centroid_pts(&below_pts), centroid_pts(&above_pts));

    let mut pts = Vec::new();
    for &i in &above {
        for &j in &below {
            pts.push(lerp_edge(i, j));
        }
    }

    if pts.len() == 3 {
        let n = cross3(sub3(pts[1], pts[0]), sub3(pts[2], pts[0]));
        if dot3(n, outward) < 0.0 {
            pts.swap(1, 2);
        }
        vec![[pts[0], pts[1], pts[2]]]
    } else {
        let c = centroid_pts(&pts);
        let mut normal = cross3(sub3(pts[1], pts[0]), sub3(pts[2], pts[0]));
        if dot3(normal, outward) < 0.0 {
            normal = (-normal.0, -normal.1, -normal.2);
        }
        let ref_dir = norm3(sub3(pts[0], c));
        let perp = norm3(cross3(normal, ref_dir));

        let mut ordered = pts.clone();
        ordered.sort_by(|a, b| {
            let va = sub3(*a, c);
            let vb = sub3(*b, c);
            let angle_a = dot3(va, perp).atan2(dot3(va, ref_dir));
            let angle_b = dot3(vb, perp).atan2(dot3(vb, ref_dir));
            angle_a.partial_cmp(&angle_b).unwrap()
        });

        vec![
            [ordered[0], ordered[1], ordered[2]],
            [ordered[0], ordered[2], ordered[3]],
        ]
    }
}

pub fn march_cube(corners: [(f64, f64, f64); 8], vals: [f64; 8], threshold: f64) -> Vec<[(f64, f64, f64); 3]> {
    let mut tris = Vec::new();
    for tet in TET_DECOMP.iter() {
        let tv = [corners[tet[0]], corners[tet[1]], corners[tet[2]], corners[tet[3]]];
        let tf = [vals[tet[0]], vals[tet[1]], vals[tet[2]], vals[tet[3]]];
        tris.extend(march_tet(tv, tf, threshold));
    }
    tris
}

pub fn isosurface_mesh(
    xs: &[f64],
    ys: &[f64],
    zs: &[f64],
    field: &[f64],
    threshold: f64,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<i64>) {
    let (nx, ny, nz) = (xs.len(), ys.len(), zs.len());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut tri = Vec::new();

    if nx < 2 || ny < 2 || nz < 2 || field.len() < nx * ny * nz {
        return (xv, yv, zv, tri);
    }

    let idx = |ix: usize, iy: usize, iz: usize| iz * ny * nx + iy * nx + ix;

    for iz in 0..nz - 1 {
        for iy in 0..ny - 1 {
            for ix in 0..nx - 1 {
                let corners = [
                    (xs[ix], ys[iy], zs[iz]),
                    (xs[ix + 1], ys[iy], zs[iz]),
                    (xs[ix + 1], ys[iy + 1], zs[iz]),
                    (xs[ix], ys[iy + 1], zs[iz]),
                    (xs[ix], ys[iy], zs[iz + 1]),
                    (xs[ix + 1], ys[iy], zs[iz + 1]),
                    (xs[ix + 1], ys[iy + 1], zs[iz + 1]),
                    (xs[ix], ys[iy + 1], zs[iz + 1]),
                ];
                let vals = [
                    field[idx(ix, iy, iz)],
                    field[idx(ix + 1, iy, iz)],
                    field[idx(ix + 1, iy + 1, iz)],
                    field[idx(ix, iy + 1, iz)],
                    field[idx(ix, iy, iz + 1)],
                    field[idx(ix + 1, iy, iz + 1)],
                    field[idx(ix + 1, iy + 1, iz + 1)],
                    field[idx(ix, iy + 1, iz + 1)],
                ];
                for t in march_cube(corners, vals, threshold) {
                    let base = xv.len() as i64;
                    for p in &t {
                        xv.push(p.0);
                        yv.push(p.1);
                        zv.push(p.2);
                    }
                    tri.push(base);
                    tri.push(base + 1);
                    tri.push(base + 2);
                }
            }
        }
    }
    (xv, yv, zv, tri)
}

#[crate::chart_demo("x=[-1,0,1], y=[-1,0,1], z=[-1,0,1], field=[-1.5,-0.5,-1.5,-0.5,0.5,-0.5,-1.5,-0.5,-1.5,-0.5,0.5,-0.5,0.5,1.5,0.5,-0.5,0.5,-0.5,-1.5,-0.5,-1.5,-0.5,0.5,-0.5,-1.5,-0.5,-1.5], iso_level=0.0")]
#[crate::params(paramsList["title", "x", "y", "z", "field", "iso_level", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("isosurface", "iso_surface", "isosurface_chart", "implicit_surface")]
#[crate::sera_builder]
pub fn build_isosurface_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let xs = a.x.clone().unwrap_or_default();
    let ys = a.y.clone().unwrap_or_default();
    let zs = a.z.clone().unwrap_or_default();
    let field = a.field.clone().unwrap_or_default();
    let threshold = o.iso_level.unwrap_or(0.0);
    let (xv, yv, zv, tri) = isosurface_mesh(&xs, &ys, &zs, &field, threshold);
    let bg_str = o.bg_str();
    apply_bg3d(
        render_mesh3d_html(
            title,
            &xv,
            &yv,
            &zv,
            &tri,
            false,
            (&o.xl(), &o.yl(), &o.zl()),
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
        ),
        &o,
    )
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 88,
        name: "isosurface",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod isosurface_tests {
    use super::*;
    use std::collections::HashMap;

    const RIGHT_TET: [(f64, f64, f64); 4] = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)];

    #[test]
    fn march_tet_all_above_or_all_below_yields_nothing() {
        assert!(march_tet(RIGHT_TET, [1.0, 1.0, 1.0, 1.0], 0.5).is_empty());
        assert!(march_tet(RIGHT_TET, [0.0, 0.0, 0.0, 0.0], 0.5).is_empty());
    }

    #[test]
    fn march_tet_single_corner_above_matches_hand_derivation() {
        let tris = march_tet(RIGHT_TET, [1.0, 0.0, 0.0, 0.0], 0.5);
        assert_eq!(tris.len(), 1);
        let t = tris[0];
        let expected: [(f64, f64, f64); 3] = [(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (0.0, 0.0, 0.5)];
        for p in &expected {
            assert!(t.iter().any(|q| (q.0 - p.0).abs() < 1e-9 && (q.1 - p.1).abs() < 1e-9 && (q.2 - p.2).abs() < 1e-9));
        }
        let n = cross3(sub3(t[1], t[0]), sub3(t[2], t[0]));
        assert!(dot3(n, (1.0, 1.0, 1.0)) > 0.0, "expected outward normal away from origin, got {n:?}");
    }

    #[test]
    fn march_tet_single_corner_below_reverses_winding() {
        let tris = march_tet(RIGHT_TET, [0.0, 1.0, 1.0, 1.0], 0.5);
        assert_eq!(tris.len(), 1);
        let t = tris[0];
        let n = cross3(sub3(t[1], t[0]), sub3(t[2], t[0]));
        assert!(dot3(n, (-1.0, -1.0, -1.0)) > 0.0, "expected outward normal toward origin, got {n:?}");
    }

    #[test]
    fn march_tet_two_two_split_matches_hand_derivation() {
        let tris = march_tet(RIGHT_TET, [1.0, 1.0, 0.0, 0.0], 0.5);
        assert_eq!(tris.len(), 2);
        let expected_pts: [(f64, f64, f64); 4] = [(0.0, 0.5, 0.0), (0.0, 0.0, 0.5), (0.5, 0.5, 0.0), (0.5, 0.0, 0.5)];
        let mut all_pts = Vec::new();
        for t in &tris {
            all_pts.extend_from_slice(t);
        }
        for p in &expected_pts {
            assert!(all_pts.iter().any(|q| (q.0 - p.0).abs() < 1e-9 && (q.1 - p.1).abs() < 1e-9 && (q.2 - p.2).abs() < 1e-9), "missing expected point {p:?}");
        }
        let outward = (-0.5, 0.5, 0.5);
        for t in &tris {
            let n = cross3(sub3(t[1], t[0]), sub3(t[2], t[0]));
            assert!(dot3(n, outward) > 0.0, "triangle {t:?} normal {n:?} not outward");
        }
    }

    #[test]
    fn march_cube_uniform_field_yields_no_triangles() {
        let corners = [
            (0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0),
        ];
        assert!(march_cube(corners, [1.0; 8], 0.5).is_empty());
        assert!(march_cube(corners, [0.0; 8], 0.5).is_empty());
    }

    fn edge_key(a: (f64, f64, f64), b: (f64, f64, f64)) -> (i64, i64, i64, i64, i64, i64) {
        let r = |v: f64| (v * 1e6).round() as i64;
        (r(a.0), r(a.1), r(a.2), r(b.0), r(b.1), r(b.2))
    }

    fn assert_watertight_and_consistently_oriented(xv: &[f64], yv: &[f64], zv: &[f64], tri: &[i64]) {
        let n_tri = tri.len() / 3;
        assert!(n_tri > 0, "expected a non-empty isosurface");
        let mut directed_edges: HashMap<(i64, i64, i64, i64, i64, i64), i32> = HashMap::new();
        for t in 0..n_tri {
            let ia = tri[t * 3] as usize;
            let ib = tri[t * 3 + 1] as usize;
            let ic = tri[t * 3 + 2] as usize;
            let pa = (xv[ia], yv[ia], zv[ia]);
            let pb = (xv[ib], yv[ib], zv[ib]);
            let pc = (xv[ic], yv[ic], zv[ic]);
            for (u, v) in [(pa, pb), (pb, pc), (pc, pa)] {
                *directed_edges.entry(edge_key(u, v)).or_insert(0) += 1;
            }
        }
        for (key, count) in &directed_edges {
            assert_eq!(*count, 1, "directed edge {key:?} appears {count} times, expected exactly 1 (non-manifold or inconsistent winding)");
            let reversed = (key.3, key.4, key.5, key.0, key.1, key.2);
            assert!(
                directed_edges.get(&reversed).copied().unwrap_or(0) == 1,
                "edge {key:?} has no matching reverse edge (mesh is not watertight)"
            );
        }
    }

    #[test]
    fn isosurface_mesh_of_a_sphere_is_watertight_and_consistently_oriented() {
        let n = 14;
        let extent = 2.0;
        let axis: Vec<f64> = (0..n).map(|i| -extent + 2.0 * extent * (i as f64) / (n as f64 - 1.0)).collect();
        let radius2 = 1.0;
        let mut field = Vec::with_capacity(n * n * n);
        for &z in &axis {
            for &y in &axis {
                for &x in &axis {
                    field.push(radius2 - (x * x + y * y + z * z));
                }
            }
        }
        let (xv, yv, zv, tri) = isosurface_mesh(&axis, &axis, &axis, &field, 0.0);
        assert_watertight_and_consistently_oriented(&xv, &yv, &zv, &tri);

        let n_tri = tri.len() / 3;
        for t in 0..n_tri {
            let ia = tri[t * 3] as usize;
            let ib = tri[t * 3 + 1] as usize;
            let ic = tri[t * 3 + 2] as usize;
            let pa = (xv[ia], yv[ia], zv[ia]);
            let pb = (xv[ib], yv[ib], zv[ib]);
            let pc = (xv[ic], yv[ic], zv[ic]);
            let centroid = ((pa.0 + pb.0 + pc.0) / 3.0, (pa.1 + pb.1 + pc.1) / 3.0, (pa.2 + pb.2 + pc.2) / 3.0);
            let n = cross3(sub3(pb, pa), sub3(pc, pa));
            assert!(dot3(n, centroid) > 0.0, "triangle {t} normal does not point away from sphere center");
        }
    }

    #[test]
    fn isosurface_mesh_of_a_torus_is_watertight() {
        let n = 16;
        let extent = 2.2;
        let axis: Vec<f64> = (0..n).map(|i| -extent + 2.0 * extent * (i as f64) / (n as f64 - 1.0)).collect();
        let (big_r, small_r) = (1.2, 0.5);
        let mut field = Vec::with_capacity(n * n * n);
        for &z in &axis {
            for &y in &axis {
                for &x in &axis {
                    let q = ((x * x + y * y).sqrt() - big_r).powi(2) + z * z;
                    field.push(small_r * small_r - q);
                }
            }
        }
        let (xv, yv, zv, tri) = isosurface_mesh(&axis, &axis, &axis, &field, 0.0);
        assert_watertight_and_consistently_oriented(&xv, &yv, &zv, &tri);
    }
}
