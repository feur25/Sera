pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod flat;
pub mod gradient;
pub mod outlined;
pub mod gapped;
pub mod nested;
pub mod heat;
pub mod mono;

pub use variant::TreemapVariant;
pub use config::TreemapConfig;

pub fn render_treemap_html(cfg: &TreemapConfig) -> String {
    use variant::TreemapVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Flat     => flat::render(cfg),
        Gradient => gradient::render(cfg),
        Outlined => outlined::render(cfg),
        Gapped   => gapped::render(cfg),
        Nested   => nested::render(cfg),
        Heat     => heat::render(cfg),
        Mono     => mono::render(cfg),
    }
}

pub fn demo_kwargs(v: TreemapVariant) -> &'static str {
    use TreemapVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Flat               => flat::DEMO_KWARGS,
        Gradient           => gradient::DEMO_KWARGS,
        Outlined           => outlined::DEMO_KWARGS,
        Gapped             => gapped::DEMO_KWARGS,
        Nested             => nested::DEMO_KWARGS,
        Heat               => heat::DEMO_KWARGS,
        Mono               => mono::DEMO_KWARGS,
    }
}
