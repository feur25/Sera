use crate::html::js_3d::render_3d_html;

pub fn render_pie3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(7, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}


