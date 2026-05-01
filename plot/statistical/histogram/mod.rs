pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod horizontal;
pub mod normalized;
pub mod cumulative;
pub mod stacked;
pub mod overlay;
pub mod step;

pub use variant::HistogramVariant;
pub use config::HistogramConfig;
pub use common::{compute_bins, bin_to_edges};

pub struct Histogram;

pub fn render_histogram_html(cfg: &HistogramConfig) -> String {
    let v = if cfg.variant == HistogramVariant::Basic && cfg.overlay_values.is_some() {
        HistogramVariant::Overlay
    } else {
        cfg.variant
    };
    match v {
        HistogramVariant::Basic => basic::render(cfg),
        HistogramVariant::Horizontal => horizontal::render(cfg),
        HistogramVariant::Normalized => normalized::render(cfg),
        HistogramVariant::Cumulative => cumulative::render(cfg),
        HistogramVariant::Stacked => stacked::render(cfg),
        HistogramVariant::Overlay => overlay::render(cfg),
        HistogramVariant::Step => step::render(cfg),
    }
}
