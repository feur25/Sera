use crate::plot::{apply, parse_all};
pub mod arc;
pub mod basic;
pub mod categorical;
pub mod common;
pub mod config;
pub mod deluxe;
pub mod density;
pub mod gradient;
pub mod highlight;
pub mod ribbon;
pub mod smooth;
pub mod variant;

pub use config::ParallelConfig;
pub use variant::ParallelVariant;

pub fn render_parallel_html(cfg: &ParallelConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    if cfg.theme == ChartTheme::Deluxe {
        return deluxe::render(cfg);
    }
    if matches!(
        cfg.theme,
        ChartTheme::Aurora | ChartTheme::Inferno | ChartTheme::Frost | ChartTheme::Prism
    ) {}
    use variant::ParallelVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Smooth => smooth::render(cfg),
        Categorical => categorical::render(cfg),
        Highlight => highlight::render(cfg),
        Density => density::render(cfg),
        Gradient => gradient::render(cfg),
        Deluxe => deluxe::render(cfg),
        Arc => arc::render(cfg),
        Ribbon => ribbon::render(cfg),
    }
}

pub use build as build_parallel;

#[crate::sera_alias(
    "parallel",
    "parallels",
    "parallel_chart",
    "parallel_family",
    "parallel_coordinates",
    "parcoords"
)]
#[crate::sera_builder("build_parallel")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes_raw = a.axes.unwrap_or_default();
    let series_values = a.series.unwrap_or_default();
    let n_vals = series_values.first().map(|v| v.len()).unwrap_or(0);
    let axes: Vec<String> = if axes_raw.is_empty() && n_vals > 0 {
        (0..n_vals).map(|i| format!("Axis {}", i + 1)).collect()
    } else {
        axes_raw
    };
    let names_raw = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if names_raw.is_empty() && !series_values.is_empty() {
        (0..series_values.len())
            .map(|i| format!("Series {}", i + 1))
            .collect()
    } else {
        names_raw
    };
    let pal = o.pal();
    let hover = o.hj();
    use crate::plot::statistical::parallel::{
        render_parallel_html, ParallelConfig, ParallelVariant,
    };
    let variant = ParallelVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let categories = o.category_indices.clone().unwrap_or_default();
    let html = render_parallel_html(&ParallelConfig {
        variant,
        title,
        axes: &axes,
        series_names: &names,
        series_values: &series_values,
        categories: &categories,
        palette: &pal,
        width: o.w(1000),
        height: o.h(500),
        highlight_index: o.highlight_index.unwrap_or(-1),
        color_axis: o.color_axis.unwrap_or(-1),
        hover: &hover,
        theme: {
            use crate::plot::statistical::ChartTheme;
            ChartTheme::from_str(o.theme.as_deref().unwrap_or("none"))
        },
        ..ParallelConfig::default()
    });
    apply(html, &o)
}
