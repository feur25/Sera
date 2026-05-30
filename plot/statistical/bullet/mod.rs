use crate::plot::{parse_all, apply};
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


pub use build as build_bullet;

#[crate::sera_alias("bullet", "bullets", "bullet_chart", "bullet_family", "bullet_graph")]
#[crate::sera_builder("build_bullet")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{BulletConfig, BulletVariant, render_bullet_html};
    let variant = BulletVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let targets = o.targets.clone().unwrap_or_default();
    let max_vals = o.max_vals.clone().unwrap_or_default();
    let ranges = o.ranges.clone().unwrap_or_default();
    let comparisons = o.comparisons.clone().unwrap_or_default();
    let hover = o.hj();
    let html = render_bullet_html(&BulletConfig {
        variant, title, labels: &labels, values: &values, targets: &targets,
        max_vals: &max_vals, ranges: &ranges, comparisons: &comparisons,
        width: o.w(800), height: o.h(300),
        hover: &hover, ..BulletConfig::default()
    });
    apply(html, &o)
}
