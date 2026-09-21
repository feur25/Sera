use super::super::bar::Bar3DBlock;

pub const PAD: f64 = 0.03;
pub const BASE_ZOOM: f64 = 1.6;
const MIN_ASPECT: f64 = 0.22;
const EPS: f64 = 1e-9;
const UNIT_RADIUS: f64 = 0.866;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Fit {
    #[default]
    Uniform,
    Stretch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Extents {
    pub x: (f64, f64),
    pub y: (f64, f64),
    pub z: (f64, f64),
}

impl Extents {
    pub fn unit() -> Self {
        Self { x: (0.0, 1.0), y: (0.0, 1.0), z: (0.0, 1.0) }
    }

    fn spans(&self) -> [f64; 3] {
        [self.x, self.y, self.z].map(|(lo, hi)| (hi - lo).max(EPS))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Zone {
    pub scale: [f64; 3],
    pub centre: [f64; 2],
    pub data: [f64; 3],
    pub dims: [f64; 3],
    pub zoom: f64,
    pub extents: Extents,
}

pub fn extents(blocks: &[Bar3DBlock]) -> Extents {
    if blocks.is_empty() {
        return Extents::unit();
    }
    let fold = |pick: &dyn Fn(&Bar3DBlock) -> (f64, f64)| {
        blocks.iter().map(pick).fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), (a, b)| (lo.min(a), hi.max(b)))
    };
    Extents {
        x: fold(&|b| (b.cx - b.hw, b.cx + b.hw)),
        y: fold(&|b| (b.cy - b.hd, b.cy + b.hd)),
        z: fold(&|b| b.z_range()),
    }
}

fn is_flat(blocks: &[Bar3DBlock]) -> bool {
    blocks.windows(2).all(|w| w[0].z0 == w[1].z0 && w[0].z1 == w[1].z1 && w[0].end.is_none() && w[1].end.is_none())
}

fn proportions(explicit: Option<[f64; 3]>) -> Option<[f64; 3]> {
    let z = explicit?;
    z.iter().all(|v| v.is_finite() && *v > 0.0).then(|| {
        let longest = z.iter().cloned().fold(0.0, f64::max);
        z.map(|v| v / longest)
    })
}

fn all_equal(blocks: &[Bar3DBlock], pick: fn(&Bar3DBlock) -> f64) -> bool {
    blocks.windows(2).all(|w| pick(&w[0]) == pick(&w[1]))
}

fn footprint_scale(blocks: &[Bar3DBlock], rx: f64, ry: f64, mode: Fit) -> (f64, f64) {
    match mode {
        Fit::Uniform => (rx.max(ry), rx.max(ry)),
        Fit::Stretch => match (all_equal(blocks, |b| b.cx), all_equal(blocks, |b| b.cy)) {
            (true, true) => (rx.max(ry), rx.max(ry)),
            (false, true) => (rx, rx),
            (true, false) => (ry, ry),
            (false, false) => (rx, ry),
        },
    }
}

pub fn fit(blocks: &[Bar3DBlock], height_ratio: f64, explicit: Option<[f64; 3]>, mode: Fit) -> Zone {
    let ex = extents(blocks);
    let [rx, ry, rz] = ex.spans();
    let (scale, data, dims) = match proportions(explicit) {
        Some(d) => ([rx / d[0], ry / d[1], rz / d[2]], d, d),
        None => {
            let (sx, sy) = footprint_scale(blocks, rx, ry, mode);
            let (dx, dy) = (rx / sx, ry / sy);
            let longest = dx.max(dy);
            let (lx, ly) = (dx.max(MIN_ASPECT * longest), dy.max(MIN_ASPECT * longest));
            let (sz, dz) = if is_flat(blocks) {
                let s = sx.min(sy);
                (s, rz / s)
            } else {
                let lz = height_ratio * (0.5 + 0.5 * lx.min(ly) / lx.max(ly));
                (rz / lz, lz)
            };
            ([sx, sy, sz], [dx, dy, dz], [lx, ly, dz])
        }
    };
    let radius = 0.5
        * ((dims[0] + 2.0 * PAD).powi(2) + (dims[1] + 2.0 * PAD).powi(2) + (dims[2] + PAD).powi(2)).sqrt();
    Zone {
        scale,
        centre: [(ex.x.0 + ex.x.1) / 2.0, (ex.y.0 + ex.y.1) / 2.0],
        data,
        dims,
        zoom: BASE_ZOOM * radius / UNIT_RADIUS,
        extents: ex,
    }
}

pub fn cube_height(blocks: &[Bar3DBlock], width: f64, height_ratio: f64) -> f64 {
    if blocks.is_empty() {
        return width;
    }
    let zone = fit(blocks, height_ratio, None, Fit::Uniform);
    width * zone.scale[2] / zone.scale[0]
}

impl Zone {
    pub fn to_js(&self) -> String {
        let e = &self.extents;
        format!(
            "{{\"sx\":{:.6},\"sy\":{:.6},\"sz\":{:.6},\"cx\":{:.6},\"cy\":{:.6},\"lx\":{:.6},\"ly\":{:.6},\"lz\":{:.6},\"dx\":{:.6},\"dy\":{:.6},\"dz\":{:.6},\"zoom\":{:.4},\"ext\":[{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}]}}",
            self.scale[0],
            self.scale[1],
            self.scale[2],
            self.centre[0],
            self.centre[1],
            self.dims[0],
            self.dims[1],
            self.dims[2],
            self.data[0],
            self.data[1],
            self.data[2],
            self.zoom,
            e.x.0,
            e.x.1,
            e.y.0,
            e.y.1,
            e.z.0,
            e.z.1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(n: usize) -> Vec<Bar3DBlock> {
        (0..n).map(|i| Bar3DBlock::new(i as f64, 0.0, 0.0, (i + 1) as f64, 0.35, 0.35, i)).collect()
    }

    fn grid(rows: usize, cols: usize) -> Vec<Bar3DBlock> {
        (0..rows)
            .flat_map(|r| (0..cols).map(move |c| Bar3DBlock::new(c as f64, r as f64, 0.0, 1.0 + (r + c) as f64, 0.4, 0.4, r * cols + c)))
            .collect()
    }

    #[test]
    fn extents_include_the_half_sizes_and_both_z_ends() {
        let ex = extents(&row(3));
        assert_eq!(ex.x, (-0.35, 2.35));
        assert_eq!(ex.y, (-0.35, 0.35));
        assert_eq!(ex.z, (0.0, 3.0));
        assert_eq!(extents(&[]), Extents::unit());
    }

    #[test]
    fn the_longest_footprint_side_is_normalised_to_one_and_the_floor_keeps_a_minimum_depth() {
        let zone = fit(&row(20), 0.8, None, Fit::Uniform);
        assert!((zone.dims[0] - 1.0).abs() < 1e-9);
        assert!(zone.dims[1] >= MIN_ASPECT - 1e-9);
        assert!(zone.data[1] < zone.dims[1]);
        assert_eq!(zone.scale[0], zone.scale[1]);
    }

    #[test]
    fn a_square_grid_gets_a_square_floor_and_the_full_height_ratio() {
        let zone = fit(&grid(4, 4), 0.8, None, Fit::Uniform);
        assert!((zone.dims[0] - zone.dims[1]).abs() < 1e-9);
        assert!((zone.dims[2] - 0.8).abs() < 1e-9);
    }

    #[test]
    fn wider_scenes_are_flatter_and_zoom_out_with_their_diagonal() {
        let wide = fit(&row(40), 0.8, None, Fit::Uniform);
        let square = fit(&grid(6, 6), 0.8, None, Fit::Uniform);
        assert!(wide.dims[2] < square.dims[2]);
        let tiny = fit(&row(1), 0.8, None, Fit::Uniform);
        assert!(tiny.zoom > 0.0 && wide.zoom > 0.0);
    }

    #[test]
    fn an_explicit_zone_sets_the_box_proportions_exactly() {
        let zone = fit(&grid(3, 3), 0.8, Some([2.0, 1.0, 1.0]), Fit::Uniform);
        assert_eq!(zone.dims, [1.0, 0.5, 0.5]);
        let stretched = zone.extents.spans();
        assert!((stretched[0] / zone.scale[0] - 1.0).abs() < 1e-9);
        assert!((stretched[1] / zone.scale[1] - 0.5).abs() < 1e-9);
    }

    #[test]
    fn invalid_explicit_zones_fall_back_to_the_automatic_fit() {
        let auto = fit(&grid(3, 3), 0.8, None, Fit::Uniform);
        for bad in [[0.0, 1.0, 1.0], [1.0, -1.0, 1.0], [f64::NAN, 1.0, 1.0]] {
            assert_eq!(fit(&grid(3, 3), 0.8, Some(bad), Fit::Uniform).dims, auto.dims);
        }
    }

    #[test]
    fn flat_scenes_keep_the_natural_thickness_instead_of_filling_the_height() {
        let plates: Vec<Bar3DBlock> = (0..5).map(|i| Bar3DBlock::new(i as f64, 0.0, 0.0, 0.5, 0.4, 0.4, i)).collect();
        let zone = fit(&plates, 0.8, None, Fit::Uniform);
        assert!(zone.dims[2] < 0.2);
    }

    #[test]
    fn stretch_keeps_a_value_axis_and_a_category_axis_readable_together() {
        let lanes: Vec<Bar3DBlock> = (0..5)
            .map(|i| {
                let length = 10.0 + 8.0 * i as f64;
                Bar3DBlock::new(length / 2.0, i as f64, 0.0, 0.7, length / 2.0, 0.35, i)
            })
            .collect();
        let uniform = fit(&lanes, 0.8, None, Fit::Uniform);
        let stretched = fit(&lanes, 0.8, None, Fit::Stretch);
        assert!(stretched.data[1] > uniform.data[1] * 3.0);
        assert!((stretched.dims[0] - 1.0).abs() < 1e-9 && (stretched.dims[1] - 1.0).abs() < 1e-9);
        assert!(stretched.dims[2] > uniform.dims[2] * 3.0);
    }

    #[test]
    fn a_cube_height_looks_as_tall_as_it_is_wide_once_fitted() {
        let blocks = row(12);
        let zone = fit(&blocks, 0.8, None, Fit::Uniform);
        let tall = cube_height(&blocks, 0.6, 0.8);
        assert!((tall / zone.scale[2] - 0.6 / zone.scale[0]).abs() < 1e-9);
        assert_eq!(cube_height(&[], 0.6, 0.8), 0.6);
    }

    #[test]
    fn the_js_literal_carries_every_field_the_engine_reads() {
        let js = fit(&row(3), 0.8, None, Fit::Uniform).to_js();
        for key in ["sx", "sy", "sz", "cx", "cy", "lx", "ly", "lz", "dx", "dy", "dz", "zoom", "ext"] {
            assert!(js.contains(&format!("\"{key}\":")), "missing {key}");
        }
    }
}
