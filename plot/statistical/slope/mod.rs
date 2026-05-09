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


