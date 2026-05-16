pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod smooth;
pub mod categorical;
pub mod highlight;
pub mod density;
pub mod gradient;
pub mod deluxe;
pub mod arc;
pub mod ribbon;

pub use variant::ParallelVariant;
pub use config::ParallelConfig;

pub fn render_parallel_html(cfg: &ParallelConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    if cfg.theme == ChartTheme::Deluxe { return deluxe::render(cfg); }
    if matches!(cfg.theme, ChartTheme::Aurora | ChartTheme::Inferno | ChartTheme::Frost | ChartTheme::Prism) {}
    use variant::ParallelVariant::*;
    match cfg.variant {
        Basic       => basic::render(cfg),
        Smooth      => smooth::render(cfg),
        Categorical => categorical::render(cfg),
        Highlight   => highlight::render(cfg),
        Density     => density::render(cfg),
        Gradient    => gradient::render(cfg),
        Deluxe      => deluxe::render(cfg),
        Arc         => arc::render(cfg),
        Ribbon      => ribbon::render(cfg),
    }
}

pub fn demo_kwargs(v: ParallelVariant) -> &'static str {
    use ParallelVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Smooth             => smooth::DEMO_KWARGS,
        Categorical        => categorical::DEMO_KWARGS,
        Highlight          => highlight::DEMO_KWARGS,
        Density            => density::DEMO_KWARGS,
        Gradient           => gradient::DEMO_KWARGS,
        Deluxe             => deluxe::DEMO_KWARGS,
        Arc                => arc::DEMO_KWARGS,
        Ribbon             => ribbon::DEMO_KWARGS,
    }
}
