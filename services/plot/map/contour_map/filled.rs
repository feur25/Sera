use super::common::{build_grid, field_bounds, grid_to_pixel, lerp_rgb, push_base_outlines, svg_open, to_html, GRID_COLS, GRID_ROWS};
use super::config::ContourMapConfig;

#[crate::chart_demo(
    "lats=[64.1,61.2,68.9,62.0,59.9,55.75,51.5,52.5,52.2,40.7,41.9,48.9,39.9,37.6,35.7,40.4,41.9,34.0,31.2,30.0,28.6,29.8,25.2,25.8,19.08,13.75,6.5,1.35,-1.3,-6.2,-23.5,-26.2,-33.9,-33.9,-34.6,-43.5,-54.8,-57.0], lons=[-21.9,-149.9,33.0,129.7,10.75,37.6,-0.12,13.4,21.0,-74.0,-87.6,2.35,116.4,127.0,139.7,-3.7,12.5,-118.2,121.5,31.2,77.2,-95.4,55.3,-80.2,72.88,100.5,3.4,103.8,36.8,106.8,-46.6,28.0,151.2,18.4,-58.4,172.6,-68.3,-63.0], field=[4,2,-3,-15,6,5,10,9,8,12,10,11,13,13,16,15,16,18,17,22,25,21,29,25,28,29,27,27,19,27,20,16,18,17,17,12,6,-10], title=\"Global Surface Temperature (C)\""
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
