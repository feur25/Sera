pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod stepped;
pub mod gradient;
pub mod rounded;
pub mod lollipop;
pub mod arrowed;
pub mod delta;
pub mod outlined;

pub use variant::WaterfallVariant;
pub use config::WaterfallConfig;

pub fn render_waterfall_html(cfg: &WaterfallConfig) -> String {
    use variant::WaterfallVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Stepped  => stepped::render(cfg),
        Gradient => gradient::render(cfg),
        Rounded  => rounded::render(cfg),
        Lollipop => lollipop::render(cfg),
        Arrowed  => arrowed::render(cfg),
        Delta    => delta::render(cfg),
        Outlined => outlined::render(cfg),
    }
}
