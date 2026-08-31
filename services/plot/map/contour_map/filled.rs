use super::common::{build_grid, field_bounds, grid_to_pixel, lerp_rgb, push_base_outlines, svg_open, to_html, GRID_COLS, GRID_ROWS};
use super::config::ContourMapConfig;

#[crate::chart_demo(
    "lats=[47.6,45.5,37.8,34.0,32.7,36.2,33.4,39.7,40.8,44.9,41.9,42.3,39.1,38.6,36.2,33.7,25.8,30.0,29.8,32.8,39.8,39.9,40.4,40.7,42.4,38.9,35.2,43.7,43.6,35.1], lons=[-122.3,-122.6,-122.4,-118.2,-117.2,-115.1,-112.1,-105.0,-111.9,-93.3,-87.6,-83.0,-94.6,-90.2,-86.8,-84.4,-80.2,-90.1,-95.4,-96.8,-86.2,-83.0,-80.0,-74.0,-71.1,-77.0,-80.8,-70.3,-116.2,-106.6], field=[12,13,16,22,21,24,28,10,8,2,6,5,9,11,15,19,27,23,24,20,7,6,5,8,4,10,16,2,9,17], title=\"Continental U.S. Surface Temperature (C)\""
)]
pub fn render(cfg: &ContourMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    if n == 0 {
        return String::new();
    }
    let bounds = field_bounds(cfg);
    let span = (bounds.max_val - bounds.min_val).max(1e-9);
    let grid = build_grid(cfg, &bounds, GRID_COLS, GRID_ROWS);

    let (cx0, cy0) = grid_to_pixel(cfg, &bounds, 0, GRID_COLS, 0, GRID_ROWS);
    let (cx1, cy1) = grid_to_pixel(cfg, &bounds, 1, GRID_COLS, 1, GRID_ROWS);
    let cell_w = (cx1 - cx0).abs().max(1.0);
    let cell_h = (cy1 - cy0).abs().max(1.0);

    let mut svg = svg_open(cfg.width, cfg.height);
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            let value = grid[row][col];
            let t = (value - bounds.min_val) / span;
            let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, t);
            let (px, py) = grid_to_pixel(cfg, &bounds, col, GRID_COLS, row, GRID_ROWS);
            svg.push_str(&format!(
                "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"rgb({r},{g},{b})\" opacity=\"0.72\"/>",
                px - cell_w / 2.0,
                py - cell_h / 2.0,
                cell_w + 0.6,
                cell_h + 0.6,
            ));
        }
    }
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    for i in 0..n {
        let (nx, ny) = crate::plot::map::world_data::latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
        let px = nx * cfg.width as f32;
        let py = ny * cfg.height as f32;
        svg.push_str(&format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.2\" fill=\"#ffffff\" stroke=\"#0d1117\" stroke-width=\"1\" data-index=\"{i}\"/>"
        ));
    }

    to_html(cfg, svg)
}
