use super::canvas::{blank_canvas, draw_axes, fill_rect, plot_area, DOWN_COLOR, UP_COLOR};
use image::{Rgba, RgbaImage};

pub(super) type TouchedRow = (u32, Vec<i16>);

pub(super) enum RenderKind {
    Scalar { axis_px: i32 },
    Point { plot_w: i32, plot_h: i32 },
    Vector { axis_px: i32, vpp: u32 },
}

pub(super) fn render_frame(kind: &RenderKind, width: u32, height: u32, touched: &[TouchedRow], color: Rgba<u8>) -> RgbaImage {
    match kind {
        RenderKind::Scalar { axis_px } => render_bar_frame(width, height, touched, *axis_px, color),
        RenderKind::Point { plot_w, plot_h } => render_point_frame(width, height, touched, *plot_w, *plot_h, color),
        RenderKind::Vector { axis_px, vpp } => render_candle_frame(width, height, touched, *axis_px, *vpp, color),
    }
}

fn value_slots(len: usize) -> (i64, f64) {
    let n = len.max(1) as i64;
    (n, 1.0 / n as f64)
}

pub(super) fn render_bar_frame(width: u32, height: u32, touched: &[TouchedRow], axis_px: i32, color: Rgba<u8>) -> RgbaImage {
    let mut img = blank_canvas(width, height);
    let (left, top, plot_w, plot_h) = plot_area(width, height);
    let baseline_y = top + plot_h;
    let axis_px = axis_px.max(1) as i64;
    draw_axes(&mut img, left, top, plot_w, plot_h);

    let (n, _) = value_slots(touched.len());
    let slot_w = plot_w as f64 / n as f64;
    for (i, (_, values)) in touched.iter().enumerate() {
        let h_px = values.first().copied().unwrap_or(0).max(0) as i64;
        let scaled_h = (h_px * plot_h) / axis_px;
        let x0 = left as f64 + slot_w * i as f64 + slot_w * 0.15;
        let x1 = left as f64 + slot_w * (i as f64 + 1.0) - slot_w * 0.15;
        fill_rect(&mut img, x0 as i64, baseline_y - scaled_h, x1 as i64, baseline_y, color);
    }
    img
}

pub(super) fn render_point_frame(width: u32, height: u32, touched: &[TouchedRow], plot_w_orig: i32, plot_h_orig: i32, color: Rgba<u8>) -> RgbaImage {
    let mut img = blank_canvas(width, height);
    let (left, top, plot_w, plot_h) = plot_area(width, height);
    draw_axes(&mut img, left, top, plot_w, plot_h);

    let sx = plot_w as f64 / (plot_w_orig.max(1) as f64);
    let sy = plot_h as f64 / (plot_h_orig.max(1) as f64);
    let radius = 4i64;
    for (_, values) in touched {
        let (Some(&vx), Some(&vy)) = (values.first(), values.get(1)) else { continue };
        let cx = left as f64 + (vx.max(0) as f64) * sx;
        let cy = top as f64 + (vy.max(0) as f64) * sy;
        fill_rect(&mut img, cx as i64 - radius, cy as i64 - radius, cx as i64 + radius, cy as i64 + radius, color);
    }
    img
}

