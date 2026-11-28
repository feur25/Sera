use crate::plot::map::svg_parser::CountryShape;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Projection {
    Orthographic,
    Polar,
}

impl Projection {
    pub fn from_key(key: &str) -> Option<Self> {
        match key.trim().to_lowercase().as_str() {
            "orthographic" | "globe" | "sphere" => Some(Projection::Orthographic),
            "polar" | "azimuthal" | "pole" => Some(Projection::Polar),
            _ => None,
        }
    }

    pub fn project(&self, lat: f64, lon: f64, center_lat: f64, center_lon: f64) -> Option<(f64, f64)> {
        match self {
            Projection::Orthographic => orthographic(lat, lon, center_lat, center_lon),
            Projection::Polar => polar(lat, lon, center_lat, center_lon),
        }
    }

    pub fn draws_disc(&self) -> bool {
        matches!(self, Projection::Orthographic)
    }
}

fn orthographic(lat: f64, lon: f64, center_lat: f64, center_lon: f64) -> Option<(f64, f64)> {
    let lat = lat.to_radians();
    let lon = lon.to_radians();
    let lat0 = center_lat.to_radians();
    let lon0 = center_lon.to_radians();
    let cos_c = lat0.sin() * lat.sin() + lat0.cos() * lat.cos() * (lon - lon0).cos();
    if cos_c < 0.0 {
        return None;
    }
    let x = lat.cos() * (lon - lon0).sin();
    let y = lat0.cos() * lat.sin() - lat0.sin() * lat.cos() * (lon - lon0).cos();
    Some((x, y))
}

fn polar(lat: f64, lon: f64, center_lat: f64, center_lon: f64) -> Option<(f64, f64)> {
    let lat = lat.to_radians();
    let lon = lon.to_radians();
    let lat0 = center_lat.to_radians();
    let lon0 = center_lon.to_radians();
    let cos_c = (lat0.sin() * lat.sin() + lat0.cos() * lat.cos() * (lon - lon0).cos()).clamp(-1.0, 1.0);
    let c = cos_c.acos();
    if c > 155f64.to_radians() {
        return None;
    }
    let k = if c < 1e-9 { 0.0 } else { c / c.sin() };
    let x = k * lat.cos() * (lon - lon0).sin();
    let y = k * (lat0.cos() * lat.sin() - lat0.sin() * lat.cos() * (lon - lon0).cos());
    Some((x, y))
}

pub fn project_shapes<'a>(
    shapes: &[&'a CountryShape],
    to_latlon: fn(f32, f32) -> (f64, f64),
    projection: Projection,
    center_lat: f64,
    center_lon: f64,
) -> Vec<(usize, Vec<Vec<[f64; 2]>>)> {
    let mut out: Vec<(usize, Vec<Vec<[f64; 2]>>)> = Vec::with_capacity(shapes.len());
    for (i, shape) in shapes.iter().enumerate() {
        let mut shape_polys: Vec<Vec<[f64; 2]>> = Vec::with_capacity(shape.polygons.len());
        for poly in &shape.polygons {
            let mut projected: Vec<[f64; 2]> = Vec::with_capacity(poly.len());
            let mut visible = true;
            for &[x, y] in poly {
                let (lat, lon) = to_latlon(x, y);
                match projection.project(lat, lon, center_lat, center_lon) {
                    Some((px, py)) => projected.push([px, py]),
                    None => {
                        visible = false;
                        break;
                    }
                }
            }
            if visible && projected.len() >= 3 {
                shape_polys.push(projected);
            }
        }
        if !shape_polys.is_empty() {
            out.push((i, shape_polys));
        }
    }
    out
}

pub struct DiscBounds {
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

pub struct ProjectionTransform {
    dcx: f64,
    dcy: f64,
    scale: f64,
    ox: f64,
    oy: f64,
}

impl ProjectionTransform {
    pub fn apply(&self, x: f64, y: f64) -> [f32; 2] {
        [(self.ox + (x - self.dcx) * self.scale) as f32, (self.oy - (y - self.dcy) * self.scale) as f32]
    }

