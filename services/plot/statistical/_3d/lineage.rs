use super::super::bar::Bar3DBlock;

pub const NODE_HW: f64 = 0.18;
pub const EDGE_HW: f64 = 0.05;
pub const EDGE_STEPS: usize = 5;

pub type Point = (f64, f64, f64);

pub fn markers(positions: &[Point], size: f64, class_of: impl Fn(usize) -> usize, tone_of: impl Fn(usize) -> f64) -> Vec<Bar3DBlock> {
    positions
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z))| Bar3DBlock::new(x, y, z - size, z + size, size, size, class_of(i)).with_tone(tone_of(i)))
        .collect()
}

fn along(path: &[Point], t: f64) -> Point {
    let segments = path.len().saturating_sub(1).max(1);
    let scaled = t * segments as f64;
    let seg = (scaled as usize).min(segments - 1);
    let local = scaled - seg as f64;
    let (x0, y0, z0) = path[seg];
    let (x1, y1, z1) = path[seg + 1];
    (x0 + (x1 - x0) * local, y0 + (y1 - y0) * local, z0 + (z1 - z0) * local)
}

pub fn paths(links: &[Vec<Point>], size: f64, steps: usize, class_of: impl Fn(usize) -> usize, tone_of: impl Fn(usize) -> f64) -> Vec<Bar3DBlock> {
    links
        .iter()
        .enumerate()
        .filter(|(_, path)| path.len() >= 2)
        .flat_map(|(li, path)| {
            let class = class_of(li);
            let tone = tone_of(li);
            (1..steps).map(move |k| {
                let (x, y, z) = along(path, k as f64 / steps as f64);
                Bar3DBlock::new(x, y, z - size, z + size, size, size, class).with_tone(tone)
            })
        })
        .collect()
}

pub fn weighted_paths(
    links: &[Vec<Point>],
    weights: &[f64],
    min_size: f64,
    max_size: f64,
    steps: usize,
    class_of: impl Fn(usize) -> usize,
    tone_of: impl Fn(usize) -> f64,
) -> Vec<Bar3DBlock> {
    let peak = weights.iter().copied().filter(|v| v.is_finite()).fold(0.0f64, f64::max).max(1e-12);
    links
        .iter()
        .enumerate()
        .flat_map(|(li, path)| {
            let w = weights.get(li).copied().unwrap_or(0.0).max(0.0);
            let size = min_size + (max_size - min_size) * (w / peak).clamp(0.0, 1.0);
            paths(std::slice::from_ref(path), size, steps, |_| class_of(li), |_| tone_of(li))
        })
        .collect()
}

pub fn midpoints(children: &[Vec<usize>], order: &mut [f64], node: usize, next_leaf: &mut f64) {
    if children[node].is_empty() {
        order[node] = *next_leaf;
        *next_leaf += 1.0;
        return;
    }
    for &child in &children[node] {
        midpoints(children, order, child, next_leaf);
    }
    let sum: f64 = children[node].iter().map(|&c| order[c]).sum();
    order[node] = sum / children[node].len() as f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_center_a_cube_on_its_own_point_and_carry_class_and_tone() {
        let out = markers(&[(1.0, 2.0, 3.0)], 0.2, |_| 7, |_| 0.4);
        assert_eq!((out[0].cx, out[0].cy, out[0].z0, out[0].z1), (1.0, 2.0, 2.8, 3.2));
        assert_eq!((out[0].ci, out[0].tone), (7, Some(0.4)));
    }

    #[test]
    fn a_direct_path_interpolates_linearly_between_its_two_points() {
        let path = vec![(0.0, 0.0, 0.0), (10.0, 0.0, 4.0)];
        let out = paths(&[path], 0.1, 5, |_| 0, |_| 0.5);
        assert_eq!(out.len(), 4);
        assert_eq!((out[1].cx, out[1].z0 + 0.1), (4.0, 1.6));
    }

    #[test]
    fn an_elbow_path_follows_its_waypoints_in_order() {
        let elbow = vec![(0.0, 0.0, 0.0), (0.0, 0.0, 5.0), (3.0, 0.0, 5.0)];
        let out = paths(&[elbow], 0.1, 4, |_| 0, |_| 0.0);
        assert_eq!(out.len(), 3);
        assert_eq!((out[0].cx, out[0].z0 + 0.1), (0.0, 2.5));
        assert_eq!((out[2].cx, out[2].z0 + 0.1), (1.5, 5.0));
    }

    #[test]
    fn midpoints_place_leaves_in_order_and_parents_at_the_mean_of_their_children() {
        let children = vec![vec![1, 2], vec![], vec![]];
        let mut order = vec![0.0; 3];
        let mut next_leaf = 0.0;
        midpoints(&children, &mut order, 0, &mut next_leaf);
        assert_eq!((order[1], order[2], order[0]), (0.0, 1.0, 0.5));
        assert_eq!(next_leaf, 2.0);
    }

    #[test]
    fn a_single_point_path_and_an_empty_link_list_draw_nothing() {
        assert!(paths(&[vec![(0.0, 0.0, 0.0)]], 0.1, 4, |_| 0, |_| 0.0).is_empty());
        assert!(paths(&[], 0.1, 4, |_| 0, |_| 0.0).is_empty());
    }

    #[test]
    fn weighted_paths_sizes_each_link_by_its_own_share_of_the_peak_weight() {
        let a = vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let b = vec![(0.0, 1.0, 0.0), (1.0, 1.0, 0.0)];
        let out = weighted_paths(&[a, b], &[1.0, 4.0], 0.1, 0.5, 3, |li| li, |_| 0.0);
        let small = out.iter().find(|b| b.cy < 0.5).unwrap();
        let big = out.iter().find(|b| b.cy > 0.5).unwrap();
        assert!((small.hw - 0.2).abs() < 1e-9);
        assert!((big.hw - 0.5).abs() < 1e-9);
    }
}
