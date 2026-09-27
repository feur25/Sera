use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::wedge;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::common::apply_sort;
use crate::plot::statistical::pie::layout3d;
use crate::plot::statistical::{PieConfig, PieVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5]")]
#[crate::params(paramsList["title","labels","values","secondary_values","secondary_labels","pull","series","subplot_cols","proportional","sort_order","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("pie3d", "pie_3d", "pie3d_chart", "pie3d_family", "pies3d")]
#[crate::sera_builder]
pub fn build_pie3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.clone().unwrap_or_default();
    let values = a.values.clone().unwrap_or_default();
    let srt = o.srt();
    let (labels, values) = apply_sort(&labels, &values, &srt);
    let series = a.series.clone().unwrap_or_default();
    let pull = o.pull.clone().unwrap_or_default();
    let secondary_values = o.secondary_values.clone().unwrap_or_default();
    let secondary_labels = o.secondary_labels.clone().unwrap_or_default();
    let variant = PieVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let cfg = PieConfig {
        variant,
        labels: &labels,
        values: &values,
        pull: &pull,
        series: &series,
        subplot_cols: o.subplot_cols.unwrap_or(0),
        proportional: o.proportional.unwrap_or(false),
        secondary_values: &secondary_values,
        secondary_labels: &secondary_labels,
        ..PieConfig::default()
    };
    let axis_labels = (o.xl(), o.yl(), o.zl());
    let axis_refs = (axis_labels.0.as_str(), axis_labels.1.as_str(), axis_labels.2.as_str());
    let html = if matches!(variant, PieVariant::Waffle) {
        let view = BlockView::new(0.7, "jet").with_zone(o.zone.as_deref());
        let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
        render_blocks3d_view_html(title, &blocks, &view, axis_refs, &names, o.w(700), o.h(560), bg_default, env)
    } else {
        match layout3d::wedges(&cfg) {
            Some(w) => wedge::render_html(title, &w, axis_refs, o.w(700), o.h(560), bg_default, env, o.zone.as_deref(), &[]),
            None => String::new(),
        }
    };
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 74,
        name: "pie_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_pie3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::PieVariant;

    fn demos() -> twin::Demos {
        let base = r#"{"labels":["Apple","Banana","Cherry","Date","Fig"],"values":[40,25,20,10,5],"secondary_values":[55,30,15],"secondary_labels":["X","Y","Z"]}"#;
        let subplots = r#"{"labels":["A","B","C","D"],"series":[[40,25,20,15],[30,30,20,20],[50,20,15,15]]}"#;
        PieVariant::keys_and_aliases()
            .iter()
            .map(|(key, _)| {
                let body = if matches!(*key, "subplots" | "proportional") { subplots } else { base };
                (*key, twin::set_field(body, "variant", key))
            })
            .collect()
    }

    fn is_block_variant(key: &str) -> bool {
        key == "waffle"
    }

    fn block_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| is_block_variant(key)).collect()
    }

    fn wedge_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| !is_block_variant(key)).collect()
    }

    #[test]
    fn every_pie_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_pie3d_chart, &block_demos(), 1);
        for (key, json) in &wedge_demos() {
            let html = build_pie3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as round wedges, not Bar3DBlock cuboids");
            assert!(html.contains("var A0="), "{key} must carry precomputed wedge angles");
            assert!(html.contains("var BFIT="), "{key} must fit its camera to the real wedge geometry like every other 3d chart");
        }
    }

    #[test]
    fn every_pie_variant_honours_an_explicit_zone_and_lives_in_real_3d_space() {
        twin::check_zone(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_3d_plane_applies_to_every_pie_variant() {
        twin::check_planes(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_pie_variant() {
        twin::check_scenes(build_pie3d_chart, &block_demos());
        for (key, json) in &wedge_demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_pie3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_pie_variant() {
        twin::check_themes(build_pie3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_pie_variants_and_the_view_axes() {
        twin::check_axes("pie3d", PieVariant::all().len());
    }

    #[test]
    fn every_pie_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_pie3d_chart, &demos());
    }

    #[test]
    fn every_pie_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_pie3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("pie3d", build_pie3d_chart, &demos(), PieVariant::default_key());
    }
}
