pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod gradient;
pub mod lines;
pub mod quartiles;
pub mod mean;
pub mod rug;
pub mod heatmap;
pub mod spaced;
pub mod deluxe;

pub use variant::RidgelineVariant;
pub use config::RidgelineConfig;

pub fn render_ridgeline_html(cfg: &RidgelineConfig) -> String {
    use variant::RidgelineVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Gradient => gradient::render(cfg),
        Lines => lines::render(cfg),
        Quartiles => quartiles::render(cfg),
        Mean => mean::render(cfg),
        Rug => rug::render(cfg),
        Heatmap => heatmap::render(cfg),
        Spaced => spaced::render(cfg),
        Deluxe => deluxe::render(cfg),
    }
}

