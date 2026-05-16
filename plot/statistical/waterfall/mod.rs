pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod stepped;
pub mod lollipop;
pub mod arrowed;
pub mod delta;
pub mod horizontal;

pub use variant::WaterfallVariant;
pub use config::WaterfallConfig;

pub fn render_waterfall_html(cfg: &WaterfallConfig) -> String {
    use variant::WaterfallVariant::*;
    match cfg.variant {
        Basic      => basic::render(cfg),
        Stepped    => stepped::render(cfg),
        Lollipop   => lollipop::render(cfg),
        Arrowed    => arrowed::render(cfg),
        Delta      => delta::render(cfg),
        Horizontal => horizontal::render(cfg),
    }
}

pub fn demo_kwargs(v: WaterfallVariant) -> &'static str {
    use WaterfallVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Stepped            => stepped::DEMO_KWARGS,
        Lollipop           => lollipop::DEMO_KWARGS,
        Arrowed            => arrowed::DEMO_KWARGS,
        Delta              => delta::DEMO_KWARGS,
        Horizontal         => horizontal::DEMO_KWARGS,
    }
}
