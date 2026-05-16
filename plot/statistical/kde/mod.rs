pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod outline;
pub mod stepped;
pub mod rug;
pub mod histogram;
pub mod normalized;
pub mod cumulative;
pub mod gradient;

pub use variant::KdeVariant;
pub use config::KdeConfig;
pub use common::{scott_bw, kde_eval};

pub fn render_kde_html(cfg: &KdeConfig) -> String {
    use variant::KdeVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Outline => outline::render(cfg),
        Stepped => stepped::render(cfg),
        Rug => rug::render(cfg),
        Histogram => histogram::render(cfg),
        Normalized => normalized::render(cfg),
        Cumulative => cumulative::render(cfg),
        Gradient => gradient::render(cfg),
    }
}

pub fn demo_kwargs(v: KdeVariant) -> &'static str {
    use KdeVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Outline            => outline::DEMO_KWARGS,
        Stepped            => stepped::DEMO_KWARGS,
        Rug                => rug::DEMO_KWARGS,
        Histogram          => histogram::DEMO_KWARGS,
        Normalized         => normalized::DEMO_KWARGS,
        Cumulative         => cumulative::DEMO_KWARGS,
        Gradient           => gradient::DEMO_KWARGS,
    }
}
