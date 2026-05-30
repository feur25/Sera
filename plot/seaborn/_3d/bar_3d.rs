use crate::plot::{parse_all, apply_bg3d};

#[crate::sera_alias("bar3d", "bar_3d", "bar3d_chart", "bar3d_family", "bars3d")]
#[crate::sera_builder]
pub fn build_bar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_bar3d_html(
        title, &x, &y, &z, (&o.xl(), &o.yl(), &o.zl()), &cv, &cl,
        o.w(900), o.h(560), bg_str.as_deref(),
    );
    apply_bg3d(html, &o)
}
