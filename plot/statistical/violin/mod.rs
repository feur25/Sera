pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod with_box;
pub mod quartile;
pub mod mean;
pub mod points;
pub mod strip;
pub mod horizontal;
pub mod split;
pub mod half;
pub mod rainbow;
pub mod aurora;
pub mod deluxe;

pub use variant::ViolinVariant;
pub use config::ViolinConfig;

pub fn render_violin_html(cfg: &ViolinConfig) -> String {
    use ViolinVariant::*;
    match cfg.variant {
        Basic      => basic::render(cfg),
        Box        => with_box::render(cfg),
        Quartile   => quartile::render(cfg),
        Mean       => mean::render(cfg),
        Points     => points::render(cfg),
        Strip      => strip::render(cfg),
        Horizontal => horizontal::render(cfg),
        Split      => split::render(cfg),
        Half       => half::render(cfg),
        Rainbow    => rainbow::render(cfg),
        Aurora     => aurora::render(cfg),
        Deluxe     => deluxe::render(cfg),
    }
}
