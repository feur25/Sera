use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::parcats::layout3d;
use crate::plot::statistical::{ParcatsConfig, ParcatsVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "axes=[\"Gender\",\"Survived\",\"Class\"], category_series=[[\"Male\",\"No\",\"3rd\"],[\"Female\",\"Yes\",\"1st\"],[\"Male\",\"No\",\"2nd\"],[\"Female\",\"Yes\",\"1st\"],[\"Male\",\"Yes\",\"1st\"],[\"Female\",\"No\",\"3rd\"]]"
)]
#[crate::params(paramsList["title","axes","category_series","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("parcats3d", "parcats_3d", "parcats3d_chart", "parallel_categories3d")]
#[crate::sera_builder]
pub fn build_parcats3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let category_series = a.category_series.unwrap_or_default();
    let variant = ParcatsVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = ParcatsConfig { variant, title, axes: &axes, category_series: &category_series, ..ParcatsConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(crate::plot::statistical::sankey::layout3d::HEIGHT_RATIO, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(900),
        o.h(520),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 114,
        name: "parcats_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_parcats3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::ParcatsVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/parcats/", ParcatsVariant::keys_and_aliases(), ParcatsVariant::default_key())
    }

    #[test]
    fn every_parcats_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_parcats3d_chart, &demos(), ParcatsVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_parcats_variant() {
        twin::check_planes(build_parcats3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_parcats_variant() {
        twin::check_scenes(build_parcats3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_parcats_variant() {
        twin::check_themes(build_parcats3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_parcats_variants_and_the_view_axes() {
        twin::check_axes("parcats3d", ParcatsVariant::all().len());
    }

    #[test]
    fn every_parcats_variant_honours_an_explicit_zone() {
        twin::check_zone(build_parcats3d_chart, &demos());
    }

    #[test]
    fn every_parcats_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_parcats3d_chart, &demos());
    }

    #[test]
    fn every_parcats_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_parcats3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("parcats3d", build_parcats3d_chart, &demos(), ParcatsVariant::default_key());
    }
}
