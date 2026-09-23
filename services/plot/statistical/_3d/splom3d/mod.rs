use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::splom::layout3d;
use crate::plot::statistical::{SplomConfig, SplomVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("axes=[\"Speed\",\"Power\",\"Range\"], series=[[80,65,70],[60,80,55],[40,70,90],[90,40,60],[55,85,45],[70,55,80]]")]
#[crate::params(paramsList["title","axes","series","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("splom3d", "splom_3d", "splom3d_chart", "splom3d_family", "sploms3d")]
#[crate::sera_builder]
pub fn build_splom3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
    let variant = SplomVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = SplomConfig { variant, title, axes: &axes, series_values: &series_values, ..SplomConfig::default() };
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
        id: 103,
        name: "splom_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_splom3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::SplomVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/splom/", SplomVariant::keys_and_aliases(), SplomVariant::default_key())
    }

    #[test]
    fn every_splom_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_splom3d_chart, &demos(), SplomVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_splom_variant() {
        twin::check_planes(build_splom3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_splom_variant() {
        twin::check_scenes(build_splom3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_splom_variant() {
        twin::check_themes(build_splom3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_splom_variants_and_the_view_axes() {
        twin::check_axes("splom3d", SplomVariant::all().len());
    }

    #[test]
    fn every_splom_variant_honours_an_explicit_zone() {
        twin::check_zone(build_splom3d_chart, &demos());
    }

    #[test]
    fn every_splom_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_splom3d_chart, &demos());
    }

    #[test]
    fn every_splom_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_splom3d_chart, &demos(), 2000);
    }

    #[test]
    fn axis_pair_labels_reach_the_tooltip_names() {
        let html = build_splom3d_chart(r#"{"title":"t","axes":["X","Y"],"series":[[1,2],[3,4],[5,6]]}"#);
        assert!(html.contains("X vs Y") || html.contains("Y vs X"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("splom3d", build_splom3d_chart, &demos(), SplomVariant::default_key());
    }
}
