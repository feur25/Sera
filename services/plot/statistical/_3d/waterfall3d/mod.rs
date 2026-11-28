use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html_tipped, BlockView};
use crate::plot::statistical::waterfall::layout3d;
use crate::plot::statistical::{WaterfallConfig, WaterfallVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"End\"], values=[100,30,-15,40,155]")]
#[crate::params(paramsList["title","labels","values","variant","sort_order","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("waterfall3d", "waterfall_3d", "waterfall3d_chart")]
#[crate::sera_builder]
pub fn build_waterfall3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let sort_order = o.srt();
    let variant = WaterfallVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = WaterfallConfig {
        title,
        variant,
        labels: &labels,
        values: &values,
        sort_order: &sort_order,
        ..WaterfallConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP)
        .with_zone(o.zone.as_deref())
        .with_fit(layout3d::fit_3d(variant));
    let (blocks, names, tip_count, trend) = layout3d::layout_named_tips(&cfg, &Budget::new(o.max_points));
    let block_opacity = match variant {
        WaterfallVariant::Trend => Some(0.55),
        _ => None,
    };
    let html = render_blocks3d_view_html_tipped(
        title,
        &blocks,
        tip_count,
        &trend,
        block_opacity,
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
        id: 90,
        name: "waterfall_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use crate::plot::build_waterfall3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::WaterfallVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/waterfall/", WaterfallVariant::keys_and_aliases(), WaterfallVariant::default_key())
    }

    #[test]
    fn every_waterfall_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_waterfall3d_chart, &demos(), WaterfallVariant::all().len());
    }

    #[test]
    fn only_arrowed_marks_its_trailing_blocks_as_real_pyramid_tips() {
        let base = r#"{"title":"t","labels":["Start","Q1","Q2","End"],"values":[100,30,-15,115],"variant":"#;
        let arrowed = build_waterfall3d_chart(&format!("{base}\"arrowed\"}}"));
        assert!(arrowed.contains("var ARTIP="), "arrowed must mark its tip blocks for pyramid rendering, or it reads as a plain bar with a box on top, not a real arrow");
        let basic = build_waterfall3d_chart(&format!("{base}\"basic\"}}"));
        assert!(!basic.contains("var ARTIP="));
    }

    #[test]
    fn only_trend_overlays_a_dimmed_running_total_line() {
        let base = r#"{"title":"t","labels":["Start","Q1","Q2","End"],"values":[100,30,-15,115],"variant":"#;
        let trend = build_waterfall3d_chart(&format!("{base}\"trend\"}}"));
        assert!(trend.contains("var TRKZ="), "trend must carry the running-total points for the line overlay");
        assert!(trend.contains("var BOP="), "trend must dim its bars so the line overlay reads clearly, like the 2D chart");
        let basic = build_waterfall3d_chart(&format!("{base}\"basic\"}}"));
        assert!(!basic.contains("var TRKZ="));
        assert!(!basic.contains("var BOP="));
    }

    #[test]
    fn every_3d_plane_applies_to_every_waterfall_variant() {
        twin::check_planes(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_waterfall_variant() {
        twin::check_scenes(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_waterfall_variant() {
        twin::check_themes(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_waterfall_variants_and_the_view_axes() {
        twin::check_axes("waterfall3d", WaterfallVariant::all().len());
    }

    #[test]
    fn every_waterfall_variant_honours_an_explicit_zone() {
        twin::check_zone(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn every_waterfall_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn every_waterfall_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_waterfall3d_chart, &demos(), 2000);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("waterfall3d", build_waterfall3d_chart, &demos(), WaterfallVariant::default_key());
    }
}