    pub fn disc(&self) -> DiscBounds {
        DiscBounds { cx: self.ox, cy: self.oy, radius: self.scale }
    }
}

pub fn normalize_projected(
    raw: &[(usize, Vec<Vec<[f64; 2]>>)],
    width: i32,
    height: i32,
    margin: f64,
) -> (Vec<(usize, Vec<Vec<[f32; 2]>>)>, DiscBounds) {
    let (transform, out) = fit_and_apply(raw, width, height, margin);
    (out, transform.disc())
}

pub fn project_and_fit(
    raw: &[(usize, Vec<Vec<[f64; 2]>>)],
    width: i32,
    height: i32,
    margin: f64,
) -> (ProjectionTransform, Vec<(usize, Vec<Vec<[f32; 2]>>)>) {
    fit_and_apply(raw, width, height, margin)
}

pub fn fit_transform(raw: &[(usize, Vec<Vec<[f64; 2]>>)], width: i32, height: i32, margin: f64) -> ProjectionTransform {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (_, polys) in raw {
        for poly in polys {
            for &[x, y] in poly {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }
    if !min_x.is_finite() {
        return ProjectionTransform { dcx: 0.0, dcy: 0.0, scale: 0.0, ox: width as f64 / 2.0, oy: height as f64 / 2.0 };
    }
    let span = (max_x - min_x).max(max_y - min_y).max(1e-9);
    ProjectionTransform {
        dcx: (min_x + max_x) / 2.0,
        dcy: (min_y + max_y) / 2.0,
        scale: (width.min(height) as f64) * margin / span,
        ox: width as f64 / 2.0,
        oy: height as f64 / 2.0,
    }
}

fn fit_and_apply(
    raw: &[(usize, Vec<Vec<[f64; 2]>>)],
    width: i32,
    height: i32,
    margin: f64,
) -> (ProjectionTransform, Vec<(usize, Vec<Vec<[f32; 2]>>)>) {
    let transform = fit_transform(raw, width, height, margin);
    let out = raw
        .iter()
        .map(|(i, polys)| {
            let out_polys = polys.iter().map(|poly| poly.iter().map(|&[x, y]| transform.apply(x, y)).collect()).collect();
            (*i, out_polys)
        })
        .collect();
    (transform, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthographic_centered_on_a_point_maps_it_to_the_origin() {
        let (x, y) = orthographic(10.0, 20.0, 10.0, 20.0).expect("center point must be visible");
        assert!(x.abs() < 1e-9, "x={x}");
        assert!(y.abs() < 1e-9, "y={y}");
    }

    #[test]
    fn orthographic_hides_the_antipodal_point() {
        assert!(orthographic(0.0, 180.0, 0.0, 0.0).is_none());
    }

    #[test]
    fn orthographic_keeps_a_point_ninety_degrees_away() {
        assert!(orthographic(0.0, 90.0, 0.0, 0.0).is_some());
    }

    #[test]
    fn polar_centered_on_the_pole_maps_it_to_the_origin() {
        let (x, y) = polar(90.0, 0.0, 90.0, 0.0).expect("pole must project");
        assert!(x.abs() < 1e-6 && y.abs() < 1e-6, "x={x} y={y}");
    }

    #[test]
    fn polar_excludes_points_near_the_antipode() {
        assert!(polar(-89.0, 0.0, 90.0, 0.0).is_none());
    }

    #[test]
    fn projection_from_key_resolves_known_aliases() {
        assert!(Projection::from_key("globe") == Some(Projection::Orthographic));
        assert!(Projection::from_key("pole") == Some(Projection::Polar));
        assert!(Projection::from_key("mercator").is_none());
    }

    #[test]
    fn normalize_projected_centers_a_single_point_on_the_canvas_middle() {
        let raw = vec![(0usize, vec![vec![[1.0, 1.0], [1.0, 1.0], [1.0, 1.0]]])];
        let (out, disc) = normalize_projected(&raw, 400, 300, 0.9);
        let poly = &out[0].1[0];
        assert!((poly[0][0] - 200.0).abs() < 1.0);
        assert!((poly[0][1] - 150.0).abs() < 1.0);
        assert!(disc.radius >= 0.0);
    }
}
