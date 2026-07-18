use crate::plot::statistical::_3d::mesh3d::{merge_meshes, render_mesh3d_html};
use crate::plot::{apply_bg3d, parse_all};

pub type FieldSample = (f64, f64, f64, f64, f64, f64);

pub fn idw_field(p: (f64, f64, f64), samples: &[FieldSample]) -> (f64, f64, f64) {
    let mut wsum = 0.0;
    let mut ux = 0.0;
    let mut uy = 0.0;
    let mut uz = 0.0;
    for &(sx, sy, sz, su, sv, sw) in samples {
        let dx = p.0 - sx;
        let dy = p.1 - sy;
        let dz = p.2 - sz;
        let d2 = dx * dx + dy * dy + dz * dz;
        let wt = 1.0 / (d2 + 1e-6);
        wsum += wt;
        ux += wt * su;
        uy += wt * sv;
        uz += wt * sw;
    }
    if wsum <= 0.0 {
        return (0.0, 0.0, 0.0);
    }
    (ux / wsum, uy / wsum, uz / wsum)
}

pub fn rk4_streamline(
    seed: (f64, f64, f64),
    samples: &[FieldSample],
    h: f64,
    n_steps: usize,
) -> Vec<(f64, f64, f64)> {
    let mut pts = Vec::with_capacity(n_steps + 1);
    let mut p = seed;
    pts.push(p);
    for _ in 0..n_steps {
        let k1 = idw_field(p, samples);
        let p2 = (p.0 + h / 2.0 * k1.0, p.1 + h / 2.0 * k1.1, p.2 + h / 2.0 * k1.2);
        let k2 = idw_field(p2, samples);
        let p3 = (p.0 + h / 2.0 * k2.0, p.1 + h / 2.0 * k2.1, p.2 + h / 2.0 * k2.2);
        let k3 = idw_field(p3, samples);
        let p4 = (p.0 + h * k3.0, p.1 + h * k3.1, p.2 + h * k3.2);
        let k4 = idw_field(p4, samples);
        p = (
            p.0 + h / 6.0 * (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0),
            p.1 + h / 6.0 * (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1),
            p.2 + h / 6.0 * (k1.2 + 2.0 * k2.2 + 2.0 * k3.2 + k4.2),
        );
        pts.push(p);
    }
    pts
}

fn normalize(v: (f64, f64, f64)) -> (f64, f64, f64) {
    let l = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt().max(1e-9);
    (v.0 / l, v.1 / l, v.2 / l)
}

fn cross(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.1 * b.2 - a.2 * b.1, a.2 * b.0 - a.0 * b.2, a.0 * b.1 - a.1 * b.0)
}

