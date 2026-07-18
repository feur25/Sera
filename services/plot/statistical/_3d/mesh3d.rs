use crate::html::js_3d::render_3d_html_impl;
use crate::plot::statistical::common::{push_b, push_i};
use crate::plot::{apply_bg3d, parse_all};

pub fn grid_to_mesh(xs: &[f64], ys: &[f64], z_grid: &[Vec<f64>]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<i64>) {
    let nr = ys.len().min(z_grid.len());
    let nc = xs.len();
    let mut xv = Vec::with_capacity(nr * nc);
    let mut yv = Vec::with_capacity(nr * nc);
    let mut zv = Vec::with_capacity(nr * nc);
    for r in 0..nr {
        let row = &z_grid[r];
        for c in 0..nc.min(row.len()) {
            xv.push(xs[c]);
            yv.push(ys[r]);
            zv.push(row[c]);
        }
    }
    let mut tri: Vec<i64> = Vec::with_capacity(nr.saturating_sub(1) * nc.saturating_sub(1) * 6);
    for r in 0..nr.saturating_sub(1) {
        for c in 0..nc.saturating_sub(1) {
            let i0 = (r * nc + c) as i64;
            let i1 = (r * nc + c + 1) as i64;
            let i2 = ((r + 1) * nc + c) as i64;
            let i3 = ((r + 1) * nc + c + 1) as i64;
            tri.extend_from_slice(&[i0, i1, i2]);
            tri.extend_from_slice(&[i1, i3, i2]);
        }
    }
    (xv, yv, zv, tri)
}

#[allow(clippy::too_many_arguments)]
pub fn render_mesh3d_html(
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    tri: &[i64],
    wireframe: bool,
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    let mut extra = Vec::<u8>::with_capacity(tri.len() * 6 + 32);
    push_b(&mut extra, b"var TI=[");
    for (i, &v) in tri.iter().enumerate() {
        if i > 0 {
            extra.push(b',');
        }
        push_i(&mut extra, v as i32);
    }
    push_b(&mut extra, b"];var WIRE=");
    push_b(&mut extra, if wireframe { b"true" } else { b"false" });
    push_b(&mut extra, b";");
    render_3d_html_impl(17, title, x, y, z, axis_labels, &[], &[], w, h, bg_color, scene, &extra)
}

fn grid_from_args(a: &crate::plot::chart_input::ChartArgs) -> (Vec<f64>, Vec<f64>, Vec<Vec<f64>>) {
    let zgrid = a.matrix.clone().unwrap_or_default();
    let xs = a.x.clone().unwrap_or_default();
    let ys = a.y.clone().unwrap_or_default();
    let xs = if xs.is_empty() {
        (0..zgrid.first().map(|r| r.len()).unwrap_or(0)).map(|i| i as f64).collect()
    } else {
        xs
    };
    let ys = if ys.is_empty() {
        (0..zgrid.len()).map(|i| i as f64).collect()
    } else {
        ys
    };
    (xs, ys, zgrid)
}

#[crate::chart_demo("x=[0,1,2], y=[0,1,2], matrix=[[0,1,0],[1,2,1],[0,1,0]]")]
#[crate::params(paramsList["title", "x", "y", "matrix", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("surface3d", "surface_3d", "surface", "surface3d_chart")]
#[crate::sera_builder]
pub fn build_surface3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let (xs, ys, zgrid) = grid_from_args(&a);
    let (xv, yv, zv, tri) = grid_to_mesh(&xs, &ys, &zgrid);
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
        id: 82,
        name: "surface_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

pub fn cube_mesh(cx: f64, cy: f64, cz: f64, half: f64) -> (Vec<(f64, f64, f64)>, Vec<[usize; 3]>) {
    let verts = vec![
        (cx - half, cy - half, cz - half),
        (cx + half, cy - half, cz - half),
        (cx + half, cy + half, cz - half),
        (cx - half, cy + half, cz - half),
        (cx - half, cy - half, cz + half),
        (cx + half, cy - half, cz + half),
        (cx + half, cy + half, cz + half),
        (cx - half, cy + half, cz + half),
    ];
    let tris = vec![
        [0, 2, 1], [0, 3, 2],
        [4, 5, 6], [4, 6, 7],
        [0, 4, 7], [0, 7, 3],
        [1, 2, 6], [1, 6, 5],
        [0, 1, 5], [0, 5, 4],
        [3, 7, 6], [3, 6, 2],
    ];
    (verts, tris)
}

