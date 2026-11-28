use crate::html::js_3d::render_3d_html_impl;
use crate::plot::map::regions::RegionSetEntry;
use crate::plot::statistical::common::{push_b, push_hex, push_i};
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn render_flat_map3d_html(
    title: &str,
    region: &RegionSetEntry,
    region_colors: Option<&HashMap<&str, u32>>,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    let map_js = build_flat_map_js(region, region_colors);
    render_3d_html_impl(
        20,
        title,
        x,
        y,
        z,
        axis_labels,
        colors,
        color_labels,
        w,
        h,
        bg_color,
        scene,
        &map_js,
    )
}

fn long_side_of(region: &RegionSetEntry) -> f32 {
    region.svg_width.max(region.svg_height).max(1.0)
}

pub fn flat_xy(region: &RegionSetEntry, normalized: [f32; 2]) -> (f64, f64) {
    let long_side = long_side_of(region);
    let true_x = normalized[0] * region.svg_width;
    let true_y = normalized[1] * region.svg_height;
    let nx = (true_x - region.svg_width / 2.0) / long_side;
    let ny = (region.svg_height / 2.0 - true_y) / long_side;
    (nx as f64, ny as f64)
}

fn build_flat_map_js(region: &RegionSetEntry, region_colors: Option<&HashMap<&str, u32>>) -> Vec<u8> {
    let shapes = (region.all)();
    let mut buf = Vec::with_capacity(60_000);
    push_b(&mut buf, b"var MAP=[");
    let mut first = true;
    let mut fills: Vec<Option<u32>> = Vec::new();
    for shape in shapes {
        let polys = (region.normalize)(shape);
        let fill = region_colors.and_then(|m| m.get(shape.id.as_str()).copied());
        for poly in &polys {
            if poly.len() < 3 {
                continue;
            }
            if !first {
                buf.push(b',');
            }
            first = false;
            buf.push(b'[');
            let mut pfirst = true;
            for pt in poly.iter() {
                let (nx, ny) = flat_xy(region, *pt);
                if !pfirst {
                    buf.push(b',');
                }
                pfirst = false;
                push_i(&mut buf, (nx * 10000.0).round() as i32);
                buf.push(b',');
                push_i(&mut buf, (ny * 10000.0).round() as i32);
            }
            buf.push(b']');
            fills.push(fill);
        }
    }
    push_b(&mut buf, b"];\n");
    if region_colors.is_some() {
        push_b(&mut buf, b"var MAPCOL=[");
        for (idx, fill) in fills.iter().enumerate() {
            if idx > 0 {
                buf.push(b',');
            }
            match fill {
                Some(hex) => {
                    buf.push(b'\'');
                    push_hex(&mut buf, *hex);
                    buf.push(b'\'');
                }
                None => push_b(&mut buf, b"null"),
            }
        }
        push_b(&mut buf, b"];\n");
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::map::regions;

    #[test]
    fn build_flat_map_js_carries_one_polygon_entry_per_shape_ring() {
        let region = regions::resolve("usa_states").unwrap();
        let js = String::from_utf8(build_flat_map_js(region, None)).unwrap();
        assert!(js.starts_with("var MAP=[["));
        assert!(!js.contains("MAPCOL"));
    }


    #[test]
    fn build_flat_map_js_emits_a_color_per_ring_when_region_colors_are_given() {
        let region = regions::resolve("usa_states").unwrap();
        let mut colors: HashMap<&str, u32> = HashMap::new();
        colors.insert("CA", 0xff0000);
        let js = String::from_utf8(build_flat_map_js(region, Some(&colors))).unwrap();
        assert!(js.contains("var MAPCOL=["));
        assert!(js.contains("'#ff0000'"));
    }

    #[test]
    fn flat_xy_centers_the_middle_of_the_viewbox_at_the_origin() {
        let region = regions::resolve("usa_states").unwrap();
        let (cx, cy) = flat_xy(region, [0.5, 0.5]);
        assert!(cx.abs() < 1e-6 && cy.abs() < 1e-6);
    }

    #[test]
    fn flat_xy_preserves_the_true_aspect_ratio_of_the_viewbox() {
        let region = regions::resolve("usa_states").unwrap();
        let (right_x, _) = flat_xy(region, [1.0, 0.5]);
        let long_side = long_side_of(region) as f64;
        let expected = (region.svg_width as f64 / 2.0) / long_side;
        assert!((right_x - expected).abs() < 1e-6);
        let (_, top_y) = flat_xy(region, [0.5, 0.0]);
        let expected_y = (region.svg_height as f64 / 2.0) / long_side;
        assert!((top_y - expected_y).abs() < 1e-6);
    }
}
