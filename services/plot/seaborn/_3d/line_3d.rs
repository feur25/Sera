use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[0,1,2,3], y=[0,1,2,3], z=[10,20,15,25]")]
#[crate::params(paramsList["title","x","y","z","color_hex","palette","bg_color","scene","orientation3d","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("line3d", "line_3d", "line3d_chart", "line3d_family", "lines3d")]
#[crate::sera_builder]
pub fn build_line3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_line3d_html(
        title,
        &x,
        &y,
        &z,
        (&o.xl(), &o.yl(), &o.zl()),
        &cv,
        &cl,
        o.w(900),
        o.h(560),
        bg_str.as_deref(),
        &o.scene3d(),
    );
    apply_bg3d(html, &o)
}