pub fn cone_mesh(
    base: (f64, f64, f64),
    dir: (f64, f64, f64),
    radius: f64,
    segments: usize,
) -> (Vec<(f64, f64, f64)>, Vec<[usize; 3]>) {
    let segments = segments.max(3);
    let dl = (dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2).sqrt().max(1e-9);
    let u = (dir.0 / dl, dir.1 / dl, dir.2 / dl);
    let up = if u.2.abs() < 0.9 { (0.0, 0.0, 1.0) } else { (1.0, 0.0, 0.0) };
    let right_raw = (
        up.1 * u.2 - up.2 * u.1,
        up.2 * u.0 - up.0 * u.2,
        up.0 * u.1 - up.1 * u.0,
    );
    let rl = (right_raw.0 * right_raw.0 + right_raw.1 * right_raw.1 + right_raw.2 * right_raw.2)
        .sqrt()
        .max(1e-9);
    let right = (right_raw.0 / rl, right_raw.1 / rl, right_raw.2 / rl);
    let up2 = (
        u.1 * right.2 - u.2 * right.1,
        u.2 * right.0 - u.0 * right.2,
        u.0 * right.1 - u.1 * right.0,
    );
    let apex = (base.0 + dir.0, base.1 + dir.1, base.2 + dir.2);
    let mut verts = Vec::with_capacity(segments + 2);
    verts.push(apex);
    verts.push(base);
    for s in 0..segments {
        let theta = 2.0 * std::f64::consts::PI * (s as f64) / (segments as f64);
        let (ct, st) = (theta.cos(), theta.sin());
        verts.push((
            base.0 + radius * (right.0 * ct + up2.0 * st),
            base.1 + radius * (right.1 * ct + up2.1 * st),
            base.2 + radius * (right.2 * ct + up2.2 * st),
        ));
    }
    let mut tris = Vec::with_capacity(segments * 2);
    for s in 0..segments {
        let r0 = 2 + s;
        let r1 = 2 + (s + 1) % segments;
        tris.push([0usize, r0, r1]);
        tris.push([1usize, r1, r0]);
    }
    (verts, tris)
}

pub(crate) fn merge_meshes(fragments: Vec<(Vec<(f64, f64, f64)>, Vec<[usize; 3]>)>) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<i64>) {
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut tri = Vec::new();
    let mut offset = 0i64;
    for (verts, tris) in fragments {
        for &(vx, vy, vz) in &verts {
            xv.push(vx);
            yv.push(vy);
            zv.push(vz);
        }
        for t in &tris {
            tri.push(offset + t[0] as i64);
            tri.push(offset + t[1] as i64);
            tri.push(offset + t[2] as i64);
        }
        offset += verts.len() as i64;
    }
    (xv, yv, zv, tri)
}

#[crate::chart_demo("x=[0,2], y=[0,0], z=[0,0], voxel_size=0.8")]
#[crate::params(paramsList["title", "x", "y", "z", "voxel_size", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("voxels", "voxel", "voxels_chart", "cubes3d")]
#[crate::sera_builder]
pub fn build_voxels_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let xs = a.x.clone().unwrap_or_default();
    let ys = a.y.clone().unwrap_or_default();
    let zs = a.z.clone().unwrap_or_default();
    let n = xs.len().min(ys.len()).min(zs.len());
    let half = o.voxel_size.unwrap_or(0.8) * 0.5;
    let fragments: Vec<_> = (0..n)
        .map(|i| cube_mesh(xs[i], ys[i], zs[i], half))
        .collect();
    let (xv, yv, zv, tri) = merge_meshes(fragments);
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
        id: 85,
        name: "voxels",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[crate::chart_demo("x=[0], y=[0], z=[0], u=[0], v=[0], w=[1], cone_size=1.0")]
