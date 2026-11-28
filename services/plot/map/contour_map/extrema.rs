use super::common::{build_grid, draw_isoline_paths, field_bounds, find_extrema, grid_to_pixel, push_base_outlines, svg_open, to_html, GRID_COLS, GRID_ROWS};
use super::config::ContourMapConfig;

#[crate::chart_demo(
    "lats=[65,55,35,55,32,-28,-28,-30,55,2,2,0,-57,-57,-57,40,50,35,-25,40,-5,55], lons=[-20,-165,-25,90,-140,-105,-5,70,-100,20,-60,110,-60,90,170,15,-35,140,135,-95,25,-10], field=[-12,-10,11,15,9,10,8,9,6,-3,-4,-5,-9,-8,-7,3,-2,2,7,1,-2,-4], title=\"Pressure Highs and Lows\", variant=\"extrema\", bins=5"
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
    draw_isoline_paths(&mut svg, &grid, &pixels, &bounds, cfg.levels, cfg.color_low, cfg.color_high, 0.9);

    for (row, col, is_high) in find_extrema(&grid, 9) {
        let (px, py) = pixels[row][col];
        let value = grid[row][col];
        let (letter, color) = if is_high { ("H", "#dc2626") } else { ("L", "#2563eb") };
        svg.push_str(&format!(
            "<text x=\"{px:.1}\" y=\"{py:.1}\" fill=\"{color}\" font-size=\"22\" font-weight=\"800\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"Arial,sans-serif\" data-index=\"{row}-{col}\">{letter}</text>"
        ));
        svg.push_str(&format!(
            "<text x=\"{px:.1}\" y=\"{:.1}\" fill=\"{color}\" font-size=\"10\" font-weight=\"700\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\">{value:.0}</text>",
            py + 15.0
        ));
    }

    for i in 0..n {
        let (nx, ny) = crate::plot::map::world_data::latlon_to_normalized(cfg.lats[i], cfg.lons[i]);
        let px = nx * cfg.width as f32;
        let py = ny * cfg.height as f32;
        svg.push_str(&format!(
            "<circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"2.2\" fill=\"#94a3b8\" stroke=\"#0d1117\" stroke-width=\"0.6\" data-index=\"{i}\"/>"
        ));
    }

    to_html(cfg, svg)
}
