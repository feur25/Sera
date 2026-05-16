pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod radial;
pub mod arc270;
pub mod sleek;
pub mod tick;
pub mod segmented;
pub mod glow;
pub mod concentric;

pub use variant::GaugeVariant;
pub use config::GaugeConfig;

pub fn render_gauge_html(cfg: &GaugeConfig) -> String {
    use variant::GaugeVariant::*;
    match cfg.variant {
        Basic      => basic::render(cfg),
        Radial     => radial::render(cfg),
        Arc270     => arc270::render(cfg),
        Sleek      => sleek::render(cfg),
        Tick       => tick::render(cfg),
        Segmented  => segmented::render(cfg),
        Glow       => glow::render(cfg),
        Concentric => concentric::render(cfg),
    }
}

pub fn demo_kwargs(v: GaugeVariant) -> &'static str {
    use GaugeVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Radial             => radial::DEMO_KWARGS,
        Arc270             => arc270::DEMO_KWARGS,
        Sleek              => sleek::DEMO_KWARGS,
        Tick               => tick::DEMO_KWARGS,
        Segmented          => segmented::DEMO_KWARGS,
        Glow               => glow::DEMO_KWARGS,
        Concentric         => concentric::DEMO_KWARGS,
    }
}