#[crate::params(paramsList["title", "x", "y", "z", "u", "v", "w", "cone_size", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("cone", "cone3d", "cone_3d", "cone_chart")]
#[crate::sera_builder]
pub fn build_cone_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let xs = a.x.clone().unwrap_or_default();
    let ys = a.y.clone().unwrap_or_default();
    let zs = a.z.clone().unwrap_or_default();
    let us = a.u.clone().unwrap_or_default();
    let vs = a.v.clone().unwrap_or_default();
    let ws = a.w.clone().unwrap_or_default();
    let n = xs.len().min(ys.len()).min(zs.len()).min(us.len()).min(vs.len()).min(ws.len());
    let size = o.cone_size.unwrap_or(1.0);
    let radius = size * 0.35;
    let fragments: Vec<_> = (0..n)
        .map(|i| {
            let dir = (us[i] * size, vs[i] * size, ws[i] * size);
            cone_mesh((xs[i], ys[i], zs[i]), dir, radius, 12)
        })
        .collect();
    let (xv, yv, zv, tri) = merge_meshes(fragments);
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
        id: 86,
        name: "cone",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod voxel_cone_tests {
    use super::*;

    fn tri_normal(v: &[(f64, f64, f64)], t: &[usize; 3]) -> (f64, f64, f64) {
        let a = v[t[0]];
        let b = v[t[1]];
        let c = v[t[2]];
        let u = (b.0 - a.0, b.1 - a.1, b.2 - a.2);
        let w = (c.0 - a.0, c.1 - a.1, c.2 - a.2);
        (
            u.1 * w.2 - u.2 * w.1,
            u.2 * w.0 - u.0 * w.2,
            u.0 * w.1 - u.1 * w.0,
        )
    }

    fn centroid(v: &[(f64, f64, f64)], t: &[usize; 3]) -> (f64, f64, f64) {
        let a = v[t[0]];
        let b = v[t[1]];
        let c = v[t[2]];
        ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0, (a.2 + b.2 + c.2) / 3.0)
    }

    #[test]
    fn cube_mesh_has_exact_vertex_and_triangle_count() {
        let (v, t) = cube_mesh(1.0, 2.0, 3.0, 0.5);
        assert_eq!(v.len(), 8);
        assert_eq!(t.len(), 12);
    }

    #[test]
    fn cube_mesh_all_normals_point_outward() {
        let center = (0.0, 0.0, 0.0);
        let (v, t) = cube_mesh(center.0, center.1, center.2, 1.0);
        for tri in &t {
            let n = tri_normal(&v, tri);
            let c = centroid(&v, tri);
            let outward = (c.0 - center.0, c.1 - center.1, c.2 - center.2);
            let dot = n.0 * outward.0 + n.1 * outward.1 + n.2 * outward.2;
            assert!(dot > 0.0, "triangle {tri:?} normal {n:?} does not point outward");
        }
    }

    #[test]
    fn cube_mesh_indices_within_bounds() {
        let (v, t) = cube_mesh(0.0, 0.0, 0.0, 1.0);
        for tri in &t {
            for &idx in tri {
                assert!(idx < v.len());
            }
        }
    }

    #[test]
    fn cone_mesh_has_exact_vertex_and_triangle_count() {
        let (v, t) = cone_mesh((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), 1.0, 8);
        assert_eq!(v.len(), 10);
        assert_eq!(t.len(), 16);
    }

    #[test]
    fn cone_mesh_apex_and_base_center_are_correct() {
        let (v, _t) = cone_mesh((1.0, 2.0, 3.0), (0.0, 0.0, 2.0), 1.0, 6);
        assert_eq!(v[0], (1.0, 2.0, 5.0));
        assert_eq!(v[1], (1.0, 2.0, 3.0));
    }

    #[test]
    fn cone_mesh_ring_points_at_correct_radius() {
        let base = (0.0, 0.0, 0.0);
        let (v, _t) = cone_mesh(base, (0.0, 0.0, 1.0), 2.0, 8);
        for p in &v[2..] {
            let r = ((p.0 - base.0).powi(2) + (p.1 - base.1).powi(2) + (p.2 - base.2).powi(2)).sqrt();
            assert!((r - 2.0).abs() < 1e-9, "ring point {p:?} not at radius 2.0");
        }
    }

    #[test]
    fn cone_mesh_lateral_normals_point_away_from_axis() {
        let base = (0.0, 0.0, 0.0);
        let dir = (0.0, 0.0, 1.0);
        let (v, t) = cone_mesh(base, dir, 1.0, 16);
        for tri in &t {
            if !tri.contains(&0) {
                continue;
            }
            let n = tri_normal(&v, tri);
            let c = centroid(&v, tri);
            let radial = (c.0, c.1, 0.0);
            let dot = n.0 * radial.0 + n.1 * radial.1;
            assert!(dot > 0.0, "lateral triangle {tri:?} normal {n:?} not radially outward");
        }
    }

    #[test]
    fn cone_mesh_base_cap_normals_point_away_from_apex() {
        let base = (0.0, 0.0, 0.0);
        let dir = (0.0, 0.0, 1.0);
        let (v, t) = cone_mesh(base, dir, 1.0, 16);
        for tri in &t {
            if tri.contains(&0) {
                continue;
            }
            let n = tri_normal(&v, tri);
            assert!(n.2 < 0.0, "base cap triangle {tri:?} normal {n:?} should point away from apex (-Z)");
        }
    }

    #[test]
    fn merge_meshes_offsets_indices_correctly() {
        let f1 = cube_mesh(0.0, 0.0, 0.0, 0.5);
        let f2 = cube_mesh(5.0, 0.0, 0.0, 0.5);
        let (xv, _yv, _zv, tri) = merge_meshes(vec![f1, f2]);
        assert_eq!(xv.len(), 16);
        assert_eq!(tri.len(), 72);
        assert!(tri.iter().all(|&i| i >= 0 && (i as usize) < xv.len()));
        assert!(tri[36..].iter().all(|&i| i >= 8));
    }
}

