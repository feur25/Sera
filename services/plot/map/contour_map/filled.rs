use super::common::{build_grid, field_bounds, grid_to_pixel, lerp_rgb, push_base_outlines, svg_open, to_html, GRID_COLS, GRID_ROWS};
use super::config::ContourMapConfig;

#[crate::chart_demo(
    "lats=[40.7,34.0,41.8,29.7,47.6,25.7,39.9,32.7,45.5,38.9], lons=[-74.0,-118.2,-87.6,-95.3,-122.3,-80.2,-75.1,-96.8,-122.6,-77.0], field=[12,24,8,28,6,30,15,26,9,18], title=\"Sampled Surface Temperature (C)\""
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
