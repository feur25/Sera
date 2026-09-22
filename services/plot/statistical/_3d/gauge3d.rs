use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::gauge::layout3d;
use crate::plot::statistical::{GaugeConfig, GaugeVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("value=72, min_val=0, max_val=100, label=\"Score\"")]
#[crate::params(paramsList["title","value","min_val","max_val","label","comparison","history","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("gauge3d", "gauge_3d", "gauge3d_chart", "gauge3d_family", "speedometer3d")]
#[crate::sera_builder]
pub fn build_gauge3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let value = a.value.unwrap_or(0.0);
    let label = o.label.clone().unwrap_or_default();
    let comparison = o.comparison.unwrap_or(0.0);
    let history = o.history.clone().unwrap_or_default();
    let cfg = GaugeConfig {
        variant: GaugeVariant::from_str(o.variant.as_deref().unwrap_or("basic")),
        title,
        value,
        min_val: o.min_val.unwrap_or(0.0),
        max_val: o.max_val.unwrap_or(100.0),
        label: &label,
        comparison,
        history: &history,
        ..GaugeConfig::default()
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
        id: 94,
        name: "gauge_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_gauge3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::GaugeVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/gauge/", GaugeVariant::keys_and_aliases(), GaugeVariant::default_key())
    }

    #[test]
    fn every_gauge_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_gauge3d_chart, &demos(), GaugeVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_gauge_variant() {
        twin::check_planes(build_gauge3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_gauge_variant() {
        twin::check_scenes(build_gauge3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_gauge_variant() {
        twin::check_themes(build_gauge3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_gauge_variants_and_the_view_axes() {
        twin::check_axes("gauge3d", GaugeVariant::all().len());
    }

    #[test]
    fn every_gauge_variant_honours_an_explicit_zone() {
        twin::check_zone(build_gauge3d_chart, &demos());
    }

    #[test]
    fn every_gauge_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_gauge3d_chart, &demos());
    }

    #[test]
    fn every_gauge_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_gauge3d_chart, &demos(), 2000);
    }

    #[test]
    fn the_label_and_value_reach_the_tooltip_name() {
        let html = build_gauge3d_chart(r#"{"title":"t","value":72,"min_val":0,"max_val":100,"label":"Score"}"#);
        assert!(html.contains("Score · 72 (0-100)"));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("gauge3d", build_gauge3d_chart, &demos(), GaugeVariant::default_key());
    }
}
