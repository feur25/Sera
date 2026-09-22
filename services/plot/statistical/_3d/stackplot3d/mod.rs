use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::stackplot::layout3d;
use crate::plot::statistical::{StackplotConfig, StackplotVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo(
    "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\",\"Jul\",\"Aug\"], series=[[10,14,12,18,20,16,13,17],[8,9,11,10,13,12,9,10],[5,6,7,9,8,7,6,8]], series_names=[\"A\",\"B\",\"C\"]"
)]
#[crate::params(paramsList["title","x_labels","series","series_names","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("stackplot3d", "stackplot_3d", "stackplot3d_chart", "stackplot3d_family", "stackplots3d")]
#[crate::sera_builder]
pub fn build_stackplot3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let sn = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() { (0..series_flat.len()).map(|_| String::new()).collect() } else { sn };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat).collect();
    let variant = StackplotVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = StackplotConfig { variant, title, x_labels: &x_labels, series: &series, ..StackplotConfig::default() };
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
        id: 100,
        name: "stackplot_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_stackplot3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::StackplotVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/stackplot/", StackplotVariant::keys_and_aliases(), StackplotVariant::default_key())
    }

    #[test]
    fn every_stackplot_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_stackplot3d_chart, &demos(), StackplotVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_stackplot_variant() {
        twin::check_planes(build_stackplot3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_stackplot_variant() {
        twin::check_scenes(build_stackplot3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_stackplot_variant() {
        twin::check_themes(build_stackplot3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_stackplot_variants_and_the_view_axes() {
        twin::check_axes("stackplot3d", StackplotVariant::all().len());
    }

    #[test]
    fn every_stackplot_variant_honours_an_explicit_zone() {
        twin::check_zone(build_stackplot3d_chart, &demos());
    }

    #[test]
    fn every_stackplot_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_stackplot3d_chart, &demos());
    }

    #[test]
    fn every_stackplot_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_stackplot3d_chart, &demos(), 2000);
    }

    #[test]
    fn series_labels_reach_the_tooltip_names() {
        let html = build_stackplot3d_chart(r#"{"title":"t","x_labels":["a","b"],"series":[[1,2]],"series_names":["North"]}"#);
        assert!(html.contains("'North'"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("stackplot3d", build_stackplot3d_chart, &demos(), StackplotVariant::default_key());
    }
}