pub(super) fn render_candle_frame(width: u32, height: u32, touched: &[TouchedRow], axis_px: i32, vpp: u32, color: Rgba<u8>) -> RgbaImage {
    let mut img = blank_canvas(width, height);
    let (left, top, plot_w, plot_h) = plot_area(width, height);
    let baseline_y = top + plot_h;
    let axis_px = axis_px.max(1) as i64;
    draw_axes(&mut img, left, top, plot_w, plot_h);

    let (n, _) = value_slots(touched.len());
    let slot_w = plot_w as f64 / n as f64;
    let to_y = |px_val: i64| -> i64 { baseline_y - (px_val * plot_h) / axis_px };

    for (i, (_, values)) in touched.iter().enumerate() {
        let x0 = left as f64 + slot_w * i as f64 + slot_w * 0.15;
        let x1 = left as f64 + slot_w * (i as f64 + 1.0) - slot_w * 0.15;

        if vpp == 4 && values.len() == 4 {
            let (open, high, low, close) = (values[0].max(0) as i64, values[1].max(0) as i64, values[2].max(0) as i64, values[3].max(0) as i64);
            let up = close >= open;
            let body_color = if up { UP_COLOR } else { DOWN_COLOR };
            let mid = ((x0 + x1) / 2.0) as i64;
            fill_rect(&mut img, mid, to_y(high), mid + 1, to_y(low) + 1, body_color);
            let (y_open, y_close) = (to_y(open), to_y(close));
            let (body_top, body_bottom) = if up { (y_close, y_open) } else { (y_open, y_close) };
            fill_rect(&mut img, x0 as i64, body_top, x1 as i64, body_bottom.max(body_top + 1), body_color);
        } else {
            let sub_n = values.len().max(1) as f64;
            let sub_w = (x1 - x0) / sub_n;
            for (k, &v) in values.iter().enumerate() {
                let y = to_y(v.max(0) as i64);
                let sx0 = x0 + sub_w * k as f64;
                let sx1 = sx0 + sub_w * 0.8;
                fill_rect(&mut img, sx0 as i64, y, sx1 as i64, baseline_y, color);
            }
        }
    }
    img
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::canvas::BAR_COLOR;

    fn row(idx: u32, values: Vec<i16>) -> TouchedRow {
        (idx, values)
    }

    #[test]
    fn render_bar_frame_produces_the_requested_dimensions() {
        let img = render_bar_frame(200, 100, &[row(0, vec![50]), row(1, vec![100]), row(2, vec![0])], 100, BAR_COLOR);
        assert_eq!((img.width(), img.height()), (200, 100));
    }

    #[test]
    fn taller_recorded_pixel_value_produces_a_taller_bar() {
        let short = render_bar_frame(200, 100, &[row(0, vec![10])], 100, BAR_COLOR);
        let tall = render_bar_frame(200, 100, &[row(0, vec![90])], 100, BAR_COLOR);
        let count = |img: &RgbaImage| img.pixels().filter(|p| **p == BAR_COLOR).count();
        assert!(count(&tall) > count(&short));
    }

    #[test]
    fn zero_height_bar_paints_no_bar_pixels() {
        let img = render_bar_frame(200, 100, &[row(0, vec![0]), row(1, vec![0])], 100, BAR_COLOR);
        assert_eq!(img.pixels().filter(|p| **p == BAR_COLOR).count(), 0);
    }

    #[test]
    fn a_single_touched_index_among_many_possible_ones_fills_the_whole_width_not_a_tiny_sliver() {
        let one_bar = render_bar_frame(800, 450, &[row(3, vec![90])], 100, BAR_COLOR);
        let five_hundred_dense = {
            let mut rows: Vec<TouchedRow> = (0..500).map(|i| row(i, vec![0])).collect();
            rows[3] = row(3, vec![90]);
            rows
        };
        let dense_frame = render_bar_frame(800, 450, &five_hundred_dense, 100, BAR_COLOR);
        let count = |img: &RgbaImage| img.pixels().filter(|p| **p == BAR_COLOR).count();
        assert!(count(&one_bar) > count(&dense_frame) * 10);
    }

    #[test]
    fn point_frame_moves_the_dot_when_the_recorded_position_changes() {
        let a = render_point_frame(400, 300, &[row(0, vec![10, 10])], 400, 300, BAR_COLOR);
        let b = render_point_frame(400, 300, &[row(0, vec![390, 290])], 400, 300, BAR_COLOR);
        assert_ne!(a, b, "moving the recorded x/y pixel position must change the rendered frame");
    }

    #[test]
    fn point_frame_with_no_values_draws_no_dot_and_does_not_panic() {
        let img = render_point_frame(400, 300, &[(0, vec![])], 400, 300, BAR_COLOR);
        assert_eq!(img.pixels().filter(|p| **p == BAR_COLOR).count(), 0);
    }

    #[test]
    fn candle_body_color_reflects_gain_vs_loss() {
        let gain = render_candle_frame(400, 300, &[row(0, vec![20, 80, 10, 70])], 100, 4, BAR_COLOR);
        let loss = render_candle_frame(400, 300, &[row(0, vec![70, 80, 10, 20])], 100, 4, BAR_COLOR);
        let up_pixels = |img: &RgbaImage| img.pixels().filter(|p| **p == UP_COLOR).count();
        let down_pixels = |img: &RgbaImage| img.pixels().filter(|p| **p == DOWN_COLOR).count();
        assert!(up_pixels(&gain) > 0 && down_pixels(&gain) == 0, "close > open should paint only the up color");
        assert!(down_pixels(&loss) > 0 && up_pixels(&loss) == 0, "close < open should paint only the down color");
    }

    #[test]
    fn candle_frame_with_non_ohlc_vpp_falls_back_to_grouped_mini_bars_without_panicking() {
        let img = render_candle_frame(400, 300, &[row(0, vec![10, 50, 90])], 100, 3, BAR_COLOR);
        assert_eq!((img.width(), img.height()), (400, 300));
        assert!(img.pixels().filter(|p| **p == BAR_COLOR).count() > 0);
    }

    #[test]
    fn candle_frame_handles_a_doji_open_equals_close_without_a_zero_height_body_disappearing() {
        let img = render_candle_frame(400, 300, &[row(0, vec![50, 80, 20, 50])], 100, 4, BAR_COLOR);
        let up_or_down = img.pixels().filter(|p| **p == UP_COLOR || **p == DOWN_COLOR).count();
        assert!(up_or_down > 0, "a doji (open == close) should still paint a visible thin body, not vanish");
    }
}
