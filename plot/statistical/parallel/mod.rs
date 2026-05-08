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
