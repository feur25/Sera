use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_funnel3d_html(
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
    scene: &str,
) -> String {
    render_3d_html(
        12,
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
    )
}

#[crate::chart_demo("labels=["Visit","Signup","Purchase"], values=[1000,400,150]")]
#[crate::params(paramsList["title","labels","values","sort_order","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("funnel3d", "funnel_3d", "funnel3d_chart")]
#[crate::sera_builder]
pub fn build_funnel3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::common::apply_sort;
    let srt = o.srt();
    let (labels, values) = apply_sort(&labels, &values, &srt);
    let n = labels.len().min(values.len());
    let xv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let yv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let cv: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_funnel3d_html(
            title,
            &xv,
            &yv,
            &values[..n],
            ("", "Stage", "Value"),
            &cv,
            &labels[..n].to_vec(),
            o.w(700),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
        ),
        &o,
    )
}
