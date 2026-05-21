pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod donut;
pub mod flame;
pub mod rainbow;
pub mod outlined;
pub mod gapped;
pub mod depth_fade;
pub mod mono;

pub use variant::SunburstVariant;
pub use config::SunburstConfig;

pub fn render_sunburst_html(cfg: &SunburstConfig) -> String {
    use variant::SunburstVariant::*;
    match cfg.variant {
        Basic     => basic::render(cfg),
        Donut     => donut::render(cfg),
        Flame     => flame::render(cfg),
        Rainbow   => rainbow::render(cfg),
        Outlined  => outlined::render(cfg),
        Gapped    => gapped::render(cfg),
        DepthFade => depth_fade::render(cfg),
        Mono      => mono::render(cfg),
    }
}

