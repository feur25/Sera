use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_lollipop3d_html(
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
        4,
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

#[crate::chart_demo("x=[1,2,3], y=[1,2,3], z=[4,5,6]")]
#[crate::params(paramsList["title","x","y","z","color_labels","x_label","y_label","z_label","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("lollipop3d", "lollipop_3d", "lollipop3d_chart")]
#[crate::sera_builder]
pub fn build_lollipop3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_lollipop3d_html(
            title,
            &x,
            &y,
            &z,
            (&o.xl(), &o.yl(), &o.zl()),
            &[],
            &cl,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
        ),
        &o,
    )
}
