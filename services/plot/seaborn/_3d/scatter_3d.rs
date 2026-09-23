use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::scatter::layout3d;
use crate::plot::statistical::{ScatterConfig, ScatterVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("x=[1,2,3,4], y=[2,1,4,3], z=[10,20,15,25]")]
#[crate::params(paramsList["title","x","y","z","labels","categories","color_values","variant","color_hex","palette","bg_color","scene","orientation3d","theme","zone","max_points","width","height","x_label","y_label","z_label"])]
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
    if a.z.is_some() {
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
        return apply_bg3d(html, &o);
    }

    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.or(a.values).unwrap_or_default();
    let labels = a.labels.unwrap_or_default();
    let cats_arg = a.categories.unwrap_or_default();
    let categories = if !cats_arg.is_empty() { cats_arg } else { o.color_groups.clone().unwrap_or_default() };
    let color_values = o.color_values.clone().unwrap_or_default();
    let series_names = o.series_names.clone().unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = a
        .series
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .map(|(si, vals)| (series_names.get(si).cloned().unwrap_or_else(|| format!("S{}", si + 1)), vals))
        .collect();
    let variant = ScatterVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = ScatterConfig {
        variant,
        title,
        x_values: &x_values,
        y_values: &y_values,
        labels: &labels,
        categories: &categories,
        color_values: &color_values,
        series: &series,
        ..ScatterConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

#[cfg(test)]
mod tests {
    use super::build_scatter3d_chart;
    use crate::plot::statistical::_3d::budget::Budget;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::ScatterVariant;

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

    fn demos() -> twin::Demos {
        let base = r#"{"x":[1,2,3,4,5,6],"y":[2.1,3.9,6.2,7.8,10.1,12.0]}"#;
        let wide = r#"{"x":[1,2,3,4,5,6],"series":[[2.1,3.9,6.2,7.8,10.1,12.0],[5.0,4.5,4.0,3.2,2.8,2.1]],"series_names":["A","B"]}"#;
        ScatterVariant::keys_and_aliases()
            .iter()
            .map(|(key, _)| {
                let src = if *key == "wide_form" { wide } else { base };
                (*key, twin::set_field(src, "variant", key))
            })
            .collect()
    }

    #[test]
    fn every_scatter_variant_has_a_working_3d_counterpart_without_z() {
        twin::check_variants(build_scatter3d_chart, &demos(), ScatterVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_scatter_variant() {
        twin::check_planes(build_scatter3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_scatter_variant() {
        twin::check_scenes(build_scatter3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_scatter_variant() {
        twin::check_themes(build_scatter3d_chart, &demos());
    }

    #[test]
    fn every_scatter_variant_honours_an_explicit_zone() {
        twin::check_zone(build_scatter3d_chart, &demos());
    }

    #[test]
    fn every_scatter_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_scatter3d_chart, &demos());
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("scatter3d", build_scatter3d_chart, &demos(), ScatterVariant::default_key());
    }
}
