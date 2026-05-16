pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod stepped;
pub mod rounded;
pub mod chevron;
pub mod gradient;
pub mod pyramid;
pub mod inverted;
pub mod conversion;

pub use variant::FunnelVariant;
pub use config::FunnelConfig;

pub fn render_funnel_html(cfg: &FunnelConfig) -> String {
    use variant::FunnelVariant::*;
    match cfg.variant {
        Basic      => basic::render(cfg),
        Stepped    => stepped::render(cfg),
        Rounded    => rounded::render(cfg),
        Chevron    => chevron::render(cfg),
        Gradient   => gradient::render(cfg),
        Pyramid    => pyramid::render(cfg),
        Inverted   => inverted::render(cfg),
        Conversion => conversion::render(cfg),
    }
}

pub fn demo_kwargs(v: FunnelVariant) -> &'static str {
    use FunnelVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Stepped            => stepped::DEMO_KWARGS,
        Rounded            => rounded::DEMO_KWARGS,
        Chevron            => chevron::DEMO_KWARGS,
        Gradient           => gradient::DEMO_KWARGS,
        Pyramid            => pyramid::DEMO_KWARGS,
        Inverted           => inverted::DEMO_KWARGS,
        Conversion         => conversion::DEMO_KWARGS,
    }
}
