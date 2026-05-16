pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod stacked;
pub mod thermo;
pub mod segmented;
pub mod minimal;
pub mod dot;
pub mod progress;
pub mod compare;

pub use variant::BulletVariant;
pub use config::BulletConfig;

pub fn render_bullet_html(cfg: &BulletConfig) -> String {
    use variant::BulletVariant::*;
    match cfg.variant {
        Basic     => basic::render(cfg),
        Stacked   => stacked::render(cfg),
        Thermo    => thermo::render(cfg),
        Segmented => segmented::render(cfg),
        Minimal   => minimal::render(cfg),
        Dot       => dot::render(cfg),
        Progress  => progress::render(cfg),
        Compare   => compare::render(cfg),
    }
}

pub fn demo_kwargs(v: BulletVariant) -> &'static str {
    use BulletVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Stacked            => stacked::DEMO_KWARGS,
        Thermo             => thermo::DEMO_KWARGS,
        Segmented          => segmented::DEMO_KWARGS,
        Minimal            => minimal::DEMO_KWARGS,
        Dot                => dot::DEMO_KWARGS,
        Progress           => progress::DEMO_KWARGS,
        Compare            => compare::DEMO_KWARGS,
    }
}
