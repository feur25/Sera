use super::common::{build_grid, field_bounds, grid_to_pixel, lerp_rgb, push_base_outlines, svg_open, to_html, GRID_COLS, GRID_ROWS};
use super::config::ContourMapConfig;

fn edge_crossing(v0: f64, v1: f64, p0: (f32, f32), p1: (f32, f32), t: f64) -> Option<(f32, f32)> {
    if (v0 - t) * (v1 - t) > 0.0 {
        return None;
    }
    if (v1 - v0).abs() < 1e-12 {
        return None;
    }
    let frac = ((t - v0) / (v1 - v0)) as f32;
    Some((p0.0 + (p1.0 - p0.0) * frac, p0.1 + (p1.1 - p0.1) * frac))
}

#[crate::chart_demo(
    "lats=[40.7,34.0,41.8,29.7,47.6,25.7,39.9,32.7,45.5,38.9], lons=[-74.0,-118.2,-87.6,-95.3,-122.3,-80.2,-75.1,-96.8,-122.6,-77.0], field=[12,24,8,28,6,30,15,26,9,18], title=\"Isobars, Sampled Pressure Anomaly\", variant=\"isolines\", levels=7"
)]
pub fn render(cfg: &ContourMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.field.len());
    if n == 0 {
        return String::new();
    }
    let bounds = field_bounds(cfg);
    let span = (bounds.max_val - bounds.min_val).max(1e-9);
    let grid = build_grid(cfg, &bounds, GRID_COLS, GRID_ROWS);
    let pixels: Vec<Vec<(f32, f32)>> = (0..GRID_ROWS)
        .map(|row| (0..GRID_COLS).map(|col| grid_to_pixel(cfg, &bounds, col, GRID_COLS, row, GRID_ROWS)).collect())
        .collect();

    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    let levels = cfg.levels.max(2);
    for level_idx in 1..levels {
        let t = bounds.min_val + span * (level_idx as f64 / levels as f64);
        let band_t = (t - bounds.min_val) / span;
        let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, band_t);
        let mut path = String::new();

        for row in 0..GRID_ROWS - 1 {
            for col in 0..GRID_COLS - 1 {
                let tl = grid[row][col];
                let tr = grid[row][col + 1];
                let br = grid[row + 1][col + 1];
                let bl = grid[row + 1][col];
                let p_tl = pixels[row][col];
                let p_tr = pixels[row][col + 1];
                let p_br = pixels[row + 1][col + 1];
                let p_bl = pixels[row + 1][col];

                let top = edge_crossing(tl, tr, p_tl, p_tr, t);
                let right = edge_crossing(tr, br, p_tr, p_br, t);
                let bottom = edge_crossing(bl, br, p_bl, p_br, t);
                let left = edge_crossing(tl, bl, p_tl, p_bl, t);
                let found: Vec<(f32, f32)> = [top, right, bottom, left].into_iter().flatten().collect();

                let mut push_seg = |a: (f32, f32), c: (f32, f32)| {
                    path.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", a.0, a.1, c.0, c.1));
                };
                match found.len() {
                    2 => push_seg(found[0], found[1]),
                    4 => {
                        let center = (tl + tr + br + bl) / 4.0;
                        if center >= t {
                            push_seg(top.unwrap(), right.unwrap());
                            push_seg(bottom.unwrap(), left.unwrap());
                        } else {
                            push_seg(top.unwrap(), left.unwrap());
                            push_seg(right.unwrap(), bottom.unwrap());
                        }
                    }
                    _ => {}
                }
            }
        }

        if !path.is_empty() {
            svg.push_str(&format!(
                "<path d=\"{path}\" fill=\"none\" stroke=\"rgb({r},{g},{b})\" stroke-width=\"1.6\" opacity=\"0.9\"/>"
            ));
        }
    }

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
