pub mod variant;
pub mod shape;
pub mod config;
pub mod common;
pub mod basic;
pub mod image;
pub mod labelmap;
pub mod network;
pub mod bubble;
pub mod context;
pub mod neuron;

pub use variant::WordCloudVariant;
pub use shape::WordCloudShape;
pub use config::WordCloudConfig;

pub fn render_wordcloud_html(cfg: &WordCloudConfig) -> String {
    use variant::WordCloudVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Image    => image::render(cfg),
        LabelMap => labelmap::render(cfg),
        Network  => network::render(cfg),
        Bubble   => bubble::render(cfg),
        Context  => context::render(cfg),
        Neuron   => neuron::render(cfg),
    }
}

