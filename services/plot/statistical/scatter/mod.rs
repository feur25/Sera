use crate::plot::{apply, parse_all};
pub mod basic;
pub mod categorical;
pub mod common;
pub mod config;
pub mod continuous_hue;
pub mod dual_style;
pub mod facet;
pub mod labeled;
pub mod regression;
pub mod residual;
pub mod sized;
pub mod symbols;
pub mod variant;
pub mod wide_form;

pub use config::ScatterConfig;
pub use variant::ScatterVariant;

pub fn render_scatter_variant_html(cfg: &ScatterConfig) -> String {
    use ScatterVariant::*;
    let v = if cfg.variant != Basic {
        cfg.variant
    } else if !cfg.series.is_empty() {
        WideForm
    } else if !cfg.categories.is_empty() && !cfg.categories2.is_empty() {
        DualStyle
    } else if !cfg.color_values.is_empty() {
        ContinuousHue
    } else if !cfg.categories.is_empty() {
        Categorical
    } else {
        cfg.variant
    };
    match v {
        Basic => basic::render(cfg),
        Categorical => categorical::render(cfg),
        Symbols => symbols::render(cfg),
        Labeled => labeled::render(cfg),
        Regression => regression::render(cfg),
        Residual => residual::render(cfg),
        DualStyle => dual_style::render(cfg),
        ContinuousHue => continuous_hue::render(cfg),
        Facet => facet::render(cfg),
        Sized => sized::render(cfg),
        WideForm => wide_form::render(cfg),
    }
}

pub use build as build_scatter_chart;

#[crate::sera_alias(
    "scatter",
    "scatter_chart",
    "scatter_family",
    "scatter_unified",
    "scatters"
)]
#[crate::sera_builder("build_scatter_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let raw_labels = a.labels.unwrap_or_default();
    let x: Vec<f64> = a.x.unwrap_or_else(|| {
        raw_labels
            .iter()
            .enumerate()
            .map(|(i, s)| s.parse::<f64>().unwrap_or(i as f64))
            .collect()
    });
    let y: Vec<f64> = a.y.unwrap_or_else(|| a.values.unwrap_or_default());
    let lbls = raw_labels;
    let sz = a.sizes.unwrap_or_default();
    let cgs = o.color_groups.clone().unwrap_or_default();
    let cats_arg = a.categories.clone().unwrap_or_default();
    let categories: Vec<String> = if !cats_arg.is_empty() {
        cats_arg
    } else {
        cgs.clone()
    };
    let categories2 = a.categories2.clone().unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = {
        let sn = o.series_names.clone().unwrap_or_default();
        a.series
            .clone()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(si, vals)| {
                (
                    sn.get(si)
                        .cloned()
                        .unwrap_or_else(|| format!("S{}", si + 1)),
                    vals,
                )
            })
            .collect()
    };

    let (x, y, lbls, sz, cgs, categories, categories2, series) = if !series.is_empty() {
        let dec = crate::plot::decimate::Decimator::for_series(o.max_points, &series);
        (
            dec.apply(x),
            y,
            lbls,
            sz,
            cgs,
            categories,
            categories2,
            dec.apply_each(series),
        )
    } else {
        let dec = crate::plot::decimate::Decimator::new(o.max_points, &y);
        (
            dec.apply(x),
            dec.apply(y),
            dec.apply(lbls),
            dec.apply(sz),
            dec.apply(cgs),
            dec.apply(categories),
            dec.apply(categories2),
            series,
        )
    };

    if o.variant.is_some()
        || !o.color_values.clone().unwrap_or_default().is_empty()
        || !categories2.is_empty()
        || !series.is_empty()
        || o.symbol.is_some()
        || o.symbols.is_some()
    {
        use crate::plot::statistical::scatter::{
            render_scatter_variant_html, ScatterConfig, ScatterVariant,
        };
        let palette = o.pal();
        let hover = o.hj();
        let xl = o.xl();
        let yl = o.yl();
        let srt = o.srt();
        let lp = o.lp();
        let color_values = o.color_values.clone().unwrap_or_default();
        let syms = o.symbols.clone().unwrap_or_default();
        let symbol = o.symbol.clone().unwrap_or_else(|| "circle".to_string());
        let reg_t = o
            .regression_type
            .clone()
            .unwrap_or_else(|| "linear".to_string());
        let mut variant = ScatterVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
        if o.show_regression.unwrap_or(false) && variant == ScatterVariant::Basic {
            variant = ScatterVariant::Regression;
        }
        let cfg = ScatterConfig {
            variant,
            title,
            x_label: &xl,
            y_label: &yl,
            width: o.w(900),
            height: o.h(500),
            gridlines: o.grid(),
            sort_order: &srt,
            legend_position: &lp,
            hover: &hover,
            palette: &palette,
            x_values: &x,
            y_values: &y,
            categories: &categories,
            categories2: &categories2,
            labels: &lbls,
            color_values: &color_values,
            symbols: &syms,
            color_hex: o.color_hex.unwrap_or(0),
            color_low: o.color_low.unwrap_or(0x636EFA),
            color_high: o.color_high.unwrap_or(0xF43F5E),
            point_size: o.point_size.unwrap_or(5.0),
            stroke_width: o.stroke_width.unwrap_or(1.0),
            show_text: o.show_values.or(o.show_text).unwrap_or(false),
            symbol: &symbol,
            regression_type: &reg_t,
            series: &series,
            size_min: o.min_size.unwrap_or(4.0),
            size_max: o.max_size.unwrap_or(18.0),
        };
        let html = render_scatter_variant_html(&cfg);
        return apply(html, &o);
    }
    let hover = o.hj();
    let html = crate::plot::default::render_scatter_html(
        title,
        &x,
        &y,
        &lbls,
        o.w(900),
        o.h(540),
        &hover,
        &sz,
        &cgs,
        &o.pal(),
        &o.xl(),
        &o.yl(),
        o.color_hex.unwrap_or(0),
        o.grid(),
        o.show_text.unwrap_or(false),
        o.show_regression.unwrap_or(false),
        o.regression_type.as_deref().unwrap_or("linear"),
    );
    apply(html, &o)
}
