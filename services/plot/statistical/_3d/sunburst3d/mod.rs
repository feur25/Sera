use crate::plot::statistical::_3d::wedge;
use crate::plot::statistical::sunburst::layout3d;
use crate::plot::statistical::{SunburstConfig, SunburstVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Root\",\"Engineering\",\"Sales\",\"Marketing\",\"Frontend\",\"Backend\",\"Enterprise\",\"SMB\",\"Digital\",\"Events\"], parents=[\"\",\"Root\",\"Root\",\"Root\",\"Engineering\",\"Engineering\",\"Sales\",\"Sales\",\"Marketing\",\"Marketing\"], values=[0,35,40,25,15,20,25,15,18,7]")]
#[crate::params(paramsList["title","labels","parents","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("sunburst3d", "sunburst_3d", "sunburst3d_chart", "sunburst3d_family", "sunbursts3d")]
#[crate::sera_builder]
pub fn build_sunburst3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = SunburstVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = SunburstConfig { variant, title, labels: &labels, parents: &parents, values: &values, ..SunburstConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let axis_labels = (o.xl(), o.yl(), o.zl());
    let axis_refs = (axis_labels.0.as_str(), axis_labels.1.as_str(), axis_labels.2.as_str());
    let html = match layout3d::wedges(&cfg) {
        Some((w, zl)) => wedge::render_html(title, &w, axis_refs, o.w(900), o.h(560), bg_default, env, o.zone.as_deref(), &zl.groups, &zl.depth, zl.hole, zl.ring_span),
        None => String::new(),
    };
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 80,
        name: "sunburst_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_sunburst3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::SunburstVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/sunburst/", SunburstVariant::keys_and_aliases(), SunburstVariant::default_key())
    }

    #[test]
    fn every_sunburst_variant_has_a_working_3d_counterpart() {
        for (key, json) in &demos() {
            let html = build_sunburst3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as round wedges, not Bar3DBlock cuboids");
            assert!(html.contains("var A0="), "{key} must carry precomputed wedge angles");
            assert!(html.contains("var BFIT="), "{key} must fit its camera to the real wedge geometry like every other 3d chart");
        }
    }

    #[test]
    fn every_sunburst_variant_honours_an_explicit_zone_and_lives_in_real_3d_space() {
        twin::check_zone(build_sunburst3d_chart, &demos());
    }

    #[test]
    fn every_3d_plane_applies_to_every_sunburst_variant() {
        twin::check_planes(build_sunburst3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_sunburst_variant() {
        for (key, json) in &demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_sunburst3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_sunburst_variant() {
        twin::check_themes(build_sunburst3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_sunburst_variants_and_the_view_axes() {
        twin::check_axes("sunburst3d", SunburstVariant::all().len());
    }

    #[test]
    fn every_sunburst_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_sunburst3d_chart, &demos());
    }

    #[test]
    fn every_sunburst_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_sunburst3d_chart, &demos(), 2000);
    }

    #[test]
    fn node_labels_reach_the_tooltip_names() {
        let html = build_sunburst3d_chart(r#"{"title":"t","labels":["Root","A","B"],"parents":["","Root","Root"],"values":[0,40,60]}"#);
        assert!(html.contains("'Root'") && html.contains("'A'") && html.contains("'B'"));
    }

    #[test]
    fn only_the_zoomable_variant_carries_click_to_zoom_groups() {
        let base = r#"{"title":"t","labels":["Root","A","B"],"parents":["","Root","Root"],"values":[0,40,60],"variant":"#;
        let zoomable = build_sunburst3d_chart(&format!("{base}\"zoomable\"}}"));
        assert!(zoomable.contains("var ZKIDS="));
        assert!(zoomable.contains("var ND="), "zoomable must carry per-node depth for the reflow animation");
        assert!(zoomable.contains("var ZHOLE="));
        assert!(zoomable.contains("var ZRSPAN="));
        let basic = build_sunburst3d_chart(&format!("{base}\"basic\"}}"));
        assert!(!basic.contains("var ZKIDS="));
        assert!(!basic.contains("var ND="));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("sunburst3d", build_sunburst3d_chart, &demos(), SunburstVariant::default_key());
    }
}
