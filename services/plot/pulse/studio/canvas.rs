use image::{Rgba, RgbaImage};

pub(super) const BG: Rgba<u8> = Rgba([255, 255, 255, 255]);
pub(super) const AXIS_COLOR: Rgba<u8> = Rgba([148, 163, 184, 255]);
pub(super) const BAR_COLOR: Rgba<u8> = Rgba([99, 110, 250, 255]);
pub(super) const DEFAULT_RGB: (u8, u8, u8) = (BAR_COLOR.0[0], BAR_COLOR.0[1], BAR_COLOR.0[2]);
pub(super) const UP_COLOR: Rgba<u8> = Rgba([34, 197, 94, 255]);
pub(super) const DOWN_COLOR: Rgba<u8> = Rgba([239, 68, 68, 255]);
pub(super) const PAD_L: u32 = 24;
pub(super) const PAD_T: u32 = 16;
pub(super) const PAD_B: u32 = 24;
pub(super) const PAD_R: u32 = 16;

pub(super) fn to_rgba(rgb: (u8, u8, u8)) -> Rgba<u8> {
    Rgba([rgb.0, rgb.1, rgb.2, 255])
}

pub(super) fn blank_canvas(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_pixel(width, height, BG)
}

pub(super) fn fill_rect(img: &mut RgbaImage, x0: i64, y0: i64, x1: i64, y1: i64, color: Rgba<u8>) {
    let (w, h) = (img.width() as i64, img.height() as i64);
    let x0 = x0.clamp(0, w);
    let x1 = x1.clamp(0, w);
    let y0 = y0.clamp(0, h);
    let y1 = y1.clamp(0, h);
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    for y in y0..y1 {
        for x in x0..x1 {
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

pub(super) fn draw_axes(img: &mut RgbaImage, left: i64, top: i64, plot_w: i64, plot_h: i64) {
    fill_rect(img, left, top, left + 1, top + plot_h + 1, AXIS_COLOR);
    fill_rect(img, left, top + plot_h, left + plot_w, top + plot_h + 1, AXIS_COLOR);
}

pub(super) fn plot_area(width: u32, height: u32) -> (i64, i64, i64, i64) {
    let plot_w = width.saturating_sub(PAD_L + PAD_R).max(1) as i64;
    let plot_h = height.saturating_sub(PAD_T + PAD_B).max(1) as i64;
    (PAD_L as i64, PAD_T as i64, plot_w, plot_h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_rect_clips_to_image_bounds_without_panicking() {
        let mut img = blank_canvas(10, 10);
        fill_rect(&mut img, -5, -5, 5, 5, BAR_COLOR);
        assert_eq!(*img.get_pixel(0, 0), BAR_COLOR);
        fill_rect(&mut img, 8, 8, 100, 100, BAR_COLOR);
        assert_eq!(*img.get_pixel(9, 9), BAR_COLOR);
    }

    #[test]
    fn fill_rect_is_a_noop_for_degenerate_or_inverted_ranges() {
        let mut img = blank_canvas(10, 10);
        fill_rect(&mut img, 5, 5, 5, 5, BAR_COLOR);
        fill_rect(&mut img, 8, 2, 2, 8, BAR_COLOR);
        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(*img.get_pixel(x, y), BG, "no pixel should have changed for degenerate rects");
            }
        }
    }

    #[test]
    fn to_rgba_carries_the_given_channels_at_full_opacity() {
        assert_eq!(to_rgba((10, 20, 30)), Rgba([10, 20, 30, 255]));
    }

    #[test]
    fn plot_area_is_never_smaller_than_one_pixel_even_for_a_tiny_canvas() {
        let (_, _, w, h) = plot_area(5, 5);
        assert!(w >= 1 && h >= 1);
    }
}
