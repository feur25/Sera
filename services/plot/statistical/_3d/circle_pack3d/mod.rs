use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::circle_pack::layout3d;
use crate::plot::statistical::{CirclePackConfig, CirclePackVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\",\"C\",\"A1\",\"A2\",\"B1\"], parents=[\"\",\"Root\",\"Root\",\"Root\",\"A\",\"A\",\"B\"], values=[0,40,30,20,20,20,30]")]
#[crate::params(paramsList["title","labels","parents","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("circle_pack3d", "circle_pack_3d", "circle_pack3d_chart", "circle_pack3d_family", "circlepacks3d")]
#[crate::sera_builder]
pub fn build_circle_pack3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = CirclePackVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = CirclePackConfig {
        variant,
        title,
        labels: &labels,
        parents: &parents,
        values: &values,
        ..CirclePackConfig::default()
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

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 97,
        name: "circle_pack_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_circle_pack3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::CirclePackVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/circle_pack/", CirclePackVariant::keys_and_aliases(), CirclePackVariant::default_key())
    }

    #[test]
    fn every_circle_pack_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_circle_pack3d_chart, &demos(), CirclePackVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_circle_pack_variant() {
        twin::check_planes(build_circle_pack3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_circle_pack_variant() {
        twin::check_scenes(build_circle_pack3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_circle_pack_variant() {
        twin::check_themes(build_circle_pack3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_circle_pack_variants_and_the_view_axes() {
        twin::check_axes("circle_pack3d", CirclePackVariant::all().len());
    }

    #[test]
    fn every_circle_pack_variant_honours_an_explicit_zone() {
        twin::check_zone(build_circle_pack3d_chart, &demos());
    }

    #[test]
    fn every_circle_pack_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_circle_pack3d_chart, &demos());
    }

    #[test]
    fn every_circle_pack_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_circle_pack3d_chart, &demos(), 2000);
    }

    #[test]
    fn node_labels_reach_the_tooltip_names() {
        let html = build_circle_pack3d_chart(r#"{"title":"t","labels":["Root","A"],"parents":["","Root"],"values":[0,40]}"#);
        assert!(html.contains("'Root'") && html.contains("'A'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("circle_pack3d", build_circle_pack3d_chart, &demos(), CirclePackVariant::default_key());
    }
}
