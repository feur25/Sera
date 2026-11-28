use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::violin::layout3d;
use crate::plot::statistical::{ViolinConfig, ViolinVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("categories=[\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\"], values=[1,2,3,2,4,5,6,5]")]
#[crate::params(paramsList["title","categories","labels","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("violin3d", "violin_3d", "violin3d_chart", "violins3d")]
#[crate::sera_builder]
pub fn build_violin3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let categories = a.categories.or(a.labels).unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = ViolinVariant::from_str(o.variant.as_deref().unwrap_or("box"));
    let cfg = ViolinConfig { variant, title, categories: &categories, values: &values, ..ViolinConfig::default() };
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
        id: 75,
        name: "violin_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_violin3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::ViolinVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/violin/", ViolinVariant::keys_and_aliases(), ViolinVariant::default_key())
    }

    #[test]
    fn every_violin_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_violin3d_chart, &demos(), ViolinVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_violin_variant() {
        twin::check_planes(build_violin3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_violin_variant() {
        twin::check_scenes(build_violin3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_violin_variant() {
        twin::check_themes(build_violin3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_violin_variants_and_the_view_axes() {
        twin::check_axes("violin3d", ViolinVariant::all().len());
    }

    #[test]
    fn every_violin_variant_honours_an_explicit_zone() {
        twin::check_zone(build_violin3d_chart, &demos());
    }

    #[test]
    fn every_violin_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_violin3d_chart, &demos());
    }

    #[test]
    fn every_violin_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_violin3d_chart, &demos(), 2000);
    }

    #[test]
    fn category_labels_reach_the_tooltip_names() {
        let html = build_violin3d_chart(r#"{"title":"t","categories":["Atlas","Atlas","Nova","Nova"],"values":[1,2,3,4]}"#);
        assert!(html.contains("'Atlas'") && html.contains("'Nova'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("violin3d", build_violin3d_chart, &demos(), ViolinVariant::default_key());
    }
}
