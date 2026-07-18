use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_pie3d_html(
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
        7,
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

#[crate::chart_demo("labels=["A","B","C"], values=[30,50,20]")]
#[crate::params(paramsList["title","labels","values","sort_order","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("pie3d", "pie_3d", "pie3d_chart", "pie3d_family", "pies3d")]
#[crate::sera_builder]
pub fn build_pie3d_chart(input: &str) -> String {
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
    let html = crate::plot::statistical::_3d::render_pie3d_html(
        title,
        &xv,
        &yv,
        &values[..n],
        ("", "", ""),
        &cv,
        &labels[..n].to_vec(),
        o.w(700),
        o.h(560),
        bg_str.as_deref(),
        &o.scene3d(),
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 74,
        name: "pie_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
