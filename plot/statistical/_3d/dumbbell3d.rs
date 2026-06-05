use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_dumbbell3d_html(
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
) -> String {
    render_3d_html(
        11,
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
    )
}

#[crate::sera_alias("dumbbell3d", "dumbbell_3d", "dumbbell3d_chart")]
#[crate::sera_builder]
pub fn build_dumbbell3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    let s_name = o
        .series_name_start
        .as_deref()
        .unwrap_or("Start")
        .to_string();
    let e_name = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let yl = o.yl();
    let y_lbl = if yl.is_empty() { "Item" } else { &yl };
    let n = labels.len().min(values_start.len()).min(values_end.len());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for i in 0..n {
        xv.push(values_start[i]);
        yv.push(i as f64);
        zv.push(values_end[i]);
        cv.push(i as f64);
    }
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_dumbbell3d_html(
            title,
            &xv,
            &yv,
            &zv,
            (&s_name, y_lbl, &e_name),
            &cv,
            &labels,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
        ),
        &o,
    )
}
