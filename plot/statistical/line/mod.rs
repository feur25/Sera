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
pub mod neon;

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
        Neon             => neon::render(cfg),
    }
}

pub fn demo_kwargs(v: LineVariant) -> &'static str {
    use LineVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Multi              => multi::DEMO_KWARGS,
        Stepped            => stepped::DEMO_KWARGS,
        Spline             => spline::DEMO_KWARGS,
        Filled             => filled::DEMO_KWARGS,
        Sparkline          => sparkline::DEMO_KWARGS,
        Dashed             => dashed::DEMO_KWARGS,
        ConnectedScatter   => connected_scatter::DEMO_KWARGS,
        Gapped             => gapped::DEMO_KWARGS,
        Neon               => neon::DEMO_KWARGS,
    }
}
