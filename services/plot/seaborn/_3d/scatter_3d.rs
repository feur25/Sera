use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::zone::{fit as zone_fit, Fit};
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::scatter::layout3d;
use crate::plot::statistical::{ScatterConfig, ScatterVariant};
use crate::plot::{apply_bg3d, parse_all};

fn fitted_zone_js(blocks: &[Bar3DBlock], zone: Option<&[f64]>) -> String {
    let explicit = zone.and_then(|p| <[f64; 3]>::try_from(p).ok());
    format!("var BFIT={};", zone_fit(blocks, 0.7, explicit, Fit::Uniform).to_js())
}

fn render_scatter_points_html(
    title: &str,
    blocks: &[Bar3DBlock],
    names: &[String],
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg: Option<&str>,
    scene: &str,
    zone: Option<&[f64]>,
) -> String {
    if blocks.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(0, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], w, h, bg, scene, b"");
    }
    let x: Vec<f64> = blocks.iter().map(|b| b.cx).collect();
    let y: Vec<f64> = blocks.iter().map(|b| b.cy).collect();
    let z: Vec<f64> = blocks.iter().map(|b| (b.z0 + b.z1) / 2.0).collect();
    let colors: Vec<f64> = blocks.iter().map(|b| b.ci as f64).collect();
    let extra = fitted_zone_js(blocks, zone);
    crate::html::js_3d::render_3d_html_impl(0, title, &x, &y, &z, axis_labels, &colors, names, w, h, bg, scene, extra.as_bytes())
}

fn render_scatter_spheres_html(
    title: &str,
    blocks: &[Bar3DBlock],
    names: &[String],
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg: Option<&str>,
    scene: &str,
    zone: Option<&[f64]>,
) -> String {
    if blocks.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(16, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], w, h, bg, scene, b"var S=[];");
    }
    let x: Vec<f64> = blocks.iter().map(|b| b.cx).collect();
    let y: Vec<f64> = blocks.iter().map(|b| b.cy).collect();
    let z: Vec<f64> = blocks.iter().map(|b| (b.z0 + b.z1) / 2.0).collect();
    let (hlo, hhi) = blocks.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), b| (lo.min(b.hw), hi.max(b.hw)));
    let hrange = (hhi - hlo).max(1e-9);
    let mut extra = fitted_zone_js(blocks, zone);
    extra.push_str(&format!(
        "var S=[{}];",
        blocks.iter().map(|b| format!("{:.4}", (b.hw - hlo) / hrange)).collect::<Vec<_>>().join(",")
    ));
    let colors: Vec<f64> = blocks.iter().map(|b| b.ci as f64).collect();
    crate::html::js_3d::render_3d_html_impl(16, title, &x, &y, &z, axis_labels, &colors, names, w, h, bg, scene, extra.as_bytes())
}

#[crate::chart_demo("x=[1.4,1.33,0.78,-0.15,-1.2,-2.03,-2.33,-1.91,-0.82,0.67,2.13,3.07,3.15,2.25,0.56,-1.47,-3.21,-4.11,-3.81,-2.3,0.02,2.51], y=[0.0,0.82,1.53,1.87,1.65,0.84,-0.37,-1.64,-2.55,-2.76,-2.12,-0.73,1.03,2.65,3.6,3.51,2.32,0.31,-1.96,-3.8,-4.6,-4.05], z=[0.0,0.42,0.84,1.26,1.68,2.1,2.52,2.94,3.36,3.78,4.2,4.62,5.04,5.46,5.88,6.3,6.72,7.14,7.56,7.98,8.4,8.82]")]
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
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let axis_labels = (o.xl(), o.yl(), o.zl());
    let html = match variant {
        ScatterVariant::Regression | ScatterVariant::Rug => {
            let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
            render_blocks3d_view_html(title, &blocks, &view, (&axis_labels.0, &axis_labels.1, &axis_labels.2), &names, o.w(900), o.h(560), bg_default, env)
        }
        ScatterVariant::Sized => render_scatter_spheres_html(title, &blocks, &names, (&axis_labels.0, &axis_labels.1, &axis_labels.2), o.w(900), o.h(560), bg_default, env, o.zone.as_deref()),
        _ => render_scatter_points_html(title, &blocks, &names, (&axis_labels.0, &axis_labels.1, &axis_labels.2), o.w(900), o.h(560), bg_default, env, o.zone.as_deref()),
    };
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
        let base = r#"{"x":[1,1.8,2.5,3.1,4,4.6,5.2,6,6.7,7.3,8.1,8.6,9.4,10,10.7,11.3,12,12.8],"y":[2.1,3.4,3.9,5.6,6.2,6.9,7.8,9.1,10.1,9.6,11.4,12.0,13.2,14.5,13.8,15.6,16.3,17.1]}"#;
        let wide = r#"{"x":[1,1.8,2.5,3.1,4,4.6,5.2,6,6.7,7.3,8.1,8.6,9.4,10,10.7,11.3,12,12.8],"series":[[2.1,3.4,3.9,5.6,6.2,6.9,7.8,9.1,10.1,9.6,11.4,12.0,13.2,14.5,13.8,15.6,16.3,17.1],[9.5,9.0,8.4,8.0,7.5,7.1,6.6,6.2,5.7,5.3,4.8,4.4,3.9,3.5,3.0,2.6,2.1,1.7]],"series_names":["A","B"]}"#;
        ScatterVariant::keys_and_aliases()
            .iter()
            .map(|(key, _)| {
                let src = if *key == "wide_form" { wide } else { base };
                (*key, twin::set_field(src, "variant", key))
            })
            .collect()
    }

    fn is_block_variant(key: &str) -> bool {
        matches!(key, "regression" | "rug")
    }

    fn block_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| is_block_variant(key)).collect()
    }

    fn point_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| !is_block_variant(key)).collect()
    }

    #[test]
    fn every_scatter_variant_has_a_working_3d_counterpart_without_z() {
        twin::check_variants(build_scatter3d_chart, &block_demos(), 2);
        for (key, json) in &point_demos() {
            let html = build_scatter3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as round points, not Bar3DBlock cuboids");
        }
    }

    #[test]
    fn every_3d_plane_applies_to_every_scatter_variant() {
        twin::check_planes(build_scatter3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_scatter_variant() {
        twin::check_scenes(build_scatter3d_chart, &block_demos());
        for (key, json) in &point_demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_scatter3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_scatter_variant() {
        twin::check_themes(build_scatter3d_chart, &demos());
    }

    #[test]
    fn every_scatter_variant_honours_an_explicit_zone() {
        twin::check_zone(build_scatter3d_chart, &block_demos());
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
