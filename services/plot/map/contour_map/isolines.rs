use super::common::{build_grid, draw_isoline_paths, field_bounds, grid_to_pixel, push_base_outlines, svg_open, to_html, GRID_COLS, GRID_ROWS};
use super::config::ContourMapConfig;

#[crate::chart_demo(
    "lats=[65,55,35,55,32,-28,-28,-30,55,2,2,0,-57,-57,-57,40,50,35,-25,40,-5,55], lons=[-20,-165,-25,90,-140,-105,-5,70,-100,20,-60,110,-60,90,170,15,-35,140,135,-95,25,-10], field=[-12,-10,11,15,9,10,8,9,6,-3,-4,-5,-9,-8,-7,3,-2,2,7,1,-2,-4], title=\"Idealized Global Sea-Level Pressure Anomaly (hPa)\", variant=\"isolines\", bins=7"
)]
pub fn render(cfg: &ContourMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    if n == 0 {
        return String::new();
    }
    let bounds = field_bounds(cfg);
    let grid = build_grid(cfg, &bounds, GRID_COLS, GRID_ROWS);
    let pixels: Vec<Vec<(f32, f32)>> = (0..GRID_ROWS)
        .map(|row| (0..GRID_COLS).map(|col| grid_to_pixel(cfg, &bounds, col, GRID_COLS, row, GRID_ROWS)).collect())
        .collect();

    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);
    draw_isoline_paths(&mut svg, &grid, &pixels, &bounds, cfg.levels, cfg.color_low, cfg.color_high, 1.6);

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
