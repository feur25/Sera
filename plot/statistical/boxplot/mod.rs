pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod horizontal;
pub mod notched;
pub mod grouped;
pub mod points;
pub mod outliers;
pub mod strip;
pub mod violin;
pub mod letter_value;
pub mod rainbow;

pub use variant::BoxplotVariant;
pub use config::BoxplotConfig;

pub fn render_boxplot_html(cfg: &BoxplotConfig) -> String {
    use BoxplotVariant::*;
    match cfg.variant {
        Basic       => basic::render(cfg),
        Horizontal  => horizontal::render(cfg),
        Notched     => notched::render(cfg),
        Grouped     => grouped::render(cfg),
        Points      => points::render(cfg),
        Outliers    => outliers::render(cfg),
        Strip       => strip::render(cfg),
        Violin      => violin::render(cfg),
        LetterValue => letter_value::render(cfg),
        Rainbow     => rainbow::render(cfg),
    }
}

pub fn demo_kwargs(v: BoxplotVariant) -> &'static str {
    use BoxplotVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Horizontal         => horizontal::DEMO_KWARGS,
        Notched            => notched::DEMO_KWARGS,
        Grouped            => grouped::DEMO_KWARGS,
        Points             => points::DEMO_KWARGS,
        Outliers           => outliers::DEMO_KWARGS,
        Strip              => strip::DEMO_KWARGS,
        Violin             => violin::DEMO_KWARGS,
        LetterValue        => letter_value::DEMO_KWARGS,
        Rainbow            => rainbow::DEMO_KWARGS,
    }
}
