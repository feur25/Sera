use crate::plot::{apply_bg3d, parse_all};
pub mod basic;
pub mod bubble;
pub mod common;
pub mod config;
pub mod context;
pub mod cosmos;
pub mod image;
pub mod labelmap;
pub mod network;
pub mod neuron;
pub mod shape;
pub mod variant;

pub use config::WordCloudConfig;
pub use shape::WordCloudShape;
pub use variant::WordCloudVariant;

pub fn render_wordcloud_html(cfg: &WordCloudConfig) -> String {
    use variant::WordCloudVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Image => image::render(cfg),
        LabelMap => labelmap::render(cfg),
        Network => network::render(cfg),
        Bubble => bubble::render(cfg),
        Context => context::render(cfg),
        Neuron => neuron::render(cfg),
        Cosmos => cosmos::render(cfg),
    }
}

pub use build as build_wordcloud;

#[crate::sera_alias(
    "wordcloud",
    "word_cloud",
    "wordCloud",
    "tag_cloud",
    "tagcloud",
    "cloud",
    "token_cloud",
    "text_cloud"
)]
#[crate::sera_builder("build_wordcloud")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let words = a.words.unwrap_or_default();
    let frequencies = a
        .frequencies
        .unwrap_or_else(|| a.values.unwrap_or_default());
    let pal = o.pal();
    let hover = o.hj();
    let bg_str = o.bg_str();
    use crate::plot::statistical::wordcloud::{
        render_wordcloud_html, WordCloudConfig, WordCloudShape, WordCloudVariant,
    };
    let variant = WordCloudVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let shape = WordCloudShape::from_str(o.shape.as_deref().unwrap_or("rect"));
    let mask = o.mask.clone().unwrap_or_default();
    let points_x = o.points_x.clone().unwrap_or_default();
    let points_y = o.points_y.clone().unwrap_or_default();
    let point_clusters = o.category_indices.clone().unwrap_or_default();
    let cluster_labels = o.cluster_labels.clone().unwrap_or_default();
    let edges_i = o.edges_i.clone().unwrap_or_default();
    let edges_j = o.edges_j.clone().unwrap_or_default();
    let edges_w = o.edges_w.clone().unwrap_or_default();
    let html = render_wordcloud_html(&WordCloudConfig {
        variant,
        shape,
        title,
        words: &words,
        frequencies: &frequencies,
        palette: &pal,
        width: o.w(900),
        height: o.h(500),
        min_font: o.min_font.unwrap_or(12.0),
        max_font: o.max_font.unwrap_or(72.0),
        bg_color: bg_str.as_deref(),
        sort_order: &o.srt(),
        hover: &hover,
        mask: &mask,
        mask_width: o.mask_width.unwrap_or(0),
        mask_height: o.mask_height.unwrap_or(0),
        points_x: &points_x,
        points_y: &points_y,
        point_clusters: &point_clusters,
        cluster_labels: &cluster_labels,
        edges_i: &edges_i,
        edges_j: &edges_j,
        edges_w: &edges_w,
        ..WordCloudConfig::default()
    });
    apply_bg3d(html, &o)
}
