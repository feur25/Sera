pub mod variant;
pub mod config;
pub mod basic;
pub mod multi;
pub mod stepped;
pub mod spline;
pub mod filled;
pub mod sparkline;
pub mod dashed;
pub mod connected_scatter;
pub mod gapped;

pub use variant::LineVariant;
pub use config::LineConfig;

pub fn render_line_html(cfg: &LineConfig) -> String {
    use LineVariant::*;
    match cfg.variant {
        Basic            => basic::render(cfg),
        Multi            => multi::render(cfg),
        Stepped          => stepped::render(cfg),
        Spline           => spline::render(cfg),
        Filled           => filled::render(cfg),
        Sparkline        => sparkline::render(cfg),
        Dashed           => dashed::render(cfg),
        ConnectedScatter => connected_scatter::render(cfg),
        Gapped           => gapped::render(cfg),
    }
}
