use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::boxplot::layout3d;
use crate::plot::statistical::{BoxplotConfig, BoxplotVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]")]
#[crate::params(paramsList["title","labels","values","series","variant","notch","jitter","boxen_depth","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("boxplot3d", "boxplot_3d", "boxplot3d_chart", "box3d")]
#[crate::sera_builder]
pub fn build_boxplot3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let category_labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let series = a.series.unwrap_or_default();
    let variant = BoxplotVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = BoxplotConfig {
        title,
        variant,
        category_labels: &category_labels,
        values: &values,
        series: &series,
        notch: o.notch.unwrap_or(false),
        jitter: o.jitter.unwrap_or(0.35),
        boxen_depth: o.boxen_depth.unwrap_or(4),
        ..BoxplotConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
    let html = render_blocks3d_view_html(
        title,
        &layout3d::layout_3d(&cfg, &Budget::new(o.max_points)),
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &category_labels,
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
        id: 89,
        name: "boxplot_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use crate::plot::build_boxplot3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::BoxplotVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/boxplot/", BoxplotVariant::keys_and_aliases(), BoxplotVariant::default_key())
    }

    #[test]
    fn every_boxplot_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_boxplot3d_chart, &demos(), BoxplotVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_boxplot_variant() {
        twin::check_planes(build_boxplot3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_boxplot_variant() {
        twin::check_scenes(build_boxplot3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_boxplot_variant() {
        twin::check_themes(build_boxplot3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_boxplot_variants_and_the_view_axes() {
        twin::check_axes("boxplot3d", BoxplotVariant::all().len());
    }

    #[test]
    fn flat_values_with_repeated_labels_render_like_the_2d_chart() {
        let json = r#"{"title":"t","labels":["A","A","A","B","B","B"],"values":[1,2,3,4,5,6]}"#;
        twin::assert_blocks(&build_boxplot3d_chart(json), "flat boxplot3d call");
    }

    #[test]
    fn every_boxplot_variant_honours_an_explicit_zone() {
        twin::check_zone(build_boxplot3d_chart, &demos());
    }

    #[test]
    fn every_boxplot_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_boxplot3d_chart, &demos());
    }

    #[test]
    fn every_boxplot_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_boxplot3d_chart, &demos(), 2000);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("boxplot3d", build_boxplot3d_chart, &demos(), BoxplotVariant::default_key());
    }
}
