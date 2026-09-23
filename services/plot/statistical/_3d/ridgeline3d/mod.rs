use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::ridgeline::layout3d;
use crate::plot::statistical::{RidgelineConfig, RidgelineVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("categories=[\"A\",\"A\",\"A\",\"B\",\"B\",\"B\"], values=[1,2,3,4,5,6]")]
#[crate::params(paramsList["title","categories","labels","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("ridgeline3d", "ridgeline_3d", "ridgeline3d_chart", "joy_plot3d")]
#[crate::sera_builder]
pub fn build_ridgeline3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let categories = a.categories.or(a.labels).unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = RidgelineVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = RidgelineConfig { variant, title, categories: &categories, values: &values, ..RidgelineConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::colormap(variant)).with_zone(o.zone.as_deref());
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

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 73,
        name: "ridgeline_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_ridgeline3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::RidgelineVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/ridgeline/", RidgelineVariant::keys_and_aliases(), RidgelineVariant::default_key())
    }

    #[test]
    fn every_ridgeline_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_ridgeline3d_chart, &demos(), RidgelineVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_ridgeline_variant() {
        twin::check_planes(build_ridgeline3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_ridgeline_variant() {
        twin::check_scenes(build_ridgeline3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_ridgeline_variant() {
        twin::check_themes(build_ridgeline3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_ridgeline_variants_and_the_view_axes() {
        twin::check_axes("ridgeline3d", RidgelineVariant::all().len());
    }

    #[test]
    fn every_ridgeline_variant_honours_an_explicit_zone() {
        twin::check_zone(build_ridgeline3d_chart, &demos());
    }

    #[test]
    fn every_ridgeline_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_ridgeline3d_chart, &demos());
    }

    #[test]
    fn every_ridgeline_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_ridgeline3d_chart, &demos(), 2000);
    }

    #[test]
    fn category_labels_reach_the_tooltip_names() {
        let html = build_ridgeline3d_chart(r#"{"title":"t","categories":["Atlas","Atlas","Nova","Nova"],"values":[1,2,3,4]}"#);
        assert!(html.contains("'Atlas'") && html.contains("'Nova'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("ridgeline3d", build_ridgeline3d_chart, &demos(), RidgelineVariant::default_key());
    }
}
