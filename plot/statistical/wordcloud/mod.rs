pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod circle;
pub mod heart;
pub mod bird;
pub mod bubble;
pub mod glasses;

pub use variant::WordCloudVariant;
pub use config::WordCloudConfig;

pub fn render_wordcloud_html(cfg: &WordCloudConfig) -> String {
    use variant::WordCloudVariant::*;
    match cfg.variant {
        Basic   => basic::render(cfg),
        Circle  => circle::render(cfg),
        Heart   => heart::render(cfg),
        Bird    => bird::render(cfg),
        Bubble  => bubble::render(cfg),
        Glasses => glasses::render(cfg),
    }
}
