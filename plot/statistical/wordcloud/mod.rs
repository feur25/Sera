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

pub fn demo_kwargs(v: WordCloudVariant) -> &'static str {
    use WordCloudVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Image              => image::DEMO_KWARGS,
        LabelMap           => labelmap::DEMO_KWARGS,
        Network            => network::DEMO_KWARGS,
        Bubble             => bubble::DEMO_KWARGS,
        Context            => context::DEMO_KWARGS,
        Neuron             => neuron::DEMO_KWARGS,
    }
}
