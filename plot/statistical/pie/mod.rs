pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod donut;
pub mod exploded;
pub mod subplots;
pub mod proportional;
pub mod semi;
pub mod kpi;
pub mod nested;
pub mod pattern;

pub use variant::PieVariant;
pub use config::{Pie, PieConfig};

use crate::html::hover::{slots_to_json, build_chart_html};

pub fn render_pie_html(cfg: &PieConfig) -> String {
    let svg = match cfg.variant {
        PieVariant::Basic        => basic::render(cfg),
        PieVariant::Donut        => donut::render(cfg),
        PieVariant::Exploded     => exploded::render(cfg),
        PieVariant::Subplots     => subplots::render(cfg),
        PieVariant::Proportional => proportional::render(cfg),
        PieVariant::Semi         => semi::render(cfg),
        PieVariant::Kpi          => kpi::render(cfg),
        PieVariant::Nested       => nested::render(cfg),
        PieVariant::Pattern      => pattern::render(cfg),
    };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

pub fn demo_kwargs(v: PieVariant) -> &'static str {
    use PieVariant::*;
    match v {
        Basic              => basic::DEMO_KWARGS,
        Donut              => donut::DEMO_KWARGS,
        Exploded           => exploded::DEMO_KWARGS,
        Subplots           => subplots::DEMO_KWARGS,
        Proportional       => proportional::DEMO_KWARGS,
        Semi               => semi::DEMO_KWARGS,
        Kpi                => kpi::DEMO_KWARGS,
        Nested             => nested::DEMO_KWARGS,
        Pattern            => pattern::DEMO_KWARGS,
    }
}
