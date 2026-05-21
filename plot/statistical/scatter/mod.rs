pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod categorical;
pub mod gradient;
pub mod symbols;
pub mod labeled;
pub mod regression;
pub mod galaxy;
pub mod nova;
pub mod deluxe;

pub use variant::ScatterVariant;
pub use config::ScatterConfig;

pub fn render_scatter_variant_html(cfg: &ScatterConfig) -> String {
    use ScatterVariant::*;
    let v = if cfg.variant == Basic && !cfg.categories.is_empty() {
        Categorical
    } else {
        cfg.variant
    };
    match v {
        Basic       => basic::render(cfg),
        Categorical => categorical::render(cfg),
        Gradient    => gradient::render(cfg),
        Symbols     => symbols::render(cfg),
        Labeled     => labeled::render(cfg),
        Regression  => regression::render(cfg),
        Nova        => nova::render(cfg),
        Galaxy      => galaxy::render(cfg),
        Deluxe      => deluxe::render(cfg),
    }
}

