use crate::plot::{parse_all, apply};
pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod categorical;
pub mod gradient;
pub mod symbols;
pub mod labeled;
pub mod regression;
pub mod galaxy;
pub mod nova;
pub mod deluxe;

pub use variant::ScatterVariant;
pub use config::ScatterConfig;

pub fn render_scatter_variant_html(cfg: &ScatterConfig) -> String {
    use ScatterVariant::*;
    let v = if cfg.variant == Basic && !cfg.categories.is_empty() {
        Categorical
    } else {
        cfg.variant
    };
    match v {
        Basic       => basic::render(cfg),
        Categorical => categorical::render(cfg),
        Gradient    => gradient::render(cfg),
        Symbols     => symbols::render(cfg),
        Labeled     => labeled::render(cfg),
        Regression  => regression::render(cfg),
        Nova        => nova::render(cfg),
        Galaxy      => galaxy::render(cfg),
        Deluxe      => deluxe::render(cfg),
    }
}


pub use build as build_scatter_chart;

#[crate::sera_alias("scatter", "scatter_chart", "scatter_family", "scatter_unified", "scatters")]
#[crate::sera_builder("build_scatter_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let raw_labels = a.labels.unwrap_or_default();
    let x: Vec<f64> = a.x.unwrap_or_else(|| {
        raw_labels.iter().enumerate().map(|(i, s)| s.parse::<f64>().unwrap_or(i as f64)).collect()
    });
    let y: Vec<f64> = a.y.unwrap_or_else(|| a.values.unwrap_or_default());
    let lbls = raw_labels;
    let sz = a.sizes.unwrap_or_default();
    let cgs = o.color_groups.clone().unwrap_or_default();
    let cats_arg = a.categories.clone().unwrap_or_default();
    let categories: Vec<String> = if !cats_arg.is_empty() { cats_arg } else { cgs.clone() };
    if o.variant.is_some() || !o.color_values.clone().unwrap_or_default().is_empty() || o.symbol.is_some() || o.symbols.is_some() {
        use crate::plot::statistical::scatter::{ScatterConfig, ScatterVariant, render_scatter_variant_html};
        let palette = o.pal();
        let hover = o.hj();
        let xl = o.xl(); let yl = o.yl(); let srt = o.srt(); let lp = o.lp();
        let color_values = o.color_values.clone().unwrap_or_default();
        let syms = o.symbols.clone().unwrap_or_default();
        let symbol = o.symbol.clone().unwrap_or_else(|| "circle".to_string());
        let reg_t = o.regression_type.clone().unwrap_or_else(|| "linear".to_string());
        let mut variant = ScatterVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
        if o.show_regression.unwrap_or(false) && variant == ScatterVariant::Basic {
            variant = ScatterVariant::Regression;
        }
        let cfg = ScatterConfig {
            variant, title,
            x_label: &xl, y_label: &yl,
            width: o.w(900), height: o.h(500),
            gridlines: o.grid(), sort_order: &srt, legend_position: &lp,
            hover: &hover, palette: &palette,
            x_values: &x, y_values: &y,
            categories: &categories, labels: &lbls, color_values: &color_values,
            symbols: &syms,
            color_hex: o.color_hex.unwrap_or(0),
            color_low: o.color_low.unwrap_or(0x6366F1),
            color_high: o.color_high.unwrap_or(0xF43F5E),
            point_size: o.point_size.unwrap_or(5.0),
            stroke_width: o.stroke_width.unwrap_or(1.0),
            show_text: o.show_values.or(o.show_text).unwrap_or(false),
            symbol: &symbol,
            regression_type: &reg_t,
        };
        let html = render_scatter_variant_html(&cfg);
        return apply(html, &o);
    }
    let hover = o.hj();
    let html = crate::plot::default::render_scatter_html(
        title, &x, &y, &lbls, o.w(900), o.h(540), &hover, &sz, &cgs,
        &o.pal(), &o.xl(), &o.yl(), o.color_hex.unwrap_or(0), o.grid(),
        o.show_text.unwrap_or(false), o.show_regression.unwrap_or(false),
        o.regression_type.as_deref().unwrap_or("linear"),
    );
    apply(html, &o)
}
