use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::zone::{fit as zone_fit, Fit};
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::bar::Bar3DBlock;
use crate::plot::statistical::wordcloud::layout3d;
use crate::plot::statistical::{WordCloudConfig, WordCloudVariant};
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

fn render_wordcloud_text_html(
    title: &str,
    blocks: &[Bar3DBlock],
    names: &[String],
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg: Option<&str>,
    scene: &str,
    zone: Option<&[f64]>,
) -> String {
    if blocks.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(18, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], w, h, bg, scene, b"var S=[];");
    }
    let x: Vec<f64> = blocks.iter().map(|b| b.cx).collect();
    let y: Vec<f64> = blocks.iter().map(|b| b.cy).collect();
    let z: Vec<f64> = blocks.iter().map(|b| (b.z0 + b.z1) / 2.0).collect();
    let (hlo, hhi) = blocks.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), b| (lo.min(b.hw), hi.max(b.hw)));
    let hrange = (hhi - hlo).max(1e-9);
    let padded: Vec<Bar3DBlock> = blocks
        .iter()
        .map(|b| {
            let pad = b.hw.max(b.hd).max(0.05) * 1.2;
            Bar3DBlock::new(b.cx, b.cy, b.z0 - pad, b.z1 + pad, b.hw + pad, b.hd + pad, b.ci)
        })
        .collect();
    let explicit = zone.and_then(|p| <[f64; 3]>::try_from(p).ok());
    let fitted = zone_fit(&padded, 0.6, explicit, Fit::Uniform);
    let mut extra = format!(
        "var BFIT={};var S=[{}];",
        fitted.to_js(),
        blocks.iter().map(|b| format!("{:.4}", (b.hw - hlo) / hrange)).collect::<Vec<_>>().join(",")
    );
    extra.push_str("var NM=[");
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            extra.push(',');
        }
        extra.push('\'');
        js_str_escape(&mut extra, name);
        extra.push('\'');
    }
    extra.push_str("];");
    let colors: Vec<f64> = blocks.iter().map(|b| b.ci as f64).collect();
    crate::html::js_3d::render_3d_html_impl(18, title, &x, &y, &z, axis_labels, &colors, &[], w, h, bg, scene, extra.as_bytes())
}

#[crate::chart_demo("words=[\"rust\",\"python\",\"wasm\",\"plot\",\"data\",\"viz\",\"chart\",\"graph\",\"fast\",\"native\"], frequencies=[42,38,30,28,25,22,18,15,12,10]")]
#[crate::params(paramsList["title","words","frequencies","points_x","points_y","category_indices","cluster_labels","edges_i","edges_j","edges_w","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("wordcloud3d", "word_cloud3d", "wordcloud3d_chart", "tag_cloud3d", "cloud3d")]
#[crate::sera_builder]
pub fn build_wordcloud3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let words = a.words.unwrap_or_default();
    let frequencies = a.frequencies.unwrap_or_else(|| a.values.unwrap_or_default());
    let points_x = o.points_x.clone().unwrap_or_default();
    let points_y = o.points_y.clone().unwrap_or_default();
    let point_clusters = o.category_indices.clone().unwrap_or_default();
    let edges_i = o.edges_i.clone().unwrap_or_default();
    let edges_j = o.edges_j.clone().unwrap_or_default();
    let edges_w = o.edges_w.clone().unwrap_or_default();
    let variant = WordCloudVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = WordCloudConfig {
        variant,
        title,
        words: &words,
        frequencies: &frequencies,
        points_x: &points_x,
        points_y: &points_y,
        point_clusters: &point_clusters,
        edges_i: &edges_i,
        edges_j: &edges_j,
        edges_w: &edges_w,
        ..WordCloudConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let axis_labels = (o.xl(), o.yl(), o.zl());
    let axis_refs = (axis_labels.0.as_str(), axis_labels.1.as_str(), axis_labels.2.as_str());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = if points_x.len() >= 2 && points_x.len() == points_y.len() {
        let view = BlockView::new(0.6, "jet").with_zone(o.zone.as_deref());
        render_blocks3d_view_html(title, &blocks, &view, axis_refs, &names, o.w(900), o.h(500), bg_default, env)
    } else {
        render_wordcloud_text_html(title, &blocks, &names, axis_refs, o.w(900), o.h(500), bg_default, env, o.zone.as_deref())
    };
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 116,
        name: "wordcloud_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_wordcloud3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::WordCloudVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/wordcloud/", WordCloudVariant::keys_and_aliases(), WordCloudVariant::default_key())
    }

    #[test]
    fn every_wordcloud_variant_has_a_working_3d_counterpart() {
        for (key, json) in &demos() {
            let html = build_wordcloud3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render real words by default, not Bar3DBlock cuboids");
            assert!(html.contains("var NM="), "{key} must carry the actual word text");
            assert!(html.contains("var BFIT="), "{key} must fit its camera to the actual word extent, like every other 3d chart");
        }
    }

    #[test]
    fn many_words_still_fit_within_the_camera_frame() {
        let words: Vec<String> = (0..40).map(|i| format!("word{i}")).collect();
        let frequencies: Vec<f64> = (0..40).map(|i| 10.0 + i as f64).collect();
        let json = serde_json::json!({"words": words, "frequencies": frequencies}).to_string();
        let html = build_wordcloud3d_chart(&json);
        assert!(html.contains("var BFIT="), "a large word count must still emit a fitted camera zone");
    }

    #[test]
    fn the_fitted_zone_is_padded_so_large_words_do_not_touch_its_own_floor() {
        use crate::plot::statistical::_3d::zone::{fit as zone_fit, Fit};
        use crate::plot::statistical::bar::Bar3DBlock;
        let blocks = vec![
            Bar3DBlock::new(0.0, 0.0, 0.0, 0.2, 0.2, 0.2, 0),
            Bar3DBlock::new(1.0, 0.5, 0.0, 0.6, 0.6, 0.6, 1),
            Bar3DBlock::new(-0.8, -0.4, 0.0, 0.4, 0.4, 0.4, 2),
        ];
        let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let html = super::render_wordcloud_text_html("t", &blocks, &names, ("x", "y", "z"), 400, 300, None, "default", None);
        let raw = zone_fit(&blocks, 0.6, None, Fit::Uniform);
        assert!(!html.contains(&format!("\"dz\":{:.6}", raw.data[2])), "the fitted zone must not equal the unpadded raw extent");
    }

    #[test]
    fn supplying_real_positions_and_edges_switches_the_chart_to_the_block_network_layout() {
        let json = r#"{"words":["a","b","c","d"],"frequencies":[10,20,15,25],"points_x":[0,1,2,3],"points_y":[0,1,0,1],"edges_i":[0,1],"edges_j":[1,2],"edges_w":[1,1]}"#;
        let html = build_wordcloud3d_chart(json);
        assert!(!html.is_empty());
        assert!(html.contains("var BN="), "supplying real positions must switch to the block network layout");
    }

    #[test]
    fn every_3d_plane_applies_to_every_wordcloud_variant() {
        twin::check_planes(build_wordcloud3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_wordcloud_variant() {
        for (key, json) in &demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_wordcloud3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_wordcloud_variant() {
        twin::check_themes(build_wordcloud3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_wordcloud_variants_and_the_view_axes() {
        twin::check_axes("wordcloud3d", WordCloudVariant::all().len());
    }

    #[test]
    fn every_wordcloud_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_wordcloud3d_chart, &demos());
    }

    #[test]
    fn every_wordcloud_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_wordcloud3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("wordcloud3d", build_wordcloud3d_chart, &demos(), WordCloudVariant::default_key());
    }
}
