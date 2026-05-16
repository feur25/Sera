pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod categorical;
pub mod gradient;
pub mod labeled;
pub mod outlined;
pub mod negative;
pub mod plasma;
pub mod deluxe;

pub use variant::BubbleVariant;
pub use config::BubbleConfig;

pub fn render_bubble_html(cfg: &BubbleConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    match cfg.theme {
        ChartTheme::Deluxe => return deluxe::render(cfg),
        ChartTheme::Prism  => return plasma::render(cfg),
        _                  => {}
    }
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
        Plasma      => plasma::render(cfg),
        Deluxe      => deluxe::render(cfg),
    }
}

pub fn demo_kwargs(v: BubbleVariant) -> &'static str {
    use BubbleVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Categorical        => categorical::DEMO_KWARGS,
        Gradient           => gradient::DEMO_KWARGS,
        Labeled            => labeled::DEMO_KWARGS,
        Outlined           => outlined::DEMO_KWARGS,
        Negative           => negative::DEMO_KWARGS,
        Plasma             => plasma::DEMO_KWARGS,
        Deluxe             => deluxe::DEMO_KWARGS,
    }
}
