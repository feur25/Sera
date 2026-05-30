use crate::plot::{parse_all, apply_bg3d};
use crate::html::js_3d::render_3d_html;

pub fn render_radar3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(3, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}



#[crate::sera_alias("radar3d", "radar_3d", "radar3d_chart", "radar3d_family")]
#[crate::sera_builder]
pub fn build_radar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let n_axes = axes.len();
    if n_axes == 0 { return String::new(); }
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let n_series = names.len().min(series_flat.len());
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new(); let mut cv = Vec::new();
    for si in 0..n_series {
        let vals = &series_flat[si];
        let max_val = vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);
        for ai in 0..n_axes.min(vals.len()) {
            let angle = std::f64::consts::TAU * ai as f64 / n_axes as f64;
            let r = vals[ai] / max_val;
            xv.push(angle.cos() * r); yv.push(si as f64); zv.push(angle.sin() * r); cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_radar3d_html(
        title, &xv, &yv, &zv, (&o.xl(), &o.yl(), &o.zl()), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}
