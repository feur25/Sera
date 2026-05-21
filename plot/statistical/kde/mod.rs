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

