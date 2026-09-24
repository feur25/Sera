use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::wordcloud::layout3d;
use crate::plot::statistical::{WordCloudConfig, WordCloudVariant};
use crate::plot::{apply_bg3d, parse_all};

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
    let view = BlockView::new(0.6, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(900),
        o.h(500),
        bg_default,
        env,
    );
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
        twin::check_variants(build_wordcloud3d_chart, &demos(), WordCloudVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_wordcloud_variant() {
        twin::check_planes(build_wordcloud3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_wordcloud_variant() {
        twin::check_scenes(build_wordcloud3d_chart, &demos());
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
    fn every_wordcloud_variant_honours_an_explicit_zone() {
        twin::check_zone(build_wordcloud3d_chart, &demos());
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