fn dot(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn sub(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

pub fn tangents(centerline: &[(f64, f64, f64)]) -> Vec<(f64, f64, f64)> {
    let n = centerline.len();
    let mut t = Vec::with_capacity(n);
    for i in 0..n {
        let d = if i + 1 < n {
            sub(centerline[i + 1], centerline[i])
        } else if i > 0 {
            sub(centerline[i], centerline[i - 1])
        } else {
            (0.0, 0.0, 1.0)
        };
        let len2 = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
        t.push(if len2 < 1e-18 { *t.last().unwrap_or(&(0.0, 0.0, 1.0)) } else { normalize(d) });
    }
    t
}

pub fn rmf_frames(centerline: &[(f64, f64, f64)]) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    let n = centerline.len();
    if n == 0 {
        return Vec::new();
    }
    let t = tangents(centerline);
    let up_guess = if t[0].2.abs() < 0.9 { (0.0, 0.0, 1.0) } else { (1.0, 0.0, 0.0) };
    let mut r0 = normalize(cross(up_guess, t[0]));
    if r0.0.abs() < 1e-12 && r0.1.abs() < 1e-12 && r0.2.abs() < 1e-12 {
        r0 = normalize(cross((0.0, 1.0, 0.0), t[0]));
    }
    let mut frames = Vec::with_capacity(n);
    frames.push((r0, normalize(cross(t[0], r0))));
    let mut r_prev = r0;
    for i in 1..n {
        let v1 = sub(centerline[i], centerline[i - 1]);
        let c1 = dot(v1, v1);
        let (r_i_l, t_i_l) = if c1 < 1e-18 {
            (r_prev, t[i - 1])
        } else {
            let f = 2.0 / c1;
            (
                sub(r_prev, scale(v1, f * dot(v1, r_prev))),
                sub(t[i - 1], scale(v1, f * dot(v1, t[i - 1]))),
            )
        };
        let v2 = sub(t[i], t_i_l);
        let c2 = dot(v2, v2);
        let r_i = if c2 < 1e-18 { r_i_l } else { sub(r_i_l, scale(v2, 2.0 / c2 * dot(v2, r_i_l))) };
        let r_i = normalize(r_i);
        frames.push((r_i, normalize(cross(t[i], r_i))));
        r_prev = r_i;
    }
    frames
}

fn scale(v: (f64, f64, f64), s: f64) -> (f64, f64, f64) {
    (v.0 * s, v.1 * s, v.2 * s)
}

pub fn tube_mesh(
    centerline: &[(f64, f64, f64)],
    frames: &[((f64, f64, f64), (f64, f64, f64))],
    radius: f64,
    sides: usize,
) -> (Vec<(f64, f64, f64)>, Vec<[usize; 3]>) {
    let sides = sides.max(3);
    let n = centerline.len().min(frames.len());
    let mut verts = Vec::with_capacity(n * sides);
    for i in 0..n {
        let (right, up) = frames[i];
        let c = centerline[i];
        for s in 0..sides {
            let theta = 2.0 * std::f64::consts::PI * (s as f64) / (sides as f64);
            let (ct, st) = (theta.cos(), theta.sin());
            verts.push((
                c.0 + radius * (right.0 * ct + up.0 * st),
                c.1 + radius * (right.1 * ct + up.1 * st),
                c.2 + radius * (right.2 * ct + up.2 * st),
            ));
        }
    }
    let mut tris = Vec::with_capacity(n.saturating_sub(1) * sides * 2);
    for i in 0..n.saturating_sub(1) {
        let ring0 = i * sides;
        let ring1 = (i + 1) * sides;
        for s in 0..sides {
            let s2 = (s + 1) % sides;
            let a = ring0 + s;
            let b = ring0 + s2;
            let c = ring1 + s2;
            let d = ring1 + s;
            tris.push([a, b, c]);
            tris.push([a, c, d]);
        }
    }
    (verts, tris)
}

#[crate::chart_demo("x=[0,0,0], y=[0,0,0], z=[0,1,2], u=[0,0,0], v=[0,0,0], w=[1,1,1], tube_radius=0.3")]
#[crate::params(paramsList["title", "x", "y", "z", "u", "v", "w", "tube_radius", "n_steps", "x_label", "y_label", "z_label", "bg_color", "scene", "width", "height"])]
#[crate::sera_alias("streamtube", "stream_tube", "streamtube_chart", "flow_tube")]
#[crate::sera_builder]
pub fn build_streamtube_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let xs = a.x.clone().unwrap_or_default();
    let ys = a.y.clone().unwrap_or_default();
    let zs = a.z.clone().unwrap_or_default();
    let us = a.u.clone().unwrap_or_default();
    let vs = a.v.clone().unwrap_or_default();
    let ws = a.w.clone().unwrap_or_default();
    let n = xs.len().min(ys.len()).min(zs.len()).min(us.len()).min(vs.len()).min(ws.len());
    if n == 0 {
        return String::new();
    }
    let samples: Vec<FieldSample> = (0..n).map(|i| (xs[i], ys[i], zs[i], us[i], vs[i], ws[i])).collect();

    let xmin = xs[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = xs[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let ymin = ys[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let ymax = ys[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let zmin = zs[..n].iter().cloned().fold(f64::INFINITY, f64::min);
    let zmax = zs[..n].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let diag = ((xmax - xmin).powi(2) + (ymax - ymin).powi(2) + (zmax - zmin).powi(2))
        .sqrt()
        .max(1e-6);

    let n_steps = o.n_steps.unwrap_or(30).max(1);
    let h = diag / (n_steps as f64 * 2.0);
    let radius = o.tube_radius.unwrap_or(diag * 0.02).max(1e-6);
    let sides = 8;

    let fragments: Vec<_> = (0..n)
        .map(|i| {
            let seed = (xs[i], ys[i], zs[i]);
            let centerline = rk4_streamline(seed, &samples, h, n_steps);
            let frames = rmf_frames(&centerline);
            tube_mesh(&centerline, &frames, radius, sides)
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
        id: 87,
        name: "streamtube",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod streamtube_tests {
    use super::*;

    #[test]
    fn idw_field_single_sample_returns_it_everywhere() {
        let samples = vec![(0.0, 0.0, 0.0, 1.0, 2.0, 3.0)];
        let f1 = idw_field((5.0, -3.0, 100.0), &samples);
        assert!((f1.0 - 1.0).abs() < 1e-9);
        assert!((f1.1 - 2.0).abs() < 1e-9);
        assert!((f1.2 - 3.0).abs() < 1e-9);
        let f2 = idw_field((0.0, 0.0, 0.0), &samples);
        assert!((f2.0 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn idw_field_two_equidistant_opposite_samples_average_at_midpoint() {
        let samples = vec![
            (-1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            (1.0, 0.0, 0.0, -1.0, 0.0, 0.0),
        ];
        let f = idw_field((0.0, 0.0, 0.0), &samples);
        assert!(f.0.abs() < 1e-9, "expected zero average, got {:?}", f);
    }

    #[test]
    fn idw_field_identical_vectors_at_different_positions_is_exact() {
        let samples = vec![
            (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            (5.0, 5.0, 5.0, 0.0, 0.0, 1.0),
            (-3.0, 2.0, 1.0, 0.0, 0.0, 1.0),
        ];
        for p in [(0.0, 0.0, 0.0), (2.0, 2.0, 2.0), (-1.0, -1.0, -1.0)] {
            let f = idw_field(p, &samples);
            assert!((f.2 - 1.0).abs() < 1e-9, "expected w=1 exactly at {:?}, got {:?}", p, f);
            assert!(f.0.abs() < 1e-9 && f.1.abs() < 1e-9);
        }
    }

    #[test]
    fn rk4_streamline_constant_field_is_exact_straight_line() {
        let samples = vec![
            (0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            (10.0, 10.0, 10.0, 0.0, 0.0, 1.0),
        ];
        let seed = (0.0, 0.0, 0.0);
        let h = 0.1;
        let n_steps = 20;
        let line = rk4_streamline(seed, &samples, h, n_steps);
        assert_eq!(line.len(), n_steps + 1);
        assert_eq!(line[0], seed);
        let last = line[n_steps];
        assert!(last.0.abs() < 1e-9);
        assert!(last.1.abs() < 1e-9);
        assert!((last.2 - h * n_steps as f64).abs() < 1e-9, "expected exact z={}, got {:?}", h * n_steps as f64, last);
    }

    #[test]
    fn rmf_frames_straight_line_stays_constant_and_orthonormal() {
        let centerline: Vec<(f64, f64, f64)> = (0..10).map(|i| (0.0, 0.0, i as f64)).collect();
        let frames = rmf_frames(&centerline);
        assert_eq!(frames.len(), 10);
        let (r0, u0) = frames[0];
        for &(r, u) in &frames {
            assert!((r.0 - r0.0).abs() < 1e-9 && (r.1 - r0.1).abs() < 1e-9 && (r.2 - r0.2).abs() < 1e-9);
            assert!((u.0 - u0.0).abs() < 1e-9 && (u.1 - u0.1).abs() < 1e-9 && (u.2 - u0.2).abs() < 1e-9);
            let rl = (r.0 * r.0 + r.1 * r.1 + r.2 * r.2).sqrt();
            let ul = (u.0 * u.0 + u.1 * u.1 + u.2 * u.2).sqrt();
            assert!((rl - 1.0).abs() < 1e-9);
            assert!((ul - 1.0).abs() < 1e-9);
            assert!(dot(r, u).abs() < 1e-9, "right/up not perpendicular: {:?} {:?}", r, u);
            assert!(dot(r, (0.0, 0.0, 1.0)).abs() < 1e-9, "right not perpendicular to tangent");
        }
    }

    #[test]
    fn tube_mesh_has_exact_vertex_and_triangle_count() {
        let centerline: Vec<(f64, f64, f64)> = (0..5).map(|i| (0.0, 0.0, i as f64)).collect();
        let frames = rmf_frames(&centerline);
        let (v, t) = tube_mesh(&centerline, &frames, 1.0, 6);
        assert_eq!(v.len(), 5 * 6);
        assert_eq!(t.len(), (5 - 1) * 6 * 2);
    }

    #[test]
    fn tube_mesh_normals_point_away_from_centerline_axis() {
        let centerline: Vec<(f64, f64, f64)> = (0..6).map(|i| (0.0, 0.0, i as f64)).collect();
        let frames = rmf_frames(&centerline);
        let (v, t) = tube_mesh(&centerline, &frames, 1.0, 8);
        for tri in &t {
            let a = v[tri[0]];
            let b = v[tri[1]];
            let c = v[tri[2]];
            let u = sub(b, a);
            let w = sub(c, a);
            let n = cross(u, w);
            let centroid = (
                (a.0 + b.0 + c.0) / 3.0,
                (a.1 + b.1 + c.1) / 3.0,
                (a.2 + b.2 + c.2) / 3.0,
            );
            let radial = (centroid.0, centroid.1, 0.0);
            let d = dot(n, radial);
            assert!(d > 0.0, "triangle {:?} normal {:?} not radially outward", tri, n);
        }
    }

    #[test]
    fn ring_points_lie_exactly_at_tube_radius() {
        let centerline: Vec<(f64, f64, f64)> = (0..3).map(|i| (0.0, 0.0, i as f64)).collect();
        let frames = rmf_frames(&centerline);
        let (v, _t) = tube_mesh(&centerline, &frames, 2.5, 10);
        for (i, p) in v.iter().enumerate() {
            let cz = centerline[i / 10].2;
            let r = ((p.0).powi(2) + (p.1).powi(2)).sqrt();
            assert!((r - 2.5).abs() < 1e-9, "point {:?} not at radius 2.5", p);
            assert!((p.2 - cz).abs() < 1e-9);
        }
    }
}
