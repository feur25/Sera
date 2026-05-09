use crate::html::js_3d::render_3d_html_impl;
use crate::plot::statistical::common::{push_b, push_i};
use crate::plot::map::world_data;

pub fn render_globe3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    let map_js = build_globe_map_js();
    render_3d_html_impl(15, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color, &map_js)
}

fn build_globe_map_js() -> Vec<u8> {
    let countries = world_data::all_countries();
    let mut buf = Vec::with_capacity(120_000);
    push_b(&mut buf, b"var MAP=[");
    let mut first = true;
    for country in countries {
        let polys = world_data::normalized_polygons(country);
        for poly in &polys {
            if poly.len() < 3 { continue; }
            let step = if poly.len() > 200 { 5 }
                       else if poly.len() > 80 { 3 }
                       else if poly.len() > 30 { 2 }
                       else { 1 };
            if !first { buf.push(b','); }
            first = false;
            buf.push(b'[');
            let mut pfirst = true;
            for (idx, pt) in poly.iter().enumerate() {
                if idx % step != 0 && idx != poly.len() - 1 { continue; }
                if !pfirst { buf.push(b','); }
                pfirst = false;
                push_i(&mut buf, (pt[0] * 10000.0).round() as i32);
                buf.push(b',');
                push_i(&mut buf, (pt[1] * 10000.0).round() as i32);
            }
            buf.push(b']');
        }
    }
    push_b(&mut buf, b"];\n");
    buf
}


