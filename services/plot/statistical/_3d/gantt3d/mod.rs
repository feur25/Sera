use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::gantt::layout3d;
use crate::plot::statistical::{GanttConfig, GanttVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Design\",\"Build\",\"Test\",\"Launch\"], start=[0,5,12,18], end=[6,14,19,22], categories=[\"Plan\",\"Dev\",\"Dev\",\"Plan\"]")]
#[crate::params(paramsList["title","labels","start","end","categories","color_values","variant","sort_order","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("gantt3d", "gantt_3d", "gantt3d_chart", "gantt3d_family", "timeline3d")]
#[crate::sera_builder]
pub fn build_gantt3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    let categories = a.categories.clone().unwrap_or_default();
    let progress = o.color_values.clone().unwrap_or_default();
    let sort_order = o.srt();
    let cfg = GanttConfig {
        variant: GanttVariant::from_str(o.variant.as_deref().unwrap_or("basic")),
        title,
        labels: &labels,
        values_start: &values_start,
        values_end: &values_end,
        categories: &categories,
        progress: &progress,
        sort_order: &sort_order,
        ..GanttConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP)
        .with_zone(o.zone.as_deref())
        .with_fit(layout3d::FIT);
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
        id: 93,
        name: "gantt_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_gantt3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::GanttVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/gantt/", GanttVariant::keys_and_aliases(), GanttVariant::default_key())
    }

    #[test]
    fn every_gantt_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_gantt3d_chart, &demos(), GanttVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_gantt_variant() {
        twin::check_planes(build_gantt3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_gantt_variant() {
        twin::check_scenes(build_gantt3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_gantt_variant() {
        twin::check_themes(build_gantt3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_gantt_variants_and_the_view_axes() {
        twin::check_axes("gantt3d", GanttVariant::all().len());
    }

    #[test]
    fn every_gantt_variant_honours_an_explicit_zone() {
        twin::check_zone(build_gantt3d_chart, &demos());
    }

    #[test]
    fn every_gantt_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_gantt3d_chart, &demos());
    }

    #[test]
    fn every_gantt_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_gantt3d_chart, &demos(), 2000);
    }

    #[test]
    fn hover_names_reach_the_engine() {
        let html = build_gantt3d_chart(r#"{"title":"t","labels":["Design"],"start":[0],"end":[6],"categories":["Plan"]}"#);
        assert!(html.contains("Design · Plan · 0 → 6"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("gantt3d", build_gantt3d_chart, &demos(), GanttVariant::default_key());
    }
}