#[crate::chart_demo("x=[0,1,2], y=[0,1,2], matrix=[[0,1,0],[1,2,1],[0,1,0]]")]
#[crate::params(paramsList["title", "x", "y", "matrix", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("wireframe3d", "wireframe_3d", "wireframe", "wireframe3d_chart")]
#[crate::sera_builder]
pub fn build_wireframe3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let (xs, ys, zgrid) = grid_from_args(&a);
    let (xv, yv, zv, tri) = grid_to_mesh(&xs, &ys, &zgrid);
    let bg_str = o.bg_str();
    apply_bg3d(
        render_mesh3d_html(
            title,
            &xv,
            &yv,
            &zv,
            &tri,
            true,
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
        id: 83,
        name: "wireframe_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[crate::chart_demo("x=[0,1,0,1], y=[0,0,1,1], z=[0,0,0,1], mesh_i=[0], mesh_j=[1], mesh_k=[2]")]
#[crate::params(paramsList["title", "x", "y", "z", "mesh_i", "mesh_j", "mesh_k", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("mesh3d", "mesh_3d", "trimesh3d", "mesh3d_chart")]
#[crate::sera_builder]
pub fn build_mesh3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let xv = a.x.clone().unwrap_or_default();
    let yv = a.y.clone().unwrap_or_default();
    let zv = a.z.clone().unwrap_or_default();
    let mi = a.mesh_i.clone().unwrap_or_default();
    let mj = a.mesh_j.clone().unwrap_or_default();
    let mk = a.mesh_k.clone().unwrap_or_default();
    let nt = mi.len().min(mj.len()).min(mk.len());
    let mut tri = Vec::with_capacity(nt * 3);
    for t in 0..nt {
        tri.push(mi[t]);
        tri.push(mj[t]);
        tri.push(mk[t]);
    }
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
        id: 84,
        name: "mesh_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
