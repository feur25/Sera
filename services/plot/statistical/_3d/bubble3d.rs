use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[1,2,3], y=[1,2,3], z=[1,2,3], sizes=[10,20,30]")]
#[crate::params(paramsList["title","x","y","z","sizes","palette","bg_color","scene","orientation3d","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias(
    "bubble3d",
    "bubble_3d",
    "bubble3d_chart",
    "bubble3d_family",
    "bubbles3d"
)]
#[crate::sera_builder]
pub fn build_bubble3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let size_values = a.size.or(a.sizes).unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let n = x.len().min(y.len()).min(z.len()).min(size_values.len());
    let smn = size_values[..n]
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let smx = size_values[..n]
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let sr = (smx - smn).max(1e-9);
    let size_js = format!(
        "var S=[{}];",
        size_values[..n]
            .iter()
            .map(|&s| format!("{:.3}", (s - smn) / sr))
            .collect::<Vec<_>>()
            .join(",")
    );
    let bg_str = o.bg_str();
    let html = crate::html::js_3d::render_3d_html_impl(
        16,
        title,
        &x[..n],
        &y[..n],
        &z[..n],
        (&o.xl(), &o.yl(), &o.zl()),
        &cv,
        &cl,
        o.w(900),
        o.h(560),
        bg_str.as_deref(),
        &o.scene3d(),
        size_js.as_bytes(),
    );
    apply_bg3d(html, &o)
}
