use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::icicle::layout3d;
use crate::plot::statistical::{IcicleConfig, IcicleVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "labels=[\"Company\",\"Engineering\",\"Sales\",\"Backend\",\"Frontend\"], parents=[\"\",\"Company\",\"Company\",\"Engineering\",\"Engineering\"], values=[0,40,60,25,15]"
)]
#[crate::params(paramsList["title","labels","parents","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("icicle3d", "icicle_3d", "icicle3d_chart", "icicle3d_family", "icicles3d")]
#[crate::sera_builder]
pub fn build_icicle3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = IcicleVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = IcicleConfig {
        variant,
        title,
        labels: &labels,
        parents: &parents,
        values: &values,
        ..IcicleConfig::default()
    };
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
        id: 95,
        name: "icicle_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_icicle3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::IcicleVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/icicle/", IcicleVariant::keys_and_aliases(), IcicleVariant::default_key())
    }

    #[test]
    fn every_icicle_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_icicle3d_chart, &demos(), IcicleVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_icicle_variant() {
        twin::check_planes(build_icicle3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_icicle_variant() {
        twin::check_scenes(build_icicle3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_icicle_variant() {
        twin::check_themes(build_icicle3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_icicle_variants_and_the_view_axes() {
        twin::check_axes("icicle3d", IcicleVariant::all().len());
    }

    #[test]
    fn every_icicle_variant_honours_an_explicit_zone() {
        twin::check_zone(build_icicle3d_chart, &demos());
    }

    #[test]
    fn every_icicle_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_icicle3d_chart, &demos());
    }

    #[test]
    fn every_icicle_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_icicle3d_chart, &demos(), 2000);
    }

    #[test]
    fn node_labels_reach_the_tooltip_names() {
        let html = build_icicle3d_chart(r#"{"title":"t","labels":["Company","Engineering"],"parents":["","Company"],"values":[0,40]}"#);
        assert!(html.contains("'Company'") && html.contains("'Engineering'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("icicle3d", build_icicle3d_chart, &demos(), IcicleVariant::default_key());
    }
}
