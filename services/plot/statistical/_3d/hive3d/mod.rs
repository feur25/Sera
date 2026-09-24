use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::hive::layout3d;
use crate::plot::statistical::{HiveConfig, HiveVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("axes=[\"Biology\",\"Chemistry\",\"Physics\"], labels=[\"n1\",\"n2\",\"n3\",\"n4\",\"n5\",\"n6\"], categories=[\"Biology\",\"Biology\",\"Chemistry\",\"Chemistry\",\"Physics\",\"Physics\"], values=[0.3,0.7,0.2,0.9,0.5,0.8], edges_i=[0,1,2,4], edges_j=[2,3,4,5], edges_w=[1,2,1.5,0.8]")]
#[crate::params(paramsList["title","axes","labels","categories","values","edges_i","edges_j","edges_w","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("hive3d", "hive_3d", "hive3d_chart", "hive3d_family", "radial_network3d")]
#[crate::sera_builder]
pub fn build_hive3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let labels = a.labels.unwrap_or_default();
    let categories = a.categories.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let sources = o.edges_i.clone().unwrap_or_default();
    let targets = o.edges_j.clone().unwrap_or_default();
    let weights = o.edges_w.clone().unwrap_or_else(|| vec![1.0; sources.len()]);
    let variant = HiveVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = HiveConfig {
        variant,
        title,
        axes: &axes,
        labels: &labels,
        categories: &categories,
        values: &values,
        sources: &sources,
        targets: &targets,
        weights: &weights,
        ..HiveConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.7, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(620),
        o.h(580),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 112,
        name: "hive_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_hive3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::HiveVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/hive/", HiveVariant::keys_and_aliases(), HiveVariant::default_key())
    }

    #[test]
    fn every_hive_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_hive3d_chart, &demos(), HiveVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_hive_variant() {
        twin::check_planes(build_hive3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_hive_variant() {
        twin::check_scenes(build_hive3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_hive_variant() {
        twin::check_themes(build_hive3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_hive_variants_and_the_view_axes() {
        twin::check_axes("hive3d", HiveVariant::all().len());
    }

    #[test]
    fn every_hive_variant_honours_an_explicit_zone() {
        twin::check_zone(build_hive3d_chart, &demos());
    }

    #[test]
    fn every_hive_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_hive3d_chart, &demos());
    }

    #[test]
    fn every_hive_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_hive3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("hive3d", build_hive3d_chart, &demos(), HiveVariant::default_key());
    }
}
