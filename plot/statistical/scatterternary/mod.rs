use crate::plot::{apply, parse_all};
pub mod basic;
pub mod bubble;
pub mod common;
pub mod config;
pub mod gradient;
pub mod variant;

pub use config::ScatterTernaryConfig;
pub use variant::ScatterTernaryVariant;

pub fn render_scatter_ternary_html(cfg: &ScatterTernaryConfig) -> String {
    use variant::ScatterTernaryVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Gradient => gradient::render(cfg),
        Bubble => bubble::render(cfg),
    }
}

pub use build as build_scatter_ternary;

#[crate::sera_alias(
    "scatterternary",
    "scatter_ternary",
    "ternary",
    "ternary_plot",
    "ternary_scatter"
)]
#[crate::sera_builder("build_scatter_ternary")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let a_values = a.x.unwrap_or_default();
    let b_values = a.y.unwrap_or_default();
    let c_values = a.z.unwrap_or_default();
    let labels = a.labels.clone().unwrap_or_default();
    use crate::plot::statistical::{render_scatter_ternary_html, ScatterTernaryConfig, ScatterTernaryVariant};
    let hover = o.hj();
    let variant = ScatterTernaryVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let palette = o.pal();
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let color_values = o.color_values.clone().unwrap_or_default();
    let al = o.x_label.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "A".to_string());
    let bl = o.y_label.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "B".to_string());
    let cl = o.z_label.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "C".to_string());
    let html = render_scatter_ternary_html(&ScatterTernaryConfig {
        variant,
        title,
        a_values: &a_values,
        b_values: &b_values,
        c_values: &c_values,
        labels: &labels,
        color_values: &color_values,
        palette: &palette,
        colorscale: &colorscale,
        point_size: o.point_size.unwrap_or(4.0),
        a_label: &al,
        b_label: &bl,
        c_label: &cl,
        hover: &hover,
        width: o.w(700),
        height: o.h(640),
        ..ScatterTernaryConfig::default()
    });
    apply(html, &o)
}
