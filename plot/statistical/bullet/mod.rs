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

