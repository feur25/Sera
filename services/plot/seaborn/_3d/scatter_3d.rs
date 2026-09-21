use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[1,2,3,4], y=[2,1,4,3], z=[10,20,15,25]")]
#[crate::params(paramsList["title","x","y","z","color_hex","palette","bg_color","scene","orientation3d","max_points","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias(
    "scatter3d",
    "scatter_3d",
    "scatter3d_chart",
    "scatter3d_family",
    "scatters3d"
)]
#[crate::sera_builder]
pub fn build_scatter3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let keep = even_indices(x.len().min(y.len()).min(z.len()), Budget::new(o.max_points).cloud());
    let (x, y, z, cv) = (pick(&x, &keep), pick(&y, &keep), pick(&z, &keep), pick(&cv, &keep));
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_scatter3d_html(
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

#[cfg(test)]
mod tests {
    use super::build_scatter3d_chart;
    use crate::plot::statistical::_3d::budget::Budget;
    use crate::plot::statistical::_3d::twin;

    fn columns(n: usize) -> serde_json::Value {
        serde_json::json!({"title": "t", "x": twin::wave(n, 0), "y": twin::wave(n, 1), "z": twin::wave(n, 2)})
    }

    #[test]
    fn a_big_cloud_is_capped_by_the_budget_and_small_ones_are_untouched() {
        twin::check_capped(build_scatter3d_chart, columns, Budget::cloud);
    }

    #[test]
    fn color_values_follow_the_kept_points() {
        let mut body = columns(100_000);
        body["color_values"] = serde_json::json!(twin::wave(100_000, 3));
        let html = build_scatter3d_chart(&body.to_string());
        let colors = html.split("],C=[").nth(1).and_then(|s| s.split(']').next()).map(|s| s.split(',').count());
        assert_eq!(colors, Some(Budget::default().cloud()));
    }
}
