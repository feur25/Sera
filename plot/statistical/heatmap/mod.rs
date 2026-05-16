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

pub fn demo_kwargs(v: HeatmapVariant) -> &'static str {
    use HeatmapVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Annotated          => annotated::DEMO_KWARGS,
        Categorical        => categorical::DEMO_KWARGS,
        Unequal            => unequal::DEMO_KWARGS,
        Log                => log::DEMO_KWARGS,
        Discrete           => discrete::DEMO_KWARGS,
        Correlation        => correlation::DEMO_KWARGS,
        Density            => density::DEMO_KWARGS,
        Contour            => contour::DEMO_KWARGS,
        Temporal           => temporal::DEMO_KWARGS,
        Cluster            => cluster::DEMO_KWARGS,
        Bubble             => bubble::DEMO_KWARGS,
        Marginal           => marginal::DEMO_KWARGS,
        Confusion          => confusion::DEMO_KWARGS,
        Pivot              => pivot::DEMO_KWARGS,
    }
}
