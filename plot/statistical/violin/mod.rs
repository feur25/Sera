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
pub mod crystal;

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
        Crystal    => crystal::render(cfg),
    }
}

pub fn demo_kwargs(v: ViolinVariant) -> &'static str {
    use ViolinVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Box                => with_box::DEMO_KWARGS,
        Quartile           => quartile::DEMO_KWARGS,
        Mean               => mean::DEMO_KWARGS,
        Points             => points::DEMO_KWARGS,
        Strip              => strip::DEMO_KWARGS,
        Horizontal         => horizontal::DEMO_KWARGS,
        Split              => split::DEMO_KWARGS,
        Half               => half::DEMO_KWARGS,
        Rainbow            => rainbow::DEMO_KWARGS,
        Aurora             => aurora::DEMO_KWARGS,
        Deluxe             => deluxe::DEMO_KWARGS,
        Crystal            => crystal::DEMO_KWARGS,
    }
}
