use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::parallel::layout3d;
use crate::plot::statistical::{ParallelConfig, ParallelVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("axes=[\"Speed\",\"Power\",\"Range\",\"Cost\"], series=[[80,65,70,40],[60,80,55,60],[40,70,90,75]], series_names=[\"A\",\"B\",\"C\"]")]
#[crate::params(paramsList["title","axes","series","series_names","category_indices","highlight_index","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("parallel3d", "parallel_3d", "parallel3d_chart", "parallel_coordinates3d", "parcoords3d")]
#[crate::sera_builder]
pub fn build_parallel3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
    let names_raw = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if names_raw.is_empty() && !series_values.is_empty() {
        (0..series_values.len()).map(|i| format!("Series {}", i + 1)).collect()
    } else {
        names_raw
    };
    let categories = o.category_indices.clone().unwrap_or_default();
    let variant = ParallelVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = ParallelConfig {
        variant,
        title,
        axes: &axes,
        series_names: &names,
        series_values: &series_values,
        categories: &categories,
        highlight_index: o.highlight_index.unwrap_or(-1),
        ..ParallelConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.6, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(1000),
        o.h(500),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 113,
        name: "parallel_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_parallel3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::ParallelVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/parallel/", ParallelVariant::keys_and_aliases(), ParallelVariant::default_key())
    }

    #[test]
    fn every_parallel_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_parallel3d_chart, &demos(), ParallelVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_parallel_variant() {
        twin::check_planes(build_parallel3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_parallel_variant() {
        twin::check_scenes(build_parallel3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_parallel_variant() {
        twin::check_themes(build_parallel3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_parallel_variants_and_the_view_axes() {
        twin::check_axes("parallel3d", ParallelVariant::all().len());
    }

    #[test]
    fn every_parallel_variant_honours_an_explicit_zone() {
        twin::check_zone(build_parallel3d_chart, &demos());
    }

    #[test]
    fn every_parallel_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_parallel3d_chart, &demos());
    }

    #[test]
    fn every_parallel_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_parallel3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("parallel3d", build_parallel3d_chart, &demos(), ParallelVariant::default_key());
    }
}
