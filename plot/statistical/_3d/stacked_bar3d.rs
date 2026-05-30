use crate::plot::{parse_all, apply_bg3d};
use crate::html::js_3d::render_3d_html;

pub fn render_stacked_bar3d_html(
    title: &str,
    x: &[f64], y: &[f64], z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32, h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(14, title, x, y, z, axis_labels, colors, color_labels, w, h, bg_color)
}



#[crate::sera_alias("stacked_bar3d", "stacked_bar_3d", "stacked_bar3d_chart")]
#[crate::sera_builder]
pub fn build_stacked_bar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
    let n_cat = category_labels.len();
    let n_ser = series_values.len();
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..n_ser).map(|_| String::new()).collect());
    let mut xv = Vec::new(); let mut yv = Vec::new(); let mut zv = Vec::new();
    let mut cv = Vec::new(); let mut cl = Vec::new();
    for ci in 0..n_cat {
        for si in 0..n_ser {
            let v = series_values[si].get(ci).copied().unwrap_or(0.0);
            xv.push(ci as f64); yv.push(si as f64); zv.push(v);
            cv.push(si as f64); cl.push(format!("{}/{}", category_labels[ci], names[si]));
        }
    }
    let xl = o.xl(); let yl = o.yl(); let zl = o.zl();
    let x_lbl = if xl.is_empty() { "Category" } else { &xl };
    let y_lbl = if yl.is_empty() { "Series" } else { &yl };
    let z_lbl = if zl == "Z" { "Value".to_string() } else { zl };
    let bg_str = o.bg_str();
    apply_bg3d(crate::plot::statistical::_3d::render_stacked_bar3d_html(
        title, &xv, &yv, &zv, (x_lbl, y_lbl, &z_lbl), &cv, &names,
        o.w(900), o.h(560), bg_str.as_deref(),
    ), &o)
}
