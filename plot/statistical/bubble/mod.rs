pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod categorical;
pub mod gradient;
pub mod labeled;
pub mod outlined;
pub mod negative;

pub use variant::BubbleVariant;
pub use config::BubbleConfig;

pub fn render_bubble_html(cfg: &BubbleConfig) -> String {
    use BubbleVariant::*;
    let v = if cfg.variant == Basic && !cfg.categories.is_empty() {
        Categorical
    } else {
        cfg.variant
    };
    match v {
        Basic       => basic::render(cfg),
        Categorical => categorical::render(cfg),
        Gradient    => gradient::render(cfg),
        Labeled     => labeled::render(cfg),
        Outlined    => outlined::render(cfg),
        Negative    => negative::render(cfg),
    }
}
