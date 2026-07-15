use crate::plot::{apply, parse_all};

pub mod basic;
pub mod config;
pub mod variant;

pub use config::OrbitaConfig;
pub use variant::OrbitaVariant;

pub fn render_orbita_html(cfg: &OrbitaConfig) -> String {
    use OrbitaVariant::*;
    match cfg.variant {
        Classic => basic::render(cfg),
        Bubble  => basic::render_bubble(cfg),
        Trail   => basic::render_trail(cfg),
        Glow    => basic::render_glow(cfg),
        Minimal => basic::render_minimal(cfg),
    }
}

pub use build as build_orbita;

#[crate::sera_alias("orbita", "orbit", "orbit_chart", "orbital", "multi_orbit", "concentric")]
#[crate::sera_builder("build_orbita")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title        = title_s.as_str();
    let series_names = o.series_names.clone().unwrap_or_default();
    let labels       = a.labels.unwrap_or_default();
    let matrix: Vec<f64> = a.matrix.unwrap_or_default().into_iter().flatten().collect();
    let palette      = o.pal();
    let variant      = OrbitaVariant::from_str(o.variant.as_deref().unwrap_or("classic"));
    let hover        = o.hj();

    let html = render_orbita_html(&OrbitaConfig {
        variant,
        title,
        series_names: &series_names,
        labels:       &labels,
        matrix:       &matrix,
        palette:      &palette,
        hover:        &hover,
        width:        o.w(580),
        height:       o.h(580),
        ..OrbitaConfig::default()
    });
    apply(html, &o)
}
