use super::common::{build_grid, field_bounds, find_extrema};
use super::config::ContourMapConfig;
use super::variant::ContourMapVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget, SAMPLE_CAP};
use crate::plot::statistical::_3d::grid::{rect_cells, CellField};
use crate::plot::statistical::_3d::lineage::{markers, Point};
use crate::plot::statistical::bar::Bar3DBlock;

const GRID_N: usize = 20;
const HEIGHT_SCALE: f64 = 3.2;
const MARK_HW: f64 = 0.3;

fn contour_map_3d(cfg: &ContourMapConfig) -> (Vec<Bar3DBlock>, Vec<String>) {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    if n == 0 {
        return (Vec::new(), Vec::new());
    }

    let capped_lats: Vec<f64>;
    let capped_lons: Vec<f64>;
    let capped_field: Vec<f64>;
    let scoped_cfg: ContourMapConfig;
    let cfg = if n > SAMPLE_CAP {
        let keep = even_indices(n, SAMPLE_CAP);
        capped_lats = pick(cfg.lats, &keep);
        capped_lons = pick(cfg.lons, &keep);
        capped_field = pick(cfg.field, &keep);
        scoped_cfg = ContourMapConfig {
            variant: cfg.variant,
            title: cfg.title,
            lats: &capped_lats,
            lons: &capped_lons,
            field: &capped_field,
            width: cfg.width,
            height: cfg.height,
            levels: cfg.levels,
            color_low: cfg.color_low,
            color_high: cfg.color_high,
        };
        &scoped_cfg
    } else {
        cfg
    };

    let bounds = field_bounds(cfg);
    let grid = build_grid(cfg, &bounds, GRID_N, GRID_N);
    let peak = grid.iter().flatten().copied().fold(1e-12, f64::max);
    let floor = grid.iter().flatten().copied().fold(f64::INFINITY, f64::min);
    let span = (peak - floor).max(1e-12);

    let heights: Vec<f64> = grid.iter().flatten().map(|&v| ((v - floor) / span).max(0.02) * HEIGHT_SCALE).collect();
    let tones: Vec<f64> = grid.iter().flatten().map(|&v| ((v - floor) / span).clamp(0.0, 1.0)).collect();
    let classes: Vec<usize> = (0..GRID_N * GRID_N).collect();
    let field = CellField { n_rows: GRID_N, n_cols: GRID_N, heights: &heights, tones: &tones, classes: &classes };
    let mut blocks = rect_cells(&field, &[], &[], &[], 0.94);
    let mut names: Vec<String> = (0..GRID_N * GRID_N).map(|i| format!("Cell {i}")).collect();

    if cfg.variant == ContourMapVariant::Extrema {
        let extrema = find_extrema(&grid, 3);
        let marker_positions: Vec<Point> = extrema
            .iter()
            .map(|&(r, c, _)| {
                let cx = c as f64 + 0.5;
                let cy = r as f64 + 0.5;
                let v = grid[r][c];
                let h = ((v - floor) / span).max(0.02) * HEIGHT_SCALE + 0.6;
                (cx, cy, h)
            })
            .collect();
        let base = blocks.len();
        blocks.extend(markers(&marker_positions, MARK_HW, |i| base + i, |i| if extrema[i].2 { 1.0 } else { 0.0 }));
        names.extend(extrema.iter().map(|&(_, _, is_high)| if is_high { "High".to_string() } else { "Low".to_string() }));
    }

    (blocks, names)
}

pub fn layout_3d(cfg: &ContourMapConfig, _budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, _budget).0
}

pub fn layout_named(cfg: &ContourMapConfig, _budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    contour_map_3d(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let n = 30;
        let lats: Vec<f64> = (0..n).map(|i| (i as f64 * 3.7) % 60.0 - 30.0).collect();
        let lons: Vec<f64> = (0..n).map(|i| (i as f64 * 5.3) % 120.0 - 60.0).collect();
        let field: Vec<f64> = (0..n).map(|i| ((i as f64 * 0.3).sin() * 20.0 + 20.0).abs()).collect();
        (lats, lons, field)
    }

    fn draw(variant: ContourMapVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let (lats, lons, field) = field();
        let cfg = ContourMapConfig {
            variant,
            title: "t",
            lats: &lats,
            lons: &lons,
            field: &field,
            width: 1200,
            height: 650,
            levels: 6,
            color_low: 0x1e3a8a,
            color_high: 0xdc2626,
        };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_variant_draws_a_full_grid_surface() {
        for &variant in ContourMapVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(blocks.len() >= GRID_N * GRID_N, "{variant:?}");
            assert_eq!(names.len(), blocks.len(), "{variant:?}");
        }
    }

    #[test]
    fn extrema_adds_marker_blocks_on_top_of_the_grid_surface() {
        let (filled, _) = draw(ContourMapVariant::Filled);
        let (extrema, _) = draw(ContourMapVariant::Extrema);
        assert!(extrema.len() > filled.len());
    }

    #[test]
    fn a_huge_point_count_is_sampled_before_the_quadratic_idw_grid_runs() {
        let n = 5000;
        let lats: Vec<f64> = (0..n).map(|i| (i as f64 * 0.037) % 60.0 - 30.0).collect();
        let lons: Vec<f64> = (0..n).map(|i| (i as f64 * 0.053) % 120.0 - 60.0).collect();
        let field: Vec<f64> = (0..n).map(|i| (i % 100) as f64).collect();
        let cfg = ContourMapConfig {
            variant: ContourMapVariant::Filled,
            title: "t",
            lats: &lats,
            lons: &lons,
            field: &field,
            width: 1200,
            height: 650,
            levels: 6,
            color_low: 0x1e3a8a,
            color_high: 0xdc2626,
        };
        let t0 = std::time::Instant::now();
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(!blocks.is_empty());
        assert!(t0.elapsed().as_secs() < 5, "took {:?}", t0.elapsed());
    }

    #[test]
    fn empty_input_draws_nothing() {
        let cfg = ContourMapConfig { variant: ContourMapVariant::Filled, title: "t", lats: &[], lons: &[], field: &[], width: 1200, height: 650, levels: 6, color_low: 0, color_high: 0 };
        let (blocks, names) = layout_named(&cfg, &Budget::default());
        assert!(blocks.is_empty() && names.is_empty());
    }
}
