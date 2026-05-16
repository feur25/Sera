pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod arrow;
pub mod delta;
pub mod barbell;
pub mod glow;
pub mod dotted;
pub mod ranked;

pub use variant::DumbbellVariant;
pub use config::DumbbellConfig;

pub fn render_dumbbell_html(cfg: &DumbbellConfig) -> String {
    use variant::DumbbellVariant::*;
    match cfg.variant {
        Basic    => basic::render(cfg),
        Arrow    => arrow::render(cfg),
        Delta    => delta::render(cfg),
        Barbell  => barbell::render(cfg),
        Glow     => glow::render(cfg),
        Dotted   => dotted::render(cfg),
        Ranked   => ranked::render(cfg),
    }
}

pub fn demo_kwargs(v: DumbbellVariant) -> &'static str {
    use DumbbellVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Arrow              => arrow::DEMO_KWARGS,
        Delta              => delta::DEMO_KWARGS,
        Barbell            => barbell::DEMO_KWARGS,
        Glow               => glow::DEMO_KWARGS,
        Dotted             => dotted::DEMO_KWARGS,
        Ranked             => ranked::DEMO_KWARGS,
    }
}
