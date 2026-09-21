use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::funnel::layout3d;
use crate::plot::statistical::{FunnelConfig, FunnelVariant};
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

#[crate::chart_demo("labels=[\"Visit\",\"Signup\",\"Purchase\"], values=[1000,400,150]")]
#[crate::params(paramsList["title","labels","values","series","series_names","category_series","variant","sort_order","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("funnel3d", "funnel_3d", "funnel3d_chart", "funnel3d_family", "funnels3d")]
#[crate::sera_builder]
pub fn build_funnel3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.clone().unwrap_or_default();
    let values = a.values.clone().unwrap_or_default();
    let names = o.series_names.clone().unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = a
        .series
        .clone()
        .unwrap_or_default()
        .into_iter()
        .enumerate()
        .map(|(i, column)| (names.get(i).cloned().unwrap_or_else(|| format!("S{}", i + 1)), column))
        .collect();
    let stage_labels = a.category_series.clone().unwrap_or_default();
    let sort_order = o.srt();
    let cfg = FunnelConfig {
        variant: FunnelVariant::from_str(o.variant.as_deref().unwrap_or("basic")),
        title,
        labels: &labels,
        values: &values,
        series: &series,
        stage_labels: &stage_labels,
        sort_order: &sort_order,
        ..FunnelConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::colormap(&cfg)).with_zone(o.zone.as_deref());
    let (blocks, block_names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &block_names,
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 79,
        name: "funnel_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
#[cfg(test)]
mod tests {
    use super::build_funnel3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::FunnelVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/funnel/", FunnelVariant::keys_and_aliases(), FunnelVariant::default_key())
    }

    #[test]
    fn every_funnel_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_funnel3d_chart, &demos(), FunnelVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_funnel_variant() {
        twin::check_planes(build_funnel3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_funnel_variant() {
        twin::check_scenes(build_funnel3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_funnel_variant() {
        twin::check_themes(build_funnel3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_funnel_variants_and_the_view_axes() {
        twin::check_axes("funnel3d", FunnelVariant::all().len());
    }

    #[test]
    fn every_funnel_variant_honours_an_explicit_zone() {
        twin::check_zone(build_funnel3d_chart, &demos());
    }

    #[test]
    fn every_funnel_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_funnel3d_chart, &demos());
    }

    #[test]
    fn every_funnel_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_funnel3d_chart, &demos(), 2000);
    }

    #[test]
    fn the_original_labels_values_call_renders_blocks_named_after_each_stage() {
        let html = build_funnel3d_chart(r#"{"title":"t","labels":["Visit","Signup"],"values":[1000,400]}"#);
        assert!(html.contains("var BN="));
        assert!(html.contains("Signup · 400 (40% of first)"));
    }

    #[test]
    fn several_series_reach_the_grouped_and_side_by_side_layouts() {
        let grouped = build_funnel3d_chart(r#"{"title":"t","labels":["a","b"],"series":[[5,3],[4,2]],"series_names":["X","Y"]}"#);
        assert!(grouped.contains("a · X · 5") && grouped.contains("b · Y · 2"));
        let compare = build_funnel3d_chart(r#"{"title":"t","series":[[5,3],[4,2,1]],"series_names":["X","Y"],"category_series":[["p","q"],["p","q","r"]]}"#);
        assert!(compare.contains("Y · r · 1"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("funnel3d", build_funnel3d_chart, &demos(), FunnelVariant::default_key());
    }
}
