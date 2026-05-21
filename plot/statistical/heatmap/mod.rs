pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod annotated;
pub mod categorical;
pub mod unequal;
pub mod log;
pub mod discrete;
pub mod correlation;
pub mod density;
pub mod contour;
pub mod temporal;
pub mod cluster;
pub mod bubble;
pub mod marginal;
pub mod confusion;
pub mod pivot;

pub use variant::HeatmapVariant;
pub use config::HeatmapConfig;

pub struct Heatmap;

pub fn render_heatmap_html(cfg: &HeatmapConfig) -> String {
    match cfg.variant {
        HeatmapVariant::Basic => basic::render(cfg),
        HeatmapVariant::Annotated => annotated::render(cfg),
        HeatmapVariant::Categorical => categorical::render(cfg),
        HeatmapVariant::Unequal => unequal::render(cfg),
        HeatmapVariant::Log => log::render(cfg),
        HeatmapVariant::Discrete => discrete::render(cfg),
        HeatmapVariant::Correlation => correlation::render(cfg),
        HeatmapVariant::Density => density::render(cfg),
        HeatmapVariant::Contour => contour::render(cfg),
        HeatmapVariant::Temporal => temporal::render(cfg),
        HeatmapVariant::Cluster => cluster::render(cfg),
        HeatmapVariant::Bubble => bubble::render(cfg),
        HeatmapVariant::Marginal => marginal::render(cfg),
        HeatmapVariant::Confusion => confusion::render(cfg),
        HeatmapVariant::Pivot => pivot::render(cfg),
    }
}

