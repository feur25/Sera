pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod lines;
pub mod filled;
pub mod markers;
pub mod dashed;
pub mod stacked;
pub mod polar_bar;
pub mod gradient;
pub mod deluxe;

pub use variant::RadarVariant;
pub use config::RadarConfig;

pub fn render_radar_html(cfg: &RadarConfig) -> String {
    use variant::RadarVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Lines    => lines::render(cfg),
        Filled   => filled::render(cfg),
        Markers  => markers::render(cfg),
        Dashed   => dashed::render(cfg),
        Stacked  => stacked::render(cfg),
        PolarBar => polar_bar::render(cfg),
        Gradient => gradient::render(cfg),
        Deluxe   => deluxe::render(cfg),
    }
}

pub fn demo_kwargs(v: RadarVariant) -> &'static str {
    use RadarVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Lines              => lines::DEMO_KWARGS,
        Filled             => filled::DEMO_KWARGS,
        Markers            => markers::DEMO_KWARGS,
        Dashed             => dashed::DEMO_KWARGS,
        Stacked            => stacked::DEMO_KWARGS,
        PolarBar           => polar_bar::DEMO_KWARGS,
        Gradient           => gradient::DEMO_KWARGS,
        Deluxe             => deluxe::DEMO_KWARGS,
    }
}
