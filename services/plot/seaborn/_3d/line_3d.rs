use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::common::format_axis_label;
use crate::plot::statistical::line::layout3d;
use crate::plot::statistical::{LineConfig, LineVariant};
use crate::plot::{apply_bg3d, parse_all, ChartOpts};

fn legacy_polyline(title: &str, x: &[f64], y: &[f64], z: &[f64], o: &ChartOpts) -> String {
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let html = crate::plot::default::render_line3d_html(
        title,
        x,
        y,
        z,
        (&o.xl(), &o.yl(), &o.zl()),
        &cv,
        &cl,
        o.w(900),
        o.h(560),
        bg_str.as_deref(),
        &o.scene3d(),
    );
    apply_bg3d(html, o)
}

#[crate::chart_demo("x=[0,1,2,3], y=[0,1,2,3], z=[10,20,15,25]")]
#[crate::params(paramsList["title","x","y","z","labels","values","series","series_names","x_labels","variant","step_shape","spline_tension","dash_pattern","gap_threshold","show_points","pace_target","color_hex","palette","bg_color","scene","orientation3d","theme","zone","max_points","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("line3d", "line_3d", "line3d_chart", "line3d_family", "lines3d")]
#[crate::sera_builder]
pub fn build_line3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let z = a.z.clone().unwrap_or_default();
    if !z.is_empty() {
        return legacy_polyline(title, &a.x.clone().unwrap_or_default(), &a.y.clone().unwrap_or_default(), &z, &o);
    }
    let x_labels = a
        .x_labels
        .clone()
        .unwrap_or_else(|| a.x.as_ref().map(|xs| xs.iter().map(|&v| format_axis_label(v)).collect()).unwrap_or_default());
    let labels = a.labels.clone().unwrap_or_else(|| x_labels.clone());
    let values = a.values.clone().unwrap_or_else(|| a.y.clone().unwrap_or_default());
    let names = o.series_names.clone().unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = a
        .series
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, v)| (names.get(i).cloned().unwrap_or_else(|| format!("S{}", i + 1)), v.clone()))
        .collect();
    let step_shape = o.step_shape.clone().or_else(|| o.line_shape.clone()).unwrap_or_else(|| "hv".to_string());
    let dash_pattern = o.dash_pattern.clone().unwrap_or_else(|| "auto".to_string());
    let variant = LineVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = LineConfig {
        variant,
        title,
        labels: &labels,
        values: &values,
        series: &series,
        x_labels: &x_labels,
        step_shape: &step_shape,
        spline_tension: o.spline_tension.unwrap_or(0.5),
        dash_pattern: &dash_pattern,
        gap_threshold: o.gap_threshold.unwrap_or(f64::NAN),
        show_points: o.show_points.unwrap_or(false),
        pace_target: o.pace_target,
        ..LineConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
    let color_labels: Vec<String> = series.iter().map(|(name, _)| name.clone()).collect();
    let html = render_blocks3d_view_html(
        title,
        &layout3d::layout_3d(&cfg, &Budget::new(o.max_points)),
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &color_labels,
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

#[cfg(test)]
mod tests {
    use crate::plot::build_line3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::LineVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/line/", LineVariant::keys_and_aliases(), LineVariant::default_key())
    }

    #[test]
    fn every_line_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_line3d_chart, &demos(), LineVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_line_variant() {
        twin::check_planes(build_line3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_line_variant() {
        twin::check_scenes(build_line3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_line_variant() {
        twin::check_themes(build_line3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_line_variants_and_the_view_axes() {
        twin::check_axes("line_3d", LineVariant::all().len());
    }

    #[test]
    fn every_line_variant_honours_an_explicit_zone() {
        twin::check_zone(build_line3d_chart, &demos());
    }

    #[test]
    fn every_line_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_line3d_chart, &demos());
    }

    #[test]
    fn every_line_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_line3d_chart, &demos(), 2000);
    }

    #[test]
    fn the_legacy_spatial_polyline_call_still_renders() {
        let json = r#"{"title":"t","x":[0,1,2,3],"y":[0,1,2,3],"z":[10,20,15,25]}"#;
        let html = build_line3d_chart(json);
        assert!(html.contains("class=\"c3w\""));
        assert!(!html.contains("var BN="));
    }

    #[test]
    fn a_million_points_reach_the_engine_as_a_bounded_ribbon() {
        let values: Vec<f64> = (0..1_000_000u64).map(|i| ((i as f64) * 0.001).sin() * 50.0 + (i % 7) as f64).collect();
        let json = serde_json::json!({"title": "t", "values": values, "variant": "spline"}).to_string();
        let html = build_line3d_chart(&json);
        assert!(twin::block_count(&html) <= crate::plot::statistical::_3d::budget::HARD_BLOCKS);
        assert!(html.len() < 2 * 1024 * 1024);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("line3d", build_line3d_chart, &demos(), LineVariant::default_key());
    }
}
