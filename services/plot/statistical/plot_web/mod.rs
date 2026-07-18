use crate::plot::{apply, parse_all};

pub mod common;
pub mod config;
pub mod radial;
pub mod scatter;
pub mod variant;

pub use config::PlotWebConfig;
pub use variant::PlotWebVariant;

pub fn render_plot_web_html(cfg: &PlotWebConfig) -> String {
    use variant::PlotWebVariant::*;
    match cfg.variant {
        Scatter => scatter::render(cfg),
        Radial  => radial::render(cfg),
    }
}

pub use build as build_plot_web;

#[crate::sera_alias(
    "plot_web",
    "web_plot",
    "plotweb",
    "carbon_web",
    "web_chart",
    "flow_web"
)]
#[crate::sera_builder("build_plot_web")]
pub fn build(input: &str) -> String {
    use crate::plot::statistical::plot_web::{
        render_plot_web_html, PlotWebConfig, PlotWebVariant,
    };
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = PlotWebVariant::from_str(o.variant.as_deref().unwrap_or("scatter"));
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let sizes = a.sizes.or(a.size).unwrap_or_default();
    let labels = a.labels.clone().unwrap_or_default();
    let mut groups = a
        .categories
        .clone()
        .or_else(|| o.color_groups.clone())
        .unwrap_or_default();
    if groups.is_empty() {
        groups = vec![String::new(); x_values.len()];
    }
    let palette = o.pal();
    let xl = o.xl();
    let yl = o.yl();
    let zl = o.z_label.clone().unwrap_or_default();
    let cfg = PlotWebConfig {
        title,
        variant,
        x_values: &x_values,
        y_values: &y_values,
        sizes: &sizes,
        labels: &labels,
        groups: &groups,
        palette: &palette,
        x_label: &xl,
        y_label: &yl,
        size_label: &zl,
        x_log: o.log_scale.unwrap_or(false),
        min_r: o.min_size.unwrap_or(6.0),
        max_r: o.max_size.unwrap_or(38.0),
        width: o.w(1440),
        height: o.h(580),
        ..PlotWebConfig::default()
    };
    let html = render_plot_web_html(&cfg);
    apply(html, &o)
}
