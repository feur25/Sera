pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod monochrome;
pub mod highlighted;
pub mod bumps;
pub mod curved;
pub mod thick;
pub mod diverging;
pub mod stepped;

pub use variant::SlopeVariant;
pub use config::SlopeConfig;

pub fn render_slope_html(cfg: &SlopeConfig) -> String {
    use variant::SlopeVariant::*;
    match cfg.variant {
        Basic       => basic::render(cfg),
        Monochrome  => monochrome::render(cfg),
        Highlighted => highlighted::render(cfg),
        Bumps       => bumps::render(cfg),
        Curved      => curved::render(cfg),
        Thick       => thick::render(cfg),
        Diverging   => diverging::render(cfg),
        Stepped     => stepped::render(cfg),
    }
}

pub fn demo_kwargs(v: SlopeVariant) -> &'static str {
    use SlopeVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Monochrome         => monochrome::DEMO_KWARGS,
        Highlighted        => highlighted::DEMO_KWARGS,
        Bumps              => bumps::DEMO_KWARGS,
        Curved             => curved::DEMO_KWARGS,
        Thick              => thick::DEMO_KWARGS,
        Diverging          => diverging::DEMO_KWARGS,
        Stepped            => stepped::DEMO_KWARGS,
    }
}
