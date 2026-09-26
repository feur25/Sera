use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html_zoomable, BlockView};
use crate::plot::statistical::sunburst::layout3d::{self, Wedges};
use crate::plot::statistical::{SunburstConfig, SunburstVariant};
use crate::plot::{apply_bg3d, parse_all};

fn js_str_escape(out: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '\'' => out.push_str("\\'"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
}

fn nums(values: &[f64], places: usize) -> String {
    values.iter().map(|v| format!("{v:.places$}")).collect::<Vec<_>>().join(",")
}

fn render_wedges_html(title: &str, w: &Wedges, axis_labels: (&str, &str, &str), width: i32, height: i32, bg: Option<&str>, scene: &str) -> String {
    if w.pct.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(13, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], width, height, bg, scene, b"");
    }
    let mut extra = format!(
        "var A0=[{}],A1=[{}],RH=[{}];HOLEF={:.4};",
        nums(&w.a0, 5),
        nums(&w.a1, 5),
        nums(&w.ring_height, 4),
        w.hole,
    );
    extra.push_str("var NM=[");
    for (i, name) in w.names.iter().enumerate() {
        if i > 0 {
            extra.push(',');
        }
        extra.push('\'');
        js_str_escape(&mut extra, name);
        extra.push('\'');
    }
    extra.push_str("];");
    crate::html::js_3d::render_3d_html_impl(13, title, &w.pct, &w.depth, &w.value, axis_labels, &w.color_idx, &[], width, height, bg, scene, extra.as_bytes())
}

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\"], parents=[\"\",\"Root\",\"Root\"], values=[0,40,60]")]
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
    let html = if matches!(variant, SunburstVariant::Zoomable) {
        let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
        let (blocks, names, groups) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
        render_blocks3d_view_html_zoomable(title, &blocks, &view, axis_refs, &names, o.w(900), o.h(560), bg_default, env, &groups)
    } else {
        match layout3d::wedges(&cfg) {
            Some(w) => render_wedges_html(title, &w, axis_refs, o.w(900), o.h(560), bg_default, env),
            None => String::new(),
        }
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

    fn is_block_variant(key: &str) -> bool {
        key == "zoomable"
    }

    fn block_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| is_block_variant(key)).collect()
    }

    fn wedge_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| !is_block_variant(key)).collect()
    }

    #[test]
    fn every_sunburst_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_sunburst3d_chart, &block_demos(), 1);
        for (key, json) in &wedge_demos() {
            let html = build_sunburst3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as round wedges, not Bar3DBlock cuboids");
            assert!(html.contains("var A0="), "{key} must carry precomputed wedge angles");
        }
    }

    #[test]
    fn every_3d_plane_applies_to_every_sunburst_variant() {
        twin::check_planes(build_sunburst3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_sunburst_variant() {
        twin::check_scenes(build_sunburst3d_chart, &block_demos());
        for (key, json) in &wedge_demos() {
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
    fn the_zoomable_variant_honours_an_explicit_zone() {
        twin::check_zone(build_sunburst3d_chart, &block_demos());
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
        let basic = build_sunburst3d_chart(&format!("{base}\"basic\"}}"));
        assert!(!basic.contains("var ZKIDS="));
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("sunburst3d", build_sunburst3d_chart, &demos(), SunburstVariant::default_key());
    }
}
